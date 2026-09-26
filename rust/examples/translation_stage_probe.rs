//! Compares the Rust translation pipeline with JSON dumps of the JavaScript
//! pipeline (`js/experiments/translation-stage-dump.mjs`), stage by stage.
//!
//! ```text
//! cargo run --example translation_stage_probe -- roundtrip-surface surface.json
//! cargo run --example translation_stage_probe -- roundtrip-ir ir.json
//! cargo run --example translation_stage_probe -- check surface.json <dump-dir>
//! cargo run --example translation_stage_probe -- parse <source> <dump-dir>
//! cargo run --example translation_stage_probe -- emit ir.json <dump-dir>
//! cargo run --example translation_stage_probe -- pipeline <source> <dump-dir>
//! ```
//!
//! `check` runs the Rust checker on the JavaScript frontend's surface program
//! and compares the result with `ir.json`, or its error with `error.json`.
//! `parse` runs the Rust frontend chosen by the source's extension and
//! compares with `surface.json` or the parse error. `emit` runs the four Rust
//! emitters on the JavaScript checker's program and compares each with
//! `emit-<target>.json` or `emit-<target>.error.json`. `pipeline` runs every
//! Rust stage on the source and compares each stage.
//!
//! Each command prints the paths at which the two JSON documents differ.

use meta_language::translation::diagnostics::TranslationError;
use meta_language::translation::emit_common::Emitted;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_lean::emit_lean,
    emit_rocq::emit_rocq, emit_rust::emit_rust, ir::Program, javascript::parse_javascript,
    lean::parse_lean, rocq::parse_rocq, rust::parse_rust, surface::SProgram,
};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;

type Emitter = fn(&Program) -> Result<Emitted, TranslationError>;

const EMITTERS: [(&str, Emitter); 4] = [
    ("javascript", emit_javascript),
    ("rust", emit_rust),
    ("lean", emit_lean),
    ("rocq", emit_rocq),
];

fn differences(path: &str, left: &Value, right: &Value, out: &mut Vec<String>) {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, value) in a {
                let child = format!("{path}.{key}");
                match b.get(key) {
                    Some(other) => differences(&child, value, other, out),
                    None => out.push(format!("{child}: only in javascript ({value})")),
                }
            }
            for (key, value) in b {
                if !a.contains_key(key) {
                    out.push(format!("{path}.{key}: only in rust ({value})"));
                }
            }
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (index, (x, y)) in a.iter().zip(b).enumerate() {
                differences(&format!("{path}[{index}]"), x, y, out);
            }
        }
        _ if left == right => {}
        _ => {
            let show = |value: &Value| {
                let text = value.to_string();
                if text.len() > 160 {
                    format!("{}…", &text[..text.floor_char_boundary_compat(160)])
                } else {
                    text
                }
            };
            out.push(format!(
                "{path}: javascript {} rust {}",
                show(left),
                show(right)
            ));
        }
    }
}

trait FloorBoundary {
    fn floor_char_boundary_compat(&self, index: usize) -> usize;
}

impl FloorBoundary for String {
    fn floor_char_boundary_compat(&self, mut index: usize) -> usize {
        while !self.is_char_boundary(index) {
            index -= 1;
        }
        index
    }
}

fn report(expected: &Value, actual: &Value) -> bool {
    let mut out = Vec::new();
    differences("$", expected, actual, &mut out);
    for line in out.iter().take(40) {
        println!("{line}");
    }
    if out.len() > 40 {
        println!("… {} more", out.len() - 40);
    }
    println!("{} differences", out.len());
    out.is_empty()
}

