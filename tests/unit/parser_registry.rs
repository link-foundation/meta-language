use std::sync::Arc;

use meta_language::{LanguageParser, LinkNetwork, ParseConfiguration, ParserRegistry};

/// A custom parser that emits a single named link carrying the whole source as
/// its term, tagged with the requested language. It deliberately avoids the
/// built-in dispatch so the test can observe that the registry routed to it.
#[derive(Debug)]
struct WholeTextParser;

impl LanguageParser for WholeTextParser {
    fn parse_source(
        &self,
        text: &str,
        _language: &str,
        _configuration: ParseConfiguration,
    ) -> LinkNetwork {
        let mut network = LinkNetwork::new();
        network.insert_object(text);
        network
    }
}

/// A parser that uppercases the source before producing a lossless network, so
/// shadowing an existing key produces an observably different reconstruction.
#[derive(Debug)]
struct ShoutParser;

impl LanguageParser for ShoutParser {
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        LinkNetwork::parse_lossless_text(&text.to_uppercase(), language, configuration)
    }
}

#[test]
fn empty_registry_matches_built_in_parse() {
    let registry = ParserRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);

    let source = "(alpha (beta))";
    let configuration = ParseConfiguration::default();

    let registry_network = registry.parse(source, "lino", configuration);
    let built_in_network = LinkNetwork::parse(source, "lino", configuration);

    assert_eq!(registry_network, built_in_network);
    assert_eq!(registry_network.reconstruct_text(), source);
}

#[test]
fn registers_parser_for_new_language_key() {
    let registry = ParserRegistry::new().with_parser("whole-text", Arc::new(WholeTextParser));

    assert!(registry.is_registered("whole-text"));
    assert_eq!(registry.len(), 1);

    let configuration = ParseConfiguration::default();
    let network = registry.parse("the whole source", "whole-text", configuration);

    assert_eq!(network.len(), 1);
    assert!(network.find_term("the whole source").is_some());

    // The built-in path is untouched by the new key: parsing without the
    // registry still goes through lossless tokenization.
    let built_in = LinkNetwork::parse("the whole source", "whole-text", configuration);
    assert!(built_in.len() > 1);
    assert!(built_in.find_term("the whole source").is_none());
}

#[test]
fn user_registration_shadows_existing_key() {
    let source = "hello";
    let configuration = ParseConfiguration::default();

    // Built-in dispatch reconstructs the source verbatim.
    let built_in = LinkNetwork::parse(source, "plain-text", configuration);
    assert_eq!(built_in.reconstruct_text(), "hello");

    // Shadowing the same key overrides dispatch with the custom parser.
    let registry = ParserRegistry::new().with_parser("plain-text", Arc::new(ShoutParser));
    let shadowed = registry.parse(source, "plain-text", configuration);
    assert_eq!(shadowed.reconstruct_text(), "HELLO");

    // Other keys still fall through to the built-in set unchanged.
    let other = registry.parse(source, "lino", configuration);
    assert_eq!(other, LinkNetwork::parse(source, "lino", configuration));
}

#[test]
fn language_keys_match_case_insensitively() {
    let registry = ParserRegistry::new().with_parser("Shout", Arc::new(ShoutParser));

    assert!(registry.is_registered("SHOUT"));
    assert!(registry.parser_for("shout").is_some());

    let configuration = ParseConfiguration::default();
    let network = registry.parse("abc", "sHoUt", configuration);
    assert_eq!(network.reconstruct_text(), "ABC");
}

#[test]
fn link_network_entry_point_honors_registry() {
    let registry = ParserRegistry::new().with_parser("shout", Arc::new(ShoutParser));
    let configuration = ParseConfiguration::default();

    let network = LinkNetwork::parse_with_registry(&registry, "abc", "shout", configuration);
    assert_eq!(network.reconstruct_text(), "ABC");
}

#[test]
fn register_returns_self_for_chaining() {
    let mut registry = ParserRegistry::new();
    registry
        .register("whole-text", Arc::new(WholeTextParser))
        .register("shout", Arc::new(ShoutParser));

    assert_eq!(registry.len(), 2);
    assert!(registry.is_registered("whole-text"));
    assert!(registry.is_registered("shout"));
}
