//! Deferred independent native catalog, full source graph and codec controls.
use meta_language::{
    FormalFunctionSelector, MetaLanguageArchiveLimits, SourceDefinitionCatalogLimits,
    create_source_definition_formal_function_links, create_source_definition_program_links,
    create_source_definition_program_links_with_limits, decode_typed_program_links,
    read_meta_language_archive_binary, read_meta_language_archive_text, read_structured_data_lino,
    run_source_definition_command, source_definition_catalog, source_definition_entry,
};
use serde_json::Value;

fn fixture() -> Value {
    let value = read_structured_data_lino(include_str!(
        "../../parity/fixtures/source-definition-catalog.lino"
    ))
    .unwrap();
    let compiler = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    let compiler = String::from_utf8(compiler.stdout).unwrap();
    let version = compiler.split_whitespace().nth(1).unwrap();
    let mut parts = version.split('.');
    assert_eq!(parts.next(), Some("1"), "uncovered Rust {version}");
    let minor: u32 = parts.next().unwrap().parse().unwrap();
    assert!(
        (90..100).contains(&minor),
        "uncovered Rust {version}; {}",
        value["versions"]["Rust"]
    );
    value
}

#[test]
fn source_identity_selection_and_limits_preserve_complete_programs() {
    let fixture = fixture();
    let identifier = fixture["selectedIdentifier"].as_str().unwrap();
    let entries = source_definition_catalog().unwrap();
    for entry in &entries {
        assert_eq!(
            entry.definition_identity,
            format!("{}@sha256:{}", entry.identifier, entry.sha256)
        );
    }
    assert!(
        source_definition_entry("array-classification-program")
            .unwrap()
            .is_none()
    );
    assert!(
        create_source_definition_program_links("missing")
            .unwrap_err()
            .contains("unknown qualified")
    );
    for limits in [
        SourceDefinitionCatalogLimits {
            byte_limit: 1,
            ..Default::default()
        },
        SourceDefinitionCatalogLimits {
            node_limit: 1,
            ..Default::default()
        },
        SourceDefinitionCatalogLimits {
            depth_limit: 0,
            ..Default::default()
        },
    ] {
        assert!(
            create_source_definition_program_links_with_limits(identifier, limits)
                .unwrap_err()
                .contains("limit")
        );
    }
    let program = create_source_definition_program_links(identifier).unwrap();
    assert_eq!(
        program.root.to_string(),
        source_definition_entry(identifier).unwrap().unwrap().root
    );
    let decoded = decode_typed_program_links(&program.network, program.root).unwrap();
    assert!(
        decoded
            .declarations
            .iter()
            .any(|entry| entry.full_name() == fixture["selectedFunction"].as_str().unwrap())
    );
    create_source_definition_formal_function_links(
        identifier,
        FormalFunctionSelector::Name(fixture["selectedFunction"].as_str().unwrap()),
    )
    .unwrap();
    assert!(
        create_source_definition_formal_function_links(
            identifier,
            FormalFunctionSelector::Name("not a declaration")
        )
        .unwrap_err()
        .contains("owned function declaration")
    );
}

#[test]
fn native_definition_commands_preserve_complete_text_and_binary_entry_archives() {
    let fixture = fixture();
    let identifier = fixture["selectedIdentifier"].as_str().unwrap();
    for command in ["program", "function"] {
        for binary in [false, true] {
            let mut args = vec![command.to_owned(), identifier.to_owned()];
            if command == "function" {
                args.push(fixture["selectedFunction"].as_str().unwrap().to_owned());
            }
            if binary {
                args.push("--binary".to_owned());
            }
            let output = run_source_definition_command(&args);
            assert_eq!(output.exit_code, 0, "{}", output.stderr);
            let loaded = if binary {
                read_meta_language_archive_binary(
                    &output.stdout,
                    MetaLanguageArchiveLimits::default(),
                )
                .unwrap()
            } else {
                read_meta_language_archive_text(
                    std::str::from_utf8(&output.stdout).unwrap(),
                    MetaLanguageArchiveLimits::default(),
                )
                .unwrap()
            };
            assert_eq!(loaded.entries.len(), 1);
            assert_eq!(
                loaded.entries[0].kind,
                if command == "function" {
                    "formal concept definition"
                } else {
                    "typed program"
                }
            );
        }
    }
    for args in [
        vec![],
        vec!["list".to_owned(), "--binary".to_owned()],
        vec!["program".to_owned()],
    ] {
        assert_eq!(run_source_definition_command(&args).exit_code, 2);
    }
}
