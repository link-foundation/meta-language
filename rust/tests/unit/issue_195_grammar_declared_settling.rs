//! Requirement I195-GRAMMAR-DECLARED-SETTLING: how two parses of one text
//! that end alike are settled is grammar data, `(settling STEP...)`, which the
//! executor reads for every grammar, whatever format it was imported from, as
//! js/tests/issue-195-grammar-declared-settling.test.js checks for
//! JavaScript.

use std::fs;
use std::path::Path;

use meta_language::grammar::{SETTLING_STEPS, default_settling};
use meta_language::{
    FeatureParseOptions, Grammar, compile_feature_grammar, import_antlr, import_pest,
    parse_grammar_links, parse_native_grammar, render_grammar_links, render_native_grammar,
};

use super::issue_195_observations::{Observation, record};

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-DECLARED-SETTLING",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-declared-settling",
        fixture_file: "docs/grammar/feature-union.md",
        assertions,
        test_name,
    });
}

fn repository() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate is in the repository")
}

/// The tree and the number of ambiguities of `input` under `grammar`.
fn settle(grammar: &Grammar, input: &str) -> (Option<String>, usize) {
    let outcome = compile_feature_grammar(grammar, None, FeatureParseOptions::default())
        .expect("the grammar compiles")
        .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
        .expect("the parse runs");
    (
        outcome.tree.map(|tree| tree.render()),
        outcome.ambiguities.len(),
    )
}

fn with_settling(grammar: &Grammar, settling: &[&str]) -> Grammar {
    let mut declarations = grammar.declarations().clone();
    declarations.settling = Some(settling.iter().map(|&step| step.to_owned()).collect());
    grammar.clone().with_declarations(declarations)
}

