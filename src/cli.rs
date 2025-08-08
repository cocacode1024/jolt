use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "jolt",
    about = "A simple HTTP benchmarking tool.",
    override_usage = "jolt -u <URL> (-n <REQUESTS> | -t <DURATION>)"
)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["requests", "duration"])
))]
pub struct Args {
    #[arg(short = 'u', long, required = true, help = "URL to test")]
    pub url: String,

    #[arg(short = 'X', long, default_value = "GET", help = "HTTP method to use")]
    pub method: String,

    #[arg(short = 'd', long, help = "Request body to send")]
    pub body: Option<String>,

    #[arg(short = 'H', long = "header", help = "Request headers to send")]
    pub headers: Vec<String>,

    #[arg(
        short = 'n',
        long,
        conflicts_with = "duration",
        help = "Number of requests to send"
    )]
    pub requests: Option<usize>,

    #[arg(
        short = 't',
        long,
        conflicts_with = "requests",
        help = "Duration of the test"
    )]
    pub duration: Option<humantime::Duration>,

    #[arg(
        short = 'c',
        long,
        default_value = "1",
        help = "Number of concurrent requests"
    )]
    pub concurrency: usize,

    #[arg(short = 'p', long, default_value = "99", help = "Percentile to report")]
    pub percentile: u32,

    #[arg(short = 'j', long, help = "Output JSON format")]
    pub json: bool,

    #[arg(short = 'o', long, help = "Output file path")]
    pub output: Option<String>,
}
