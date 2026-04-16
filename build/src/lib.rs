// SPDX-License-Identifier: Apache-2.0

//! Shared build utilities for substrait-validator build scripts.
//!
//! This crate provides common functionality used by both the `rs` and `py`
//! build scripts to prepare files from the substrait submodule and proto
//! directories.

use std::ffi::OsStr;
use std::fs;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};

/// Copies the file at src_tree/path to dest_tree/path if it's newer.
/// Automatically creates parent directories in dest as needed.
///
/// This function also informs cargo that the build should re-run if the
/// source file changes.
pub fn synchronize(src_tree: &Path, dest_tree: &Path, path: &Path) -> Result<()> {
    // Construct paths.
    let src = src_tree.join(path);
    let dest = dest_tree.join(path);

    // Inform cargo that we should re-run if src changes.
    println!("cargo:rerun-if-changed={}", src.display());

    // Ensure that the source exists.
    if !src.exists() {
        return Err(Error::other("source file not found"));
    }

    // Check if destination already exists.
    if dest.exists() {
        // Check if it's newer than or equally old as the source; in that case
        // we don't have to copy it again.
        if dest.metadata()?.modified()? >= src.metadata()?.modified()? {
            return Ok(());
        }
    } else {
        // Check if the destination directory exists, and if not, create it.
        if let Some(parent) = dest.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        }
    }

    // Copy the file.
    fs::copy(&src, &dest)?;

    Ok(())
}

/// Returns all protobuf files in the given directory.
///
/// This function recursively walks the directory tree and collects all files
/// with a `.proto` extension.
pub fn find_proto_files(proto_path: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("proto")) && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect()
}

/// Configuration for synchronizing substrait files.
///
/// This struct encapsulates the common logic for determining source directories
/// and synchronizing files from the substrait submodule.
pub struct SubstraitSync {
    /// The manifest directory (where Cargo.toml is located)
    pub manifest_dir: PathBuf,
    /// The validator git directory (parent of manifest_dir)
    pub validator_git_dir: PathBuf,
    /// The substrait git directory (substrait submodule)
    pub substrait_git_dir: PathBuf,
    /// The resource/destination directory
    pub resource_dir: PathBuf,
}

impl SubstraitSync {
    /// Creates a new SubstraitSync configuration.
    ///
    /// # Arguments
    /// * `manifest_dir` - The directory containing Cargo.toml (from CARGO_MANIFEST_DIR)
    /// * `resource_subdir` - The subdirectory within manifest_dir for resources (e.g., "src/resources")
    ///
    /// This function does not panic if the substrait submodule is missing, as it may be
    /// building from a source distribution where files are already in place.
    pub fn new(manifest_dir: PathBuf, resource_subdir: &str) -> Self {
        let validator_git_dir = manifest_dir.join("..");
        let substrait_git_dir = validator_git_dir.join("substrait");
        let resource_dir = manifest_dir.join(resource_subdir);

        Self {
            manifest_dir,
            validator_git_dir,
            substrait_git_dir,
            resource_dir,
        }
    }

    /// Checks if we're building from the git repository (vs from a crate file).
    ///
    /// Returns true if the substrait submodule directory exists.
    pub fn is_git_build(&self) -> bool {
        self.substrait_git_dir.join("proto").exists()
    }

    /// Synchronizes YAML schema files from substrait/text.
    pub fn sync_text_schemas(&self) -> Result<()> {
        synchronize(
            &self.substrait_git_dir,
            &self.resource_dir,
            &PathBuf::from("text/simple_extensions_schema.yaml"),
        )?;
        synchronize(
            &self.substrait_git_dir,
            &self.resource_dir,
            &PathBuf::from("text/dialect_schema.yaml"),
        )?;
        Ok(())
    }

    /// Synchronizes protobuf files from substrait/proto.
    pub fn sync_substrait_proto_files(&self) -> Result<()> {
        for proto_file in find_proto_files(&self.substrait_git_dir.join("proto")) {
            synchronize(
                &self.substrait_git_dir,
                &self.resource_dir,
                proto_file
                    .strip_prefix(&self.substrait_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }
        Ok(())
    }

    /// Synchronizes validator-specific protobuf files from proto/.
    pub fn sync_validator_proto_files(&self) -> Result<()> {
        for proto_file in find_proto_files(&self.validator_git_dir.join("proto")) {
            synchronize(
                &self.validator_git_dir,
                &self.resource_dir,
                proto_file
                    .strip_prefix(&self.validator_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }
        Ok(())
    }
}

#[cfg(feature = "protoc")]
/// Returns the path to the vendored protoc binary.
pub fn vendored_protoc() -> PathBuf {
    protobuf_src::protoc()
}
