[English](README.md) | [简体中文](README_zh.md)

# Jolt

Jolt is a simple, yet powerful, HTTP benchmarking tool written in Rust. It allows you to test the performance of your web services by sending a specified number of requests or running for a certain duration, with configurable concurrency, HTTP methods, headers, and request bodies.

## Features

- **HTTP Method Support**: Supports various HTTP methods (GET, POST, PUT, etc.).
- **Custom Headers & Body**: Send custom HTTP headers and request bodies.
- **Flexible Load Generation**: Define load by number of requests or duration.
- **Concurrency Control**: Adjust the number of concurrent requests.
- **Performance Metrics**: Reports key metrics like requests per second, latency percentiles.
- **JSON Output**: Option to output results in JSON format for easy parsing.


## Usage

Jolt is a command-line tool. Here's a basic overview of its usage:

```bash
jolt -u <URL> (-n <REQUESTS> | -t <DURATION>) [OPTIONS]
```

### Arguments

- `-u, --url <URL>`: **Required**. The URL to test.
- `-n, --requests <REQUESTS>`: Number of requests to send. Conflicts with `--duration`.
- `-t, --duration <DURATION>`: Duration of the test (e.g., `10s`, `1m`, `5m30s`). Conflicts with `--requests`.

### Options

- `-X, --method <METHOD>`: HTTP method to use (default: `GET`).
- `-d, --body <BODY>`: Request body to send.
- `-H, --header <HEADER>`: Request headers to send (can be specified multiple times, e.g., `-H "Content-Type: application/json"`).
- `-c, --concurrency <CONCURRENCY>`: Number of concurrent requests (default: `1`).
- `-p, --percentile <PERCENTILE>`: Percentile to report (default: `99`).
- `-j, --json`: Output results in JSON format.
- `-o, --output <FILE_PATH>`: Output file path for results.

### Examples

- Send 1000 GET requests to `http://localhost:8080` with 10 concurrent connections:

  ```bash
  jolt -u http://localhost:8080 -n 1000 -c 10
  ```

- Run a test for 30 seconds with 50 concurrent POST requests, sending a JSON body and custom header:

  ```bash
  jolt -u http://localhost:8080/api/data -t 30s -c 50 -X POST -H "Content-Type: application/json" -d '{"key": "value"}'
  ```

- Get JSON output and save to a file:

  ```bash
  jolt -u http://localhost:8080 -n 100 -j -o results.json
  ```

