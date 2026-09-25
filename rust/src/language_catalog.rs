//! The registered language catalog shared with the JavaScript runtime.
//!
//! `src/data/language-catalog.json` is generated from
//! `parity/language-grammar-inventory.json` by
//! `js/scripts/build-language-catalog.mjs`; the npm package ships the same
//! file, so both runtimes agree on every alias, file extension, and default
//! grammar.

use std::sync::OnceLock;

use serde_json::Value;

const LANGUAGE_CATALOG_JSON: &str = include_str!("data/language-catalog.json");

/// A grammar that parses a registered language by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarProvenance {
    /// Grammar id in the grammar lock, such as `javascript` or `markdown_inline`.
    pub id: String,
    /// Exact grammar version (crate version or pinned upstream revision).
    pub version: String,
    /// SHA-256 of the generated `parser.c` both runtimes compile.
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

fn parse_catalog() -> Vec<LanguageEntry> {
    let value: Value =
        serde_json::from_str(LANGUAGE_CATALOG_JSON).expect("language catalog is valid JSON");
    value["languages"]
        .as_array()
        .expect("language catalog lists languages")
        .iter()
        .map(|language| LanguageEntry {
            name: string(&language["name"]),
            family: string(&language["family"]),
            aliases: strings(&language["aliases"]),
            extensions: strings(&language["extensions"]),
            grammars: language["grammars"]
                .as_array()
                .expect("language grammars")
                .iter()
                .map(|grammar| GrammarProvenance {
                    id: string(&grammar["id"]),
                    version: string(&grammar["version"]),
                    parser_sha256: string(&grammar["parserSha256"]),
                })
                .collect(),
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
