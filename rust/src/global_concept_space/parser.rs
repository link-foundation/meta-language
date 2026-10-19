//! Archive-only grammar resolution delegates to the ordinary grammar runtime.

use std::collections::BTreeMap;

use super::links::read_validated_global_concept_archive;
use super::{GlobalConceptOptions, GlobalConceptSource};
use crate::global_concept_identity_rules::contains_global_concept_identity;
use crate::grammar::feature_runtime::{
    FeatureGrammarParser, FeatureParseOptions, GrammarParseError, GrammarRuntimeError,
    ParseOutcome, ParseRejection, ParseWithGrammarError, SyntaxTree, compile_feature_grammar,
};

/// A parser whose grammar identity and resolver belong to its validated archive.
#[derive(Debug)]
pub struct GlobalConceptSpaceParser {
    parser: FeatureGrammarParser,
}

fn require_global_concept_parser_options(
    options: &FeatureParseOptions,
) -> Result<(), GrammarRuntimeError> {
    // Typed options express presence through Some; there is no resolver override.
    let name = if options.global_concept_space.is_some() {
        Some("globalConceptSpace")
    } else if options.grammar_name.is_some() {
        Some("grammarName")
    } else {
        None
    };
    if let Some(name) = name {
        return Err(GrammarRuntimeError {
            reason: "globalConceptSpace",
            message: format!("the archived global concept space owns parser option {name}"),
        });
    }
    Ok(())
}

const fn archive_error(message: String) -> GrammarRuntimeError {
    GrammarRuntimeError {
        reason: "globalConceptSpace",
        message,
    }
}

/// Validates a complete archive and compiles one exact archived source identity.
///
/// All imports and embeddings resolve from reconstructed archived grammars.
/// `settings.records` configures archive validation; parser limits and decorators
/// come from `options`. A supplied grammar name or concept space is refused.
///
/// # Errors
/// Returns the existing archive or compilation error, or a `selection` error
/// when `source_identifier` is not an exact archived source identifier.
pub fn create_global_concept_space_parser(
    source: &str,
    source_identifier: &str,
    settings: &GlobalConceptOptions,
    mut options: FeatureParseOptions,
) -> Result<GlobalConceptSpaceParser, GrammarRuntimeError> {
    require_global_concept_parser_options(&options)?;
    let archive = read_validated_global_concept_archive(source, settings).map_err(archive_error)?;
    let identities = archive
        .imported
        .iter()
        .map(|source| super::string(&source["id"]).map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()
        .map_err(archive_error)?;
    if !contains_global_concept_identity(&identities, source_identifier.to_owned()) {
        return Err(GrammarRuntimeError {
            reason: "selection",
            message: "an archived grammar needs an exact existing source identifier".to_owned(),
        });
    }
    let grammars = archive
        .imported
        .iter()
        .map(|source| {
            GlobalConceptSource::from_json(source).map(|source| (source.id, source.grammar))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
        .map_err(archive_error)?;
    let grammar = &grammars[source_identifier];
    let resolve = |name: &str| grammars.get(name).cloned();
    options.grammar_name = Some(source_identifier.to_owned());
    options.global_concept_space = Some(archive.space);
    let parser = compile_feature_grammar(grammar, Some(&resolve), options)?;
    Ok(GlobalConceptSpaceParser { parser })
}

impl GlobalConceptSpaceParser {
    /// Parses with the archive's identity, preserving ordinary CST and budgets.
    ///
    /// # Errors
    /// Returns a runtime error for protected option overrides or an invalid start.
    pub fn parse_tree(
        &self,
        source: &[u8],
        options: &FeatureParseOptions,
    ) -> Result<ParseOutcome, GrammarRuntimeError> {
        require_global_concept_parser_options(options)?;
        self.parser.parse_tree(source, options)
    }

    /// Parses into the concrete tree under the archive's identity.
    ///
    /// # Errors
    /// Returns `Runtime` for a protected option override or runtime failure,
    /// and `Parse` for an ordinary input rejection.
    pub fn parse(
        &self,
        source: &[u8],
        options: &FeatureParseOptions,
    ) -> Result<SyntaxTree, ParseWithGrammarError> {
        let outcome = self
            .parse_tree(source, options)
            .map_err(ParseWithGrammarError::Runtime)?;
        if outcome.ok && outcome.source_planes.is_some() {
            return Err(ParseWithGrammarError::Runtime(GrammarRuntimeError {
                reason: "selection",
                message: "a mapped grammar result needs parseTree to retain its sourcePlanes"
                    .to_owned(),
            }));
        }
        match (outcome.rejection, outcome.tree) {
            (None, Some(tree)) => Ok(tree),
            (rejection, _) => Err(ParseWithGrammarError::Parse(GrammarParseError {
                rejection: Box::new(rejection.unwrap_or(ParseRejection {
                    diagnostics: None,
                    offset_semantics: None,
                    reason: "syntax",
                    offset: Some(0),
                    line: Some(1),
                    column: Some(1),
                    expected: None,
                    limit: None,
                })),
            })),
        }
    }
}
