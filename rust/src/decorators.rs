//! Decorators: declared, removable changes to the items of a pipeline level.
//!
//! Each level shows a decorator its items as records, maps of string fields,
//! and reads the decorated records back (docs/decorators.md lists the fields
//! and what `drop` means at each level). A [`DecoratorSet`] is stored as
//! Links Notation, one decorator per line, in the same canonical form the
//! JavaScript runtime writes, so both runtimes share one corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};

use crate::grammar::{percent_decode_links_text, percent_encode_links_text};

/// A record a decorator sees: field names and their texts.
pub type DecoratorRecord = BTreeMap<String, String>;

/// The pipeline levels a decorator extends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecoratorLevel {
    /// The rules of an imported grammar.
    Importer,
    /// The rules of a grammar a parser is compiled from.
    GrammarRule,
    /// The decisions of a grammar merge.
    MergeDecision,
    /// The translation of a construct between native grammars.
    ConceptMapping,
    /// The nodes and tokens of a syntax tree.
    Executor,
    /// The ERROR and MISSING nodes of a syntax tree.
    Recovery,
    /// The syntax facts a program model is built from.
    CstToAst,
    /// The replacements of a transformation.
    Transformation,
    /// The lines of an emitted grammar.
    Emitter,
    /// The templates of a translation rule set.
    TranslationRule,
}

/// The pipeline levels a decorator extends, in pipeline order.
pub const DECORATOR_LEVELS: [DecoratorLevel; 10] = [
    DecoratorLevel::Importer,
    DecoratorLevel::GrammarRule,
    DecoratorLevel::MergeDecision,
    DecoratorLevel::ConceptMapping,
    DecoratorLevel::Executor,
    DecoratorLevel::Recovery,
    DecoratorLevel::CstToAst,
    DecoratorLevel::Transformation,
    DecoratorLevel::Emitter,
    DecoratorLevel::TranslationRule,
];

impl DecoratorLevel {
    /// The name the JavaScript runtime and the Links Notation use.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Importer => "importer",
            Self::GrammarRule => "grammar-rule",
            Self::MergeDecision => "merge-decision",
            Self::ConceptMapping => "concept-mapping",
            Self::Executor => "executor",
            Self::Recovery => "recovery",
            Self::CstToAst => "cst-to-ast",
            Self::Transformation => "transformation",
            Self::Emitter => "emitter",
            Self::TranslationRule => "translation-rule",
        }
    }

    /// The level named `name`.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        DECORATOR_LEVELS
            .into_iter()
            .find(|level| level.as_str() == name)
    }
}

impl fmt::Display for DecoratorLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Why a decorator, a set or a decoration is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecoratorError {
    /// What is wrong.
    pub message: String,
}

impl DecoratorError {
    /// An error with `message`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DecoratorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DecoratorError {}

/// One action of a decorator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecoratorAction {
    /// Sets `field` to `value`.
    Set {
        /// The field.
        field: String,
        /// The text it is set to.
        value: String,
    },
    /// Replaces every occurrence of the non-empty text `from` in `field`.
    Replace {
        /// The field.
        field: String,
        /// The text replaced.
        from: String,
        /// The text written instead.
        to: String,
    },
    /// Removes the record.
    Drop,
}

/// One decorator: the records of `level` that have every field of `when`
/// get `actions`, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decorator {
    id: String,
    level: DecoratorLevel,
    order: i64,
    when: Vec<(String, String)>,
    actions: Vec<DecoratorAction>,
}

fn is_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

