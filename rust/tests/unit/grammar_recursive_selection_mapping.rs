//! Test-only transport of public syntax fields and official `LiNo` syntax.
//! No grammar/source-mapping decoding or validation algorithms live here.
use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use meta_language::{
    BinaryLinksOptions, FeatureParseOptions, Grammar, GrammarExpr, GrammarRule, LeafText,
    LinkNetwork, OperationValue, ParseOutcome, SyntaxAttributes, SyntaxTree,
    compile_feature_grammar, decode_link_network_binary, decode_structured_data_links,
    encode_link_network_binary, encode_structured_data_links, parse_grammar_links,
    parse_links_expression, percent_encode_links_text, render_grammar_links,
};
use serde_json::{Value, json};
use std::collections::HashMap;

pub fn mapped_fixture(path: &str) -> Value {
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity")
            .join(path),
    )
    .unwrap()
}

pub fn mapped_json_grammar() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap()
}

pub fn mapped_python_grammar() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/python.lino")).unwrap()
}

pub fn mapped_json_host(names: &[(&str, &str)], mode: &str, prefix: &str) -> Grammar {
    let mut grammar = mapped_json_grammar();
    assert!(grammar.rule("mapped_document").is_none());
    let candidates = names
        .iter()
        .map(|(name, mapping)| {
            format!(
                "(grammar-candidate {} {mapping})",
                percent_encode_links_text(name),
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    let selection = parse_links_expression(&format!(
        "({mode} (ref {}) {candidates})",
        grammar.start().unwrap(),
    ))
    .unwrap();
    let expression = if prefix.is_empty() {
        selection
    } else {
        GrammarExpr::Sequence(vec![GrammarExpr::Terminal(prefix.to_owned()), selection])
    };
    let rule = GrammarRule::new("mapped_document", expression).with_source_names(vec![
        meta_language::GrammarSourceName {
            source: "authored".to_owned(),
            name: "mapped_document".to_owned(),
        },
    ]);
    grammar.add_rule(rule);
    grammar.set_start("mapped_document");
    grammar
}

pub fn mapped_literal(source: &str) -> Grammar {
    Grammar::new()
        .with_start("decoded")
        .with_rule(GrammarRule::new(
            "decoded",
            GrammarExpr::Terminal(source.to_owned()),
        ))
}

pub fn mapped_parse(
    grammar: &Grammar,
    grammars: &HashMap<String, Grammar>,
    source: &str,
) -> ParseOutcome {
    let resolve = |name: &str| grammars.get(name).cloned();
    compile_feature_grammar(grammar, Some(&resolve), FeatureParseOptions::default())
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap()
}

fn mapped_attributes(attributes: Option<&SyntaxAttributes>) -> Value {
    attributes.map_or(Value::Null, |fields| {
        Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    let value = match value {
                        OperationValue::Integer(value) => json!(value),
                        OperationValue::Text(value) => json!(value),
                    };
                    (key.clone(), value)
                })
                .collect(),
        )
    })
}

