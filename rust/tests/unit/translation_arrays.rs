//! JavaScript arrays the program reads: a Rust `Vec`, a Lean `Array` and a
//! Rocq list, read through an optional bounds-checked helper that retains
//! JavaScript's observable absence outside an array.

use meta_language::translation::javascript::parse_javascript;
use meta_language::{TranslationSupport, translate_program};

const ARRAYS: &str = "const xs = [1, 2];
console.log(xs[1] + xs.length);
function sum(values) {
  let s = 0n;
  for (const v of values) s += v;
  return s;
}
console.log(sum([1n, ...[2n]]));
";

const IN_BOUNDS: &str = "the translation agrees with the source on executions whose array reads are in bounds; JavaScript reads undefined at an index outside an array, where the target aborts";

fn code(source: &str, target: &str) -> String {
    let translated =
        translate_program(source, "JavaScript", target).expect("translation descriptor");
    assert!(
        translated.diagnostic().is_none(),
        "{target}: {:?}",
        translated.diagnostic()
    );
    translated.code().to_owned()
}

fn refusal(source: &str) -> String {
    translate_program(source, "JavaScript", "Rust")
        .expect("translation descriptor")
        .diagnostic()
        .expect("a diagnostic")
        .message
        .clone()
}

#[test]
fn arrays_the_program_reads_are_a_semantic_translation_in_every_target_with_observable_absence() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(ARRAYS, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        let contract = translated.contract();
        assert_eq!(
            contract.support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
        assert!(!contract.assumptions.contains(&IN_BOUNDS), "{target}");
    }
}

#[test]
fn an_array_is_a_vec_an_array_and_a_list_read_through_an_optional_bounds_checked_helper() {
    let rust = code(ARRAYS, "Rust");
    for expected in [
        "let xs = vec![1f64, 2f64];",
        "match crate::ml_array::optional_at(&xs, crate::ml_array::number_index(1f64)) { Some(ml_present) => ml_present, None => f64::NAN } + (xs.len() as f64)",
        "crate::ml_array::append(vec![crate::ml::Big::from_i128(1)], vec![crate::ml::Big::from_i128(2)])",
    ] {
        assert!(rust.contains(expected), "{rust}");
    }
    let lean = code(ARRAYS, "Lean");
    for expected in [
        "let xs := (#[(1 : Float), (2 : Float)] : (Array Float))",
        "xs[ml_index.toUInt64.toNat]? else none",
        "| none => (0 / 0 : Float)) + (Float.ofNat xs.size)",
        "((#[(1 : Int)] : (Array Int)) ++ (#[(2 : Int)] : (Array Int)))",
    ] {
        assert!(lean.contains(expected), "{lean}");
    }
    let rocq = code(ARRAYS, "Rocq");
    for expected in [
        "let xs := (1%float :: 2%float :: @nil float) in",
        "List.nth_error xs (Z.to_nat ml_index)",
        "| None => PrimFloat.nan end)",
        "(List.app (1%Z :: @nil Z) (2%Z :: @nil Z))",
    ] {
        assert!(rocq.contains(expected), "{rocq}");
    }
}

#[test]
fn inference_fixes_the_element_type_of_an_empty_array_from_its_later_use() {
    let source = "function range(n) {\n  let out = [];\n  for (let i = 0n; i < n; i++) out = [...out, i];\n  return out;\n}\nconsole.log(range(3n).length);\n";
    let rust = code(source, "Rust");
    assert!(rust.contains("Vec::<crate::ml::Big>::new()"), "{rust}");
}

#[test]
fn a_typedef_field_may_hold_an_array_of_data_too() {
    let source = "/** @typedef {{ $: 'leaf', items: bigint[] } | { $: 'node', kids: Array<Tree> }} Tree */\nfunction size(t) {\n  if (t.$ === 'leaf') return t.items.length;\n  let n = 0;\n  for (const kid of t.kids) n += size(kid);\n  return n;\n}\nconsole.log(size({ $: 'node', kids: [{ $: 'leaf', items: [1n, 2n] }] }));\n";
    code(source, "Rust");
    // Lean derives no DecidableEq through an Array of data, and Rocq would warn of every nested definition.
    assert!(code(source, "Lean").contains("  deriving Repr, Inhabited"));
    assert!(code(source, "Rocq").contains("#[warnings=\"-register-all\"]\nInductive Tree"));
}

