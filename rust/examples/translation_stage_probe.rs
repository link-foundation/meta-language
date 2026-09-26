//! Compares the Rust translation pipeline with JSON dumps of the JavaScript
//! pipeline (`js/experiments/translation-stage-dump.mjs`), stage by stage.
//!
//! ```text
//! cargo run --example translation_stage_probe -- roundtrip-surface surface.json
//! cargo run --example translation_stage_probe -- roundtrip-ir ir.json
//! cargo run --example translation_stage_probe -- check surface.json <dump-dir>
//! ```
//!
//! `check` runs the Rust checker on the JavaScript frontend's surface program
//! and compares the result with `ir.json`, or its error with `error.json`.
//!
//! Each command prints the paths at which the two JSON documents differ.

use meta_language::translation::{check::check_program, ir::Program, surface::SProgram};
use serde_json::Value;

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
        _ => {
            eprintln!("usage: translation_stage_probe (roundtrip-surface|roundtrip-ir <file>|check <surface> <dir>)");
            false
        }
    };
    std::process::exit(i32::from(!ok));
}
