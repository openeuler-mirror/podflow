/*
Copyright 2026 KylinSoft  Co., Ltd.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! PodFlow Collector Daemon
//!
//! Standalone collector daemon that runs privileged collection tasks
//! (bpftrace, NRI, etc.) and forwards evidence to the main service.
//!
//! Requires the `nri-grpc` feature.

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = "podflow-collector";

fn print_help() {
    println!("{} v{}", NAME, VERSION);
    println!("PodFlow Collector Daemon");
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

    eprintln!("{} not yet implemented. Use --help for usage.", NAME);
}
