//! Schema validation and opt-in recovery over the lossless JSON grammar.

mod document;
mod emit;
mod import;
mod integer_policy;
mod numeric_ranges;
mod pattern_policy;
mod patterns;
mod property_names;
mod recovery;
mod schema;
mod tag_policy;
mod validation;

use std::sync::LazyLock;

use serde_json::Value;

use super::feature_runtime::{FeatureGrammarParser, FeatureParseOptions, compile_feature_grammar};
use super::{Grammar, parse_grammar_links};
use crate::{LinkId, SourceSpan};

pub use emit::{JsonSchemaEmission, emit_json_schema};
pub use import::{JsonSchemaGrammar, JsonSchemaImportError, import_json_schema};
pub use recovery::{JsonRecovery, JsonRecoveryAction, JsonRecoveryEdit, recover_json};
pub use validation::{JsonValidation, validate_json};

/// A JSON document diagnostic anchored to the syntax network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonDocumentIssue {
    /// The violated schema keyword, syntax, or repetition.
    pub keyword: String,
    /// RFC 6901 pointer into the document.
    pub pointer: String,
    /// The original UTF-8 source span.
    pub span: SourceSpan,
    /// The concrete syntax link that produced this diagnostic.
    pub link_id: LinkId,
    /// Expected grammar tokens or schema values.
    pub expected: Vec<Value>,
    /// Why parsing rejected the source, for syntax issues.
    pub reason: Option<&'static str>,
    /// The repeated text, for repetition issues.
    pub pattern: Option<String>,
    /// Per-branch diagnostics, for combination keywords.
    pub alternatives: Vec<Vec<Self>>,
    /// The number of accepted branches, for combination keywords.
    pub accepted: Option<usize>,
}

/// A JSON Schema keyword the operation cannot apply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonSchemaIssue {
    /// The keyword that needs a diagnostic.
    pub keyword: String,
    /// RFC 6901 pointer into the schema.
    pub pointer: String,
    /// Why the keyword could not be applied.
    pub message: String,
}

fn base_grammar() -> &'static Grammar {
    static GRAMMAR: LazyLock<Grammar> = LazyLock::new(|| {
        parse_grammar_links(include_str!("../../data/json-document/grammar.lino"))
            .expect("shipped JSON document grammar")
    });
    &GRAMMAR
}

fn parser() -> &'static FeatureGrammarParser {
    static PARSER: LazyLock<FeatureGrammarParser> = LazyLock::new(|| {
        compile_feature_grammar(base_grammar(), None, FeatureParseOptions::default())
            .expect("compilable JSON document grammar")
    });
    &PARSER
}

fn pointer_child(pointer: &str, key: &str) -> String {
    format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"))
}

const fn whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\r' | '\n')
}
