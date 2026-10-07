use flate2::read::GzDecoder;
use std::{fs::File, io, path::PathBuf};
fn main() {
    let vendor = PathBuf::from("../../vendor/tree-sitter-cmake/src");
    let parser = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("parser.c");
    io::copy(
        &mut GzDecoder::new(File::open(vendor.join("parser.c.gz")).unwrap()),
        &mut File::create(&parser).unwrap(),
    )
    .unwrap();
    cc::Build::new()
        .std("c11")
        .include(&vendor)
        .file(&parser)
        .file(vendor.join("scanner.c"))
        .compile("cmake-source-oracle");
    println!("cargo:rerun-if-changed={}", vendor.display());
}
