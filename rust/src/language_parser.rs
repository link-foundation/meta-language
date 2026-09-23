use crate::{
    data_format_parser, docx_parser, formal_language_parser, lino_parser, pdf_parser,
    tree_sitter_adapter, LinkNetwork, ParseConfiguration,
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

impl LanguageParser for BuiltInLanguageParser {
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        if language.eq_ignore_ascii_case("lino") {
            return lino_parser::parse(text, language, configuration);
        }

        if language.eq_ignore_ascii_case("pdf") {
            return pdf_parser::parse(text, language, configuration);
        }

        if language.eq_ignore_ascii_case("docx") {
            return docx_parser::parse(text, language, configuration);
        }

        if let Some(network) = data_format_parser::parse(text, language, configuration) {
            return network;
        }

        // Lean has a complete tree-sitter frontend. Keep Rocq on the portable
        // recovery parser until an ABI-compatible Rust grammar is available.
        if language.eq_ignore_ascii_case("lean") || language.eq_ignore_ascii_case("lean4") {
            if let Some(network) = tree_sitter_adapter::parse(text, language, configuration) {
                return network;
            }
        }

        if let Some(network) = formal_language_parser::parse(text, language, configuration) {
            return network;
        }

        tree_sitter_adapter::parse(text, language, configuration)
            .unwrap_or_else(|| LinkNetwork::parse_lossless_text(text, language, configuration))
    }
}
