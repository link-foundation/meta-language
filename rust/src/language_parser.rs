use crate::{
    docx_parser, language_catalog, lino_parser, pdf_parser, structured_text_parser,
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

        LinkNetwork::parse_lossless_text(text, language, configuration)
    }
}

fn parse_builtin_grammar(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> Option<LinkNetwork> {
    let entry = language_catalog::language_entry(language)?;
    match entry.name.as_str() {
        "LiNo" => Some(lino_parser::parse(text, language, configuration)),
        "txt" => Some(structured_text_parser::parse_plain(
            text,
            language,
            configuration,
        )),
        "PDF" => Some(pdf_parser::parse(text, language, configuration)),
        "DOCX" => Some(docx_parser::parse(text, language, configuration)),
        _ if entry.family == "natural" => Some(structured_text_parser::parse_natural(
            text,
            language,
            configuration,
        )),
        _ => None,
    }
}
