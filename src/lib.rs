pub mod access;
pub mod api_styles;
mod concept_ontology;
pub mod configuration;
mod data_format_parser;
pub mod document_formatting;
mod docx_parser;
mod embedded_region_parser;
pub mod grammar;
mod incremental;
mod language_fixtures;
pub mod language_parser;
pub mod language_profile;
pub mod link_flags;
pub mod link_network;
mod lino_parser;
pub mod lino_serialization;
pub mod mixed_regions;
mod natural_language;
mod natural_language_grammar;
pub mod parity;
mod parity_fixtures;
pub mod parser_registry;
mod pdf_parser;
pub mod query;
pub mod query_algebra;
mod reconstruction;
pub mod rust_codec;
pub mod semantics;
pub mod snapshots;
pub mod source;
mod source_generation;
pub mod storage;
pub mod substitution;
pub mod transform;
pub mod translation_rules;
pub mod verification;

pub use access::{EngineNetwork, ReadOnlyNetwork, ReadOnlyViolation};
pub use api_styles::{
    run_api_style_fixture, ApiOperation, ApiOperationEntry, ApiStyle, ApiStyleCell,
    ApiStyleFixture, FluentNetworkApi, FluentPipeline, LinkCliSubstitution,
    LinkCliSubstitutionError, LinkCliSubstitutionKind, API_OPERATIONS,
};
pub use concept_ontology::{ConceptOntologyImportReport, ConceptOntologySeedReport};
pub use configuration::{
    AccessMode, FormalizationLevel, LanguageIdentificationDetector, NaturalizationDirection,
    ParseConfiguration, RegionDetectionPolicy, TriviaAttachmentPolicy,
};
pub use document_formatting::{
    canonical_document_format, document_format_profile, docx_package_is_recognized,
    docx_profile_is_recognized, parse_docx_document, parse_docx_package, parse_markup_document,
    parse_pdf_document, pdf_profile_is_recognized, render_docx_document, render_docx_package,
    render_pdf_document, BlockNode, DocumentFormatInstance, DocumentFormatMatch,
    DocumentFormattingSeedReport, FormattingDocument, InlineNode, CROSS_FORMAT_CONCEPTS,
    DOCUMENT_FORMATS,
};
pub use grammar::{
    annotate_grammar_concepts, emit_abnf, emit_bnf, emit_ebnf, emit_gbnf, emit_pest,
    emit_rust_parser, grammar_expr_concept_id, grammar_from_lino, grammar_to_lino, import_abnf,
    import_antlr, import_bnf, import_ebnf, import_gbnf, import_lark, import_pest,
    import_tree_sitter_json, parse_grammar_surface, render_rust_type, rule_concept_id,
    write_grammar_surface, CharClassItem, EmitReport, ExprBuilder, Grammar, GrammarBuilder,
    GrammarConcept, GrammarEmitError, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule,
    GrammarSurfaceError, RuleKind, RustParserArtifacts, GRAMMAR_CONCEPTS,
};
pub use language_parser::{BuiltInLanguageParser, LanguageParser};
pub use language_profile::{LanguageProfile, LanguageProfileLinks, LanguageProfileViolation};
pub use link_flags::LinkFlags;
pub use link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType, NetworkProjection};
pub use lino_serialization::LinoSerializationError;
pub use mixed_regions::EmbeddedRegion;
pub use natural_language_grammar::{
    NaturalLanguageGrammarFixture, NATURAL_LANGUAGE_GRAMMAR_FIXTURES,
};
pub use parity::{
    GrammarEmbeddingTarget, LanguageFamily, LanguageFixture, LanguageTarget, ParityCapability,
    ParityFixture, ParityTarget, ParityTransformExpectation, ParityVerificationExpectation,
    DATA_FORMAT_TARGETS, GRAMMAR_EMBEDDING_TARGETS, LANGUAGE_FIXTURES, MARKUP_LANGUAGE_TARGETS,
    NATURAL_LANGUAGE_TARGETS, PARITY_FIXTURES, PARITY_TARGETS, PROGRAMMING_LANGUAGE_TARGETS,
    SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS,
};
pub use parser_registry::ParserRegistry;
pub use query::{
    LinkQuery, QueryCapture, QueryCaptures, QueryMatch, QueryParseError, QueryPredicate,
    QueryPredicateArgument, QueryPredicateHost,
};
pub use query_algebra::{
    LinkRule, LinkRuleCapture, LinkRuleCaptures, LinkRuleMatch, LinkRuleParseError,
    LinkRuleRegistry, LinkRuleSnapshotCase, LinkRuleSnapshotExpectation, LinkRuleSnapshotReport,
    LinkRuleSnapshotResult, LinkRuleSnapshotSuite, TraversalReport, TraversalStrategy,
};
pub use rust_codec::{
    FromLinks, LinksCodecError, LinksDecoder, LinksEncoder, LinksObject, RustFieldShape,
    RustTypeKind, RustTypeShape, ToLinks,
};
pub use semantics::{ProbabilisticTruthValue, Probability, TruthValue};
pub use snapshots::{MutableNetworkSnapshot, NetworkSnapshot, StructuralDiff};
pub use source::{ByteRange, Point, SourceSpan};
#[cfg(feature = "doublets")]
pub use storage::DoubletsLinkStore;
pub use storage::{EngineLinkStore, LinkStore, LinkStoreBackend, LinkStoreQuery, StorageError};
pub use substitution::{
    SubstitutionBindings, SubstitutionReport, SubstitutionRule, SubstitutionValue,
    VariableSubstitutionRule,
};
pub use transform::{
    QuasiquoteError, QuasiquoteTemplate, ReplacementReport, ReplacementRule,
    SourceTextPredicateHost, TextReplacement,
};
pub use translation_rules::{
    TranslationRule, TranslationRuleRegistry, TranslationRuleSet, TranslationRuleSetLoadError,
    TranslationTemplate,
};
pub use verification::{VerificationIssue, VerificationIssueKind, VerificationReport};

mod self_description;
mod tree_sitter_adapter;
