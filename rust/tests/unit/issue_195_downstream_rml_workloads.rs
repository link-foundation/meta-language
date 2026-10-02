//! Issue #195 downstream relative-meta-logic workloads: the workloads of
//! relative-meta-logic pull request 184 run against the clean installed
//! meta-language artifacts. `js/scripts/run-rml-pr184-workloads.mjs` checks
//! out the pinned head, verifies every workload blob, installs the npm tarball
//! and patches the unpacked crate in, runs the workload tests and the consumer
//! probes, and writes a report. This suite ports the report validator of
//! `js/scripts/issue-195-rml-workloads.mjs`, checks it on synthetic reports,
//! and, when `ISSUE_195_RML_WORKLOAD_REPORT` names the CI report, validates its
//! crate part and records the Rust observations. Mirrors
//! `js/tests/issue-195-downstream-rml-workloads.test.js`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::issue_195_observations as observations;

const REQUIREMENT_ID: &str = "I195-DOWNSTREAM-RML-WORKLOADS";
const FIXTURE_ID: &str = "planned:repository-directive:i195-downstream-rml-workloads";
const FIXTURE: &str = "parity/fixtures/rml-pr184-workloads.json";
const REPORT_VARIABLE: &str = "ISSUE_195_RML_WORKLOAD_REPORT";
const SCHEMA_VERSION: u64 = 1;
const WORKLOADS_RUN: &str = "workloadsRunOnInstalledArtifacts";
const ASSERTIONS: [&str; 4] = [
    WORKLOADS_RUN,
    "sharedConceptsReused",
    "distinctionsPreserved",
    "foundationAuthorityStaysInRml",
];
/// The consumer-probe assertions; the first assertion is read from the report itself.
const PROBE_ASSERTIONS: [&str; 3] = [
    "sharedConceptsReused",
    "distinctionsPreserved",
    "foundationAuthorityStaysInRml",
];
const COUNT_KEYS: [&str; 4] = ["passed", "failed", "skipped", "total"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Runtime {
    JavaScript,
    Rust,
}

impl Runtime {
    const fn name(self) -> &'static str {
        match self {
            Self::JavaScript => "javascript",
            Self::Rust => "rust",
        }
    }

    /// The report part that carries the runtime.
    const fn part(self) -> &'static str {
        match self {
            Self::JavaScript => "npm",
            Self::Rust => "crate",
        }
    }

    /// The workload directory of the runtime in relative-meta-logic.
    const fn directory(self) -> &'static str {
        match self {
            Self::JavaScript => "js/",
            Self::Rust => "rust/",
        }
    }
}

/// The problems of a report for one runtime, keyed by assertion, and the
/// assertions without a problem in declaration order.
#[derive(Debug)]
struct Validation {
    problems: BTreeMap<&'static str, Vec<String>>,
    observed: Vec<&'static str>,
}

impl Validation {
    fn unobserved(&self) -> Vec<&'static str> {
        ASSERTIONS
            .into_iter()
            .filter(|assertion| !self.observed.contains(assertion))
            .collect()
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{} is JSON: {error}", path.display()))
}

fn fixture() -> Value {
    read_json(&root().join(FIXTURE))
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or_default()
}

fn describe(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), ToOwned::to_owned)
}

/// The inventoried workload test files of `runtime`, as repository paths.
fn workload_test_files(fixture: &Value, runtime: Runtime) -> Vec<String> {
    array(&fixture["workloads"])
        .iter()
        .filter(|workload| workload["kind"] == "test")
        .filter_map(|workload| workload["file"].as_str())
        .filter(|file| file.starts_with(runtime.directory()))
        .map(str::to_owned)
        .collect()
}

/// Counts of test outcomes, as the runner writes them.
fn count_outcomes(tests: &[Value]) -> Value {
    let count = |outcome: &str| {
        tests
            .iter()
            .filter(|entry| entry["outcome"] == outcome)
            .count()
    };
    json!({
        "passed": count("passed"),
        "failed": count("failed"),
        "skipped": count("skipped"),
        "total": tests.len(),
    })
}

