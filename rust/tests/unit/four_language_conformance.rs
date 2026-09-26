use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use meta_language::{
    analyze_program, construct_program, construct_program_from_fragments,
    decode_program_translation, language_support, translate_program, translation_contracts, LinkId,
    LinkNetwork, LinkQuery, LinkType, ParseConfiguration, ProgramConstructStatus,
    ProgramProjectContext, ProgramRepresentation, ReplacementRule, RepresentationLevel,
    TranslationSupport, LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::issue_195_observations as observations;

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

fn record_negative_cst_observation(language: &str, assertion_id: &str, fixture_digest: &str) {
    let Some(path) = std::env::var_os("ISSUE_195_OBSERVATION_FILE") else {
        return;
    };
    let normalized = language
        .to_ascii_lowercase()
        .replace('+', "-plus")
        .replace('#', "-sharp");
    let slug = regex::Regex::new("[^a-z0-9]+")
        .expect("valid slug expression")
        .replace_all(&normalized, "-");
    let record = serde_json::json!({
        "testId": format!("i195-cst-{}-rust-negative", slug.trim_matches('-')),
        "assertionId": assertion_id,
        "fixtureId": format!("planned:cst-negative:{language}"),
        "fixtureDigest": fixture_digest,
        "runtime": "rust",
        "commit": std::env::var("ISSUE_195_COMMIT").expect("observation commit"),
        "outcome": "passed",
        "testName": "every_rust_grammar_inventory_frontend_retains_and_diagnoses_prohibited_nul_input",
    });
    // One `write` call, so records of the concurrently running JavaScript and
    // Rust suites never interleave.
    let line = format!("{record}\n");
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("observation file opens");
    let written = file
        .write(line.as_bytes())
        .expect("observation record writes");
    assert_eq!(written, line.len(), "observation record writes at once");
}

#[test]
fn translated_javascript_print_executes_in_rust() {
    let corpus = corpus();
    let fixture = corpus["translationBehaviorCases"]
        .as_array()
        .expect("translation behavior cases")
        .iter()
        .find(|case| case["sourceLanguage"] == "JavaScript" && case["targetLanguage"] == "Rust")
        .expect("JavaScript to Rust case");
    let source_text = fixture["source"].as_str().expect("source");
    let expected_stdout = fixture["expectedStdout"].as_str().expect("stdout");
    let translated = translate_program(source_text, "JavaScript", "Rust")
        .expect("JavaScript to Rust translation");
    let directory = std::env::temp_dir().join(format!(
        "meta-language-translation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir(&directory).expect("temporary directory");
    let source = directory.join("translated.rs");
    let executable = directory.join(format!("translated{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, translated.code()).expect("translated source");
    let mut rustc = Command::new("rustc");
    rustc.args(["--edition", "2024", "--crate-type", "bin"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        rustc.arg("-C").arg(format!("linker={linker}"));
    }
    let compiler = rustc
        .arg("-o")
        .arg(&executable)
        .arg(&source)
        .output()
        .expect("rustc available");
    assert!(
        compiler.status.success(),
        "{}",
        String::from_utf8_lossy(&compiler.stderr)
    );
    let output = Command::new(&executable)
        .output()
        .expect("translated program runs");
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_stdout.as_bytes());
    fs::remove_dir_all(directory).expect("temporary directory cleanup");
    observations::record(&observations::Observation {
        requirement_id: "I195-TRANSLATE-javascript-to-rust",
        suffix: "positive",
        fixture_id: "planned:translation:JavaScript:Rust",
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: &[
            "realTargetArtifact",
            "nativeTargetValidation",
            "semanticPreservationChecked",
        ],
        test_name: "translated_javascript_print_executes_in_rust",
    });
}

#[test]
fn translated_rust_function_exports_javascript_behavior() {
    let corpus = corpus();
    let fixture = corpus["translationBehaviorCases"]
        .as_array()
        .expect("translation behavior cases")
        .iter()
        .find(|case| case["sourceLanguage"] == "Rust" && case["targetLanguage"] == "JavaScript")
        .expect("Rust to JavaScript case");
    let source_text = fixture["source"].as_str().expect("source");
    let exported_name = fixture["export"].as_str().expect("exported name");
    let expected_result = fixture["expectedResult"].as_u64().expect("expected result");
    let translated = translate_program(source_text, "Rust", "JavaScript")
        .expect("Rust to JavaScript translation");
    assert!(translated
        .code()
        .contains("export function answer() { return 42; }"));
    assert_eq!(
        decode_program_translation(translated.code(), "JavaScript")
            .expect("envelope still decodes")
            .source(),
        source_text
    );
    let directory = std::env::temp_dir().join(format!(
        "meta-language-javascript-translation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir(&directory).expect("temporary directory");
    fs::write(directory.join("translated.mjs"), translated.code()).expect("translated module");
    let name = serde_json::to_string(exported_name).expect("export name is JSON-safe");
    let script =
        format!("const module = await import('./translated.mjs'); console.log(module[{name}]());");
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &script])
        .current_dir(&directory)
        .output()
        .expect("Node.js available for translated JavaScript");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, format!("{expected_result}\n").as_bytes());
    fs::remove_dir_all(directory).expect("temporary directory cleanup");
}

#[test]
fn rust_identifier_reserved_by_strict_javascript_stays_transport_only() {
    let translated = translate_program("pub fn public() -> u32 { 42 }", "Rust", "JavaScript")
        .expect("translation descriptor");
    assert_eq!(
        translated.contract().support,
        TranslationSupport::PortableEncoding
    );
    assert!(!translated.code().contains("export function public"));
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

// Tree-sitter starts a root node after its leading padding, so the adapter
// must retain the text before the grammar root itself.
#[test]
fn every_rust_inventory_frontend_retains_whitespace_around_the_grammar_root() {
    for fixture in grammar_inventory()["languages"].as_array().unwrap() {
        let language = fixture["name"].as_str().unwrap();
        let source = format!(" \n\t{}\n \n", fixture["source"].as_str().unwrap());
        let network = LinkNetwork::parse(&source, language, ParseConfiguration::default());
        assert_eq!(
            network.reconstruct_text(),
            source,
            "{language} padded reconstruction"
        );
    }
    let network = LinkNetwork::parse(
        "<script>\n  const value = 1;\n</script>\n",
        "HTML",
        ParseConfiguration::default(),
    );
    let mut embedded = network
        .links()
        .filter(|link| {
            link.metadata().link_type() == Some(LinkType::Token)
                && link.metadata().language() == Some("JavaScript")
        })
        .filter_map(|link| {
            link.metadata().span().map(|span| {
                (
                    span.byte_range().start(),
                    link.metadata().term().unwrap_or_default(),
                )
            })
        })
        .collect::<Vec<_>>();
    embedded.sort_unstable();
    assert_eq!(
        embedded
            .into_iter()
            .map(|(_, term)| term)
            .collect::<String>(),
        "\n  const value = 1;\n",
        "embedded JavaScript region tokens"
    );
}

#[test]
fn every_rust_grammar_inventory_frontend_retains_and_diagnoses_prohibited_nul_input() {
    let inventory_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../parity/language-grammar-inventory.json");
    let inventory_bytes = fs::read(&inventory_path).expect("grammar inventory is readable");
    let fixture_digest = format!("{:x}", Sha256::digest(&inventory_bytes));
    let inventory: Value =
        serde_json::from_slice(&inventory_bytes).expect("grammar inventory is valid JSON");
    for fixture in inventory["languages"].as_array().unwrap() {
        let language = fixture["name"].as_str().unwrap();
        let source = format!("{}\0", fixture["source"].as_str().unwrap());
        let network = LinkNetwork::parse(&source, language, ParseConfiguration::default());
        assert_eq!(
            network.reconstruct_text(),
            source,
            "{language} malformed reconstruction"
        );
        record_negative_cst_observation(language, "malformedInputRetained", &fixture_digest);
        record_negative_cst_observation(language, "exactReconstruction", &fixture_digest);
        let verification = network.verify_full_match(None);
        assert!(
            !verification.issues().is_empty(),
            "{language} malformed diagnostic"
        );
        record_negative_cst_observation(language, "diagnosticReported", &fixture_digest);
        assert!(!verification.is_clean(), "{language} malformed not clean");
        record_negative_cst_observation(language, "notReportedAsClean", &fixture_digest);
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
fn plain_and_natural_language_grammars_keep_control_characters_as_error_nodes() {
    for (language, source, error) in [
        ("txt", "a\u{1}b\n", "\u{1}"),
        ("English", "Hi\u{0}\u{7f} there.\n", "\u{0}\u{7f}"),
        ("Hindi", "नमस्ते\u{1b}।\n", "\u{1b}"),
    ] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source, "{language}");
        assert!(
            !network.verify_full_match(None).is_clean(),
            "{language} diagnostics"
        );
        let errors: Vec<_> = network
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Syntax)
                    && link.metadata().term() == Some("ERROR")
            })
            .collect();
        assert_eq!(errors.len(), 1, "{language} ERROR node");
        assert!(
            errors[0].metadata().flags().is_error(),
            "{language} ERROR flag"
        );
        let range = errors[0]
            .metadata()
            .span()
            .expect("ERROR span")
            .byte_range();
        assert_eq!(
            &source[range.start()..range.end()],
            error,
            "{language} ERROR span"
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
        let capabilities = &fixture["capabilities"];
        assert_eq!(
            representation_level(support.binding_resolution),
            capabilities["bindingResolution"].as_str().unwrap()
        );
        assert_eq!(
            representation_level(support.type_elaboration),
            capabilities["typeElaboration"].as_str().unwrap()
        );
        assert_eq!(
            representation_level(support.dynamic_extensions),
            capabilities["dynamicExtensions"].as_str().unwrap()
        );
        assert_eq!(
            representation_level(support.proof_syntax),
            capabilities["proofSyntax"].as_str().unwrap()
        );
    }
}

#[test]
fn project_aware_analysis_diagnoses_missing_context_and_resolves_dependencies() {
    for fixture in corpus()["semanticPrograms"]
        .as_array()
        .expect("semantic programs")
    {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let without_context = analyze_program(source, language, ProgramProjectContext::default())
            .expect("analysis without context");
        assert!(
            without_context
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == "missing-project-context"),
            "{language} missing project context"
        );

        let project = ProgramProjectContext::new(
            fixture["project"]["root"].as_str().expect("project root"),
            json_strings(&fixture["project"]["files"]),
            json_strings(&fixture["project"]["dependencies"]),
        );
        let with_context = analyze_program(source, language, project).expect("context analysis");
        assert!(
            !with_context
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == "missing-project-context"),
            "{language} supplied project context"
        );
        assert!(
            with_context
                .modules()
                .iter()
                .any(|fact| fact.kind() == "recognized-toolchain-module"),
            "{language} recognized toolchain module"
        );
    }
}

#[test]
fn unrelated_dependency_does_not_resolve_project_import() {
    for fixture in corpus()["semanticPrograms"]
        .as_array()
        .expect("semantic programs")
    {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let project = ProgramProjectContext::new(
            fixture["project"]["root"].as_str().expect("project root"),
            json_strings(&fixture["project"]["files"]),
            vec!["unrelated-package".to_string()],
        );
        let program = analyze_program(source, language, project).expect("analysis");
        assert!(
            program
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == "missing-project-context"),
            "{language} import remains unresolved"
        );
    }
}

