//! Requirement I195-RESOURCE-PARSE-MEMORY-BUDGET: each parse has a hard
//! memory budget. The memo cells it keeps, across its runs, repair rounds and
//! embedded grammars, are counted against `memory_limit`, and a parse that
//! needs more stops with a `memoryBudget` rejection instead of growing until
//! the process runs out of memory, as the formal-ai workloads did parsing 0.4
//! to 0.9 MB TypeScript files through the native default. The JavaScript twin
//! is js/tests/issue-195-parse-memory-budget.test.js.

use std::fmt::Write as _;

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, compile_feature_grammar, parse_grammar_links,
};

use super::issue_195_observations::{Observation, record};

const JSON_GRAMMAR: &str = include_str!("../../src/data/native-grammars/json.lino");
const TYPESCRIPT_GRAMMAR: &str = include_str!("../../src/data/native-grammars/typescript.lino");
/// The child test that parses under the capped address space.
const CAPPED_CHILD: &str = "issue_195_parse_memory_budget::capped_typescript_parse";

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-RESOURCE-PARSE-MEMORY-BUDGET",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-resource-parse-memory-budget",
        fixture_file: "docs/vision.md",
        assertions,
        test_name,
    });
}

fn parser(text: &str) -> FeatureGrammarParser {
    let grammar = parse_grammar_links(text).expect("the native grammar parses");
    compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .expect("the native grammar compiles")
}

fn budget(limit: usize, recovery: bool) -> FeatureParseOptions {
    FeatureParseOptions {
        memory_limit: Some(limit),
        error_recovery: Some(recovery),
        accept_recovery: Some(recovery),
        ..FeatureParseOptions::default()
    }
}

#[test]
fn a_parse_that_needs_more_memo_cells_than_its_budget_is_a_memory_budget_rejection() {
    let json = parser(JSON_GRAMMAR);
    let items: Vec<String> = (0..200)
        .map(|i| format!("{{\"key{i}\": [{i}, true, null]}}"))
        .collect();
    let source = format!("[{}]", items.join(", "));
    let outcome = json
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .expect("the parse runs");
    assert!(outcome.ok, "{:?}", outcome.rejection);
    for recovery in [false, true] {
        let outcome = json
            .parse_tree(source.as_bytes(), &budget(500, recovery))
            .expect("the parse runs");
        assert!(!outcome.ok);
        assert_eq!(outcome.tree, None);
        let rejection = outcome.rejection.expect("a rejection");
        assert_eq!(
            (rejection.reason, rejection.limit),
            ("memoryBudget", Some(500))
        );
        assert_eq!(
            rejection.to_string(),
            "the parse needed more than 500 memo cells"
        );
    }
    // Repair rounds share the one budget: invalid input does not get a new budget per round.
    let broken = source.replace("\"key7\": [7,", "\"key7\": [7,,");
    let outcome = json
        .parse_tree(broken.as_bytes(), &budget(500, true))
        .expect("the parse runs");
    assert_eq!(
        outcome.rejection.map(|rejection| rejection.reason),
        Some("memoryBudget")
    );
    let error = json
        .parse(source.as_bytes(), &budget(500, false))
        .expect_err("rejected");
    assert_eq!(error.rejection.reason, "memoryBudget");
    observe(
        &[
            "budgetExceededIsRejection",
            "budgetSharedAcrossRepairRounds",
            "rejectionNamesLimit",
        ],
        "a_parse_that_needs_more_memo_cells_than_its_budget_is_a_memory_budget_rejection",
    );
}

/// About 33 kB of TypeScript, which keeps some 360000 memo cells.
fn large_typescript() -> String {
    (0..400).fold(String::new(), |mut source, i| {
        let _ = writeln!(
            source,
            "export const table{i}: Map<string, Array<number>> = new Map([[\"a\", [1, 2, {i}]]]);"
        );
        source
    })
}

/// Run only by the test below, inside its capped address space.
#[test]
#[ignore = "run by under_a_capped_address_space_a_large_typescript_parse_ends_with_a_diagnostic"]
fn capped_typescript_parse() {
    let typescript = parser(TYPESCRIPT_GRAMMAR);
    let outcome = typescript
        .parse_tree(large_typescript().as_bytes(), &budget(100_000, true))
        .expect("the parse runs");
    let rejection = outcome.rejection.expect("a rejection");
    assert_eq!(
        (rejection.reason, rejection.limit),
        ("memoryBudget", Some(100_000))
    );
}

#[cfg(target_os = "linux")]
#[test]
fn under_a_capped_address_space_a_large_typescript_parse_ends_with_a_diagnostic() {
    let executable = std::env::current_exe().expect("the test executable");
    // 1 GiB of address space, the parse thread's 256 MiB stack included.
    let run = std::process::Command::new("sh")
        .args(["-c", "ulimit -v 1048576 && exec \"$0\" \"$@\""])
        .arg(&executable)
        .args(["--exact", CAPPED_CHILD, "--ignored", "--test-threads", "1"])
        .output()
        .expect("the capped child runs");
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        run.status.success() && stdout.contains("1 passed"),
        "{stdout}\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    observe(
        &["cappedHeapParseEndsWithDiagnostic"],
        "under_a_capped_address_space_a_large_typescript_parse_ends_with_a_diagnostic",
    );
}
