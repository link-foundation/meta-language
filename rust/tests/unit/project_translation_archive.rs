//! Deferred full original graphs, rule premises, tamper refusals and CLI codecs.
use meta_language::{
    BinaryLinksOptions, LinkNetwork, LinkType, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    NetworkProjection, StructuredDataLinksLimits, create_meta_language_archive,
    create_project_translation_archive, decode_link_network_binary, encode_link_network_binary,
    encode_structured_data_links_with_limits, read_meta_language_archive,
    read_meta_language_archive_binary, read_meta_language_archive_text,
    read_project_translation_archive, run_project_translation_archive_command,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::Arc;
#[path = "../../src/project_translation_archive_rules.rs"]
mod rules;
fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/project-translation-archive.lino"
    ))
    .unwrap()
}
fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap()
}
fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap()
}
fn flag(value: &Value, key: &str) -> bool {
    value[key].as_bool().unwrap()
}
fn texts(value: &Value, key: &str) -> Vec<String> {
    value[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn complete_shared_archive_membership_and_command_premises() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    for row in data["grammarProfiles"].as_array().unwrap() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(text(row, "path"));
        assert_eq!(
            format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())),
            text(row, "sha256"),
            "uncovered native grammar revision"
        );
    }
    for row in data["vocabulary"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_vocabulary(text(row, "role")),
            text(row, "expected")
        );
    }
    for row in data["languages"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_source_language(
                text(row, "language"),
                text(row, "path")
            ),
            text(row, "expected")
        );
    }
    for row in data["headers"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_header_error(
                number(row, "version"),
                text(row, "interpretation"),
                number(row, "inventory"),
                number(row, "modules"),
                number(row, "sources")
            ),
            text(row, "expected")
        );
    }
    for row in data["sources"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_source_error(
                text(row, "identifier"),
                text(row, "language"),
                number(row, "bytes"),
                text(row, "digest"),
                text(row, "path"),
                text(row, "actualLanguage"),
                number(row, "actualBytes"),
                text(row, "actualDigest")
            ),
            text(row, "expected")
        );
    }
    for row in data["products"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_product_error(
                flag(row, "draft"),
                flag(row, "executable"),
                number(row, "gaps"),
                flag(row, "identical"),
                flag(row, "digest")
            ),
            text(row, "expected")
        );
    }
    for row in data["bindings"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_binding_error(
                &texts(row, "issues"),
                &texts(row, "actualIssues"),
                &texts(row, "gapIssues"),
                &texts(row, "specifiers"),
                &texts(row, "providers"),
                &texts(row, "actualSpecifiers"),
                &texts(row, "actualProviders")
            ),
            text(row, "expected")
        );
    }
    for row in data["configurations"].as_array().unwrap() {
        assert_eq!(
            rules::read_project_translation_archive_configuration_error(
                number(row, "version"),
                number(row, "count"),
                number(row, "inventory"),
                text(row, "target"),
                text(row, "selected")
            ),
            text(row, "expected")
        );
    }
    for row in data["commands"].as_array().unwrap() {
        assert_eq!(
            rules::plan_project_translation_archive_command(&texts(row, "args")),
            row["expected"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap())
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn full_project_source_graphs_survive_both_codecs_and_optional_ast() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let packet = create_project_translation_archive(
        &data["inputs"],
        &json!({"targetLanguage":"JavaScript"}),
    )
    .unwrap();
    let text_graph = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let binary = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    let mut reference = None;
    let mut configuration = None;
    let mut source_bytes = std::collections::BTreeMap::new();
    for network in [&packet.network, &text_graph, &binary] {
        let restored = read_project_translation_archive(network, packet.root, &json!({})).unwrap();
        assert_eq!(
            restored.concrete.sources.len(),
            data["inputs"].as_array().unwrap().len()
        );
        if let Some(previous) = &reference {
            assert_eq!(&restored.draft, previous);
        }
        reference = Some(restored.draft.clone());
        if let Some(previous) = &configuration {
            assert_eq!(&restored.configuration, previous);
        }
        configuration = Some(restored.configuration.clone());
        assert_eq!(
            restored.configuration["options"]["targetLanguage"],
            "JavaScript"
        );
        assert_eq!(
            restored.configuration["modules"]
                .as_array()
                .unwrap()
                .iter()
                .find(|module| module["identifier"] == "src/b.js")
                .unwrap()["options"]["moduleDirectory"],
            json!(["src"])
        );

        for row in data["inputs"].as_array().unwrap() {
            let source = restored
                .concrete
                .sources
                .iter()
                .find(|file| file.path == text(row, "identifier"))
                .unwrap();
            let bytes =
                encode_link_network_binary(&source.network, BinaryLinksOptions::default()).unwrap();
            if let Some(previous) = source_bytes.get(&source.path) {
                assert_eq!(&bytes, previous);
            } else {
                source_bytes.insert(source.path.clone(), bytes);
            }
            assert_eq!(source.source, text(row, "source"));
            assert_eq!(source.network.reconstruct_text(), text(row, "source"));
            assert_eq!(source.projection, NetworkProjection::ConcreteSyntax);
            assert_eq!(
                source.language,
                if source.path.ends_with(".tsx") {
                    "TSX"
                } else {
                    text(row, "language")
                }
            );
            assert!(!source.view.is_empty());
            assert!(
                source
                    .network
                    .links()
                    .any(|link| link.metadata().link_type() == Some(LinkType::Syntax))
            );
        }
        let unresolved = restored.draft["modules"]
            .as_array()
            .unwrap()
            .iter()
            .find(|module| module["identifier"] == "src/unresolved.js")
            .unwrap();
        assert!(unresolved["emittedCode"].is_null());
        assert_eq!(
            unresolved["sourceIssues"],
            data["inputs"].as_array().unwrap().last().unwrap()["sourceIssues"]
        );
    }
    let restored = read_project_translation_archive(
        &packet.network,
        packet.root,
        &json!({"projection":"AbstractSyntax"}),
    )
    .unwrap();
    for source in restored.concrete.sources {
        assert_eq!(source.projection, NetworkProjection::AbstractSyntax);
        let original = data["inputs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["identifier"] == source.path)
            .unwrap();
        assert_eq!(source.network.reconstruct_text(), text(original, "source"));
    }
}
#[test]
fn archive_tampering_and_resource_exhaustion_refuse() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let packet = create_project_translation_archive(
        &data["inputs"],
        &json!({"targetLanguage":"JavaScript"}),
    )
    .unwrap();
    let view = read_project_translation_archive(&packet.network, packet.root, &json!({})).unwrap();
    let entries = read_meta_language_archive(
        &packet.network,
        packet.root,
        MetaLanguageArchiveLimits::default(),
    )
    .unwrap()
    .entries;
    for change in 0..3 {
        let mut draft = view.draft.clone();
        match change {
            0 => draft["inventory"][0]["sourceSha256"] = json!("0".repeat(64)),
            1 => draft["modules"][0]["draftCode"] = json!("changed product"),
            _ => {
                let module = draft["modules"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|module| module["identifier"] == "src/unresolved.js")
                    .unwrap();
                module["gaps"] = json!([]);
            }
        }
        let replacement =
            encode_structured_data_links_with_limits(&draft, StructuredDataLinksLimits::default())
                .unwrap();
        let replacement_root = replacement.root;
        let replacement = Arc::new(replacement.network);
        let forged_entries = entries
            .iter()
            .map(|entry| {
                if entry.identifier == rules::read_project_translation_archive_vocabulary("draft") {
                    MetaLanguageArchiveEntry {
                        network: Arc::clone(&replacement),
                        root: replacement_root,
                        ..entry.clone()
                    }
                } else {
                    entry.clone()
                }
            })
            .collect::<Vec<_>>();
        let forged =
            create_meta_language_archive(&forged_entries, MetaLanguageArchiveLimits::default())
                .unwrap();
        let error = read_project_translation_archive(&forged.network, forged.root, &json!({}))
            .err()
            .unwrap();
        assert!(
            error.contains("changed") || error.contains("cannot be executable"),
            "{error}"
        );
    }
    let missing =
        create_meta_language_archive(&entries[..1], MetaLanguageArchiveLimits::default()).unwrap();
    assert!(read_project_translation_archive(&missing.network, missing.root, &json!({})).is_err());
    assert!(
        read_project_translation_archive(&packet.network, packet.root, &json!({"nodeLimit":1}))
            .is_err()
    );
}
#[test]
fn public_project_command_exports_complete_text_and_binary() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let input = meta_language::structured_data_to_lino(&json!({"inputs":data["inputs"]})).unwrap();
    for binary in [false, true] {
        let mut args = vec![
            "inputs.lino".to_owned(),
            "--to".to_owned(),
            "JavaScript".to_owned(),
        ];
        if binary {
            args.push("--binary".to_owned());
        }
        let result = run_project_translation_archive_command(&args, |path| {
            assert_eq!(path, "inputs.lino");
            Ok(input.clone())
        });
        assert_eq!(result.exit_code, 0, "{}", result.stderr);
        let packet = if binary {
            read_meta_language_archive_binary(&result.stdout, MetaLanguageArchiveLimits::default())
                .unwrap()
        } else {
            read_meta_language_archive_text(
                std::str::from_utf8(&result.stdout).unwrap(),
                MetaLanguageArchiveLimits::default(),
            )
            .unwrap()
        };
        assert_eq!(
            read_project_translation_archive(&packet.network, packet.root, &json!({}))
                .unwrap()
                .concrete
                .sources
                .len(),
            data["inputs"].as_array().unwrap().len()
        );
    }
    assert_eq!(
        run_project_translation_archive_command(&["--help".to_owned()], |_| Err(
            "must not read".to_owned()
        ))
        .exit_code,
        0
    );
    assert_eq!(
        run_project_translation_archive_command(&[], |_| Err("must not read".to_owned())).exit_code,
        2
    );
}

