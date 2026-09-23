use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use meta_language::{
    analyze_program, decode_program_translation, language_support, translate_program,
    translation_contracts, LinkId, LinkNetwork, LinkQuery, LinkType, ParseConfiguration,
    ProgramConstructStatus, ProgramProjectContext, ReplacementRule, RepresentationLevel,
    TranslationSupport, LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
};
use serde_json::Value;

fn corpus() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    serde_json::from_str(&fs::read_to_string(path).expect("shared corpus is readable"))
        .expect("shared corpus is valid JSON")
}

fn grammar_inventory() -> Value {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../parity/language-grammar-inventory.json");
    serde_json::from_str(&fs::read_to_string(path).expect("grammar inventory is readable"))
        .expect("grammar inventory is valid JSON")
}

#[test]
fn four_language_corpus_produces_lossless_structured_syntax() {
    for fixture in corpus()["languages"].as_array().expect("language fixtures") {
        let language = fixture["name"].as_str().expect("language name");
        let source = fixture["source"].as_str().expect("source");
        let root = fixture["root"].as_str().expect("root term");
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());

        assert_eq!(
            network.reconstruct_text(),
            source,
            "{language} reconstruction"
        );
        assert!(
            network.verify_full_match(None).is_clean(),
            "{language} diagnostics"
        );
        assert!(
            network.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().term() == Some(root)
                    && link.metadata().span().is_some()
            }),
            "{language} root syntax"
        );

        let query = LinkQuery::from_sexpression("(identifier) @identifier")
            .expect("identifier query parses");
        let identifiers = network
            .find(&query)
            .iter()
            .filter_map(|query_match| query_match.captures().first("identifier"))
            .map(|link_id| captured_text(&network, link_id))
            .collect::<Vec<_>>();
        let expected = fixture["identifiers"]
            .as_array()
            .expect("identifiers")
            .iter()
            .map(|value| value.as_str().expect("identifier").to_string())
            .collect::<Vec<_>>();
        assert_eq!(identifiers, expected, "{language} identifiers");
    }
}

#[test]
fn all_four_language_aliases_select_a_structured_frontend() {
    for fixture in corpus()["languages"].as_array().expect("language fixtures") {
        let source = fixture["source"].as_str().expect("source");
        let root = fixture["root"].as_str().expect("root term");
        for alias in fixture["aliases"].as_array().expect("aliases") {
            let alias = alias.as_str().expect("alias");
            let network = LinkNetwork::parse(source, alias, ParseConfiguration::default());
            assert_eq!(network.reconstruct_text(), source, "{alias} reconstruction");
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().term() == Some(root)
                }),
                "{alias} structured root"
            );
        }
    }
}

#[test]
fn formal_language_invalid_input_stays_lossless_and_diagnostic() {
    for fixture in corpus()["negativeCases"]
        .as_array()
        .expect("negative fixtures")
    {
        let source = fixture["source"].as_str().expect("source");
        let language = fixture["language"].as_str().expect("language");
        let diagnostic = fixture["diagnostic"].as_str().expect("diagnostic");
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{diagnostic}");
        assert!(!network.verify_full_match(None).is_clean(), "{diagnostic}");
    }
}

#[test]
fn structured_edits_emit_from_retained_tokens_without_touching_comments_or_strings() {
    for fixture in corpus()["languages"].as_array().expect("language fixtures") {
        let language = fixture["name"].as_str().expect("language");
        let edit = &fixture["edit"];
        let identifier = edit["identifier"].as_str().expect("identifier");
        let mut network = LinkNetwork::parse(
            fixture["source"].as_str().expect("source"),
            language,
            ParseConfiguration::default(),
        );
        let query = LinkQuery::from_sexpression(&format!(
            "(identifier) @target\n(#eq? @target \"{identifier}\")"
        ))
        .expect("identifier query parses");
        let matches = network.find(&query);
        network.replace(
            &matches,
            &ReplacementRule::captured_text(
                "target",
                edit["replacement"].as_str().expect("replacement"),
            ),
        );
        assert_eq!(
            network.reconstruct_text(),
            edit["expected"].as_str().expect("expected output"),
            "{language}"
        );
    }
}

