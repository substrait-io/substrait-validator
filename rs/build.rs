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

/// Returns the version that the given package is resolved to in the given
/// Cargo.lock contents, if it appears there.
///
/// The lockfile is scanned rather than parsed as TOML to avoid a build
/// dependency on a TOML parser; this relies only on cargo emitting `name`
/// before `version` within each `[[package]]` stanza, which it has always
/// done.
fn locked_package_version(lock: &str, package: &str) -> Option<String> {
    let mut in_package = false;
    for line in lock.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("name = ") {
            in_package = name.trim_matches('"') == package;
        } else if in_package {
            if let Some(version) = line.strip_prefix("version = ") {
                return Some(version.trim_matches('"').to_string());
            }
        }
    }
    None
}

/// Determines the version of the Substrait specification that this build
/// targets, which is the version of the `substrait-*` crates that provide the
/// specification's protobuf definitions, extension files, and grammar.
///
/// These crates are published from the specification with matching version
/// numbers, so they must be pinned to the same version; disagreement would
/// mean the validator checks plans against a mixture of specification
/// versions, so it is treated as an error.
fn substrait_version(lock_path: &Path) -> Result<String> {
    const SPEC_CRATES: &[&str] = &["substrait-prost", "substrait-extensions", "substrait-antlr"];
    println!("cargo:rerun-if-changed={}", lock_path.display());
    let lock = fs::read_to_string(lock_path)?;
    let versions = SPEC_CRATES
        .iter()
        .map(|package| {
            locked_package_version(&lock, package).ok_or_else(|| {
                Error::other(format!(
                    "package {package} not found in {}",
                    lock_path.display()
                ))
            })
        })
        .collect::<Result<Vec<_>>>()?;
    if versions.iter().any(|version| version != &versions[0]) {
        let pins = SPEC_CRATES
            .iter()
            .zip(&versions)
            .map(|(package, version)| format!("{package} {version}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(Error::other(format!(
            "the crates providing the Substrait specification must all be \
             pinned to the same version, but found {pins}"
        )));
    }
    Ok(versions.into_iter().next().expect("no spec crates"))
}

/// Writes contents to the file at path, unless it already has those contents.
///
/// Used for files that live in the source tree: rewriting them unconditionally
/// would update their modification time on every build, needlessly
/// invalidating anything that depends on them.
fn write_if_changed(path: &Path, contents: &str) -> Result<()> {
    if fs::read_to_string(path).ok().as_deref() != Some(contents) {
        fs::write(path, contents)?;
    }
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

        // Synchronize the validator-specific protobuf files (the
        // `substrait.validator` package) from the repository root into
        // src/resources, so that they are shipped with this crate. The core
        // Substrait protobuf files are neither compiled nor vendored here: we
        // consume their pre-generated types from the `substrait-prost` crate
        // and synthesize our introspection trait impls from its embedded
        // descriptor (see the prost_meta module), and the Python bindings take
        // theirs from the `substrait-protobuf` package.
        for proto_file in find_proto_files(&validator_git_dir.join("proto")) {
            synchronize(
                &validator_git_dir,
                &resource_dir,
                proto_file
                    .strip_prefix(&validator_git_dir)
                    .expect("failed to strip prefix"),
            )?;
        }

        // Write the version of the Substrait specification that this build
        // targets to a resource file, taking it from the resolved version of
        // the crates that provide the specification. The generated file is
        // also checked in, because build scripts do not run during packaging:
        // whatever this write produces is what `cargo package` ships (see the
        // release process in the crate documentation).
        write_if_changed(
            &resource_dir.join("substrait-version"),
            &substrait_version(&validator_git_dir.join("Cargo.lock"))?,
        )?;
    }

    #[cfg(feature = "protoc")]
    // Use vendored protobuf compiler if requested.
    std::env::set_var("PROTOC", protobuf_src::protoc());

    // Compile the validator-specific protobuf files (the `substrait.validator`
    // package) using prost, applying our ProtoMeta derive to obtain the
    // introspection traits. These types are not provided by substrait-prost.
    // The include path is the whole proto resource directory so that the
    // validator protos can import each other and the google/protobuf
    // well-known types (which prost-build provides); they do not import any
    // core `substrait` protos.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let proto_path = PathBuf::from(&resource_dir).join("proto");
    let validator_proto_files = find_proto_files(&proto_path.join("substrait/validator"));
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");
    config.type_attribute(".", "#[allow(deprecated)]");
    // `enable_type_names` gives the validator types `prost::Name` impls (so the
    // ProtoMeta derive can obtain authoritative type names), and the descriptor
    // set is embedded so the derive's `ReflectMessage`/`ProtoEnum` impls can
    // resolve their descriptors at runtime (see `proto::DESCRIPTOR_POOL`).
    config.enable_type_names();
    config.file_descriptor_set_path(out_dir.join("file_descriptor_set.bin"));
    config.compile_protos(&validator_proto_files, &[&proto_path.display().to_string()])?;

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &validator_proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // Generate the introspection trait impls (InputNode/ProtoMessage/
    // ProtoOneOf/ProtoEnum) for the foreign substrait-prost types by walking
    // that crate's embedded FileDescriptorSet for the type names and structure.
    // The impls are thin: unknown-field handling defers to runtime reflection
    // (parse_proto_message_unknown), so this generator no longer reconstructs
    // prost's field boxing/naming. Emitting impls for the foreign types is
    // legal by the orphan rule, as the traits are local to this crate.
    let meta_source = prost_meta::generate(substrait_prost::FILE_DESCRIPTOR_SET);
    fs::write(out_dir.join("substrait_prost_meta.rs"), meta_source)?;

    Ok(())
}