impl Decorator {
    /// Checks one decorator.
    ///
    /// # Errors
    ///
    /// A [`DecoratorError`] for an invalid id or field name, no action or a
    /// replacement of empty text.
    pub fn new(
        id: impl Into<String>,
        level: DecoratorLevel,
        order: i64,
        when: Vec<(String, String)>,
        actions: Vec<DecoratorAction>,
    ) -> Result<Self, DecoratorError> {
        let id = id.into();
        if !is_name(&id) {
            return Err(DecoratorError::new(format!("invalid decorator id {id:?}")));
        }
        if when.iter().any(|(field, _)| !is_name(field)) {
            return Err(DecoratorError::new(format!(
                "decorator {id} has an invalid condition"
            )));
        }
        if actions.is_empty() {
            return Err(DecoratorError::new(format!("decorator {id} has no action")));
        }
        for action in &actions {
            match action {
                DecoratorAction::Drop => {}
                DecoratorAction::Set { field, .. } if is_name(field) => {}
                DecoratorAction::Replace { field, from, .. } if is_name(field) => {
                    if from.is_empty() {
                        return Err(DecoratorError::new(format!(
                            "decorator {id} replaces an empty or non-string text in {field}"
                        )));
                    }
                }
                _ => {
                    return Err(DecoratorError::new(format!(
                        "decorator {id} has an action without a field"
                    )));
                }
            }
        }
        Ok(Self {
            id,
            level,
            order,
            when,
            actions,
        })
    }

    /// The id, unique in a set.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The level the decorator extends.
    #[must_use]
    pub const fn level(&self) -> DecoratorLevel {
        self.level
    }

    /// The position among the decorators of the level; ties go by id.
    #[must_use]
    pub const fn order(&self) -> i64 {
        self.order
    }

    /// The fields a record must have for the decorator to apply.
    #[must_use]
    pub fn when(&self) -> &[(String, String)] {
        &self.when
    }

    /// The actions, applied in order.
    #[must_use]
    pub fn actions(&self) -> &[DecoratorAction] {
        &self.actions
    }

    fn render(&self) -> String {
        let mut parts = vec![
            format!("decorator {}", self.id),
            format!("(level {})", self.level),
            format!("(order {})", self.order),
        ];
        if !self.when.is_empty() {
            let pairs: Vec<String> = self
                .when
                .iter()
                .map(|(field, value)| format!("({field} {})", percent_encode_links_text(value)))
                .collect();
            parts.push(format!("(when {})", pairs.join(" ")));
        }
        for action in &self.actions {
            parts.push(match action {
                DecoratorAction::Drop => "(drop)".to_owned(),
                DecoratorAction::Set { field, value } => {
                    format!("(set {field} {})", percent_encode_links_text(value))
                }
                DecoratorAction::Replace { field, from, to } => format!(
                    "(replace {field} {} {})",
                    percent_encode_links_text(from),
                    percent_encode_links_text(to)
                ),
            });
        }
        format!("({})", parts.join(" "))
    }
}

/// An immutable set of decorators.
///
/// The decorators of a level compose in a defined order, by `order` and then
/// by id, each one decorating what the one before it produced;
/// [`DecoratorSet::add`] and [`DecoratorSet::remove`] return new sets, so
/// removing a decorator gives exactly the output of the set without it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DecoratorSet {
    decorators: Vec<Decorator>,
}

impl DecoratorSet {
    /// A set of `decorators`, sorted by order and then by id.
    ///
    /// # Errors
    ///
    /// A [`DecoratorError`] when two decorators share an id.
    pub fn new(mut decorators: Vec<Decorator>) -> Result<Self, DecoratorError> {
        let mut ids = BTreeSet::new();
        for decorator in &decorators {
            if !ids.insert(decorator.id.as_str()) {
                return Err(DecoratorError::new(format!(
                    "duplicate decorator id {}",
                    decorator.id
                )));
            }
        }
        decorators.sort_by(|left, right| {
            left.order
                .cmp(&right.order)
                .then_with(|| left.id.cmp(&right.id))
        });
        Ok(Self { decorators })
    }