#[test]
fn changed_configuration_target_refuses() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let packet = create_project_translation_archive(
        &data["inputs"],
        &json!({"targetLanguage":"JavaScript"}),
    )
    .unwrap();
    let view = read_project_translation_archive(&packet.network, packet.root, &json!({})).unwrap();
    let entries = read_meta_language_archive(
        &packet.network,
        packet.root,
        MetaLanguageArchiveLimits::default(),
    )
    .unwrap()
    .entries;
    let mut configuration = view.configuration;
    configuration["options"]["targetLanguage"] = json!("Rust");
    let replacement = encode_structured_data_links_with_limits(
        &configuration,
        StructuredDataLinksLimits::default(),
    )
    .unwrap();
    let root = replacement.root;
    let network = Arc::new(replacement.network);
    let entries = entries
        .into_iter()
        .map(|entry| {
            if entry.identifier
                == rules::read_project_translation_archive_vocabulary("configuration")
            {
                MetaLanguageArchiveEntry {
                    root,
                    network: Arc::clone(&network),
                    ..entry
                }
            } else {
                entry
            }
        })
        .collect::<Vec<_>>();
    let forged =
        create_meta_language_archive(&entries, MetaLanguageArchiveLimits::default()).unwrap();
    let error = read_project_translation_archive(&forged.network, forged.root, &json!({}))
        .err()
        .unwrap();
    assert!(error.contains("configuration changed"), "{error}");
}

