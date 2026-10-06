//! The decorator set and its hook at every pipeline level, over the cases
//! the JavaScript runtime checks too (`parity/decorators`): the levels set
//! holds one decorator per level, the composition set decorators of one level
//! that compose by order and then by id.

use std::path::PathBuf;

use meta_language::{
    DECORATOR_LEVELS, Decorator, DecoratorAction, DecoratorLevel, DecoratorRecord, DecoratorSet,
    FeatureParseOptions, GrammarMergeOptions, GrammarMergeSource, LinkNetwork, LinkQuery, LinkType,
    ParseConfiguration, ProgramProjectContext, ProgramSourceMapping, ReplacementRule, SyntaxTree,
    TranslationRule, TranslationRuleSet, analyze_program_decorated, compile_feature_grammar,
    decorate_emitted, decorate_grammar, emit_gbnf, import_gbnf, merge_grammars,
    translate_native_construct_decorated,
};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/decorators/cases.json";

fn read(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/decorators")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn cases() -> Value {
    serde_json::from_str(&read("cases.json")).expect("the decorator cases parse")
}

fn set(name: &str) -> DecoratorSet {
    DecoratorSet::from_lino(&read(&format!("{name}.lino"))).expect("the decorator set reads")
}

fn observe(assertion: &str, test_name: &str) {
    record(&Observation {
        requirement_id: "I195-DECORATORS-EVERY-LEVEL",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-decorators-every-level",
        fixture_file: FIXTURE,
        assertions: &[assertion],
        test_name,
    });
}

fn text<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field]
        .as_str()
        .unwrap_or_else(|| panic!("{field} is text"))
}

fn level(name: &str) -> DecoratorLevel {
    DecoratorLevel::parse(name).expect("a known level")
}

fn to_record(value: &Value) -> DecoratorRecord {
    serde_json::from_value(value.clone()).expect("a record of text fields")
}

fn record_value(record: Option<DecoratorRecord>) -> Value {
    record.map_or(Value::Null, |record| json!(record))
}

// The node kinds of a syntax tree in preorder.
fn kinds(tree: &SyntaxTree) -> Vec<String> {
    match tree {
        SyntaxTree::Node { kind, children, .. } => std::iter::once(kind.clone())
            .chain(children.iter().flat_map(kinds))
            .collect(),
        _ => Vec::new(),
    }
}

fn missing(tree: &SyntaxTree) -> Vec<String> {
    match tree {
        SyntaxTree::Missing { kind, .. } => vec![kind.clone().unwrap_or_default()],
        SyntaxTree::Node { children, .. } => children.iter().flat_map(missing).collect(),
        _ => Vec::new(),
    }
}

fn parse_options(decorators: &DecoratorSet) -> FeatureParseOptions {
    FeatureParseOptions {
        decorators: decorators.clone(),
        ..FeatureParseOptions::default()
    }
}