/// Generates the validator's protobuf introspection trait impls
/// (`InputNode`/`ProtoMessage`/`ProtoOneOf`/`ProtoEnum`) for the `substrait`
/// and `substrait.extensions` types provided by the `substrait-prost` crate.
///
/// substrait-prost ships plain `prost`-generated types; a derive cannot be
/// applied to a foreign crate's types, so we emit these impls here instead
/// (legal by the orphan rule, as the traits are local to this crate). We walk
/// the [`FileDescriptorSet`](prost_types::FileDescriptorSet) that substrait-prost
/// embeds only to learn the set of types, their protobuf names, and their
/// oneof/enum variants; the emitted impls are uniform boilerplate that defers
/// unknown-field handling to runtime reflection
/// (`crate::parse::traversal::parse_proto_message_unknown`), which leans on the
/// `prost::Name` + `prost_reflect::ReflectMessage` impls substrait-prost's
/// `reflect` feature provides. Consequently no reconstruction of prost's field
/// boxing is needed; the only prost naming rules reproduced here are the
/// module/variant identifier casing used to *name* each target type.
mod prost_meta {
    use heck::{ToSnakeCase, ToUpperCamelCase};
    use prost::Message;
    use prost_types::{
        DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorSet,
    };
    use std::fmt::Write;