#[test]
fn javascript_grammar_distinguishes_regex_text_and_template_interpolation() {
    let corpus = corpus();
    for fixture in [
        &corpus["javascriptRegressions"]["regularExpression"],
        &corpus["javascriptRegressions"]["templateInterpolation"],
    ] {
        let mut network = LinkNetwork::parse(
            fixture["source"].as_str().expect("source"),
            "JavaScript",
            ParseConfiguration::default(),
        );
        let identifier = fixture["identifier"].as_str().expect("identifier");
        let query = LinkQuery::from_sexpression(&format!(
            "(identifier) @target\n(#eq? @target \"{identifier}\")"
        ))
        .expect("identifier query parses");
        let matches = network.find(&query);
        assert_eq!(
            matches.len(),
            usize::try_from(fixture["matches"].as_u64().expect("match count"))
                .expect("match count fits usize")
        );
        network.replace(
            &matches,
            &ReplacementRule::captured_text(
                "target",
                fixture["replacement"].as_str().expect("replacement"),
            ),
        );
        assert_eq!(
            network.reconstruct_text(),
            fixture["expected"].as_str().expect("expected output")
        );
    }
}

#[test]
fn javascript_grammar_reports_syntactically_invalid_programs() {
    let corpus = corpus();
    let fixture = &corpus["javascriptRegressions"]["invalidProgram"];
    let network = LinkNetwork::parse(
        fixture["source"].as_str().expect("source"),
        "JavaScript",
        ParseConfiguration::default(),
    );
    assert_eq!(
        network.reconstruct_text(),
        fixture["source"].as_str().expect("source")
    );
    assert!(
        !network.verify_full_match(None).is_clean(),
        "{}",
        fixture["diagnostic"].as_str().expect("diagnostic")
    );
}

#[test]
fn ordinary_parse_dispatch_returns_grammar_csts_for_the_audited_inventory() {
    for fixture in corpus()["defaultCstCases"].as_array().expect("CST cases") {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{language}");
        for term in [
            fixture["root"].as_str().expect("root"),
            fixture["requiredNode"].as_str().expect("required node"),
        ] {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().term() == Some(term)
                }),
                "{language} {term}"
            );
        }
    }
}

#[test]
fn every_rust_grammar_inventory_alias_selects_a_nontrivial_lossless_cst() {
    for fixture in grammar_inventory()["languages"]
        .as_array()
        .expect("language inventory")
        .iter()
        .filter(|fixture| fixture["rust"]["status"] == "grammar-cst")
    {
        let source = fixture["source"].as_str().expect("source");
        for alias in fixture["aliases"].as_array().expect("aliases") {
            let alias = alias.as_str().expect("alias");
            let network = LinkNetwork::parse(source, alias, ParseConfiguration::default());
            assert_eq!(network.reconstruct_text(), source, "{alias} reconstruction");
            let syntax = network
                .links()
                .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
                .collect::<Vec<_>>();
            assert!(
                syntax.len() > 1,
                "{alias} must expose grammar nodes below its root"
            );
            assert!(
                syntax.iter().any(|link| link.metadata().span().is_some()),
                "{alias} grammar nodes must retain spans"
            );
        }
    }
}

#[test]
fn rust_markdown_and_json5_frontends_expose_grammar_nodes_and_diagnostics() {
    for (language, source, expected_terms) in [
        (
            "Markdown",
            "# Title\n\nText.\n",
            ["document", "atx_heading", "paragraph"],
        ),
        ("JSON5", "{value: 1,}\n", ["file", "object", "member"]),
    ] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{language}");
        assert!(network.verify_full_match(None).is_clean(), "{language}");
        for expected in expected_terms {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().term() == Some(expected)
                }),
                "{language} must expose the {expected} grammar node"
            );
        }
    }

    let invalid = "{value: }\n";
    let network = LinkNetwork::parse(invalid, "JSON5", ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), invalid);
    assert!(!network.verify_full_match(None).is_clean());
}

