//! Runtime-neutral rendering of grammar IR rules, shared by the unit tests that
//! compare imported grammars against recorded parity fixtures. The JavaScript
//! twin is `js/tests/support/render-grammar-expression.js`; both must render
//! the same text for the same grammar.

use meta_language::{CharClassItem, GrammarExpr, GrammarRule, RuleKind};

/// Renders a rule as `<kind> <expression>`, for example `normal seq(ref(a))`.
pub fn render_rule(rule: &GrammarRule) -> String {
    let kind = match rule.kind() {
        RuleKind::Normal => "normal",
        RuleKind::Atomic => "atomic",
        RuleKind::Silent => "silent",
        RuleKind::Token => "token",
    };
    format!("{kind} {}", render_expr(rule.expr()))
}

/// Renders one expression with the same spelling as the JavaScript renderer.
pub fn render_expr(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::Empty => "empty".to_string(),
        GrammarExpr::AnyChar => "any".to_string(),
        GrammarExpr::Terminal(value) => format!("literal({})", quote(value)),
        GrammarExpr::TerminalInsensitive(value) => {
            format!("literalInsensitive({})", quote(value))
        }
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
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => format!(
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
    format!(
        "range({}, {})",
        quote(&start.to_string()),
        quote(&end.to_string())
    )
}

fn render_list(items: &[GrammarExpr]) -> String {
    items.iter().map(render_expr).collect::<Vec<_>>().join(", ")
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).expect("strings serialize to JSON")
}