    /// Rust keywords that prost escapes as raw identifiers (`r#ident`) when a
    /// protobuf name maps onto them. We only need this for module and field
    /// identifiers; protobuf type and enum-variant names are UpperCamelCase and
    /// effectively never collide (the rare `Self`/`Super` cases do not occur in
    /// the Substrait schema).
    const RUST_KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "abstract", "become", "box", "do", "final", "macro", "override",
        "priv", "typeof", "unsized", "virtual", "yield", "async", "await", "try",
    ];

    /// Escapes a snake_case identifier as a raw identifier if it is a Rust
    /// keyword, matching prost's behavior for module and field names.
    fn escape(ident: &str) -> String {
        if RUST_KEYWORDS.contains(&ident) {
            format!("r#{ident}")
        } else {
            ident.to_string()
        }
    }

    /// Reproduces prost-build's `strip_enum_prefix` + `to_upper_camel`: the Rust
    /// variant ident is the UpperCamelCase value name with the UpperCamelCase
    /// enum name stripped as a prefix, unless that would leave an empty string
    /// or one starting with a digit.
    fn enum_variant_ident(enum_name: &str, value_name: &str) -> String {
        let prefix = enum_name.to_upper_camel_case();
        let camel = value_name.to_upper_camel_case();
        let stripped = camel.strip_prefix(&prefix).unwrap_or(&camel);
        match stripped.chars().next() {
            Some(c) if !c.is_ascii_digit() => stripped.to_string(),
            _ => camel,
        }
    }

    /// Entry point: produces the full generated Rust source.
    pub fn generate(descriptor_bytes: &[u8]) -> String {
        let fds = FileDescriptorSet::decode(descriptor_bytes)
            .expect("failed to decode substrait-prost FileDescriptorSet");

        let mut out = String::new();
        // Note: this file is `include!`d inside a module already annotated with
        // `#[allow(deprecated)]` (for the deprecated `from_i32`), so no inner
        // attribute is emitted here (inner attributes are illegal mid-module).
        out.push_str(
            "// @generated by build.rs (prost_meta) from substrait-prost's \
             FileDescriptorSet.\n// Introspection trait impls for the \
             substrait-prost types. Do not edit.\n\n",
        );
        for file in &fds.file {
            let pkg = file.package();
            let base = match pkg {
                "substrait" => "::substrait_prost".to_string(),
                "substrait.extensions" => "::substrait_prost::extensions".to_string(),
                // Skip the validator package (generated locally) and the
                // google.protobuf well-known types (hand-implemented in
                // input/proto.rs and input/traits.rs).
                _ => continue,
            };
            let fqn_prefix = format!(".{pkg}");
            for msg in &file.message_type {
                emit_message(&mut out, msg, &base, &fqn_prefix);
            }
            for en in &file.enum_type {
                emit_enum(&mut out, en, &base, &fqn_prefix);
            }
        }
        out
    }

    /// Emits all impls for a message and recurses into its nested types.
    ///
    /// * `mod_path` is the Rust path of the module containing this message's
    ///   type (e.g. `::substrait_prost` or `::substrait_prost::r#type`).
    /// * `fqn_prefix` is the protobuf FQN of the containing scope.
    fn emit_message(out: &mut String, msg: &DescriptorProto, mod_path: &str, fqn_prefix: &str) {
        if msg
            .options
            .as_ref()
            .and_then(|o| o.map_entry)
            .unwrap_or(false)
        {
            return;
        }
        let name = msg.name();
        let fqn = format!("{fqn_prefix}.{name}");
        let proto_type = &fqn[1..]; // strip leading dot
        let type_path = format!("{mod_path}::{}", name.to_upper_camel_case());
        // The module that holds this message's nested types and oneof enums.
        let child_mod = format!("{mod_path}::{}", escape(&name.to_snake_case()));

        emit_message_impls(out, &type_path, proto_type);

        // Emit oneof enum impls (real oneofs only; synthetic proto3-optional
        // oneofs do not produce a generated enum).
        for (idx, oneof) in msg.oneof_decl.iter().enumerate() {
            if is_synthetic_oneof(msg, idx) {
                continue;
            }
            let enum_path = format!("{child_mod}::{}", oneof.name().to_upper_camel_case());
            emit_oneof_impls(out, msg, idx, &enum_path);
        }

        // Nested enums live in the child module.
        for en in &msg.enum_type {
            emit_enum(out, en, &child_mod, &fqn);
        }

        // Recurse into nested messages.
        for nested in &msg.nested_type {
            emit_message(out, nested, &child_mod, &fqn);
        }
    }

    /// Whether oneof `idx` of `msg` is a synthetic oneof generated for a proto3
    /// `optional` field (prost renders these as a plain `Option<T>` field, not
    /// as a generated enum).
    fn is_synthetic_oneof(msg: &DescriptorProto, idx: usize) -> bool {
        msg.field
            .iter()
            .any(|f| f.oneof_index == Some(idx as i32) && f.proto3_optional())
    }

    /// Emits the `ProtoMessage` and `InputNode` impls for a message. The
    /// `parse_unknown` body defers to `parse_proto_message_unknown`, which
    /// enumerates unrecognized fields at runtime via `prost-reflect`, so no
    /// per-field code (and no boxing analysis) is emitted here.
    fn emit_message_impls(out: &mut String, type_path: &str, proto_type: &str) {
        let _ = write!(
            out,
            "impl crate::input::traits::ProtoMessage for {type_path} {{\n\
             \x20   fn proto_message_type() -> &'static str {{ {proto_type:?} }}\n\
             }}\n\
             impl crate::input::traits::InputNode for {type_path} {{\n\
             \x20   fn type_to_node() -> crate::output::tree::Node {{\n\
             \x20       crate::output::tree::NodeType::ProtoMessage({proto_type:?}.to_string()).into()\n\
             \x20   }}\n\
             \x20   fn data_to_node(&self) -> crate::output::tree::Node {{\n\
             \x20       crate::output::tree::NodeType::ProtoMessage({proto_type:?}.to_string()).into()\n\
             \x20   }}\n\
             \x20   fn oneof_variant(&self) -> Option<&'static str> {{ None }}\n\
             \x20   fn parse_unknown(&self, y: &mut crate::parse::context::Context<'_>) -> bool {{\n\
             \x20       crate::parse::traversal::parse_proto_message_unknown(self, y)\n\
             \x20   }}\n\
             }}\n",
        );
    }

    /// Emits the `ProtoOneOf` and `InputNode` impls for a oneof enum, mirroring
    /// `proto_meta_derive_oneof`.
    fn emit_oneof_impls(out: &mut String, msg: &DescriptorProto, idx: usize, enum_path: &str) {
        let members: Vec<&FieldDescriptorProto> = msg
            .field
            .iter()
            .filter(|f| f.oneof_index == Some(idx as i32) && !f.proto3_optional())
            .collect();

        let mut variant_arms = String::new();
        let mut node_arms = String::new();
        let mut unknown_arms = String::new();
        for member in &members {
            let variant = member.name().to_upper_camel_case();
            let proto_name = member.name().to_snake_case();
            let _ = writeln!(
                variant_arms,
                "            {enum_path}::{variant}(_) => {proto_name:?},"
            );
            let _ = writeln!(
                node_arms,
                "            {enum_path}::{variant}(x) => x.data_to_node(),"
            );
            let _ = writeln!(
                unknown_arms,
                "            {enum_path}::{variant}(x) => x.parse_unknown(y),"
            );
        }

        let _ = write!(
            out,
            "impl crate::input::traits::ProtoOneOf for {enum_path} {{\n\
             \x20   fn proto_oneof_variant(&self) -> &'static str {{\n\
             \x20       match self {{\n{variant_arms}        }}\n\
             \x20   }}\n\
             }}\n\
             impl crate::input::traits::InputNode for {enum_path} {{\n\
             \x20   fn type_to_node() -> crate::output::tree::Node {{\n\
             \x20       crate::output::tree::NodeType::ProtoMissingOneOf.into()\n\
             \x20   }}\n\
             \x20   fn data_to_node(&self) -> crate::output::tree::Node {{\n\
             \x20       match self {{\n{node_arms}        }}\n\
             \x20   }}\n\
             \x20   fn oneof_variant(&self) -> Option<&'static str> {{\n\
             \x20       use crate::input::traits::ProtoOneOf;\n\
             \x20       Some(self.proto_oneof_variant())\n\
             \x20   }}\n\
             \x20   fn parse_unknown(&self, y: &mut crate::parse::context::Context<'_>) -> bool {{\n\
             \x20       match self {{\n{unknown_arms}        }}\n\
             \x20   }}\n\
             }}\n",
        );
    }

    /// Emits the `ProtoEnum` impl for an enum, mirroring
    /// `proto_meta_derive_enum`. The blanket impls in `input/traits.rs` supply
    /// `ProtoPrimitive` and `InputNode` from this.
    fn emit_enum(out: &mut String, en: &EnumDescriptorProto, mod_path: &str, fqn_prefix: &str) {
        let name = en.name();
        let proto_type = format!("{fqn_prefix}.{name}")[1..].to_string();
        let type_path = format!("{mod_path}::{}", name.to_upper_camel_case());
        let default_variant = en.value[0].name().to_string();

        let mut arms = String::new();
        for value in &en.value {
            let variant = enum_variant_ident(name, value.name());
            let proto_name = value.name();
            let _ = writeln!(
                arms,
                "            {type_path}::{variant} => {proto_name:?},"
            );
        }

        let _ = write!(
            out,
            "impl crate::input::traits::ProtoEnum for {type_path} {{\n\
             \x20   fn proto_enum_type() -> &'static str {{ {proto_type:?} }}\n\
             \x20   fn proto_enum_default_variant() -> &'static str {{ {default_variant:?} }}\n\
             \x20   fn proto_enum_variant(&self) -> &'static str {{\n\
             \x20       match self {{\n{arms}        }}\n\
             \x20   }}\n\
             \x20   fn proto_enum_from_i32(x: i32) -> Option<Self> {{ Self::from_i32(x) }}\n\
             }}\n",
        );
    }
}
