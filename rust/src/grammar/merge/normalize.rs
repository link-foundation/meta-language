//! Meaning-aware normalization of rule definitions.

use std::collections::{BTreeMap, BTreeSet};

use super::GrammarMergeError;
use crate::grammar::{CharClassItem, GrammarExpr};

// Meaning-aware normalization. Each form carries its canonical text and its
// normalized shape; `label` prints a reference, which lets the same pass serve
// source names, external names and bisimulation classes.
#[derive(Clone)]
pub(super) struct Form {
    pub(super) text: String,
    shape: Shape,
}

#[derive(Clone)]
enum Shape {
    Leaf(GrammarExpr),
    Sequence(Vec<Form>),
    Choice {
        ordered: bool,
        items: Vec<Form>,
    },
    Repeat {
        min: usize,
        max: Option<usize>,
        inner: Box<Form>,
    },
    And(Box<Form>),
    Not(Box<Form>),
    Capture {
        label: Option<String>,
        inner: Box<Form>,
    },
}

impl Form {
    const fn leaf(expr: GrammarExpr, text: String) -> Self {
        Self {
            text,
            shape: Shape::Leaf(expr),
        }
    }

    fn empty() -> Self {
        Self::leaf(GrammarExpr::Empty, "empty".to_owned())
    }

    const fn is_empty(&self) -> bool {
        matches!(self.shape, Shape::Leaf(GrammarExpr::Empty))
    }

    pub(super) fn expr(&self) -> GrammarExpr {
        match &self.shape {
            Shape::Leaf(expr) => expr.clone(),
            Shape::Sequence(items) => GrammarExpr::Sequence(items.iter().map(Self::expr).collect()),
            Shape::Choice { ordered, items } => GrammarExpr::Choice {
                ordered: *ordered,
                alternatives: items.iter().map(Self::expr).collect(),
            },
            Shape::Repeat { min, max, inner } => {
                let expr = Box::new(inner.expr());
                match (*min, *max) {
                    (0, None) => GrammarExpr::ZeroOrMore(expr),
                    (1, None) => GrammarExpr::OneOrMore(expr),
                    (0, Some(1)) => GrammarExpr::Optional(expr),
                    (min, max) => GrammarExpr::Repeat { expr, min, max },
                }
            }
            Shape::And(inner) => GrammarExpr::And(Box::new(inner.expr())),
            Shape::Not(inner) => GrammarExpr::Not(Box::new(inner.expr())),
            Shape::Capture { label, inner } => GrammarExpr::Capture {
                label: label.clone(),
                expr: Box::new(inner.expr()),
            },
        }
    }
}

pub(super) fn normalize(
    expr: &GrammarExpr,
    label: &dyn Fn(&str) -> String,
) -> Result<Form, GrammarMergeError> {
    Ok(match expr {
        GrammarExpr::Empty => Form::empty(),
        GrammarExpr::Terminal(value) => literal(value),
        GrammarExpr::TerminalInsensitive(value) => {
            if value.to_lowercase() == value.to_uppercase() {
                literal(value)
            } else {
                Form::leaf(
                    GrammarExpr::TerminalInsensitive(value.clone()),
                    format!("ilit({})", quote(value)),
                )
            }
        }
        GrammarExpr::CharRange(start, end) => char_range(*start, *end),
        GrammarExpr::CharClass { negated, items } => char_class(*negated, items),
        GrammarExpr::AnyChar => Form::leaf(GrammarExpr::AnyChar, "any".to_owned()),
        GrammarExpr::NonTerminal(name) => {
            Form::leaf(GrammarExpr::NonTerminal(name.clone()), label(name))
        }
        GrammarExpr::Sequence(items) => sequence_form(
            items
                .iter()
                .map(|item| normalize(item, label))
                .collect::<Result<_, _>>()?,
        ),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => choice_form(
            alternatives
                .iter()
                .map(|item| normalize(item, label))
                .collect::<Result<_, _>>()?,
            *ordered,
        ),
        GrammarExpr::ZeroOrMore(inner) => repeat_form(normalize(inner, label)?, 0, None)?,
        GrammarExpr::OneOrMore(inner) => repeat_form(normalize(inner, label)?, 1, None)?,
        GrammarExpr::Optional(inner) => repeat_form(normalize(inner, label)?, 0, Some(1))?,
        GrammarExpr::Repeat { expr, min, max } => repeat_form(normalize(expr, label)?, *min, *max)?,
        GrammarExpr::And(inner) => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!("and({})", inner.text),
                shape: Shape::And(Box::new(inner)),
            }
        }
        GrammarExpr::Not(inner) => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!("not({})", inner.text),
                shape: Shape::Not(Box::new(inner)),
            }
        }
        GrammarExpr::Capture {
            label: name,
            expr: inner,
        } => {
            let inner = normalize(inner, label)?;
            Form {
                text: format!(
                    "capture({},{})",
                    name.as_deref().map_or_else(|| "_".to_owned(), quote),
                    inner.text
                ),
                shape: Shape::Capture {
                    label: name.clone(),
                    inner: Box::new(inner),
                },
            }
        }
        GrammarExpr::Feature(feature) => {
            return Err(GrammarMergeError::new(format!(
                "unsupported grammar expression kind: {}",
                feature.head()
            )));
        }
    })
}

