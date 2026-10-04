//! Stand-ins for the meta-language functions the merge test uses beyond the
//! grammar IR: `import_pest` returns the grammar the JavaScript importer made
//! from the same fixture text (exported by export-js-grammars.mjs), and the
//! LiNo round trip keeps grammars in memory.

use std::cell::RefCell;

use serde_json::Value;

use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

thread_local! {
    static STORED: RefCell<Vec<Grammar>> = const { RefCell::new(Vec::new()) };
}

pub fn import_pest(text: &str) -> Result<Grammar, String> {
    let path = std::env::var("GRAMMAR_MERGE_JS_GRAMMARS")
        .unwrap_or_else(|_| "/tmp/grammar-merge-js-grammars.json".to_owned());
    let exported: Value =
        serde_json::from_str(&std::fs::read_to_string(path).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?;
    let serialized = exported["sources"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|source| source["text"] == text)
        .map(|source| source["grammar"].clone())
        .or_else(|| {
            (exported["upstreamChangeText"] == text).then(|| exported["upstreamChange"].clone())
        })
        .ok_or_else(|| format!("no exported grammar for {text:?}"))?;
    let value: Value = serde_json::from_str(serialized.as_str().ok_or("serialized grammar")?)
        .map_err(|error| error.to_string())?;
    Ok(grammar(&value))
}

pub fn grammar_to_lino(grammar: &Grammar) -> String {
    STORED.with(|stored| {
        stored.borrow_mut().push(grammar.clone());
        (stored.borrow().len() - 1).to_string()
    })
}

pub fn grammar_from_lino(text: &str) -> Result<Grammar, String> {
    let index: usize = text.parse().map_err(|_| "unknown lino".to_owned())?;
    STORED.with(|stored| {
        stored
            .borrow()
            .get(index)
            .cloned()
            .ok_or_else(|| "unknown lino".to_owned())
    })
}

fn grammar(value: &Value) -> Grammar {
    let mut grammar = Grammar::new();
    for entry in value["rules"].as_array().expect("rules") {
        let kind = match entry["kind"].as_str() {
            Some("normal") => RuleKind::Normal,
            Some("atomic") => RuleKind::Atomic,
            Some("silent") => RuleKind::Silent,
            Some("token") => RuleKind::Token,
            other => panic!("rule kind {other:?}"),
        };
        grammar.add_rule(
            GrammarRule::new(
                entry["name"].as_str().expect("name"),
                expr(&entry["expression"]),
            )
            .with_kind(kind),
        );
    }
    if let Some(start) = value["start"].as_str() {
        grammar.set_start(start);
    }
    assert_eq!(value["sourceFormat"], "peg");
    grammar.with_source_format(GrammarFormat::Peg)
}

fn expr(value: &Value) -> GrammarExpr {
    let items = || {
        value["items"]
            .as_array()
            .expect("items")
            .iter()
            .map(expr)
            .collect::<Vec<_>>()
    };
    let item = || Box::new(expr(&value["item"]));
    let text = |key: &str| value[key].as_str().expect(key).to_owned();
    match value["kind"].as_str().expect("kind") {
        "empty" => GrammarExpr::Empty,
        "literal" => GrammarExpr::Terminal(text("value")),
        "literalInsensitive" => GrammarExpr::TerminalInsensitive(text("value")),
        "any" => GrammarExpr::AnyChar,
        "ref" => GrammarExpr::NonTerminal(text("name")),
        "charRange" => GrammarExpr::CharRange(
            text("start").chars().next().expect("start"),
            text("end").chars().next().expect("end"),
        ),
        "seq" => GrammarExpr::Sequence(items()),
        "choice" => GrammarExpr::Choice {
            ordered: value["ordered"] == true,
            alternatives: items(),
        },
        "optional" => GrammarExpr::Optional(item()),
        "repeat0" => GrammarExpr::ZeroOrMore(item()),
        "repeat1" => GrammarExpr::OneOrMore(item()),
        "and" => GrammarExpr::And(item()),
        "not" => GrammarExpr::Not(item()),
        "capture" => GrammarExpr::Capture {
            label: value["label"].as_str().map(str::to_owned),
            expr: item(),
        },
        other => panic!("unsupported expression kind {other}"),
    }
}
