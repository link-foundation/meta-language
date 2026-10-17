//! Binding-aware grammar renaming, including recursion and source aliases.

use meta_language::{
    Grammar, GrammarExpr, GrammarFormat, GrammarRenameErrorKind, GrammarRule, RenamedGrammar,
    RuleAlias, RuleKind, grammar_from_lino, grammar_to_lino, rename_grammar_rule,
    restore_source_names,
};
use serde_json::Value;

use super::{fixture, observe, rule, text, to_json};

// Builds a grammar from the normalized JSON grammar shape used by the
// JavaScript runtime (`serializeGrammar`).
fn grammar_from_json(value: &Value) -> Grammar {
    let mut grammar = Grammar::new();
    for entry in value["rules"].as_array().expect("rules") {
        let kind = match text(&entry["kind"]) {
            "normal" => RuleKind::Normal,
            "atomic" => RuleKind::Atomic,
            "silent" => RuleKind::Silent,
            "token" => RuleKind::Token,
            other => panic!("unknown rule kind {other}"),
        };
        grammar.add_rule(
            GrammarRule::new(text(&entry["name"]), expr_from_json(&entry["expression"]))
                .with_kind(kind),
        );
    }
    if let Some(start) = value["start"].as_str() {
        grammar.set_start(start);
    }
    assert_eq!(value["sourceFormat"], "peg");
    grammar.with_source_format(GrammarFormat::Peg)
}

fn expr_from_json(value: &Value) -> GrammarExpr {
    let items = || {
        value["items"]
            .as_array()
            .expect("items")
            .iter()
            .map(expr_from_json)
            .collect::<Vec<_>>()
    };
    let item = || expr_from_json(&value["item"]);
    let character = |key: &str| text(&value[key]).chars().next().expect("character");
    match text(&value["kind"]) {
        "empty" => GrammarExpr::Empty,
        "literal" => GrammarExpr::terminal(text(&value["value"])),
        "any" => GrammarExpr::AnyChar,
        "ref" => GrammarExpr::non_terminal(text(&value["name"])),
        "charRange" => GrammarExpr::CharRange(character("start"), character("end")),
        "seq" => GrammarExpr::Sequence(items()),
        "choice" => GrammarExpr::choice(value["ordered"] == true, items()),
        "optional" => GrammarExpr::optional(item()),
        "repeat0" => GrammarExpr::zero_or_more(item()),
        "repeat1" => GrammarExpr::one_or_more(item()),
        "and" => GrammarExpr::and(item()),
        "not" => GrammarExpr::not(item()),
        "capture" => value["label"].as_str().map_or_else(
            || GrammarExpr::capture_unlabeled(item()),
            |label| GrammarExpr::capture(label, item()),
        ),
        other => panic!("unsupported expression kind {other}"),
    }
}

fn rename_steps(
    fixture: &Value,
    grammar: Grammar,
    steps: &[Value],
    aliases: Vec<RuleAlias>,
) -> RenamedGrammar {
    let namespace = fixture["rename"]["namespace"].as_str();
    steps
        .iter()
        .fold(RenamedGrammar { grammar, aliases }, |state, step| {
            rename_grammar_rule(
                &state.grammar,
                text(&step["from"]),
                text(&step["to"]),
                namespace,
                &state.aliases,
            )
            .expect("fixture rename succeeds")
        })
}

fn steps(fixture: &Value) -> Vec<Value> {
    fixture["rename"]["steps"]
        .as_array()
        .expect("steps")
        .clone()
}

fn sequence(expr: &GrammarExpr) -> &[GrammarExpr] {
    match expr {
        GrammarExpr::Sequence(items) => items,
        other => panic!("expected a sequence, got {other:?}"),
    }
}

fn alternatives(expr: &GrammarExpr) -> &[GrammarExpr] {
    match expr {
        GrammarExpr::Choice { alternatives, .. } => alternatives,
        other => panic!("expected a choice, got {other:?}"),
    }
}

