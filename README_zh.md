[English](README.md) | [简体中文](README_zh.md)

# Jolt

Jolt 是一个用 Rust 编写的简单而强大的 HTTP 性能测试工具。它通过发送指定数量的请求或运行特定持续时间来测试 Web 服务的性能，并可配置并发数、HTTP 方法、请求头和请求体。

## 功能

- **HTTP 方法支持**: 支持各种 HTTP 方法（GET、POST、PUT 等）。
- **自定义请求头和请求体**: 发送自定义 HTTP 请求头和请求体。
- **灵活的负载生成**: 通过请求数量或持续时间定义负载。
- **并发控制**: 调整并发请求的数量。
- **性能指标**: 报告关键指标，如每秒请求数、延迟百分位数。
- **JSON 输出**: 可选择以 JSON 格式输出结果，便于解析。

## 使用方法

Jolt 是一个命令行工具。以下是其基本用法概述：

```bash
jolt -u <URL> (-n <请求数> | -t <持续时间>) [选项]
```

### 参数

- `-u, --url <URL>`: **必填**。要测试的 URL。
- `-n, --requests <请求数>`: 要发送的请求数量。与 `--duration` 冲突。
- `-t, --duration <持续时间>`: 测试持续时间（例如，`10s`、`1m`、`5m30s`）。与 `--requests` 冲突。

### 选项

- `-X, --method <方法>`: 要使用的 HTTP 方法（默认值：`GET`）。
- `-d, --body <请求体>`: 要发送的请求体。
- `-H, --header <请求头>`: 要发送的请求头（可以多次指定，例如，`-H "Content-Type: application/json"`）。
- `-c, --concurrency <并发数>`: 并发请求的数量（默认值：`1`）。
- `-p, --percentile <百分位数>`: 要报告的百分位数（默认值：`99`）。
- `-j, --json`: 以 JSON 格式输出结果。
- `-o, --output <文件路径>`: 结果输出文件路径。

### 示例

- 向 `http://localhost:8080` 发送 1000 个 GET 请求，并发连接数为 10：

  ```bash
  jolt -u http://localhost:8080 -n 1000 -c 10
  ```

- 运行 30 秒的测试，并发 50 个 POST 请求，发送 JSON 请求体和自定义请求头：

  ```bash
  jolt -u http://localhost:8080/api/data -t 30s -c 50 -X POST -H "Content-Type: application/json" -d '{"key": "value"}'
  ```

- 获取 JSON 输出并保存到文件：

  ```bash
  jolt -u http://localhost:8080 -n 100 -j -o results.json
  ```