#[test]
fn the_array_forms_that_are_not_kept_are_refused_with_a_reason() {
    for (source, message) in [
        (
            "const xs = [1, 2];\nconsole.log(xs);\n",
            "text of an array: console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one at 31..33",
        ),
        (
            "const xs = [1, 2];\nxs[0] = 3;\n",
            "reference mutation: this mutation requires its shared reference and slot operations to be represented at 19..28",
        ),
        (
            "const xs = [1, 2];\nconsole.log(xs === xs);\n",
            "comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40",
        ),
    ] {
        if source == "const xs = [1, 2];\nconsole.log(xs === xs);\n" {
            assert_modern_array_identity_translation(source, message);
        } else {
            assert_eq!(refusal(source), message, "{source}");
        }
    }
    // This original input was historically refused; the current shared
    // JavaScript model preserves UTF-16 length and therefore admits it.
    for target in ["Rust", "Lean", "Rocq"] {
        code("const s = 'ab';\nconsole.log(s.length);\n", target);
    }
    assert!(refusal("const xs = [1, , 2];\n").starts_with("array hole"));
    assert!(refusal("for (const k in { a: 1 }) console.log(k);\n").starts_with("for…in loop"));
}

#[test]
fn utf16_string_length_retains_supplementary_characters_in_each_target() {
    let source = "const s = 'a😀b';\nconsole.log(s.length);\n";
    assert!(code(source, "Rust").contains("encode_utf16().count() as f64"));
    assert!(code(source, "Lean").contains("if c.toNat > 65535 then 2 else 1"));
    assert!(code(source, "Rocq").contains("ml_utf16_length"));
}

#[test]
fn out_of_bounds_array_reads_retain_absence_instead_of_an_in_bounds_assumption() {
    let source = "function read(xs, i) { return xs[i]; }\nconsole.log(read([1], 4) ?? 9);\n";
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(source, "JavaScript", target).expect("translation descriptor");
        assert!(
            translated.diagnostic().is_none(),
            "{target}: {:?}",
            translated.diagnostic()
        );
        assert!(
            !translated.contract().assumptions.contains(&IN_BOUNDS),
            "{target}"
        );
        let expected = match target {
            "Rust" => "crate::ml_array::optional_at",
            "Lean" => "else none",
            _ => "List.nth_error",
        };
        assert!(
            translated.code().contains(expected),
            "{}",
            translated.code()
        );
    }
}

#[test]
fn source_iteration_retains_its_actual_iterable_wrapper_and_generated_read_proof() {
    fn count(value: &serde_json::Value, kind: &str, proven: bool) -> usize {
        match value {
            serde_json::Value::Object(fields) => {
                if fields.get("k").and_then(serde_json::Value::as_str) == Some("forOfIterable") {
                    assert_eq!(
                        fields.get("type"),
                        Some(&serde_json::json!({"kind":"array","element":{"kind":"int"}}))
                    );
                }
                usize::from(
                    fields.get("k").and_then(serde_json::Value::as_str) == Some(kind)
                        && (!proven
                            || fields.get("provenInBounds")
                                == Some(&serde_json::Value::Bool(true))),
                ) + fields
                    .values()
                    .map(|value| count(value, kind, proven))
                    .sum::<usize>()
            }
            serde_json::Value::Array(values) => {
                values.iter().map(|value| count(value, kind, proven)).sum()
            }
            _ => 0,
        }
    }
    let program = parse_javascript(ARRAYS).expect("the original array program parses");
    let raw = serde_json::to_value(program).expect("raw source tree serialization");
    assert_eq!(count(&raw, "forOfIterable", false), 1);
    assert_eq!(count(&raw, "index", true), 1);
}

#[test]
fn string_iteration_uses_code_points_while_length_uses_utf16_units() {
    let source = "function count(s) { let n = 0; for (const point of s) n += 1; return n; }\nconsole.log(count('a😀b'));\n";
    assert!(code(source, "Rust").contains(".chars().map(|ml_character| ml_character.to_string())"));
    assert!(
        code(source, "Lean")
            .contains(".toList.map (fun ml_character => String.singleton ml_character)")
    );
    let rocq = translate_program(source, "JavaScript", "Rocq").expect("translation descriptor");
    assert!(
        rocq.diagnostic()
            .expect("explicit unsupported iterator encoding")
            .message
            .starts_with("string iteration:")
    );
}

// BEGIN modern JavaScript array parity observations
fn modern_array_digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut text, byte| {
            write!(text, "{byte:02x}").expect("String writes");
            text
        })
}