/// Problems of the pinned checkout: the head and every inventoried blob.
fn checkout_problems(report: &Value, fixture: &Value) -> Vec<String> {
    let mut problems = Vec::new();
    let rml = &report["rml"];
    if rml["repository"] != fixture["repository"] {
        problems.push(format!(
            "the report checks out {}, not {}",
            rml["repository"], fixture["repository"]
        ));
    }
    for key in ["headRevision", "checkedOutRevision"] {
        if rml[key] != fixture["headRevision"] {
            problems.push(format!(
                "the RML {key} is {}, not the pinned {}",
                rml[key], fixture["headRevision"]
            ));
        }
    }
    let verified = array(&rml["verifiedBlobs"]);
    for workload in array(&fixture["workloads"]) {
        let file = &workload["file"];
        let blob = &workload["blob"];
        let Some(entry) = verified.iter().find(|entry| entry["file"] == *file) else {
            problems.push(format!(
                "the workload blob of {} was not verified",
                describe(file)
            ));
            continue;
        };
        if entry["actual"] != *blob || entry["expected"] != *blob {
            problems.push(format!(
                "{} has blob {}, the fixture pins {}",
                describe(file),
                entry["actual"],
                describe(blob)
            ));
        }
    }
    problems
}

fn is_sha256(value: &Value) -> bool {
    value.as_str().is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
    })
}

/// A key that is present and JSON `null`, as the runner writes a path source.
fn is_present_null(value: &Value, key: &str) -> bool {
    value.get(key).is_some_and(Value::is_null)
}

fn javascript_resolution_problems(checksum: &Value, resolution: &Value) -> Vec<String> {
    let mut problems = Vec::new();
    if !resolution["lockResolved"]
        .as_str()
        .is_some_and(|resolved| resolved.starts_with("file:"))
    {
        problems.push(format!(
            "meta-language resolves from {}, not from the installed artifact",
            resolution["lockResolved"]
        ));
    }
    if checksum["installedIntegrity"]
        .as_str()
        .is_none_or(str::is_empty)
        || checksum["installedIntegrity"] != checksum["expectedIntegrity"]
    {
        problems.push(format!(
            "the installed integrity {} is not the tarball's {}",
            checksum["installedIntegrity"], checksum["expectedIntegrity"]
        ));
    }
    if resolution["freshCache"] != true || resolution["consumerHadNoModules"] != true {
        problems.push(
            "the npm install did not start from an empty node_modules and a fresh cache".to_owned(),
        );
    }
    problems
}

fn rust_resolution_problems(resolution: &Value) -> Vec<String> {
    let mut problems = Vec::new();
    let unpacked = resolution["unpackedDirectory"].as_str();
    let manifest = resolution["manifestPath"].as_str();
    let patched = unpacked
        .zip(manifest)
        .is_some_and(|(unpacked, manifest)| manifest.starts_with(unpacked));
    if !is_present_null(resolution, "metadataSource") || !patched {
        problems.push(format!(
            "meta-language resolves to {} (source {}), not to the unpacked crate",
            resolution["manifestPath"], resolution["metadataSource"]
        ));
    }
    if !is_present_null(resolution, "lockSource") {
        problems.push(format!(
            "Cargo.lock resolves meta-language from {}, not from a path",
            resolution["lockSource"]
        ));
    }
    if resolution["metaLanguagePackages"].as_u64() != Some(1) {
        problems.push(format!(
            "the RML build resolves {} meta-language packages, not one",
            resolution["metaLanguagePackages"]
        ));
    }
    problems
}

/// Problems of the artifact: its checksum and that the consumer resolved
/// meta-language to it.
fn artifact_problems(runtime: Runtime, part: &Value) -> Vec<String> {
    let name = runtime.name();
    let checksum = &part["checksum"];
    let resolution = &part["resolution"];
    let mut problems = Vec::new();
    if !is_sha256(&checksum["sha256"]) || checksum["sha256"] != checksum["expectedSha256"] {
        problems.push(format!(
            "the {name} artifact sha256 {} does not match its published checksum {}",
            checksum["sha256"], checksum["expectedSha256"]
        ));
    }
    problems.extend(match runtime {
        Runtime::JavaScript => javascript_resolution_problems(checksum, resolution),
        Runtime::Rust => rust_resolution_problems(resolution),
    });
    if part["version"].as_str().is_none_or(str::is_empty)
        || resolution["installedVersion"] != part["version"]
    {
        problems.push(format!(
            "the consumer resolved meta-language {}, the artifact is {}",
            resolution["installedVersion"], part["version"]
        ));
    }
    problems
}

