//! Prints the Rust GBNF importer and emitter results for every case in
//! `cases.json` as JSON, in the shape `compare.mjs` expects from the
//! JavaScript port.

use meta_language::{CharClassItem, Grammar, GrammarExpr, emit_gbnf, import_gbnf};
use serde_json::{Value, json};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "cases.json".to_string());
    let cases: Value =
        serde_json::from_str(&std::fs::read_to_string(path).expect("cases file")).expect("JSON");

    let imports = cases["import"]
        .as_array()
        .expect("import cases")
        .iter()
        .map(|source| import_case(source.as_str().expect("source")))
        .collect::<Vec<_>>();
    let emits = cases["emit"]
        .as_array()
        .expect("emit cases")
        .iter()
        .map(emit_case)
        .collect::<Vec<_>>();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "import": imports, "emit": emits })).expect("JSON")
    );
}

fn import_case(source: &str) -> Value {
    match import_gbnf(source) {
        Ok(grammar) => json!({
            "start": grammar.start(),
            "sourceFormat": grammar.source_format().map(|format| format.as_str()),
            "undefined": grammar.undefined_nonterminals().into_iter().collect::<Vec<_>>(),
            "rules": grammar.rules().iter().map(|rule| json!({
                "name": rule.name(),
                "doc": rule.doc(),
                "rendered": format!("{} {}", rule.kind().as_str(), render(rule.expr())),
            })).collect::<Vec<_>>(),
        }),
        Err(error) => json!({ "error": error.to_string() }),
    }
}

fn emit_case(case: &Value) -> Value {
    let mut builder = Grammar::builder();
    if let Some(start) = case["start"].as_str() {
        builder = builder.start(start);
    }
    for rule in case["rules"].as_array().expect("rules") {
        builder = builder.rule(
            rule["name"].as_str().expect("rule name"),
            expr(&rule["expression"]),
        );
    }
    match emit_gbnf(&builder.build()) {
        Ok((source, report)) => json!({ "source": source, "lossy": report.lossy }),
        Err(error) => json!({ "error": error.to_string() }),
    }
}

fn character(value: &Value) -> char {
    let text = value.as_str().expect("character");
    let mut chars = text.chars();
    let character = chars.next().expect("one character");
    assert!(chars.next().is_none(), "exactly one character");
    character
}

fn text(value: &Value) -> String {
    value.as_str().expect("string").to_string()
}

fn boxed(value: &Value) -> Box<GrammarExpr> {
    Box::new(expr(&value["item"]))
}

fn count(value: &Value) -> usize {
    usize::try_from(value.as_u64().expect("count")).expect("usize")
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
    match value["kind"].as_str().expect("kind") {
        "empty" => GrammarExpr::Empty,
        "literal" => GrammarExpr::Terminal(text(&value["value"])),
        "literalInsensitive" => GrammarExpr::TerminalInsensitive(text(&value["value"])),
        "ref" => GrammarExpr::NonTerminal(text(&value["name"])),
        "seq" => GrammarExpr::Sequence(items()),
        "choice" => GrammarExpr::Choice {
            ordered: value["ordered"].as_bool().unwrap_or(false),
            alternatives: items(),
        },
        "optional" => GrammarExpr::Optional(boxed(value)),
        "repeat0" => GrammarExpr::ZeroOrMore(boxed(value)),
        "repeat1" => GrammarExpr::OneOrMore(boxed(value)),
        "repeat" => GrammarExpr::Repeat {
            expr: boxed(value),
            min: count(&value["min"]),
            max: value["max"].as_u64().map(|_| count(&value["max"])),
        },
        "and" => GrammarExpr::And(boxed(value)),
        "not" => GrammarExpr::Not(boxed(value)),
        "capture" => GrammarExpr::Capture {
            label: value["label"].as_str().map(str::to_string),
            expr: boxed(value),
        },
        "charRange" => GrammarExpr::CharRange(character(&value["start"]), character(&value["end"])),
        "charClass" => GrammarExpr::CharClass {
            negated: value["negated"].as_bool().unwrap_or(false),
            items: value["items"]
                .as_array()
                .expect("class items")
                .iter()
                .map(|item| match item["kind"].as_str().expect("item kind") {
                    "range" => CharClassItem::Range(character(&item["start"]), character(&item["end"])),
                    _ => CharClassItem::Char(character(&item["value"])),
                })
                .collect(),
        },
        "any" => GrammarExpr::AnyChar,
        other => panic!("unsupported expression kind {other}"),
    }
}

// Mirrors js/tests/support/render-grammar-expression.js.
fn render(expr: &GrammarExpr) -> String {
    let quote = |value: &str| serde_json::to_string(value).expect("JSON string");
    let quote_char = |value: char| quote(&value.to_string());
    let list = |items: &[GrammarExpr]| items.iter().map(render).collect::<Vec<_>>().join(", ");
    match expr {
        GrammarExpr::Empty => "empty".to_string(),
        GrammarExpr::AnyChar => "any".to_string(),
        GrammarExpr::Terminal(value) => format!("literal({})", quote(value)),
        GrammarExpr::TerminalInsensitive(value) => format!("literalInsensitive({})", quote(value)),
        GrammarExpr::CharRange(start, end) => {
            format!("range({}, {})", quote_char(*start), quote_char(*end))
        }
        GrammarExpr::CharClass { negated, items } => format!(
            "{}({})",
            if *negated { "notClass" } else { "class" },
            items
                .iter()
                .map(|item| match item {
                    CharClassItem::Range(start, end) => {
                        format!("range({}, {})", quote_char(*start), quote_char(*end))
                    }
                    CharClassItem::Char(value) => format!("char({})", quote_char(*value)),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        GrammarExpr::NonTerminal(name) => format!("ref({name})"),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => format!(
            "{}({})",
            if *ordered { "orderedChoice" } else { "choice" },
            list(alternatives)
        ),
        GrammarExpr::Sequence(items) => format!("seq({})", list(items)),
        GrammarExpr::Optional(inner) => format!("optional({})", render(inner)),
        GrammarExpr::ZeroOrMore(inner) => format!("repeat0({})", render(inner)),
        GrammarExpr::OneOrMore(inner) => format!("repeat1({})", render(inner)),
        GrammarExpr::And(inner) => format!("and({})", render(inner)),
        GrammarExpr::Not(inner) => format!("not({})", render(inner)),
        GrammarExpr::Repeat { expr, min, max } => format!(
            "repeat({}, {min}, {})",
            render(expr),
            max.map_or_else(|| "unbounded".to_string(), |max| max.to_string())
        ),
        GrammarExpr::Capture { label, expr } => format!(
            "capture({}, {})",
            label.as_deref().map_or_else(|| "null".to_string(), quote),
            render(expr)
        ),
    }
}
