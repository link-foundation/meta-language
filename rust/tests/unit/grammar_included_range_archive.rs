//! Archive admission parity; executed only by native CI.
use super::*;
use meta_language::grammar::interchange::{deserialize_grammar, serialize_grammar};
use meta_language::grammar::{FeatureExpr, FeatureForm, FieldValue};
use meta_language::{
    GlobalConceptOptions, GlobalConceptSource, LinksEncoder, ToLinks, create_global_concept_space,
    create_global_concept_space_parser, global_concept_space_to_links,
};

fn included_range_archive_settings() -> GlobalConceptOptions {
    GlobalConceptOptions {
        records: Some(json!([])),
        ..GlobalConceptOptions::default()
    }
}

fn included_range_archive_candidate() -> FeatureForm {
    let GrammarExpr::Feature(feature) = parse_links_expression(
        "(grammar-range-candidate Inline included-ranges (source-range 0 1) (source-range 2 3))",
    )
    .unwrap() else {
        panic!("the control is a feature expression")
    };
    let FeatureExpr::Form(form) = *feature else {
        panic!("the control is a form")
    };
    form
}

fn included_range_archive_host(candidate: FeatureForm, mode: &str) -> Grammar {
    Grammar::new()
        .with_start("document")
        .with_rule(GrammarRule::new(
            "document",
            GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
                mode,
                vec![
                    FieldValue::Expression(GrammarExpr::Terminal("a+b".to_owned())),
                    FieldValue::Expressions(vec![GrammarExpr::feature(FeatureExpr::Form(
                        candidate,
                    ))]),
                ],
            ))),
        ))
}

fn included_range_archive_sources(host: Grammar) -> Vec<GlobalConceptSource> {
    [("Outer", host), ("Inline", included_range_literal("ab"))]
        .into_iter()
        .map(|(id, grammar)| GlobalConceptSource {
            id: id.to_owned(),
            language: id.to_owned(),
            edition: String::new(),
            grammar,
        })
        .collect()
}

#[test]
fn source_free_global_archives_execute_original_included_range_cst() {
    for mode in ["any-grammar", "all-grammar"] {
        let mut sources = included_range_archive_sources(included_range_archive_host(
            included_range_archive_candidate(),
            mode,
        ));
        let settings = included_range_archive_settings();
        let space = create_global_concept_space(&sources, &settings).unwrap();
        let options = FeatureParseOptions {
            grammar_name: Some("Outer".to_owned()),
            global_concept_space: Some(space.clone()),
            ..FeatureParseOptions::default()
        };
        let resolve = |name: &str| {
            sources
                .iter()
                .find(|source| source.id == name)
                .map(|source| source.grammar.clone())
        };
        let expected =
            compile_feature_grammar(&sources[0].grammar, Some(&resolve), options.clone())
                .unwrap()
                .parse_tree(b"a+b", &options)
                .unwrap();
        assert!(expected.ok);
        let document: Value =
            serde_json::from_str(&serialize_grammar(&sources[0].grammar)).unwrap();
        assert!(sources[0].grammar.declarations().settling.is_none());
        assert!(
            space["sources"]
                .as_array()
                .unwrap()
                .iter()
                .all(|source| source["context"]["settling"].is_null())
        );
        let default_steps = json!(meta_language::grammar::default_settling("generalized"));
        for settling in [None, Some(Value::Null), Some(default_steps)] {
            let expected_settling = settling.clone().unwrap_or(Value::Null);
            let mut changed = document.clone();
            let declarations = changed
                .as_object_mut()
                .unwrap()
                .entry("declarations")
                .or_insert_with(|| json!({}))
                .as_object_mut()
                .unwrap();
            if let Some(settling) = settling {
                declarations.insert("settling".to_owned(), settling);
            } else {
                declarations.remove("settling");
            }
            let restored = deserialize_grammar(&changed.to_string()).unwrap();
            assert_eq!(json!(restored.declarations().settling), expected_settling);
            let actual = compile_feature_grammar(&restored, Some(&resolve), options.clone())
                .unwrap()
                .parse_tree(b"a+b", &options)
                .unwrap();
            assert!(actual.ok);
            assert_eq!(
                included_range_bundle(&actual),
                included_range_bundle(&expected)
            );
        }
        let mut empty = document;
        empty
            .as_object_mut()
            .unwrap()
            .entry("declarations")
            .or_insert_with(|| json!({}))["settling"] = json!([]);
        let empty = deserialize_grammar(&empty.to_string()).unwrap();
        assert_eq!(empty.declarations().settling, Some(Vec::new()));
        let error = compile_feature_grammar(&empty, Some(&resolve), options.clone()).unwrap_err();
        assert_eq!(error.reason, "declaration");
        assert_eq!(error.message, "the settling names no step");
        let archive = global_concept_space_to_links(&space).unwrap();
        sources.clear();
        for stored in included_range_stored(&json!({"archive":archive})) {
            let parser = create_global_concept_space_parser(
                stored["archive"].as_str().unwrap(),
                "Outer",
                &settings,
                FeatureParseOptions::default(),
            )
            .unwrap();
            let actual = parser
                .parse_tree(b"a+b", &FeatureParseOptions::default())
                .unwrap();
            let expected = included_range_bundle(&expected);
            let actual = included_range_bundle(&actual);
            assert_eq!(actual, expected);
            assert_eq!(included_range_physical(&actual["tree"]), b"a+b".to_vec());
            check_grammar_selection_mapped_tree(&actual, &FeatureParseOptions::default()).unwrap();
        }
    }
}

