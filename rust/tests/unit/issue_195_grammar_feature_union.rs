//! Requirement I195-GRAMMAR-FEATURE-UNION: the native grammar representation
//! and its executor cover every feature of docs/vision.md#grammar-feature-union.
//! parity/fixtures/grammar-feature-union.json holds, per feature, a native
//! grammar listing, positive inputs with their expected concrete syntax trees,
//! negative inputs with their expected rejections and a grammar mutation that
//! changes the outcome; this suite runs every case through the public crate
//! API and compares the outcome with the fixture byte for byte, as
//! js/tests/issue-195-grammar-feature-union.test.js does for JavaScript.
//! docs/grammar/feature-union.md is the specification.

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use meta_language::grammar::feature::{
    ByteClassItem, FeatureExpr, FeatureForm, FieldValue, UnicodeClassItem, operation_form,
};
use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, Grammar, GrammarExpr, GrammarRuntimeError,
    ParseOutcome, RuleKind, compile_feature_grammar, deserialize_grammar, parse_grammar_links,
    parse_native_grammar, render_grammar_links, render_native_grammar, serialize_grammar,
};
use regex::Regex;
use serde_json::{Map, Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/grammar-feature-union.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-feature-union.json");
const VISION: &str = include_str!("../../../docs/vision.md");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-FEATURE-UNION",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-feature-union",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the feature union fixture is JSON")
}

fn features(fixture: &Value) -> &[Value] {
    fixture["features"].as_array().expect("features")
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is text"))
}

fn native(listing: &str) -> Grammar {
    parse_native_grammar(listing).expect("the listing parses")
}

fn languages(fixture: &Value) -> HashMap<String, Grammar> {
    fixture["languages"]
        .as_object()
        .expect("languages")
        .iter()
        .map(|(name, listing)| (name.clone(), native(listing.as_str().expect("listing"))))
        .collect()
}

/// The bullet list of docs/vision.md#grammar-feature-union, without its punctuation.
fn vision_feature_titles() -> Vec<String> {
    let section = &VISION[VISION.find("## Grammar feature union").expect("section")..];
    let list_start = section.find("\n- ").expect("list");
    let list_end = list_start + section[list_start..].find("\n\n").expect("list end");
    section[list_start..list_end]
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .map(|line| line.trim_end_matches([';', '.']).to_owned())
        .collect()
}

/// The fixture's parse options, `{maxDepth, stepLimit, ambiguity, recovery}`.
fn options_of(value: Option<&Value>) -> FeatureParseOptions {
    let Some(value) = value else {
        return FeatureParseOptions::default();
    };
    let limit = |key: &str| {
        value
            .get(key)
            .and_then(Value::as_u64)
            .map(|limit| usize::try_from(limit).expect("limit fits"))
    };
    FeatureParseOptions {
        max_depth: limit("maxDepth"),
        step_limit: limit("stepLimit"),
        reject_ambiguity: value.get("ambiguity").map(|policy| policy == "reject"),
        accept_recovery: value.get("recovery").map(|policy| policy == "accept"),
        ..FeatureParseOptions::default()
    }
}

fn input_of(item: &Value) -> Vec<u8> {
    if let Some(input) = item["input"].as_str() {
        return input.as_bytes().to_vec();
    }
    let hex = text(item, "inputHex");
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex pair"))
        .collect()
}

fn label(item: &Value) -> String {
    item["input"]
        .as_str()
        .or_else(|| item["inputHex"].as_str())
        .unwrap_or_default()
        .to_owned()
}

struct Suite {
    languages: HashMap<String, Grammar>,
}

impl Suite {
    fn new(fixture: &Value) -> Self {
        Self {
            languages: languages(fixture),
        }
    }

    fn compile(
        &self,
        grammar: &Grammar,
        feature: &Value,
    ) -> Result<FeatureGrammarParser, GrammarRuntimeError> {
        let resolve = |name: &str| self.languages.get(name).cloned();
        compile_feature_grammar(grammar, Some(&resolve), options_of(feature.get("options")))
    }

