use crate::{cli::Args, utils::round3};
use anyhow::{Context, Result};
use hdrhistogram::Histogram;
use std::{fs::File, io::Write, time::Duration};

pub fn print_report(
    args: &Args,
    success: usize,
    failure: usize,
    duration: Duration,
    hist: &Histogram<u64>,
) -> Result<()> {
    let total = success + failure;
    let rps = total as f64 / duration.as_secs_f64();

    if args.json {
        use serde_json::json;
        let json_output = json!({
            "url": args.url,
            "method": args.method,
            "requests": total,
            "concurrency": args.concurrency,
            "duration_sec": round3(duration.as_secs_f64()),
            "success": success,
            "failures": failure,
            "rps": round3(rps),
            "latency_ms": {
                "min": hist.min(),
                "max": hist.max(),
                "mean": hist.mean() as u64,
                format!("latency_p{}", args.percentile): hist.value_at_percentile(args.percentile as f64),
            }
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("================ Jolt Benchmark Result ================");
        println!("Target URL:         {}", args.url);
        println!("Method:             {}", args.method);
        println!("Total Requests:     {}", total);
        println!("Concurrency:        {}", args.concurrency);
        println!("Success:            {}", success);
        println!("Failed:             {}", failure);
        println!("Total Time:         {:.3?}", duration);
        println!("Requests/sec:       {:.3}", rps);
        println!("Fastest:            {} ms", hist.min());
        println!("Slowest:            {} ms", hist.max());
        println!("Average:            {:.0} ms", hist.mean());
        println!(
            "Latency_p{}:        {} ms",
            args.percentile,
            hist.value_at_percentile(args.percentile as f64)
        );
        println!("=======================================================\n");
    }

    if let Some(path) = &args.output {
        let mut file =
            File::create(path).with_context(|| format!("Failed to create file: {}", path))?;
        writeln!(file, "# jolt benchmark result")?;
        writeln!(file, "# url: {}", args.url)?;
        writeln!(file, "# method: {}", args.method)?;
        writeln!(file, "# total_requests: {}", total)?;
        writeln!(file, "# success: {}", success)?;
        writeln!(file, "# failures: {}", failure)?;
        writeln!(file, "# duration_sec: {:.6}", duration.as_secs_f64())?;
        writeln!(file, "# rps: {:.2}", rps)?;
        writeln!(file, "# latency_min_ms: {}", hist.min())?;
        writeln!(file, "# latency_max_ms: {}", hist.max())?;
        writeln!(file, "# latency_mean_ms: {:.2}", hist.mean())?;
        writeln!(
            file,
            "# latency_p{}_ms: {}",
            args.percentile,
            hist.value_at_percentile(args.percentile as f64)
        )?;
        writeln!(file, "# latency_p99_ms: {}", hist.value_at_percentile(99.0))?;
        if let Some(reqs) = args.requests {
            writeln!(file, "# target_requests: {}", reqs)?;
        }
        if let Some(dur) = args.duration {
            writeln!(file, "# target_duration_sec: {:.6}", dur.as_secs_f64())?;
        }
        writeln!(file, "#")?;
        writeln!(file, "latency_ms,frequency")?;
        for v in hist.iter_recorded() {
            writeln!(file, "{},{}", v.value_iterated_to(), v.count_at_value())?;
        }
        eprintln!("[info] Benchmark result saved to: {}", path);
    };

    Ok(())
}
