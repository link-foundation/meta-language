use crate::grammar::{CharClassItem, Grammar, GrammarExpr};

pub(super) fn write_surface(grammar: &Grammar) -> String {
    let mut output = String::new();
    if let Some(start) = grammar.start() {
        output.push_str("(start: ");
        output.push_str(start);
        output.push_str(")\n");
    }
    for rule in grammar.rules() {
        output.push('(');
        output.push_str(rule.name());
        output.push_str(": ");
        output.push_str(&write_expr(rule.expr(), Precedence::Choice));
        output.push_str(")\n");
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Choice = 0,
    Sequence = 1,
    Prefix = 2,
    Postfix = 3,
    Atom = 4,
}

fn write_expr(expr: &GrammarExpr, parent: Precedence) -> String {
    let (text, precedence) = match expr {
        GrammarExpr::Empty => ("()".to_string(), Precedence::Atom),
        GrammarExpr::Terminal(value) => (quote(value, '"'), Precedence::Atom),
        GrammarExpr::TerminalInsensitive(value) => (quote(value, '`'), Precedence::Atom),
        GrammarExpr::CharRange(start, end) => (write_char_range(*start, *end), Precedence::Atom),
        GrammarExpr::CharClass { negated, items } => {
            (write_char_class(*negated, items), Precedence::Atom)
        }
        GrammarExpr::AnyChar => (".".to_string(), Precedence::Atom),
        GrammarExpr::NonTerminal(name) => (name.clone(), Precedence::Atom),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let separator = if *ordered { " / " } else { " | " };
            (
                alternatives
                    .iter()
                    .map(|alternative| write_expr(alternative, Precedence::Choice))
                    .collect::<Vec<_>>()
                    .join(separator),
                Precedence::Choice,
            )
        }
        GrammarExpr::Sequence(items) => (
            items
                .iter()
                .map(|item| write_expr(item, Precedence::Sequence))
                .collect::<Vec<_>>()
                .join(" "),
            Precedence::Sequence,
        ),
        GrammarExpr::Optional(expr) => (
            format!("{}?", write_expr(expr, Precedence::Postfix)),
            Precedence::Postfix,
        ),
        GrammarExpr::ZeroOrMore(expr) => (
            format!("{}*", write_expr(expr, Precedence::Postfix)),
            Precedence::Postfix,
        ),
        GrammarExpr::OneOrMore(expr) => (
            format!("{}+", write_expr(expr, Precedence::Postfix)),
            Precedence::Postfix,
        ),
        GrammarExpr::Repeat { expr, min, max } => {
            let bounds = max.map_or_else(|| format!("{min},"), |max| format!("{min},{max}"));
            (
                format!("{}{{{bounds}}}", write_expr(expr, Precedence::Postfix)),
                Precedence::Postfix,
            )
        }
        GrammarExpr::And(expr) => (
            format!("& {}", write_expr(expr, Precedence::Prefix)),
            Precedence::Prefix,
        ),
        GrammarExpr::Not(expr) => (
            format!("! {}", write_expr(expr, Precedence::Prefix)),
            Precedence::Prefix,
        ),
        GrammarExpr::Capture { label, expr } => {
            let inner = write_expr(expr, Precedence::Choice);
            let text = label.as_ref().map_or_else(
                || format!("{{ {inner} }}"),
                |label| format!("{{ {label} : {inner} }}"),
            );
            (text, Precedence::Atom)
        }
    };

    if precedence < parent {
        format!("({text})")
    } else {
        text
    }
}

fn write_char_range(start: char, end: char) -> String {
    if simple_class_char(start) && simple_class_char(end) {
        format!("[{start}-{end}]")
    } else {
        format!(
            "[{} {}]",
            quote(&start.to_string(), '\''),
            quote(&end.to_string(), '\'')
        )
    }
}

fn write_char_class(negated: bool, items: &[CharClassItem]) -> String {
    let mut output = String::new();
    output.push('[');
    if negated {
        output.push('^');
        if !items.is_empty() {
            output.push(' ');
        }
    }
    output.push_str(
        &items
            .iter()
            .map(write_char_class_item)
            .collect::<Vec<_>>()
            .join(" "),
    );
    output.push(']');
    output
}

fn write_char_class_item(item: &CharClassItem) -> String {
    match item {
        CharClassItem::Char(value) if simple_class_char(*value) => value.to_string(),
        CharClassItem::Char(value) => quote(&value.to_string(), '\''),
        CharClassItem::Range(start, end)
            if simple_class_char(*start) && simple_class_char(*end) =>
        {
            format!("{start}-{end}")
        }
        CharClassItem::Range(start, end) => {
            format!(
                "{} {}",
                quote(&start.to_string(), '\''),
                quote(&end.to_string(), '\'')
            )
        }
    }
}

fn simple_class_char(value: char) -> bool {
    !value.is_whitespace()
        && !matches!(
            value,
            '[' | ']'
                | '{'
                | '}'
                | '('
                | ')'
                | '\''
                | '"'
                | '`'
                | ':'
                | ','
                | '?'
                | '*'
                | '+'
                | '/'
                | '|'
                | '&'
                | '!'
                | '.'
                | '^'
                | '-'
        )
}

fn quote(value: &str, quote: char) -> String {
    let escaped = value.replace(quote, &quote.to_string().repeat(2));
    format!("{quote}{escaped}{quote}")
}
