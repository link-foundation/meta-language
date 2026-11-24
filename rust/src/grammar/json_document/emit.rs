use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;
use serde_json::{Value, json};

use super::super::feature::FeatureExpr;
use super::super::{EmitReport, FieldValue, Grammar, GrammarExpr};

const MAXIMUM_ALTERNATIVES: usize = 1024;
const MAXIMUM_SOURCE_LENGTH: usize = 65_536;
const MAXIMUM_EXPANSION: usize = 4096;

/// Result of inferring a JSON Schema from grammar productions.
#[derive(Clone, Debug)]
pub struct JsonSchemaEmission {
    /// Inferred schema, if the grammar expressed JSON values.
    pub schema: Option<Value>,
    /// Grammar characteristics the schema cannot express.
    pub report: EmitReport,
}

/// Infers semantic constants from a bounded finite language of JSON documents.
#[must_use]
pub fn emit_json_schema(grammar: &Grammar) -> JsonSchemaEmission {
    let mut report = EmitReport::default();
    let mut expansion = Expansion {
        grammar,
        active: HashSet::new(),
        cache: HashMap::new(),
        remaining: MAXIMUM_EXPANSION,
    };
    let mut declarations = grammar.declarations().clone();
    declarations.matching = None;
    if declarations.is_empty()
        && let Some(language) = grammar.start().and_then(|start| expansion.rule(start))
    {
        let mut values = indexmap::IndexMap::new();
        for text in language {
            let Ok(value) = serde_json::from_str::<Value>(&text) else {
                report.lossy.push(
                    "The grammar includes productions outside the JSON document language."
                        .to_owned(),
                );
                return JsonSchemaEmission {
                    schema: None,
                    report,
                };
            };
            values.insert(value.to_string(), value);
        }
        if !values.is_empty() {
            report.lossy.push("The schema represents parsed JSON values. Grammar-specific spellings and trivia are omitted.".to_owned());
            let schema = if values.len() == 1 {
                json!({ "const": values.into_values().next().unwrap() })
            } else {
                json!({ "enum": values.into_values().collect::<Vec<_>>() })
            };
            return JsonSchemaEmission {
                schema: Some(schema),
                report,
            };
        }
    }
    report
        .lossy
        .push("A JSON Schema could not be inferred from the grammar productions.".to_owned());
    JsonSchemaEmission {
        schema: None,
        report,
    }
}

type Language = IndexSet<String>;

fn combine(prefixes: &Language, suffixes: &Language) -> Option<Language> {
    if prefixes.len() * suffixes.len() > MAXIMUM_ALTERNATIVES {
        return None;
    }
    let mut joined = Language::new();
    for prefix in prefixes {
        for suffix in suffixes {
            if prefix.len() + suffix.len() > MAXIMUM_SOURCE_LENGTH {
                return None;
            }
            joined.insert(format!("{prefix}{suffix}"));
        }
    }
    Some(joined)
}

struct Expansion<'grammar> {
    grammar: &'grammar Grammar,
    active: HashSet<&'grammar str>,
    cache: HashMap<&'grammar str, Option<Language>>,
    remaining: usize,
}

impl<'grammar> Expansion<'grammar> {
    fn rule(&mut self, name: &'grammar str) -> Option<Language> {
        if let Some(language) = self.cache.get(name) {
            return language.clone();
        }
        let rule = self.grammar.rule(name)?;
        if rule.attributes.modes.is_some()
            || !rule.attributes.parameters.is_empty()
            || rule.attributes.action.is_some()
            || !self.active.insert(name)
        {
            return None;
        }
        let language = self.expression(&rule.expr);
        self.active.remove(name);
        self.cache.insert(name, language.clone());
        language
    }

    fn expression(&mut self, expression: &'grammar GrammarExpr) -> Option<Language> {
        self.remaining = self.remaining.checked_sub(1)?;
        if self.active.len() > 128 {
            return None;
        }
        match expression {
            GrammarExpr::Empty => Some(Language::from([String::new()])),
            GrammarExpr::Terminal(text) => {
                (text.len() <= MAXIMUM_SOURCE_LENGTH).then(|| Language::from([text.clone()]))
            }
            GrammarExpr::NonTerminal(name) => self.rule(name),
            GrammarExpr::Sequence(items) => {
                let mut parts = Language::from([String::new()]);
                for item in items {
                    parts = combine(&parts, &self.expression(item)?)?;
                }
                Some(parts)
            }
            GrammarExpr::Choice {
                ordered: false,
                alternatives,
            } => {
                let mut choices = Language::new();
                for item in alternatives {
                    choices.extend(self.expression(item)?);
                    if choices.len() > MAXIMUM_ALTERNATIVES {
                        return None;
                    }
                }
                Some(choices)
            }
            GrammarExpr::Optional(item) => self.repeat(item, 0, 1),
            GrammarExpr::Repeat {
                expr,
                min,
                max: Some(max),
            } => self.repeat(expr, *min, *max),
            GrammarExpr::Capture { expr, .. } => self.expression(expr),
            GrammarExpr::Feature(feature) => match feature.as_ref() {
                FeatureExpr::Form(form)
                    if matches!(form.head.as_str(), "alias" | "token" | "immediateToken") =>
                {
                    form.fields
                        .iter()
                        .find_map(|value| match value {
                            FieldValue::Expression(expression) => Some(expression),
                            _ => None,
                        })
                        .and_then(|expression| self.expression(expression))
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn repeat(
        &mut self,
        expression: &'grammar GrammarExpr,
        min: usize,
        max: usize,
    ) -> Option<Language> {
        if max > 64 {
            return None;
        }
        let parts = self.expression(expression)?;
        let mut strings = Language::from([String::new()]);
        let mut choices = Language::new();
        for count in 0..=max {
            if count >= min {
                choices.extend(strings.iter().cloned());
            }
            if choices.len() > MAXIMUM_ALTERNATIVES {
                return None;
            }
            if count < max {
                strings = combine(&strings, &parts)?;
            }
        }
        Some(choices)
    }
}
