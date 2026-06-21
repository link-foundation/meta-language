use ::bnf::{Expression as BnfExpression, Grammar as BnfGrammar, Term as BnfTerm};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

/// Parses classic Backus-Naur Form text into the grammar IR.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the BNF text cannot be parsed, when a
/// parsed construct cannot be represented, or when a non-terminal reference does
/// not resolve to a rule in the imported grammar.
pub fn import_bnf(text: &str) -> Result<Grammar, GrammarImportError> {
    let normalized = normalize_empty_alternatives(text);
    let parsed = BnfGrammar::parse_from::<::bnf::BNF>(&normalized)
        .map_err(|error| parse_error(GrammarFormat::Bnf, error.to_string()))?;
    let grammar = lower_grammar(&parsed)?;
    validate_references(&grammar)?;
    Ok(grammar)
}

fn lower_grammar(parsed: &BnfGrammar) -> Result<Grammar, GrammarImportError> {
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Bnf);
    for production in parsed.productions_iter() {
        let name = match &production.lhs {
            BnfTerm::Nonterminal(name) => name.clone(),
            BnfTerm::Terminal(value) => {
                return Err(unsupported_error(
                    GrammarFormat::Bnf,
                    format!("terminal production lhs {value:?}"),
                ));
            }
        };
        let alternatives = production
            .rhs_iter()
            .map(lower_expression)
            .collect::<Vec<_>>();
        let expr = lower_alternatives(alternatives);
        grammar.add_rule(GrammarRule::new(name, expr));
    }
    Ok(grammar)
}

fn lower_alternatives(alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    if alternatives.iter().all(|expr| expr == &GrammarExpr::Empty) {
        return GrammarExpr::Empty;
    }
    match alternatives.len() {
        0 => GrammarExpr::Empty,
        1 => alternatives
            .into_iter()
            .next()
            .expect("one alternative exists"),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives,
        },
    }
}

fn lower_expression(expression: &BnfExpression) -> GrammarExpr {
    let mut items = Vec::new();
    for term in expression.terms_iter() {
        let item = lower_term(term);
        if item != GrammarExpr::Empty {
            items.push(item);
        }
    }

    match items.len() {
        0 => GrammarExpr::Empty,
        1 => items.remove(0),
        _ => GrammarExpr::Sequence(items),
    }
}

fn lower_term(term: &BnfTerm) -> GrammarExpr {
    match term {
        BnfTerm::Terminal(value) if value.is_empty() => GrammarExpr::Empty,
        BnfTerm::Terminal(value) => GrammarExpr::Terminal(value.clone()),
        BnfTerm::Nonterminal(name) => GrammarExpr::NonTerminal(name.clone()),
    }
}

fn validate_references(grammar: &Grammar) -> Result<(), GrammarImportError> {
    if let Some(name) = grammar.undefined_nonterminals().into_iter().next() {
        return Err(parse_error(
            GrammarFormat::Bnf,
            format!("undefined non-terminal <{name}>"),
        ));
    }
    Ok(())
}

fn normalize_empty_alternatives(text: &str) -> String {
    text.lines()
        .map(normalize_empty_alternatives_in_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_empty_alternatives_in_line(line: &str) -> String {
    let Some(separator) = find_production_separator(line) else {
        return line.to_string();
    };
    let (head, tail) = line.split_at(separator + "::=".len());
    let (rhs, comment) = split_comment(tail);
    let alternatives = split_alternatives(rhs);
    if alternatives
        .iter()
        .all(|alternative| !alternative.trim().is_empty())
    {
        return line.to_string();
    }

    let normalized = alternatives
        .into_iter()
        .map(|alternative| {
            if alternative.trim().is_empty() {
                " '' ".to_string()
            } else {
                alternative
            }
        })
        .collect::<Vec<_>>()
        .join("|");
    format!("{head}{normalized}{comment}")
}

fn find_production_separator(line: &str) -> Option<usize> {
    let mut scanner = Scanner::new(line);
    while let Some((index, character)) = scanner.next() {
        if scanner.is_code() && character == ':' && line[index..].starts_with("::=") {
            return Some(index);
        }
    }
    None
}

fn split_comment(line: &str) -> (&str, &str) {
    let mut scanner = Scanner::new(line);
    while let Some((index, character)) = scanner.next() {
        if scanner.is_code() && character == ';' {
            return line.split_at(index);
        }
    }
    (line, "")
}

fn split_alternatives(text: &str) -> Vec<String> {
    let mut scanner = Scanner::new(text);
    let mut start = 0;
    let mut alternatives = Vec::new();
    while let Some((index, character)) = scanner.next() {
        if scanner.is_code() && character == '|' {
            alternatives.push(text[start..index].to_string());
            start = index + character.len_utf8();
        }
    }
    alternatives.push(text[start..].to_string());
    alternatives
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScanState {
    Code,
    SingleQuote,
    DoubleQuote,
    NonTerminal,
}

#[derive(Clone, Debug)]
struct Scanner<'text> {
    text: &'text str,
    cursor: usize,
    state: ScanState,
}

impl<'text> Scanner<'text> {
    const fn new(text: &'text str) -> Self {
        Self {
            text,
            cursor: 0,
            state: ScanState::Code,
        }
    }

    const fn is_code(&self) -> bool {
        matches!(self.state, ScanState::Code)
    }

    fn next(&mut self) -> Option<(usize, char)> {
        let rest = self.text.get(self.cursor..)?;
        let mut chars = rest.char_indices();
        let (_, character) = chars.next()?;
        let index = self.cursor;
        self.cursor += character.len_utf8();

        match (self.state, character) {
            (ScanState::Code, '\'') => self.state = ScanState::SingleQuote,
            (ScanState::Code, '"') => self.state = ScanState::DoubleQuote,
            (ScanState::Code, '<') => self.state = ScanState::NonTerminal,
            (ScanState::SingleQuote, '\'') | (ScanState::DoubleQuote, '"') => {
                self.state = ScanState::Code;
            }
            (ScanState::NonTerminal, '>') => self.state = ScanState::Code,
            _ => {}
        }

        Some((index, character))
    }
}
