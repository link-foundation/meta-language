//! The foundations meta-language knows, as data.
//!
//! `src/data/foundation-models.json` is written from
//! `parity/foundation-models.json` by `js/scripts/build-foundation-models.mjs`;
//! the npm package ships the same file. It records integer, overflow,
//! universe, effect, proof and logic models, the models each language uses,
//! and the explicit correspondences between distinct models. No model is the
//! universal logic every other model reduces to: a consumer brings its own
//! foundation and proof authority and relates it to these models through
//! correspondences.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::SourceAlias;

const FOUNDATION_MODELS_JSON: &str = include_str!("data/foundation-models.json");

/// The kinds of correspondence between two distinct models; none of them claims the models are the same.
pub const CORRESPONDENCE_KINDS: [&str; 4] = ["embedding", "conditional", "encoding", "restatement"];

/// The families whose models are logics or proof authorities.
///
/// A model of one of them that every other model of its family embeds into, or (beyond the
/// booleans every language has) that every language is made to use, would be
/// a hard-coded universal logic.
pub const LOGIC_FAMILIES: [&str; 3] = ["logic-model", "proof-system", "universe-model"];

/// A family of foundation models, such as `integer-model` or `proof-system`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoundationFamily {
    /// Stable identity of the family.
    pub id: String,
    /// What the family's models describe.
    pub definition: String,
}

/// One foundation model: a meaning, not a spelling.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoundationModel {
    /// Stable identity, `<family>.<name>`, such as `overflow-model.wrapping`.
    pub id: String,
    /// The family the model belongs to.
    pub family: String,
    /// What the model means.
    pub definition: String,
    /// The properties that distinguish the model from the other models of its family.
    pub properties: Map<String, Value>,
    /// The names languages give the model.
    pub source_aliases: Vec<SourceAlias>,
}

/// A recorded relation between two distinct models and the condition under which it holds.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoundationCorrespondence {
    /// The model the correspondence starts from.
    pub from: String,
    /// The model the correspondence leads to.
    pub to: String,
    /// One of [`CORRESPONDENCE_KINDS`].
    pub kind: String,
    /// When the two models agree.
    pub condition: String,
    /// The translation encodings and assumptions this correspondence justifies.
    #[serde(default)]
    pub translation_encodings: Vec<String>,
}

/// Two models that must never be merged, with the reason.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoundationDistinction {
    /// The two models.
    pub models: Vec<String>,
    /// What distinguishes them.
    pub reason: String,
    /// The translation encodings and assumptions this distinction justifies.
    #[serde(default)]
    pub translation_encodings: Vec<String>,
}

/// The models one language uses.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageFoundation {
    /// The language, such as `Rust` or `Lean`.
    pub language: String,
    /// The identities of the models the language uses, one or more per family.
    pub models: Vec<String>,
}

/// A foundation register: the shape of `parity/foundation-models.json`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoundationRegister {
    /// What the register records.
    pub description: String,
    /// The model families.
    pub families: Vec<FoundationFamily>,
    /// The models.
    pub models: Vec<FoundationModel>,
    /// The correspondences between distinct models.
    pub correspondences: Vec<FoundationCorrespondence>,
    /// The models that must stay distinct.
    pub distinctions: Vec<FoundationDistinction>,
    /// The models each language uses.
    pub languages: Vec<LanguageFoundation>,
}

/// A way a register fails to keep foundations neutral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationProblem {
    /// The problem kind, the same string the JavaScript runtime reports, such as `universal-model`.
    pub kind: &'static str,
    /// The model, correspondence or language the problem is about.
    pub subject: String,
    /// A readable explanation.
    pub message: String,
}

/// Returns the embedded foundation register.
#[must_use]
pub fn foundation_register() -> &'static FoundationRegister {
    static REGISTER: OnceLock<FoundationRegister> = OnceLock::new();
    REGISTER.get_or_init(|| {
        serde_json::from_str(FOUNDATION_MODELS_JSON)
            .expect("the embedded foundation models are valid")
    })
}

/// Returns the model named `id`.
#[must_use]
pub fn foundation_model(id: &str) -> Option<&'static FoundationModel> {
    foundation_register()
        .models
        .iter()
        .find(|model| model.id == id)
}

/// Returns the models `language` uses, optionally only those of one family.
#[must_use]
pub fn language_foundation_models(
    language: &str,
    family: Option<&str>,
) -> Vec<&'static FoundationModel> {
    foundation_register()
        .languages
        .iter()
        .filter(|entry| entry.language == language)
        .flat_map(|entry| entry.models.iter().filter_map(|id| foundation_model(id)))
        .filter(|model| family.is_none_or(|family| model.family == family))
        .collect()
}

/// Returns the recorded correspondences between two models, in either direction.
#[must_use]
pub fn foundation_correspondences(a: &str, b: &str) -> Vec<&'static FoundationCorrespondence> {
    foundation_register()
        .correspondences
        .iter()
        .filter(|entry| (entry.from == a && entry.to == b) || (entry.from == b && entry.to == a))
        .collect()
}

