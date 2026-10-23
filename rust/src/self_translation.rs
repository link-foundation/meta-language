//! Translates meta-language's own modules between JavaScript, TypeScript and
//! Rust through meta-language links.
//!
//! The source is parsed losslessly with its default grammar; a translation
//! into the same language writes the links back byte for byte, and a
//! translation into the other language translates each top-level item the
//! portable core can express and carries every other item verbatim in a
//! marked comment. Both kinds of item keep their source as provenance, so
//! translating an unedited translation back restores the source byte for
//! byte.
//!
//! Mirrors `js/src/self-translation.js`.

use crate::translation::frontend_rules::{
    accept_source_envelope_restoration, accept_source_prefix_restoration,
};
use crate::translation::surface::SExpr;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write as _;
use std::sync::LazyLock;

use regex::Regex;
use sha2::{Digest, Sha256};

use crate::decorators::DecoratorSet;
use crate::grammar::decorate_emitted;
use crate::translation::surface::External;
use crate::{LinkNetwork, ParseConfiguration};

mod binding;
mod emission;
mod publication;
use publication::{carry, comment_fits, record_translated_definitions, source_line};
pub mod representation;
mod representation_rules;
mod shared_array_handles;
pub use binding::{accept_declaration_owned_emission, read_inert_module_binding};
use emission::emit_item;
pub use emission::{
    CheckedItem as CheckedDeclarationEmission,
    check_and_emit_item_with_ownership as check_and_emit_declaration_with_ownership,
};
pub use representation::Context as DeclarationTranslationContext;
use representation::Context;
mod diagnostics;
mod group_translation;
mod scope;
mod semantic_input;
pub use diagnostics::{SelfTranslationDiagnosticReport, self_translate_with_diagnostics};
use group_translation::{translate_group, translate_group_bound};
mod typescript_erasure;
mod typescript_erasure_rules;
mod typescript_portable_typing;
use scope::{BindingScopeContext, translate_binding_groups};

use self::binding::{Binder, ModuleSignatureContext};
use typescript_erasure::requires_type_script_erasure;
pub use typescript_erasure::{ErasedTypeScript, Erasure, erase_type_script};
pub use typescript_portable_typing::project_type_script_typing;

/// The signature of an exported item of a JavaScript or TypeScript module.
///
/// It binds the name a module that imports it uses: a function's parameters
/// and result, or a constant's type and whether a literal gives its value.
pub type SelfTranslationSignature = External;

/// What [`self_translate_with`] translates a module with.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelfTranslationOptions {
    /// Reparse actual emitted code; saved originals never restore semantics.
    pub semantic_translation: bool,
    /// Independent interpretation of complete actual standard Rust shared handles.
    pub rust_shared_array_handle_source_model: String,
    /// The emitter decorators, as [`self_translate_decorated`] applies them.
    pub decorators: DecoratorSet,
    /// The module's directory inside its crate, as path segments: the crate
    /// root when empty. A relative import becomes a `use crate::…` of the
    /// module it names from there.
    pub module_directory: Vec<String>,
    /// The signatures [`self_translation_signatures`] gives for each module
    /// the module imports, by import specifier, which bind the imported names.
    pub imports: BTreeMap<String, Vec<SelfTranslationSignature>>,
    /// Select the shared UTF16 prelude and paired Text lowering; unsupported emissions are carried.
    pub utf16_strings: bool,
}

/// The languages self-translation reads and writes.
pub const SELF_TRANSLATION_LANGUAGES: [&str; 3] = ["JavaScript", "TypeScript", "Rust"];

const HEADER: &str = "// meta-language:self-translation:v1 ";
const CARRIED: &str = "// meta-language:carried ";
const TRANSLATED: &str = "// meta-language:translated ";
const SOURCE_LINE: &str = "// |";
const PRELUDE_BEGIN: &str = "// meta-language:prelude begin";
const PRELUDE_END: &str = "// meta-language:prelude end";
// Portable-core Rust keeps the source's parentheses and names.
const RUST_ALLOW: &str = "#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]";
const COMMENTS: [&str; 3] = ["comment", "line_comment", "block_comment"];

static RUST_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^pub(\([^)]*\))?\s").expect("a valid pattern"));
static JAVASCRIPT_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^export\s").expect("a valid pattern"));
static RUST_MAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bfn\s+main\s*\(").expect("a valid pattern"));
static LINE_BREAK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\r?\n").expect("a valid pattern"));

/// An unknown language or a source the links do not reproduce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslationError {
    /// What went wrong.
    pub message: String,
}

