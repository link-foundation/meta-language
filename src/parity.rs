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
    /// Capture immutable snapshots, edit mutable forks, and commit versions.
    SnapshotVersioning,
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
        test_plan: "Executable fixture covers concrete syntax, injection, query, and recovery behavior.",
    },
    ParityTarget {
        name: "LibCST",
        upstream: "https://github.com/Instagram/LibCST",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Executable fixture covers Python parse, metadata, transform, and round-trip behavior.",
    },
    ParityTarget {
        name: "Recast",
        upstream: "https://github.com/benjamn/recast",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Executable fixture covers JavaScript/TypeScript parse-print preservation behavior.",
    },
    ParityTarget {
        name: "jscodeshift",
        upstream: "https://github.com/facebook/jscodeshift",
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
        test_plan: "Executable fixture covers substitution-rule transform behavior.",
    },
    ParityTarget {
        name: "Rowan",
        upstream: "https://github.com/rust-analyzer/rowan",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
        test_plan: "Executable fixture covers persistent syntax and trivia preservation behavior.",
    },
    ParityTarget {
        name: "cstree",
        upstream: "https://github.com/domenicquirl/cstree",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
        test_plan: "Executable fixture covers Rust concrete syntax and checkpoint behavior.",
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
        test_plan: "Executable fixture covers C# syntax, trivia, diagnostics, and formatting behavior.",
    },
    ParityTarget {
        name: "links-notation",
        upstream: "https://github.com/link-foundation/links-notation",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ObjectRoundTrip,
            ParityCapability::SelfDescription,
        ],
        test_plan: "Executable fixture covers doublet, triplet, N-tuple, and indented LiNo behavior.",
    },
    ParityTarget {
        name: "link-cli",
        upstream: "https://github.com/link-foundation/link-cli",
        capabilities: &[ParityCapability::TransformBySubstitution],
        test_plan: "Executable fixture covers create, update, delete, swap, trigger, and dedup substitution behavior.",
    },
    ParityTarget {
        name: "lino-objects-codec",
        upstream: "https://github.com/link-foundation/lino-objects-codec",
        capabilities: &[ParityCapability::ObjectRoundTrip],
        test_plan: "Executable fixture covers encode/decode, identity, shared-reference, and circular-reference behavior.",
    },
    ParityTarget {
        name: "relative-meta-logic",
        upstream: "https://github.com/link-foundation/relative-meta-logic",
        capabilities: &[
            ParityCapability::SemanticEvaluation,
            ParityCapability::SelfDescription,
        ],
        test_plan: "Executable fixture covers dependent-type, many-valued, probabilistic, and paradox behavior.",
    },
    ParityTarget {
        name: "formal-ai",
        upstream: "https://github.com/link-assistant/formal-ai",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
            ParityCapability::CrossLanguageReconstruction,
        ],
        test_plan: "Executable fixture covers formalization corpus and cross-language reconstruction behavior.",
    },
    ParityTarget {
        name: "meta-expression",
        upstream: "https://github.com/link-assistant/meta-expression",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
        test_plan: "Executable fixture covers formalize, semantic-link, naturalize, span, and self-reference behavior.",
    },
];

/// Executable source fixture tied to a parity target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParityFixture {
    target_name: &'static str,
    name: &'static str,
    language: &'static str,
    source: &'static str,
    expected_reconstruction: &'static str,
    capabilities: &'static [ParityCapability],
}

impl ParityFixture {
    /// Target project name.
    #[must_use]
    pub const fn target_name(&self) -> &'static str {
        self.target_name
    }

    /// Fixture name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Source language label used by the parser.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// Source text exercised by the fixture.
    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    /// Expected lossless reconstruction.
    #[must_use]
    pub const fn expected_reconstruction(&self) -> &'static str {
        self.expected_reconstruction
    }

    /// Capabilities exercised by this fixture.
    #[must_use]
    pub const fn capabilities(&self) -> &'static [ParityCapability] {
        self.capabilities
    }

    /// Parity target for this fixture.
    #[must_use]
    pub fn target(&self) -> &'static ParityTarget {
        PARITY_TARGETS
            .iter()
            .find(|target| target.name() == self.target_name)
            .expect("parity fixture target must exist")
    }
}

