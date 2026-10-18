use meta_language::grammar::emit::emit_antlr;
use meta_language::grammar::import::import_antlr;
use meta_language::grammar::interchange::{parse_grammar_links, render_grammar_links};
use meta_language::{
    FeatureParseOptions, GRAMMAR_ROUND_TRIP_MARKER, compile_feature_grammar,
    mutate_grammar_start_rule,
};

#[test]
fn lexical_round_trip_mutation_retains_source_order_and_renumbers_later_implicit_tokens() {
    let original = import_antlr("grammar Start; start: A ' ' A EOF; later: 'later'; A: [a-z]+;")
        .expect("original source imports");
    let before = render_grammar_links(&original);
    let mutated = mutate_grammar_start_rule(&original, GRAMMAR_ROUND_TRIP_MARKER)
        .expect("source-scoped literal extension");
    let (emitted, report) = emit_antlr(&mutated).expect("ANTLR emission");
    assert!(report.lossy.is_empty());
    let reimported = import_antlr(&emitted).expect("the edited emitted source imports");
    let expected = render_grammar_links(&mutated);
    assert_eq!(render_grammar_links(&reimported), expected);
    assert!(expected.contains("lexicalLiteral round-trip-mutation %40literal%3A1"));
    assert!(expected.contains("lexicalLiteral later %40literal%3A2"));
    for candidate in [
        mutated,
        parse_grammar_links(&expected).expect("Links-only reload"),
        reimported,
    ] {
        let options = FeatureParseOptions::default();
        let parser = compile_feature_grammar(&candidate, None, options.clone())
            .expect("complete lexical plan");
        for (text, accepted) in [
            (GRAMMAR_ROUND_TRIP_MARKER, true),
            ("a a", true),
            ("a  a", false),
            ("a", false),
        ] {
            assert_eq!(
                parser
                    .parse_tree(text.as_bytes(), &options)
                    .expect("bounded native parsing")
                    .ok,
                accepted,
                "{text}"
            );
        }
    }
    assert_eq!(render_grammar_links(&original), before);
}

#[test]
fn lexical_round_trip_mutation_reuses_existing_tokens_and_preserves_unknown_aliases() {
    for (source, marker, unresolved) in [
        ("grammar Start; start: B EOF; A: 'a'; B: 'b';", "a", false),
        (
            "grammar Start; start: B ' ' B EOF; A: 'a'; B: 'b';",
            " ",
            false,
        ),
        (
            "grammar Start; start: C EOF; A: 'a'; B: 'a'; C: 'c';",
            "a",
            true,
        ),
    ] {
        let original = import_antlr(source).expect("original source imports");
        let mutated = mutate_grammar_start_rule(&original, marker).expect("literal plan extension");
        let (emitted, report) = emit_antlr(&mutated).expect("source emission");
        assert!(report.lossy.is_empty());
        assert_eq!(
            render_grammar_links(&import_antlr(&emitted).expect("reimport")),
            render_grammar_links(&mutated)
        );
        let compiled = compile_feature_grammar(&mutated, None, FeatureParseOptions::default());
        if unresolved {
            assert_eq!(
                compiled
                    .expect_err("multiple original literal aliases remain unresolved")
                    .reason,
                "operation"
            );
        } else {
            assert!(
                compiled
                    .expect("the existing identity remains executable")
                    .parse_tree(marker.as_bytes(), &FeatureParseOptions::default())
                    .expect("bounded parse")
                    .ok
            );
            assert_eq!(
                mutate_grammar_start_rule(&mutated, marker)
                    .expect("a repeated literal reuses its identity")
                    .declarations(),
                mutated.declarations()
            );
        }
    }
}

#[test]
fn lexical_owner_comparison_checks_actual_bijections_and_keeps_every_other_clause() {
    use meta_language::grammar::interchange::{deserialize_grammar, serialize_grammar};
    use meta_language::grammar::merge::rename_grammar_rule;
    use meta_language::render_grammar_lexical_owner_comparison;
    use serde_json::{Value, json};

    let source = "grammar Start; start: word '!' EOF; word: A {effect();}; A: 'a';";
    let original = import_antlr(source).expect("original owners remain inspectable");
    let renamed = rename_grammar_rule(&original, "word", "term", None, &[])
        .expect("public binding rename")
        .grammar;
    let reimported =
        import_antlr(&source.replace("word", "term")).expect("new source revision imports");
    let actual = render_grammar_links(&renamed);
    assert!(actual.contains("(lexicalOwner word (ref term))"));
    assert_ne!(actual, render_grammar_links(&reimported));
    let expected =
        render_grammar_lexical_owner_comparison(&renamed).expect("checked original owners");
    assert_eq!(
        expected,
        render_grammar_lexical_owner_comparison(&reimported).expect("checked new source owners")
    );
    assert_eq!(render_grammar_links(&renamed), actual);
    let value: Value = serde_json::from_str(&serialize_grammar(&renamed)).expect("grammar wire");
    for mutation in 0..6 {
        let mut current = value.clone();
        let entries = current["declarations"]["lexers"][0]["expression"]["items"]
            .as_array_mut()
            .expect("ordered clauses");
        let find = |kind: &str| {
            entries
                .iter()
                .position(|entry| entry["kind"] == kind)
                .expect("authored source clause")
        };
        let index = find(match mutation {
            0 => "lexicalLiteral",
            1 | 2 => "lexicalRule",
            3 => "lexicalObligation",
            _ => "lexicalOwner",
        });
        match mutation {
            0 => entries[index]["value"] = json!("?"),
            1 => entries[index]["type"] = json!("DIFFERENT"),
            2 => entries[index]["action"]
                .as_array_mut()
                .expect("commands")
                .push(json!({"operation":"pushMode","mode":"default"})),
            3 => entries[index]["source"] = json!("different();"),
            4 => {
                let duplicate = entries[index].clone();
                entries.push(duplicate);
            }
            _ => entries.push(
                json!({"kind":"lexicalOwner","name":"other","item":{"kind":"ref","name":"term"}}),
            ),
        }
        let changed = deserialize_grammar(&serde_json::to_string(&current).expect("wire"))
            .expect("changed clauses stay inspectable");
        let compared = render_grammar_lexical_owner_comparison(&changed);
        if mutation >= 4 {
            assert!(
                compared.is_err(),
                "duplicate and nonbijective original owners refuse comparison"
            );
        } else {
            assert_ne!(
                compared.expect("the owner map is still bijective"),
                expected,
                "a changed literal, token or action cannot disappear"
            );
        }
    }
}
