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
    /// Serialize the whole network to links-notation text and read it back.
    LinoSerialization,
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
            ParityCapability::TriviaPreservation,
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
            ParityCapability::ErrorRecovery,
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
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
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
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
            ParityCapability::LinoSerialization,
        ],
        test_plan: "Executable fixtures cover doublet, triplet, N-tuple, indented, nested self-reference, and whole-network serialization LiNo behavior.",
    },
    ParityTarget {
        name: "link-cli",
        upstream: "https://github.com/link-foundation/link-cli",
        capabilities: &[ParityCapability::TransformBySubstitution],
        test_plan: "Executable fixtures cover create, update, delete, and swap substitution behavior.",
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
        test_plan: "Executable fixtures cover actual data/seed/*.lino and data/benchmarks/*.lino corpus sources plus cross-language reconstruction behavior.",
    },
    ParityTarget {
        name: "meta-expression",
        upstream: "https://github.com/link-assistant/meta-expression",
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
        test_plan: "Executable fixtures cover Hawaii naturalization, 1 + 1 formalization, self-reference behavior, and the verified 351-concept lexicon.",
    },
];

/// Expected verification result for an executable parity fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParityVerificationExpectation {
    /// Fixture should parse without recovery diagnostics.
    Clean,
    /// Fixture should round-trip while exposing recovery diagnostics.
    Recoverable,
}

/// Text transform expectation attached to a query/transform parity fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParityTransformExpectation {
    pub(crate) query: &'static str,
    pub(crate) capture_name: &'static str,
    pub(crate) replacement: &'static str,
    pub(crate) expected_output: &'static str,
}

impl ParityTransformExpectation {
    /// Query used to select the links to replace.
    #[must_use]
    pub const fn query(&self) -> &'static str {
        self.query
    }

    /// Capture name whose source text should be replaced.
    #[must_use]
    pub const fn capture_name(&self) -> &'static str {
        self.capture_name
    }

    /// Replacement source text.
    #[must_use]
    pub const fn replacement(&self) -> &'static str {
        self.replacement
    }

    /// Expected reconstructed source after applying the transform.
    #[must_use]
    pub const fn expected_output(&self) -> &'static str {
        self.expected_output
    }
}

