pub mod access;
pub mod api_styles;
pub mod benchmark;
mod binary_format;
mod builtin_grammar;
mod concept_distinctions;
mod concept_ontology;
mod concept_records;
pub mod configuration;
pub mod document_formatting;
mod docx_parser;
mod embedded_region_parser;
mod foundation_models;
pub mod grammar;
pub mod graphql_adapter;
mod incremental;
pub mod language_catalog;
mod language_fixtures;
mod language_identification;
pub mod language_parser;
pub mod language_profile;
pub mod language_support;
mod line_index;
pub mod link_flags;
pub mod link_network;
mod lino_grammar;
mod lino_parser;
pub mod lino_serialization;
pub mod mixed_regions;
mod natural_language;
mod natural_language_grammar;
mod network_projection;
pub mod parity;
mod parity_fixtures;
pub mod parser_registry;
mod pdf_grammar;
mod pdf_parser;
pub mod program_representation;
pub mod program_translation;
pub mod query;
pub mod query_algebra;
pub mod query_plan;
mod reconstruction;
pub mod rust_codec;
mod semantic_translation;
pub mod semantics;
pub mod snapshots;
pub mod source;
mod source_generation;
pub mod sql_adapter;
pub mod storage;
mod structured_text_parser;
pub mod substitution;
pub mod transform;
pub mod translation;
pub mod translation_rules;
pub mod verification;

