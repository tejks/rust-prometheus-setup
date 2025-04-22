use prometheus::{labels, opts, push_metrics, register_gauge, Gauge};
use tokio::time::{sleep, Duration};

lazy_static::lazy_static! {
    static ref ACCOUNT_BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Current account balance in the system."
    ))
    .unwrap();
}

async fn push_metrics_to_gateway(pushgateway_url: &str, job_name: &str) {
    loop {
        ACCOUNT_BALANCE.set(ACCOUNT_BALANCE.get() + 10.0);
        if let Err(e) = push_metrics(
            job_name,
            labels! {"handler".to_owned() => "all".to_owned(),},
            pushgateway_url,
            prometheus::gather(),
            None,
        ) {
            eprintln!("Failed to push metrics to Pushgateway: {}", e);
        } else {
            println!("Metrics pushed successfully.");
        }

        sleep(Duration::from_secs(10)).await; // Push co 10 sekund
    }
}

#[tokio::main]
async fn main() {
    let pushgateway_url =
        "http://prometheus-prometheus-pushgateway.monitoring.svc.cluster.local:9091";
    let job_name = "pushgateway";

    tokio::task::spawn(push_metrics_to_gateway(pushgateway_url, job_name));

    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
