#[test]
fn four_language_semantic_programs_expose_every_required_representation_phase() {
    for fixture in corpus()["semanticPrograms"]
        .as_array()
        .expect("semantic programs")
    {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let project = ProgramProjectContext::new(
            fixture["project"]["root"].as_str().expect("project root"),
            json_strings(&fixture["project"]["files"]),
            json_strings(&fixture["project"]["dependencies"]),
        );
        let program = analyze_program(source, language, project).expect("semantic analysis");
        assert_eq!(program.emit(), source, "{language} source generation");
        assert_eq!(
            program.network().reconstruct_text(),
            source,
            "{language} network"
        );
        assert!(program.diagnostics().is_empty(), "{language} diagnostics");
        assert!(!program.bindings().is_empty(), "{language} bindings");
        assert!(!program.scopes().is_empty(), "{language} scopes");
        assert!(
            !program.source_mappings().is_empty(),
            "{language} source mappings"
        );
        assert!(
            program
                .types()
                .iter()
                .all(|fact| !matches!(fact.phase(), Some("resolved" | "elaborated"))),
            "{language} has only surface type facts"
        );

        for kind in json_strings(&fixture["represented"]) {
            let construct = program
                .constructs()
                .iter()
                .find(|construct| construct.kind() == kind)
                .expect("construct record");
            assert_eq!(
                construct.status(),
                ProgramConstructStatus::Represented,
                "{language} {kind}"
            );
            assert!(
                !construct.evidence().is_empty(),
                "{language} {kind} evidence"
            );
        }
        for kind in json_strings(&fixture["notApplicable"]) {
            let construct = program
                .constructs()
                .iter()
                .find(|construct| construct.kind() == kind)
                .expect("construct record");
            assert_eq!(
                construct.status(),
                ProgramConstructStatus::NotApplicable,
                "{language} {kind}"
            );
            assert!(
                construct.rationale().is_some(),
                "{language} {kind} rationale"
            );
        }
        for kind in json_strings(&fixture["unavailable"]) {
            let construct = program
                .constructs()
                .iter()
                .find(|construct| construct.kind() == kind)
                .expect("construct record");
            assert_eq!(
                construct.status(),
                ProgramConstructStatus::Unavailable,
                "{language} {kind}"
            );
            assert!(
                construct.evidence().is_empty(),
                "{language} {kind} no fabricated trace"
            );
        }
    }
}

#[test]
fn structured_construction_query_edits_cloning_movement_and_emission_reparse_cleanly() {
    for fixture in corpus()["transformationPrograms"]
        .as_array()
        .expect("transformation programs")
    {
        let language = fixture["language"].as_str().unwrap();
        let source = fixture["source"].as_str().unwrap();
        let first_source = fixture["first"].as_str().unwrap();
        let second_source = fixture["second"].as_str().unwrap();
        let inserted = fixture["inserted"].as_str().unwrap();
        let program = construct_program(source, language, ProgramProjectContext::default())
            .expect("structured construction");
        assert_eq!(program.emit(), source, "{language} construct and emit");
        assert!(
            program.query_syntax("identifier").len() >= 2,
            "{language} query"
        );
        let boundary = first_source.len();
        let first = meta_language::ProgramRange::new(0, boundary);
        let second = meta_language::ProgramRange::new(boundary, source.len());
        let replacement = first_source
            .replace("first", "primary")
            .replace("FIRST", "primary");
        assert_eq!(
            program.replace(first, &replacement).unwrap().emit(),
            replacement + second_source,
            "{language} replace"
        );
        assert_eq!(
            program.insert(second.end(), inserted).unwrap().emit(),
            source.to_string() + inserted,
            "{language} insert"
        );
        assert_eq!(
            program.delete(second).unwrap().emit(),
            first_source,
            "{language} delete"
        );
        assert_eq!(
            program.clone_range(first, second.end()).unwrap().emit(),
            source.to_string() + first_source,
            "{language} clone"
        );
        assert_eq!(
            program.move_range(second, 0).unwrap().emit(),
            second_source.to_string() + first_source,
            "{language} move"
        );
        assert!(
            program
                .replace(meta_language::ProgramRange::new(0, source.len() + 1), "")
                .is_err()
        );
        assert!(program.move_range(first, 1).is_err());
    }
    assert!(
        construct_program(
            "const = ;\n",
            "JavaScript",
            ProgramProjectContext::default()
        )
        .is_err()
    );
}

#[test]
fn retained_concrete_syntax_uses_original_links_and_both_coordinate_systems() {
    let source = "const emoji = '😀'; const after = 1;";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let facts = read_program_concrete_syntax_facts(&network, source).unwrap();
    let byte_start = source.find("after").unwrap();
    let fact = facts
        .iter()
        .find(|fact| {
            fact.term.as_deref() == Some("identifier") && fact.byte_range.start() == byte_start
        })
        .expect("the identifier retains its concrete syntax link");
    assert!(network.link(fact.link_id).is_some());
    assert_eq!(fact.byte_range.end(), byte_start + "after".len());
    assert_eq!(fact.start, source[..byte_start].encode_utf16().count());
    assert_eq!(fact.end, fact.start + "after".len());
    assert!(matches!(
        read_program_concrete_syntax_facts(&network, &source.replace("after", "other")),
        Err(meta_language::ProgramRepresentationError::ConcreteSourceMismatch)
    ));
}

