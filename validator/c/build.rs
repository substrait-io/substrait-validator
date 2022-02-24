use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config = cbindgen::Config {
        cpp_compat: true,
        language: cbindgen::Language::C,
        ..Default::default()
    };
    config.export.prefix = Some("substrait_validator_".to_string());
    config
        .export
        .rename
        .insert("Handle".to_string(), "handle".to_string());

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/substrait_validator.h");
}