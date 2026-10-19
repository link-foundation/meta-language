fn source_text_meaning() -> Value {
    let declaration = FormalConceptDefinitionInput::Declaration(None);
    encode_formal_concept_definition(declaration, "source text value")
        .unwrap()
        .to_json()
}

#[test]
fn archived_source_formats_retain_peg_repetition_and_declared_matching_overrides() {
    let listing = "(grammar (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n";
    let grammar = parse_grammar_links(listing).unwrap();
    for format in [
        None,
        Some(GrammarFormat::Bnf),
        Some(GrammarFormat::Ebnf),
        Some(GrammarFormat::Abnf),
        Some(GrammarFormat::Antlr),
        Some(GrammarFormat::Lark),
        Some(GrammarFormat::Gbnf),
        Some(GrammarFormat::TreeSitter),
        Some(GrammarFormat::MetaLanguage),
        Some(GrammarFormat::Inferred),
    ] {
        let mut source = archived_source("generalized-default", listing);
        if let Some(format) = format {
            source.grammar.set_source_format(format);
        }
        let archive = archive_sources(&[source]);
        assert!(
            archive_parser(&archive, "generalized-default")
                .parse_tree(b"a", &FeatureParseOptions::default())
                .unwrap()
                .ok
        );
    }
    let specifications = [
        ("peg-default", Some(GrammarFormat::Peg), None, false),
        ("peg-explicit", Some(GrammarFormat::Peg), Some("peg"), false),
        ("generalized", None, None, true),
        (
            "override",
            Some(GrammarFormat::Peg),
            Some("generalized"),
            true,
        ),
    ];
    let mut sources = specifications
        .iter()
        .map(|(id, format, matching, _)| {
            let mut source = GlobalConceptSource {
                id: (*id).to_owned(),
                language: (*id).to_owned(),
                edition: String::new(),
                grammar: grammar.clone(),
            };
            if let Some(format) = format {
                source.grammar.set_source_format(*format);
            }
            let mut declarations = source.grammar.declarations().clone();
            declarations.matching = matching.map(str::to_owned);
            source.grammar.set_declarations(declarations);
            source
        })
        .collect::<Vec<_>>();
    let meaning = source_text_meaning();
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        formal_definitions: Some(Value::Object(
            sources
                .iter()
                .map(|source| (format!("{}:document", source.id), meaning.clone()))
                .collect(),
        )),
        equivalences: vec![[
            "peg-default:document".to_owned(),
            "peg-explicit:document".to_owned(),
        ]],
        ..GlobalConceptOptions::default()
    };
    let space = create_global_concept_space(&sources, &settings).unwrap();
    assert_eq!(space["failures"], json!([]));
    assert_eq!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["peg-explicit:document"]
    );
    assert_ne!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["generalized:document"]
    );
    assert_ne!(
        space["mappings"]["peg-default:document"],
        space["mappings"]["override:document"]
    );
    let original = space["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|source| source["id"] == "peg-default")
        .unwrap();
    assert!(original["context"]["matching"].is_null());
    let expected = sources
        .iter()
        .zip(&specifications)
        .map(|(source, (_, _, _, accepted))| {
            let ordinary =
                compile_feature_grammar(&source.grammar, None, FeatureParseOptions::default())
                    .unwrap()
                    .parse_tree(b"a", &FeatureParseOptions::default())
                    .unwrap();
            assert_eq!(ordinary.ok, *accepted, "{}", source.id);
            if !ordinary.ok {
                assert_eq!(ordinary.rejection.as_ref().unwrap().reason, "syntax");
            }
            let qualified = compile_feature_grammar(
                &source.grammar,
                None,
                FeatureParseOptions {
                    grammar_name: Some(source.id.clone()),
                    global_concept_space: Some(space.clone()),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap()
            .parse_tree(b"a", &FeatureParseOptions::default())
            .unwrap();
            assert_eq!(qualified.ok, ordinary.ok, "{}", source.id);
            qualified
        })
        .collect::<Vec<_>>();
    let archive = global_concept_space_to_links(&space).unwrap();
    assert!(archive.contains("(grammar-source-format peg-default peg)"));
    assert!(!archive.contains("(grammar-source-format generalized "));
    sources.clear();
    drop(grammar);
    let reloaded = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    assert_eq!(reloaded, space);
    assert_eq!(global_concept_space_to_links(&reloaded).unwrap(), archive);
    for ((id, _, _, _), expected) in specifications.iter().zip(expected) {
        assert_eq!(
            archive_parser(&archive, id)
                .parse_tree(b"a", &FeatureParseOptions::default())
                .unwrap(),
            expected
        );
    }
}

#[test]
fn archived_format_and_matching_tampering_cannot_bypass_current_proof_witnesses() {
    let listing = "(grammar (format peg) (start document))\n(rule document normal (seq (repeat0 (literal a)) (literal a)))\n";
    let mut sources = [
        archived_source("first", listing),
        archived_source("second", listing),
    ];
    for source in &mut sources {
        source.edition.clear();
    }
    let mut declarations = sources[1].grammar.declarations().clone();
    declarations.matching = Some("peg".to_owned());
    sources[1].grammar.set_declarations(declarations);
    let meaning = source_text_meaning();
    let settings = GlobalConceptOptions {
        records: Some(json!([])),
        formal_definitions: Some(json!({"first:document":meaning,"second:document":meaning})),
        equivalences: vec![["first:document".to_owned(), "second:document".to_owned()]],
        ..GlobalConceptOptions::default()
    };
    let archive =
        global_concept_space_to_links(&create_global_concept_space(&sources, &settings).unwrap())
            .unwrap();
    let original = read_global_concept_space_links(&archive, &archive_settings()).unwrap();
    let first_format = "(grammar-source-format first peg)";
    let explicit_format_edit = archive.replacen(
        "(grammar-source-format second peg)",
        "(grammar-source-format second bnf)",
        1,
    );
    assert_ne!(explicit_format_edit, archive);
    let unchanged =
        read_global_concept_space_links(&explicit_format_edit, &archive_settings()).unwrap();
    assert_eq!(
        unchanged["sources"]
            .as_array()
            .unwrap()
            .iter()
            .find(|source| source["id"] == "second")
            .unwrap()["sourceFormat"],
        "bnf"
    );
    assert_eq!(unchanged["mappings"], original["mappings"]);
    assert_eq!(unchanged["proofs"], original["proofs"]);
    assert!(
        !archive_parser(&explicit_format_edit, "second")
            .parse_tree(b"a", &FeatureParseOptions::default())
            .unwrap()
            .ok
    );
    for changed in [
        archive.replacen(first_format, "(grammar-source-format first bnf)", 1),
        archive.replacen("(grammar-source-format first peg)\n", "", 1),
        archive.replacen(
            "(grammar-context second (matching peg))",
            "(grammar-context second (matching generalized))",
            1,
        ),
    ] {
        assert_ne!(changed, archive);
        for id in ["first", "missing"] {
            let error = create_global_concept_space_parser(
                &changed,
                id,
                &archive_settings(),
                FeatureParseOptions::default(),
            )
            .unwrap_err();
            assert!(
                error
                    .message
                    .contains("unproved global concept correspondence")
                    || error.message.contains("one-to-one proof"),
                "{}",
                error.message
            );
        }
    }
    for changed in [
        archive.replacen(first_format, "(grammar-source-format first unknown)", 1),
        archive.replacen(first_format, "(grammar-source-format first (peg))", 1),
        archive.replacen(
            first_format,
            "(grammar-source-format first peg generalized)",
            1,
        ),
        format!("{archive}(grammar-source-format first peg)\n"),
        format!("{archive}(grammar-source-format absent peg)\n"),
        archive.replacen(
            "(grammar-context second (matching peg))",
            "(grammar-context second (matching unknown))",
            1,
        ),
    ] {
        assert_ne!(changed, archive);
        let existing = read_global_concept_space_links(&changed, &archive_settings()).unwrap_err();
        assert!(
            ["format", "matching", "word"]
                .iter()
                .any(|word| existing.contains(word)),
            "{existing}"
        );
        let error = create_global_concept_space_parser(
            &changed,
            "missing",
            &archive_settings(),
            FeatureParseOptions::default(),
        )
        .unwrap_err();
        assert_eq!(error.message, existing);
    }
    let grammar: Value = serde_json::from_str(
        &meta_language::grammar::interchange::serialize_grammar(&sources[0].grammar),
    )
    .unwrap();
    for format in [json!("unknown"), json!(""), json!(0), json!(false)] {
        let mut source = json!({"id":"unsupported","language":"unsupported","grammar":grammar});
        source["grammar"]["sourceFormat"] = format;
        assert!(
            GlobalConceptSource::from_json(&source)
                .unwrap_err()
                .contains("format")
        );
    }
}