#[test]
fn source_free_inputs_restore_settings_and_reproduce_the_original_draft() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    for row in data["dependencies"].as_array().unwrap() {
        let providers = row["providers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_bool().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rules::read_project_translation_archive_import_error(
                &texts(row, "specifiers"),
                &providers
            ),
            text(row, "expected")
        );
    }
    let packet = create_project_translation_archive(
        &data["inputs"],
        &json!({"targetLanguage":"JavaScript"}),
    )
    .unwrap();
    let view = read_project_translation_archive(&packet.network, packet.root, &json!({})).unwrap();
    let graph = decode_link_network_binary(
        &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
        BinaryLinksOptions::default(),
    )
    .unwrap();
    let mut restored =
        meta_language::restore_project_translation_archive_inputs(&graph, packet.root, &json!({}))
            .unwrap();
    let mut expected=data["inputs"].as_array().unwrap().iter().map(|row|json!({"identifier":row["identifier"],"language":row["language"],"source":row["source"],"imports":row.get("imports").cloned().unwrap_or_else(||json!({})),"sourceIssues":row.get("sourceIssues").cloned().unwrap_or_else(||json!([])),"options":row.get("options").cloned().unwrap_or_else(||json!({}))})).collect::<Vec<_>>();
    expected.sort_by(|a, b| a["identifier"].as_str().cmp(&b["identifier"].as_str()));
    assert_eq!(restored.inputs, json!(expected));
    assert_eq!(restored.options, json!({"targetLanguage":"JavaScript"}));
    let rebuilt = create_project_translation_archive(&restored.inputs, &restored.options).unwrap();
    let again =
        read_project_translation_archive(&rebuilt.network, rebuilt.root, &json!({})).unwrap();
    assert_eq!(again.draft, view.draft);
    assert_eq!(again.configuration, view.configuration);
    for source in again.concrete.sources {
        let original = view
            .concrete
            .sources
            .iter()
            .find(|file| file.path == source.path)
            .unwrap();
        assert_eq!(
            encode_link_network_binary(&source.network, BinaryLinksOptions::default()).unwrap(),
            encode_link_network_binary(&original.network, BinaryLinksOptions::default()).unwrap()
        );
    }
    restored.options["targetLanguage"] = json!("Rust");
    restored.inputs[0]["options"]["moduleDirectory"] = json!(["changed"]);
    assert_eq!(
        meta_language::restore_project_translation_archive_inputs(&graph, packet.root, &json!({}))
            .unwrap()
            .inputs,
        json!(expected)
    );
}
#[test]
fn duplicate_and_foreign_dependency_directories_cannot_be_restored() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = fixture();
    let packet = create_project_translation_archive(
        &data["inputs"],
        &json!({"targetLanguage":"JavaScript"}),
    )
    .unwrap();
    let view = read_project_translation_archive(&packet.network, packet.root, &json!({})).unwrap();
    let entries = read_meta_language_archive(
        &packet.network,
        packet.root,
        MetaLanguageArchiveLimits::default(),
    )
    .unwrap()
    .entries;
    for imports in [
        json!([{"specifier":"./missing.js","identifier":"src/missing.js"}]),
        json!([{"specifier":"./a.js","identifier":"src/a.js"},{"specifier":"./a.js","identifier":"src/a.js"}]),
    ] {
        let mut draft = view.draft.clone();
        let mut configuration = view.configuration.clone();
        for key in ["inventory", "modules"] {
            draft[key]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|row| row["identifier"] == "src/b.js")
                .unwrap()["imports"] = imports.clone();
        }
        configuration["modules"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|row| row["identifier"] == "src/b.js")
            .unwrap()["imports"] = imports;
        let data =
            encode_structured_data_links_with_limits(&draft, StructuredDataLinksLimits::default())
                .unwrap();
        let settings = encode_structured_data_links_with_limits(
            &configuration,
            StructuredDataLinksLimits::default(),
        )
        .unwrap();
        let draft_root = data.root;
        let draft_network = Arc::new(data.network);
        let settings_root = settings.root;
        let settings_network = Arc::new(settings.network);
        let forged_entries = entries
            .iter()
            .map(|entry| {
                if entry.identifier == rules::read_project_translation_archive_vocabulary("draft") {
                    MetaLanguageArchiveEntry {
                        root: draft_root,
                        network: Arc::clone(&draft_network),
                        ..entry.clone()
                    }
                } else if entry.identifier
                    == rules::read_project_translation_archive_vocabulary("configuration")
                {
                    MetaLanguageArchiveEntry {
                        root: settings_root,
                        network: Arc::clone(&settings_network),
                        ..entry.clone()
                    }
                } else {
                    entry.clone()
                }
            })
            .collect::<Vec<_>>();
        let forged =
            create_meta_language_archive(&forged_entries, MetaLanguageArchiveLimits::default())
                .unwrap();
        let error = meta_language::restore_project_translation_archive_inputs(
            &forged.network,
            forged.root,
            &json!({}),
        )
        .err()
        .unwrap();
        assert!(
            error.contains("duplicate or foreign project source dependency"),
            "{error}"
        );
    }
}
