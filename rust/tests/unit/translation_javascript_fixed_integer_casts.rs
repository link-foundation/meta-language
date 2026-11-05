//! Deferred native frontend controls with independent emitted JavaScript results.
use super::issue_195_translation_pairs::run;
use meta_language::read_structured_data_lino;
use std::process::Command;

#[test]
fn native_constant_width_frontend_retains_every_scalar_original() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual.split('.').collect::<Vec<_>>();
    assert!(
        parts[0] == "1" && (90..100).contains(&parts[1].parse::<u32>().unwrap()),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-call-source.lino"
    ))
    .unwrap();
    let mut count = 0;
    let mut parsed = 0;
    for row in fixture["rows"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        if !name.starts_with("BigInt fixed width ") {
            continue;
        }
        let surface = meta_language::translation::javascript::parse_javascript(
            row["source"].as_str().unwrap(),
        )
        .unwrap();
        let program = meta_language::translation::check::check_program(&surface).unwrap();
        let javascript =
            meta_language::translation::emit_javascript::emit_javascript(&program).unwrap();
        meta_language::translation::emit_rust::emit_rust(&program).unwrap();
        parsed += 1;
        if row["referenceSemantics"] == true || row["expectedOutcome"] == "sourceAllocation" {
            continue;
        }
        count += 1;
        let target = &javascript
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "inspect")
            .unwrap()
            .target;
        let input = row["arguments"][0]["value"].as_str().unwrap();
        let source = format!(
            "import {{assertVersions}} from '../experiments/javascript-properties/controls.mjs';assertVersions();\n{}\nconsole.log(String({target}(BigInt({input:?}))));",
            javascript.text
        );
        let actual = run(Command::new("node")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "--max-old-space-size=352",
                "--input-type=module",
                "--eval",
                &source,
            ]));
        assert!(actual.ok, "{name}: {}", actual.stderr);
        assert_eq!(
            actual.stdout.trim(),
            row["expected"]["value"].as_str().unwrap(),
            "{name}"
        );
    }
    assert_eq!(
        count, 18,
        "all scalar width originals retain independent runtime expectations"
    );
    assert_eq!(
        parsed, 21,
        "all fixed-width originals retain native parsing and both source emissions"
    );
}