impl fmt::Display for SelfTranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SelfTranslationError {}

/// A top-level item of the source and what self-translation did with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslationItem {
    /// The item's syntax node kind.
    pub term: String,
    /// The item's first byte in the source.
    pub start: usize,
    /// The byte after the item in the source.
    pub end: usize,
    /// `kept` (same language), `translated`, `restored` (provenance gave back
    /// its source), `comment`, `provenance` (a self-translation header or
    /// prelude) or `carried`.
    pub status: &'static str,
    /// Why a carried item was carried.
    pub reason: Option<String>,
}

/// A translated module with the status of each of its top-level items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslation {
    /// The canonical source language.
    pub source_language: &'static str,
    /// The canonical target language.
    pub target_language: &'static str,
    /// The translated module.
    pub code: String,
    /// The source's top-level items in order.
    pub items: Vec<SelfTranslationItem>,
}

/// The self-translation language `language` names, or `None`.
#[must_use]
pub fn self_translation_language(language: &str) -> Option<&'static str> {
    match language.to_lowercase().as_str() {
        "javascript" | "js" | "mjs" => Some("JavaScript"),
        "typescript" | "ts" => Some("TypeScript"),
        "rust" | "rs" => Some("Rust"),
        _ => None,
    }
}

fn required(language: &str) -> Result<&'static str, SelfTranslationError> {
    self_translation_language(language).ok_or_else(|| SelfTranslationError {
        message: format!(
            "self-translation reads and writes {}, not {language}",
            SELF_TRANSLATION_LANGUAGES.join(", ")
        ),
    })
}

// The family a language's code is written in: TypeScript items are read and
// written as JavaScript.
fn family(language: &str) -> &'static str {
    if language == "Rust" {
        "Rust"
    } else {
        "JavaScript"
    }
}

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::new(), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

#[derive(Clone, Debug)]
struct Item<'a> {
    term: String,
    start: usize,
    end: usize,
    comment: bool,
    text: &'a str,
    after: &'a str,
}

/// Translates `source` from `source_language` to `target_language`, each one
/// of [`SELF_TRANSLATION_LANGUAGES`] or an alias of one.
///
/// # Errors
///
/// A [`SelfTranslationError`] for an unknown language or a source its links
/// do not reproduce.
pub fn self_translate(
    source: &str,
    source_language: &str,
    target_language: &str,
) -> Result<SelfTranslation, SelfTranslationError> {
    self_translate_decorated(
        source,
        source_language,
        target_language,
        &DecoratorSet::default(),
    )
}

/// [`self_translate`] with the emitter `decorators` applied.
///
/// They decorate the code each translated item writes before its provenance
/// marker is hashed, so the decorated translation still translates back to
/// `source`.
///
/// # Errors
///
/// As [`self_translate`].
pub fn self_translate_decorated(
    source: &str,
    source_language: &str,
    target_language: &str,
    decorators: &DecoratorSet,
) -> Result<SelfTranslation, SelfTranslationError> {
    let options = SelfTranslationOptions {
        decorators: decorators.clone(),
        ..SelfTranslationOptions::default()
    };
    self_translate_with(source, source_language, target_language, &options)
}

/// [`self_translate`] with `options`.
///
/// From JavaScript or TypeScript into Rust, an item may call and read the
/// module's other top-level items: each item is translated after the items
/// it names, with the signatures of those that translated bound, and the
/// Rust names them as its own translation does. `import { a, b as c } from
/// './m.mjs'` becomes `use crate::m::{a, b as c};`, and the signatures
/// `options.imports` gives for `./m.mjs` bind the imported names the same way.
///
/// # Errors
///
/// As [`self_translate`].
pub fn self_translate_with(
    source: &str,
    source_language: &str,
    target_language: &str,
    options: &SelfTranslationOptions,
) -> Result<SelfTranslation, SelfTranslationError> {
    translate_module(source, source_language, target_language, options)
        .map(|(translation, _)| translation)
}

