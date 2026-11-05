//! Deferred CST controls; native compilation/execution belongs exclusively to CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{ProgramProjectContext, ProgramRepresentation, analyze_program};
use std::{fs, process::Command};

#[test]
fn actual_rust_literal_spans_preserve_binding_names_and_original_program_values() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-rust-literal-masking.lino"
    ))
    .unwrap();
    let directory = TemporaryDirectory::new("source-rust-literal-masking");
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let source = row["source"].as_str().unwrap();
        let program = analyze_program(source, "Rust", ProgramProjectContext::default()).unwrap();
        assert_eq!(program.network().reconstruct_text(), source);
        let binding = program
            .bindings()
            .iter()
            .find(|binding| binding.name() == "value" && binding.kind() == "parameter")
            .unwrap();
        assert_eq!(binding.references().len(), 1);
        for name in row["excluded"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            assert!(
                !program
                    .bindings()
                    .iter()
                    .any(|binding| binding.name() == name)
            );
            assert!(
                !program
                    .unresolved_references()
                    .iter()
                    .any(|reference| reference.name() == name)
            );
        }
        let renamed = program.rename_binding(binding.id(), "selected").unwrap();
        assert_eq!(renamed.source(), row["renamed"].as_str().unwrap());
        assert_eq!(renamed.network().reconstruct_text(), renamed.source());
        let restored = ProgramRepresentation::from_snapshot(&renamed.serialize_snapshot()).unwrap();
        assert_eq!(restored.source(), renamed.source());
        assert_eq!(restored.bindings(), renamed.bindings());
        for (variant, source) in [("original", source), ("renamed", renamed.source())] {
            let path = directory.0.join(format!("case-{index}-{variant}.rs"));
            let binary = directory.0.join(format!(
                "case-{index}-{variant}{}",
                std::env::consts::EXE_SUFFIX
            ));
            fs::write(
                &path,
                format!(
                    "{source}\nfn main(){{println!(\"{{}}\",{});}}",
                    row["probe"].as_str().unwrap()
                ),
            )
            .unwrap();
            let result = run(Command::new("rustc")
                .arg("--edition=2024")
                .arg("--crate-name")
                .arg("source_literal_case")
                .arg(&path)
                .arg("-o")
                .arg(&binary));
            assert!(result.ok, "{}", result.stderr);
            let result = run(&mut Command::new(&binary));
            assert!(result.ok, "{}", result.stderr);
            assert_eq!(result.stdout.trim(), row["value"].as_str().unwrap());
        }
    }
}