#[test]
fn matching_but_nonexistent_dependency_or_file_cannot_resolve_import() {
    for fixture in corpus()["phantomImportCases"]
        .as_array()
        .expect("phantom cases")
    {
        let language = fixture["language"].as_str().expect("language");
        let source = fixture["source"].as_str().expect("source");
        let dependency = fixture["dependency"].as_str().expect("dependency");
        let file = fixture["file"].as_str().expect("file");
        let project = ProgramProjectContext::new(
            "/workspace",
            vec![file.to_string()],
            vec![dependency.to_string()],
        );
        let program = analyze_program(source, language, project).expect("analysis");
        assert!(
            program
                .modules()
                .iter()
                .any(|fact| fact.kind() == "module-import" && fact.name() == dependency),
            "{language} import request"
        );
        assert!(
            program
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == "missing-project-context"),
            "{language} missing module"
        );
        assert!(
            !program
                .modules()
                .iter()
                .any(|fact| fact.kind() == "recognized-toolchain-module"),
            "{language} phantom resolution"
        );
    }
}

const fn representation_level(level: RepresentationLevel) -> &'static str {
    match level {
        RepresentationLevel::Preserved => "preserved",
        RepresentationLevel::ConcreteSyntax => "concrete-syntax",
        RepresentationLevel::Parsed => "parsed",
        RepresentationLevel::Resolved => "resolved",
        RepresentationLevel::Elaborated => "elaborated",
        RepresentationLevel::Opaque => "opaque",
        RepresentationLevel::NotApplicable => "not-applicable",
        RepresentationLevel::Unavailable => "unavailable",
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
        assert!(program
            .replace(meta_language::ProgramRange::new(0, source.len() + 1), "")
            .is_err());
        assert!(program.move_range(first, 1).is_err());
    }
    assert!(construct_program(
        "const = ;\n",
        "JavaScript",
        ProgramProjectContext::default()
    )
    .is_err());
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
        let snapshot: Value = serde_json::from_str(&serialized).expect("snapshot JSON");
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
        assert!(contract
            .obligation
            .as_deref()
            .unwrap()
            .contains("semantic translation is not implemented"));
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