pub use access::{EngineNetwork, ReadOnlyNetwork, ReadOnlyViolation};
pub use api_styles::{
    API_OPERATIONS, ApiOperation, ApiOperationEntry, ApiStyle, ApiStyleCell, ApiStyleFixture,
    FluentNetworkApi, FluentPipeline, LinkCliSubstitution, LinkCliSubstitutionError,
    LinkCliSubstitutionKind, run_api_style_fixture,
};
pub use concept_distinctions::{
    ConceptCorrespondence, ConceptDistinctionProblem, CorrespondenceRelation, PrecedenceUse,
    REQUIRED_CONCEPT_DISTINCTIONS, REQUIRED_FOUNDATION_DISTINCTIONS, RequiredDistinction,
    check_concept_distinctions, concept_correspondence, concept_correspondence_in,
    grammar_precedence_concepts, source_meanings_in,
};
pub use concept_ontology::{
    ConceptOntologyImportReport, ConceptOntologySeedReport, ConceptRecordSeedReport,
    FORMER_CONCEPT_ID_VOCABULARY, FORMER_CONCEPT_IDS, current_concept_id,
};
pub use concept_records::{
    ConceptDistinction, ConceptRecord, ConceptRole, SourceAlias, concept_record, concept_records,
    concept_records_for_source_name,
};
pub use configuration::{
    AccessMode, FormalizationLevel, LanguageIdentificationDetector, NaturalizationDirection,
    ParseConfiguration, RegionDetectionPolicy, TriviaAttachmentPolicy,
};
pub use document_formatting::{
    BlockNode, CROSS_FORMAT_CONCEPTS, DOCUMENT_FORMATS, DocumentFormatInstance,
    DocumentFormatMatch, DocumentFormattingSeedReport, FormattingDocument, InlineNode,
    canonical_document_format, document_format_profile, docx_package_is_recognized,
    docx_profile_is_recognized, parse_docx_document, parse_docx_package, parse_markup_document,
    parse_pdf_document, pdf_profile_is_recognized, render_docx_document, render_docx_package,
    render_pdf_document,
};
pub use foundation_models::{
    CORRESPONDENCE_KINDS, FoundationCorrespondence, FoundationDistinction, FoundationFamily,
    FoundationModel, FoundationProblem, FoundationRegister, LOGIC_FAMILIES, LanguageFoundation,
    check_foundation_models, foundation_correspondences, foundation_model, foundation_register,
    language_foundation_models,
};
pub use grammar::{
    ActiveLearningConfig, ActiveLearningError, ActiveLearningOracle, ActiveSymbol, AdviceDecision,
    AdviceDecisionKind, AdviceSource, BenchmarkReport, ByteSpan, CharCategory, CharClassItem,
    ConceptNamingAdvisor, ConstraintAtom, ConstraintClause, ConstraintPattern, Delimiter, Dfa,
    DiagnosticKind, EmitReport, EvalError, ExprBuilder, FallbackAdvisor, GOLDEN_CORPORA,
    GRAMMAR_CONCEPTS, GRAMMAR_CONSTRUCTS, GRAMMAR_FORMATS, GoldenCorpus, Grammar,
    GrammarAcceptorOracle, GrammarBuilder, GrammarConcept, GrammarDiagnostic, GrammarEmitError,
    GrammarExpr, GrammarFidelityLevel, GrammarFormat, GrammarFormatProfile, GrammarImportError,
    GrammarOracle, GrammarParser, GrammarRule, GrammarSurfaceError, GrammarTranslateError,
    InferenceOptions, InferenceReport, InferenceResult, InferredAutomaton, JsParserArtifacts,
    LeafKind, LengthUnit, LexicalConfig, LexicalModel, Mdl, MdlMergeAdvisor, MembershipOracle,
    MergeAdvisor, MergeCandidate, MergeRequest, MergeScore, MergeStrategy, MetricScores,
    MinimizeOptions, MinimizeReport, MinimizeResult, NameCandidate, NamingAdvisor, NamingRequest,
    NonTerminalRef, Oracle, ParserAcceptancePredicate, ParserMembershipOracle, PositiveOnlyOracle,
    PriorOptions, RuleKind, RuleSpan, RustParserArtifacts, Sample, SampleConfig,
    SamplingEquivalenceOracle, ScoringMode, SeedNode, SeedTree, SemanticConstraint,
    SemanticInferenceConfig, Severity, StructuralPrior, Symbol, Token, WhitespacePolicy,
    annotate_grammar_concepts, build_structural_prior, canonical_grammar_format, categorise,
    clean_structural_acceptance, default_pattern_catalog, emit_abnf, emit_antlr, emit_bnf,
    emit_ebnf, emit_gbnf, emit_javascript_parser, emit_lark, emit_peggy, emit_pest,
    emit_rust_parser, emit_tree_sitter_grammar_js, emit_tree_sitter_grammar_js_with_report,
    emit_tree_sitter_json, evaluate, evaluate_atom, evaluate_clause, evaluate_constraint,
    evaluate_probabilistic, grammar_concept_translation_rules, grammar_expr_concept_id,
    grammar_format_profile, grammar_from_lino, grammar_to_lino, import_abnf, import_antlr,
    import_bnf, import_ebnf, import_gbnf, import_lark, import_pest, import_tree_sitter_json,
    infer_cfg, infer_cfg_with_advisors, infer_dfa, infer_lexical_classes, learn_dfa, learn_grammar,
    mdl, mdl_cost, mine_semantic_constraints, minimize, parse_grammar_surface, register_grammar,
    render_rust_type, rule_concept_id, run_corpus, run_named_corpus, run_sequitur, sample,
    size_symbols, translate_grammar_surface, validate, with_grammar, write_grammar_surface,
};
pub use grammar::{FORMER_GRAMMAR_CONSTRUCTS, current_grammar_construct};
pub use grammar::{
    GRAMMAR_COMMAND_USAGE, GRAMMAR_EXPORT_FORMATS, GRAMMAR_IMPORT_FORMATS,
    GRAMMAR_LOSSLESS_FORMATS, GrammarCommandOutput, GrammarEmitter, GrammarFileReader,
    GrammarImporter, GrammarLayout, GrammarLayoutDefinition, GrammarLayoutImplicit,
    GrammarLosslessError, GrammarSourceDefinition, GrammarSourceSplit, capture_grammar_layout,
    emit_grammar_lossless, grammar_emitter, grammar_importer, import_grammar_lossless,
    parse_grammar_layout_links, parse_grammar_links, parse_links_expression, parse_native_grammar,
    percent_decode_links_text, percent_encode_links_text, render_grammar_layout_links,
    render_grammar_links, render_links_expression, render_native_expression, render_native_grammar,
    render_rule_link, run_grammar_command, split_grammar_source,
};
pub use grammar::{
    GRAMMAR_MERGE_METHOD, GrammarMergeAlternative, GrammarMergeAlternativeReason,
    GrammarMergeDecision, GrammarMergeDecisionKind, GrammarMergeError, GrammarMergeFailure,
    GrammarMergeFailureKind, GrammarMergeFailureReason, GrammarMergeNomination,
    GrammarMergeNominationBasis, GrammarMergeNominationOutcome, GrammarMergeOptions,
    GrammarMergeResult, GrammarMergeSource, GrammarRenameError, GrammarRenameErrorKind,
    MergedGrammarGroup, RenamedGrammar, RuleAlias, assert_merge_complete, merge_grammars,
    normalized_rule_definition, rename_grammar_rule, restore_source_names,
};
pub use grammar::{
    GRAMMAR_ROUND_TRIP_MARKER, GrammarEmitFn, GrammarImportFn, GrammarRoundTrip,
    GrammarRoundTripError, GrammarRoundTripFailure, GrammarRoundTripFailureKind,
    GrammarRoundTripReport, GrammarRoundTripStage, GrammarRoundTripStatus,
    check_grammar_round_trip, mutate_grammar_start_rule,
};
pub use grammar::{
    GrammarReverseConversion, GrammarReverseFailure, GrammarReverseFailureKind,
    GrammarReverseReport, GrammarReverseStage, GrammarReverseStatus,
    check_grammar_reverse_conversion,
};
#[cfg(feature = "llm-assist")]
pub use grammar::{LlmClient, LlmError, LlmMergeAdvisor, LlmNamingAdvisor};
pub use graphql_adapter::{
    GraphQlAdapterError, GraphQlArgumentRole, GraphQlOperationType, GraphQlRootMapping,
    GraphQlSchemaRegistry, lower_graphql,
};
pub use language_catalog::{
    GrammarProvenance, LanguageEntry, canonical_language_name, grammar_provenance,
    language_candidates_for_path, language_catalog, language_entry, language_for_path,
};
pub use language_identification::identify_language;
pub use language_parser::{BuiltInLanguageParser, LanguageParser};
pub use language_profile::{LanguageProfile, LanguageProfileLinks, LanguageProfileViolation};
pub use language_support::{
    FOUR_LANGUAGE_SUPPORT, LANGUAGE_REPRESENTATION_SCHEMA_VERSION, LanguageSupport,
    RepresentationLevel, TranslationContract, TranslationSupport, language_support,
    translation_contract, translation_contracts,
};
pub use link_flags::LinkFlags;
pub use link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType, NetworkProjection};
pub use lino_parser::LinksNotationReading;
pub use lino_serialization::LinoSerializationError;
pub use mixed_regions::{EmbeddedRegion, script_language};
pub use natural_language_grammar::{
    NATURAL_LANGUAGE_GRAMMAR_FIXTURES, NaturalLanguageGrammarFixture,
};
pub use parity::{
    DATA_FORMAT_TARGETS, GRAMMAR_EMBEDDING_TARGETS, GrammarEmbeddingTarget, LANGUAGE_FIXTURES,
    LanguageFamily, LanguageFixture, LanguageTarget, MARKUP_LANGUAGE_TARGETS,
    NATURAL_LANGUAGE_TARGETS, PARITY_FIXTURES, PARITY_TARGETS, PROGRAMMING_LANGUAGE_TARGETS,
    ParityCapability, ParityFixture, ParityTarget, ParityTransformExpectation,
    ParityVerificationExpectation, SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS,
};
pub use parser_registry::ParserRegistry;
pub use program_representation::{
    PROGRAM_REPRESENTATION_SCHEMA_VERSION, PROGRAM_SNAPSHOT_SCHEMA_VERSION, ProgramBinding,
    ProgramConstruct, ProgramConstructStatus, ProgramDiagnostic, ProgramExpansion, ProgramFact,
    ProgramProjectContext, ProgramProjectModule, ProgramProjectReference, ProgramProjectSource,
    ProgramRange, ProgramRepresentation, ProgramRepresentationError, ProgramScope,
    ProgramSourceMapping, SEMANTIC_CONSTRUCTS, analyze_program, construct_program,
    construct_program_from_fragments,
};
pub use program_translation::{
    DecodedProgramTranslation, ProgramTranslation, ProgramTranslationError,
    decode_program_translation, read_translation_provenance, translate_program,
};
pub use query::{
    LinkQuery, QueryCapture, QueryCaptures, QueryMatch, QueryParseError, QueryPredicate,
    QueryPredicateArgument, QueryPredicateHost,
};
pub use query_algebra::{
    LinkRule, LinkRuleCapture, LinkRuleCaptures, LinkRuleMatch, LinkRuleParseError,
    LinkRuleRegistry, LinkRuleSnapshotCase, LinkRuleSnapshotExpectation, LinkRuleSnapshotReport,
    LinkRuleSnapshotResult, LinkRuleSnapshotSuite, TraversalReport, TraversalStrategy,
};
pub use query_plan::{
    LoweredQueryPlan, QUERY_PLAN_VERSION, QueryAggregate, QueryAggregateFunction,
    QueryAuthorization, QueryComparisonOperator, QueryFilter, QueryOperation, QueryOrder,
    QueryPlan, QuerySortDirection, QuerySourceEvidence, QueryValue,
};
pub use rust_codec::{
    FromLinks, LinksCodecError, LinksDecoder, LinksEncoder, LinksObject, RustFieldShape,
    RustTypeKind, RustTypeShape, ToLinks,
};
pub use semantic_translation::{
    ReadTranslationProvenance, SEMANTIC_ENCODING, SEMANTIC_OBSERVATION, SemanticTranslation,
    TranslationDiagnostic, TranslationObligation, TranslationProvenance,
};
pub use semantics::{ProbabilisticTruthValue, Probability, TruthValue};
pub use snapshots::{MutableNetworkSnapshot, NetworkSnapshot, StructuralDiff};
pub use source::{ByteRange, Point, SourceSpan};
pub use sql_adapter::{
    SQL_DIALECT_PROFILES, SqlAdapterError, SqlAdapterErrorKind, SqlDialectProfile,
    SqlRelationMapping, SqlSchemaRegistry, lower_sql, lower_sql_cst,
};
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
pub use tree_sitter_adapter::{GrammarNames, grammar_names};
pub use verification::{VerificationIssue, VerificationIssueKind, VerificationReport};

mod self_description;
mod tree_sitter_adapter;