/// The signatures of the exported items of the JavaScript or TypeScript
/// module `source` that translate into Rust, in source order, for the
/// `imports` of a module that imports them.
///
/// # Errors
///
/// A [`SelfTranslationError`] for an unknown or a Rust language or a source
/// its links do not reproduce.
pub fn self_translation_signatures(
    source: &str,
    language: &str,
    options: &SelfTranslationOptions,
) -> Result<Vec<SelfTranslationSignature>, SelfTranslationError> {
    if family(required(language)?) == "Rust"
        && !options.rust_shared_array_handle_source_model.is_empty()
    {
        return translate_module(source, language, "JavaScript", options)
            .map(|(_, signatures)| signatures);
    }
    if family(required(language)?) == "Rust" && options.semantic_translation {
        return semantic_input::read_independent_rust_module_signatures(source, options);
    }
    if family(required(language)?) != "JavaScript" {
        return Err(SelfTranslationError {
            message: format!(
                "signatures are read from JavaScript or TypeScript modules, not {language}"
            ),
        });
    }
    translate_module(source, language, "Rust", options).map(|(_, signatures)| signatures)
}

fn translate_module(
    source: &str,
    source_language: &str,
    target_language: &str,
    options: &SelfTranslationOptions,
) -> Result<(SelfTranslation, Vec<SelfTranslationSignature>), SelfTranslationError> {
    translate_module_diagnostic(source, source_language, target_language, options, None)
}