/// Problems of the executed workload tests: every inventoried test file ran
/// and nothing failed.
fn test_problems(runtime: Runtime, part: &Value, fixture: &Value) -> Vec<String> {
    let name = runtime.name();
    let tests = array(&part["tests"]);
    let counts = count_outcomes(tests);
    let mut problems = Vec::new();
    if COUNT_KEYS
        .into_iter()
        .any(|key| counts[key] != part["counts"][key])
    {
        problems.push(format!(
            "the {name} counts {} do not match the executed tests {counts}",
            part["counts"]
        ));
    }
    for entry in tests.iter().filter(|entry| entry["outcome"] == "failed") {
        problems.push(format!(
            "the {name} workload {} › {} failed",
            describe(&entry["file"]),
            describe(&entry["name"])
        ));
    }
    for file in workload_test_files(fixture, runtime) {
        if !tests
            .iter()
            .any(|entry| entry["file"] == file.as_str() && entry["outcome"] == "passed")
        {
            problems.push(format!("no {name} test of {file} passed"));
        }
    }
    if part["exit"].as_i64() != Some(0) {
        problems.push(format!(
            "the {name} workload run exited with {}",
            part["exit"]
        ));
    }
    problems
}

/// Problems of one probe assertion: it holds, every check holds, and there is
/// at least one check.
fn probe_problems(runtime: Runtime, part: &Value, assertion: &str) -> Vec<String> {
    let name = runtime.name();
    let observed = &part["probe"][assertion];
    let checks = array(&observed["checks"]);
    if checks.is_empty() {
        return vec![format!("the {name} probe made no {assertion} check")];
    }
    let mut problems: Vec<String> = checks
        .iter()
        .filter(|probe_check| probe_check["holds"] != true)
        .map(|probe_check| {
            format!(
                "the {name} {assertion} check \"{}\" failed: {}",
                describe(&probe_check["name"]),
                describe(&probe_check["detail"])
            )
        })
        .collect();
    if problems.is_empty() && observed["holds"] != true {
        problems.push(format!(
            "the {name} probe reports {assertion} as not holding"
        ));
    }
    problems
}

/// Port of `validateRmlWorkloadReport`: a probe observation counts only when
/// the probe ran against the verified checkout and the installed artifact.
fn validate(report: &Value, fixture: &Value, runtime: Runtime) -> Validation {
    let mut problems: BTreeMap<&'static str, Vec<String>> = ASSERTIONS
        .into_iter()
        .map(|assertion| (assertion, Vec::new()))
        .collect();
    let part = &report[runtime.part()];
    if report["schemaVersion"].as_u64() != Some(SCHEMA_VERSION) {
        problems.entry(WORKLOADS_RUN).or_default().push(format!(
            "the report schema version is {}, not {SCHEMA_VERSION}",
            report["schemaVersion"]
        ));
    }
    if part.is_null() || part["skipped"] == true {
        let reason = part["reason"]
            .as_str()
            .map(|reason| format!(": {reason}"))
            .unwrap_or_default();
        for messages in problems.values_mut() {
            messages.push(format!("the report has no {} run{reason}", runtime.name()));
        }
        return Validation {
            problems,
            observed: Vec::new(),
        };
    }
    let mut grounding = checkout_problems(report, fixture);
    grounding.extend(artifact_problems(runtime, part));
    let workloads = problems.entry(WORKLOADS_RUN).or_default();
    workloads.extend_from_slice(&grounding);
    workloads.extend(test_problems(runtime, part, fixture));
    for assertion in PROBE_ASSERTIONS {
        let messages = problems.entry(assertion).or_default();
        messages.extend(
            grounding
                .iter()
                .map(|problem| format!("not observed on the installed artifact: {problem}")),
        );
        messages.extend(probe_problems(runtime, part, assertion));
    }
    let observed = ASSERTIONS
        .into_iter()
        .filter(|assertion| problems.get(assertion).is_some_and(Vec::is_empty))
        .collect();
    Validation { problems, observed }
}

