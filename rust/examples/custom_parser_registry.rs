//! Registering a custom language parser and dispatching through the registry.
//!
//! Run with `cargo run --example custom_parser_registry`.
//!
//! The [`ParserRegistry`] maps language keys to [`LanguageParser`]s. An
//! unmodified registry behaves exactly like [`LinkNetwork::parse`]; user
//! registrations shadow the built-in dispatch for the same key, while every
//! other key still falls through to the built-in set.

use std::sync::Arc;

use meta_language::{LanguageParser, LinkNetwork, ParseConfiguration, ParserRegistry};

/// A minimal custom parser for a hypothetical `shout` language: it uppercases
/// the source before producing a lossless links network. Real parsers emit
/// richer link structure, but the boundary contract is the same — return a
/// [`LinkNetwork`] for the supplied text.
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

fn main() {
    let configuration = ParseConfiguration::default();

    // Register the custom parser for a brand-new language key.
    let registry = ParserRegistry::new().with_parser("shout", Arc::new(ShoutParser));

    let shouted = registry.parse("hello world", "shout", configuration);
    println!("shout -> {:?}", shouted.reconstruct_text());

    // Keys without a registration fall through to the built-in dispatch, so the
    // same registry parses `lino` exactly as `LinkNetwork::parse` would.
    let lino = registry.parse("(alpha (beta))", "lino", configuration);
    println!("lino  -> {:?}", lino.reconstruct_text());

    // The convenience entry point on `LinkNetwork` honors the registry too.
    let via_network = LinkNetwork::parse_with_registry(&registry, "again", "shout", configuration);
    println!("entry -> {:?}", via_network.reconstruct_text());
}