    /// The set without decorators.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            decorators: Vec::new(),
        }
    }

    /// Reads the decorators of a Links Notation document (see
    /// [`DecoratorSet::to_lino`]).
    ///
    /// # Errors
    ///
    /// A [`DecoratorError`] for text that is not Links Notation or a
    /// statement that is not a valid decorator.
    pub fn from_lino(text: &str) -> Result<Self, DecoratorError> {
        let statements = parse_lino_to_links_with_config(text, &ParserConfig::without_comments())
            .map_err(|error| DecoratorError::new(error.to_string()))?;
        Self::new(
            statements
                .iter()
                .map(read_decorator)
                .collect::<Result<_, _>>()?,
        )
    }

    /// The decorators, in the order they compose.
    #[must_use]
    pub fn decorators(&self) -> &[Decorator] {
        &self.decorators
    }

    /// The number of decorators.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.decorators.len()
    }

    /// Whether the set has no decorator.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.decorators.is_empty()
    }

    /// The ids, in the order the decorators compose.
    #[must_use]
    pub fn ids(&self) -> Vec<&str> {
        self.decorators.iter().map(Decorator::id).collect()
    }

    /// The set with `decorators` added.
    ///
    /// # Errors
    ///
    /// A [`DecoratorError`] when an id is taken.
    pub fn add(
        &self,
        decorators: impl IntoIterator<Item = Decorator>,
    ) -> Result<Self, DecoratorError> {
        Self::new(self.decorators.iter().cloned().chain(decorators).collect())
    }

    /// The set without the decorator `id`.
    ///
    /// # Errors
    ///
    /// A [`DecoratorError`] when the set has no decorator `id`.
    pub fn remove(&self, id: &str) -> Result<Self, DecoratorError> {
        if !self.decorators.iter().any(|decorator| decorator.id == id) {
            return Err(DecoratorError::new(format!("no decorator {id} to remove")));
        }
        Ok(Self {
            decorators: self
                .decorators
                .iter()
                .filter(|decorator| decorator.id != id)
                .cloned()
                .collect(),
        })
    }

    /// The decorators of `level`, in the order they compose.
    pub fn for_level(&self, level: DecoratorLevel) -> impl Iterator<Item = &Decorator> {
        self.decorators
            .iter()
            .filter(move |decorator| decorator.level == level)
    }

    /// Whether the set has a decorator of `level`.
    #[must_use]
    pub fn has(&self, level: DecoratorLevel) -> bool {
        self.for_level(level).next().is_some()
    }

    /// The record the decorators of `level` make of `record`, or `None` when
    /// one of them drops it. A missing field reads as empty text.
    #[must_use]
    pub fn decorate(
        &self,
        level: DecoratorLevel,
        record: &DecoratorRecord,
    ) -> Option<DecoratorRecord> {
        let mut current = record.clone();
        for decorator in self.for_level(level) {
            let applies = decorator
                .when
                .iter()
                .all(|(field, value)| current.get(field).map_or("", String::as_str) == value);
            if !applies {
                continue;
            }
            for action in &decorator.actions {
                match action {
                    DecoratorAction::Drop => return None,
                    DecoratorAction::Set { field, value } => {
                        current.insert(field.clone(), value.clone());
                    }
                    DecoratorAction::Replace { field, from, to } => {
                        let text = current
                            .get(field)
                            .map_or("", String::as_str)
                            .replace(from, to);
                        current.insert(field.clone(), text);
                    }
                }
            }
        }
        Some(current)
    }

    /// Decorates every record of `records`, leaving out the dropped ones.
    #[must_use]
    pub fn decorate_all(
        &self,
        level: DecoratorLevel,
        records: &[DecoratorRecord],
    ) -> Vec<DecoratorRecord> {
        records
            .iter()
            .filter_map(|record| self.decorate(level, record))
            .collect()
    }

    /// The canonical Links Notation of the set, one decorator per line.
    #[must_use]
    pub fn to_lino(&self) -> String {
        self.decorators
            .iter()
            .fold(String::new(), |mut text, decorator| {
                text.push_str(&decorator.render());
                text.push('\n');
                text
            })
    }
}

/// Builds a record from `(field, text)` pairs.
#[must_use]
pub fn decorator_record<const N: usize>(fields: [(&str, &str); N]) -> DecoratorRecord {
    fields
        .into_iter()
        .map(|(field, text)| (field.to_owned(), text.to_owned()))
        .collect()
}