fn merged(mut base: Value, extra: Value) -> Value {
    if let (Some(base_map), Value::Object(extra_map)) = (base.as_object_mut(), extra) {
        base_map.extend(extra_map);
    }
    base
}

fn passing_part(fixture: &Value, runtime: Runtime, resolution: Value, checksum: Value) -> Value {
    let digest = "a".repeat(64);
    let tests: Vec<Value> = workload_test_files(fixture, runtime)
        .into_iter()
        .map(|file| json!({ "file": file, "name": "a workload", "outcome": "passed" }))
        .collect();
    let counts = count_outcomes(&tests);
    let probe: serde_json::Map<String, Value> = PROBE_ASSERTIONS
        .into_iter()
        .map(|assertion| {
            (
                assertion.to_owned(),
                json!({
                    "holds": true,
                    "checks": [{ "name": format!("{assertion} check"), "holds": true, "detail": {} }],
                }),
            )
        })
        .collect();
    json!({
        "skipped": false,
        "version": "1.2.3",
        "checksum": merged(json!({ "sha256": digest, "expectedSha256": digest }), checksum),
        "resolution": merged(json!({ "installedVersion": "1.2.3" }), resolution),
        "tests": tests,
        "counts": counts,
        "exit": 0,
        "probe": probe,
    })
}

/// A report on which every assertion of both runtimes holds.
fn passing_report(fixture: &Value) -> Value {
    let verified: Vec<Value> = array(&fixture["workloads"])
        .iter()
        .map(|workload| {
            json!({
                "file": workload["file"],
                "expected": workload["blob"],
                "actual": workload["blob"],
                "matches": true,
            })
        })
        .collect();
    json!({
        "schemaVersion": SCHEMA_VERSION,
        "requirementId": REQUIREMENT_ID,
        "rml": {
            "repository": fixture["repository"],
            "headRevision": fixture["headRevision"],
            "checkedOutRevision": fixture["headRevision"],
            "verifiedBlobs": verified,
        },
        "npm": passing_part(
            fixture,
            Runtime::JavaScript,
            json!({
                "lockResolved": "file:../../meta-language-1.2.3.tgz",
                "freshCache": true,
                "consumerHadNoModules": true,
            }),
            json!({ "installedIntegrity": "sha512-same", "expectedIntegrity": "sha512-same" }),
        ),
        "crate": passing_part(
            fixture,
            Runtime::Rust,
            json!({
                "unpackedDirectory": "/work/crate-source/meta-language-1.2.3/",
                "manifestPath": "/work/crate-source/meta-language-1.2.3/Cargo.toml",
                "metadataSource": null,
                "lockSource": null,
                "metaLanguagePackages": 1,
            }),
            json!({}),
        ),
    })
}

fn tests_of<'report>(report: &'report mut Value, part: &str) -> &'report mut Vec<Value> {
    report[part]["tests"]
        .as_array_mut()
        .expect("the synthetic part lists its tests")
}

fn recount(report: &mut Value, part: &str) {
    let counts = count_outcomes(array(&report[part]["tests"]));
    report[part]["counts"] = counts;
}

#[test]
fn the_fixture_inventories_the_workload_test_files_of_both_runtimes() {
    let fixture = fixture();
    assert_eq!(
        workload_test_files(&fixture, Runtime::JavaScript),
        [
            "js/tests/meta-language-support.test.mjs",
            "js/tests/theory-network-linked.test.mjs",
            "js/tests/theory-network.test.mjs",
        ]
    );
    assert_eq!(
        workload_test_files(&fixture, Runtime::Rust),
        [
            "rust/tests/meta_language_support_tests.rs",
            "rust/tests/theory_network_tests.rs",
        ]
    );
}

#[test]
fn a_report_on_which_everything_holds_observes_every_assertion_of_both_runtimes() {
    let fixture = fixture();
    for runtime in [Runtime::JavaScript, Runtime::Rust] {
        let validation = validate(&passing_report(&fixture), &fixture, runtime);
        assert_eq!(validation.observed, ASSERTIONS, "{validation:#?}");
    }
}

