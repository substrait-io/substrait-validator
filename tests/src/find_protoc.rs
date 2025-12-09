// SPDX-License-Identifier: Apache-2.0

//! Prost has some magic for finding the path to protoc, so let's use that in
//! the Python code as well...

use std::path::PathBuf;

fn main() {
    let protoc_path: PathBuf;
    #[cfg(feature = "protoc")]
    {
        // Use vendored protobuf compiler if requested.
        protoc_path = protobuf_src::protoc();
        println!("cargo:warning=Using vendored protoc: {protoc_path:?}");
    }
    #[cfg(not(feature = "protoc"))]
    {
        protoc_path = prost_build::protoc_from_env();
    }

    println!("{}", protoc_path.display());
}