// This adapter transports every public descriptor field; it supplies no semantics.
fn modern_array_translation_record(
    value: &meta_language::program_translation::ProgramTranslation,
) -> serde_json::Value {
    use serde_json::json;
    let contract = value.contract();
    let semantics = value.semantics().map(|record| json!({
        "entry": record.entry,
        "observationProcedure": record.observation_procedure,
        "encodings": record.encodings,
        "assumptions": record.assumptions,
        "obligations": record.obligations.iter().map(|obligation| json!({
            "source": obligation.source, "target": obligation.target, "kind": obligation.kind,
            "closedGoal": obligation.closed_goal, "discharge": obligation.discharge, "check": obligation.check,
        })).collect::<Vec<_>>(),
        "mappings": record.mappings,
        "runtimeDependencies": record.runtime_dependencies,
        "provenance": {
            "translator": record.provenance.translator,
            "sourceLanguage": record.provenance.source_language,
            "sourceSha256": record.provenance.source_sha256,
            "sourceBytes": record.provenance.source_bytes,
            "header": record.provenance.header,
        },
    }));
    let diagnostic = value.diagnostic().map(|record| {
        json!({
            "kind": record.kind, "message": record.message, "span": record.span,
        })
    });
    json!({
        "sourceLanguage": value.source_language(), "targetLanguage": value.target_language(),
        "code": value.code(),
        "contract": {
            "schemaVersion": contract.schema_version, "source": contract.source, "target": contract.target,
            "support": match contract.support {
                meta_language::TranslationSupport::PortableEncoding => "portable-encoding",
                meta_language::TranslationSupport::SemanticSubset => "semantic-subset",
                meta_language::TranslationSupport::SemanticTranslation => "semantic-translation",
            },
            "observation": contract.observation, "requiredRuntime": contract.required_runtime,
            "encoding": contract.encoding, "assumptions": contract.assumptions, "obligation": contract.obligation,
        },
        "semantics": semantics, "diagnostic": diagnostic,
    })
}