fn literal(value: &str) -> Form {
    Form::leaf(
        GrammarExpr::Terminal(value.to_owned()),
        format!("lit({})", quote(value)),
    )
}

fn char_range(start: char, end: char) -> Form {
    if start == end {
        return literal(&start.to_string());
    }
    Form::leaf(
        GrammarExpr::CharRange(start, end),
        format!(
            "range({},{})",
            quote(&start.to_string()),
            quote(&end.to_string())
        ),
    )
}

fn char_class(negated: bool, items: &[CharClassItem]) -> Form {
    let mut unique = BTreeMap::new();
    for item in items {
        match *item {
            CharClassItem::Range(start, end) if start != end => {
                unique.insert(
                    format!("{}-{}", quote(&start.to_string()), quote(&end.to_string())),
                    CharClassItem::Range(start, end),
                );
            }
            CharClassItem::Range(value, _) | CharClassItem::Char(value) => {
                unique.insert(quote(&value.to_string()), CharClassItem::Char(value));
            }
        }
    }
    if !negated && unique.len() == 1 {
        return match unique.into_values().next() {
            Some(CharClassItem::Range(start, end)) => char_range(start, end),
            Some(CharClassItem::Char(value)) => literal(&value.to_string()),
            None => Form::empty(),
        };
    }
    let texts: Vec<&str> = unique.keys().map(String::as_str).collect();
    let text = format!(
        "class({}[{}])",
        if negated { "!" } else { "" },
        texts.join(",")
    );
    Form::leaf(
        GrammarExpr::CharClass {
            negated,
            items: unique.into_values().collect(),
        },
        text,
    )
}

fn sequence_form(forms: Vec<Form>) -> Form {
    let mut items: Vec<Form> = Vec::new();
    for form in forms {
        let parts = match form.shape {
            Shape::Sequence(parts) => parts,
            shape => vec![Form {
                text: form.text,
                shape,
            }],
        };
        for part in parts {
            if part.is_empty() {
                continue;
            }
            // `x x*` matches exactly what `x+` matches, in PEG and in CFG alike.
            let folds = matches!(
                (&part.shape, items.last()),
                (Shape::Repeat { min: 0, max: None, inner }, Some(previous)) if inner.text == previous.text
            );
            if folds && let Some(previous) = items.pop() {
                items.push(some_form(previous));
            } else {
                items.push(part);
            }
        }
    }
    match items.len() {
        0 => Form::empty(),
        1 => items.remove(0),
        _ => Form {
            text: format!(
                "seq({})",
                items
                    .iter()
                    .map(|item| item.text.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            shape: Shape::Sequence(items),
        },
    }
}

fn choice_form(forms: Vec<Form>, ordered: bool) -> Form {
    let mut flattened = Vec::new();
    for form in forms {
        match form.shape {
            Shape::Choice {
                ordered: nested,
                items,
            } if nested == ordered => flattened.extend(items),
            shape => flattened.push(Form {
                text: form.text,
                shape,
            }),
        }
    }
    // A repeated alternative adds nothing: in an ordered choice the later copy
    // can never match where the earlier one failed, and union is idempotent.
    let mut seen = BTreeSet::new();
    let mut items: Vec<Form> = flattened
        .into_iter()
        .filter(|form| seen.insert(form.text.clone()))
        .collect();
    if items.len() == 1 {
        return items.remove(0);
    }
    // The comparison text of an unordered choice is order-free, but the merged
    // expression keeps the source order: a PEG-style parser commits to the
    // first matching alternative, so `letter | letter word` would stop after
    // one letter.
    let mut texts: Vec<&str> = items.iter().map(|item| item.text.as_str()).collect();
    if !ordered {
        texts.sort_unstable();
    }
    Form {
        text: format!(
            "{}({})",
            if ordered { "first" } else { "alt" },
            texts.join(",")
        ),
        shape: Shape::Choice { ordered, items },
    }
}

fn some_form(inner: Form) -> Form {
    Form {
        text: format!("some({})", inner.text),
        shape: Shape::Repeat {
            min: 1,
            max: None,
            inner: Box::new(inner),
        },
    }
}

fn repeat_form(inner: Form, min: usize, max: Option<usize>) -> Result<Form, GrammarMergeError> {
    if max.is_some_and(|max| max < min) {
        return Err(GrammarMergeError::new(format!(
            "invalid repetition bounds {min}..{}",
            max.map_or_else(String::new, |max| max.to_string())
        )));
    }
    if inner.is_empty() || max == Some(0) {
        return Ok(Form::empty());
    }
    let text = match (min, max) {
        (1, Some(1)) => return Ok(inner),
        (0, None) => format!("many({})", inner.text),
        (1, None) => format!("some({})", inner.text),
        (0, Some(1)) => format!("opt({})", inner.text),
        (min, max) => format!(
            "rep({min},{},{})",
            max.map_or_else(|| "*".to_owned(), |max| max.to_string()),
            inner.text
        ),
    };
    Ok(Form {
        text,
        shape: Shape::Repeat {
            min,
            max,
            inner: Box::new(inner),
        },
    })
}

pub(super) fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}
