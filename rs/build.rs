// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use substrait_validator_build::{find_proto_files, SubstraitSync};

/// Tries to determine and write the Substrait submodule version.
///
/// This can fail for various reasons at various stages of the build/distribution
/// methods, so the generated file is also checked in.
fn write_substrait_version(sync: &SubstraitSync) -> Result<()> {
    let substrait_version = std::process::Command::new("git")
        .args(["describe", "--dirty", "--tags"])
        .current_dir(&sync.substrait_git_dir)
        .output();

    if let Ok(substrait_version) = substrait_version {
        if substrait_version.status.success() {
            let substrait_version = String::from_utf8_lossy(&substrait_version.stdout)
                .trim()
                .to_string();
            let substrait_version = substrait_version
                .strip_prefix('v')
                .unwrap_or(&substrait_version);
            fs::write(
                sync.resource_dir.join("substrait-version"),
                substrait_version,
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // Determine the directory of Cargo.toml for this crate.
    let manifest_dir =
        PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    // Create the sync configuration
    let sync = SubstraitSync::new(manifest_dir, "src/resources");

    // Synchronize files if building from git
    if sync.is_git_build() {
        sync.sync_text_schemas()?;
        sync.sync_substrait_proto_files()?;
        sync.sync_validator_proto_files()?;
        write_substrait_version(&sync)?;
    }

    #[cfg(feature = "protoc")]
    // Use vendored protobuf compiler if requested.
    std::env::set_var("PROTOC", substrait_validator_build::vendored_protoc());

    // Find all protobuf files in our resource directory. We just synchronized
    // these files if we're building from git.
    let proto_path = sync.resource_dir.join("proto");
    let proto_files: Vec<_> = find_proto_files(&proto_path);

    // Compile the protobuf files using prost.
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");
    config.type_attribute(".", "#[allow(deprecated)]");
    config.disable_comments([
        "substrait.AggregateRel.Measure.filter",
        "substrait.Type.Parameter.data_type",
    ]);
    config.compile_protos(&proto_files, &[&proto_path.display().to_string()])?;

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    Ok(())
}