#[test]
fn retained_modern_javascript_array_contract_executes_every_original_input() {
    use super::issue_195_observations::{Observation, record};
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::translation::{check::check_program, emit_javascript::emit_javascript};
    use std::fs;
    use std::process::Command;

    const FIXTURE: &str =
        include_str!("../../../parity/fixtures/modern-javascript-array-parity.lino");
    const NAME: &str = "retained_modern_javascript_array_contract_executes_every_original_input";
    let fixture = meta_language::read_structured_data_lino(FIXTURE).unwrap();
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(
        fixture["model"],
        "retained modern JavaScript array parity 1"
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let contract_bytes =
        fs::read(root.join("parity/sources/modern-javascript-parity-expectations-2026-10-16.txt"))
            .unwrap();
    assert_eq!(
        modern_array_digest(&contract_bytes),
        "e2c4a14e4cff20b8bd4e77c1110151f5add84292836591c5eac2c1dc40c141af"
    );
    assert_eq!(
        modern_array_digest(
            fixture["historicalNative"]["source"]
                .as_str()
                .unwrap()
                .as_bytes()
        ),
        "1eec6a923a4d78d79b85fb5da684670f7b2383727539e19b26a363ff8a0bbee2"
    );
    // Fresh actual JavaScript calls and all original-source executions, not a stored emitter snapshot.
    // run() has no per-command timeout. The unchanged physical CI module deadline bounds this test.
    let probe = run(Command::new("node").current_dir(root).args([
        "js/tests/support/modern-javascript-array-parity.js",
        "--probe",
    ]));
    assert!(probe.ok, "{}", probe.stderr);
    let records = meta_language::read_structured_data_records(&probe.stdout).unwrap();
    assert_eq!(records.len(), 12);
    assert_eq!(records[0]["kind"], "metadata");
    assert_eq!(
        records[0]["fixtureDigest"],
        modern_array_digest(FIXTURE.as_bytes())
    );
    assert_ne!(records[0]["nodeVersion"].as_str().unwrap(), "");
    assert_eq!(records[0]["admitted"], 7);
    assert_eq!(records[0]["refused"], 4);
    assert_eq!(fixture["identityCorrection"]["previous"]["admitted"], 6);
    assert_eq!(fixture["identityCorrection"]["previous"]["refused"], 5);
    let rows = fixture["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 11);
    assert_eq!(
        rows.iter()
            .filter(|row| row["origin"] == "original")
            .count(),
        9
    );
    let mut admitted = 0;
    let mut refused = 0;
    for (index, row) in rows.iter().enumerate() {
        let expected_record = &records[index + 1];
        assert_eq!(expected_record["kind"], "case");
        assert_eq!(expected_record["id"], row["id"]);
        assert_eq!(expected_record["sourceSha256"], row["sourceSha256"]);
        let source = row["source"].as_str().unwrap();
        assert_eq!(modern_array_digest(source.as_bytes()), row["sourceSha256"]);
        if row["id"] == "original-array-identity-refusal" {
            assert_modern_array_identity_translation(
                source,
                fixture["identityCorrection"]["previous"]["row"]["expected"]["diagnostic"]
                    .as_str()
                    .unwrap(),
            );
        }
        let targets = row["targets"].as_array().unwrap();
        let expected_targets = expected_record["targets"].as_array().unwrap();
        assert_eq!(targets.len(), expected_targets.len());
        let mut rust_code = None;
        for (target, expected) in targets.iter().zip(expected_targets) {
            let target = target.as_str().unwrap();
            assert_eq!(expected["target"], target);
            let translated = translate_program(source, "JavaScript", target).unwrap();
            assert_eq!(
                modern_array_translation_record(&translated),
                expected["descriptor"],
                "{}:{target}",
                row["id"]
            );
            if row["expected"]["outcome"] == "refused" {
                assert_eq!(target, "Rust");
                let message = &translated
                    .diagnostic()
                    .expect("a genuine retained refusal")
                    .message;
                let expected = row["expected"]["diagnostic"].as_str().unwrap();
                match row["expected"]["match"].as_str().unwrap() {
                    "exact" => assert_eq!(message, expected),
                    "prefix" => assert!(message.starts_with(expected), "{message}"),
                    _ => panic!(
                        "only an independently declared exact or prefix diagnostic is admitted"
                    ),
                }
            } else {
                assert_eq!(row["expected"]["outcome"], "admitted");
                assert!(translated.diagnostic().is_none(), "{}:{target}", row["id"]);
                assert_eq!(
                    translated.contract().support,
                    TranslationSupport::SemanticTranslation
                );
                assert!(!translated.contract().assumptions.contains(&IN_BOUNDS));
                if target == "Rust" {
                    rust_code = Some(translated.code().to_owned());
                }
            }
        }
        if row["expected"]["outcome"] == "refused" {
            refused += 1;
            continue;
        }
        let expected = row["expected"]["stdout"].as_str().unwrap();
        assert_eq!(expected_record["originalStdout"], expected);
        assert_eq!(expected_record["checkedJavaScriptStdout"], expected);
        let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
        let javascript = emit_javascript(&checked).unwrap();
        assert_eq!(
            javascript.text, expected_record["checkedJavaScript"],
            "{}: complete checked JavaScript emission",
            row["id"]
        );
        let directory = TemporaryDirectory::new("retained-modern-array-parity");
        let original_file = directory.0.join("source.mjs");
        let javascript_file = directory.0.join("checked.mjs");
        fs::write(&original_file, source).unwrap();
        fs::write(&javascript_file, javascript.text).unwrap();
        for file in [&original_file, &javascript_file] {
            let output = run(Command::new("node").arg(file));
            assert!(output.ok, "{}: {}", row["id"], output.stderr);
            assert_eq!(output.stdout, expected, "{}", row["id"]);
        }
        let rust_file = directory.0.join("program.rs");
        let executable = directory
            .0
            .join(format!("program{}", std::env::consts::EXE_SUFFIX));
        fs::write(
            &rust_file,
            rust_code.expect("every admitted original target set includes Rust"),
        )
        .unwrap();
        let mut compiler = Command::new("rustc");
        compiler.arg("--edition=2024");
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            compiler.arg("-C").arg(format!("linker={linker}"));
        }
        let build_result = run(compiler.arg(&rust_file).arg("-o").arg(&executable));
        assert!(build_result.ok, "{}: {}", row["id"], build_result.stderr);
        let output = run(&mut Command::new(&executable));
        assert!(output.ok, "{}: {}", row["id"], output.stderr);
        assert_eq!(output.stdout, expected, "{}", row["id"]);
        admitted += 1;
    }
    assert_eq!(admitted, 7);
    assert_eq!(refused, 4);
    // No record is produced until actual native frontends, all preserved refusals,
    // full descriptors, both JS paths and all seven Rust executables pass.
    let modern = [
        "originalArrayAndStringInputsRetained",
        "utf16StringLengthParityExecuted",
        "observableOutOfBoundsParityExecuted",
    ];
    let obsolete = [
        "obsoleteExpectationConflictRecorded",
        "correctedExpectationsFollowModernJavaScript",
        "originalCoverageAndGenuineRefusalsRetained",
    ];
    for (fixture_id, fixture_file) in [
        (
            "conversation-contract:modernJavaScriptParityRequest",
            "parity/sources/modern-javascript-parity-expectations-2026-10-16.txt",
        ),
        (
            "modern-javascript-array-parity-programs",
            "parity/fixtures/modern-javascript-array-parity.lino",
        ),
    ] {
        for (requirement_id, assertions) in [
            (
                "I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR",
                modern.as_slice(),
            ),
            (
                "I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS",
                obsolete.as_slice(),
            ),
        ] {
            record(&Observation {
                requirement_id,
                suffix: "behavior",
                fixture_id,
                fixture_file,
                assertions,
                test_name: NAME,
            });
        }
    }
}

// Same original input and old value-mode diagnostic; native execution is CI only.
fn assert_modern_array_identity_translation(source: &str, previous_message: &str) {
    use super::issue_195_translation_pairs::run;
    use meta_language::translation::{
        check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    };
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    use std::{fs, path::Path, process::Command};
    assert_eq!(
        env!("CARGO_PKG_VERSION"),
        "1.0.0",
        "uncovered crate version; [1.0.0,1.0.1) required"
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let package: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("js/package.json")).unwrap()).unwrap();
    assert_eq!(
        package["version"], "1.0.0",
        "uncovered npm version; [1.0.0,1.0.1) required"
    );
    for (tool, rust) in [("node", false), ("rustc", true)] {
        let actual = run(Command::new(tool).arg("--version"));
        assert!(actual.ok, "{}", actual.stderr);
        let version = if rust {
            actual.stdout.split_whitespace().nth(1).unwrap()
        } else {
            actual.stdout.trim().trim_start_matches('v')
        };
        let parts = version
            .split('.')
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        assert!(
            if rust {
                parts[0] == 1 && (90..100).contains(&parts[1])
            } else {
                parts[0] == 22 && parts[1] >= 15 || parts[0] == 24
            },
            "uncovered {tool} version {version}"
        );
    }
    let policy = meta_language::read_structured_data_lino(include_str!(
        "../../src/data/reference-identity-rules.lino"
    ))
    .unwrap();
    assert_eq!(
        policy["model"],
        "same-domain structural reference identity equality 1"
    );
    assert_eq!(
        policy["sourceClosure"]["sourceSha256"],
        "31c61031a2b2bc4c856f85687f8195bb60f51edc823344711962966df39aae1e"
    );
    assert_eq!(
        modern_array_digest(&fs::read(root.join(policy["sourcePath"].as_str().unwrap())).unwrap()),
        policy["sourceClosure"]["sourceSha256"].as_str().unwrap()
    );
    let original = run(Command::new("node").args(["--input-type=module", "-e", source]));
    assert!(original.ok, "{}", original.stderr);
    assert_eq!(original.stdout, "true\n");
    let value_script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
console.log(JSON.stringify(parseJavaScript(process.argv[1],{referenceSemantics:false})));
";
    let value = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        value_script,
        source,
    ]));
    assert!(value.ok, "{}", value.stderr);
    let value_surface = serde_json::from_str(&value.stdout).unwrap();
    assert_eq!(
        check_program(&value_surface).unwrap_err().to_string(),
        previous_message
    );
    let translated = translate_program(source, "JavaScript", "Rust").unwrap();
    assert!(translated.diagnostic().is_none());
    assert_eq!(
        translated.contract().support,
        TranslationSupport::SemanticTranslation
    );
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    let rust = emit_rust(&checked).unwrap().text;
    let packet = encode_typed_program_links(&checked).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, checked);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
        let output = run(Command::new("node").args([
            "--input-type=module",
            "-e",
            &emit_javascript(&restored).unwrap().text,
        ]));
        assert!(output.ok, "{}", output.stderr);
        assert_eq!(output.stdout, original.stdout);
    }
    // The existing retained corpus compiles/runs the full public Rust output
    // for this newly admitted row, after both source-free codecs above.
}