#[test]
fn structured_programs_survive_snapshots_without_an_original_source_buffer() {
    for fixture in corpus()["transformationPrograms"]
        .as_array()
        .expect("transformation programs")
    {
        let language = fixture["language"].as_str().unwrap();
        let source = fixture["source"].as_str().unwrap();
        let fragments = vec![
            fixture["first"].as_str().unwrap().to_string(),
            fixture["second"].as_str().unwrap().to_string(),
        ];
        let project = ProgramProjectContext::new(
            format!("/workspace/{language}"),
            vec!["main".to_string()],
            vec!["standard-library".to_string()],
        )
        .with_extensions(vec!["fixture-extension".to_string()]);
        let constructed = construct_program_from_fragments(&fragments, language, project.clone())
            .expect("fragment construction");
        assert_eq!(
            constructed.emit(),
            source,
            "{language} fragment construction"
        );

        let serialized = constructed.serialize_snapshot();
        let snapshot: Value =
            super::project_data::read_project_data(&serialized).expect("snapshot JSON");
        assert!(
            snapshot.get("source").is_none(),
            "{language} source omitted"
        );
        assert!(
            !snapshot["fragments"].as_array().unwrap().is_empty(),
            "{language} retained fragments"
        );
        drop(constructed);

        let restored = ProgramRepresentation::from_snapshot(&serialized).expect("snapshot reload");
        assert_eq!(restored.emit(), source, "{language} snapshot emission");
        assert_eq!(restored.project(), &project, "{language} restored project");
        assert!(restored.network().verify_full_match(None).is_clean());
        assert!(restored.query_syntax("identifier").len() >= 2);

        let mut corrupt = snapshot;
        corrupt["fragments"][0]["byteEnd"] =
            serde_json::json!(corrupt["fragments"][0]["byteEnd"].as_u64().unwrap() + 1);
        assert!(
            ProgramRepresentation::from_snapshot(&corrupt.to_string()).is_err(),
            "{language} rejects corrupt fragment spans"
        );
    }
}

#[test]
fn all_twelve_translation_hooks_emit_reversible_target_native_source_envelopes() {
    let contracts = translation_contracts();
    assert_eq!(contracts.len(), 12);
    let pairs = contracts
        .iter()
        .map(|contract| (contract.source, contract.target))
        .collect::<BTreeSet<_>>();
    assert_eq!(pairs.len(), 12);
    for contract in contracts {
        assert_eq!(contract.support, TranslationSupport::PortableEncoding);
        assert!(contract.encoding.contains("portable source envelope v1"));
        assert!(
            contract
                .obligation
                .as_deref()
                .unwrap()
                .contains("semantic translation is not implemented")
        );
        let fixture = corpus()["semanticPrograms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|fixture| fixture["language"].as_str() == Some(contract.source))
            .cloned()
            .expect("source fixture");
        let source = fixture["source"].as_str().unwrap();
        let translated = translate_program(source, contract.source, contract.target)
            .expect("portable translation");
        assert_eq!(translated.source_language(), contract.source);
        assert_eq!(translated.target_language(), contract.target);
        assert_ne!(translated.code(), source);
        let network = LinkNetwork::parse(
            translated.code(),
            contract.target,
            ParseConfiguration::default(),
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{} -> {} target CST",
            contract.source,
            contract.target
        );
        let decoded = decode_program_translation(translated.code(), contract.target)
            .expect("decode portable translation");
        assert_eq!(decoded.source_language(), contract.source);
        assert_eq!(decoded.source(), source);
        assert_eq!(
            analyze_program(
                decoded.source(),
                decoded.source_language(),
                ProgramProjectContext::default()
            )
            .unwrap()
            .emit(),
            source
        );
        let corrupted =
            translated
                .code()
                .replacen("portable-source-envelope", "portable-source-envelopf", 1);
        assert!(decode_program_translation(&corrupted, contract.target).is_err());
    }
}

fn captured_text(network: &LinkNetwork, root: LinkId) -> String {
    let mut visited = BTreeSet::new();
    let mut tokens = Vec::new();
    collect_tokens(network, root, &mut visited, &mut tokens);
    tokens.sort_by_key(|(start, id, _)| (*start, *id));
    tokens.into_iter().map(|(_, _, text)| text).collect()
}

fn json_strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|entry| entry.as_str().expect("string entry").to_string())
        .collect()
}

fn collect_tokens(
    network: &LinkNetwork,
    root: LinkId,
    visited: &mut BTreeSet<LinkId>,
    tokens: &mut Vec<(usize, u64, String)>,
) {
    if !visited.insert(root) {
        return;
    }
    let Some(link) = network.link(root) else {
        return;
    };
    if link.metadata().link_type() == Some(LinkType::Token) {
        let start = link
            .metadata()
            .span()
            .map_or(usize::MAX, |span| span.byte_range().start());
        tokens.push((
            start,
            link.id().as_u64(),
            link.metadata().term().unwrap_or_default().to_string(),
        ));
        return;
    }
    let children = network
        .links()
        .filter(|candidate| candidate.references().first().copied() == Some(root))
        .map(meta_language::Link::id)
        .collect::<Vec<_>>();
    for child in children {
        collect_tokens(network, child, visited, tokens);
    }
}
