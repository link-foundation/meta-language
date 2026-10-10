//! The registered language catalog shared with the JavaScript runtime.
//!
//! `src/data/language-catalog.json` is generated from
//! `parity/language-grammar-inventory.json` by
//! `js/scripts/build-language-catalog.mjs`; the npm package ships the same
//! file, so both runtimes agree on every alias, file extension, and default
//! grammar.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::Value;

const LANGUAGE_CATALOG_JSON: &str = include_str!("data/language-catalog.json");

/// A grammar that parses a registered language by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarProvenance {
    /// Grammar id in the grammar lock, such as `javascript` or `markdown_inline`,
    /// or a native grammar id of the catalog, such as `native-json`.
    pub id: String,
    /// Exact grammar version (crate version or pinned upstream revision).
    pub version: String,
    /// SHA-256 of the generated `parser.c` both runtimes compile, or of a
    /// native grammar's Links Notation text.
    pub parser_sha256: String,
}

/// One registered language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    /// Canonical inventory name, such as `JavaScript` or `sql-postgres`.
    pub name: String,
    /// Inventory family, such as `programming`, `sql`, or `natural`.
    pub family: String,
    /// Case-insensitive aliases accepted wherever a language name is.
    pub aliases: Vec<String>,
    /// File extensions (lowercase, with the leading dot) that dispatch here.
    pub extensions: Vec<String>,
    /// Grammars that parse the language by default, primary grammar first.
    pub grammars: Vec<GrammarProvenance>,
}

/// Returns every registered language in inventory order.
#[must_use]
pub fn language_catalog() -> &'static [LanguageEntry] {
    static CATALOG: OnceLock<Vec<LanguageEntry>> = OnceLock::new();
    CATALOG.get_or_init(parse_catalog)
}

/// Returns the catalog entry for a language name or alias (case-insensitive).
#[must_use]
pub fn language_entry(language: &str) -> Option<&'static LanguageEntry> {
    language_catalog().iter().find(|entry| {
        entry.name.eq_ignore_ascii_case(language)
            || entry
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(language))
    })
}

/// Returns the canonical inventory name for a language name or alias.
#[must_use]
pub fn canonical_language_name(language: &str) -> Option<&'static str> {
    language_entry(language).map(|entry| entry.name.as_str())
}

/// Returns every language registered for a file path, most specific first:
/// languages whose extension is a longer case-insensitive suffix of the path
/// come before shorter ones, and ties keep catalog order.
#[must_use]
pub fn language_candidates_for_path(path: &str) -> Vec<&'static str> {
    let lower = path.to_lowercase();
    let mut candidates: Vec<(usize, usize, &'static str)> = language_catalog()
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            entry
                .extensions
                .iter()
                .filter(|extension| lower.ends_with(extension.as_str()))
                .map(String::len)
                .max()
                .map(|length| (length, index, entry.name.as_str()))
        })
        .collect();
    candidates.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    candidates.into_iter().map(|(_, _, name)| name).collect()
}

/// Returns the language a file path dispatches to.
#[must_use]
pub fn language_for_path(path: &str) -> Option<&'static str> {
    language_candidates_for_path(path).into_iter().next()
}

/// Returns the grammars that parse a language by default.
#[must_use]
pub fn grammar_provenance(language: &str) -> &'static [GrammarProvenance] {
    language_entry(language).map_or(&[], |entry| entry.grammars.as_slice())
}

/// Returns the pinned tree-sitter grammars a language's native default
/// grammar replaced, which stay its oracles; empty for a language without a
/// native default grammar.
#[must_use]
pub fn oracle_grammar_provenance(language: &str) -> &'static [GrammarProvenance] {
    static ORACLES: OnceLock<Vec<(String, Vec<GrammarProvenance>)>> = OnceLock::new();
    let Some(entry) = language_entry(language) else {
        return &[];
    };
    ORACLES
        .get_or_init(|| {
            catalog_value()["languages"]
                .as_array()
                .expect("language catalog lists languages")
                .iter()
                .filter(|language| language["oracleGrammars"].is_array())
                .map(|language| {
                    (
                        string(&language["name"]),
                        provenance(&language["oracleGrammars"]),
                    )
                })
                .collect()
        })
        .iter()
        .find(|(name, _)| *name == entry.name)
        .map_or(&[], |(_, grammars)| grammars.as_slice())
}

/// A native Links Notation grammar of the catalog, which parses its languages
/// by default, with the kinds its tree places as its tree-sitter oracle does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeGrammarEntry {
    /// Grammar id, such as `native-json`.
    pub id: String,
    /// The grammar file below `src/data/`, such as `native-grammars/json.lino`.
    pub file: String,
    /// Leaf kinds the oracle drops, such as a byte order mark.
    pub hidden: Vec<String>,
    /// Leaf kinds the oracle keeps inside a node without a node of their own.
    pub anonymous: Vec<String>,
    /// Node kinds the oracle marks as extras.
    pub extras: Vec<String>,
    /// Whether the root span includes trivia before its first token.
    pub root_includes_leading_trivia: bool,
    /// The tree-sitter kind of each rule renamed from it, by rule name.
    pub oracle_kinds: BTreeMap<String, String>,
}

/// Returns every native grammar of the catalog.
#[must_use]
pub fn native_grammars() -> &'static [NativeGrammarEntry] {
    static NATIVE: OnceLock<Vec<NativeGrammarEntry>> = OnceLock::new();
    NATIVE.get_or_init(|| {
        catalog_value()["nativeGrammars"]
            .as_object()
            .map(|grammars| {
                grammars
                    .iter()
                    .map(|(id, grammar)| NativeGrammarEntry {
                        id: id.clone(),
                        file: string(&grammar["file"]),
                        hidden: strings(&grammar["hidden"]),
                        anonymous: strings(&grammar["anonymous"]),
                        extras: strings(&grammar["extras"]),
                        root_includes_leading_trivia: grammar["rootIncludesLeadingTrivia"]
                            .as_bool()
                            .unwrap_or(false),
                        oracle_kinds: grammar["oracleKinds"]
                            .as_object()
                            .map(|kinds| {
                                kinds
                                    .iter()
                                    .map(|(name, kind)| (name.clone(), string(kind)))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

/// Returns the native grammar of the catalog with id `id`.
#[must_use]
pub fn native_grammar(id: &str) -> Option<&'static NativeGrammarEntry> {
    native_grammars().iter().find(|grammar| grammar.id == id)
}

fn catalog_value() -> &'static Value {
    static VALUE: OnceLock<Value> = OnceLock::new();
    VALUE.get_or_init(|| {
        serde_json::from_str(LANGUAGE_CATALOG_JSON).expect("language catalog is valid JSON")
    })
}

fn provenance(grammars: &Value) -> Vec<GrammarProvenance> {
    grammars
        .as_array()
        .expect("language grammars")
        .iter()
        .map(|grammar| GrammarProvenance {
            id: string(&grammar["id"]),
            version: string(&grammar["version"]),
            parser_sha256: string(&grammar["parserSha256"]),
        })
        .collect()
}

fn parse_catalog() -> Vec<LanguageEntry> {
    catalog_value()["languages"]
        .as_array()
        .expect("language catalog lists languages")
        .iter()
        .map(|language| LanguageEntry {
            name: string(&language["name"]),
            family: string(&language["family"]),
            aliases: strings(&language["aliases"]),
            extensions: strings(&language["extensions"]),
            grammars: provenance(&language["grammars"]),
        })
        .collect()
}

fn string(value: &Value) -> String {
    value.as_str().expect("language catalog string").to_string()
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("language catalog array")
        .iter()
        .map(string)
        .collect()
}
