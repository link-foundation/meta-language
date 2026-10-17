#[test]
fn archived_roots_retain_start_and_rule_order_and_allow_ordinary_start_overrides() {
    let explicit = archived_source(
        "explicit",
        "(grammar (start chosen))\n(rule fallback normal (literal x))\n(rule chosen normal (literal %CF%80))\n",
    );
    let mut implicit = Grammar::new();
    for rule in parse_grammar_links("(grammar (start first))\n(rule first normal (literal %CF%80))\n(rule second normal (literal x))\n").unwrap().rules() {
        implicit.add_rule(rule.clone());
    }
    let sources = [
        explicit,
        GlobalConceptSource {
            id: "implicit".to_owned(),
            language: "implicit".to_owned(),
            edition: String::new(),
            grammar: implicit,
        },
    ];
    let archive = archive_sources(&sources);
    // Archive reconstruction must retain the full serializer's required version;
    // accepting an unversioned grammar would conceal a broken transport boundary.
    for source in &sources {
        let encoded = meta_language::grammar::interchange::serialize_grammar(&source.grammar);
        let document: Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(document["schemaVersion"], json!(1), "{}", source.id);
        let mut imported = json!({
            "id":source.id,"language":source.language,"edition":source.edition,"grammar":document
        });
        let decoded = GlobalConceptSource::from_json(&imported)
            .unwrap_or_else(|error| panic!("{} versioned direct reload: {error}", source.id));
        assert_eq!(
            decoded.grammar.start(),
            source.grammar.start(),
            "{}",
            source.id
        );
        assert_eq!(
            meta_language::grammar::interchange::serialize_grammar(&decoded.grammar),
            encoded,
            "{} direct reload preserves explicit or implicit start and rule order",
            source.id
        );
        imported["grammar"]
            .as_object_mut()
            .unwrap()
            .remove("schemaVersion");
        assert!(
            GlobalConceptSource::from_json(&imported)
                .unwrap_err()
                .contains("invalid serialized grammar document"),
            "{} still requires the actual grammar schema version",
            source.id
        );
    }
    let reloaded = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    for (id, start, order) in [
        ("explicit", json!("chosen"), json!(["fallback", "chosen"])),
        ("implicit", Value::Null, json!(["first", "second"])),
    ] {
        let source = reloaded["sources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|source| source["id"] == id)
            .unwrap();
        assert_eq!(source["start"], start, "{id} archived start");
        assert_eq!(source["ruleOrder"], order, "{id} archived rule order");
    }
    for (id, kind) in [("explicit", "chosen"), ("implicit", "first")] {
        let actual = archive_parser(&archive, id)
            .parse_tree("π".as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(actual.ok, "{:?}", actual.rejection);
        assert!(
            matches!(actual.tree.unwrap(), SyntaxTree::Embed { root, .. } if matches!(*root, SyntaxTree::Node { kind: ref actual, .. } if actual == kind))
        );
    }
    let parser = archive_parser(&archive, "explicit");
    let options = FeatureParseOptions {
        start_rule: Some("fallback".to_owned()),
        ..FeatureParseOptions::default()
    };
    assert!(parser.parse_tree(b"x", &options).unwrap().ok);
    let missing_start = FeatureParseOptions {
        start_rule: Some("absent".to_owned()),
        ..FeatureParseOptions::default()
    };
    let error = parser
        .parse_tree("π".as_bytes(), &missing_start)
        .unwrap_err();
    assert_eq!(error.reason, "reference");
    assert_eq!(error.message, "undefined start rule absent");
    let ParseWithGrammarError::Runtime(parse_error) =
        parser.parse("π".as_bytes(), &missing_start).unwrap_err()
    else {
        panic!("parse preserves the original invalid-start runtime error")
    };
    assert_eq!(parse_error, error);
    let changed = archive.replacen("(start defined chosen)", "(start defined absent)", 1);
    let reordered = archive.replacen(
        "(rule-order fallback chosen)",
        "(rule-order chosen chosen)",
        1,
    );
    for tampered in [&changed, &reordered] {
        assert_ne!(tampered, &archive);
        let existing = read_global_concept_space_links(tampered, &archive_settings()).unwrap_err();
        assert_eq!(
            create_global_concept_space_parser(
                tampered,
                "explicit",
                &archive_settings(),
                FeatureParseOptions::default()
            )
            .unwrap_err()
            .message,
            existing
        );
    }
}
