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

//! PodFlow Adapters CLI - Adapter testing and management tool.
//!
//! Skeleton entry point. Full adapter subcommands are added in later PRs.

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = "podflow-adapters";

fn print_help() {
    println!("{} v{}", NAME, VERSION);
    println!("PodFlow Adapters CLI - test and manage collector adapters");
    println!();
    println!("Usage: {} [OPTIONS] [COMMAND]", NAME);
    println!();
    println!("Options:");
    println!("  -h, --help       Print help information");
    println!("  -v, --version    Print version information");
    println!();
    println!("Commands (planned):");
    println!("  list             List available adapters");
    println!("  test             Test an adapter");
    println!("  validate         Validate an adapter configuration");
}

fn print_version() {
    println!("{} v{}", NAME, VERSION);
}

fn main() {
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

    print_help();
}
