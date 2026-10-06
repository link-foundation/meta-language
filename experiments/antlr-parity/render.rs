//! Prints every case of `cases.txt` and the Rust ANTLR fixtures imported by
//! `meta_language::import_antlr`, in the same text as `render.mjs`.

use std::fs;
use std::path::Path;

use meta_language::{CharClassItem, GrammarExpr, GrammarRule, RuleKind, import_antlr};

fn main() {
    let directory = std::env::args().nth(1).unwrap_or_else(|| ".".to_owned());
    let root = Path::new(&directory);
    for (name, source) in cases(root) {
        println!("### {name}");
        match import_antlr(&source) {
            Ok(grammar) => {
                let start = grammar.start_rule().map_or("-", |rule| rule.name.as_str());
                println!("start: {start}");
                for rule in grammar.rules() {
                    let doc = rule.doc().map_or_else(|| "-".to_string(), quote);
                    println!("rule {}: {} | doc: {doc}", rule.name, render_rule(rule));
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn cases(root: &Path) -> Vec<(String, String)> {
    let fixtures = root.join("../../rust/tests/fixtures/grammar/antlr");
    let mut result = Vec::new();
    for file in ["arithmetic.g4", "covering.g4", "lexer-mode.g4", "case-insensitive.g4"] {
        let source = fs::read_to_string(fixtures.join(file)).expect("fixture reads");
        result.push((format!("fixture {file}"), source));
    }
    let text = fs::read_to_string(root.join("cases.txt")).expect("cases.txt reads");
    let mut current: Option<(String, Vec<&str>)> = None;
    for line in text.split('\n') {
        if let Some(name) = line.strip_prefix("### ") {
            if let Some((name, lines)) = current.take() {
                result.push((name, lines.join("\n")));
            }
            current = Some((name.to_string(), Vec::new()));
        } else if let Some((_, lines)) = current.as_mut() {
            lines.push(line);
        }
    }
    if let Some((name, lines)) = current {
        result.push((name, lines.join("\n")));
    }
    result
}

// Same spelling as rust/tests/unit/grammar_render.rs and
// js/tests/support/render-grammar-expression.js.
fn render_rule(rule: &GrammarRule) -> String {
    let kind = match rule.kind() {
        RuleKind::Normal => "normal",
        RuleKind::Atomic => "atomic",
        RuleKind::Silent => "silent",
        RuleKind::Token => "token",
    };
    format!("{kind} {}", render_expr(rule.expr()))
}

fn render_expr(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::Feature(_) => meta_language::render_native_expression(expr),
        GrammarExpr::Empty => "empty".to_string(),
        GrammarExpr::AnyChar => "any".to_string(),
        GrammarExpr::Terminal(value) => format!("literal({})", quote(value)),
        GrammarExpr::TerminalInsensitive(value) => format!("literalInsensitive({})", quote(value)),
        GrammarExpr::CharRange(start, end) => render_range(*start, *end),
        GrammarExpr::CharClass { negated, items } => {
            let items = items
                .iter()
                .map(|item| match item {
                    CharClassItem::Range(start, end) => render_range(*start, *end),
                    CharClassItem::Char(value) => format!("char({})", quote(&value.to_string())),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({items})", if *negated { "notClass" } else { "class" })
        }
        GrammarExpr::NonTerminal(name) => format!("ref({name})"),
        GrammarExpr::Choice { ordered, alternatives } => format!(
            "{}({})",
            if *ordered { "orderedChoice" } else { "choice" },
            render_list(alternatives)
        ),
        GrammarExpr::Sequence(items) => format!("seq({})", render_list(items)),
        GrammarExpr::Optional(inner) => format!("optional({})", render_expr(inner)),
        GrammarExpr::ZeroOrMore(inner) => format!("repeat0({})", render_expr(inner)),
        GrammarExpr::OneOrMore(inner) => format!("repeat1({})", render_expr(inner)),
        GrammarExpr::And(inner) => format!("and({})", render_expr(inner)),
        GrammarExpr::Not(inner) => format!("not({})", render_expr(inner)),
        GrammarExpr::Repeat { expr, min, max } => format!(
            "repeat({}, {min}, {})",
            render_expr(expr),
            max.map_or_else(|| "unbounded".to_string(), |max| max.to_string())
        ),
        GrammarExpr::Capture { label, expr } => format!(
            "capture({}, {})",
            label.as_deref().map_or_else(|| "null".to_string(), quote),
            render_expr(expr)
        ),
    }
}

fn render_range(start: char, end: char) -> String {
    format!("range({}, {})", quote(&start.to_string()), quote(&end.to_string()))
}

fn render_list(items: &[GrammarExpr]) -> String {
    items.iter().map(render_expr).collect::<Vec<_>>().join(", ")
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).expect("strings serialize to JSON")
}