#[test]
fn renaming_follows_recursive_references_qualified_names_and_captures() {
    let fixture = fixture();
    let original = grammar_from_json(&fixture["rename"]["grammar"]);
    let renamed = rename_steps(&fixture, original, &steps(&fixture), Vec::new());
    let grammar = &renamed.grammar;
    assert_eq!(grammar.start(), Some("sum"));
    let sum = sequence(&rule(grammar, "sum").expr);
    let GrammarExpr::Optional(tail) = &sum[1] else {
        panic!("sum tail is optional");
    };
    assert_eq!(sequence(tail)[1], GrammarExpr::non_terminal("sum"));
    let operand = alternatives(&rule(grammar, "operand").expr);
    assert_eq!(sequence(&operand[1])[1], GrammarExpr::non_terminal("sum"));
    assert_eq!(
        sequence(&rule(grammar, "spaced").expr)[1],
        GrammarExpr::non_terminal("operand")
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["recursiveRulesRenamed"],
        "renaming follows recursive references",
    );

    assert_eq!(
        alternatives(&rule(grammar, "qualified").expr),
        [
            GrammarExpr::non_terminal("arithmetic.operand"),
            GrammarExpr::non_terminal("arithmetic::operand"),
            GrammarExpr::non_terminal("library.value"),
        ]
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["qualifiedReferencesRenamed"],
        "renaming follows qualified references",
    );

    // The capture label `value` names a local binding, not the rule: it stays,
    // while the reference it wraps follows the rule.
    assert_eq!(
        sum[0],
        GrammarExpr::capture("value", GrammarExpr::non_terminal("operand"))
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["shadowingHandled"],
        "capture labels shadowing a rule are not renamed",
    );
}

#[test]
fn renaming_refuses_collisions_and_unknown_rules() {
    let fixture = fixture();
    let case = &fixture["rename"];
    let original = grammar_from_json(&case["grammar"]);
    let kinds: Vec<&str> = case["collisions"]
        .as_array()
        .expect("collisions")
        .iter()
        .map(|collision| {
            rename_grammar_rule(
                &original,
                text(&collision["from"]),
                text(&collision["to"]),
                case["namespace"].as_str(),
                &[],
            )
            .map_or_else(|error| error.kind().as_str(), |_| "accepted")
        })
        .collect();
    assert_eq!(to_json(&kinds), case["expected"]["collisionKinds"]);
    assert_eq!(
        original,
        grammar_from_json(&case["grammar"]),
        "a refused rename leaves the grammar unchanged"
    );
    for invalid in ["", "two words"] {
        assert_eq!(
            rename_grammar_rule(&original, "value", invalid, None, &[])
                .expect_err("invalid name")
                .kind(),
            GrammarRenameErrorKind::InvalidName
        );
    }
    let restore = restore_source_names(
        &original,
        &[RuleAlias {
            canonical: "value".to_owned(),
            original: "number".to_owned(),
        }],
        None,
    );
    assert_eq!(
        restore.expect_err("restoring onto an existing rule").kind(),
        GrammarRenameErrorKind::Collision
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["collisionsHandled"],
        "renaming refuses collisions",
    );
}

#[test]
fn renaming_works_on_a_reloaded_grammar_and_keeps_source_aliases_for_export() {
    let fixture = fixture();
    let case = &fixture["rename"];
    let original = grammar_from_json(&case["grammar"]);
    let renamed = rename_steps(&fixture, original.clone(), &steps(&fixture), Vec::new());
    let reloaded =
        grammar_from_lino(&grammar_to_lino(&renamed.grammar)).expect("renamed grammar reloads");
    assert_eq!(reloaded, renamed.grammar);
    let after_reload = [case["afterReload"].clone()];
    let last = rename_steps(&fixture, reloaded, &after_reload, renamed.aliases);
    assert_eq!(
        last.grammar,
        grammar_from_json(&case["expected"]["grammar"])
    );
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["referencesAfterReloadRenamed"],
        "renaming works on a reloaded grammar",
    );

    assert_eq!(to_json(&last.aliases), case["expected"]["aliases"]);
    let exported = restore_source_names(&last.grammar, &last.aliases, case["namespace"].as_str())
        .expect("source names restore");
    assert_eq!(exported, original);
    observe(
        "I195-MERGE-BINDING-AWARE-RENAME",
        &["sourceAliasesKept"],
        "source aliases restore the original names",
    );
}
