//! Runs `rust/tests/unit/grammar_merge.rs` against the stub crate.
mod issue_195_observations {
    pub struct Observation<'a> {
        pub requirement_id: &'a str,
        pub suffix: &'a str,
        pub fixture_id: &'a str,
        pub fixture_file: &'static str,
        pub assertions: &'a [&'a str],
        pub test_name: &'a str,
    }

    pub fn record(observation: &Observation<'_>) {
        println!(
            "observed {} {} {} {} {:?} {}",
            observation.requirement_id,
            observation.suffix,
            observation.fixture_id,
            observation.fixture_file,
            observation.assertions,
            observation.test_name
        );
    }
}

// build-stub.sh copies rust/tests/unit/grammar_merge.rs here with the fixture
// path adjusted for this crate.
#[path = "probe/grammar_merge_copy.rs"]
mod grammar_merge_copy;