    fn parser(&self, grammar: &Grammar, feature: &Value) -> FeatureGrammarParser {
        self.compile(grammar, feature)
            .unwrap_or_else(|error| panic!("{}: {error}", feature["id"]))
    }

    fn load_failure(&self, listing: &str, feature: &Value) -> Value {
        match self.compile(&native(listing), feature) {
            Ok(_) => Value::Null,
            Err(error) => json!({ "reason": error.reason }),
        }
    }
}

fn run(parser: &FeatureGrammarParser, item: &Value) -> ParseOutcome {
    parser
        .parse_tree(&input_of(item), &options_of(item.get("options")))
        .expect("the start rule exists")
}

/// The outcome of one parse in the fixture's shape: the rendered tree, the
/// reported ambiguities and the rejection, each only when present.
fn summary(result: &ParseOutcome) -> Value {
    assert_eq!(result.ok, result.rejection.is_none());
    let mut summary = Map::new();
    if let Some(tree) = &result.tree {
        summary.insert("tree".into(), Value::String(tree.render()));
    }
    if !result.ambiguities.is_empty() {
        let ambiguities = result
            .ambiguities
            .iter()
            .map(|found| json!({"rule": found.rule, "start": found.start, "end": found.end}))
            .collect();
        summary.insert("ambiguities".into(), Value::Array(ambiguities));
    }
    if let Some(rejection) = &result.rejection {
        let mut shape = Map::new();
        shape.insert("reason".into(), json!(rejection.reason));
        for (key, value) in [
            ("offset", rejection.offset),
            ("line", rejection.line),
            ("column", rejection.column),
            ("limit", rejection.limit),
        ] {
            if let Some(value) = value {
                shape.insert(key.into(), json!(value));
            }
        }
        if let Some(expected) = &rejection.expected {
            shape.insert("expected".into(), json!(expected));
        }
        summary.insert("rejection".into(), Value::Object(shape));
    }
    Value::Object(summary)
}

fn outcome(parser: &FeatureGrammarParser, item: &Value) -> Value {
    summary(&run(parser, item))
}

fn expected(item: &Value) -> Value {
    let mut expected = Map::new();
    for key in ["tree", "ambiguities", "rejection"] {
        if let Some(value) = item.get(key) {
            expected.insert(key.into(), value.clone());
        }
    }
    Value::Object(expected)
}

/// What a grammar uses, as names: expression kinds (with the choice order and
/// class item kinds), rule kinds and fields, declarations, operations, and
/// left recursion; the names of the JavaScript suite's `formsOf`.
fn forms_of(grammar: &Grammar) -> BTreeSet<String> {
    let mut forms = BTreeSet::new();
    for rule in grammar.rules() {
        let kind = match rule.kind {
            RuleKind::Normal => "normal",
            RuleKind::Atomic => "atomic",
            RuleKind::Silent => "silent",
            RuleKind::Token => "token",
        };
        forms.insert(format!("rule:{kind}"));
        let attributes = &rule.attributes;
        for (field, present) in [
            ("parameters", !attributes.parameters.is_empty()),
            ("channel", attributes.channel.is_some()),
            ("modes", attributes.modes.is_some()),
            ("action", attributes.action.is_some()),
        ] {
            if present {
                forms.insert(format!("rule:{field}"));
            }
        }
        visit_expression(&rule.expr, &mut forms);
        for operation in attributes.action.iter().flatten() {
            visit_operation(operation, &mut forms);
        }
    }
    let declarations = grammar.declarations();
    for present in declarations.present() {
        forms.insert(format!("declaration:{present}"));
    }
    for extra in &declarations.extras {
        visit_expression(extra, &mut forms);
    }
    for declared in &declarations.macros {
        visit_expression(&declared.expression, &mut forms);
    }
    for scanner in &declarations.scanners {
        for operation in &scanner.operations {
            visit_operation(operation, &mut forms);
        }
    }
    if left_recursive(grammar) {
        forms.insert("left-recursion".into());
    }
    forms
}

