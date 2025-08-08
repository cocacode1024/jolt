use hdrhistogram::Histogram;
use reqwest::{Client, Method};
use std::{sync::atomic::{AtomicUsize, Ordering}, time::Instant};

pub async fn make_request(
    client: &Client,
    url: &str,
    method: Method,
    body: &Option<String>,
    headers: &reqwest::header::HeaderMap,
    success: &AtomicUsize,
    failure: &AtomicUsize,
    hist: &mut Histogram<u64>,
) {
    let req_start = Instant::now();
    let mut request = client.request(method, url);

    if let Some(body) = body {
        request = request.body(body.clone());
    }

    let result = request.headers(headers.clone()).send().await;
    let elapsed = req_start.elapsed().as_millis() as u64;

    match result {
        Ok(resp) if resp.status().is_success() => {
            success.fetch_add(1, Ordering::Relaxed);
            let _ = hist.record(elapsed);
        }
        _ => {
            failure.fetch_add(1, Ordering::Relaxed);
        }
    }
}