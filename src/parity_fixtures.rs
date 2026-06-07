use crate::parity::{
    ParityCapability, ParityFixture, ParityTransformExpectation, ParityVerificationExpectation,
};

pub const PARITY_FIXTURES: &[ParityFixture] = &[
    // Upstream: tree-sitter/tree-sitter test/fixtures/test_grammars/readme_grammar/corpus.txt; license: MIT.
    ParityFixture {
        target_name: "tree-sitter",
        name: "readme grammar expression cst",
        language: "JavaScript",
        source: "a + b * c\n",
        expected_reconstruction: "a + b * c\n",
        provenance:
            "tree-sitter/tree-sitter test/fixtures/test_grammars/readme_grammar/corpus.txt; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: tree-sitter/tree-sitter test/fixtures/test_grammars/extra_non_terminals/corpus.txt; license: MIT.
    ParityFixture {
        target_name: "tree-sitter",
        name: "extras and comment trivia",
        language: "JavaScript",
        source: "const a = 1; /* one */\n// e\nconst b = 2;\n",
        expected_reconstruction: "const a = 1; /* one */\n// e\nconst b = 2;\n",
        provenance:
            "tree-sitter/tree-sitter test/fixtures/test_grammars/extra_non_terminals/corpus.txt; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
        ],
    },
    // Upstream: tree-sitter/tree-sitter test/fixtures/error_corpus/javascript_errors.txt; license: MIT.
    ParityFixture {
        target_name: "tree-sitter",
        name: "javascript error corpus extra identifiers",
        language: "JavaScript",
        source: "if (a b) {\n  c d;\n}\ne f;\n",
        expected_reconstruction: "if (a b) {\n  c d;\n}\ne f;\n",
        provenance:
            "tree-sitter/tree-sitter test/fixtures/error_corpus/javascript_errors.txt; license: MIT",
        verification_expectation: ParityVerificationExpectation::Recoverable,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
        ],
    },
    // Upstream: tree-sitter/tree-sitter docs/src/using-parsers/queries/1-syntax.md; license: MIT.
    ParityFixture {
        target_name: "tree-sitter",
        name: "class identifier query source",
        language: "JavaScript",
        source: "class Person {\n  getName() { return name; }\n}\n",
        expected_reconstruction: "class Person {\n  getName() { return name; }\n}\n",
        provenance:
            "tree-sitter/tree-sitter docs/src/using-parsers/queries/1-syntax.md; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::QueryMatching],
    },
    // Upstream: tree-sitter/tree-sitter docs/src/3-syntax-highlighting.md; license: MIT.
    ParityFixture {
        target_name: "tree-sitter",
        name: "markdown fenced rust with queryable tokens",
        language: "Markdown",
        source: "Intro\n```rust\nfn main() {}\n```\n",
        expected_reconstruction: "Intro\n```rust\nfn main() {}\n```\n",
        provenance: "tree-sitter/tree-sitter docs/src/3-syntax-highlighting.md; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
            ParityCapability::MixedLanguageRegions,
            ParityCapability::QueryMatching,
        ],
    },
    // Upstream: Instagram/LibCST native/libcst/tests/fixtures/comments.py; license: MIT.
    ParityFixture {
        target_name: "LibCST",
        name: "python round trip with indentation and comments",
        language: "Python",
        source: "x = 1\n\n# Some comment before a function.\ndef function(default=None):\n    return default\n",
        expected_reconstruction:
            "x = 1\n\n# Some comment before a function.\ndef function(default=None):\n    return default\n",
        provenance: "Instagram/LibCST native/libcst/tests/fixtures/comments.py; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: Instagram/LibCST libcst/_nodes/tests/test_empty_line.py; license: MIT.
    ParityFixture {
        target_name: "LibCST",
        name: "empty line comment trivia",
        language: "Python",
        source: "# comment\n\nvalue = 1\n",
        expected_reconstruction: "# comment\n\nvalue = 1\n",
        provenance: "Instagram/LibCST libcst/_nodes/tests/test_empty_line.py; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
        ],
    },
    // Upstream: Instagram/LibCST libcst/_parser/tests/test_parse_errors.py; license: MIT.
    ParityFixture {
        target_name: "LibCST",
        name: "mismatched brace parse error",
        language: "Python",
        source: "abcd)",
        expected_reconstruction: "abcd)",
        provenance: "Instagram/LibCST libcst/_parser/tests/test_parse_errors.py; license: MIT",
        verification_expectation: ParityVerificationExpectation::Recoverable,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
        ],
    },
    // Upstream: Instagram/LibCST libcst/_nodes/tests/test_removal_behavior.py; license: MIT.
    ParityFixture {
        target_name: "LibCST",
        name: "python captured identifier transform",
        language: "Python",
        source: "old_name = call(old_name)\n",
        expected_reconstruction: "old_name = call(old_name)\n",
        provenance:
            "Instagram/LibCST libcst/_nodes/tests/test_removal_behavior.py; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: Some(ParityTransformExpectation {
            query: r#"
            (identifier) @target
            (#eq? @target "old_name")
            "#,
            capture_name: "target",
            replacement: "renamed",
            expected_output: "renamed = call(renamed)\n",
        }),
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: benjamn/recast test/identity.ts via test/data/regexp-props.js; license: MIT.
    ParityFixture {
        target_name: "Recast",
        name: "javascript comment preservation",
        language: "JavaScript",
        source: "const value = 1; // keep trivia\n",
        expected_reconstruction: "const value = 1; // keep trivia\n",
        provenance: "benjamn/recast test/identity.ts and test/data/regexp-props.js; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: benjamn/recast test/data/regexp-props.js; license: MIT.
    ParityFixture {
        target_name: "Recast",
        name: "regexp properties identity fixture",
        language: "JavaScript",
        source:
            "_.templateSettings = {\n    evaluate    : /<%([\\s\\S]+?)%>/g,\n    interpolate : /<%=([\\s\\S]+?)%>/g,\n    escape      : /<%-([\\s\\S]+?)%>/g // this line parsed oddly\n};\n",
        expected_reconstruction:
            "_.templateSettings = {\n    evaluate    : /<%([\\s\\S]+?)%>/g,\n    interpolate : /<%=([\\s\\S]+?)%>/g,\n    escape      : /<%-([\\s\\S]+?)%>/g // this line parsed oddly\n};\n",
        provenance: "benjamn/recast test/data/regexp-props.js; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: benjamn/recast test/data/empty.js; license: MIT.
    ParityFixture {
        target_name: "Recast",
        name: "empty program identity fixture",
        language: "JavaScript",
        source: "",
        expected_reconstruction: "",
        provenance: "benjamn/recast test/data/empty.js; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: facebook/jscodeshift README.md __testfixtures__ example; license: MIT.
    ParityFixture {
        target_name: "jscodeshift",
        name: "javascript transform input",
        language: "JavaScript",
        source: "const oldName = call(oldName);\n",
        expected_reconstruction: "const oldName = call(oldName);\n",
        provenance: "facebook/jscodeshift README.md __testfixtures__ example; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: Some(ParityTransformExpectation {
            query: r#"
            (identifier) @target
            (#eq? @target "oldName")
            "#,
            capture_name: "target",
            replacement: "newName",
            expected_output: "const newName = call(newName);\n",
        }),
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: facebook/jscodeshift sample/__testfixtures__/reverse-identifiers.input.js; license: MIT.
    ParityFixture {
        target_name: "jscodeshift",
        name: "reverse identifiers input fixture",
        language: "JavaScript",
        source:
            "var firstWord = 'Hello ';\nvar secondWord = 'world';\nvar message = firstWord + secondWord;\n",
        expected_reconstruction:
            "var firstWord = 'Hello ';\nvar secondWord = 'world';\nvar message = firstWord + secondWord;\n",
        provenance:
            "facebook/jscodeshift sample/__testfixtures__/reverse-identifiers.input.js; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: Some(ParityTransformExpectation {
            query: r#"
            (identifier) @target
            (#eq? @target "firstWord")
            "#,
            capture_name: "target",
            replacement: "droWtsrif",
            expected_output:
                "var droWtsrif = 'Hello ';\nvar secondWord = 'world';\nvar message = droWtsrif + secondWord;\n",
        }),
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: facebook/jscodeshift sample/__testfixtures__/reverse-identifiers.output.js; license: MIT.
    ParityFixture {
        target_name: "jscodeshift",
        name: "reverse identifiers output fixture",
        language: "JavaScript",
        source:
            "var droWtsrif = 'Hello ';\nvar droWdnoces = 'world';\nvar egassem = droWtsrif + droWdnoces;\n",
        expected_reconstruction:
            "var droWtsrif = 'Hello ';\nvar droWdnoces = 'world';\nvar egassem = droWtsrif + droWdnoces;\n",
        provenance:
            "facebook/jscodeshift sample/__testfixtures__/reverse-identifiers.output.js; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::SameLanguageReconstruction],
    },
    // Upstream: rust-analyzer/rowan examples/s_expressions.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "Rowan",
        name: "s-expression tutorial input",
        language: "txt",
        source: "(+ (* 15 2) 62)\n",
        expected_reconstruction: "(+ (* 15 2) 62)\n",
        provenance: "rust-analyzer/rowan examples/s_expressions.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: rust-analyzer/rowan examples/math.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "Rowan",
        name: "math checkpoint expression",
        language: "txt",
        source: "1 + 2 * 3 + 4\n",
        expected_reconstruction: "1 + 2 * 3 + 4\n",
        provenance: "rust-analyzer/rowan examples/math.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: rust-analyzer/rowan examples/s_expressions.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "Rowan",
        name: "rust trivia preservation",
        language: "Rust",
        source: "fn main() {\n    // keep\n}\n",
        expected_reconstruction: "fn main() {\n    // keep\n}\n",
        provenance: "rust-analyzer/rowan examples/s_expressions.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
    },
    // Upstream: domenicquirl/cstree cstree/src/getting_started.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "cstree",
        name: "calculator nested expression",
        language: "txt",
        source: "1 - (2 + 5)\n",
        expected_reconstruction: "1 - (2 + 5)\n",
        provenance:
            "domenicquirl/cstree cstree/src/getting_started.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: domenicquirl/cstree test_suite/tests/derive.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "cstree",
        name: "derive syntax kind fixture",
        language: "Rust",
        source:
            "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u32)]\nenum SyntaxKind { Int, Plus, Root }\n",
        expected_reconstruction:
            "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u32)]\nenum SyntaxKind { Int, Plus, Root }\n",
        provenance: "domenicquirl/cstree test_suite/tests/derive.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: domenicquirl/cstree cstree/src/getting_started.rs; license: Apache-2.0 OR MIT.
    ParityFixture {
        target_name: "cstree",
        name: "rust checkpoint source",
        language: "Rust",
        source: "let checkpoint = value + 1;\n",
        expected_reconstruction: "let checkpoint = value + 1;\n",
        provenance:
            "domenicquirl/cstree cstree/src/getting_started.rs; license: Apache-2.0 OR MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::SameLanguageReconstruction,
            ParityCapability::SnapshotVersioning,
        ],
    },
    // Upstream: dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs; license: MIT.
    ParityFixture {
        target_name: "Roslyn",
        name: "csharp diagnostic recovery source",
        language: "C#",
        source: "class C { void M() { Console.WriteLine(1); } }\n",
        expected_reconstruction: "class C { void M() { Console.WriteLine(1); } }\n",
        provenance:
            "dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
            ParityCapability::ErrorRecovery,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs TestGetNextTokenIncludingSkippedTokens; license: MIT.
    ParityFixture {
        target_name: "Roslyn",
        name: "skipped token recovery round trip",
        language: "C#",
        source: "garbage\nusing goo.bar;\n",
        expected_reconstruction: "garbage\nusing goo.bar;\n",
        provenance:
            "dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs; license: MIT",
        verification_expectation: ParityVerificationExpectation::Recoverable,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
        ],
    },
    // Upstream: dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxRewriterTests.cs TestReplaceTriviaShouldNotLoseParseOptions; license: MIT.
    ParityFixture {
        target_name: "Roslyn",
        name: "leading trivia preservation",
        language: "C#",
        source: "/* c */ class C { }\n",
        expected_reconstruction: "/* c */ class C { }\n",
        provenance:
            "dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxRewriterTests.cs; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::TriviaPreservation,
        ],
    },
    // Upstream: dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs TestReplaceSingleTokenWithMultipleTokens; license: MIT.
    ParityFixture {
        target_name: "Roslyn",
        name: "class identifier transform",
        language: "C#",
        source: "private class C { }\n",
        expected_reconstruction: "private class C { }\n",
        provenance:
            "dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs; license: MIT",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: Some(ParityTransformExpectation {
            query: r#"
            (identifier) @target
            (#eq? @target "C")
            "#,
            capture_name: "target",
            replacement: "D",
            expected_output: "private class D { }\n",
        }),
        capabilities: &[
            ParityCapability::QueryMatching,
            ParityCapability::TransformBySubstitution,
            ParityCapability::SameLanguageReconstruction,
        ],
    },
    // Upstream: dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs missing-token diagnostics; license: MIT.
    ParityFixture {
        target_name: "Roslyn",
        name: "missing token diagnostic recovery",
        language: "C#",
        source: "class C { void M() { if ( }",
        expected_reconstruction: "class C { void M() { if ( }",
        provenance:
            "dotnet/roslyn src/Compilers/CSharp/Test/Syntax/Syntax/SyntaxNodeTests.cs; license: MIT",
        verification_expectation: ParityVerificationExpectation::Recoverable,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ErrorRecovery,
        ],
    },
    // Upstream: link-foundation/links-notation LiNo tuple forms; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "lino tuple forms",
        language: "LiNo",
        source: "(lovesMama: loves mama)\npapa has car\n",
        expected_reconstruction: "(lovesMama: loves mama)\npapa has car\n",
        provenance: "link-foundation/links-notation tuple fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ObjectRoundTrip,
            ParityCapability::SelfDescription,
        ],
    },
    // Upstream: link-foundation/link-cli substitution examples; license: Unlicense.
    ParityFixture {
        target_name: "link-cli",
        name: "substitution patterns",
        language: "LiNo",
        source: "((1: 1 1)) ((1: 1 2))\n",
        expected_reconstruction: "((1: 1 1)) ((1: 1 2))\n",
        provenance: "link-foundation/link-cli substitution fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    // Upstream: link-foundation/lino-objects-codec object graph examples; license: Unlicense.
    ParityFixture {
        target_name: "lino-objects-codec",
        name: "shared and circular references",
        language: "LiNo",
        source: "(object: object object)\n(shared: object object)\n",
        expected_reconstruction: "(object: object object)\n(shared: object object)\n",
        provenance: "link-foundation/lino-objects-codec object fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::ObjectRoundTrip],
    },
    // Upstream: link-foundation/relative-meta-logic dependent-type examples; license: Unlicense.
    ParityFixture {
        target_name: "relative-meta-logic",
        name: "dependent type and paradox source",
        language: "RML",
        source: "(Type: Type Type)\n(this_statement_is_false)\n",
        expected_reconstruction: "(Type: Type Type)\n(this_statement_is_false)\n",
        provenance: "link-foundation/relative-meta-logic semantic fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::SemanticEvaluation,
            ParityCapability::SelfDescription,
        ],
    },
    // Upstream: link-assistant/formal-ai formalization corpus examples; license: Unlicense.
    ParityFixture {
        target_name: "formal-ai",
        name: "formalization reconstruction source",
        language: "English",
        source: "Hawaii is a state.\n",
        expected_reconstruction: "Hawaii is a state.\n",
        provenance: "link-assistant/formal-ai formalization fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
            ParityCapability::CrossLanguageReconstruction,
        ],
    },
    // Upstream: link-assistant/meta-expression naturalization span examples; license: Unlicense.
    ParityFixture {
        target_name: "meta-expression",
        name: "naturalization span source",
        language: "English",
        source: "1 + 1 = 2\n",
        expected_reconstruction: "1 + 1 = 2\n",
        provenance: "link-assistant/meta-expression naturalization fixtures; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
    },
];