#[test]
fn a_failing_workload_test_leaves_the_workloads_unobserved() {
    let fixture = fixture();
    let mut report = passing_report(&fixture);
    tests_of(&mut report, "crate").push(json!({
        "file": "rust/tests/theory_network_tests.rs",
        "name": "loads_the_network",
        "outcome": "failed",
    }));
    recount(&mut report, "crate");
    report["crate"]["exit"] = json!(101);
    let validation = validate(&report, &fixture, Runtime::Rust);
    assert_eq!(validation.unobserved(), [WORKLOADS_RUN]);
    assert!(
        validation.problems[WORKLOADS_RUN]
            .iter()
            .any(|problem| problem.contains("loads_the_network failed")),
        "{validation:#?}"
    );

    let mut unrun = passing_report(&fixture);
    tests_of(&mut unrun, "npm").retain(|entry| entry["file"] != "js/tests/theory-network.test.mjs");
    recount(&mut unrun, "npm");
    assert_eq!(
        validate(&unrun, &fixture, Runtime::JavaScript).unobserved(),
        [WORKLOADS_RUN],
        "a workload file without a passed test"
    );

    let mut miscounted = passing_report(&fixture);
    miscounted["crate"]["counts"]["passed"] = json!(99);
    assert_eq!(
        validate(&miscounted, &fixture, Runtime::Rust).unobserved(),
        [WORKLOADS_RUN],
        "counts that differ from the executed tests"
    );
}

#[test]
fn a_consumer_that_resolved_meta_language_from_the_registry_observes_nothing() {
    let fixture = fixture();
    let registry = "registry+https://github.com/rust-lang/crates.io-index";
    let mut report = passing_report(&fixture);
    report["crate"]["resolution"]["metadataSource"] = json!(registry);
    report["crate"]["resolution"]["lockSource"] = json!(registry);
    report["crate"]["resolution"]["manifestPath"] =
        json!("/cargo/registry/src/meta-language-1.2.3/Cargo.toml");
    let validation = validate(&report, &fixture, Runtime::Rust);
    assert!(validation.observed.is_empty(), "{validation:#?}");
    assert!(
        validation.problems["sharedConceptsReused"]
            .iter()
            .any(|problem| problem.contains("crates.io-index")),
        "{validation:#?}"
    );

    let mut unrecorded = passing_report(&fixture);
    unrecorded["crate"]["resolution"]
        .as_object_mut()
        .expect("the synthetic resolution is an object")
        .remove("lockSource");
    assert!(
        validate(&unrecorded, &fixture, Runtime::Rust)
            .observed
            .is_empty(),
        "an unrecorded lock source"
    );

    let mut twice = passing_report(&fixture);
    twice["crate"]["resolution"]["metaLanguagePackages"] = json!(2);
    assert!(
        validate(&twice, &fixture, Runtime::Rust)
            .observed
            .is_empty(),
        "two meta-language packages in the build"
    );

    let mut npm = passing_report(&fixture);
    npm["npm"]["resolution"]["lockResolved"] =
        json!("https://registry.npmjs.org/meta-language/-/meta-language-0.46.0.tgz");
    assert!(
        validate(&npm, &fixture, Runtime::JavaScript)
            .observed
            .is_empty()
    );

    let mut checksum = passing_report(&fixture);
    checksum["crate"]["checksum"]["expectedSha256"] = json!("b".repeat(64));
    assert!(
        validate(&checksum, &fixture, Runtime::Rust)
            .observed
            .is_empty(),
        "a crate that is not the candidate"
    );
}

#[test]
fn a_checkout_whose_workload_blob_differs_from_the_pin_observes_nothing() {
    let fixture = fixture();
    let mut report = passing_report(&fixture);
    report["rml"]["verifiedBlobs"][0]["actual"] = json!("0".repeat(40));
    let validation = validate(&report, &fixture, Runtime::Rust);
    assert!(validation.observed.is_empty(), "{validation:#?}");
    let first = describe(&fixture["workloads"][0]["file"]);
    assert!(
        validation.problems["foundationAuthorityStaysInRml"]
            .iter()
            .any(|problem| problem.contains(&first)),
        "{validation:#?}"
    );

    let mut missing = passing_report(&fixture);
    missing["rml"]["verifiedBlobs"]
        .as_array_mut()
        .expect("the synthetic checkout lists its blobs")
        .pop();
    assert!(
        validate(&missing, &fixture, Runtime::JavaScript)
            .observed
            .is_empty(),
        "an unverified blob"
    );

    let mut moved = passing_report(&fixture);
    moved["rml"]["checkedOutRevision"] = json!("e".repeat(40));
    assert!(
        validate(&moved, &fixture, Runtime::Rust)
            .observed
            .is_empty(),
        "another head"
    );
}

