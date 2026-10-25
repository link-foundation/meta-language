//! Deferred complete source products; native execution belongs to CI.
use super::translation_rust_records::assert_nominal_translation_product;

pub(super) fn require_supported_rust_version() {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let version = text.split_whitespace().nth(1).unwrap();
    let mut parts = version.split('.');
    assert_eq!(parts.next(), Some("1"), "uncovered Rust {version}");
    let minor: u32 = parts.next().unwrap().parse().unwrap();
    assert!(
        (90..100).contains(&minor),
        "uncovered Rust {version}; >=1.90 <1.100; edition 2024"
    );
}

#[test]
fn actual_object_declarations_preserve_lexical_bindings_and_lazy_defaults() {
    require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/object-binding-declarations.mjs",
        "JavaScript actual lexical object binding declarations",
        meta_language::create_java_script_record_parameter_program_links,
    );
}

#[test]
fn actual_reader_bodies_keep_disjoint_primitive_result_domains() {
    require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/owned-array-reader-domains.mjs",
        "JavaScript actual disjoint primitive array readers",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/owned-array-reader-domain-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