// Runs every hook with `decorators` and returns the runtime-neutral view the
// cases file records.
#[allow(clippy::too_many_lines)]
fn run_hooks(decorators: &DecoratorSet) -> Value {
    let cases = cases();
    let hooks = &cases["hooks"];
    let grammar_text = text(hooks, "grammar");
    let plain_grammar = import_gbnf(grammar_text).expect("the hook grammar imports");
    let imported = decorate_grammar(&plain_grammar, decorators, DecoratorLevel::Importer)
        .expect("the importer decorators apply");
    let options = parse_options(decorators);
    let parser = compile_feature_grammar(&imported, None, options.clone()).expect("compiles");
    let plain = compile_feature_grammar(&plain_grammar, None, options.clone()).expect("compiles");

    let source = |id: &str| GrammarMergeSource {
        id: id.to_owned(),
        language: "demo".to_owned(),
        edition: String::new(),
        precedence: 0,
        grammar: plain_grammar.clone(),
    };
    let merged = merge_grammars(
        &[source("one"), source("two")],
        &GrammarMergeOptions {
            decorators: decorators.clone(),
            ..GrammarMergeOptions::default()
        },
    )
    .expect("the hook grammars merge");

    let mapping_case = &hooks["conceptMapping"];
    let (from, rule, to) = (
        text(mapping_case, "from"),
        text(mapping_case, "rule"),
        text(mapping_case, "to"),
    );
    let mapping = translate_native_construct_decorated(from, rule, to, decorators)
        .expect("the concept-mapping decorators apply");

    let recovering = compile_feature_grammar(
        &imported,
        None,
        FeatureParseOptions {
            error_recovery: Some(true),
            accept_recovery: Some(true),
            ..options.clone()
        },
    )
    .expect("compiles");
    let recovery_source = text(&hooks["recovery"], "source");
    let recovered = recovering
        .parse_tree(recovery_source.as_bytes(), &options)
        .expect("recovers")
        .tree
        .expect("a recovered tree");

    let cst = &hooks["cstToAst"];
    let program = analyze_program_decorated(
        text(cst, "source"),
        text(cst, "language"),
        ProgramProjectContext::default(),
        decorators,
    )
    .expect("the program analyzes");
    let mut terms: Vec<&str> = program
        .source_mappings()
        .iter()
        .map(ProgramSourceMapping::term)
        .collect();
    terms.sort_unstable();

    let transformation = &hooks["transformation"];
    let mut network = LinkNetwork::parse(
        text(transformation, "source"),
        text(transformation, "language"),
        ParseConfiguration::default(),
    );
    let query = LinkQuery::from_sexpression(text(transformation, "query")).expect("query parses");
    let matches = network.find(&query);
    network.replace_decorated(
        &matches,
        &ReplacementRule::captured_text(
            text(transformation, "capture"),
            text(transformation, "replacement"),
        ),
        decorators,
    );

    let (emitted, _) = emit_gbnf(&imported).expect("the grammar emits");

    let translation = &hooks["translationRule"];
    let mut shell = LinkNetwork::new();
    let token = shell.insert_source_token("Shell", text(translation, "token"));
    let command = shell.insert_syntax_node("Shell", "command", [token]);
    let rules = TranslationRuleSet::new("shell-to-js").with_rule(
        TranslationRule::new(
            "command",
            LinkQuery::by_type(LinkType::Syntax)
                .with_language("Shell")
                .with_term("command"),
        )
        .with_reference_capture("body", 0)
        .with_template("JavaScript", text(translation, "template")),
    );

    let grammar_rule_source = text(&hooks["grammarRule"], "source");
    let executor_source = text(&hooks["executor"], "source");
    let mut cst_view = cst.clone();
    cst_view["terms"] = json!(terms);
    let mut transformation_view = transformation.clone();
    transformation_view["text"] = json!(network.reconstruct_text());
    let mut translation_view = translation.clone();
    translation_view["text"] = json!(rules.decorated(decorators).render_link(
        &shell,
        command,
        "JavaScript",
        ParseConfiguration::default()
    ));
    json!({
        "importer": { "ruleNames": imported.rule_names() },
        "grammarRule": {
            "source": grammar_rule_source,
            "kinds": kinds(&plain.parse(grammar_rule_source.as_bytes(), &options).expect("parses")),
        },
        "mergeDecision": {
            "bases": merged.groups[0].decisions.iter().map(|decision| decision.basis.as_str()).collect::<Vec<_>>(),
        },
        "conceptMapping": {
            "from": from, "rule": rule, "to": to,
            "relation": mapping.relation.as_str(), "rules": mapping.rules,
        },
        "executor": {
            "source": executor_source,
            "kinds": kinds(&parser.parse(executor_source.as_bytes(), &options).expect("parses")),
        },
        "recovery": { "source": recovery_source, "missing": missing(&recovered) },
        "cstToAst": cst_view,
        "transformation": transformation_view,
        "emitter": { "format": "gbnf", "source": decorate_emitted("gbnf", &emitted, decorators) },
        "translationRule": translation_view,
    })
}

#[test]
fn one_decorator_api_extends_every_pipeline_level() {
    let names: Vec<&str> = DECORATOR_LEVELS
        .iter()
        .map(|level| level.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "importer",
            "grammar-rule",
            "merge-decision",
            "concept-mapping",
            "executor",
            "recovery",
            "cst-to-ast",
            "transformation",
            "emitter",
            "translation-rule",
        ]
    );
    let levels = set("levels");
    for level in DECORATOR_LEVELS {
        assert!(levels.has(level), "{level}");
    }
    let mut expected = cases()["hooks"].clone();
    expected.as_object_mut().expect("hooks").remove("grammar");
    assert_eq!(run_hooks(&levels), expected);
    observe(
        "decoratorAtEveryLevel",
        "one decorator API extends every pipeline level",
    );
}

#[test]
fn decorators_compose_by_order_and_then_by_id() {
    for case in cases()["records"].as_array().expect("records") {
        let decorated = set(text(case, "set"))
            .decorate(level(text(case, "level")), &to_record(&case["record"]));
        assert_eq!(record_value(decorated), case["expected"], "{case}");
    }
    // The file lists the composition decorators out of order; reading sorts them.
    let composition = set("composition");
    assert_eq!(
        composition.ids(),
        ["first", "second", "tie-a", "tie-b", "drop-comment"]
    );
    let reversed = DecoratorSet::new(composition.decorators().iter().rev().cloned().collect())
        .expect("the reversed set");
    assert_eq!(reversed.ids(), composition.ids());
    let duplicate = Decorator::new(
        "first",
        DecoratorLevel::Emitter,
        0,
        Vec::new(),
        vec![DecoratorAction::Drop],
    )
    .expect("a decorator");
    assert!(composition.add([duplicate]).is_err());
    observe(
        "compositionOrderDeterministic",
        "decorators compose in a defined order",
    );
}