fn read(path: &str) -> Value {
    let text = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{path}: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// Compares a stage's result with `<dir>/<name>.json`, or its error with
/// `<dir>/<error>` when the JavaScript stage failed.
fn stage<T: Serialize>(
    label: &str,
    actual: &Result<T, TranslationError>,
    dir: &str,
    name: &str,
    error: &str,
) -> bool {
    let expected = format!("{dir}/{name}.json");
    let failure = format!("{dir}/{error}");
    match (actual, Path::new(&expected).exists()) {
        (Ok(actual), true) => {
            let mut out = Vec::new();
            differences(
                "$",
                &read(&expected),
                &serde_json::to_value(actual).unwrap(),
                &mut out,
            );
            for line in out.iter().take(20) {
                println!("{label} {line}");
            }
            if !out.is_empty() {
                println!("{label}: {} differences", out.len());
            }
            out.is_empty()
        }
        (Err(error), false) if Path::new(&failure).exists() => {
            let expected = read(&failure);
            let matches = expected["message"] == error.message();
            if !matches {
                println!(
                    "{label}: javascript {} rust {}",
                    expected["message"],
                    error.message()
                );
            }
            matches
        }
        (Ok(_), false) => {
            println!("{label}: rust succeeds where javascript fails");
            false
        }
        (Err(error), _) => {
            println!(
                "{label}: rust fails where javascript succeeds: {}",
                error.message()
            );
            false
        }
    }
}

fn parse(source: &str) -> Result<SProgram, TranslationError> {
    let text = std::fs::read_to_string(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    match source.rsplit('.').next() {
        Some("lean") => parse_lean(&text),
        Some("v") => parse_rocq(&text),
        Some("rs") => parse_rust(&text),
        Some("mjs" | "js") => parse_javascript(&text),
        _ => panic!("{source}: unknown source language"),
    }
}

fn emit_all(program: &Program, dir: &str) -> bool {
    let mut ok = true;
    for (target, emit) in EMITTERS {
        ok &= stage(
            &format!("emit-{target}"),
            &emit(program),
            dir,
            &format!("emit-{target}"),
            &format!("emit-{target}.error.json"),
        );
    }
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ok = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["roundtrip-surface", path] => {
            let expected = read(path);
            let program: SProgram = serde_json::from_value(expected.clone())
                .unwrap_or_else(|error| panic!("surface: {error}"));
            report(&expected, &serde_json::to_value(&program).unwrap())
        }
        ["roundtrip-ir", path] => {
            let expected = read(path);
            let program: Program = serde_json::from_value(expected.clone())
                .unwrap_or_else(|error| panic!("ir: {error}"));
            report(&expected, &serde_json::to_value(&program).unwrap())
        }
        ["check", surface, dir] => {
            let program: SProgram = serde_json::from_value(read(surface))
                .unwrap_or_else(|error| panic!("surface: {error}"));
            let ir = format!("{dir}/ir.json");
            match (check_program(&program), std::path::Path::new(&ir).exists()) {
                (Ok(actual), true) => report(&read(&ir), &serde_json::to_value(&actual).unwrap()),
                (Err(error), false) => {
                    let expected = read(&format!("{dir}/error.json"));
                    let matches = expected["message"] == error.message();
                    println!(
                        "javascript {} rust {}",
                        expected["message"],
                        error.message()
                    );
                    matches
                }
                (Ok(_), false) => {
                    println!(
                        "rust checked a program javascript rejects: {}",
                        read(&format!("{dir}/error.json"))["message"]
                    );
                    false
                }
                (Err(error), true) => {
                    println!(
                        "rust rejected a program javascript checks: {}",
                        error.message()
                    );
                    false
                }
            }
        }
        ["parse", source, dir] => stage("parse", &parse(source), dir, "surface", "error.json"),
        ["emit", ir, dir] => {
            let program: Program =
                serde_json::from_value(read(ir)).unwrap_or_else(|error| panic!("ir: {error}"));
            emit_all(&program, dir)
        }
        ["pipeline", source, dir] => {
            let surface = parse(source);
            let mut ok = stage("parse", &surface, dir, "surface", "error.json");
            if let Ok(surface) = surface {
                let program = check_program(&surface);
                ok &= stage("check", &program, dir, "ir", "error.json");
                if let Ok(program) = program {
                    ok &= emit_all(&program, dir);
                }
            }
            ok
        }
        _ => {
            eprintln!("usage: translation_stage_probe (roundtrip-surface|roundtrip-ir <file>|check <surface> <dir>|parse <source> <dir>|emit <ir> <dir>|pipeline <source> <dir>)");
            false
        }
    };
    std::process::exit(i32::from(!ok));
}
