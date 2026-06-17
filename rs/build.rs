// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Error;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

/// Copies the file at src_tree/path to dest_tree/path if it's newer.
/// Automatically creates parent directories in dest as needed.
fn synchronize(src_tree: &Path, dest_tree: &Path, path: &Path) -> Result<()> {
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
    std::fs::copy(&src, &dest)?;

    Ok(())
}

/// Returns all protobuf files in the given directory.
fn find_proto_files(proto_path: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(proto_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("proto")) && e.metadata().unwrap().is_file()
        })
        .map(|e| e.into_path())
        .collect()
}

/// Returns all YAML files in the given directory (non-recursive).
fn find_yaml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = walkdir::WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let ext = e.path().extension();
            (ext == Some(OsStr::new("yaml")) || ext == Some(OsStr::new("yml")))
                && e.metadata().map(|m| m.is_file()).unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect();
    // Sort for deterministic output across builds.
    files.sort();
    files
}

/// Scans a YAML extension file for its top-level `urn:` field and returns its
/// value. The standard extension files always declare this on a line of their
/// own, so a simple line scan suffices and avoids pulling a YAML parser into
/// the build dependencies.
fn extract_urn(yaml_path: &Path) -> Option<String> {
    let contents = fs::read_to_string(yaml_path).ok()?;
    for line in contents.lines() {
        let line = line.trim_end();
        if let Some(rest) = line.strip_prefix("urn:") {
            let urn = rest.trim().trim_matches(|c| c == '"' || c == '\'').trim();
            if !urn.is_empty() {
                return Some(urn.to_string());
            }
        }
    }
    None
}

/// Generates a Rust source file (in OUT_DIR) containing a static slice that
/// maps each bundled standard extension URN to the bytes of its YAML file.
/// This is included by the URN resolver to provide an offline, batteries-
/// included registry for the `extension:io.substrait:*` extensions.
fn generate_stdlib_registry(extensions_dir: &Path) -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest = out_dir.join("stdlib_extension_registry.rs");

    let mut entries = String::new();
    for yaml_file in find_yaml_files(extensions_dir) {
        println!("cargo:rerun-if-changed={}", yaml_file.display());
        let Some(urn) = extract_urn(&yaml_file) else {
            // Skip files without a recognizable urn field.
            continue;
        };
        entries.push_str(&format!(
            "    ({urn:?}, include_bytes!({path:?})),\n",
            urn = urn,
            path = yaml_file.display().to_string(),
        ));
    }

    let source = format!(
        "/// Maps standard extension URNs to the bytes of their bundled YAML \
         file.\npub static STDLIB_EXTENSIONS: &[(&str, &[u8])] = &[\n{entries}];\n"
    );
    fs::write(&dest, source)?;
    Ok(())
}

fn main() -> Result<()> {
    // Determine the directory of Cargo.toml for this crate.
    let manifest_dir =
        PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let resource_dir = manifest_dir.join("src/resources");

    // Determine whether we're building from the git repository or from a
    // crate file. If the former, we first synchronize our src/resources
    // directory with the rest of the repository.
    if manifest_dir.join("in-git-repo").exists() {
        let validator_git_dir = manifest_dir.join("..");
        let substrait_git_dir = validator_git_dir.join("substrait");

        // Give a proper error message if submodules aren't checked out.
        assert!(
            substrait_git_dir.join("proto").exists(),
            "Could not find (git-root)/substrait/proto. Did you check out submodules?"
        );

        // Synchronize the YAML extension file schema.
        synchronize(
            &substrait_git_dir,
            &resource_dir,
            &PathBuf::from("text/simple_extensions_schema.yaml"),
        )?;

        // Synchronize the protobuf files from the main repository.
        for proto_file in find_proto_files(&substrait_git_dir.join("proto")) {
            synchronize(
                &substrait_git_dir,
                &resource_dir,
                proto_file
                    .strip_prefix(&substrait_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }

        // Synchronize the validator-specific protobuf files.
        for proto_file in find_proto_files(&validator_git_dir.join("proto")) {
            synchronize(
                &validator_git_dir,
                &resource_dir,
                proto_file
                    .strip_prefix(&validator_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }

        // Synchronize the standard extension YAML files so they can be bundled
        // into the validator and resolved offline by their URN.
        for yaml_file in find_yaml_files(&substrait_git_dir.join("extensions")) {
            synchronize(
                &substrait_git_dir,
                &resource_dir,
                yaml_file
                    .strip_prefix(&substrait_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }

        // Try to determine Substrait submodule version and write it to a
        // resource file. This can fail for various reasons at various stages
        // of the various build/distribution methods, so the generated file
        // is also checked in; this pretty much just makes it hard to forget
        // to update it.
        let substrait_version = std::process::Command::new("git")
            .args(["describe", "--dirty", "--tags"])
            .current_dir(&substrait_git_dir)
            .output();
        if let Ok(substrait_version) = substrait_version {
            if substrait_version.status.success() {
                let substrait_version = String::from_utf8_lossy(&substrait_version.stdout)
                    .trim()
                    .to_string();
                let substrait_version = substrait_version
                    .strip_prefix('v')
                    .unwrap_or(&substrait_version);
                fs::write(resource_dir.join("substrait-version"), substrait_version)
                    .expect("failed to write substrait submodule version file");
            }
        }
    }

    #[cfg(feature = "protoc")]
    // Use vendored protobuf compiler if requested.
    std::env::set_var("PROTOC", protobuf_src::protoc());

    // Generate the bundled standard-extension registry (URN -> YAML bytes)
    // from the synchronized resource directory. This runs regardless of
    // whether we're building from git, so that crate builds use the checked-in
    // copies under src/resources/extensions.
    generate_stdlib_registry(&PathBuf::from(&resource_dir).join("extensions"))?;

    // Find all protobuf files in our resource directory. We just synchronized
    // these files if we're building from git.
    let proto_path = PathBuf::from(&resource_dir).join("proto");
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