/// Executable source fixture tied to a parity target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParityFixture {
    pub(crate) target_name: &'static str,
    pub(crate) name: &'static str,
    pub(crate) language: &'static str,
    pub(crate) source: &'static str,
    pub(crate) expected_reconstruction: &'static str,
    pub(crate) provenance: &'static str,
    pub(crate) verification_expectation: ParityVerificationExpectation,
    pub(crate) transform_expectation: Option<ParityTransformExpectation>,
    pub(crate) capabilities: &'static [ParityCapability],
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

    /// Upstream source path and license for this ported fixture.
    #[must_use]
    pub const fn provenance(&self) -> &'static str {
        self.provenance
    }

    /// Expected verification result after parsing this fixture.
    #[must_use]
    pub const fn verification_expectation(&self) -> ParityVerificationExpectation {
        self.verification_expectation
    }

    /// Optional query/transform assertion for this fixture.
    #[must_use]
    pub const fn transform_expectation(&self) -> Option<ParityTransformExpectation> {
        self.transform_expectation
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

pub use crate::parity_fixtures::PARITY_FIXTURES;

/// Family of language coverage targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageFamily {
    /// Markup or document container languages.
    Markup,
    /// Programming languages.
    Programming,
    /// Natural languages.
    Natural,
    /// Data-exchange / interchange formats.
    DataFormat,
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

/// Required data-exchange / interchange format targets.
///
/// Each entry has a wired tree-sitter grammar in `src/tree_sitter_adapter.rs`
/// and a round-trip [`LANGUAGE_FIXTURES`] entry. CSV and JSON5 are intentionally
/// absent: their crates.io grammar bindings still pin `tree-sitter ~0.20` and
/// remain incompatible with the project's tree-sitter front end as published.
/// See `docs/parity-roadmap.md` for the explicit deferral.
pub const DATA_FORMAT_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "JSON",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "YAML",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "TOML",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "XML",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "INI",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "protobuf",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-3 data-exchange format target",
    },
    LanguageTarget {
        name: "GraphQL",
        family: LanguageFamily::DataFormat,
        basis: "Issue #47 R-4 schema/interface-definition target",
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

/// Second-tier programming-language parser targets immediately below the TIOBE
/// top ten.
///
/// Each entry has a wired tree-sitter grammar in `src/tree_sitter_adapter.rs`
/// and a round-trip [`LANGUAGE_FIXTURES`] entry.
pub const SECOND_TIER_PROGRAMMING_LANGUAGE_TARGETS: &[LanguageTarget] = &[
    LanguageTarget {
        name: "PHP",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10",
    },
    LanguageTarget {
        name: "Swift",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10",
    },
    LanguageTarget {
        name: "Kotlin",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10",
    },
    LanguageTarget {
        name: "Scala",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10",
    },
    LanguageTarget {
        name: "Lua",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10",
    },
    LanguageTarget {
        name: "Perl",
        family: LanguageFamily::Programming,
        basis: "Issue #47 R-2 popular language immediately below the TIOBE top 10; issue #70 deferred Perl grammar follow-up",
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
        source: "unit DemoUnit;\n\ninterface\n\ntype\n  TBox<T> = class\n  private\n    FValue: T;\n  public\n    [Stored]\n    property Value: T read FValue write FValue;\n  end;\n\nimplementation\n\nend.\n",
        description: "Delphi/Object Pascal unit with a generic class property",
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
    LanguageFixture {
        language: "JSON",
        source: "{\n  \"name\": \"café\",\n  \"items\": [1, 2, 3]\n}\n",
        description: "JSON object with UTF-8 string and array",
    },
    LanguageFixture {
        language: "YAML",
        source: "name: café\nitems:\n  - 1\n  - 2\n",
        description: "YAML mapping with UTF-8 value and sequence",
    },
    LanguageFixture {
        language: "TOML",
        source: "title = \"café\"\n\n[owner]\nname = \"Tom\"\n",
        description: "TOML document with a UTF-8 value and a table",
    },
    LanguageFixture {
        language: "XML",
        source: "<note lang=\"en\">\n  <body>café</body>\n</note>\n",
        description: "XML element tree with attribute and UTF-8 text",
    },
    LanguageFixture {
        language: "INI",
        source: "; comment\n[owner]\nname = café\n",
        description: "INI section with a comment and a UTF-8 value",
    },
    LanguageFixture {
        language: "protobuf",
        source: "syntax = \"proto3\";\n\nmessage Person {\n  string name = 1;\n}\n",
        description: "Protocol Buffers message definition",
    },
    LanguageFixture {
        language: "GraphQL",
        source: "type Person {\n  name: String!\n}\n",
        description: "GraphQL schema-definition type",
    },
    LanguageFixture {
        language: "PHP",
        source: "<?php\nfunction greet($name) {\n    return \"café \" . $name;\n}\n",
        description: "PHP function returning a UTF-8 string",
    },
    LanguageFixture {
        language: "Swift",
        source: "func greet(_ name: String) -> String {\n    return \"café \\(name)\"\n}\n",
        description: "Swift function with a UTF-8 interpolated string",
    },
    LanguageFixture {
        language: "Kotlin",
        source: "fun greet(name: String): String {\n    return \"café $name\"\n}\n",
        description: "Kotlin function with a UTF-8 template string",
    },
    LanguageFixture {
        language: "Scala",
        source: "object Demo {\n  def greet(name: String): String = s\"café $name\"\n}\n",
        description: "Scala object with a UTF-8 interpolated method",
    },
    LanguageFixture {
        language: "Lua",
        source: "local function greet(name)\n  return \"café \" .. name\nend\n",
        description: "Lua function concatenating a UTF-8 string",
    },
    LanguageFixture {
        language: "Perl",
        source: "use utf8;\nsub greet {\n    my ($name) = @_;\n    return \"café $name\";\n}\n",
        description: "Perl subroutine returning a UTF-8 interpolated string",
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
