//! Prints every case of `cases.json` imported by the Rust Lark importer in the
//! runtime-neutral rendering shared with `render.mjs`, so the two outputs can
//! be diffed line for line.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use meta_language::{CharClassItem, GrammarExpr, GrammarImportError, RuleKind, import_lark};

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "cases.json".to_owned());
    let text = std::fs::read_to_string(&path).expect("cases.json is readable");
    let cases: Vec<(String, String)> = serde_json::from_str(&text).expect("cases.json is valid");
    let mut output = String::new();
    for (name, source) in cases {
        writeln!(output, "== {name}").unwrap();
        match import_lark(&source) {
            Ok(grammar) => {
                writeln!(
                    output,
                    "format {}",
                    grammar.source_format().map_or("none", |format| format.as_str())
                )
                .unwrap();
                writeln!(output, "start {}", grammar.start().unwrap_or("none")).unwrap();
                // The JavaScript grammar is keyed by rule name, so print each
                // distinct name once, resolved like `Grammar::rule`.
                let mut seen = BTreeSet::new();
                for name in grammar.rule_names() {
                    if !seen.insert(name) {
                        continue;
                    }
                    let rule = grammar.rule(name).expect("named rule exists");
                    writeln!(output, "rule {name} = {} {}", kind(rule.kind()), render(rule.expr()))
                        .unwrap();
                    if let Some(doc) = rule.doc() {
                        writeln!(output, "  doc {}", quote(doc)).unwrap();
                    }
                }
                writeln!(output, "undefined {:?}", grammar.undefined_nonterminals()).unwrap();
            }
            Err(error) => {
                let kind = match error {
                    GrammarImportError::Parse { .. } => "parse",
                    GrammarImportError::Unsupported { .. } => "unsupported",
                };
                writeln!(output, "error {kind}: {error}").unwrap();
            }
        }
    }
    print!("{output}");
}

const fn kind(kind: RuleKind) -> &'static str {
    match kind {
        RuleKind::Normal => "normal",
        RuleKind::Atomic => "atomic",
        RuleKind::Silent => "silent",
        RuleKind::Token => "token",
    }
}

// Same spelling as js/tests/support/render-grammar-expression.js and
// rust/tests/unit/grammar_render.rs.
fn render(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::Empty => "empty".to_string(),
        GrammarExpr::AnyChar => "any".to_string(),
        GrammarExpr::Terminal(value) => format!("literal({})", quote(value)),
        GrammarExpr::TerminalInsensitive(value) => format!("literalInsensitive({})", quote(value)),
        GrammarExpr::CharRange(start, end) => range(*start, *end),
        GrammarExpr::CharClass { negated, items } => {
            let items = items
                .iter()
                .map(|item| match item {
                    CharClassItem::Range(start, end) => range(*start, *end),
                    CharClassItem::Char(value) => format!("char({})", quote(&value.to_string())),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({items})", if *negated { "notClass" } else { "class" })
        }
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

fn range(start: char, end: char) -> String {
    format!(
        "range({}, {})",
        quote(&start.to_string()),
        quote(&end.to_string())
    )
}

fn list(items: &[GrammarExpr]) -> String {
    items.iter().map(render).collect::<Vec<_>>().join(", ")
}

// JSON string quoting, identical to `JSON.stringify` for strings.
fn quote(value: &str) -> String {
    serde_json::to_string(value).expect("strings serialize")
}
