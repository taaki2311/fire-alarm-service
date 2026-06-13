# Fire-Alarm Service

A CLI tool that monitors transit incidents and sends email alerts to subscribed users based on their preferred stations.

## Overview

The Fire-Alarm Service processes a stream of incident data (from stdin), checks each incident against a database of user preferences, and sends email notifications to users whose stations are affected. It's designed to run on a schedule (e.g., via cron) to deliver timely transit alerts.

## Prerequisites

- Rust 2024 or later
- Access to an SMTP relay server for sending emails
- SQL database (SQLite, MySQL, or PostgreSQL)
- A database seeded with users, stations, and rail lines (see [parent README](../README.md))

## Installation

Clone the parent repository and build the service:

```bash
git clone https://github.com/taaki2311/fire-alarm.git
cd fire-alarm/service
cargo build --release
```

## Configuration

Configure via command-line arguments or environment variables (with the `env` feature enabled):

| Argument | Env Var | Default | Description |
| -------- | ------- | ------- | ----------- |
| `-a, --address` | `ADDRESS` | `no-reply@fire-alarm.org` | Email address to send from |
| `-n, --name` | `NAME` | Address value | SMTP relay username (optional) |
| `-p, --password` | `PASSWORD` | (required) | SMTP relay password |
| `-r, --relay` | `RELAY` | (required) | SMTP relay server URL (e.g., `smtp.gmail.com:587`) |
| `-t, --timestamp` | `TIMESTAMP` | `timestamp.txt` | File storing the last check timestamp |
| `-d, --database` | `DATABASE` | (required) | Database connection URL |
| `-i, --index` | `INDEX` | `index.html` | Email template file path |

### Example `.env` file

```bash
# .env
PASSWORD=your_password
RELAY=smtp.gmail.com:587
ADDRESS=alerts@example.com
DATABASE=sqlite://db.sqlite
TIMESTAMP=timestamp.txt
INDEX=email_template.html
```

## Usage

Pipe incident JSON to the service:

```bash
cat incidents.json | cargo run --release -- \
  --password "$PASSWORD" \
  --relay "$RELAY" \
  --database "$DATABASE"
```

Or with environment variables:

```bash
cargo run --release --features env < incidents.json
```

## Input Format

The service reads JSON from stdin containing an array of incidents:

```json
[
  {
    "timestamp": "2026-01-15T10:30:00Z",
    "description": "Train delay at Metro Center due to signal problems"
  },
  {
    "timestamp": "2026-01-15T10:45:00Z",
    "description": "Red Line service advisory for Union Station"
  }
]
```

## How It Works

1. **Reads incidents** from stdin
2. **Loads timestamp** from file to filter only new incidents
3. **Connects to database** and retrieves user subscription preferences
4. **Matches incidents to stations** mentioned in user preferences
5. **Sends email alerts** via SMTP for matching incidents
6. **Updates timestamp** to prevent duplicate notifications

## Features

- Supports multiple database backends (SQLite, MySQL, PostgreSQL)
- Optional email file transport for testing (with `file-transport` feature)
- Optional logging support (with `log` feature)
- Graceful SMTP error handling

## Building

```bash
# Default features (SQLite, env support)
cargo build --release

# With logging
cargo build --release --features log

# With file-based email transport (for testing)
cargo build --release --features file-transport
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

The test suite includes:

- Incident file parsing validation
- SMTP connection testing
- End-to-end service flow

## Related

- **Website**: [../website/README.md](../website/README.md) — Web interface for subscribing to alerts
- **Parent README**: [../README.md](../README.md) — Project overview and setup

## License

MIT License — See [LICENSE](./LICENSE)
