use std::convert::Infallible;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use rand::Rng;

use lazy_static::lazy_static;
use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};
use tokio::time::sleep;
use tokio::time::Duration;

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
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
        let random_number = rand::thread_rng().gen_range(-50.0..100.0);

        if (ACCOUNT_BALANCE.get() + random_number) < 0.0 {
            print!("Account balance cannot be negative\n");
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        ACCOUNT_BALANCE.add(random_number);
        print!("Updated account balance\n");
        sleep(Duration::from_secs(1)).await;
    }
}

async fn handle_request(
    _req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    HTTP_COUNTER.inc();
    let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HTTP_BODY_GAUGE.set(buffer.len() as f64);

    let response = Response::builder()
        .status(200)
        .header("Content-Type", encoder.format_type())
        .body(Full::from(Bytes::from(buffer)))
        .unwrap();

    timer.observe_duration();

    Ok(response)
}

#[tokio::main]
async fn main() {
    tokio::task::spawn(update_account_balance());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

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
