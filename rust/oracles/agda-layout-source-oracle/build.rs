use flate2::read::GzDecoder;
use std::{fs::File, io, path::PathBuf, process::Command};

fn main() {
    let vendor = PathBuf::from("../../vendor/tree-sitter-agda-layout/src");
    let parser = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("parser.c");
    io::copy(
        &mut GzDecoder::new(File::open(vendor.join("parser.c.gz")).unwrap()),
        &mut File::create(&parser).unwrap(),
    )
    .unwrap();
    let mut compiler = cc::Build::new();
    compiler
        .std("c11")
        .include(&vendor)
        .file(&parser)
        .file(vendor.join("scanner.c"));
    // Keep the original released oracle linked for its existing assertions.
    compiler.define("tree_sitter_agda", "tree_sitter_agda_layout");
    for operation in ["create", "destroy", "scan", "serialize", "deserialize"] {
        compiler.define(
            &format!("tree_sitter_agda_external_scanner_{operation}"),
            format!("tree_sitter_agda_layout_external_scanner_{operation}").as_str(),
        );
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        compiler.flag("-utf-8");
    }
    compiler.compile("agda-layout-source-oracle");
    // Isolate the existing queue-relocation hunk while retaining the released
    // serialization behavior required by the original focused assertions.
    let queue_scanner = parser.with_file_name("queue-safe-scanner.c");
    let script = PathBuf::from("../../../js/scripts/agda-queue-safe-oracle.mjs");
    let vendor_root = vendor
        .parent()
        .expect("the scanner retains its oracle root");
    let status = Command::new("node")
        .arg(&script)
        .arg(vendor_root)
        .arg(&queue_scanner)
        .status()
        .expect("Node generates the authenticated queue-only oracle");
    assert!(
        status.success(),
        "the authenticated queue-only generation failed"
    );
    let mut queue_compiler = cc::Build::new();
    queue_compiler
        .std("c11")
        .include(&vendor)
        .file(&parser)
        .file(&queue_scanner)
        .define("tree_sitter_agda", "tree_sitter_agda_queue_safe");
    for operation in ["create", "destroy", "scan", "serialize", "deserialize"] {
        queue_compiler.define(
            &format!("tree_sitter_agda_external_scanner_{operation}"),
            format!("tree_sitter_agda_queue_safe_external_scanner_{operation}").as_str(),
        );
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        queue_compiler.flag("-utf-8");
    }
    queue_compiler.compile("agda-queue-safe-source-oracle");
    println!("cargo:rerun-if-changed={}", script.display());
    println!("cargo:rerun-if-changed={}", vendor.display());
}
