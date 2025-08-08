use crate::{cli::Args, report::print_report, request::make_request};
use anyhow::{Context, Ok, Result};
use hdrhistogram::Histogram;
use reqwest::{Client, Method};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn run_benchmark(args: Args) -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host((args.concurrency * 2).max(10))
        .build()
        .with_context(|| "Failed to create HTTP client")?;

    let method: Method = args
        .method
        .parse()
        .with_context(|| format!("Invalid HTTP method: {}", args.method))?;

    let body = args.body.clone();

    let mut header_map = reqwest::header::HeaderMap::new();
    for h in &args.headers {
        if let Some((k, v)) = h.split_once(':') {
            let key = reqwest::header::HeaderName::from_bytes(k.trim().as_bytes())
                .with_context(|| format!("Invalid header name: '{}'", k))?;
            let value = reqwest::header::HeaderValue::from_str(v.trim())?;
            header_map.insert(key, value);
        } else {
            anyhow::bail!("Invalid header format: '{}'", h);
        }
    }

    if body.is_some() && !header_map.contains_key(reqwest::header::CONTENT_TYPE) {
        header_map.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }

    let start = Instant::now();
    let success = Arc::new(AtomicUsize::new(0));
    let failure = Arc::new(AtomicUsize::new(0));
    let histogram = Arc::new(Mutex::new(
        Histogram::<u64>::new_with_max(3600000, 3)
            .with_context(|| "Failed to create histogram")?,
    ));
    let running = Arc::new(AtomicUsize::new(1));

    let mut handles = Vec::new();

    if let Some(total_requests) = args.requests {
        let per_worker = total_requests / args.concurrency;
        let remainder = total_requests % args.concurrency;

        for i in 0..args.concurrency {
            let count = per_worker + if i < remainder { 1 } else { 0 };
            let client = client.clone();
            let url = args.url.clone();
            let method = method.clone();
            let body = body.clone();
            let headers = header_map.clone();
            let success = Arc::clone(&success);
            let failure = Arc::clone(&failure);
            let histogram = Arc::clone(&histogram);

            let task = tokio::spawn(async move {
                let mut local_hist = Histogram::<u64>::new_with_max(3600000, 3)
                    .with_context(|| "Failed to create histogram")?;
                for _ in 0..count {
                    make_request(
                        &client,
                        &url,
                        method.clone(),
                        &body,
                        &headers,
                        &success,
                        &failure,
                        &mut local_hist,
                    )
                    .await;
                }

                histogram
                    .lock()
                    .unwrap()
                    .add(&local_hist)
                    .with_context(|| "Failed to merge histogram")?;
                Ok(())
            });

            handles.push(task);
        }
    } else if let Some(duration) = args.duration {
        for _ in 0..args.concurrency {
            let client = client.clone();
            let url = args.url.clone();
            let method = method.clone();
            let body = body.clone();
            let headers = header_map.clone();
            let success = Arc::clone(&success);
            let failure = Arc::clone(&failure);
            let histogram = Arc::clone(&histogram);
            let running = Arc::clone(&running);

            let task = tokio::spawn(async move {
                let mut local_hist = Histogram::<u64>::new_with_max(3600000, 3)
                    .with_context(|| "Failed to create histogram")?;
                while running.load(Ordering::Relaxed) == 1 {
                    make_request(
                        &client,
                        &url,
                        method.clone(),
                        &body,
                        &headers,
                        &success,
                        &failure,
                        &mut local_hist,
                    )
                    .await;
                }

                histogram
                    .lock()
                    .unwrap()
                    .add(&local_hist)
                    .with_context(|| "Failed to merge histogram")?;
                Ok(())
            });
            handles.push(task);
        }

        sleep(duration.into()).await;
        running.store(0, Ordering::Relaxed);
    };

    for h in handles {
        if let Err(e) = h.await {
            anyhow::bail!("Worker task failed: {:?}", e);
        }
    }

    let total_duration = start.elapsed();
    let ok = success.load(Ordering::Relaxed);
    let fail = failure.load(Ordering::Relaxed);
    let hist = histogram.lock().unwrap();

    print_report(&args, ok, fail, total_duration, &hist)?;

    Ok(())
}
