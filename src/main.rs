use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use lazy_static::lazy_static;
use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};
use tokio::time::sleep;
use tokio::time::Duration;

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
    static ref ACCOUNT_BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Current account balance in the system."
    ))
    .unwrap();
}

async fn update_account_balance() {
    loop {
        ACCOUNT_BALANCE.add(10.0);
        print!("Updated account balance\n");
        sleep(Duration::from_secs(1)).await;
    }
}

async fn handle_request(
    _req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    print!("Received request\n");
    // Increment Prometheus metrics
    HTTP_COUNTER.inc();
    let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

    // Gather and encode metrics
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HTTP_BODY_GAUGE.set(buffer.len() as f64);

    // Create a response
    let response = Response::builder()
        .status(200)
        .header("Content-Type", encoder.format_type())
        .body(Full::from(Bytes::from(buffer)))
        .unwrap();

    // Observe the request duration
    timer.observe_duration();

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 9899));
    println!("Listening on http://{}", addr);

    tokio::task::spawn(update_account_balance());

    // Create a TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