/// Two rules match `x`; `b` has the higher dynamic precedence.
const CHOICE: &str = "rule s = normal choice(ref(a), ref(b))
rule a = normal literal(\"x\")
rule b = normal dynamicPrecedence(1, literal(\"x\"))
";
/// Under `(matching longest)` the literal and the token rule's pattern both
/// match `if`.
const LEXED: &str = "rule s = normal choice(ref(keyword), ref(name))
rule keyword = normal literal(\"if\")
rule name = token repeat1(range(\"a\", \"z\"))
";

fn listing(header: &str, body: &str) -> Grammar {
    parse_native_grammar(&format!("start s\n{header}{body}")).expect("the listing parses")
}

fn tree(text: &str, ambiguities: usize) -> (Option<String>, usize) {
    (Some(text.to_owned()), ambiguities)
}

#[test]
fn a_grammar_declares_how_its_parses_are_settled_step_by_step() {
    assert_eq!(
        SETTLING_STEPS,
        ["tokens", "precedence", "dynamic", "first", "ambiguity"]
    );
    // Without a declaration a grammar settles by its matching's default.
    assert_eq!(settle(&listing("", CHOICE), "x"), tree("(s (b \"x\"))", 0));
    assert_eq!(
        settle(&listing("settling dynamic ambiguity\n", CHOICE), "x"),
        tree("(s (b \"x\"))", 0)
    );
    // Without `dynamic` the tie is reported, or settled by the first parse.
    assert_eq!(
        settle(&listing("settling ambiguity\n", CHOICE), "x"),
        tree("(s (a \"x\"))", 1)
    );
    assert_eq!(
        settle(&listing("settling first\n", CHOICE), "x"),
        tree("(s (a \"x\"))", 0)
    );
    assert_eq!(
        settle(&listing("settling dynamic first\n", CHOICE), "x"),
        tree("(s (b \"x\"))", 0)
    );

    // `tokens` prefers the tokens a lexer lexes, so the keyword is no ambiguity.
    assert_eq!(
        settle(&listing("matching longest\n", LEXED), "if"),
        tree("(s (keyword \"if\"))", 0)
    );
    assert_eq!(
        settle(
            &listing(
                "matching longest\nsettling tokens precedence dynamic ambiguity\n",
                LEXED
            ),
            "if"
        ),
        tree("(s (keyword \"if\"))", 0)
    );
    assert_eq!(
        settle(
            &listing(
                "matching longest\nsettling precedence dynamic ambiguity\n",
                LEXED
            ),
            "if"
        ),
        tree("(s (keyword \"if\"))", 1)
    );

    // The declaration travels through both native forms.
    let declared = listing("matching longest\nsettling tokens dynamic first\n", LEXED);
    let steps = Some(vec![
        "tokens".to_owned(),
        "dynamic".to_owned(),
        "first".to_owned(),
    ]);
    assert_eq!(declared.declarations().settling, steps);
    assert!(
        render_native_grammar(&declared)
            .contains("matching longest\nsettling tokens dynamic first\n")
    );
    let links = render_grammar_links(&declared);
    assert!(links.contains("(matching longest) (settling tokens dynamic first))"));
    assert_eq!(
        parse_grammar_links(&links)
            .expect("the links parse")
            .declarations()
            .settling,
        steps
    );

    // A settling is known steps, each once, with exactly one tie step, last.
    for steps in [
        "",
        "dynamic",
        "louder ambiguity",
        "dynamic dynamic first",
        "first ambiguity",
        "ambiguity dynamic",
    ] {
        assert!(
            parse_native_grammar(&format!("start s\nsettling {steps}\n{CHOICE}")).is_err(),
            "{steps}"
        );
        let refused = with_settling(
            &listing("", CHOICE),
            &steps
                .split(' ')
                .filter(|step| !step.is_empty())
                .collect::<Vec<_>>(),
        );
        assert!(
            compile_feature_grammar(&refused, None, FeatureParseOptions::default()).is_err(),
            "{steps}"
        );
    }
    observe(
        &["settlingDeclaredInGrammar"],
        "a grammar declares how its parses are settled, step by step",
    );
}

#[test]
fn every_importer_settles_through_the_same_declared_steps() {
    // The tree-sitter grammars declare the steps of a tree-sitter parser.
    let directory = repository().join("rust/src/data/native-grammars");
    let mut declared = 0;
    for entry in fs::read_dir(&directory).expect("the native grammars are listed") {
        let path = entry.expect("a native grammar").path();
        if path.extension().is_none_or(|extension| extension != "lino") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("the native grammar reads");
        let header = text.lines().next().unwrap_or_default();
        if !header.contains("(matching longest)") {
            continue;
        }
        declared += 1;
        let expected = format!("(settling {})", default_settling("longest").join(" "));
        assert!(header.contains(&expected), "{}", path.display());
    }
    assert!(declared >= 6);

    // An ANTLR import settles by the generalized default, a pest import by
    // the PEG one, and either follows a settling declared on it.
    let antlr = import_antlr("grammar g;\ns : a | b ;\na : 'x' ;\nb : 'x' ;\n")
        .expect("the ANTLR grammar imports");
    assert_eq!(antlr.declarations().settling, None);
    assert_eq!(settle(&antlr, "x"), tree("(s (a \"x\"))", 1));
    assert_eq!(
        settle(&with_settling(&antlr, default_settling("generalized")), "x"),
        tree("(s (a \"x\"))", 1)
    );
    assert_eq!(
        settle(&with_settling(&antlr, &["first"]), "x"),
        tree("(s (a \"x\"))", 0)
    );
    let pest = import_pest("s = { a | b }\na = { \"x\" }\nb = { \"x\" }\n")
        .expect("the pest grammar imports");
    assert_eq!(
        settle(&pest, "x"),
        settle(&with_settling(&pest, default_settling("peg")), "x")
    );
    observe(
        &["sharedByEveryImporter"],
        "every importer settles through the same declared steps",
    );
}

#[test]
fn the_executor_settles_by_the_declared_steps_not_by_source_format() {
    let runtime = repository().join("rust/src/grammar/feature_runtime");
    let format = regex::Regex::new(
        r#"GrammarFormat::(?:TreeSitter|Antlr|Lark)|"tree-sitter"|"antlr"|"lark""#,
    )
    .expect("the format pattern compiles");
    // The runtime files and the files of their submodules.
    let mut directories = vec![runtime.clone()];
    let mut paths = Vec::new();
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory).expect("the runtime is listed") {
            let path = entry.expect("a runtime file").path();
            if path.is_dir() {
                directories.push(path);
            } else {
                paths.push(path);
            }
        }
    }
    assert!(
        paths
            .iter()
            .any(|path| path.ends_with("forking/lead_sets.rs"))
    );
    for path in paths {
        let source = fs::read_to_string(&path).expect("the runtime file reads");
        // No settling decided by the format a grammar was imported from.
        let found: Vec<_> = format
            .find_iter(&source)
            .map(|found| found.as_str())
            .collect();
        assert!(found.is_empty(), "{}: {found:?}", path.display());
    }
    let ordering = fs::read_to_string(runtime.join("ordering.rs")).expect("ordering.rs reads");
    for step in ["Tokens", "Precedence", "Dynamic"] {
        assert!(
            ordering.contains(&format!("SettlingStep::{step}")),
            "{step}"
        );
    }
    observe(
        &["noHostCodeSettling"],
        "the executor settles by the declared steps, not by source format",
    );
}