#[test]
fn typed_source_view_forms_refuse_before_codec_field_projection() {
    for mutation in 0..12 {
        let mut candidate = included_range_archive_candidate();
        match mutation {
            0 => candidate.fields.push(FieldValue::Text("opaque".to_owned())),
            1 => {
                candidate.fields.pop();
            }
            2 => candidate.fields[0] = FieldValue::Word("Inline".to_owned()),
            3 => candidate.fields[1] = FieldValue::Text("included-ranges".to_owned()),
            4 => candidate.fields[2] = FieldValue::Expression(GrammarExpr::Empty),
            10 => candidate.fields[0] = FieldValue::Name(String::new()),
            11 => candidate.fields[1] = FieldValue::Name("identity".to_owned()),
            _ => {
                let FieldValue::Expressions(ranges) = &mut candidate.fields[2] else {
                    unreachable!()
                };
                if mutation == 9 {
                    ranges.clear();
                } else {
                    let GrammarExpr::Feature(feature) = &mut ranges[0] else {
                        unreachable!()
                    };
                    let FeatureExpr::Form(range) = feature.as_mut() else {
                        unreachable!()
                    };
                    match mutation {
                        5 => range.fields.push(FieldValue::Integer(7)),
                        6 => range.fields[0] = FieldValue::Text("0".to_owned()),
                        7 => range.fields[1] = FieldValue::Integer(i64::from(u32::MAX) + 1),
                        8 => range.head = "empty".to_owned(),
                        _ => unreachable!(),
                    }
                }
            }
        }
        let host = included_range_archive_host(candidate, "any-grammar");
        let resolve = |_: &str| Some(included_range_literal("ab"));
        assert!(
            compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default()).is_err(),
            "mutation {mutation}"
        );
        assert!(
            create_global_concept_space(
                &included_range_archive_sources(host.clone()),
                &included_range_archive_settings()
            )
            .is_err(),
            "mutation {mutation}"
        );
        assert!(
            std::panic::catch_unwind(|| render_grammar_links(&host)).is_err(),
            "links mutation {mutation}"
        );
        assert!(
            std::panic::catch_unwind(|| render_native_grammar(&host)).is_err(),
            "listing mutation {mutation}"
        );
        assert!(
            std::panic::catch_unwind(|| serialize_grammar(&host)).is_err(),
            "structured mutation {mutation}"
        );
        assert!(
            std::panic::catch_unwind(|| host.to_links(&mut LinksEncoder::new())).is_err(),
            "network mutation {mutation}"
        );
    }
}

#[test]
fn raw_source_view_records_refuse_before_global_archive_field_loss() {
    let sources = included_range_archive_sources(included_range_archive_host(
        included_range_archive_candidate(),
        "any-grammar",
    ));
    let settings = included_range_archive_settings();
    let original = create_global_concept_space(&sources, &settings).unwrap();
    let document: Value = serde_json::from_str(&serialize_grammar(&sources[0].grammar)).unwrap();
    let candidate_path = "/rules/0/expression/items/0";
    for (suffix, value) in [
        ("/opaqueBinding", json!(true)),
        ("/ranges/0/opaqueBinding", json!(true)),
        ("/ranges/0/start", json!("0")),
        ("/ranges/0/end", json!("1")),
        ("/ranges/0/end", json!(4_294_967_296_u64)),
        ("/name", json!(7)),
        ("/mapping", json!(7)),
        ("/ranges", json!([])),
    ] {
        let mut changed = document.clone();
        let mut keys = suffix.trim_start_matches('/').rsplitn(2, '/');
        let key = keys.next().unwrap();
        let parent = keys.next().map_or_else(
            || candidate_path.to_owned(),
            |path| format!("{candidate_path}/{path}"),
        );
        changed.pointer_mut(&parent).unwrap()[key] = value.clone();
        let candidate = changed.pointer(candidate_path).unwrap().clone();
        assert!(
            deserialize_grammar(&changed.to_string()).is_err(),
            "{suffix}"
        );
        assert!(
            GlobalConceptSource::from_json(
                &json!({"id":"Outer","language":"Outer","grammar":changed})
            )
            .is_err(),
            "{suffix}"
        );
        let mut space = original.clone();
        let definition = space["concepts"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .flat_map(|concept| concept["definitions"].as_array_mut().unwrap())
            .find(|definition| definition["origin"] == "Outer:document")
            .unwrap();
        definition["definition"]["expression"]["items"][0] = candidate;
        assert!(global_concept_space_to_links(&space).is_err(), "{suffix}");
    }
    let floating = document
        .to_string()
        .replace("\"start\":0", "\"start\":-0.0")
        .replace("\"end\":1", "\"end\":1.0");
    assert_ne!(floating, document.to_string());
    let host = deserialize_grammar(&floating).unwrap();
    let resolve = |_: &str| Some(included_range_literal("ab"));
    let outcome = compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default())
        .unwrap()
        .parse_tree(b"a+b", &FeatureParseOptions::default())
        .unwrap();
    assert!(outcome.ok);
}
