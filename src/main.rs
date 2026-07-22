//! PodFlow - Container Intelligent Fault Diagnosis Plugin
//!
//! Main program entry point.

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn print_help() {
    println!("{} v{}", NAME, VERSION);
    println!("Container Intelligent Fault Diagnosis Plugin");
    println!();
    println!("Usage: {} [OPTIONS]", NAME);
    println!();
    println!("Options:");
    println!("  -h, --help       Print help information");
    println!("  -v, --version    Print version information");
}

fn print_version() {
    println!("{} v{}", NAME, VERSION);
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                print_version();
                return;
            }
            _ => {}
        }
    }

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("PodFlow starting up...");

    // TODO: Initialize runtime and start server
    eprintln!("Server mode not yet implemented. Use --help for usage.");
}