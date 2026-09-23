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
