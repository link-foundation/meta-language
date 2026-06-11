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
    // Upstream: link-foundation/links-notation TEST_CASE_COMPARISON.md verifies cross-language counts as Python 137, JavaScript 138, Rust 138, C# 140; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "doublet link fixture",
        language: "LiNo",
        source: "(papa mama)\n",
        expected_reconstruction: "(papa mama)\n",
        provenance:
            "link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/SingleLineParserTests.cs and TEST_CASE_COMPARISON.md 137/138/138/140 tests; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/SingleLineParserTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "triplet single link fixture",
        language: "LiNo",
        source: "(papa loves mama)\n",
        expected_reconstruction: "(papa loves mama)\n",
        provenance:
            "link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/SingleLineParserTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/TupleTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "n-tuple tuple fixture",
        language: "LiNo",
        source: "(papa (lovesMama: loves mama))\n(son lovesMama)\n(daughter lovesMama)\n(all (love mama))\n",
        expected_reconstruction:
            "(papa (lovesMama: loves mama))\n(son lovesMama)\n(daughter lovesMama)\n(all (love mama))\n",
        provenance:
            "link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/TupleTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::SelfDescription,
        ],
    },
    // Upstream: link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/IndentedIdSyntaxTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "indented id fixture",
        language: "LiNo",
        source: "greeting:\n  hello\n",
        expected_reconstruction: "greeting:\n  hello\n",
        provenance:
            "link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/IndentedIdSyntaxTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::LosslessParsing],
    },
    // Upstream: link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/NestedSelfReferenceTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "self reference object fixture",
        language: "LiNo",
        source: "(obj_0: list (int 1) (int 2) (obj_1: list (int 3) (int 4) obj_0))\n",
        expected_reconstruction:
            "(obj_0: list (int 1) (int 2) (obj_1: list (int 3) (int 4) obj_0))\n",
        provenance:
            "link-foundation/links-notation csharp/Link.Foundation.Links.Notation.Tests/NestedSelfReferenceTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::ObjectRoundTrip,
        ],
    },
    // Upstream: link-foundation/links-notation README documents parse_lino_to_links/format_links as the round-trip API the 0.13 crate exposes; license: Unlicense.
    ParityFixture {
        target_name: "links-notation",
        name: "whole network serialization fixture",
        language: "LiNo",
        source: "(papa loves mama)\n",
        expected_reconstruction: "(papa loves mama)\n",
        provenance:
            "link-foundation/links-notation README parse_lino_to_links/format_links round-trip API; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::LosslessParsing,
            ParityCapability::LinoSerialization,
        ],
    },
    // Upstream: link-foundation/link-cli uses the Foundation.Data.Doublets.Cli.Tests project name in C#; license: Unlicense.
    ParityFixture {
        target_name: "link-cli",
        name: "create substitution fixture",
        language: "LiNo",
        source: "(() ((1 1)))\n",
        expected_reconstruction: "(() ((1 1)))\n",
        provenance:
            "link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/BasicQueryProcessor.cs Foundation.Data.Doublets.Cli.Tests; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    // Upstream: link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/BasicQueryProcessor.cs; license: Unlicense.
    ParityFixture {
        target_name: "link-cli",
        name: "update substitution fixture",
        language: "LiNo",
        source: "(((1: 1 1)) ((1: 1 2)))\n",
        expected_reconstruction: "(((1: 1 1)) ((1: 1 2)))\n",
        provenance:
            "link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/BasicQueryProcessor.cs Foundation.Data.Doublets.Cli.Tests; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    // Upstream: link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/BasicQueryProcessor.cs; license: Unlicense.
    ParityFixture {
        target_name: "link-cli",
        name: "delete substitution fixture",
        language: "LiNo",
        source: "(((1 1)) ())\n",
        expected_reconstruction: "(((1 1)) ())\n",
        provenance:
            "link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/BasicQueryProcessor.cs Foundation.Data.Doublets.Cli.Tests; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    // Upstream: link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/AdvancedMixedQueryProcessor.cs; license: Unlicense.
    ParityFixture {
        target_name: "link-cli",
        name: "swap substitution fixture",
        language: "LiNo",
        source: "((($index: $source $target)) (($index: $target $source)))\n",
        expected_reconstruction: "((($index: $source $target)) (($index: $target $source)))\n",
        provenance:
            "link-foundation/link-cli csharp/Foundation.Data.Doublets.Cli.Tests/AdvancedMixedQueryProcessor.cs Foundation.Data.Doublets.Cli.Tests; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::TransformBySubstitution],
    },
    // Upstream: link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/BasicTypesTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "lino-objects-codec",
        name: "roundtrip primitive object fixture",
        language: "LiNo",
        source: "(int 42)\n(str aGVsbG8=)\n",
        expected_reconstruction: "(int 42)\n(str aGVsbG8=)\n",
        provenance:
            "link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/BasicTypesTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::ObjectRoundTrip],
    },
    // Upstream: link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/CircularReferencesTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "lino-objects-codec",
        name: "shared reference object fixture",
        language: "LiNo",
        source: "(obj_0: list obj_1 obj_1 obj_1)\n(obj_1: dict ((str c2hhcmVk) (str dmFsdWU=)))\n",
        expected_reconstruction:
            "(obj_0: list obj_1 obj_1 obj_1)\n(obj_1: dict ((str c2hhcmVk) (str dmFsdWU=)))\n",
        provenance:
            "link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/CircularReferencesTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::ObjectRoundTrip],
    },
    // Upstream: link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/CircularReferencesTests.cs; license: Unlicense.
    ParityFixture {
        target_name: "lino-objects-codec",
        name: "circular reference object fixture",
        language: "LiNo",
        source: "(obj_0: list obj_0)\n",
        expected_reconstruction: "(obj_0: list obj_0)\n",
        provenance:
            "link-foundation/lino-objects-codec csharp/tests/Lino.Objects.Codec.Tests/CircularReferencesTests.cs; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::ObjectRoundTrip],
    },
    // Upstream: link-foundation/relative-meta-logic examples/dependent-types.lino; license: Unlicense.
    ParityFixture {
        target_name: "relative-meta-logic",
        name: "dependent type fixture",
        language: "RML",
        source: "(Type: Type Type)\n(Natural: Type Natural)\n(? (Type of Type))\n",
        expected_reconstruction: "(Type: Type Type)\n(Natural: Type Natural)\n(? (Type of Type))\n",
        provenance:
            "link-foundation/relative-meta-logic examples/dependent-types.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::SemanticEvaluation,
            ParityCapability::SelfDescription,
        ],
    },
    // Upstream: link-foundation/relative-meta-logic examples/belnap-four-valued.lino and examples/ternary-kleene.lino; license: Unlicense.
    ParityFixture {
        target_name: "relative-meta-logic",
        name: "many-valued truth fixture",
        language: "RML",
        source: "(and: min)\n(or: max)\n(? (both true and false))\n(? (neither true nor false))\n",
        expected_reconstruction:
            "(and: min)\n(or: max)\n(? (both true and false))\n(? (neither true nor false))\n",
        provenance:
            "link-foundation/relative-meta-logic examples/belnap-four-valued.lino and examples/ternary-kleene.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::SemanticEvaluation],
    },
    // Upstream: link-foundation/relative-meta-logic examples/liar-paradox.lino; license: Unlicense.
    ParityFixture {
        target_name: "relative-meta-logic",
        name: "probabilistic liar paradox fixture",
        language: "RML",
        source: "(s: s is s)\n((s = false) has probability 0.5)\n(? (s = false))\n(? (not (s = false)))\n",
        expected_reconstruction:
            "(s: s is s)\n((s = false) has probability 0.5)\n(? (s = false))\n(? (not (s = false)))\n",
        provenance:
            "link-foundation/relative-meta-logic examples/liar-paradox.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::SemanticEvaluation],
    },
    // Upstream: link-assistant/formal-ai data/seed/concepts.lino, an actual seed corpus file rather than issue #1's unverified 706 estimate; license: Unlicense.
    ParityFixture {
        target_name: "formal-ai",
        name: "seed concepts corpus fixture",
        language: "LiNo",
        source: "concept_links_notation\n  term \"Links Notation\"\n  intent \"concept_lookup\"\n  category \"data-format\"\n  source \"https://github.com/linksplatform/Documentation\"\n  source_kind \"project-docs\"\n",
        expected_reconstruction:
            "concept_links_notation\n  term \"Links Notation\"\n  intent \"concept_lookup\"\n  category \"data-format\"\n  source \"https://github.com/linksplatform/Documentation\"\n  source_kind \"project-docs\"\n",
        provenance: "link-assistant/formal-ai data/seed/concepts.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
        ],
    },
    // Upstream: link-assistant/formal-ai data/seed/meanings-translation.lino; license: Unlicense.
    ParityFixture {
        target_name: "formal-ai",
        name: "seed translation meanings corpus fixture",
        language: "LiNo",
        source: "meanings\n  meaning \"translate\"\n    role \"translation_action\"\n    lexeme \"en\"\n      word \"translate\"\n",
        expected_reconstruction:
            "meanings\n  meaning \"translate\"\n    role \"translation_action\"\n    lexeme \"en\"\n      word \"translate\"\n",
        provenance:
            "link-assistant/formal-ai data/seed/meanings-translation.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::CrossLanguageReconstruction,
        ],
    },
    // Upstream: link-assistant/formal-ai data/benchmarks/industry-suite.lino; license: Unlicense.
    ParityFixture {
        target_name: "formal-ai",
        name: "industry benchmark corpus fixture",
        language: "LiNo",
        source: "benchmark_suite_issue_304_industry_permissive_slice\n  record_type \"benchmark_suite\"\n  id \"issue_304_industry_permissive_slice\"\n  title \"Permissive industry benchmark slice\"\n  minimum_pass_count \"10\"\n",
        expected_reconstruction:
            "benchmark_suite_issue_304_industry_permissive_slice\n  record_type \"benchmark_suite\"\n  id \"issue_304_industry_permissive_slice\"\n  title \"Permissive industry benchmark slice\"\n  minimum_pass_count \"10\"\n",
        provenance:
            "link-assistant/formal-ai data/benchmarks/industry-suite.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[ParityCapability::SemanticEvaluation],
    },
    // Upstream: link-assistant/formal-ai data/benchmarks/coding-modification-suite.lino; license: Unlicense.
    ParityFixture {
        target_name: "formal-ai",
        name: "coding modification benchmark corpus fixture",
        language: "LiNo",
        source: "coding_modification_case_en_reverse_sort\n  record_type \"coding_modification_case\"\n  id \"en_reverse_sort\"\n  expected_intent \"write_program\"\n  expected_answer_contains \"names.sort_by(|a, b| b.cmp(a))\"\n",
        expected_reconstruction:
            "coding_modification_case_en_reverse_sort\n  record_type \"coding_modification_case\"\n  id \"en_reverse_sort\"\n  expected_intent \"write_program\"\n  expected_answer_contains \"names.sort_by(|a, b| b.cmp(a))\"\n",
        provenance:
            "link-assistant/formal-ai data/benchmarks/coding-modification-suite.lino; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::SemanticEvaluation,
        ],
    },
    // Upstream: link-assistant/meta-expression docs/FORMALIZE.md and the 351-concept js/data/semantic-lexicon.json contract mirrored in src/data/semantic-lexicon.json; license: Unlicense.
    ParityFixture {
        target_name: "meta-expression",
        name: "Hawaii naturalization fixture",
        language: "English",
        source: "Hawaii is a state.\n",
        expected_reconstruction: "Hawaii is a state.\n",
        provenance:
            "link-assistant/meta-expression docs/FORMALIZE.md and js/data/semantic-lexicon.json 351 concepts; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
            ParityCapability::CrossLanguageReconstruction,
        ],
    },
    // Upstream: link-assistant/meta-expression docs/FORMALIZE.md formalization examples; license: Unlicense.
    ParityFixture {
        target_name: "meta-expression",
        name: "1 + 1 formalization fixture",
        language: "English",
        source: "1 + 1 = 2\n",
        expected_reconstruction: "1 + 1 = 2\n",
        provenance: "link-assistant/meta-expression docs/FORMALIZE.md; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
        ],
    },
    // Upstream: link-assistant/meta-expression docs/FORMAL_AI_COMPATIBILITY.md self-reference and unknowns contract; license: Unlicense.
    ParityFixture {
        target_name: "meta-expression",
        name: "this statement is false fixture",
        language: "English",
        source: "this statement is false\n",
        expected_reconstruction: "this statement is false\n",
        provenance:
            "link-assistant/meta-expression docs/FORMAL_AI_COMPATIBILITY.md; license: Unlicense",
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: None,
        capabilities: &[
            ParityCapability::FormalizationRoundTrip,
            ParityCapability::TriviaPreservation,
        ],
    },
];