/// Checks that a register keeps foundations neutral.
///
/// Every model belongs to a declared family, distinct models are not merged or
/// claimed equal, every correspondence states its condition, every language
/// names its own models, and no model is a universal logic. Unknown fields such
/// as `universal` are rejected when the register is parsed. An empty result
/// means the register is neutral; the kinds match the JavaScript runtime's
/// `checkFoundationModels`.
#[must_use]
pub fn check_foundation_models(register: &FoundationRegister) -> Vec<FoundationProblem> {
    let mut problems = Vec::new();
    let mut report = |kind, subject: &str, message: String| {
        problems.push(FoundationProblem {
            kind,
            subject: subject.to_owned(),
            message,
        });
    };
    let mut families = BTreeSet::new();
    for family in &register.families {
        if !families.insert(family.id.as_str()) {
            report(
                "duplicate",
                &family.id,
                format!("family {} is declared twice", family.id),
            );
        }
    }
    let mut models = BTreeMap::new();
    for model in &register.models {
        if models.insert(model.id.as_str(), model).is_some() {
            report(
                "duplicate",
                &model.id,
                format!("model {} is declared twice", model.id),
            );
        }
        if !families.contains(model.family.as_str()) {
            report(
                "unknown-family",
                &model.id,
                format!("{} belongs to undeclared family {}", model.id, model.family),
            );
        } else if !model.id.starts_with(&format!("{}.", model.family)) {
            report(
                "identity",
                &model.id,
                format!(
                    "{} is not named under its family {}",
                    model.id, model.family
                ),
            );
        }
        if model.definition.is_empty() {
            report(
                "definition",
                &model.id,
                format!("{} has no definition", model.id),
            );
        }
        if model.properties.is_empty() {
            report(
                "properties",
                &model.id,
                format!(
                    "{} records no properties, so its distinctness cannot be checked",
                    model.id
                ),
            );
        }
    }
    for (index, model) in register.models.iter().enumerate() {
        for other in &register.models[index + 1..] {
            // Map equality ignores key order, like the JavaScript runtime's canonical JSON comparison.
            if model.family == other.family && model.properties == other.properties {
                report(
                    "indistinct-models",
                    &model.id,
                    format!(
                        "{} and {} record the same properties: one meaning must be one model with source aliases",
                        model.id, other.id
                    ),
                );
            }
        }
    }
    let mut pairs = BTreeSet::new();
    for entry in &register.correspondences {
        let subject = format!("{} -> {}", entry.from, entry.to);
        for end in [&entry.from, &entry.to] {
            if !models.contains_key(end.as_str()) {
                report(
                    "unknown-model",
                    &subject,
                    format!("{subject} names undeclared model {end}"),
                );
            }
        }
        if entry.from == entry.to {
            report(
                "self-correspondence",
                &subject,
                format!("{subject} relates a model to itself"),
            );
        }
        if !CORRESPONDENCE_KINDS.contains(&entry.kind.as_str()) {
            report(
                "correspondence-kind",
                &subject,
                format!(
                    "{subject} has kind {}; two distinct models never correspond exactly, and the same meaning must be one model",
                    entry.kind
                ),
            );
        }
        if entry.condition.trim().chars().count() < 20 {
            report(
                "unconditioned-correspondence",
                &subject,
                format!("{subject} does not state the condition under which the models agree"),
            );
        }
        let key = if entry.from <= entry.to {
            (&entry.from, &entry.to)
        } else {
            (&entry.to, &entry.from)
        };
        if !pairs.insert(key) {
            report(
                "duplicate",
                &subject,
                format!("{subject} is recorded twice"),
            );
        }
    }
    for entry in &register.distinctions {
        let subject = entry.models.join(" / ");
        if entry.models.len() != 2 {
            report(
                "distinction",
                &subject,
                "a distinction names exactly two models".to_owned(),
            );
        }
        for id in &entry.models {
            if !models.contains_key(id.as_str()) {
                report(
                    "unknown-model",
                    &subject,
                    format!("distinction {subject} names undeclared model {id}"),
                );
            }
        }
        if entry.reason.trim().chars().count() < 20 {
            report(
                "distinction",
                &subject,
                format!("distinction {subject} gives no reason"),
            );
        }
    }
    for entry in &register.languages {
        let mut used = BTreeSet::new();
        for id in &entry.models {
            match models.get(id.as_str()) {
                Some(model) => {
                    used.insert(model.family.as_str());
                }
                None => report(
                    "unknown-model",
                    &entry.language,
                    format!("{} uses undeclared model {id}", entry.language),
                ),
            }
        }
        for family in &families {
            if !used.contains(family) {
                report(
                    "language-family",
                    &entry.language,
                    format!("{} does not record its {family}", entry.language),
                );
            }
        }
    }
    for family in LOGIC_FAMILIES
        .into_iter()
        .filter(|family| families.contains(family))
    {
        let members: Vec<_> = register
            .models
            .iter()
            .filter(|model| model.family == family)
            .collect();
        if members.len() < 2 {
            continue;
        }
        for model in &members {
            let embedded = members
                .iter()
                .filter(|other| other.id != model.id)
                .all(|other| {
                    register.correspondences.iter().any(|entry| {
                        entry.kind == "embedding" && entry.from == other.id && entry.to == model.id
                    })
                });
            if embedded {
                report(
                    "universal-model",
                    &model.id,
                    format!(
                        "every other {family} embeds into {}, which makes it a hard-coded universal logic",
                        model.id
                    ),
                );
            }
            if family != "logic-model"
                && register.languages.len() > 1
                && register
                    .languages
                    .iter()
                    .all(|entry| entry.models.contains(&model.id))
            {
                report(
                    "universal-model",
                    &model.id,
                    format!(
                        "every language is made to use {}, which makes it a hard-coded universal foundation",
                        model.id
                    ),
                );
            }
        }
    }
    problems
}