fn translate_module_diagnostic(
    source: &str,
    source_language: &str,
    target_language: &str,
    options: &SelfTranslationOptions,
    mut diagnostics: Option<&mut Vec<Option<serde_json::Value>>>,
) -> Result<(SelfTranslation, Vec<SelfTranslationSignature>), SelfTranslationError> {
    let decorators = &options.decorators;
    let from = required(source_language)?;
    let to = required(target_language)?;
    shared_array_handles::validate_options(source, from, options)?;
    let network = LinkNetwork::parse(source, from, ParseConfiguration::default());
    let reconstructed = network.reconstruct_text();
    let utf16_strings = representation::select(&network, source, from, options.utf16_strings);
    if reconstructed != source {
        return Err(SelfTranslationError {
            message: format!("the {from} links of the source do not reproduce it"),
        });
    }
    let items = top_level_items(source, from);
    let kept_family =
        from == "JavaScript" || (from == "TypeScript" && !requires_type_script_erasure(source));
    if family(from) == family(to) && (from == to || (kept_family && !source.starts_with(HEADER))) {
        let items = items
            .iter()
            .map(|item| SelfTranslationItem {
                term: item.term.clone(),
                start: item.start,
                end: item.end,
                status: "kept",
                reason: None,
            })
            .collect();
        let translation = SelfTranslation {
            source_language: from,
            target_language: to,
            code: reconstructed,
            items,
        };
        return Ok((translation, Vec::new()));
    }
    let mut blocks: Vec<String> = Vec::new();
    let mut preludes: Vec<String> = Vec::new();
    let mut recorded = Vec::new();
    let mut gap = String::new();
    let mut whole_publication = if options.rust_shared_array_handle_source_model.is_empty() {
        None
    } else {
        Some(shared_array_handles::read_whole_module(
            source,
            &options.rust_shared_array_handle_source_model,
        )?)
    };
    if whole_publication.is_none()
        && options.semantic_translation
        && from == "Rust"
        && family(to) == "JavaScript"
        && options.imports.is_empty()
        && !options.utf16_strings
    {
        // A refused whole module retains all ordinary per-item diagnostics;
        // unsupported owners cannot publish provisional helper signatures.
        whole_publication =
            semantic_input::read_independent_rust_module_publication(source, &items)
                .ok()
                .flatten();
    }
    let groups = if whole_publication.is_some() {
        vec![Group {
            kind: GroupKind::Item {
                text: source.to_owned(),
                term: "source_file".to_owned(),
            },
            items: items.clone(),
            after: "",
        }]
    } else if options.semantic_translation {
        semantic_input::group_semantic_items(&items, source, from)
    } else {
        group_items(&items, source)
    };
    let (outs, scans, signature_context) = if let Some((emitted, signatures)) = &whole_publication {
        (
            vec![record_translated_definitions(
                (source, "source_file", from, to, decorators),
                emitted.clone(),
                signatures.clone(),
                Some(false),
            )],
            Vec::new(),
            ModuleSignatureContext {
                module_scope_known: true,
                owner_counts: HashMap::new(),
                module_binding_names: Vec::new(),
            },
        )
    } else if family(to) == "Rust" {
        Binder {
            groups: &groups,
            source,
            from,
            to,
            decorators,
            module_directory: &options.module_directory,
            imports: &options.imports,
            utf16_strings,
        }
        .translate()
    } else {
        let outs = groups
            .iter()
            .map(|group| {
                translate_group(
                    group,
                    from,
                    to,
                    decorators,
                    &Context {
                        utf16_strings,
                        ..Context::default()
                    },
                )
            })
            .collect();
        (
            outs,
            Vec::new(),
            ModuleSignatureContext {
                module_scope_known: true,
                owner_counts: HashMap::new(),
                module_binding_names: Vec::new(),
            },
        )
    };
    let module_scope_known = signature_context.module_scope_known;
    let results = translate_binding_groups(
        &groups,
        source,
        from,
        to,
        decorators,
        &outs,
        BindingScopeContext {
            utf16_strings,
            module_scope_known,
            module_binding_names: signature_context.module_binding_names.clone(),
        },
    );
    let layout_envelope =
        crate::translation::self_translation_layout_rules::require_self_translation_layout_envelope(
            (
                source.contains('\r'),
                results
                    .first()
                    .and_then(|(group, _)| group.items.first())
                    .map_or(source.len(), |item| item.start)
                    > 0,
                !matches!(
                    results.last().map_or(source, |(group, _)| group.after),
                    "" | "\n"
                ),
                results
                    .iter()
                    .take(results.len().saturating_sub(1))
                    .any(|(group, _)| group.after != breaks(group.after)),
            ),
        );
    let exported_names: HashSet<&str> = scans
        .iter()
        .flatten()
        .filter(|scan| scan.exported)
        .filter_map(|scan| scan.declares.as_deref())
        .collect();
    let signatures: Vec<SelfTranslationSignature> = if let Some((_, signatures)) = whole_publication
    {
        if results
            .first()
            .is_some_and(|(_, out)| out.status == "translated")
        {
            signatures
        } else {
            Vec::new()
        }
    } else {
        results
            .iter()
            .filter(|(_, out)| out.status == "translated")
            .flat_map(|(_, out)| {
                out.signatures
                    .iter()
                    .filter(|signature| {
                        exported_names.contains(signature.name())
                            && signature_context.accept_signature_binding(signature.name())
                    })
                    .cloned()
            })
            .collect()
    };
    for (group, out) in results {
        for prelude in out.preludes {
            if !preludes.contains(&prelude) {
                preludes.push(prelude);
            }
        }
        for item in &group.items {
            if let Some(records) = diagnostics.as_deref_mut() {
                records.push(out.diagnostic.as_ref().map(|error| {
                    diagnostics::retained(
                        error,
                        group.items.first().map_or(item.start, |item| item.start),
                        group.items.last().map_or(item.end, |item| item.end),
                    )
                }));
            }
            recorded.push(SelfTranslationItem {
                term: item.term.clone(),
                start: item.start,
                end: item.end,
                status: out.status,
                reason: out.reason.clone(),
            });
        }
        if let Some(code) = out.code {
            let separator = if blocks.is_empty() {
                String::new()
            } else {
                breaks(&gap)
            };
            blocks.push(format!("{separator}{code}"));
            group.after.clone_into(&mut gap);
        } else if gap.is_empty() {
            // A dropped group keeps the layout before it.
            group.after.clone_into(&mut gap);
        }
    }
    let body = format!("{}\n", blocks.concat());
    // An unedited translation back gives the source its header describes.
    let header = items.iter().find(|item| {
        !options.semantic_translation && item.comment && item.text.starts_with(HEADER)
    });
    if let Some(header) = header
        && header.text.contains(&format!(" source={to} "))
        && header.text.contains(&format!(" sha256={} ", sha256(&body)))
    {
        let translation = SelfTranslation {
            source_language: from,
            target_language: to,
            code: body,
            items: recorded,
        };
        return Ok((translation, signatures));
    }
    if let Some(header) = header {
        static SOURCE_ENVELOPE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r" envelope-sha256=([a-f0-9]{64}) original=(.*)$").unwrap()
        });
        if let Some(envelope) = SOURCE_ENVELOPE.captures(header.text)
            && let Ok(original) = serde_json::from_str::<String>(&envelope[2])
            && accept_source_envelope_restoration(
                header.text.contains(&format!(" source={to} ")),
                sha256(&source[header.end..]) == envelope[1],
                header.text.split_ascii_whitespace().find_map(|field| {
                    field
                        .strip_prefix("bytes=")
                        .and_then(|value| value.parse::<usize>().ok())
                }) == Some(original.len())
                    && header
                        .text
                        .contains(&format!(" sha256={} ", sha256(&original))),
            )
        {
            return Ok((
                SelfTranslation {
                    source_language: from,
                    target_language: to,
                    code: original,
                    items: recorded,
                },
                signatures,
            ));
        }
        let original_length = header.text.split_ascii_whitespace().find_map(|field| {
            field
                .strip_prefix("bytes=")
                .and_then(|value| value.parse::<usize>().ok())
        });
        if let Some(length) = original_length
            && let Some(original) = body.get(..length)
            && let Some(layout) = body.get(length..)
            && accept_source_prefix_restoration(
                header.text.contains(&format!(" source={to} ")),
                header
                    .text
                    .contains(&format!(" sha256={} ", sha256(original))),
                layout
                    .encode_utf16()
                    .all(crate::translation::lexer::is_js_space),
            )
        {
            return Ok((
                SelfTranslation {
                    source_language: from,
                    target_language: to,
                    code: original.to_owned(),
                    items: recorded,
                },
                signatures,
            ));
        }
    }
    if family(to) == "Rust" && recorded.iter().any(|item| item.status == "translated") {
        preludes.insert(0, RUST_ALLOW.to_owned());
    }
    let mut lines = vec![
        format!(
            "{HEADER}source={from} target={to} sha256={} bytes={}",
            sha256(source),
            source.len()
        ),
        String::new(),
    ];
    if !preludes.is_empty() {
        lines.push(PRELUDE_BEGIN.to_owned());
        for (index, prelude) in preludes.into_iter().enumerate() {
            if index > 0 {
                lines.push(String::new());
            }
            lines.push(prelude);
        }
        lines.push(PRELUDE_END.to_owned());
        lines.push(String::new());
    }
    if layout_envelope {
        let tail = format!("\n{}\n{body}", lines[1..].join("\n"));
        let original = serde_json::to_string(source)
            .expect("source text serializes")
            .replace('\u{2028}', "\\u2028")
            .replace('\u{2029}', "\\u2029");
        write!(
            lines[0],
            " envelope-sha256={} original={original}",
            sha256(&tail)
        )
        .expect("writing a string succeeds");
    }
    let translation = SelfTranslation {
        source_language: from,
        target_language: to,
        code: format!("{}\n{body}", lines.join("\n")),
        items: recorded,
    };
    Ok((translation, signatures))
}

