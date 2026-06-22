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

        // Synchronize the protobuf files from the main repository. Note that
        // the core Substrait protobuf files (the `substrait` and
        // `substrait.extensions` packages) are no longer *compiled* by this
        // crate: we consume their pre-generated types from the `substrait-prost`
        // crate and synthesize our introspection trait impls from its embedded
        // descriptor (see generate_prost_meta). We still vendor the .proto files
        // here, however, because the Python bindings (py/build.rs) generate
        // their own protobuf modules from them, and sdist builds rely on this
        // vendored copy.
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

    // Compile the validator-specific protobuf files (the `substrait.validator`
    // package) using prost, applying our ProtoMeta derive to obtain the
    // introspection traits. These types are not provided by substrait-prost.
    // The include path is the whole proto resource directory so that the
    // validator protos can import each other and the google/protobuf
    // well-known types (which prost-build provides); they do not import any
    // core `substrait` protos.
    let proto_path = PathBuf::from(&resource_dir).join("proto");
    let validator_proto_files = find_proto_files(&proto_path.join("substrait/validator"));
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::substrait_validator_derive::ProtoMeta)]");
    config.type_attribute(".", "#[allow(deprecated)]");
    config.compile_protos(&validator_proto_files, &[&proto_path.display().to_string()])?;

    // Inform cargo that changes to the .proto files require a rerun.
    for path in &validator_proto_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // Generate the introspection trait impls (InputNode/ProtoMessage/
    // ProtoOneOf/ProtoEnum) for the substrait-prost types by decoding that
    // crate's embedded FileDescriptorSet. This mirrors what the ProtoMeta
    // derive macro produces for locally-generated types, but targets the
    // foreign substrait-prost types (legal by the orphan rule, as the traits
    // are local to this crate).
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let meta_source = prost_meta::generate(substrait_prost::FILE_DESCRIPTOR_SET);
    fs::write(out_dir.join("substrait_prost_meta.rs"), meta_source)?;

    Ok(())
}

/// Generates the validator's protobuf introspection trait impls
/// (`InputNode`/`ProtoMessage`/`ProtoOneOf`/`ProtoEnum`) for the `substrait`
/// and `substrait.extensions` types provided by the `substrait-prost` crate.
///
/// substrait-prost ships plain `prost`-generated types without our `ProtoMeta`
/// derive, and a derive cannot be applied to a foreign crate's types. So we
/// reconstruct exactly what the derive would have emitted, but from the
/// protobuf [`FileDescriptorSet`](prost_types::FileDescriptorSet) that
/// substrait-prost embeds (via its `embed-descriptor` feature). The emitted
/// impls target the foreign `::substrait_prost::*` types, which is legal
/// because the traits are local to this crate (the orphan rule).
///
/// The hardest part is reproducing prost's code-generation decisions from the
/// descriptor alone — in particular which singular message fields prost wraps
/// in `Box` to break recursive cycles, and how prost names Rust modules,
/// fields, and enum variants. Getting any of these wrong is a hard compile
/// error in the generated file, never a silent behavioral difference.
mod prost_meta {
    use heck::{ToSnakeCase, ToUpperCamelCase};
    use prost::Message;
    use prost_types::field_descriptor_proto::{Label, Type};
    use prost_types::{
        DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorSet,
    };
    use std::collections::{HashMap, HashSet};
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

    /// A fully-qualified protobuf name with a leading dot, e.g.
    /// `.substrait.Type.List`. Matches the form used in descriptors'
    /// `type_name` fields.
    type Fqn = String;

