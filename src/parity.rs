/// Capability tracked for comparison with existing language tooling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParityCapability {
    /// Preserve every byte of source text and source metadata.
    LosslessParsing,
    /// Preserve and explicitly attach trivia.
    TriviaPreservation,
    /// Produce recoverable error and missing links for invalid input.
    ErrorRecovery,
    /// Parse mixed-language documents as one links network.
    MixedLanguageRegions,
    /// Match syntax patterns with captures and predicates.
    QueryMatching,
    /// Transform the network with match-and-substitute rules.
    TransformBySubstitution,
    /// Reconstruct text in the same language without losing unchanged regions.
    SameLanguageReconstruction,
    /// Reconstruct text in a different language through shared concepts.
    CrossLanguageReconstruction,
    /// Round-trip ordinary host objects through LiNo-compatible links.
    ObjectRoundTrip,
    /// Evaluate type, meaning, and truth-value links.
    SemanticEvaluation,
    /// Formalize and deformalize text through shared meaning links.
    FormalizationRoundTrip,
    /// Describe the meta language with links in the same network.
    SelfDescription,
}

/// Upstream project whose feature set and tests should be tracked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParityTarget {
    name: &'static str,
    upstream: &'static str,
    capabilities: &'static [ParityCapability],
    test_plan: &'static str,
}

impl ParityTarget {
    /// Project name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Upstream project URL.
    #[must_use]
    pub const fn upstream(&self) -> &'static str {
        self.upstream
    }

    /// Capabilities tracked for parity.
    #[must_use]
    pub const fn capabilities(&self) -> &'static [ParityCapability] {
        self.capabilities
    }

    /// Test adoption plan for this project.
    #[must_use]
    pub const fn test_plan(&self) -> &'static str {
        self.test_plan
    }
}

/// Competitor and ecosystem projects called out by the founding issue.
pub const PARITY_TARGETS: &[ParityTarget] = &[
    ParityTarget {
        name: "tree-sitter",
        upstream: "https://github.com/tree-sitter/tree-sitter",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
            ParityCapability::MixedLanguageRegions,
            ParityCapability::QueryMatching,
        ],
        test_plan: "Port representative concrete syntax, injection, query, and recovery fixtures.",
    },
    ParityTarget {
        name: "LibCST",
        upstream: "https://github.com/Instagram/LibCST",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port Python parse, metadata, transform, and round-trip fixtures.",
    },
    ParityTarget {
        name: "Recast",
        upstream: "https://github.com/benjamn/recast",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port JavaScript/TypeScript parse-print preservation fixtures.",
    },
    ParityTarget {
        name: "jscodeshift",
        upstream: "https://github.com/facebook/jscodeshift",
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port transform fixtures as substitution-rule parity cases.",
    },
    ParityTarget {
        name: "Rowan",
        upstream: "https://github.com/rust-analyzer/rowan",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port green/red syntax and trivia preservation fixtures as links-network cases.",
    },
    ParityTarget {
        name: "cstree",
        upstream: "https://github.com/domenicquirl/cstree",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port Rust concrete syntax and checkpoint fixtures.",
    },
    ParityTarget {
        name: "Roslyn",
        upstream: "https://github.com/dotnet/roslyn",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::ErrorRecovery,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Port C# syntax, trivia, diagnostic, and formatter fixtures.",
    },
    ParityTarget {
        name: "links-notation",
        upstream: "https://github.com/link-foundation/links-notation",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ObjectRoundTrip,
            ParityCapability::SelfDescription,
        ],
        test_plan: "Port doublet, triplet, N-tuple, and indented LiNo fixtures.",
    },
    ParityTarget {
        name: "link-cli",
        upstream: "https://github.com/link-foundation/link-cli",
        capabilities: &[ParityCapability::TransformBySubstitution],
        test_plan: "Port create, update, delete, swap, trigger, and dedup substitution fixtures.",
    },
    ParityTarget {
        name: "lino-objects-codec",
        upstream: "https://github.com/link-foundation/lino-objects-codec",
        capabilities: &[ParityCapability::ObjectRoundTrip],
        test_plan:
            "Port encode/decode, identity, shared-reference, and circular-reference fixtures.",
    },
    ParityTarget {
        name: "relative-meta-logic",
        upstream: "https://github.com/link-foundation/relative-meta-logic",
        capabilities: &[
            ParityCapability::SemanticEvaluation,
            ParityCapability::SelfDescription,
        ],
        test_plan: "Port dependent-type, many-valued, probabilistic, and paradox fixtures.",
    },
    ParityTarget {
        name: "formal-ai",
        upstream: "https://github.com/link-assistant/formal-ai",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
            ParityCapability::CrossLanguageReconstruction,
        ],
        test_plan: "Replay the formal-ai corpus as a parity gate.",
    },
    ParityTarget {
        name: "meta-expression",
        upstream: "https://github.com/link-assistant/meta-expression",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
        test_plan: "Port formalize, semantic-link, naturalize, span, and self-reference fixtures.",
    },
];

