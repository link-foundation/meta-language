use crate::parity::{
    ParityCapability, ParityFixture, ParityTransformExpectation, ParityVerificationExpectation,
};

const IDENTIFIER_QUERY: &str = "(identifier) @target";
const QUERY_TRANSFORM_RECONSTRUCT: &[ParityCapability] = &[
    ParityCapability::QueryMatching,
    ParityCapability::TransformBySubstitution,
    ParityCapability::SameLanguageReconstruction,
];
const LOSSLESS_RECONSTRUCT: &[ParityCapability] = &[
    ParityCapability::LosslessParsing,
    ParityCapability::SameLanguageReconstruction,
];
const LOSSLESS_TRIVIA_RECONSTRUCT: &[ParityCapability] = &[
    ParityCapability::LosslessParsing,
    ParityCapability::TriviaPreservation,
    ParityCapability::SameLanguageReconstruction,
];
const LOSSLESS_SNAPSHOT: &[ParityCapability] = &[
    ParityCapability::LosslessParsing,
    ParityCapability::SnapshotVersioning,
];
const FORMALIZATION_CROSS_LANGUAGE: &[ParityCapability] = &[
    ParityCapability::FormalizationRoundTrip,
    ParityCapability::CrossLanguageReconstruction,
];
const OBJECT_SELF_DESCRIPTION: &[ParityCapability] = &[
    ParityCapability::ObjectRoundTrip,
    ParityCapability::SelfDescription,
];
const OBJECT_SNAPSHOT: &[ParityCapability] = &[
    ParityCapability::ObjectRoundTrip,
    ParityCapability::SnapshotVersioning,
];

const fn round_trip_fixture(
    target_name: &'static str,
    name: &'static str,
    language: &'static str,
    source: &'static str,
    provenance: &'static str,
    verification_expectation: ParityVerificationExpectation,
    capabilities: &'static [ParityCapability],
) -> ParityFixture {
    ParityFixture {
        target_name,
        name,
        language,
        source,
        expected_reconstruction: source,
        provenance,
        verification_expectation,
        transform_expectation: None,
        capabilities,
    }
}

const fn identifier_transform_fixture(
    target_name: &'static str,
    name: &'static str,
    language: &'static str,
    source: &'static str,
    provenance: &'static str,
    replacement: &'static str,
    expected_output: &'static str,
) -> ParityFixture {
    ParityFixture {
        target_name,
        name,
        language,
        source,
        expected_reconstruction: source,
        provenance,
        verification_expectation: ParityVerificationExpectation::Clean,
        transform_expectation: Some(ParityTransformExpectation {
            query: IDENTIFIER_QUERY,
            capture_name: "target",
            replacement,
            expected_output,
        }),
        capabilities: QUERY_TRANSFORM_RECONSTRUCT,
    }
}

const fn js_old_to_new_fixture(
    target_name: &'static str,
    provenance: &'static str,
) -> ParityFixture {
    identifier_transform_fixture(
        target_name,
        "javascript identifier replacement",
        "JavaScript",
        "oldValue;\n",
        provenance,
        "newValue",
        "newValue;\n",
    )
}

const fn java_old_to_new_fixture(
    target_name: &'static str,
    provenance: &'static str,
) -> ParityFixture {
    identifier_transform_fixture(
        target_name,
        "java identifier replacement",
        "Java",
        "class OldName {}\n",
        provenance,
        "NewName",
        "class NewName {}\n",
    )
}

const fn c_old_to_new_fixture(
    target_name: &'static str,
    provenance: &'static str,
) -> ParityFixture {
    identifier_transform_fixture(
        target_name,
        "c identifier replacement",
        "C",
        "int old_value;\n",
        provenance,
        "new_value",
        "int new_value;\n",
    )
}

pub(super) const AST_GREP_FIXTURE: ParityFixture = js_old_to_new_fixture(
    "ast-grep",
    "ast-grep/ast-grep rule-tests/no-console-test.yml style valid/invalid rule test; license: MIT",
);

pub(super) const SEMGREP_FIXTURE: ParityFixture = identifier_transform_fixture(
    "Semgrep",
    "python pattern autofix replacement",
    "Python",
    "dangerous_call\n",
    "semgrep/semgrep tests/patterns/python/ac_matching_dots.py and .sgrep paired pattern files; license: LGPL-2.1",
    "safe_call",
    "safe_call\n",
);

pub(super) const COMBY_FIXTURE: ParityFixture = js_old_to_new_fixture(
    "Comby",
    "comby-tools/comby test/common/test_generic.ml structural rewrite cases; license: Apache-2.0",
);

pub(super) const GRITQL_FIXTURE: ParityFixture = js_old_to_new_fixture(
    "GritQL",
    "getgrit/gritql crates/core/src/test.rs and stdlib .grit/patterns markdown tests; license: MIT",
);

pub(super) const SRCML_FIXTURE: ParityFixture = round_trip_fixture(
    "srcML",
    "xml source markup round trip",
    "XML",
    "<unit language=\"C\"><decl_stmt><decl><type><name>int</name></type> <name>x</name></decl>;</decl_stmt></unit>\n",
    "srcML/srcML test/parser/testsuite/decl_stmt_c.c.xml construct fixture; license: GPL-3.0",
    ParityVerificationExpectation::Clean,
    LOSSLESS_TRIVIA_RECONSTRUCT,
);

