pub mod configuration;
pub mod language_parser;
pub mod link_flags;
pub mod link_network;
pub mod mixed_regions;
mod natural_language;
pub mod parity;
pub mod query;
pub mod semantics;
pub mod snapshots;
pub mod source;
pub mod substitution;
pub mod verification;

pub use configuration::{
    LanguageIdentificationDetector, ParseConfiguration, RegionDetectionPolicy,
    TriviaAttachmentPolicy,
};
pub use language_parser::{BuiltInLanguageParser, LanguageParser};
pub use link_flags::LinkFlags;
pub use link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType, NetworkProjection};
pub use mixed_regions::EmbeddedRegion;
pub use parity::{
    GrammarEmbeddingTarget, LanguageFamily, LanguageFixture, LanguageTarget, ParityCapability,
    ParityFixture, ParityTarget, GRAMMAR_EMBEDDING_TARGETS, LANGUAGE_FIXTURES,
    MARKUP_LANGUAGE_TARGETS, NATURAL_LANGUAGE_TARGETS, PARITY_FIXTURES, PARITY_TARGETS,
    PROGRAMMING_LANGUAGE_TARGETS,
};
pub use query::{
    LinkQuery, QueryCapture, QueryCaptures, QueryMatch, QueryParseError, QueryPredicate,
    QueryPredicateArgument, QueryPredicateHost,
};
pub use semantics::TruthValue;
pub use snapshots::{MutableNetworkSnapshot, NetworkSnapshot};
pub use source::{ByteRange, Point, SourceSpan};
pub use substitution::{
    SubstitutionBindings, SubstitutionReport, SubstitutionRule, SubstitutionValue,
    VariableSubstitutionRule,
};
pub use verification::{VerificationIssue, VerificationIssueKind, VerificationReport};

mod tree_sitter_adapter;
