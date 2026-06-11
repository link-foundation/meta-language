use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::language_parser::{BuiltInLanguageParser, LanguageParser};
use crate::{LinkNetwork, ParseConfiguration};

/// A pluggable dispatch table mapping language keys to [`LanguageParser`]s.
///
/// The registry starts with the [`BuiltInLanguageParser`] as a fallback, so an
/// unmodified registry behaves exactly like [`LinkNetwork::parse`]: `lino` is
/// handled by the links-notation parser and every other key is routed through
/// the tree-sitter adapter with a lossless text fallback.
///
/// Users register parsers for new language keys or override an existing one.
/// Following TXL's grammar-override model and SWC's plugin lesson, a user
/// registration *shadows* the built-in dispatch for the same key rather than
/// forking the pipeline: any key without an explicit registration still falls
/// through to the built-in set.
///
/// Language keys are matched case-insensitively, mirroring the built-in
/// dispatch (`LINO`, `Lino`, and `lino` resolve to the same parser).
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use meta_language::{
///     LanguageParser, LinkNetwork, ParseConfiguration, ParserRegistry,
/// };
///
/// #[derive(Debug)]
/// struct ShoutParser;
///
/// impl LanguageParser for ShoutParser {
///     fn parse_source(
///         &self,
///         text: &str,
///         language: &str,
///         configuration: ParseConfiguration,
///     ) -> LinkNetwork {
///         LinkNetwork::parse_lossless_text(&text.to_uppercase(), language, configuration)
///     }
/// }
///
/// let registry = ParserRegistry::new().with_parser("shout", Arc::new(ShoutParser));
/// let network = registry.parse("hi", "shout", ParseConfiguration::default());
/// assert_eq!(network.reconstruct_text(), "HI");
/// ```
#[derive(Clone)]
pub struct ParserRegistry {
    parsers: HashMap<String, Arc<dyn LanguageParser>>,
    fallback: Arc<dyn LanguageParser>,
}

impl ParserRegistry {
    /// Creates a registry backed by the [`BuiltInLanguageParser`] fallback and
    /// no user registrations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
            fallback: Arc::new(BuiltInLanguageParser),
        }
    }

    /// Registers `parser` for `language`, shadowing any prior registration or
    /// built-in dispatch for the same (case-insensitive) key.
    ///
    /// Returns `&mut Self` so registrations can be chained.
    pub fn register(
        &mut self,
        language: impl Into<String>,
        parser: Arc<dyn LanguageParser>,
    ) -> &mut Self {
        let key: String = language.into();
        self.parsers.insert(normalize(&key), parser);
        self
    }

    /// Builder-style variant of [`register`](Self::register) that consumes and
    /// returns the registry.
    #[must_use]
    pub fn with_parser(
        mut self,
        language: impl Into<String>,
        parser: Arc<dyn LanguageParser>,
    ) -> Self {
        self.register(language, parser);
        self
    }

    /// Returns the parser explicitly registered for `language`, if any.
    ///
    /// Keys served by the built-in fallback report `None`; use
    /// [`parse`](Self::parse) to dispatch including the fallback.
    #[must_use]
    pub fn parser_for(&self, language: &str) -> Option<&Arc<dyn LanguageParser>> {
        self.parsers.get(&normalize(language))
    }

    /// Whether `language` has an explicit (non-fallback) registration.
    #[must_use]
    pub fn is_registered(&self, language: &str) -> bool {
        self.parsers.contains_key(&normalize(language))
    }

    /// The number of explicit registrations, excluding the built-in fallback.
    #[must_use]
    pub fn len(&self) -> usize {
        self.parsers.len()
    }

    /// Whether the registry holds no explicit registrations.
    ///
    /// An empty registry still parses every key through the built-in fallback.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parsers.is_empty()
    }

    /// Parses `text` for `language`, dispatching to a registered parser when
    /// one shadows the key and otherwise to the built-in fallback.
    #[must_use]
    pub fn parse(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        self.parsers.get(&normalize(language)).map_or_else(
            || self.fallback.parse_source(text, language, configuration),
            |parser| parser.parse_source(text, language, configuration),
        )
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkNetwork {
    /// Parses source text through a pluggable [`ParserRegistry`].
    ///
    /// Dispatch honors user registrations, which shadow the built-in set for
    /// the same language key; keys without an explicit registration fall
    /// through to the same built-in dispatch [`LinkNetwork::parse`] uses.
    #[must_use]
    pub fn parse_with_registry(
        registry: &ParserRegistry,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> Self {
        registry.parse(text, language, configuration)
    }
}

impl fmt::Debug for ParserRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<&str> = self.parsers.keys().map(String::as_str).collect();
        keys.sort_unstable();
        f.debug_struct("ParserRegistry")
            .field("registered", &keys)
            .finish_non_exhaustive()
    }
}

/// Normalizes a language key for case-insensitive lookup, matching the
/// built-in dispatch's `eq_ignore_ascii_case` handling.
fn normalize(language: &str) -> String {
    language.to_ascii_lowercase()
}