/// Executable fixtures for every parity target called out by the founding issue.
pub const PARITY_FIXTURES: &[ParityFixture] = &[
    ParityFixture {
        target_name: "tree-sitter",
        name: "markdown fenced rust with queryable tokens",
        language: "Markdown",
        source: "Intro\n```rust\nfn main() {}\n```\n",
        expected_reconstruction: "Intro\n```rust\nfn main() {}\n```\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
            ParityCapability::MixedLanguageRegions,
            ParityCapability::QueryMatching,
        ],
    },
    ParityFixture {
        target_name: "LibCST",
        name: "python round trip with indentation",
        language: "Python",
        source: "def f(x):\n    return x + 1\n",
        expected_reconstruction: "def f(x):\n    return x + 1\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    ParityFixture {
        target_name: "Recast",
        name: "javascript comment preservation",
        language: "JavaScript",
        source: "const value = 1; // keep trivia\n",
        expected_reconstruction: "const value = 1; // keep trivia\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    ParityFixture {
        target_name: "jscodeshift",
        name: "javascript transform input",
        language: "JavaScript",
        source: "const oldName = call(oldName);\n",
        expected_reconstruction: "const oldName = call(oldName);\n",
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    ParityFixture {
        target_name: "Rowan",
        name: "rust trivia preservation",
        language: "Rust",
        source: "fn main() {\n    // keep\n}\n",
        expected_reconstruction: "fn main() {\n    // keep\n}\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
    },
    ParityFixture {
        target_name: "cstree",
        name: "rust checkpoint source",
        language: "Rust",
        source: "let checkpoint = value + 1;\n",
        expected_reconstruction: "let checkpoint = value + 1;\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
    },
    ParityFixture {
        target_name: "Roslyn",
        name: "csharp diagnostic recovery source",
        language: "C#",
        source: "class C { void M() { Console.WriteLine(1); } }\n",
        expected_reconstruction: "class C { void M() { Console.WriteLine(1); } }\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::ErrorRecovery,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    ParityFixture {
        target_name: "links-notation",
        name: "lino tuple forms",
        language: "LiNo",
        source: "(lovesMama: loves mama)\npapa has car\n",
        expected_reconstruction: "(lovesMama: loves mama)\npapa has car\n",
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ObjectRoundTrip,
            ParityCapability::SelfDescription,
        ],
    },
    ParityFixture {
        target_name: "link-cli",
        name: "substitution patterns",
        language: "LiNo",
        source: "((1: 1 1)) ((1: 1 2))\n",
        expected_reconstruction: "((1: 1 1)) ((1: 1 2))\n",
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    ParityFixture {
        target_name: "lino-objects-codec",
        name: "shared and circular references",
        language: "LiNo",
        source: "(object: object object)\n(shared: object object)\n",
        expected_reconstruction: "(object: object object)\n(shared: object object)\n",
        capabilities: &[ParityCapability::ObjectRoundTrip],
    },
    ParityFixture {
        target_name: "relative-meta-logic",
        name: "dependent type and paradox source",
        language: "RML",
        source: "(Type: Type Type)\n(this_statement_is_false)\n",
        expected_reconstruction: "(Type: Type Type)\n(this_statement_is_false)\n",
        capabilities: &[
            ParityCapability::SemanticEvaluation,
            ParityCapability::SelfDescription,
        ],
    },
    ParityFixture {
        target_name: "formal-ai",
        name: "formalization reconstruction source",
        language: "English",
        source: "Hawaii is a state.\n",
        expected_reconstruction: "Hawaii is a state.\n",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
            ParityCapability::CrossLanguageReconstruction,
        ],
    },
    ParityFixture {
        target_name: "meta-expression",
        name: "naturalization span source",
        language: "English",
        source: "1 + 1 = 2\n",
        expected_reconstruction: "1 + 1 = 2\n",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
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

/// Executable lossless parse fixture for a required language target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LanguageFixture {
    language: &'static str,
    source: &'static str,
    description: &'static str,
}

impl LanguageFixture {
    /// Language label used by the parser.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// Source text that must parse and reconstruct losslessly.
    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    /// Behavior represented by this fixture.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }
}

