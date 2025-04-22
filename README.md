# Rust Prometheus Metrics Template

This repository provides a simple and efficient template for integrating [Prometheus](https://prometheus.io/) metrics into your Rust applications.  
It supports both **pull-based metrics exposition** and **push-based metrics via PushGateway**, depending on your deployment model.

&nbsp;
&nbsp;
&nbsp;

## 🚀 Branch Overview

| Branch            | Description                                     | Link                                           |
|-------------------|-------------------------------------------------|------------------------------------------------|
| `main`            | Pull-based Prometheus metrics exposition        | 👉 [View `main` branch](https://github.com/your-username/your-repo/tree/main) |
| `push-gateway`    | Push metrics to Prometheus PushGateway          | 👉 [View `push-gateway` branch](https://github.com/your-username/your-repo/tree/push-gateway) |

&nbsp;
&nbsp;
&nbsp;

## ⚡️ Quick Start

```bash
# Clone the repository
git clone https://github.com/tejks/rust-prometheus-setup.git
cd your-repo

# Checkout the desired branch:
git checkout main            # For pull-based
# or
git checkout push-gateway    # For push-based
```

&nbsp;
&nbsp;
&nbsp;

## 🧾 License

MIT — feel free to use, modify, and contribute!  
Feedback and PRs are welcome.

&nbsp;
&nbsp;
&nbsp;

## 🌐 Useful Links

- [Prometheus Docs](https://prometheus.io/docs/introduction/overview/)
- [PushGateway Docs](https://prometheus.io/docs/practices/pushing/)
- [prometheus crate (Rust)](https://crates.io/crates/prometheus)
