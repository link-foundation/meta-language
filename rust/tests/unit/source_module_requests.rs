//! Deferred original CSTs, shared decision premises and complete graph reloads.
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, SourceModuleRequestLimits,
    analyze_source_module_requests_with_limits, decode_link_network_binary,
    encode_link_network_binary,
};
use sha2::{Digest, Sha256};
#[path = "../../src/translation/source_module_request_rules.rs"]
mod rules;
#[test]
fn actual_cst_fields_retain_every_module_request_and_original_byte() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-module-requests.lino"
    ))
    .unwrap();
    for row in data["grammarProfiles"].as_array().unwrap() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(row["path"].as_str().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())),
            row["sha256"].as_str().unwrap(),
            "uncovered native grammar revision"
        );
    }
    for row in data["roles"].as_array().unwrap() {
        assert_eq!(
            rules::accept_source_module_request_term(row["term"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["scopes"].as_array().unwrap() {
        assert_eq!(
            rules::accept_source_module_request_type_scope(row["term"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["literals"].as_array().unwrap() {
        assert_eq!(
            rules::accept_source_module_request_literal(row["term"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["callees"].as_array().unwrap() {
        assert_eq!(
            rules::accept_source_module_request_import_callee(row["term"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["kinds"].as_array().unwrap() {
        assert_eq!(
            rules::read_source_module_request_kind(row["code"].as_f64().unwrap()),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["codes"].as_array().unwrap() {
        assert_eq!(
            rules::read_source_module_request_code(
                row["language"].as_str().unwrap(),
                row["term"].as_str().unwrap(),
                row["second"].as_str().unwrap(),
                row["third"].as_str().unwrap(),
                row["context"].as_bool().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in data["grammars"].as_array().unwrap() {
        assert_eq!(
            rules::read_source_module_request_grammar(
                row["language"].as_str().unwrap(),
                row["tsx"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in data["commands"].as_array().unwrap() {
        let args = row["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rules::plan_source_module_request_command(&args), expected);
    }
    for row in data["cases"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let packet = analyze_source_module_requests_with_limits(
            source,
            row["language"].as_str().unwrap(),
            SourceModuleRequestLimits {
                tsx: row["tsx"].as_bool().unwrap(),
                ..SourceModuleRequestLimits::default()
            },
        )
        .unwrap();
        assert_eq!(packet.coordinate_unit, "UTF-8 bytes");
        let actual=packet.requests.iter().map(|request|serde_json::json!({"kind":request.kind,"specifier":request.specifier,"specifierUnits":request.specifier_units,"start":request.start,"end":request.end})).collect::<Vec<_>>();
        assert_eq!(serde_json::json!(actual), row["expected"]);
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            assert_eq!(network.reconstruct_text(), source);
            assert_eq!(
                network
                    .link(packet.root)
                    .unwrap()
                    .references()
                    .iter()
                    .map(|id| id.as_u64().to_string())
                    .collect::<Vec<_>>(),
                packet
                    .requests
                    .iter()
                    .map(|request| request.link_id.clone())
                    .collect::<Vec<_>>()
            );
            for request in &packet.requests {
                let id = LinkId::from_u64(request.link_id.parse().unwrap());
                let refs = network.link(id).unwrap().references();
                assert_eq!(refs.len(), 3);
                assert_eq!(
                    network.link(refs[0]).unwrap().metadata().term(),
                    Some(request.kind.as_str())
                );
                assert_eq!(refs[1].as_u64().to_string(), request.syntax_link_id);
                let span = network
                    .link(refs[1])
                    .unwrap()
                    .metadata()
                    .span()
                    .unwrap()
                    .byte_range();
                assert_eq!((span.start(), span.end()), (request.start, request.end));
                let value = network.link(refs[2]).unwrap();
                if let Some(units) = &request.specifier_units {
                    assert_eq!(
                        value
                            .references()
                            .iter()
                            .map(|id| network
                                .link(*id)
                                .unwrap()
                                .metadata()
                                .definition()
                                .unwrap()
                                .parse::<u16>()
                                .unwrap())
                            .collect::<Vec<_>>(),
                        *units
                    );
                } else {
                    assert_eq!(value.metadata().term(), Some("computed module specifier"));
                }
            }
        }
    }
    assert!(
        analyze_source_module_requests_with_limits(
            "import \"a\";",
            "JavaScript",
            SourceModuleRequestLimits {
                byte_limit: 1,
                ..SourceModuleRequestLimits::default()
            }
        )
        .err()
        .unwrap()
        .contains("source byte limit exceeded")
    );
    assert!(
        analyze_source_module_requests_with_limits(
            "import \"a\";",
            "JavaScript",
            SourceModuleRequestLimits {
                work_limit: 1,
                ..SourceModuleRequestLimits::default()
            }
        )
        .err()
        .unwrap()
        .contains("request work limit exceeded")
    );
}

#[test]
fn public_module_request_command_exports_text_and_binary() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let source = "export {value} from \"./a.js\";";
    for binary in [false, true] {
        let mut args = vec!["source.js".to_owned()];
        if binary {
            args.push("--binary".to_owned());
        }
        let result = meta_language::run_source_module_request_command(&args, |path| {
            assert_eq!(path, "source.js");
            Ok(source.to_owned())
        });
        assert_eq!(result.exit_code, 0, "{}", result.stderr);
        let network = if binary {
            decode_link_network_binary(&result.stdout, BinaryLinksOptions::default()).unwrap()
        } else {
            LinkNetwork::from_lino(std::str::from_utf8(&result.stdout).unwrap()).unwrap()
        };
        assert_eq!(network.reconstruct_text(), source);
        assert_eq!(
            network
                .links()
                .filter(|link| link.metadata().term() == Some("source module request"))
                .count(),
            1
        );
    }
    assert_eq!(
        meta_language::run_source_module_request_command(&["--help".to_owned()], |_| Err(
            "must not read".to_owned()
        ))
        .exit_code,
        0
    );
    assert_eq!(
        meta_language::run_source_module_request_command(&[], |_| Err("must not read".to_owned()))
            .exit_code,
        2
    );
}
