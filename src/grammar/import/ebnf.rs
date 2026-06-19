use ::ebnf::{
    Grammar as EbnfGrammar, Node as EbnfNode, RegexExtKind as EbnfRegexExtKind,
    SymbolKind as EbnfSymbolKind,
};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const EMPTY_SENTINEL: &str = "\u{0}meta-language-empty\u{0}";

/// Parses Extended Backus-Naur Form text into the grammar IR.
///
/// The parser accepts the ISO-style `name = expression ;` spelling used by the
/// issue fixtures and the `::=` spelling accepted by the upstream `ebnf` crate.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the EBNF text cannot be parsed, when a
/// parsed construct cannot be represented, or when a non-terminal reference does
/// not resolve to a rule in the imported grammar.
pub fn import_ebnf(text: &str) -> Result<Grammar, GrammarImportError> {
    if let Some(construct) = find_special_sequence(text) {
        return Err(unsupported_error(GrammarFormat::Ebnf, construct));
    }

    let normalized = normalize_empty_alternatives(text);
    let parsed = ::ebnf::get_grammar(&normalized)
        .map_err(|error| parse_error(GrammarFormat::Ebnf, format!("{error:?}")))?;
    let grammar = lower_grammar(&parsed)?;
    validate_references(&grammar)?;
    Ok(grammar)
}

fn lower_grammar(parsed: &EbnfGrammar) -> Result<Grammar, GrammarImportError> {
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Ebnf);
    for expression in &parsed.expressions {
        grammar.add_rule(GrammarRule::new(
            expression.lhs.clone(),
            lower_node(&expression.rhs)?,
        ));
    }
    Ok(grammar)
}

fn lower_node(node: &EbnfNode) -> Result<GrammarExpr, GrammarImportError> {
    match node {
        EbnfNode::String(value) if value.is_empty() || value == EMPTY_SENTINEL => {
            Ok(GrammarExpr::Empty)
        }
        EbnfNode::String(value) => Ok(GrammarExpr::Terminal(value.clone())),
        EbnfNode::RegexString(value) => Err(unsupported_error(
            GrammarFormat::Ebnf,
            format!("inline regex {value:?}"),
        )),
        EbnfNode::Terminal(name) => Ok(GrammarExpr::NonTerminal(name.clone())),
        EbnfNode::Multiple(nodes) => lower_sequence(nodes.iter().map(lower_node)),
        EbnfNode::RegexExt(inner, kind) => lower_regex_extension(inner, kind),
        EbnfNode::Symbol(left, EbnfSymbolKind::Concatenation, right) => {
            lower_sequence([lower_node(left), lower_node(right)])
        }
        EbnfNode::Symbol(left, EbnfSymbolKind::Alternation, right) => {
            lower_choice([lower_node(left), lower_node(right)])
        }
        EbnfNode::Group(inner) => lower_node(inner),
        EbnfNode::Optional(inner) => lower_node(inner).map(GrammarExpr::optional),
        EbnfNode::Repeat(inner) => lower_node(inner).map(GrammarExpr::zero_or_more),
        EbnfNode::Unknown => Err(unsupported_error(GrammarFormat::Ebnf, "unknown node")),
    }
}

fn lower_regex_extension(
    inner: &EbnfNode,
    kind: &EbnfRegexExtKind,
) -> Result<GrammarExpr, GrammarImportError> {
    let expr = lower_node(inner)?;
    Ok(match kind {
        EbnfRegexExtKind::Repeat0 => GrammarExpr::zero_or_more(expr),
        EbnfRegexExtKind::Repeat1 => GrammarExpr::one_or_more(expr),
        EbnfRegexExtKind::Optional => GrammarExpr::optional(expr),
    })
}

fn lower_sequence<I>(items: I) -> Result<GrammarExpr, GrammarImportError>
where
    I: IntoIterator<Item = Result<GrammarExpr, GrammarImportError>>,
{
    let mut lowered = Vec::new();
    for item in items {
        push_sequence_item(&mut lowered, item?);
    }

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Sequence(lowered),
    })
}

fn push_sequence_item(items: &mut Vec<GrammarExpr>, item: GrammarExpr) {
    match item {
        GrammarExpr::Empty => {}
        GrammarExpr::Sequence(nested) => {
            for item in nested {
                push_sequence_item(items, item);
            }
        }
        item => items.push(item),
    }
}

fn lower_choice<I>(alternatives: I) -> Result<GrammarExpr, GrammarImportError>
where
    I: IntoIterator<Item = Result<GrammarExpr, GrammarImportError>>,
{
    let mut lowered = Vec::new();
    for alternative in alternatives {
        push_choice_alternative(&mut lowered, alternative?);
    }

    if lowered.iter().all(|expr| expr == &GrammarExpr::Empty) {
        return Ok(GrammarExpr::Empty);
    }

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives: lowered,
        },
    })
}