#[test]
fn decorators_are_links_data() {
    let levels = set("levels");
    assert_eq!(levels.to_lino(), read("levels.lino"));
    let composition = set("composition");
    assert_eq!(
        DecoratorSet::from_lino(&composition.to_lino())
            .expect("reads")
            .to_lino(),
        composition.to_lino()
    );
    let odd = DecoratorSet::new(vec![
        Decorator::new(
            "odd-text",
            DecoratorLevel::Emitter,
            0,
            vec![("line".to_owned(), "a (b) \"c\" 'd'".to_owned())],
            vec![
                DecoratorAction::Replace {
                    field: "line".to_owned(),
                    from: "%".to_owned(),
                    to: " ( ) \n".to_owned(),
                },
                DecoratorAction::Set {
                    field: "note".to_owned(),
                    value: String::new(),
                },
            ],
        )
        .expect("a decorator"),
    ])
    .expect("a set");
    assert_eq!(DecoratorSet::from_lino(&odd.to_lino()).expect("reads"), odd);
    assert!(DecoratorSet::from_lino("(decorator x (level nowhere) (drop))").is_err());
    assert!(DecoratorSet::from_lino("(decorator x (level emitter))").is_err());
    assert!(DecoratorSet::from_lino("(decorator x (level emitter) (set line %zz))").is_err());
    observe("storedAsLinksData", "decorators are stored as links data");
}

#[test]
fn removing_a_decorator_gives_the_output_of_the_set_without_it() {
    let cases = cases();
    let removal = &cases["removal"];
    let removed = set(text(removal, "set"))
        .remove(text(removal, "remove"))
        .expect("removes");
    assert_eq!(
        record_value(removed.decorate(
            level(text(removal, "level")),
            &to_record(&removal["record"])
        )),
        removal["expected"]
    );
    assert!(set("composition").remove("absent").is_err());
    // Removing every decorator restores the undecorated pipeline.
    let levels = set("levels");
    let mut empty = levels.clone();
    for id in levels.ids() {
        empty = empty.remove(id).expect("removes");
    }
    assert_eq!(run_hooks(&empty), run_hooks(&DecoratorSet::empty()));
    // Removing one level's decorator changes that hook alone.
    let without_emitter = run_hooks(&levels.remove("spaced-definition").expect("removes"));
    let all = run_hooks(&levels);
    for (key, value) in all.as_object().expect("hooks") {
        if key == "emitter" {
            assert_ne!(&without_emitter[key], value);
        } else {
            assert_eq!(&without_emitter[key], value, "{key}");
        }
    }
    observe("removable", "decorators can be removed");
}

#[test]
fn a_hook_rejects_a_decoration_that_would_lose_input() {
    let grammar_text = text(&cases()["hooks"], "grammar").to_owned();
    let grammar = import_gbnf(&grammar_text).expect("imports");
    let single = |lino: &str| DecoratorSet::from_lino(lino).expect("reads");
    let drop_token = parse_options(&single(
        "(decorator drop-token (level executor) (when (type token)) (drop))",
    ));
    let parser = compile_feature_grammar(&grammar, None, drop_token.clone()).expect("compiles");
    assert!(parser.parse_tree(b"ab", &drop_token).is_err());
    let unwrap = parse_options(&single(
        "(decorator unwrap (level executor) (when (kind item)) (drop))",
    ));
    let parser = compile_feature_grammar(&grammar, None, unwrap.clone()).expect("compiles");
    assert_eq!(
        kinds(&parser.parse(b"ab", &unwrap).expect("parses")),
        ["root"]
    );
    let drop_rule = single("(decorator drop-rule (level grammar-rule) (when (name root)) (drop))");
    assert_eq!(
        decorate_grammar(&grammar, &drop_rule, DecoratorLevel::GrammarRule)
            .expect("decorates")
            .rule_names(),
        ["item"]
    );
    let concept = single(
        "(decorator concept (level grammar-rule) (when (name item)) (set concept grammar.letter))",
    );
    let decorated =
        decorate_grammar(&grammar, &concept, DecoratorLevel::GrammarRule).expect("decorates");
    assert_eq!(
        decorated
            .rule("item")
            .and_then(|rule| rule.concept.as_deref()),
        Some("grammar.letter")
    );
    assert!(
        Decorator::new(
            "bad id",
            DecoratorLevel::Emitter,
            0,
            Vec::new(),
            vec![DecoratorAction::Drop]
        )
        .is_err()
    );
    let empty_replace = DecoratorAction::Replace {
        field: "line".to_owned(),
        from: String::new(),
        to: "x".to_owned(),
    };
    assert!(
        Decorator::new(
            "empty",
            DecoratorLevel::Emitter,
            0,
            Vec::new(),
            vec![empty_replace]
        )
        .is_err()
    );
}