pub(super) const DIFFTASTIC_FIXTURE: ParityFixture = round_trip_fixture(
    "difftastic",
    "rust sample before structural diff",
    "Rust",
    "fn answer() -> i32 { 1 }\n",
    "Wilfred/difftastic sample_files/rust_1.rs before/after sample-pair layout; license: MIT",
    ParityVerificationExpectation::Clean,
    LOSSLESS_SNAPSHOT,
);

pub(super) const BABEL_FIXTURE: ParityFixture = identifier_transform_fixture(
    "Babel",
    "parser fixture identifier transform",
    "JavaScript",
    "legacyName;\n",
    "babel/babel packages/babel-parser/test/fixtures/*/*/*/input.js parser fixture layout; license: MIT",
    "modernName",
    "modernName;\n",
);

pub(super) const SWC_FIXTURE: ParityFixture = round_trip_fixture(
    "SWC",
    "typescript parser corpus round trip",
    "TypeScript",
    "const value: number = 1;\n",
    "swc-project/swc crates/swc_ecma_parser/tests/typescript parser corpus fixture; license: Apache-2.0",
    ParityVerificationExpectation::Clean,
    LOSSLESS_RECONSTRUCT,
);

pub(super) const OPENREWRITE_FIXTURE: ParityFixture = java_old_to_new_fixture(
    "OpenRewrite",
    "openrewrite/rewrite rewrite-java-tck and RewriteTest rewriteRun(java(before, after)); license: Apache-2.0",
);

pub(super) const SPOON_FIXTURE: ParityFixture = java_old_to_new_fixture(
    "Spoon",
    "INRIA/spoon src/test/java/spoon/test/template and prettyprinter fixtures; license: MIT OR CeCILL-C",
);

pub(super) const JAVAPARSER_FIXTURE: ParityFixture = java_old_to_new_fixture(
    "JavaParser",
    "javaparser/javaparser javaparser-core-testing LexicalPreservingPrinterTest fixtures; license: Apache-2.0 OR LGPL OR GPL",
);

pub(super) const RASCAL_FIXTURE: ParityFixture = round_trip_fixture(
    "Rascal",
    "in language test declaration round trip",
    "txt",
    "test bool sample() = true;\n",
    "usethesource/rascal src/org/rascalmpl/library/lang/**/tests in-language test functions; license: BSD-2-Clause",
    ParityVerificationExpectation::Clean,
    LOSSLESS_RECONSTRUCT,
);

pub(super) const STRATEGO_SPOOFAX_FIXTURE: ParityFixture = js_old_to_new_fixture(
    "Stratego/Spoofax",
    "metaborg/spt .spt parse/transform embedded-fragment expectations; license: Apache-2.0",
);

pub(super) const TXL_FIXTURE: ParityFixture = c_old_to_new_fixture(
    "TXL",
    "txl.ca/examples by-example source transformation cases; license: FreeTXL redistribution license",
);

pub(super) const MPS_FIXTURE: ParityFixture = round_trip_fixture(
    "MPS",
    "projectional model xml round trip",
    "XML",
    "<model><node concept=\"Statement\" name=\"Example\"/></model>\n",
    "JetBrains/MPS testbench projectional model XML fixtures; license: Apache-2.0",
    ParityVerificationExpectation::Clean,
    OBJECT_SELF_DESCRIPTION,
);

pub(super) const COCCINELLE_FIXTURE: ParityFixture = c_old_to_new_fixture(
    "Coccinelle",
    "coccinelle/coccinelle tests/a.c tests/a.cocci tests/a.res input/semantic-patch/result triples; license: GPL-2.0",
);

pub(super) const GF_FIXTURE: ParityFixture = round_trip_fixture(
    "GF",
    "rgl statehood grammar linearization",
    "English",
    "Hawaii is a state.\n",
    "GrammaticalFramework/gf-rgl src/*/test grammar examples and RGL linearization suites; license: LGPL/BSD",
    ParityVerificationExpectation::Clean,
    FORMALIZATION_CROSS_LANGUAGE,
);

pub(super) const UNIVERSAL_DEPENDENCIES_FIXTURE: ParityFixture = round_trip_fixture(
    "Universal Dependencies",
    "ud morphosyntax vocabulary fixture",
    "English",
    "Hawaii is a state.\n",
    "Universal Dependencies v2 CoNLL-U UPOS/UFeats/deprel vocabulary; license: CC BY-SA or treebank-specific",
    ParityVerificationExpectation::Clean,
    &[ParityCapability::LosslessParsing],
);

pub(super) const LANGUAGETOOL_FIXTURE: ParityFixture = round_trip_fixture(
    "LanguageTool",
    "negative grammar rule example",
    "English",
    "Hawaii are a state.\n",
    "languagetool-org/languagetool rules/*/grammar.xml embedded incorrect example pairs; license: LGPL-2.1",
    ParityVerificationExpectation::Recoverable,
    &[ParityCapability::ErrorRecovery],
);

pub(super) const DOUBLETS_RS_FIXTURE: ParityFixture = round_trip_fixture(
    "doublets-rs",
    "file mapped doublets storage gate",
    "LiNo",
    "(storage (doublets-rs snapshot))\n",
    "linksplatform/doublets-rs doublets/src/data/traits.rs and file-mapped storage API; license: Unlicense",
    ParityVerificationExpectation::Clean,
    OBJECT_SNAPSHOT,
);
