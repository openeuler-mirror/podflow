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

//! PodFlow CLI - Command-line client for PodFlow diagnostic server.
//!
//! Skeleton implementation. Subcommands and full functionality will be
//! added in subsequent PRs.

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = "podflow-cli";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{} v{}", NAME, VERSION);
                println!("Command-line client for PodFlow diagnostic server");
                println!();
                println!("Usage: {} [OPTIONS] <COMMAND>", NAME);
                return;
            }
            "-v" | "--version" => {
                println!("{} v{}", NAME, VERSION);
                return;
            }
            _ => {}
        }
    }

    eprintln!("CLI not yet implemented. Use --help for usage.");
}
