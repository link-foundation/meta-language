use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Independent development grammars vendored under `../../vendor/` (see each `NOTICE.md`).
const VENDORED_GRAMMARS: &[&str] = &["rocq", "lean"];

macro_rules! native_oracle_baselines {
    ($(($id:literal, $version:literal, $directory:literal, $archive:literal, $symbol:literal, $entry:ident),)*) => {
        const BASELINES: &[(&str, &str, &str)] = &[$(($directory, $archive, $symbol)),*];
    };
}
include!("baselines.rs");

fn decompress_parser(compressed: &Path, parser: &Path) {
    let mut input = GzDecoder::new(File::open(compressed).expect("open vendored parser"));
    let mut output = File::create(parser).expect("create decompressed parser");
    io::copy(&mut input, &mut output).expect("decompress vendored parser");
}

fn grammar_compiler(sources: &Path, parser: &Path) -> cc::Build {
    let mut compiler = cc::Build::new();
    compiler.std("c11").include(sources).file(parser);
    let scanner = sources.join("scanner.c");
    if scanner.exists() {
        compiler.file(&scanner);
        println!("cargo:rerun-if-changed={}", scanner.display());
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        compiler.flag("-utf-8");
    }
    compiler
}

fn compile_grammar(name: &str, out_dir: &Path) {
    let vendor = PathBuf::from(format!("../../vendor/tree-sitter-{name}/src"));
    let compressed = vendor.join("parser.c.gz");
    let parser = out_dir.join(format!("tree-sitter-{name}-parser.c"));
    decompress_parser(&compressed, &parser);
    let mut compiler = grammar_compiler(&vendor, &parser);
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

fn compile_baseline(directory: &str, archive: &str, symbol: &str, out_dir: &Path) {
    let archive = Path::new("../../..").join(archive);
    println!("cargo:rerun-if-changed={}", archive.display());
    let status = Command::new("tar")
        .arg("-xzf")
        .arg(&archive)
        .arg("-C")
        .arg(out_dir)
        .arg(format!("{directory}/src"))
        .status()
        .expect("tar extracts the authenticated baseline archive");
    assert!(status.success(), "could not extract {}", archive.display());

    let sources = out_dir.join(directory).join("src");
    let mut compiler = grammar_compiler(&sources, &sources.join("parser.c"));
    let baseline = format!("{symbol}_baseline");
    compiler.define(symbol, baseline.as_str());
    for operation in [
        "create",
        "destroy",
        "reset",
        "serialize",
        "deserialize",
        "scan",
    ] {
        let original = format!("{symbol}_external_scanner_{operation}");
        let renamed = format!("{baseline}_external_scanner_{operation}");
        compiler.define(&original, renamed.as_str());
    }
    if std::env::var("TARGET").as_deref() == Ok("wasm32-unknown-unknown") {
        compiler.include(
            std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS")
                .expect("tree-sitter-language supplies WebAssembly headers"),
        );
    }
    compiler.compile(directory);
}

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo supplies OUT_DIR"));
    for name in VENDORED_GRAMMARS {
        compile_grammar(name, &out_dir);
    }
    for &(directory, archive, symbol) in BASELINES {
        compile_baseline(directory, archive, symbol, &out_dir);
    }
}
