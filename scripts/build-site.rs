#!/usr/bin/env rust-script
//! Assemble the GitHub Pages website into `_site/`.
//!
//! The published site at <https://link-foundation.github.io/meta-language> is
//! more than raw `cargo doc` output. `cargo doc` writes the crate docs to
//! `target/doc/<crate>/index.html` and never creates a root `target/doc/index.html`,
//! so deploying `target/doc` directly makes the Pages root URL return HTTP 404
//! (this was issue #90). This script assembles a proper site:
//!
//! ```text
//! _site/
//!   index.html, styles.css, app.js   <- landing page (description + demo + docs)
//!   demo/pkg/...                      <- WebAssembly interactive demo (from web/)
//!   api/                              <- rustdoc, with a root redirect index.html
//! ```
//!
//! Usage:
//!   rust-script scripts/build-site.rs            # full build (docs + wasm + assemble)
//!   rust-script scripts/build-site.rs --assemble-only
//!       Skip `cargo doc` / `wasm-pack` and only assemble from existing
//!       `target/doc` and `web/pkg` (used by CI, which runs those steps itself).
//!
//! Requirements for a full build: `cargo`, and `wasm-pack` with the
//! `wasm32-unknown-unknown` target installed.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

/// Crate library name (with dashes turned into underscores) used by rustdoc as
/// the docs subdirectory: `target/doc/<crate>/index.html`.
const CRATE_DOC_DIR: &str = "meta_language";

fn main() {
    let assemble_only = env::args().any(|arg| arg == "--assemble-only");

    let root = repo_root();
    let site = root.join("_site");
    let target_doc = root.join("target/doc");
    let web_pkg = root.join("web/pkg");

    if !assemble_only {
        run(
            "cargo",
            &["doc", "--no-deps", "--all-features"],
            &root,
        );
        run(
            "wasm-pack",
            &[
                "build",
                "--release",
                "--target",
                "web",
                "--out-dir",
                "pkg",
            ],
            &root.join("web"),
        );
    }

    // Fail early with a clear message if the inputs are missing.
    require_dir(&target_doc, "run `cargo doc --no-deps --all-features` first");
    require_dir(
        &web_pkg,
        "run `wasm-pack build --release --target web --out-dir pkg` in web/ first",
    );

    // Fresh _site.
    if site.exists() {
        fs::remove_dir_all(&site).expect("remove existing _site");
    }
    fs::create_dir_all(&site).expect("create _site");

    // 1. Landing page assets at the site root.
    copy_dir(&root.join("docs/site"), &site);

    // 2. rustdoc under /api with a root redirect so /api/ does not 404.
    let api = site.join("api");
    copy_dir(&target_doc, &api);
    write_redirect(
        &api.join("index.html"),
        &format!("{CRATE_DOC_DIR}/index.html"),
    );

    // 3. WebAssembly demo under /demo/pkg.
    let demo_pkg = site.join("demo/pkg");
    copy_dir(&web_pkg, &demo_pkg);

    // 4. Tell GitHub Pages not to run the uploaded files through Jekyll, which
    //    would strip the rustdoc directories whose names start with `_`.
    fs::write(site.join(".nojekyll"), b"").expect("write .nojekyll");

    println!("Assembled site at {}", site.display());
    println!("  /              -> landing page");
    println!("  /demo/pkg/     -> WebAssembly demo");
    println!("  /api/          -> rustdoc (redirects to /api/{CRATE_DOC_DIR}/)");
}

/// Locate the repository root (the directory containing `Cargo.toml`).
fn repo_root() -> PathBuf {
    let mut dir = env::current_dir().expect("current dir");
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("docs/site").exists() {
            return dir;
        }
        if !dir.pop() {
            // Fall back to current dir; later checks will produce a clear error.
            return env::current_dir().expect("current dir");
        }
    }
}

fn require_dir(path: &Path, hint: &str) {
    if !path.is_dir() {
        eprintln!("error: expected directory {} ({hint})", path.display());
        exit(1);
    }
}

fn run(program: &str, args: &[&str], cwd: &Path) {
    println!("$ {program} {} (in {})", args.join(" "), cwd.display());
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|error| {
            eprintln!("error: failed to launch {program}: {error}");
            exit(1);
        });
    if !status.success() {
        eprintln!("error: {program} exited with {status}");
        exit(1);
    }
}

/// Recursively copy `from` directory contents into `to`.
fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create destination dir");
    for entry in fs::read_dir(from).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let file_type = entry.file_type().expect("file type");
        let dest = to.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&entry.path(), &dest);
        } else if file_type.is_file() {
            fs::copy(entry.path(), &dest).expect("copy file");
        }
    }
}

/// Write a minimal HTML page that redirects to `target` (a relative URL).
fn write_redirect(path: &Path, target: &str) {
    let html = format!(
        "<!DOCTYPE html><meta charset=\"utf-8\">\
<meta http-equiv=\"refresh\" content=\"0; url={target}\">\
<link rel=\"canonical\" href=\"{target}\">\
<title>Redirecting…</title>\
<a href=\"{target}\">Redirecting to the documentation…</a>\n"
    );
    fs::write(path, html).expect("write redirect");
}
