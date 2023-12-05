// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    // Directory that the proto files are stored in. If the local_dependencies
    // directory exists, we're building from an sdist package, in which case
    // the proto files should have been copied to a local directory.
    let input_paths = if std::path::Path::new("local_dependencies").exists() {
        vec!["proto"]
    } else {
        assert!(
            std::path::Path::new("..")
                .join("substrait")
                .join("proto")
                .exists(),
            "Could not find (git-root)/substrait/proto. Did you check out submodules?"
        );
        vec!["../proto", "../substrait/proto"]
    };

    // Ensure above path is relative to the Cargo.toml directory.
    let pwd = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let input_paths = input_paths.iter().map(|p| PathBuf::from(&pwd).join(p));

    // Output directory for protoc. This is a temporary directory: it will be
    // completely deleted and then reconstructed. Afterward, the build script
    // will patch the files in here and then move them to python_out.
    let intermediate_path = "protoc_out";

    // Where the final Python files will be moved to.
    let output_path = "substrait_validator";

    // The Python module prefix to patch into use statements of the files
    // generated by protobuf.
    let python_prefix = "substrait_validator.";

    // Canonicalize all paths to prevent ambiguity.
    let input_paths = input_paths
        .map(|p| dunce::canonicalize(p).unwrap())
        .collect::<Vec<_>>();
    let workdir = std::env::current_dir().unwrap();
    let intermediate_path = workdir.join(intermediate_path);
    let output_path = workdir.join(output_path);

    // Gather all .proto files.
    let proto_files = input_paths
        .iter()
        .flat_map(|p| {
            WalkDir::new(p)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension() == Some(OsStr::new("proto"))
                        && e.metadata().unwrap().is_file()
                })
                .map(|e| dunce::canonicalize(e.into_path()).unwrap())
        })
        .collect::<Vec<_>>();

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // Clean and recreate output directory.
    fs::remove_dir_all(&intermediate_path).ok();
    fs::create_dir_all(&intermediate_path).expect("failed to create protoc output directory");

    // Run protoc.
    if cfg!(not(target_os = "windows")) {
        std::env::set_var("PROTOC", protobuf_src::protoc());
        std::env::set_var("PROTOC_INCLUDE", protobuf_src::include());
    }
    let mut cmd = Command::new(env::var("PROTOC").unwrap_or("protoc".to_string()));
    for input_path in input_paths.iter() {
        let mut proto_path_arg = OsString::new();
        proto_path_arg.push("--proto_path=");
        proto_path_arg.push(input_path);
        cmd.arg(proto_path_arg);
    }
    let mut python_out_arg = OsString::new();
    python_out_arg.push("--python_out=");
    python_out_arg.push(&intermediate_path);
    cmd.arg(python_out_arg);
    cmd.args(proto_files.iter());
    let output = cmd.output().expect("failed to run protoc");
    if !output.status.success() {
        eprintln!("cmd: {:?}", cmd.get_program());
        for arg in cmd.get_args() {
            eprintln!("arg: {:?}", arg);
        }
        panic!("{:?}", output);
    }

    // Gather all Python files generated by protoc.
    let intermediate_files: Vec<_> = WalkDir::new(&intermediate_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension() == Some(OsStr::new("py")) && e.metadata().unwrap().is_file()
        })
        .map(|e| dunce::canonicalize(e.into_path()).unwrap())
        .collect();

    // Patch the files.
    let mut output_dirs = HashSet::new();
    for intermediate_file in intermediate_files {
        // Determine the output filename.
        let output_file = output_path.join(
            intermediate_file
                .strip_prefix(&intermediate_path)
                .expect("intermediate file is not based in the expected directory"),
        );

        // Determine the output directory.
        let mut path = output_file.to_path_buf();
        path.pop();

        // Ensure that the directory exists, and create an __init__.py for it
        // if we haven't already.
        let mut path = output_file.to_path_buf();
        path.pop();
        if output_dirs.insert(path.clone()) {
            fs::create_dir_all(&path).expect("failed to create output directory");
            path.push("__init__.py");
            fs::File::create(path).expect("failed to create __init__.py");
        }

        // Copy and patch the file.
        let intermediate =
            fs::File::open(&intermediate_file).expect("failed to open intermediate file");
        let mut output = fs::File::create(&output_file).expect("failed to create output file");
        for line in BufReader::new(intermediate).lines() {
            let line = line.expect("failed to read from intermediate file");
            let line = if line.starts_with("from ") && !line.starts_with("from google") {
                format!("from {}{}", python_prefix, &line[5..])
            } else {
                line
            };
            writeln!(output, "{}", line).unwrap();
        }
    }

    // https://pyo3.github.io/pyo3/v0.17.3/building_and_distribution.html#macos
    pyo3_build_config::add_extension_module_link_args();
}
