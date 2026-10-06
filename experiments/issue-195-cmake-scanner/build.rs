// Compiles the tree-sitter-cmake 0.7.5 parser with its scanner, with the
// scanner's initial token forced by -DINITIAL_TOKEN (see src/main.rs).
fn main() {
    let src = std::path::PathBuf::from("/home/box/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tree-sitter-cmake-0.7.5/src");
    let token = std::env::var("INITIAL_TOKEN").unwrap_or_else(|_| "garbage".into());
    println!("cargo:rerun-if-env-changed=INITIAL_TOKEN");
    println!("cargo:rerun-if-changed=scanner-probe.c");
    let mut build = cc::Build::new();
    build.include(&src).file(src.join("parser.c")).file("scanner-probe.c").warnings(false);
    build.flag(format!("-DPROBE_TOKEN_{token}").as_str());
    build.compile("cmake_probe");
}