fn visit_fields(form: &FeatureForm, forms: &mut BTreeSet<String>) {
    for value in &form.fields {
        match value {
            FieldValue::Expression(expr) => visit_expression(expr, forms),
            FieldValue::Expressions(items) => {
                for item in items {
                    visit_expression(item, forms);
                }
            }
            FieldValue::Operation(operation) => visit_operation(operation, forms),
            FieldValue::Operations(items) | FieldValue::Block(Some(items)) => {
                for operation in items {
                    visit_operation(operation, forms);
                }
            }
            _ => {}
        }
    }
}

fn visit_operation(operation: &FeatureForm, forms: &mut BTreeSet<String>) {
    forms.insert(format!("operation:{}", operation.head));
    visit_fields(operation, forms);
}

fn visit_expression(expr: &GrammarExpr, forms: &mut BTreeSet<String>) {
    let kind = match expr {
        GrammarExpr::Choice { ordered, .. } => {
            if *ordered {
                "choice:ordered"
            } else {
                "choice:unordered"
            }
        }
        GrammarExpr::Capture { .. } => "capture",
        GrammarExpr::NonTerminal(_) => "ref",
        GrammarExpr::Feature(feature) => match &**feature {
            FeatureExpr::Form(form) => {
                forms.insert(form.head.clone());
                visit_fields(form, forms);
                return;
            }
            FeatureExpr::ByteClass { items, .. } => {
                for item in items {
                    forms.insert(match item {
                        ByteClassItem::Byte(_) => "byteClass:byte".into(),
                        ByteClassItem::Range(..) => "byteClass:byteRange".into(),
                    });
                }
                "byteClass"
            }
            FeatureExpr::UnicodeClass { items, .. } => {
                for item in items {
                    forms.insert(match item {
                        UnicodeClassItem::Char(_) => "charClass:char".into(),
                        UnicodeClassItem::Range(..) => "charClass:range".into(),
                        UnicodeClassItem::Category(_) => "charClass:category".into(),
                        UnicodeClassItem::Script(_) => "charClass:script".into(),
                    });
                }
                "charClass"
            }
            FeatureExpr::Call { arguments, .. } => {
                forms.insert("ref:arguments".into());
                for argument in arguments {
                    visit_expression(argument, forms);
                }
                "ref"
            }
        },
        _ => "plain",
    };
    forms.insert(kind.into());
    for child in direct_children(expr) {
        visit_expression(child, forms);
    }
}

fn direct_children(expr: &GrammarExpr) -> Vec<&GrammarExpr> {
    match expr {
        GrammarExpr::Choice { alternatives, .. } => alternatives.iter().collect(),
        GrammarExpr::Sequence(items) => items.iter().collect(),
        GrammarExpr::Optional(item)
        | GrammarExpr::ZeroOrMore(item)
        | GrammarExpr::OneOrMore(item)
        | GrammarExpr::And(item)
        | GrammarExpr::Not(item)
        | GrammarExpr::Repeat { expr: item, .. }
        | GrammarExpr::Capture { expr: item, .. } => vec![&**item],
        _ => Vec::new(),
    }
}

/// The rules an expression can call before consuming input: the first item
/// of sequences, every alternative of choices and longest matches, and the
/// `item` of every other form.
fn leftmost<'a>(expr: &'a GrammarExpr, found: &mut Vec<&'a str>) {
    match expr {
        GrammarExpr::NonTerminal(name) => found.push(name),
        GrammarExpr::Sequence(items) => {
            if let Some(first) = items.first() {
                leftmost(first, found);
            }
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for item in alternatives {
                leftmost(item, found);
            }
        }
        GrammarExpr::Feature(feature) => match &**feature {
            FeatureExpr::Call { name, .. } => found.push(name),
            FeatureExpr::Form(form) if form.head == "longest" => {
                for item in form.expressions("items") {
                    leftmost(item, found);
                }
            }
            FeatureExpr::Form(form) => {
                if let Some(item) = form.expression("item") {
                    leftmost(item, found);
                }
            }
            _ => {}
        },
        _ => {
            for child in direct_children(expr) {
                leftmost(child, found);
            }
        }
    }
}

