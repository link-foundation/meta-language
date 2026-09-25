use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

/// Generated grammars vendored under `vendor/` (see each `NOTICE.md`).
const VENDORED_GRAMMARS: &[&str] = &["rocq", "csv"];

fn decompress_parser(compressed: &Path, parser: &Path) {
    let mut input = GzDecoder::new(File::open(compressed).expect("open vendored parser"));
    let mut output = File::create(parser).expect("create decompressed parser");
    io::copy(&mut input, &mut output).expect("decompress vendored parser");
}

fn compile_grammar(name: &str, out_dir: &Path) {
    let vendor = PathBuf::from(format!("vendor/tree-sitter-{name}/src"));
    let compressed = vendor.join("parser.c.gz");
    let parser = out_dir.join(format!("tree-sitter-{name}-parser.c"));
    decompress_parser(&compressed, &parser);

    let mut compiler = cc::Build::new();
    compiler.std("c11").include(&vendor).file(&parser);
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
    compiler.compile(&format!("tree-sitter-{name}"));
    println!("cargo:rerun-if-changed={}", compressed.display());
    println!(
        "cargo:rerun-if-changed={}",
        vendor.join("tree_sitter/parser.h").display()
    );
}

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"));
    for name in VENDORED_GRAMMARS {
        compile_grammar(name, &out_dir);
    }
}