/// Family of language coverage targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageFamily {
    /// Markup or document container languages.
    Markup,
    /// Programming languages.
    Programming,
    /// Natural languages.
    Natural,
}

/// Language whose grammar or natural-language parser should be supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LanguageTarget {
    name: &'static str,
    family: LanguageFamily,
    basis: &'static str,
}

impl LanguageTarget {
    /// Language name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Language family.
    #[must_use]
    pub const fn family(&self) -> LanguageFamily {
        self.family
    }

    /// Basis for including this target.
    #[must_use]
    pub const fn basis(&self) -> &'static str {
        self.basis
    }
}

/// Required document-container languages.
pub const MARKUP_LANGUAGE_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "Markdown",
        family: LanguageFamily::Markup,
        basis: "Founding issue full-document target",
    },
    LanguageTarget {
        name: "HTML",
        family: LanguageFamily::Markup,
        basis: "Founding issue full-document target",
    },
];

/// Initial top-ten programming-language parser targets.
pub const PROGRAMMING_LANGUAGE_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "Python",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "C",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "Java",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "C++",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "C#",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "JavaScript",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "Visual Basic",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "R",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "SQL",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
    LanguageTarget {
        name: "Delphi/Object Pascal",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
];

/// Initial top-ten natural-language parser targets by total speakers.
pub const NATURAL_LANGUAGE_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "English",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Mandarin Chinese",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Hindi",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Spanish",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "French",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Modern Standard Arabic",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Bengali",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Russian",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Portuguese",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Urdu",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
];

/// Mixed-grammar embedding case that must become one unified links network.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrammarEmbeddingTarget {
    host_language: &'static str,
    embedded_language: &'static str,
    trigger: &'static str,
}

impl GrammarEmbeddingTarget {
    /// Host language containing the embedded region.
    #[must_use]
    pub const fn host_language(&self) -> &'static str {
        self.host_language
    }

    /// Embedded region language or language family.
    #[must_use]
    pub const fn embedded_language(&self) -> &'static str {
        self.embedded_language
    }

    /// Detection trigger or boundary.
    #[must_use]
    pub const fn trigger(&self) -> &'static str {
        self.trigger
    }
}

/// Initial mixed-grammar coverage targets.
pub const GRAMMAR_EMBEDDING_TARGETS: &[GrammarEmbeddingTarget] = &[
    GrammarEmbeddingTarget {
        host_language: "Markdown",
        embedded_language: "Programming language region",
        trigger: "fenced code language tag",
    },
    GrammarEmbeddingTarget {
        host_language: "Markdown",
        embedded_language: "HTML",
        trigger: "inline or block HTML",
    },
    GrammarEmbeddingTarget {
        host_language: "HTML",
        embedded_language: "JavaScript",
        trigger: "script element",
    },
    GrammarEmbeddingTarget {
        host_language: "HTML",
        embedded_language: "CSS",
        trigger: "style element or style attribute",
    },
];
