//! Emitters for external grammar definition formats.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::grammar::{Grammar, GrammarFormat, GrammarRule};
use crate::translation_rules::TranslationTemplate;

mod abnf;
mod bnf;
mod ebnf;
mod pest;

pub use abnf::emit_abnf;
pub use bnf::emit_bnf;
pub use ebnf::emit_ebnf;
pub use pest::emit_pest;

pub(super) const BNF_RULE_TEMPLATE: &str = "<{name}> ::= {body}";
pub(super) const EBNF_RULE_TEMPLATE: &str = "{name} = {body} ;";
pub(super) const ABNF_RULE_TEMPLATE: &str = "{name} = {body}";
pub(super) const PEST_RULE_TEMPLATE: &str = "{name} = {modifier}{{ {body} }}";

/// Error raised while emitting an external grammar notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarEmitError {
    /// The target notation cannot represent this construct at all.
    Unsupported {
        /// Grammar format being emitted.
        format: GrammarFormat,
        /// Construct name or summary.
        construct: String,
    },
}

impl fmt::Display for GrammarEmitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { format, construct } => {
                write!(
                    formatter,
                    "{format} emit unsupported construct: {construct}"
                )
            }
        }
    }
}

impl Error for GrammarEmitError {}

/// Non-fatal fidelity notes collected while emitting a grammar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EmitReport {
    /// Fidelity-reducing conversions, such as dropped labels or case flags.
    pub lossy: Vec<String>,
}

impl EmitReport {
    pub(super) fn add_lossy(&mut self, note: impl Into<String>) {
        self.lossy.push(note.into());
    }
}

pub(super) fn unsupported_error(
    format: GrammarFormat,
    construct: impl Into<String>,
) -> GrammarEmitError {
    GrammarEmitError::Unsupported {
        format,
        construct: construct.into(),
    }
}

pub(super) fn render_rule_line(template: &str, name: &str, body: &str) -> String {
    let template = TranslationTemplate::new(template);
    render_template_source(template.source(), name, "", body)
}

pub(super) fn render_rule_line_with_modifier(
    template: &str,
    name: &str,
    modifier: &str,
    body: &str,
) -> String {
    let template = TranslationTemplate::new(template);
    render_template_source(template.source(), name, modifier, body)
}

fn render_template_source(source: &str, name: &str, modifier: &str, body: &str) -> String {
    let mut output = String::new();
    let mut chars = source.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                output.push('{');
            }
            '{' => render_placeholder(&mut output, &mut chars, name, modifier, body),
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                output.push('}');
            }
            other => output.push(other),
        }
    }
    output
}

fn render_placeholder<I>(
    output: &mut String,
    chars: &mut std::iter::Peekable<I>,
    name: &str,
    modifier: &str,
    body: &str,
) where
    I: Iterator<Item = char>,
{
    let mut placeholder = String::new();
    let mut closed = false;
    for next in chars.by_ref() {
        if next == '}' {
            closed = true;
            break;
        }
        placeholder.push(next);
    }

    if !closed {
        output.push('{');
        output.push_str(&placeholder);
        return;
    }

    match placeholder.trim() {
        "name" => output.push_str(name),
        "modifier" => output.push_str(modifier),
        "body" => output.push_str(body),
        _ => {
            output.push('{');
            output.push_str(&placeholder);
            output.push('}');
        }
    }
}

pub(super) fn ordered_rules(grammar: &Grammar) -> Vec<&GrammarRule> {
    let rules = grammar.rules();
    let Some(start) = grammar.start() else {
        return rules.iter().collect();
    };
    let Some(start_index) = rules.iter().position(|rule| rule.name() == start) else {
        return rules.iter().collect();
    };

    let mut ordered = Vec::with_capacity(rules.len());
    ordered.push(&rules[start_index]);
    ordered.extend(rules[..start_index].iter());
    ordered.extend(rules[start_index + 1..].iter());
    ordered
}

pub(super) fn finish_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        String::new()
    } else {
        let mut output = lines.join("\n");
        output.push('\n');
        output
    }
}

pub(super) fn expanded_chars(
    format: GrammarFormat,
    construct: &str,
    start: char,
    end: char,
    max_chars: u32,
) -> Result<Vec<char>, GrammarEmitError> {
    let start = start as u32;
    let end = end as u32;
    if start > end {
        return Err(unsupported_error(
            format,
            format!("{construct} has descending bounds U+{start:04X}..=U+{end:04X}"),
        ));
    }
    let span = end - start + 1;
    if span > max_chars {
        return Err(unsupported_error(
            format,
            format!("{construct} expands to {span} characters"),
        ));
    }
    Ok((start..=end).filter_map(char::from_u32).collect())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct HelperRule {
    pub(super) name: String,
    pub(super) body: String,
}

#[derive(Clone, Debug)]
pub(super) struct HelperRules {
    used_names: BTreeSet<String>,
    names_by_key: BTreeMap<String, String>,
    entries: Vec<HelperRule>,
    next_id: usize,
}

impl HelperRules {
    pub(super) fn new(grammar: &Grammar) -> Self {
        Self {
            used_names: grammar
                .rules()
                .iter()
                .map(|rule| rule.name().to_string())
                .collect(),
            names_by_key: BTreeMap::new(),
            entries: Vec::new(),
            next_id: 0,
        }
    }

    pub(super) fn reserve(&mut self, kind: &str, key: impl Into<String>) -> (String, bool) {
        let key = format!("{kind}:{}", key.into());
        if let Some(name) = self.names_by_key.get(&key) {
            return (name.clone(), false);
        }

        let name = self.next_name(kind);
        self.names_by_key.insert(key, name.clone());
        (name, true)
    }

    pub(super) fn push(&mut self, name: String, body: String) {
        self.entries.push(HelperRule { name, body });
    }

    pub(super) fn entries(&self) -> &[HelperRule] {
        &self.entries
    }

    fn next_name(&mut self, kind: &str) -> String {
        loop {
            let name = format!("ml{kind}{}", self.next_id);
            self.next_id += 1;
            if self.used_names.insert(name.clone()) {
                return name;
            }
        }
    }
}