#[test]
fn rust_custom_language_frontends_expose_complete_default_csts() {
    let fixtures: [(&str, &str, &[&str]); 6] = [
        ("LiNo", "1 1 1\n", &["lino_document", "link"]),
        ("txt", "Plain text.\n", &["text_document", "line"]),
        (
            "PDF",
            "%PDF-1.7\n%%EOF\n",
            &["pdf_file", "header", "end_of_file"],
        ),
        (
            "DOCX",
            "<w:document><w:body/></w:document>\n",
            &["document", "element"],
        ),
        (
            "English",
            "Hawaii is a state.\n",
            &["natural_language_document", "sentence", "word"],
        ),
        (
            "Mandarin Chinese",
            "你好。\n",
            &["natural_language_document", "sentence", "word"],
        ),
    ];

    for (language, source, expected_terms) in fixtures {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{language}");
        assert!(network.verify_full_match(None).is_clean(), "{language}");
        for expected in expected_terms {
            assert!(
                network.links().any(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().term() == Some(expected)
                }),
                "{language} must expose the {expected} grammar node"
            );
        }
    }

    for (language, source) in [("LiNo", "(broken\n"), ("PDF", "not a PDF\n")] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{language} recovery");
        assert!(
            !network.verify_full_match(None).is_clean(),
            "{language} diagnostics"
        );
    }
}

#[test]
fn capability_reports_match_the_shared_versioned_corpus() {
    let corpus = corpus();
    assert_eq!(
        u64::from(LANGUAGE_REPRESENTATION_SCHEMA_VERSION),
        corpus["schemaVersion"].as_u64().expect("schema version")
    );
    for fixture in corpus["languages"].as_array().expect("language fixtures") {
        let name = fixture["name"].as_str().expect("name");
        let support = language_support(name).expect("registered support");
        assert_eq!(support.version, fixture["version"].as_str().unwrap());
        assert_eq!(support.edition, fixture["edition"].as_str().unwrap());
        assert_eq!(
            support.extensions,
            fixture["extensions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<Vec<_>>()
        );
        assert_eq!(support.binding_resolution, RepresentationLevel::Unavailable);
        assert_eq!(support.type_elaboration, RepresentationLevel::Unavailable);
    }
}

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
    }
}

#[test]
fn binding_aware_rename_preserves_shadowing_unicode_templates_and_comments() {
    for fixture in corpus()["renameCases"].as_array().expect("rename cases") {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let program = analyze_program(source, language, ProgramProjectContext::default())
            .expect("semantic analysis");
        let occurrence = usize::try_from(
            fixture["declarationOccurrence"]
                .as_u64()
                .expect("declaration occurrence"),
        )
        .expect("occurrence fits usize");
        let binding = program
            .bindings()
            .iter()
            .filter(|binding| binding.name() == fixture["binding"].as_str().unwrap())
            .nth(occurrence)
            .expect("selected binding");
        let renamed = program
            .rename_binding(binding.id(), fixture["replacement"].as_str().unwrap())
            .expect("capture-safe rename");
        assert_eq!(
            renamed.emit(),
            fixture["expected"].as_str().expect("expected source"),
            "{language} binding rename"
        );
        assert!(renamed.network().verify_full_match(None).is_clean());
        let error = program
            .rename_binding(binding.id(), fixture["capture"].as_str().unwrap())
            .expect_err("capture must be rejected");
        assert!(
            error.to_string().contains("capture") || error.to_string().contains("conflict"),
            "{language} capture avoidance: {error}"
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
        assert!(contract.obligation.is_none());
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
