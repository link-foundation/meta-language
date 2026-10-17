//! Final array uses consume storage while immutable source aliases keep values.
//! Native execution belongs to CI; this module is never compiled locally.

use std::fs;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_rust::{
    RustEmissionOptions, emit_rust, emit_rust_with_array_consumption,
};
use meta_language::translation::javascript::parse_javascript;
use regex::Regex;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

#[test]
fn generated_array_loops_consume_actual_final_bindings_without_repeated_vector_copies() {
    let original = "/** @param {number} count @returns {number[]} */ function build(count) { const values=[]; for(let index=0;index<count;index+=1) { if(index%2===0) values.push(index,index+1); } return values; }\n/** @returns {number} */ function hygiene(){const ml_append_right=[1];const ml_append_left=2;ml_append_right.push(ml_append_left);return ml_append_right.length;}\n/** @returns {number} */function preserve(){const original=[1];const alias=original;const updated=[...original,2];return alias.length*10+updated.length;}\n/** @returns {number} */function readAfter(){const original=[1];const updated=[...original,2];return original.length*10+updated.length;}\nconst values=build(100001);console.log(values.length,values[0]??-1,values[100001]??-1);console.log(hygiene(),preserve(),readAfter());";
    // Retain the complete original input under its existing prefix refusal.
    let refused =
        parse_javascript(original).expect_err("the original reserved source stays refused");
    assert_eq!(
        refused.reason,
        "reserved identifier: ml_append_right uses the translator's reserved ml_ prefix"
    );
    assert_eq!(
        refused.span,
        Some(meta_language::translation::Span::new(243, 258))
    );
    // The legal source names become the same Rust temporary names, so the
    // simultaneous initializer still has to preserve the actual collision.
    let source = original
        .replace("ml_append_right", "mlAppendRight")
        .replace("ml_append_left", "mlAppendLeft");
    let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
    let emitted =
        emit_rust_with_array_consumption(&program, RustEmissionOptions::default()).unwrap();
    let consume = Regex::new(r"let \(ml_append_right, ml_append_left\) = \(vec!\[[^\]]+\], [A-Za-z_][A-Za-z_0-9]*\); crate::ml_array::append").unwrap();
    assert!(consume.is_match(&emitted.text), "{}", emitted.text);
    let hygiene = emitted
        .definitions
        .iter()
        .find(|text| text.starts_with("pub fn hygiene("))
        .unwrap();
    assert!(hygiene.contains("let ml_append_right ="), "{hygiene}");
    assert!(hygiene.contains("let ml_append_left ="), "{hygiene}");
    assert!(
        hygiene.contains("let (ml_append_right, ml_append_left)"),
        "{hygiene}"
    );
    let read_after = emitted
        .definitions
        .iter()
        .find(|text| text.starts_with("pub fn read_after("))
        .unwrap();
    assert!(read_after.contains("original.clone()"), "{read_after}");
    assert!(
        !read_after.contains("let (ml_append_right, ml_append_left)"),
        "{read_after}"
    );
    let directory = TemporaryDirectory::new("owned-array-final-consumption");
    let source_path = directory.0.join("program.rs");
    let executable = directory.0.join("program");
    fs::write(&source_path, &emitted.text).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(compiler.arg(&source_path).arg("-o").arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    let observed = run(&mut Command::new(executable));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "100002 0 100001\n2 12 12\n");
}

#[test]
fn copied_and_consumed_array_profiles_preserve_actual_alias_observations() {
    let source = "/** @returns {number} */function preserve(){const original=[1];const alias=original;const updated=[...original,2];return alias.length*10+updated.length;}console.log(preserve());";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let copied = emit_rust(&program).unwrap();
    let consumed =
        emit_rust_with_array_consumption(&program, RustEmissionOptions::default()).unwrap();
    assert!(
        copied
            .text
            .contains("crate::ml_array::append(original.clone(),")
    );
    assert!(
        !copied
            .text
            .contains("let (ml_append_right, ml_append_left)")
    );
    assert!(
        consumed
            .text
            .contains("let (ml_append_right, ml_append_left)")
    );
    assert!(consumed.text.contains("let alias = original.clone();"));
    assert_eq!(consumed.assumptions, copied.assumptions);
    assert_eq!(consumed.encodings, copied.encodings);
    for (name, emitted) in [("copied", copied), ("consumed", consumed)] {
        let directory = TemporaryDirectory::new(name);
        let source_path = directory.0.join("program.rs");
        let executable = directory.0.join("program");
        fs::write(&source_path, &emitted.text).unwrap();
        let mut compiler = Command::new("rustc");
        compiler.args(["--edition", "2024"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            compiler.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(compiler.arg(&source_path).arg("-o").arg(&executable));
        assert!(compiled.ok, "{name}: {}", compiled.stderr);
        let observed = run(&mut Command::new(executable));
        assert!(observed.ok, "{name}: {}", observed.stderr);
        assert_eq!(observed.stdout, "12\n", "{name}");
    }
}
