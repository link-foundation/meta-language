//! Deferred original Rust oracle, complete definitions and public products.
#[path = "../../src/translation/rust_type_alias_rules.rs"]
mod alias_rules;
#[test]
fn ordinary_source_aliases_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/type-aliases.mjs",
        "ordinary transparent Rust source type aliases",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-type-alias-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn native_alias_publications_keep_actual_values_and_signatures() {
    super::translation_rust_function_qualifiers::assert_rust_publication_products(
        "../experiments/rust-records/type-alias-publications.mjs",
    );
}

#[test]
fn shipped_alias_declaration_rules_match_independent_lexers_and_source_owners() {
    use super::issue_195_translation_pairs::run;
    use meta_language::translation::{
        Language,
        lexer::{TokenKind, tokenize},
    };
    use serde_json::json;
    use std::process::Command;
    let compiler = run(Command::new("rustc").arg("--version"));
    assert!(compiler.ok, "{}", compiler.stderr);
    let version = compiler
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert!(
        version[0] == "1" && (90..100).contains(&version[1].parse::<u32>().unwrap()),
        "uncovered Rust {}",
        compiler.stdout
    );
    let produced =
        run(Command::new("node").arg("../experiments/rust-records/type-alias-plans.mjs"));
    assert!(produced.ok, "{}", produced.stderr);
    let product = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    for row in product["plans"].as_array().unwrap() {
        let tokens = tokenize(row["source"].as_str().unwrap(), Language::Rust)
            .unwrap()
            .tokens;
        let kinds = tokens
            .iter()
            .map(|token| {
                match token.kind {
                    TokenKind::Identifier
                        if matches!(token.value.as_str(), "mod" | "type")
                            && token.raw == token.value =>
                    {
                        "keyword"
                    }
                    TokenKind::Identifier => "identifier",
                    TokenKind::Punct => "punct",
                    TokenKind::Eof => "eof",
                    _ => "atom",
                }
                .to_owned()
            })
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        assert_eq!(json!(kinds), row["kinds"]);
        assert_eq!(json!(values), row["values"]);
        let actual = alias_rules::read_rust_type_alias_declaration_plan(&kinds, &values);
        assert_eq!(json!(actual), row["actual"]);
        assert_eq!(actual[0], "aliases");
        let names = actual[1..]
            .chunks_exact(5)
            .map(|fields| fields[0].clone())
            .collect::<Vec<_>>();
        assert_eq!(json!(names), row["keys"]);
        for fields in actual[1..].chunks_exact(5) {
            assert_eq!(tokens[fields[1].parse::<usize>().unwrap()].raw, "type");
            assert_eq!(tokens[fields[3].parse::<usize>().unwrap()].value, ";");
            assert_eq!(
                fields[4].parse::<usize>().unwrap(),
                fields[3].parse::<usize>().unwrap() + 1
            );
        }
    }
}

#[test]
fn primitive_alias_members_require_complete_matching_owner_facts() {
    super::unsigned_source_documents::require_versions();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-aliases.lino"
    ))
    .unwrap();
    for row in fixture["primitiveOwners"].as_array().unwrap() {
        assert_eq!(
            alias_rules::read_rust_type_alias_primitive_member_owner(
                row["kind"].as_str().unwrap(),
                row["bits"].as_str().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["ownerKind"].as_str().unwrap(),
                row["owner"].as_str().unwrap(),
                row["children"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["suffix"].as_f64().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}

#[test]
fn owned_string_alias_members_keep_the_actual_library_owner() {
    super::unsigned_source_documents::require_versions();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-aliases.lino"
    ))
    .unwrap();
    for row in fixture["stringMembers"].as_array().unwrap() {
        assert_eq!(
            alias_rules::accept_rust_type_alias_owned_string_member(
                row["kind"].as_str().unwrap(),
                row["ownerKind"].as_str().unwrap(),
                row["owner"].as_str().unwrap(),
                row["children"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["member"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
