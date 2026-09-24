use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let vendor = Path::new("vendor/tree-sitter-rocq/src");
    let compressed = vendor.join("parser.c.gz");
    let parser = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"))
        .join("tree-sitter-rocq-parser.c");
    let mut input = GzDecoder::new(File::open(&compressed).expect("open vendored Rocq parser"));
    let mut output = File::create(&parser).expect("create decompressed Rocq parser");
    io::copy(&mut input, &mut output).expect("decompress vendored Rocq parser");

    let mut compiler = cc::Build::new();
    compiler.std("c11").include(vendor).file(&parser);
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        compiler.flag("-utf-8");
    }
    if std::env::var("TARGET").as_deref() == Ok("wasm32-unknown-unknown") {
        let headers = std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS")
            .expect("tree-sitter-language supplies WebAssembly headers");
        let sources = PathBuf::from(
            std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_SRC")
                .expect("tree-sitter-language supplies WebAssembly sources"),
        );
        compiler.include(headers).files([
            sources.join("stdio.c"),
            sources.join("stdlib.c"),
            sources.join("string.c"),
        ]);
    }
    compiler.compile("tree-sitter-rocq");
    println!("cargo:rerun-if-changed={}", compressed.display());
    println!(
        "cargo:rerun-if-changed={}",
        vendor.join("tree_sitter/parser.h").display()
    );
}