// Exhaustive test-only transport of public fields, not a decoder or validator.
pub fn mapped_tree_data(tree: &SyntaxTree) -> Value {
    let mut value = json!({"start":tree.start(),"end":tree.end()});
    match tree {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            children,
            attributes,
            ..
        } => {
            value["type"] = json!("node");
            value["kind"] = json!(kind);
            value["field"] = json!(field);
            value["trivia"] = json!(trivia);
            value["attributes"] = mapped_attributes(attributes.as_ref());
            value["children"] = json!(children.iter().map(mapped_tree_data).collect::<Vec<_>>());
        }
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            attributes,
            lexical,
            ..
        } => {
            value["type"] = json!("token");
            value["kind"] = json!(kind);
            value["field"] = json!(field);
            value["trivia"] = json!(trivia);
            value["attributes"] = mapped_attributes(attributes.as_ref());
            value["lexical"] = json!(lexical);
        }
        SyntaxTree::Error { reason, .. } => {
            value["type"] = json!("error");
            value["reason"] = json!(reason);
        }
        SyntaxTree::Missing { kind, literal, .. } => {
            value["type"] = json!("missing");
            value["kind"] = json!(kind);
            value["literal"] = json!(literal);
        }
        SyntaxTree::Embed {
            language,
            root,
            source_children,
            selection,
            ..
        } => {
            value["type"] = json!("embed");
            value["language"] = json!(language);
            value["root"] = mapped_tree_data(root);
            if let Some(children) = source_children {
                value["sourceChildren"] =
                    json!(children.iter().map(mapped_tree_data).collect::<Vec<_>>());
            }
            value["selection"] = selection.as_ref().map_or(Value::Null, |selected| {
                let mut data = json!({"mode":selected.mode,"mapping":selected.mapping,
                    "start":selected.start,"end":selected.end,"candidates":selected.candidates,
                    "provenance":selected.provenance,"ambiguous":selected.ambiguous,
                    "alternatives":selected.alternatives.iter().map(|item| json!({
                        "language":item.language,"root":mapped_tree_data(&item.root),"provenance":item.provenance,
                    })).collect::<Vec<_>>()});
                if let Some(mapping) = &selected.source_mapping { data["sourceMapping"].clone_from(mapping); }
                data
            });
        }
    }
    if let SyntaxTree::Token { text, .. } | SyntaxTree::Error { text, .. } = tree {
        match text {
            LeafText::Text(text) => value["text"] = json!(text),
            LeafText::Hex(hex) => {
                value["text"] = Value::Null;
                value["hex"] = json!(hex);
            }
        }
    }
    value
}

pub fn mapped_bundle(outcome: &ParseOutcome) -> Value {
    json!({"tree":outcome.tree.as_ref().map(mapped_tree_data),"sourcePlanes":outcome.source_planes})
}

pub fn mapped_embeds<'a>(tree: &'a Value, found: &mut Vec<&'a Value>) {
    if tree["type"] == "embed" && tree["selection"]["mapping"] == "json-string" {
        found.push(tree);
    }
    for field in ["children", "sourceChildren"] {
        if let Some(children) = tree[field].as_array() {
            for child in children {
                mapped_embeds(child, found);
            }
        }
    }
    if tree.get("root").is_some() {
        mapped_embeds(&tree["root"], found);
    }
    if let Some(witnesses) = tree["selection"]["alternatives"].as_array() {
        for witness in witnesses {
            mapped_embeds(&witness["root"], found);
        }
    }
}

pub fn mapped_first(tree: &Value) -> &Value {
    let mut found = Vec::new();
    mapped_embeds(tree, &mut found);
    found[0]
}

pub fn mapped_physical(tree: &Value) -> Vec<u8> {
    match tree["type"].as_str().unwrap() {
        "missing" => Vec::new(),
        "token" | "error" => tree["text"].as_str().map_or_else(
            || {
                tree["hex"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|word| u8::from_str_radix(std::str::from_utf8(word).unwrap(), 16).unwrap())
                    .collect()
            },
            |text| text.as_bytes().to_vec(),
        ),
        _ => tree
            .get("sourceChildren")
            .or_else(|| tree.get("children"))
            .map_or_else(
                || mapped_physical(&tree["root"]),
                |children| {
                    children
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(mapped_physical)
                        .collect()
                },
            ),
    }
}

pub fn mapped_stored_variants(value: &Value) -> [Value; 2] {
    let graph = encode_structured_data_links(value).unwrap();
    let text = graph.network.to_lino();
    let restored = LinkNetwork::from_lino(&text).unwrap();
    let binary = encode_link_network_binary(&restored, BinaryLinksOptions::default()).unwrap();
    let binary = decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap();
    assert_eq!(
        binary.to_lino(),
        text,
        "all actual link identities and metadata survive"
    );
    [
        decode_structured_data_links(&restored, graph.root).unwrap(),
        decode_structured_data_links(&binary, graph.root).unwrap(),
    ]
}

