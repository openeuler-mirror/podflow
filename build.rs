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

use std::io::Result;

fn main() -> Result<()> {
    // Compile collector protobuf files
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/collector.proto"], &["proto"])?;

    // Compile NRI protobuf files (containerd official protocol)
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/nri.proto"], &["proto"])?;

    // Tell cargo to re-run when proto files change
    println!("cargo:rerun-if-changed=proto/collector.proto");
    println!("cargo:rerun-if-changed=proto/nri.proto");

    Ok(())
}