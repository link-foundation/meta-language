use flate2::read::GzDecoder;
use std::{fs::File, io, path::PathBuf};

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
    println!("cargo:rerun-if-changed={}", vendor.display());
}