// The official LiNo syntax is inspectable data. The official grammar parser,
// rather than a test-owned domain decoder, reconstructs all grammar semantics.
pub fn mapped_link_data(node: &LiNo<String>) -> Value {
    match node {
        LiNo::Ref(word) => json!(word),
        LiNo::Link { id, values } => {
            json!({"id":id,"values":values.iter().map(mapped_link_data).collect::<Vec<_>>()})
        }
    }
}

pub fn mapped_link_text(node: &Value) -> String {
    node.as_str().map_or_else(
        || {
            let prefix = node["id"]
                .as_str()
                .map_or(String::new(), |id| format!("{id}: "));
            format!(
                "({prefix}{})",
                node["values"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(mapped_link_text)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        },
        str::to_owned,
    )
}

pub fn mapped_root_expression(grammar: &Grammar, expression: GrammarExpr) -> Grammar {
    let mut restored = Grammar::new();
    let root = grammar.start().unwrap();
    restored.set_start(root);
    if let Some(format) = grammar.source_format() {
        restored.set_source_format(format);
    }
    restored.set_declarations(grammar.declarations().clone());
    restored.set_kinds(grammar.kinds().to_vec());
    for mut rule in grammar.rules().iter().cloned() {
        if rule.name == root {
            rule.expr.clone_from(&expression);
        }
        restored.add_rule(rule);
    }
    restored
}

#[test]
fn mapped_scalar_profile_does_not_relabel_invalid_utf8_or_admit_lexical_escapes() {
    let bytes = [255, 34, 120, 34];
    let grammars = HashMap::from([
        ("decoded".to_owned(), mapped_literal("x")),
        ("raw".to_owned(), mapped_literal("\"x\"")),
    ]);
    let resolve = |name: &str| grammars.get(name).cloned();
    for (name, mapping, accepted) in [("decoded", "json-string", false), ("raw", "identity", true)]
    {
        let host = mapped_json_host(&[(name, mapping)], "any-grammar", "");
        let expression = GrammarExpr::Sequence(vec![
            GrammarExpr::AnyChar,
            host.rule("mapped_document").unwrap().expr.clone(),
        ]);
        let host = mapped_root_expression(&host, expression);
        let result = compile_feature_grammar(&host, Some(&resolve), FeatureParseOptions::default())
            .unwrap()
            .parse_tree(&bytes, &FeatureParseOptions::default())
            .unwrap();
        assert_eq!(result.ok, accepted, "{mapping}: {:?}", result.rejection);
        assert!(
            result.source_planes.is_none(),
            "no invalid UTF-8 plane is declared"
        );
        if accepted {
            let tree = mapped_tree_data(result.tree.as_ref().unwrap());
            assert_eq!(mapped_physical(&tree), bytes);
            let forged = json!({"tree":tree,"sourcePlanes":[{
                "kind":"grammar-source","identity":0,"encoding":"UTF-8","byteLength":4,"derivation":null,
            }]});
            let error = meta_language::check_grammar_selection_mapped_tree(
                &forged,
                &FeatureParseOptions::default(),
            )
            .unwrap_err();
            assert_eq!(error.reason, "selection");
            assert!(error.message.contains("UTF-8"));
        } else {
            assert_eq!(
                result.rejection.unwrap().reason,
                "unsupportedEmbeddingMapping"
            );
        }
    }
    let expression = parse_links_expression(
        "(any-grammar (literal %22x%22) (grammar-candidate decoded json-string))",
    )
    .unwrap();
    let lexical = Grammar::new().with_start("lexical").with_rule(
        GrammarRule::new("lexical", expression).with_kind(meta_language::RuleKind::Token),
    );
    let result = mapped_parse(&lexical, &grammars, "\"x\"");
    assert!(!result.ok);
    assert_eq!(
        result.rejection.unwrap().reason,
        "unsupportedEmbeddingMapping"
    );
    for raw in [r#""\q""#, r#""\u{41}""#, r#""\u12""#, "\"line\nbreak\""] {
        // Authored literal admission isolates the mapping's syntax refusal;
        // the shipped JSON/Python comparisons live in the parent suite.
        let expression = parse_links_expression(&format!(
            "(any-grammar (literal {}) (grammar-candidate decoded json-string))",
            percent_encode_links_text(raw),
        ))
        .unwrap();
        let host = Grammar::new()
            .with_start("document")
            .with_rule(GrammarRule::new("document", expression));
        let result = mapped_parse(&host, &grammars, raw);
        assert!(!result.ok, "{raw}");
        assert_eq!(
            result.rejection.unwrap().reason,
            "invalidEmbeddingMapping",
            "{raw}"
        );
    }
    record_mapped_lexical_inputs();
}

fn mapped_profile_fixture() -> Value {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    let fixture = mapped_fixture("fixtures/grammar-selection-json-string.lino");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["id"], "grammar-selection-json-string");
    let expected_captures = [
        ("inventory", "parity/language-grammar-inventory.lino"),
        (
            "originalConcreteOracle",
            "parity/fixtures/default-cst-expected.lino",
        ),
        (
            "nativeConcreteOracle",
            "parity/fixtures/native-default-cst-expected.lino",
        ),
        (
            "javascriptJsonGrammar",
            "js/src/data/native-grammars/json.lino",
        ),
        (
            "javascriptPythonGrammar",
            "js/src/data/native-grammars/python.lino",
        ),
        ("rustJsonGrammar", "rust/src/data/native-grammars/json.lino"),
        (
            "rustPythonGrammar",
            "rust/src/data/native-grammars/python.lino",
        ),
        (
            "javascriptTests",
            "js/tests/grammar-selection-json-string.test.js",
        ),
        (
            "rustTests",
            "rust/tests/unit/grammar_recursive_selection.rs",
        ),
        (
            "rustTestTransport",
            "rust/tests/unit/grammar_recursive_selection_mapping.rs",
        ),
        (
            "fixtureAuthor",
            "js/tests/support/grammar-selection-mapping-cases.js",
        ),
        (
            "fixtureGenerator",
            "js/scripts/generate-grammar-selection-mapping-fixtures.mjs",
        ),
    ];
    let captures = fixture["captures"].as_array().unwrap();
    assert_eq!(captures.len(), expected_captures.len());
    for (capture, (id, relative)) in captures.iter().zip(expected_captures) {
        assert_eq!(capture["id"], id);
        assert_eq!(capture["path"], relative);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(relative);
        let bytes =
            std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        assert_eq!(json!(bytes.len()), capture["bytes"], "{}", path.display());
        let digest =
            Sha256::digest(bytes)
                .iter()
                .fold(String::with_capacity(64), |mut text, byte| {
                    write!(text, "{byte:02x}").unwrap();
                    text
                });
        assert_eq!(
            digest,
            capture["sha256"].as_str().unwrap(),
            "{}",
            path.display()
        );
    }
    fixture
}

fn record_mapped_profile(
    fixture: &Value,
    section: &str,
    requirement: &str,
    fixture_id: &str,
    assertions: &[&str],
    test_name: &str,
) {
    let declared = &fixture["observations"][section];
    assert_eq!(declared["requirementId"], requirement);
    assert_eq!(declared["suffix"], "json-string-profile");
    assert_eq!(declared["fixtureIds"]["rust"], fixture_id);
    for assertion in assertions {
        assert_eq!(
            declared["assertions"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|candidate| candidate.as_str() == Some(*assertion))
                .count(),
            1
        );
    }
    super::super::issue_195_observations::record(
        &super::super::issue_195_observations::Observation {
            requirement_id: requirement,
            suffix: "json-string-profile",
            fixture_id,
            fixture_file: "parity/fixtures/grammar-selection-json-string.lino",
            assertions,
            test_name,
        },
    );
}

pub fn record_mapped_primary(source: &str, raw: &str, decoded: &str, intervals: &[[usize; 4]]) {
    let fixture = mapped_profile_fixture();
    assert_eq!(fixture["inputs"]["python"]["source"], source);
    let native = mapped_fixture("fixtures/native-default-cst-expected.lino");
    let original = mapped_fixture("fixtures/default-cst-expected.lino");
    let expected = native["languages"]
        .get("Python")
        .unwrap_or_else(|| &original["languages"]["Python"]);
    assert_eq!(
        fixture["inputs"]["python"]["positiveRows"],
        expected["positive"]
    );
    let escape = &fixture["inputs"]["escape"];
    assert_eq!(escape["raw"], raw);
    assert_eq!(escape["decoded"], decoded);
    assert_eq!(escape["physicalBytes"], json!(raw.as_bytes()));
    assert_eq!(escape["decodedBytes"], json!(decoded.as_bytes()));
    assert_eq!(escape["prefixes"], json!(["", "λ<"]));
    assert_eq!(escape["intervals"], json!(intervals));
    record_mapped_profile(
        &fixture,
        "boundaries",
        "I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS",
        "grammar-selection-json-string-boundaries-rust",
        &[
            "shippedJsonPythonConcreteTreesRemainExact",
            "jsonStringUtf8IntervalsAndQuoteOwnershipExact",
        ],
        "mapped_json_retains_shipped_python_cst_original_oracle_rows_and_exact_escape_intervals",
    );
}

pub fn record_mapped_archives(source: &str, inner: &str, physical: &str) {
    let fixture = mapped_profile_fixture();
    assert_eq!(fixture["inputs"]["python"]["source"], source);
    assert_eq!(serde_json::to_string(source).unwrap(), inner);
    assert_eq!(serde_json::to_string(inner).unwrap(), physical);
    let test_name =
        "nested_mapped_registry_and_complete_grammars_replay_from_text_and_binary_links";
    record_mapped_profile(
        &fixture,
        "boundaries",
        "I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS",
        "grammar-selection-json-string-boundaries-rust",
        &["mappedSourceRegistryAndGrammarReplayTextAndBinary"],
        test_name,
    );
    record_mapped_profile(
        &fixture,
        "recursion",
        "I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES",
        "grammar-selection-json-string-recursion-rust",
        &["nestedMappedSourcesRetainActualDerivationOrder"],
        test_name,
    );
}

pub fn record_mapped_forgeries(replacements: &[Value], absent: &Value) {
    let fixture = mapped_profile_fixture();
    let cases = &fixture["runtimeCases"]["rust"]["serializableForgeries"];
    assert_eq!(cases["replacements"], json!(replacements));
    assert_eq!(
        cases["extraFields"],
        json!([
            "/sourcePlanes/0",
            "/sourcePlanes/1/derivation",
            "/tree/children/0/selection/sourceMapping",
        ])
    );
    assert_eq!(cases["missingRegistry"], true);
    for restored in mapped_stored_variants(absent) {
        assert_eq!(&restored, absent);
        assert_eq!(
            meta_language::check_grammar_selection_mapped_tree(
                &restored,
                &FeatureParseOptions::default()
            )
            .unwrap_err()
            .reason,
            "selection"
        );
    }
    record_mapped_profile(
        &fixture,
        "boundaries",
        "I195-CST-RECURSIVE-BOUNDARIES-LOSSLESS",
        "grammar-selection-json-string-boundaries-rust",
        &["serializableMappingForgeriesRefusedAfterReload"],
        "mapped_source_forgeries_survive_generic_archives_but_fail_the_correspondence_checker",
    );
}

pub fn record_mapped_profiles(recovery_source: &str) {
    let fixture = mapped_profile_fixture();
    assert_eq!(
        fixture["inputs"]["python"]["recoverySource"],
        recovery_source
    );
    assert_eq!(
        fixture["inputs"]["selection"]["modes"],
        json!(["any-grammar", "all-grammar"])
    );
    let refusals = &fixture["inputs"]["refusals"];
    assert_eq!(
        refusals["loneSurrogateEscapes"],
        json!([r#""\uD800""#, r#""\uDC00""#])
    );
    assert_eq!(refusals["unsupportedMapping"], "quoted-escape");
    assert_eq!(
        refusals["mixedMappings"],
        json!(["json-string", "identity"])
    );
    assert_eq!(
        refusals["limitOptions"],
        json!([
            {"options":{"stepLimit":0},"reason":"stepLimit"},
            {"options":{"memoryLimit":0},"reason":"memoryBudget"},
            {"options":{"maxDepth":0},"reason":"nestingDepth"},
        ])
    );
    let test_name =
        "mapped_profiles_keep_witnesses_and_existing_limits_while_failed_derivations_are_not_csts";
    record_mapped_profile(
        &fixture,
        "selection",
        "I195-CST-ANY-GRAMMAR-RECURSIVE-RULE",
        "grammar-selection-json-string-selection-rust",
        &[
            "jsonStringAnyAllCandidateWitnessesPreserved",
            "jsonStringUnsupportedAndNonScalarProfilesRefused",
        ],
        test_name,
    );
    record_mapped_profile(
        &fixture,
        "recursion",
        "I195-CST-DEEPLY-RECURSIVE-MIXED-LANGUAGES",
        "grammar-selection-json-string-recursion-rust",
        &[
            "mappedParsersRetainExistingLimitRefusals",
            "mappedCycleAndSyntaxFailuresRetainSourceRoutes",
            "failedMappedParseCannotAuthenticateMissingRoot",
        ],
        test_name,
    );
}

pub fn record_mapped_memo() {
    let fixture = mapped_profile_fixture();
    let memo = &fixture["inputs"]["memo"];
    assert_eq!(memo["decoded"], "x");
    assert_eq!(memo["identityName"], "json-string|decoded");
    assert_eq!(memo["warmMappings"], json!(["identity", "json-string"]));
    record_mapped_profile(
        &fixture,
        "selection",
        "I195-CST-ANY-GRAMMAR-RECURSIVE-RULE",
        "grammar-selection-json-string-selection-rust",
        &["mappedProfileMemoKeysKeepExactGrammarNames"],
        "mapped_selection_memo_keys_keep_delimiter_bearing_names_separate_from_profiles",
    );
}

fn record_mapped_lexical_inputs() {
    let fixture = mapped_profile_fixture();
    let refusals = &fixture["inputs"]["refusals"];
    assert_eq!(refusals["invalidBytes"], json!([255, 34, 120, 34]));
    assert_eq!(
        refusals["invalidEscapes"],
        json!([r#""\q""#, r#""\u{41}""#, r#""\u12""#, "\"line\nbreak\"",])
    );
    assert_eq!(fixture["inputs"]["nesting"]["oneLevel"], "\"x\"");
    record_mapped_profile(
        &fixture,
        "selection",
        "I195-CST-ANY-GRAMMAR-RECURSIVE-RULE",
        "grammar-selection-json-string-selection-rust",
        &[
            "jsonStringLexicalAndMalformedInputsRefused",
            "invalidUtf8RootCannotClaimUtf8SourcePlane",
        ],
        "mapped_scalar_profile_does_not_relabel_invalid_utf8_or_admit_lexical_escapes",
    );
}

pub fn check_nested_mapped_registry_and_complete_grammar_archives() {
    let started = std::time::Instant::now();
    let report_stage = |phase: &str, format: &str, source_bytes: usize| {
        eprintln!(
            "mapped archive phase={phase}; format={format}; elapsedMillis={}; retainedSourceBytes={source_bytes}",
            started.elapsed().as_millis()
        );
    };
    let inventory = mapped_fixture("language-grammar-inventory.lino");
    let source = inventory["languages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == "Python")
        .unwrap()["source"]
        .as_str()
        .unwrap();
    let middle = mapped_json_host(&[("Python", "json-string")], "any-grammar", "");
    let outer = mapped_json_host(&[("Middle", "json-string")], "any-grammar", "");
    let grammars = HashMap::from([
        ("outer".to_owned(), outer),
        ("Middle".to_owned(), middle),
        ("Python".to_owned(), mapped_python_grammar()),
    ]);
    let inner_source = serde_json::to_string(source).unwrap();
    let physical = serde_json::to_string(&inner_source).unwrap();
    let retained_source_bytes = source.len() + inner_source.len() + physical.len();
    report_stage(
        "parse nested source planes",
        "direct",
        retained_source_bytes,
    );
    let outcome = mapped_parse(&grammars["outer"], &grammars, &physical);
    report_stage(
        "parsed nested source planes",
        "direct",
        retained_source_bytes,
    );
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let bundle = mapped_bundle(&outcome);
    let mut embeds = Vec::new();
    mapped_embeds(&bundle["tree"], &mut embeds);
    assert_eq!(embeds.len(), 2);
    assert_eq!(bundle["sourcePlanes"].as_array().unwrap().len(), 3);
    assert_eq!(
        bundle["sourcePlanes"][2]["derivation"],
        json!({"operation":"json-string","input":1,"start":0,"end":inner_source.len()})
    );
    assert_eq!(
        embeds[0]["selection"]["sourceMapping"]["output"],
        embeds[1]["selection"]["sourceMapping"]["input"]
    );
    assert_eq!(embeds[1]["selection"]["sourceMapping"]["output"], 2);
    for (embedded, physical) in embeds
        .iter()
        .zip([physical.as_str(), inner_source.as_str()])
    {
        let direct_json = mapped_parse(&mapped_json_grammar(), &HashMap::new(), physical);
        assert!(direct_json.ok);
        assert_eq!(
            embedded["sourceChildren"],
            json!([mapped_tree_data(direct_json.tree.as_ref().unwrap())])
        );
        assert_eq!(mapped_physical(embedded), physical.as_bytes());
    }
    assert_eq!(
        embeds[1]["root"],
        mapped_tree_data(
            mapped_parse(&grammars["Python"], &HashMap::new(), source)
                .tree
                .as_ref()
                .unwrap()
        )
    );
    let documents = ["outer", "Middle", "Python"].map(|name| {
        let syntax = parse_lino_to_links_with_config(
            &render_grammar_links(&grammars[name]),
            &ParserConfig::without_comments(),
        )
        .unwrap();
        json!({"name":name,"syntax":syntax.iter().map(mapped_link_data).collect::<Vec<_>>()})
    });
    let archive = json!({"grammars":documents,"bundle":bundle});
    report_stage(
        "encode and reload complete text and binary archives",
        "both",
        retained_source_bytes,
    );
    let stored_variants = mapped_stored_variants(&archive);
    report_stage(
        "reloaded complete text and binary archives",
        "both",
        retained_source_bytes,
    );
    for (format, stored) in ["text", "binary"].into_iter().zip(stored_variants) {
        assert_eq!(stored, archive);
        report_stage(
            "check complete mapped source correspondence",
            format,
            retained_source_bytes,
        );
        check_grammar_selection_mapped_tree(&stored["bundle"], &FeatureParseOptions::default())
            .unwrap();
        report_stage(
            "reparse complete grammar definitions",
            format,
            retained_source_bytes,
        );
        let restored: HashMap<_, _> = stored["grammars"]
            .as_array()
            .unwrap()
            .iter()
            .map(|document| {
                let listing = document["syntax"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(mapped_link_text)
                    .collect::<Vec<_>>()
                    .join("\n");
                let grammar = parse_grammar_links(&listing).unwrap();
                let name = document["name"].as_str().unwrap();
                assert_eq!(
                    grammar, grammars[name],
                    "full rule/context/source/meaning metadata, not only listing syntax"
                );
                assert_eq!(
                    render_grammar_links(&grammar),
                    render_grammar_links(&grammars[name])
                );
                (name.to_owned(), grammar)
            })
            .collect();
        report_stage(
            "execute restored complete grammars",
            format,
            retained_source_bytes,
        );
        let replay = mapped_parse(&restored["outer"], &restored, &physical);
        assert_eq!(
            replay, outcome,
            "all CST, registry, ambiguity and rejection fields replay"
        );
        assert_eq!(mapped_bundle(&replay), stored["bundle"]);
        report_stage(
            "verified restored complete grammars",
            format,
            retained_source_bytes,
        );
    }
    record_mapped_archives(source, &inner_source, &physical);
}