#[test]
fn a_missing_or_skipped_runtime_observes_nothing_for_that_runtime_only() {
    let fixture = fixture();
    let mut report = passing_report(&fixture);
    report
        .as_object_mut()
        .expect("the synthetic report is an object")
        .remove("crate");
    assert!(
        validate(&report, &fixture, Runtime::Rust)
            .observed
            .is_empty()
    );
    assert_eq!(
        validate(&report, &fixture, Runtime::JavaScript).observed,
        ASSERTIONS
    );

    let mut skipped = passing_report(&fixture);
    skipped["crate"] =
        json!({ "skipped": true, "reason": "--skip-rust: the Rust workloads were not run" });
    let validation = validate(&skipped, &fixture, Runtime::Rust);
    assert!(validation.observed.is_empty(), "{validation:#?}");
    assert!(
        validation.problems["distinctionsPreserved"]
            .iter()
            .any(|problem| problem.contains("--skip-rust")),
        "{validation:#?}"
    );

    let mut stale = passing_report(&fixture);
    stale["schemaVersion"] = json!(SCHEMA_VERSION + 1);
    assert_eq!(
        validate(&stale, &fixture, Runtime::JavaScript).unobserved(),
        [WORKLOADS_RUN]
    );
}

#[test]
fn a_failed_or_empty_probe_check_leaves_only_its_assertion_unobserved() {
    let fixture = fixture();
    for assertion in PROBE_ASSERTIONS {
        let mut report = passing_report(&fixture);
        report["crate"]["probe"][assertion]["checks"]
            .as_array_mut()
            .expect("the synthetic probe lists its checks")
            .push(json!({
                "name": "the evaluator closure",
                "holds": false,
                "detail": "meta-language reached",
            }));
        let validation = validate(&report, &fixture, Runtime::Rust);
        assert_eq!(validation.unobserved(), [assertion]);
        assert!(
            validation.problems[assertion][0]
                .contains("the evaluator closure\" failed: meta-language reached"),
            "{validation:#?}"
        );
    }

    let mut empty = passing_report(&fixture);
    empty["npm"]["probe"]["distinctionsPreserved"]["checks"] = json!([]);
    assert_eq!(
        validate(&empty, &fixture, Runtime::JavaScript).unobserved(),
        ["distinctionsPreserved"]
    );

    let mut contradicted = passing_report(&fixture);
    contradicted["crate"]["probe"]["sharedConceptsReused"]["holds"] = json!(false);
    assert_eq!(
        validate(&contradicted, &fixture, Runtime::Rust).unobserved(),
        ["sharedConceptsReused"]
    );
}

#[test]
fn the_ci_workload_report_holds_every_assertion_for_the_patched_crate() {
    let Some(path) = std::env::var_os(REPORT_VARIABLE).map(PathBuf::from) else {
        eprintln!(
            "skipped: {REPORT_VARIABLE} is not set; the workload report is produced by the CI job \"Relative Meta Logic Workloads\""
        );
        return;
    };
    if !path.exists() {
        eprintln!(
            "skipped: {REPORT_VARIABLE} names {}, which does not exist; the workloads stay unobserved",
            path.display()
        );
        return;
    }
    let report = read_json(&path);
    assert_eq!(report["requirementId"], REQUIREMENT_ID);
    assert_eq!(report["fixture"], FIXTURE);
    let validation = validate(&report, &fixture(), Runtime::Rust);
    assert!(
        validation.problems.values().all(Vec::is_empty),
        "{:#?}",
        validation.problems
    );
    assert_eq!(validation.observed, ASSERTIONS);
    observations::record(&observations::Observation {
        requirement_id: REQUIREMENT_ID,
        suffix: "behavior",
        fixture_id: FIXTURE_ID,
        fixture_file: FIXTURE,
        assertions: &validation.observed,
        test_name: "the_ci_workload_report_holds_every_assertion_for_the_patched_crate",
    });
}