fn left_recursive(grammar: &Grammar) -> bool {
    grammar.rules().iter().any(|rule| {
        let mut seen = BTreeSet::new();
        let mut pending = Vec::new();
        leftmost(&rule.expr, &mut pending);
        while let Some(name) = pending.pop() {
            if name == rule.name {
                return true;
            }
            if !seen.insert(name) {
                continue;
            }
            if let Some(called) = grammar.rule(name) {
                leftmost(&called.expr, &mut pending);
            }
        }
        false
    })
}

/// The forms each feature's grammar must use, so a fixture grammar cannot
/// pass a feature without representing it.
const FEATURE_FORMS: &[(&str, &[&str])] = &[
    ("alternatives", &["choice:ordered", "choice:unordered"]),
    ("recursion", &["left-recursion"]),
    ("precedence", &["precedence"]),
    (
        "ambiguity",
        &[
            "declaration:conflicts",
            "declaration:matching",
            "dynamicPrecedence",
        ],
    ),
    ("lexical", &["longest", "lexicalPrecedence"]),
    (
        "unicode",
        &[
            "charClass:category",
            "charClass:script",
            "byteClass",
            "byteClass:byteRange",
        ],
    ),
    ("trivia", &["declaration:extras", "token", "immediateToken"]),
    (
        "modes",
        &[
            "declaration:modes",
            "rule:modes",
            "rule:channel",
            "operation:pushMode",
            "operation:popMode",
        ],
    ),
    (
        "layout",
        &[
            "declaration:scanners",
            "operation:emit",
            "operation:push",
            "operation:pop",
            "predicate",
        ],
    ),
    (
        "predicates",
        &["predicate", "rule:action", "operation:fieldText"],
    ),
    (
        "actions",
        &[
            "rule:action",
            "operation:setAttribute",
            "operation:buildNode",
            "operation:sumOf",
        ],
    ),
    ("fields", &["capture", "alias"]),
    (
        "parameterization",
        &["rule:parameters", "parameter", "ref:arguments"],
    ),
    ("imports", &["declaration:imports"]),
    ("macros", &["declaration:macros", "expand"]),
    ("embedded", &["embed"]),
    ("recovery", &["recover", "missing"]),
];

#[test]
fn every_union_feature_has_a_native_representation_both_serializations_carry() {
    let fixture = fixture();
    let titles: Vec<String> = features(&fixture)
        .iter()
        .map(|feature| text(feature, "title").to_owned())
        .collect();
    assert_eq!(titles, vision_feature_titles());
    let ids: Vec<&str> = features(&fixture)
        .iter()
        .map(|feature| text(feature, "id"))
        .collect();
    let expected_ids: Vec<&str> = FEATURE_FORMS.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, expected_ids);
    for (feature, (id, required)) in features(&fixture).iter().zip(FEATURE_FORMS) {
        let listing = text(feature, "listing");
        let grammar = native(listing);
        let forms = forms_of(&grammar);
        for form in *required {
            assert!(forms.contains(*form), "{id} uses {form}: {forms:?}");
        }
        assert_eq!(
            render_native_grammar(&grammar),
            listing,
            "{id}: the listing is its own rendering"
        );
        let links = render_grammar_links(&grammar);
        let from_links = parse_grammar_links(&links).expect("the links parse");
        assert_eq!(
            from_links, grammar,
            "{id}: the links form carries the grammar"
        );
        assert_eq!(render_grammar_links(&from_links), links);
        assert_eq!(render_native_grammar(&from_links), listing);
        let serialized = serialize_grammar(&grammar);
        assert_eq!(
            deserialize_grammar(&serialized).expect("the JSON form reads"),
            grammar,
            "{id}: the JSON form carries the grammar"
        );
    }
    observe(
        &["everyUnionFeatureRepresented"],
        "every_union_feature_has_a_native_representation_both_serializations_carry",
    );
}