// The layout between two emitted blocks: its line breaks, at least one.
fn breaks(gap: &str) -> String {
    "\n".repeat(gap.matches('\n').count().max(1))
}

/// The top-level items of `text`, with their text and the layout after each.
fn top_level_items<'a>(text: &'a str, language: &str) -> Vec<Item<'a>> {
    let bytes = text.as_bytes();
    let nodes: Vec<(String, usize, usize)> =
        crate::tree_sitter_adapter::top_level_nodes(text, language)
            .unwrap_or_default()
            .into_iter()
            .filter(|(term, _, _)| term != "whitespace")
            .map(|(term, start, mut end)| {
                // A Rust line comment ends with its line break, which is layout here.
                if COMMENTS.contains(&term.as_str()) && end > 0 && bytes[end - 1] == b'\n' {
                    end -= if end > 1 && bytes[end - 2] == b'\r' {
                        2
                    } else {
                        1
                    };
                }
                (term, start, end)
            })
            .collect();
    nodes
        .iter()
        .enumerate()
        .map(|(index, (term, start, end))| Item {
            term: term.clone(),
            start: *start,
            end: *end,
            comment: COMMENTS.contains(&term.as_str()),
            text: &text[*start..*end],
            after: &text[*end..nodes.get(index + 1).map_or(text.len(), |next| next.1)],
        })
        .collect()
}

fn line_break(layout: &str) -> bool {
    layout == "\n" || layout == "\r\n"
}

fn source_lines(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let rest = &item.text[SOURCE_LINE.len()..];
            rest.strip_prefix(' ').unwrap_or(rest).to_owned()
        })
        .collect()
}

fn is_marker(item: &Item<'_>) -> bool {
    item.comment
        && [HEADER, CARRIED, TRANSLATED, PRELUDE_BEGIN]
            .iter()
            .any(|marker| item.text.starts_with(marker))
}

#[derive(Clone, Debug)]
enum GroupKind {
    Provenance,
    Carried {
        language: String,
        lines: Vec<String>,
    },
    Comment {
        text: String,
        term: String,
    },
    Item {
        text: String,
        term: String,
    },
}

#[derive(Clone)]
struct Group<'a> {
    kind: GroupKind,
    items: Vec<Item<'a>>,
    after: &'a str,
}