    /// Collected information about every message in the descriptor set, used to
    /// compute prost's boxing decisions and to emit impls.
    struct Schema<'a> {
        /// Maps a message FQN to its descriptor.
        messages: HashMap<Fqn, &'a DescriptorProto>,
        /// Adjacency for the message-reference graph: an edge `M -> T` exists
        /// for every singular (non-repeated) message-typed field of `M`,
        /// including oneof members. Mirrors prost's MessageGraph.
        edges: HashMap<Fqn, Vec<Fqn>>,
    }

    impl<'a> Schema<'a> {
        /// Returns whether there is a directed path `from -> ... -> to` in the
        /// message-reference graph. prost boxes a singular message field of
        /// type `T` in message `M` exactly when `is_nested(T, M)` holds.
        fn is_nested(&self, from: &str, to: &str) -> bool {
            let mut stack = vec![from.to_string()];
            let mut seen = HashSet::new();
            while let Some(node) = stack.pop() {
                if node == to {
                    return true;
                }
                if !seen.insert(node.clone()) {
                    continue;
                }
                if let Some(next) = self.edges.get(&node) {
                    stack.extend(next.iter().cloned());
                }
            }
            false
        }
    }

    /// Whether a field is a singular (non-repeated) message/group field. Oneof
    /// members count; map entries and repeated fields do not.
    fn is_singular_message(field: &FieldDescriptorProto) -> bool {
        field.label() != Label::Repeated && matches!(field.r#type(), Type::Message | Type::Group)
    }

    /// Recursively records a message and its nested messages into the schema,
    /// adding graph edges for singular message fields.
    fn collect_message<'a>(schema: &mut Schema<'a>, msg: &'a DescriptorProto, fqn_prefix: &str) {
        let fqn = format!("{fqn_prefix}.{}", msg.name());
        // Map-entry messages are synthetic; Substrait does not use protobuf
        // maps, but skip them defensively so they never enter the graph.
        if msg
            .options
            .as_ref()
            .and_then(|o| o.map_entry)
            .unwrap_or(false)
        {
            return;
        }
        let targets: Vec<Fqn> = msg
            .field
            .iter()
            .filter(|f| is_singular_message(f))
            .map(|f| f.type_name().to_string())
            .collect();
        schema.edges.insert(fqn.clone(), targets);
        schema.messages.insert(fqn.clone(), msg);
        for nested in &msg.nested_type {
            collect_message(schema, nested, &fqn);
        }
    }

    /// Entry point: produces the full generated Rust source.
    pub fn generate(descriptor_bytes: &[u8]) -> String {
        let fds = FileDescriptorSet::decode(descriptor_bytes)
            .expect("failed to decode substrait-prost FileDescriptorSet");

        // Pass 1: collect all messages and the reference graph.
        let mut schema = Schema {
            messages: HashMap::new(),
            edges: HashMap::new(),
        };
        for file in &fds.file {
            let pkg = file.package();
            let prefix = format!(".{pkg}");
            for msg in &file.message_type {
                collect_message(&mut schema, msg, &prefix);
            }
        }

        // Pass 2: emit impls for the substrait + substrait.extensions packages.
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
                emit_message(&mut out, &schema, msg, &base, &fqn_prefix);
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
    fn emit_message(
        out: &mut String,
        schema: &Schema,
        msg: &DescriptorProto,
        mod_path: &str,
        fqn_prefix: &str,
    ) {
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

        emit_message_impls(out, schema, msg, &type_path, proto_type, &fqn);

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
            emit_message(out, schema, nested, &child_mod, &fqn);
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

    /// Emits the `ProtoMessage` and `InputNode` impls for a message, mirroring
    /// `proto_meta_derive_message` in the derive crate.
    fn emit_message_impls(
        out: &mut String,
        schema: &Schema,
        msg: &DescriptorProto,
        type_path: &str,
        proto_type: &str,
        fqn: &str,
    ) {
        let mut arms = String::new();
        let mut emitted_oneofs = HashSet::new();
        for field in &msg.field {
            if field.proto3_optional() {
                // proto3 optional scalar/message -> Option<T>.
                push_field_arm(&mut arms, &field_name(field), FieldKind::Optional);
            } else if let Some(idx) = field.oneof_index {
                // Member of a real oneof: emit one grouped arm at the position
                // of the first member, named after the oneof.
                if is_synthetic_oneof(msg, idx as usize) {
                    push_field_arm(&mut arms, &field_name(field), FieldKind::Optional);
                } else if emitted_oneofs.insert(idx) {
                    let oneof = &msg.oneof_decl[idx as usize];
                    push_field_arm(
                        &mut arms,
                        &oneof.name().to_snake_case(),
                        FieldKind::Optional,
                    );
                }
            } else {
                push_field_arm(
                    &mut arms,
                    &field_name(field),
                    classify_field(schema, field, fqn),
                );
            }
        }

        // A message with no traversable fields can never have unknown
        // subtrees, so emit a body that does not trip unused-variable lints.
        let parse_unknown_body = if arms.is_empty() {
            "        fn parse_unknown(&self, _y: &mut crate::parse::context::Context<'_>) -> bool { false }\n".to_string()
        } else {
            format!(
                "        fn parse_unknown(&self, y: &mut crate::parse::context::Context<'_>) -> bool {{\n\
                 \x20           let mut unknowns = false;\n{arms}\
                 \x20           unknowns\n\
                 \x20       }}\n",
            )
        };

        let _ = write!(
            out,
            "impl crate::input::traits::ProtoMessage for {type_path} {{\n\
             \x20   fn proto_message_type() -> &'static str {{ {proto_type:?} }}\n\
             }}\n\
             impl crate::input::traits::InputNode for {type_path} {{\n\
             \x20   fn type_to_node() -> crate::output::tree::Node {{\n\
             \x20       use crate::input::traits::ProtoMessage;\n\
             \x20       crate::output::tree::NodeType::ProtoMessage(Self::proto_message_type()).into()\n\
             \x20   }}\n\
             \x20   fn data_to_node(&self) -> crate::output::tree::Node {{\n\
             \x20       use crate::input::traits::ProtoMessage;\n\
             \x20       crate::output::tree::NodeType::ProtoMessage(Self::proto_message_type()).into()\n\
             \x20   }}\n\
             \x20   fn oneof_variant(&self) -> Option<&'static str> {{ None }}\n\
             {parse_unknown_body}\
             }}\n",
        );
    }

    /// The kind of `parse_unknown` action to emit for a struct field, matching
    /// the four cases of the derive's `is_repeated` classification.
    enum FieldKind {
        /// `Option<T>` — emitted with `.as_ref()`.
        Optional,
        /// `Option<Box<T>>` — emitted without `.as_ref()`.
        Boxed,
        /// `Vec<T>` (repeated, non-bytes).
        Repeated,
        /// A bare scalar/enum/bytes value.
        Primitive,
    }

    /// Classifies a non-oneof, non-proto3-optional field into prost's field
    /// shape, mirroring `is_repeated` in the derive crate plus the boxing rule.
    fn classify_field(
        schema: &Schema,
        field: &FieldDescriptorProto,
        container_fqn: &str,
    ) -> FieldKind {
        if field.label() == Label::Repeated {
            return FieldKind::Repeated;
        }
        if matches!(field.r#type(), Type::Message | Type::Group) {
            if schema.is_nested(field.type_name(), container_fqn) {
                FieldKind::Boxed
            } else {
                FieldKind::Optional
            }
        } else {
            // Scalars (including bytes -> Vec<u8>) and enums (i32) have implicit
            // presence in proto3 and are bare values.
            FieldKind::Primitive
        }
    }

    /// The field-name string used both for `field_parsed` and as the node
    /// label, matching `cook_ident(stringify!(ident))` in the derive: prost's
    /// snake_case Rust ident with any raw-identifier prefix stripped.
    fn field_name(field: &FieldDescriptorProto) -> String {
        field.name().to_snake_case()
    }

    /// Appends one guarded `parse_unknown` block for a field.
    fn push_field_arm(arms: &mut String, name: &str, kind: FieldKind) {
        let ident = escape(name);
        let action = match kind {
            FieldKind::Optional => format!(
                "            crate::parse::traversal::push_proto_field(y, &self.{ident}.as_ref(), {name:?}, true, |_, _| Ok(()));\n",
            ),
            FieldKind::Boxed => format!(
                "            crate::parse::traversal::push_proto_field(y, &self.{ident}, {name:?}, true, |_, _| Ok(()));\n",
            ),
            FieldKind::Repeated => format!(
                "            crate::parse::traversal::push_proto_repeated_field(y, &self.{ident}.as_ref(), {name:?}, true, |_, _| Ok(()), |_, _, _, _, _| ());\n",
            ),
            FieldKind::Primitive => format!(
                "            {{\n\
                 \x20               use crate::input::traits::ProtoPrimitive;\n\
                 \x20               if !y.config.ignore_unknown_fields || !self.{ident}.proto_primitive_is_default() {{\n\
                 \x20                   crate::parse::traversal::push_proto_field(y, &Some(&self.{ident}), {name:?}, true, |_, _| Ok(()));\n\
                 \x20               }}\n\
                 \x20           }}\n",
            ),
        };
        let _ = write!(
            arms,
            "        if !y.field_parsed({name:?}) {{\n            unknowns = true;\n{action}        }}\n",
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