/// The text of `field` in a decorated record, empty when absent.
#[must_use]
pub fn record_field<'a>(record: &'a DecoratorRecord, field: &str) -> &'a str {
    record.get(field).map_or("", String::as_str)
}

type Node = LiNo<String>;

fn word_of(node: &Node) -> Option<&str> {
    match node {
        LiNo::Ref(word) => Some(word),
        LiNo::Link {
            id: Some(word),
            values,
        } if values.is_empty() => Some(word),
        LiNo::Link { .. } => None,
    }
}

fn values_of(node: &Node) -> &[Node] {
    match node {
        LiNo::Link { values, .. } => values,
        LiNo::Ref(_) => &[],
    }
}

fn words(values: &[Node]) -> Result<Vec<&str>, DecoratorError> {
    values
        .iter()
        .map(|value| {
            word_of(value).ok_or_else(|| {
                DecoratorError::new("a decorator field holds a nested link where a word belongs")
            })
        })
        .collect()
}

fn unquote(text: &str, id: &str) -> Result<String, DecoratorError> {
    percent_decode_links_text(text).map_err(|_| {
        DecoratorError::new(format!(
            "decorator {id} has a malformed percent-encoded value {text}"
        ))
    })
}

fn read_decorator(statement: &Node) -> Result<Decorator, DecoratorError> {
    let shape = || {
        DecoratorError::new("a decorator statement must read (decorator <id> (level <level>) …)")
    };
    let LiNo::Link { id: None, values } = statement else {
        return Err(shape());
    };
    let [head, name, rest @ ..] = values.as_slice() else {
        return Err(shape());
    };
    if word_of(head) != Some("decorator") {
        return Err(shape());
    }
    let id = word_of(name).ok_or_else(shape)?.to_owned();
    let mut level = None;
    let mut order = 0;
    let mut when = Vec::new();
    let mut actions = Vec::new();
    for part in rest {
        let parts = values_of(part);
        let keyword = parts.first().and_then(word_of);
        let args = parts.get(1..).unwrap_or_default();
        match keyword {
            Some("level") => {
                let name = words(args)?.first().copied().unwrap_or_default();
                level = Some(DecoratorLevel::parse(name).ok_or_else(|| {
                    DecoratorError::new(format!("decorator {id} names unknown level {name:?}"))
                })?);
            }
            Some("order") => {
                let text = words(args)?.first().copied().unwrap_or_default();
                order = text
                    .parse()
                    .ok()
                    .filter(|_| {
                        let digits = text.strip_prefix('-').unwrap_or(text);
                        !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
                    })
                    .ok_or_else(|| {
                        DecoratorError::new(format!("decorator {id} has a non-integer order"))
                    })?;
            }
            Some("when") => {
                for pair in args {
                    let [field, value] = words(values_of(pair))?[..] else {
                        return Err(DecoratorError::new(format!(
                            "decorator {id} has an invalid condition"
                        )));
                    };
                    when.push((field.to_owned(), unquote(value, &id)?));
                }
            }
            Some("set") => {
                let [field, value] = words(args)?[..] else {
                    return Err(DecoratorError::new(format!(
                        "decorator {id} has an action without a field"
                    )));
                };
                actions.push(DecoratorAction::Set {
                    field: field.to_owned(),
                    value: unquote(value, &id)?,
                });
            }
            Some("replace") => {
                let [field, from, to] = words(args)?[..] else {
                    return Err(DecoratorError::new(format!(
                        "decorator {id} replaces an empty or non-string text"
                    )));
                };
                actions.push(DecoratorAction::Replace {
                    field: field.to_owned(),
                    from: unquote(from, &id)?,
                    to: unquote(to, &id)?,
                });
            }
            Some("drop") => actions.push(DecoratorAction::Drop),
            other => {
                return Err(DecoratorError::new(format!(
                    "decorator {id} has an unknown part {other:?}"
                )));
            }
        }
    }
    let level =
        level.ok_or_else(|| DecoratorError::new(format!("decorator {id} names no level")))?;
    Decorator::new(id, level, order, when, actions)
}
