# substrait-validator-build

Shared build utilities for the substrait-validator project.

## Overview

This crate provides common build script functionality used by both the `rs` (Rust) and `py` (Python) modules of substrait-validator.

## Features

### Core Functionality

- **File Synchronization**: Efficiently copies files from source to destination, only updating when source is newer
- **Proto File Discovery**: Recursively finds all `.proto` files in a directory tree
- **Substrait Sync**: Provides configuration and methods for synchronizing files from the substrait submodule
- **Git Build Detection**: Determines whether building from git repository or from a packaged source distribution

## Usage

### In Rust Build Scripts (build.rs)

```rust
use substrait_validator_build::{find_proto_files, SubstraitSync};
use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let manifest_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());

    // Create sync configuration
    let sync = SubstraitSync::new(manifest_dir, "src/resources");

    // Synchronize files if building from git
    if sync.is_git_build() {
        sync.sync_text_schemas()?;
        sync.sync_substrait_proto_files()?;
        sync.sync_validator_proto_files()?;
        // Add any module-specific sync operations here
    }

    // Find proto files for compilation
    let proto_files = find_proto_files(&sync.resource_dir.join("proto"));

    // ... compile proto files with prost or protoc

    Ok(())
}
```


## Features

## Features

### `protoc`

When enabled, provides access to a vendored protobuf compiler via the `vendored_protoc()` function. This ensures consistent protoc versions across different build environments.

## Design Philosophy

This crate contains only the **shared** build utilities that are used by both the `rs` and `py` modules:

- File synchronization logic (`synchronize()`)
- Proto file discovery (`find_proto_files()`)
- Git build detection (`SubstraitSync::is_git_build()`)
- Common sync operations (`SubstraitSync::sync_text_schemas()`, `sync_substrait_proto_files()`, `sync_validator_proto_files()`)

Module-specific build logic (such as Python-specific protoc execution and import patching, or Rust-specific version detection) is kept in the respective module's `build.rs` file to maintain clear separation of concerns.