#[test]
fn every_union_feature_executes_its_positive_cases_and_mutation() {
    let fixture = fixture();
    let suite = Suite::new(&fixture);
    for feature in features(&fixture) {
        let id = text(feature, "id");
        let positive = feature["positive"].as_array().expect("positive cases");
        assert!(!positive.is_empty(), "{id} has positive cases");
        let grammar = native(text(feature, "listing"));
        let from_links =
            parse_grammar_links(&render_grammar_links(&grammar)).expect("the links parse");
        let parsers = [
            suite.parser(&grammar, feature),
            suite.parser(&from_links, feature),
        ];
        for item in positive {
            for parser in &parsers {
                let result = run(parser, item);
                assert!(
                    result.ok,
                    "{id} accepts {}: {:?}",
                    label(item),
                    result.rejection
                );
                assert_eq!(summary(&result), expected(item), "{id}: {}", label(item));
                let tree = parser
                    .parse(&input_of(item), &options_of(item.get("options")))
                    .expect("parse accepts");
                assert_eq!(tree.render(), text(item, "tree"));
                // The tree is lossless: it spans the input, byte for byte.
                assert_eq!((tree.start(), tree.end()), (0, input_of(item).len()));
            }
        }
        let more = feature.get("mutations").and_then(Value::as_array);
        for mutation in std::iter::once(&feature["mutation"]).chain(more.into_iter().flatten()) {
            let listing = text(feature, "listing");
            let (from, to) = (text(mutation, "from"), text(mutation, "to"));
            assert!(listing.contains(from), "{id}: the mutation applies");
            let mutated = listing.replacen(from, to, 1);
            assert_ne!(
                mutation["after"], mutation["before"],
                "{id}: the mutation changes the result"
            );
            assert_eq!(
                outcome(&parsers[0], mutation),
                mutation["before"],
                "{id}: before"
            );
            let after = if mutation["after"].get("loadError").is_some() {
                json!({ "loadError": suite.load_failure(&mutated, feature) })
            } else {
                outcome(&suite.parser(&native(&mutated), feature), mutation)
            };
            assert_eq!(after, mutation["after"], "{id}: the mutated grammar");
        }
    }
    observe(
        &["everyUnionFeatureExecuted"],
        "every_union_feature_executes_its_positive_cases_and_mutation",
    );
}

#[test]
fn every_union_feature_rejects_its_negative_cases() {
    let fixture = fixture();
    let suite = Suite::new(&fixture);
    for feature in features(&fixture) {
        let id = text(feature, "id");
        let negative = feature["negative"].as_array().expect("negative cases");
        assert!(!negative.is_empty(), "{id} has negative cases");
        let parser = suite.parser(&native(text(feature, "listing")), feature);
        for item in negative {
            if let Some(listing) = item["listing"].as_str() {
                assert_eq!(
                    suite.load_failure(listing, feature),
                    item["loadError"],
                    "{id}: {listing}"
                );
                continue;
            }
            let result = run(&parser, item);
            assert!(!result.ok, "{id} rejects {}", label(item));
            assert_eq!(summary(&result), expected(item), "{id}: {}", label(item));
            let error = parser
                .parse(&input_of(item), &options_of(item.get("options")))
                .expect_err("parse rejects");
            assert_eq!(error.rejection.reason, text(&item["rejection"], "reason"));
            assert_ne!(error.to_string(), "");
        }
    }
    // The step budget bounds a parse as the depth limit does.
    let recursion = features(&fixture)
        .iter()
        .find(|feature| feature["id"] == "recursion")
        .expect("recursion feature");
    let limited = suite.parser(
        &native(text(recursion, "listing")),
        &json!({ "options": { "stepLimit": 5 } }),
    );
    let result = run(&limited, &json!({ "input": "e:1-2-3" }));
    assert_eq!(
        summary(&result)["rejection"],
        json!({ "reason": "stepLimit", "limit": 5 })
    );
    observe(
        &["negativeCasesRejected"],
        "every_union_feature_rejects_its_negative_cases",
    );
}