/// Required document-container languages.
pub const MARKUP_LANGUAGE_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "txt",
        family: LanguageFamily::Markup,
        basis: "Issue #5 degenerate plain-text container target",
    },
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
        name: "sql-ansi",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10 SQL family baseline dialect",
    },
    LanguageTarget {
        name: "Delphi/Object Pascal",
        family: LanguageFamily::Programming,
        basis: "TIOBE May 2026 top 10",
    },
];

/// Initial top-ten natural-language parser targets in Ethnologue 2025 total-speaker order.
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
        name: "Modern Standard Arabic",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "French",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Bengali",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Portuguese",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Russian",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
    LanguageTarget {
        name: "Urdu",
        family: LanguageFamily::Natural,
        basis: "Ethnologue/Britannica total-speaker top 10",
    },
];

/// Executable fixtures for every language target requested by the founding issue.
pub const LANGUAGE_FIXTURES: &[LanguageFixture] = &[
    LanguageFixture {
        language: "txt",
        source: "Plain text region\ncafe au lait\nUTF-8 line: café\n",
        description: "Plain-text UTF-8 prose with trailing newline",
    },
    LanguageFixture {
        language: "Markdown",
        source: "# Title\n\n```rust\nfn main() {}\n```\n",
        description: "Markdown document with embedded fenced code",
    },
    LanguageFixture {
        language: "HTML",
        source: "<script>const x = 1;</script><style>.x { color: red; }</style><p style=\"color: blue\">text</p>\n",
        description: "HTML document with script, style, and style-attribute regions",
    },
    LanguageFixture {
        language: "Python",
        source: "def f(x):\n    return x + 1\n",
        description: "Python function with indentation",
    },
    LanguageFixture {
        language: "C",
        source: "int main(void) { return 0; }\n",
        description: "C entry point",
    },
    LanguageFixture {
        language: "Java",
        source: "class Main { public static void main(String[] args) {} }\n",
        description: "Java class entry point",
    },
    LanguageFixture {
        language: "C++",
        source: "int main() { return 0; }\n",
        description: "C++ entry point",
    },
    LanguageFixture {
        language: "C#",
        source: "class C { static void Main() {} }\n",
        description: "C# class entry point",
    },
    LanguageFixture {
        language: "JavaScript",
        source: "const value = 1;\n",
        description: "JavaScript binding",
    },
    LanguageFixture {
        language: "Visual Basic",
        source: "Module Program\nEnd Module\n",
        description: "Visual Basic module",
    },
    LanguageFixture {
        language: "R",
        source: "value <- 1\n",
        description: "R assignment",
    },
    LanguageFixture {
        language: "sql-ansi",
        source: "SELECT id, name FROM users WHERE active = TRUE;\n",
        description: "ANSI SQL select statement",
    },
    LanguageFixture {
        language: "Delphi/Object Pascal",
        source: "program Demo;\nbegin\nend.\n",
        description: "Delphi/Object Pascal program",
    },
    LanguageFixture {
        language: "English",
        source: "Hawaii is a state.\n",
        description: "English formalization sentence",
    },
    LanguageFixture {
        language: "Mandarin Chinese",
        source: "你好。\n",
        description: "Mandarin Chinese sentence",
    },
    LanguageFixture {
        language: "Hindi",
        source: "नमस्ते।\n",
        description: "Hindi sentence",
    },
    LanguageFixture {
        language: "Spanish",
        source: "Hawaii es un estado.\n",
        description: "Spanish reconstruction sentence",
    },
    LanguageFixture {
        language: "French",
        source: "Hawaii est un etat.\n",
        description: "French reconstruction sentence",
    },
    LanguageFixture {
        language: "Modern Standard Arabic",
        source: "مرحبا.\n",
        description: "Modern Standard Arabic sentence",
    },
    LanguageFixture {
        language: "Bengali",
        source: "নমস্কার।\n",
        description: "Bengali sentence",
    },
    LanguageFixture {
        language: "Russian",
        source: "Гавайи это штат.\n",
        description: "Russian reconstruction sentence",
    },
    LanguageFixture {
        language: "Portuguese",
        source: "Hawaii e um estado.\n",
        description: "Portuguese reconstruction sentence",
    },
    LanguageFixture {
        language: "Urdu",
        source: "سلام۔\n",
        description: "Urdu sentence",
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