fn push_choice_alternative(alternatives: &mut Vec<GrammarExpr>, alternative: GrammarExpr) {
    match alternative {
        GrammarExpr::Choice {
            ordered: false,
            alternatives: nested,
        } => alternatives.extend(nested),
        alternative => alternatives.push(alternative),
    }
}

fn validate_references(grammar: &Grammar) -> Result<(), GrammarImportError> {
    if let Some(name) = grammar.undefined_nonterminals().into_iter().next() {
        return Err(parse_error(
            GrammarFormat::Ebnf,
            format!("undefined non-terminal {name}"),
        ));
    }
    Ok(())
}

fn find_special_sequence(text: &str) -> Option<String> {
    let mut scanner = Scanner::new(text);
    while let Some((index, character, is_code, _)) = scanner.next() {
        if is_code && character == '?' && is_special_sequence_start(text, index) {
            let end = text[index + character.len_utf8()..]
                .find('?')
                .map_or(text.len(), |relative| {
                    index + character.len_utf8() + relative + 1
                });
            let construct = text[index..end].trim();
            return Some(format!("special sequence {construct:?}"));
        }
    }
    None
}

fn is_special_sequence_start(text: &str, index: usize) -> bool {
    match text[..index]
        .chars()
        .rev()
        .find(|character| !character.is_whitespace())
    {
        Some(character) => matches!(character, '=' | '|' | ',' | '(' | '[' | '{' | ';'),
        None => true,
    }
}

fn normalize_empty_alternatives(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut scanner = Scanner::new(text);
    let mut in_rhs = false;
    let mut empty_alternative_pending = false;

    while let Some((index, character, is_code, depth)) = scanner.next() {
        if is_code {
            if !in_rhs {
                if depth == 0 && character == '=' {
                    in_rhs = true;
                    empty_alternative_pending = true;
                }
            } else {
                match character {
                    '|' => {
                        if empty_alternative_pending {
                            push_empty_sentinel(&mut normalized);
                        }
                        empty_alternative_pending = true;
                    }
                    ';' if depth == 0 => {
                        if empty_alternative_pending {
                            push_empty_sentinel(&mut normalized);
                        }
                        in_rhs = false;
                        empty_alternative_pending = false;
                    }
                    ')' | ']' | '}' => {
                        if empty_alternative_pending {
                            push_empty_sentinel(&mut normalized);
                        }
                        empty_alternative_pending = false;
                    }
                    '(' | '[' | '{' => {
                        empty_alternative_pending = true;
                    }
                    ',' => {
                        empty_alternative_pending = false;
                    }
                    _ if character.is_whitespace() => {}
                    _ => {
                        empty_alternative_pending = false;
                    }
                }
            }
        }
        normalized.push_str(&text[index..index + character.len_utf8()]);
    }
    normalized
}

fn push_empty_sentinel(text: &mut String) {
    text.push('"');
    text.push_str(EMPTY_SENTINEL);
    text.push('"');
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScanState {
    Code,
    SingleQuote,
    DoubleQuote,
}

#[derive(Clone, Debug)]
struct Scanner<'text> {
    text: &'text str,
    cursor: usize,
    state: ScanState,
    escaped: bool,
    depth: usize,
}

impl<'text> Scanner<'text> {
    const fn new(text: &'text str) -> Self {
        Self {
            text,
            cursor: 0,
            state: ScanState::Code,
            escaped: false,
            depth: 0,
        }
    }

    fn next(&mut self) -> Option<(usize, char, bool, usize)> {
        let rest = self.text.get(self.cursor..)?;
        let mut chars = rest.char_indices();
        let (_, character) = chars.next()?;
        let index = self.cursor;
        let was_code = matches!(self.state, ScanState::Code);
        let previous_depth = self.depth;
        self.cursor += character.len_utf8();

        match self.state {
            ScanState::Code => match character {
                '\'' => self.state = ScanState::SingleQuote,
                '"' => self.state = ScanState::DoubleQuote,
                '(' | '[' | '{' => self.depth += 1,
                ')' | ']' | '}' => self.depth = self.depth.saturating_sub(1),
                _ => {}
            },
            ScanState::SingleQuote => self.scan_quoted(character, '\''),
            ScanState::DoubleQuote => self.scan_quoted(character, '"'),
        }

        Some((index, character, was_code, previous_depth))
    }

    fn scan_quoted(&mut self, character: char, quote: char) {
        if self.escaped {
            self.escaped = false;
        } else if character == '\\' {
            self.escaped = true;
        } else if character == quote {
            self.state = ScanState::Code;
        }
    }
}