#[test]
fn external_scanners_and_semantic_actions_are_executable_link_definitions() {
    let fixture = fixture();
    let suite = Suite::new(&fixture);
    let by_id = |wanted: &str| {
        features(&fixture)
            .iter()
            .find(|feature| feature["id"] == wanted)
            .expect("feature")
    };
    for id in ["layout", "modes", "predicates", "actions"] {
        let grammar = native(text(by_id(id), "listing"));
        let operations: Vec<&FeatureForm> = grammar
            .declarations()
            .scanners
            .iter()
            .flat_map(|scanner| scanner.operations.iter())
            .chain(
                grammar
                    .rules()
                    .iter()
                    .flat_map(|rule| rule.attributes.action.iter().flatten()),
            )
            .collect();
        assert!(
            !operations.is_empty(),
            "{id} has scanner or action operations"
        );
        for operation in operations {
            assert!(
                operation_form(&operation.head).is_some(),
                "{id}: {}",
                operation.head
            );
        }
    }

    // The layout scanner and the sum action are links of the links form, and
    // the grammar read back from those links runs them.
    let layout = by_id("layout");
    let layout_links = render_grammar_links(&native(text(layout, "listing")));
    let scanner_link = Regex::new(
        r"(?m)^\(scanner layout \(tokens newline indent dedent\) \(operations \(if \(valid newline\)",
    )
    .expect("pattern");
    assert!(scanner_link.is_match(&layout_links), "{layout_links}");
    let actions = by_id("actions");
    let action_links = render_grammar_links(&native(text(actions, "listing")));
    assert!(
        action_links.contains("(action (setAttribute value (sumOf terms value))"),
        "{action_links}"
    );
    let run_links = |links: &str, feature: &Value, input: &Value| {
        let grammar = parse_grammar_links(links).expect("the links parse");
        outcome(&suite.parser(&grammar, feature), &json!({ "input": input }))
    };
    let nested = &layout["positive"][0];
    let summed = &actions["positive"][0];
    assert_eq!(
        run_links(&layout_links, layout, &nested["input"]),
        expected(nested)
    );
    assert_eq!(
        run_links(&action_links, actions, &summed["input"]),
        expected(summed)
    );

    // Changing one operation link changes what the parser does: a scanner that
    // never emits indent cannot open a block, and a lower action limit fails
    // the sum.
    assert!(text(nested, "tree").contains("(indent \"\")"));
    let without_indent = layout_links.replacen(
        "(push indents (variable pending)) (emit indent)",
        "(push indents (variable pending)) fail",
        1,
    );
    assert_ne!(without_indent, layout_links);
    assert_eq!(
        run_links(&without_indent, layout, &nested["input"])["rejection"]["reason"],
        "syntax"
    );
    assert!(text(summed, "tree").starts_with("(sum {value=6}"));
    let lower_limit = action_links.replacen("(integer 100)", "(integer 2)", 1);
    assert_ne!(lower_limit, action_links);
    assert_eq!(
        run_links(&lower_limit, actions, &summed["input"])["rejection"]["reason"],
        "syntax"
    );

    // An operation outside its context, or one naming a token the scanner does
    // not declare, is rejected when the grammar loads.
    let out_of_context = "start s\nrule s = normal literal(\"a\") action(emit(s))\n";
    assert_eq!(
        suite.load_failure(out_of_context, &json!({})),
        json!({ "reason": "operation" })
    );
    let undeclared = text(layout, "listing").replacen("emit(dedent)", "emit(outdent)", 1);
    assert_eq!(
        suite.load_failure(&undeclared, layout),
        json!({ "reason": "operation" })
    );

    // The executor interprets those links; it runs no host code: no process,
    // no dynamic library and no unsafe code in its sources.
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/grammar/feature_runtime");
    let host_code = Regex::new(r"\bunsafe\b|process::Command|libloading|dlopen").expect("pattern");
    let mut files = 0;
    for entry in std::fs::read_dir(&runtime).expect("the runtime directory reads") {
        let path = entry.expect("entry").path();
        let source = std::fs::read_to_string(&path).expect("the source reads");
        assert!(!host_code.is_match(&source), "{}", path.display());
        files += 1;
    }
    assert!(files > 0);
    observe(
        &["scannersAndActionsExecutableAsLinks"],
        "external_scanners_and_semantic_actions_are_executable_link_definitions",
    );
}