/// Groups the items: a self-translation header, a prelude block, a carried or
/// translated item with its provenance, and an item with the comments directly
/// before it are one group each.
fn group_items<'a>(items: &[Item<'a>], text: &'a str) -> Vec<Group<'a>> {
    let mut groups = Vec::new();
    let between = |first: &Item<'_>, last: &Item<'_>| text[first.start..last.end].to_owned();
    // The `// |` source lines that follow the item at `at` line by line.
    let lines_after = |at: usize| {
        let mut end = at;
        while end + 1 < items.len()
            && items[end + 1].comment
            && items[end + 1].text.starts_with(SOURCE_LINE)
            && line_break(items[end].after)
        {
            end += 1;
        }
        end
    };
    let mut index = 0;
    while index < items.len() {
        let item = &items[index];
        let (end, kind) = if item.comment && item.text.starts_with(HEADER) {
            (index, GroupKind::Provenance)
        } else if item.comment && item.text == PRELUDE_BEGIN {
            let mut end = index;
            while end + 1 < items.len() && items[end].text != PRELUDE_END {
                end += 1;
            }
            (end, GroupKind::Provenance)
        } else if item.comment && item.text.starts_with(CARRIED) {
            let end = lines_after(index);
            let language = first_word(&item.text[CARRIED.len()..]);
            let lines = source_lines(&items[index + 1..=end]);
            (end, GroupKind::Carried { language, lines })
        } else if item.comment && item.text.starts_with(TRANSLATED) {
            let rest = &item.text[TRANSLATED.len()..];
            let field = |name: &str| {
                rest.split(' ').skip(2).find_map(|field| {
                    let (key, value) = field.split_once('=').unwrap_or((field, ""));
                    (key == name).then_some(value)
                })
            };
            let lines_end = lines_after(index);
            let count = field("items").and_then(|count| count.parse::<usize>().ok());
            if count == Some(0) && field("sha256") == Some(sha256("").as_str()) {
                let lines = source_lines(&items[index + 1..=lines_end]);
                groups.push(Group {
                    kind: GroupKind::Carried {
                        language: first_word(rest),
                        lines,
                    },
                    items: items[index..=lines_end].to_vec(),
                    after: items[lines_end].after,
                });
                index = lines_end + 1;
                continue;
            }
            count
                .and_then(|count| definitions_end(items, lines_end, count))
                .map_or_else(
                    || {
                        (
                            index,
                            GroupKind::Comment {
                                text: item.text.to_owned(),
                                term: item.term.clone(),
                            },
                        )
                    },
                    |last| {
                        let code = between(&items[lines_end + 1], &items[last]);
                        if field("sha256") == Some(sha256(&code).as_str()) {
                            let lines = source_lines(&items[index + 1..=lines_end]);
                            (
                                last,
                                GroupKind::Carried {
                                    language: first_word(rest),
                                    lines,
                                },
                            )
                        } else {
                            // An edited translation is translated again; its provenance is dropped.
                            (lines_end, GroupKind::Provenance)
                        }
                    },
                )
        } else {
            // Comments directly before an item document it and travel with it.
            let mut end = index;
            while items[end].comment
                && end + 1 < items.len()
                && line_break(items[end].after)
                && !is_marker(&items[end + 1])
            {
                end += 1;
            }
            let text = between(item, &items[end]);
            let term = items[end].term.clone();
            // A run of comments no item follows is one comment group.
            if items[end].comment {
                (end, GroupKind::Comment { text, term })
            } else {
                (end, GroupKind::Item { text, term })
            }
        };
        groups.push(Group {
            kind,
            items: items[index..=end].to_vec(),
            after: items[end].after,
        });
        index = end + 1;
    }
    groups
}

/// The last item of the `count` definitions after the item at `after`: a
/// definition's attributes (`#[derive(…)]`) are items of their own in Rust.
fn definitions_end(items: &[Item<'_>], after: usize, count: usize) -> Option<usize> {
    if count == 0 {
        return None;
    }
    let mut left = count;
    for (at, item) in items.iter().enumerate().skip(after + 1) {
        if item.term != "attribute_item" && !item.comment {
            left -= 1;
        }
        if left == 0 {
            return Some(at);
        }
    }
    None
}

fn first_word(text: &str) -> String {
    text.split(' ').next().unwrap_or_default().to_owned()
}

#[derive(Clone)]
struct Translated {
    code: Option<String>,
    preludes: Vec<String>,
    status: &'static str,
    reason: Option<String>,
    diagnostic: Option<crate::translation::diagnostics::TranslationError>,
    /// The signatures a translated item gives the other items of its module.
    signatures: Vec<External>,
}
