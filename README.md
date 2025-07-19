# System Monitor

A simple system monitoring tool written in Rust.

## Features

- Monitor CPU, memory, and disk usage.
- Send notifications via:
  - Console
  - Webhook (e.g., Slack, Discord)
  - Email (optional)

## Usage

```bash
cargo run -- --config ./config.toml


```bash
curl -X POST http://localhost:5000 \
     -H "Content-Type: application/json" \
     -d '{"text": "Test message from curl"}'
```