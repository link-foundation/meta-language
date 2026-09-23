use crate::{
    data_format_parser, docx_parser, formal_language_parser, lino_parser, natural_language,
    pdf_parser, structured_text_parser, tree_sitter_adapter, LinkNetwork, ParseConfiguration,
};

/// Parser boundary that produces lossless links networks for source text.
pub trait LanguageParser {
    /// Parses `text` using the requested language label.
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork;
}

/// Built-in parser registry used by [`LinkNetwork::parse`].
#[derive(Clone, Copy, Debug, Default)]
pub struct BuiltInLanguageParser;

// Explicitly audited alongside `tree_sitter_adapter::grammar_for_language` by
// the issue-195 manifest validator. Every alias here selects a structured
// parser through `parse_builtin_grammar` below.
const BUILT_IN_GRAMMAR_ALIASES: &[&str] = &[
    "lino",
    "txt",
    "text",
    "plain text",
    "pdf",
    "docx",
    "csv",
    "english",
    "en",
    "mandarin chinese",
    "chinese",
    "zh",
    "hindi",
    "hi",
    "spanish",
    "es",
    "modern standard arabic",
    "arabic",
    "ar",
    "french",
    "fr",
    "bengali",
    "bn",
    "portuguese",
    "pt",
    "russian",
    "ru",
    "urdu",
    "ur",
];

impl LanguageParser for BuiltInLanguageParser {
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        if let Some(network) = parse_builtin_grammar(text, language, configuration) {
            return network;
        }

        // Lean has a statically linked tree-sitter frontend. A build-time ABI
        // mismatch is an implementation error, not permission to silently
        // relabel its lexical fallback as a grammar CST.
        if language.eq_ignore_ascii_case("lean") || language.eq_ignore_ascii_case("lean4") {
            return tree_sitter_adapter::parse(text, language, configuration)
                .expect("the built-in Lean tree-sitter grammar must initialize");
        }

        if let Some(network) = tree_sitter_adapter::parse(text, language, configuration) {
            return network;
        }

        // Structured parsers without a tree-sitter grammar remain explicit
        // fallbacks. Keeping this after the grammar registry ensures JSON5 is
        // handled by its complete grammar while CSV retains its lossless
        // record/field parser.
        if let Some(network) = data_format_parser::parse(text, language, configuration) {
            return network;
        }

        // The portable recovery parser remains available for formal-language
        // aliases that do not yet have a registered grammar. Registered
        // grammar aliases above must never silently take this path.
        if let Some(network) = formal_language_parser::parse(text, language, configuration) {
            return network;
        }

        LinkNetwork::parse_lossless_text(text, language, configuration)
    }
}

fn parse_builtin_grammar(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> Option<LinkNetwork> {
    let normalized = language.to_ascii_lowercase();
    if !BUILT_IN_GRAMMAR_ALIASES.contains(&normalized.as_str()) {
        return None;
    }

    match normalized.as_str() {
        "lino" => Some(lino_parser::parse(text, language, configuration)),
        "txt" | "text" | "plain text" => Some(structured_text_parser::parse_plain(
            text,
            language,
            configuration,
        )),
        "pdf" => Some(pdf_parser::parse(text, language, configuration)),
        "docx" => Some(docx_parser::parse(text, language, configuration)),
        "csv" => data_format_parser::parse(text, language, configuration),
        _ if natural_language::canonical_natural_language(language).is_some() => Some(
            structured_text_parser::parse_natural(text, language, configuration),
        ),
        _ => None,
    }
}
