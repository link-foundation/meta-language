//! Actual JavaScript-authored optional arithmetic archives use the same checker.
use meta_language::translation::ir::{Decl, Expr, LitValue, Node, Program};
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, LinkType, TypedProgramLinks,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
    typed_program_links_schema,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/program-number-projection.lino"
    ))
    .unwrap()
}

fn packet(fixture: &Value) -> TypedProgramLinks {
    TypedProgramLinks {
        network: LinkNetwork::from_lino(fixture["links"].as_str().unwrap()).unwrap(),
        root: LinkId::from_u64(fixture["root"].as_u64().unwrap()),
    }
}

fn archives(packet: &TypedProgramLinks) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn observe(source: &str) -> Vec<String> {
    let output = Command::new("node").args(["-e", source]).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

fn edit_arithmetic(expression: &Expr, edit: &mut impl FnMut(&mut Expr)) -> Expr {
    let mut result = expression.map_children(&mut |child| edit_arithmetic(child, edit));
    if matches!(&result.node, Node::Binary { left, .. } if matches!(&left.node, Node::NullishNumber { .. }))
    {
        edit(&mut result);
    }
    result
}

fn edit_program(program: &mut Program, edit: &mut impl FnMut(&mut Expr)) {
    for declaration in &mut program.declarations {
        if let Decl::Fn(function) = declaration {
            function.body = edit_arithmetic(&function.body, edit);
        }
    }
}

#[test]
fn source_free_numeric_operands_recheck_after_both_archives_and_keep_real_value_edits() {
    let fixture = fixture();
    let source = fixture["source"].as_str().unwrap();
    assert_eq!(
        Sha256::digest(source.as_bytes()).iter().fold(
            String::with_capacity(64),
            |mut hexadecimal, byte| {
                use std::fmt::Write as _;
                write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
                hexadecimal
            },
        ),
        fixture["sourceSha256"].as_str().unwrap()
    );
    let expected: Vec<String> = serde_json::from_value(fixture["output"].clone()).unwrap();
    let changed: Vec<String> = serde_json::from_value(fixture["editedOutput"].clone()).unwrap();
    assert_eq!(observe(source), expected);
    let original = packet(&fixture);
    for network in archives(&original) {
        let mut program = decode_typed_program_links(&network, original.root).unwrap();
        assert_eq!(
            observe(
                &emit_typed_program_links(&network, original.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
        let mut edits = 0;
        edit_program(&mut program, &mut |operation| {
            let Node::Binary { right, .. } = &mut operation.node else {
                unreachable!()
            };
            let Node::Lit {
                value: LitValue::Text(value),
            } = &mut right.node
            else {
                unreachable!()
            };
            assert_eq!(
                value.as_str(),
                fixture["edit"]["original"].as_str().unwrap()
            );
            *value = fixture["edit"]["replacement"].as_str().unwrap().to_owned();
            edits += 1;
        });
        assert_eq!(edits, 1);
        let edited = encode_typed_program_links(&program).unwrap();
        for saved in archives(&edited) {
            assert_eq!(
                decode_typed_program_links(&saved, edited.root).unwrap(),
                program
            );
            assert_eq!(
                observe(
                    &emit_typed_program_links(&saved, edited.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                changed
            );
            assert!(emit_typed_program_links(&saved, edited.root, "Rust").is_ok());
        }
    }
}

#[test]
fn standalone_wrappers_and_counterfeit_result_or_child_types_cannot_pass_full_rechecking() {
    let fixture = fixture();
    let original = packet(&fixture);
    for mode in 0..3 {
        let mut program = decode_typed_program_links(&original.network, original.root).unwrap();
        let mut edits = 0;
        edit_program(&mut program, &mut |operation| {
            edits += 1;
            if mode == 0 {
                let Node::Binary { left, .. } = &operation.node else {
                    unreachable!()
                };
                *operation = (**left).clone();
            } else {
                let Node::Binary { left, .. } = &mut operation.node else {
                    unreachable!()
                };
                if mode == 1 {
                    left.ty = Type::Bool;
                } else {
                    let Node::NullishNumber { arg } = &mut left.node else {
                        unreachable!()
                    };
                    arg.ty = Type::Option {
                        element: Box::new(Type::String),
                        absence: "undefined".to_owned(),
                    };
                }
            }
        });
        assert_eq!(edits, 1);
        if mode == 0 {
            // The original recursive edit preserves the Let that owns values.
            // Whole-program rechecking therefore retains this valid operand;
            // only the removed addition changes the observed result to five.
            let Some(Decl::Fn(function)) = program.declaration("probe") else {
                panic!("the original fixture retains its actual probe function")
            };
            assert!(matches!(&function.body.node, Node::Let { name, body, .. }
                if name == "values" && matches!(&body.node, Node::NullishNumber { .. })));
            let edited = encode_typed_program_links(&program).unwrap();
            for saved in archives(&edited) {
                assert_eq!(
                    decode_typed_program_links(&saved, edited.root).unwrap(),
                    program
                );
                assert_eq!(
                    observe(
                        &emit_typed_program_links(&saved, edited.root, "JavaScript")
                            .unwrap()
                            .code
                    ),
                    vec!["5".to_owned()]
                );
                assert!(emit_typed_program_links(&saved, edited.root, "Rust").is_ok());
            }
            continue;
        }
        let failure = encode_typed_program_links(&program)
            .err()
            .unwrap_or_else(|| panic!("counterfeit numeric operand mode {mode} was accepted"))
            .to_string();
        assert!(
            failure.contains("derived number operand") || failure.contains("no surface projection"),
            "{failure}"
        );
    }
    // Match the existing JavaScript whole-body edit: removing the owning Let
    // really does orphan values, and must preserve its source-scope refusal.
    let mut orphan = decode_typed_program_links(&original.network, original.root).unwrap();
    let mut replacement = None;
    let mut edits = 0;
    edit_program(&mut orphan, &mut |operation| {
        let Node::Binary { left, .. } = &operation.node else {
            unreachable!()
        };
        replacement = Some((**left).clone());
        edits += 1;
    });
    assert_eq!(edits, 1);
    let Some(Decl::Fn(function)) = orphan
        .declarations
        .iter_mut()
        .find(|declaration| declaration.full_name() == "probe")
    else {
        panic!("the original fixture retains its actual probe function")
    };
    function.body = replacement.unwrap();
    let failure = encode_typed_program_links(&orphan)
        .err()
        .unwrap_or_else(|| panic!("a whole-body numeric operand with orphaned values was accepted"))
        .to_string();
    assert!(failure.contains("unknown name values"), "{failure}");
    let bound = meta_language::create_typed_program_links(
        "/** @param {number[]} values @returns {number} */ function probe(values){return values[0]+2;}console.log(probe([5]));",
        "JavaScript",
    )
    .unwrap();
    for network in archives(&bound) {
        let mut program = decode_typed_program_links(&network, bound.root).unwrap();
        let mut edits = 0;
        edit_program(&mut program, &mut |operation| {
            let Node::Binary { left, .. } = &operation.node else {
                unreachable!()
            };
            *operation = (**left).clone();
            edits += 1;
        });
        assert_eq!(edits, 1);
        let edited = encode_typed_program_links(&program).unwrap();
        for saved in archives(&edited) {
            assert_eq!(
                decode_typed_program_links(&saved, edited.root).unwrap(),
                program
            );
            assert_eq!(
                observe(
                    &emit_typed_program_links(&saved, edited.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                vec!["5".to_owned()]
            );
            assert!(emit_typed_program_links(&saved, edited.root, "Rust").is_ok());
        }
    }
}

#[test]
fn raw_wrapper_ownership_extensions_are_refused_after_source_free_reload() {
    let fixture = fixture();
    let mut packet = packet(&fixture);
    let schema = typed_program_links_schema();
    let kind = schema["fields"]["k"].as_str().unwrap_or("k");
    let (wrapper, scalar) = packet
        .network
        .links()
        .find_map(|link| match link.references() {
            [owner, label, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && packet
                        .network
                        .link(*label)
                        .and_then(|value| value.metadata().term())
                        == Some(kind)
                    && packet
                        .network
                        .link(*child)
                        .and_then(|value| value.metadata().definition())
                        == Some("nullishNumber") =>
            {
                Some((*owner, *child))
            }
            _ => None,
        })
        .unwrap();
    packet
        .network
        .insert_field(wrapper, "borrow escape", scalar);
    let saved = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert!(decode_typed_program_links(&saved, packet.root).is_err());
    for target in ["JavaScript", "Rust"] {
        assert!(emit_typed_program_links(&saved, packet.root, target).is_err());
    }
}

// CI-only native consumer of the same normally authored Boolean-conversion inputs.
fn truthy_projection_versions() {
    for (tool, rust) in [("rustc", true), ("node", false)] {
        let result = super::issue_195_translation_pairs::run(Command::new(tool).arg("--version"));
        assert!(result.ok, "{}", result.stderr);
        let version = if rust {
            result.stdout.split_whitespace().nth(1).unwrap()
        } else {
            result.stdout.trim().trim_start_matches('v')
        };
        let mut parts = version.split('.');
        let major = parts.next().unwrap().parse::<u32>().unwrap();
        let minor = parts.next().unwrap().parse::<u32>().unwrap();
        assert!(
            if rust {
                major == 1 && (90..100).contains(&minor)
            } else {
                major == 22 && minor >= 15 || major == 24
            },
            "uncovered {tool} version {version}"
        );
    }
}

#[test]
fn source_boolean_conversions_recheck_exactly_through_both_archives_and_execute_in_native_ci() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    truthy_projection_versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/truthy-projection.lino"
    ))
    .unwrap();
    assert_eq!(fixture["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
    assert_eq!(
        fixture["assertionVersions"]["Node"],
        ">=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
    );
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let expected: Vec<String> = serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(observe(source), expected);
        let hash =
            Sha256::digest(source.as_bytes())
                .iter()
                .fold(String::new(), |mut value, byte| {
                    use std::fmt::Write as _;
                    write!(value, "{byte:02x}").unwrap();
                    value
                });
        assert_eq!(hash, case["sourceSha256"].as_str().unwrap());
        let mut actual = meta_language::translation::check::check_program(
            &meta_language::translation::javascript::parse_javascript(source).unwrap(),
        )
        .unwrap();
        actual.utf16_strings = Some(true);
        let root = LinkId::from_u64(case["archive"]["root"].as_str().unwrap().parse().unwrap());
        let text = case["archive"]["text"].as_str().unwrap();
        let hex = case["archive"]["binaryHex"].as_str().unwrap();
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
            .collect();
        let mut rust_source = None;
        for network in [
            LinkNetwork::from_lino(text).unwrap(),
            decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
        ] {
            let restored = decode_typed_program_links(&network, root).unwrap();
            assert_eq!(
                restored, actual,
                "{} complete actual native checked IR",
                case["id"]
            );
            let saved = encode_typed_program_links(&restored).unwrap();
            for again in archives(&saved) {
                assert_eq!(
                    decode_typed_program_links(&again, saved.root).unwrap(),
                    restored
                );
            }
            assert_eq!(
                observe(
                    &emit_typed_program_links(&network, root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected
            );
            let emitted = emit_typed_program_links(&network, root, "Rust")
                .unwrap()
                .code;
            if let Some(previous) = &rust_source {
                assert_eq!(previous, &emitted);
            }
            rust_source = Some(emitted);
        }
        let directory = TemporaryDirectory::new("truthy-projection");
        let file = directory.0.join("actual.rs");
        let binary = directory
            .0
            .join(format!("actual{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&file, rust_source.unwrap()).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&file).arg("-o").arg(&binary));
        assert!(compiled.ok, "{}\n{}", compiled.stdout, compiled.stderr);
        let observed = run(&mut Command::new(binary));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(
            observed
                .stdout
                .lines()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn original_lone_literal_refusal_and_complete_string_parameter_keep_all_original_values_in_native_ci()
 {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::translation::{
        check::check_program,
        emit_rust::{RustEmissionOptions, emit_rust_with_array_consumption},
        javascript::parse_javascript,
    };
    use std::fmt::Write as _;
    truthy_projection_versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/truthy-projection.lino"
    ))
    .unwrap();
    let literal = &fixture["literalBoundary"];
    let source = literal["source"].as_str().unwrap();
    let expected: Vec<String> = serde_json::from_value(literal["expected"].clone()).unwrap();
    assert_eq!(observe(source), expected);
    let failure = parse_javascript(source).unwrap_err();
    assert_eq!(
        failure.kind.as_str(),
        literal["expectedRefusal"]["kind"].as_str().unwrap()
    );
    assert!(
        failure
            .reason
            .contains(literal["expectedRefusal"]["reason"].as_str().unwrap())
    );
    assert_eq!(
        failure.span.unwrap().start as u64,
        literal["expectedRefusal"]["start"].as_u64().unwrap()
    );
    let host = &fixture["hostParameter"];
    let host_source = host["source"].as_str().unwrap();
    assert_eq!(
        host_source,
        source.split('\n').next().unwrap(),
        "the complete original function body is unchanged"
    );
    let values = host["values"].as_array().unwrap();
    let host_expected: Vec<String> = values
        .iter()
        .map(|value| value["expected"].as_bool().unwrap().to_string())
        .collect();
    assert_eq!(
        &host_expected[..6],
        expected.as_slice(),
        "every original literal oracle has the same UTF16 host argument"
    );
    let units: Vec<Vec<u16>> = values
        .iter()
        .map(|value| {
            value["units"]
                .as_array()
                .unwrap()
                .iter()
                .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
                .collect()
        })
        .collect();
    assert_eq!(
        &units[..6],
        &[
            vec![],
            vec![48],
            vec![102, 97, 108, 115, 101],
            vec![0],
            vec![55296],
            vec![55357, 56832]
        ]
    );
    let calls = format!(
        "\nfor(const units of {})console.log(probe(String.fromCharCode(...units)));",
        serde_json::to_string(&units).unwrap()
    );
    assert_eq!(observe(&format!("{host_source}{calls}")), host_expected);
    let mut actual = check_program(&parse_javascript(host_source).unwrap()).unwrap();
    actual.utf16_strings = Some(true);
    let root = LinkId::from_u64(host["archive"]["root"].as_str().unwrap().parse().unwrap());
    let hex = host["archive"]["binaryHex"].as_str().unwrap();
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
        .collect();
    let mut complete_rust = None;
    for network in [
        LinkNetwork::from_lino(host["archive"]["text"].as_str().unwrap()).unwrap(),
        decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, root).unwrap();
        assert_eq!(restored, actual);
        assert_eq!(restored.utf16_strings, Some(true));
        let saved = encode_typed_program_links(&restored).unwrap();
        for again in archives(&saved) {
            assert_eq!(
                decode_typed_program_links(&again, saved.root).unwrap(),
                restored
            );
        }
        assert_eq!(
            observe(&format!(
                "{}{calls}",
                emit_typed_program_links(&network, root, "JavaScript")
                    .unwrap()
                    .code
            )),
            host_expected
        );
        let emitted =
            emit_rust_with_array_consumption(&restored, RustEmissionOptions::default()).unwrap();
        let mapping: Vec<_> = emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == "probe")
            .collect();
        assert_eq!(
            mapping.len(),
            1,
            "the actual normal emitter owns the tested function target"
        );
        let target = &mapping[0].target;
        let mut main = "fn main(){\n".to_owned();
        for value in &units {
            writeln!(
                main,
                "println!(\"{{}}\", {target}(crate::ml_string::Text::from_units(vec!{value:?}))); "
            )
            .unwrap();
        }
        main.push_str("}\n");
        // Use every complete actual normal library block and its actual
        // prelude; only the independent host calls replace the empty driver.
        let complete = emitted
            .preludes
            .iter()
            .chain(emitted.definitions.iter())
            .cloned()
            .chain(std::iter::once(main))
            .collect::<Vec<_>>()
            .join("\n\n");
        if let Some(previous) = &complete_rust {
            assert_eq!(previous, &complete);
        }
        complete_rust = Some(complete);
    }
    let directory = TemporaryDirectory::new("truthy-utf16-host");
    let file = directory.0.join("actual.rs");
    let binary = directory
        .0
        .join(format!("actual{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&file, complete_rust.unwrap()).unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(command.arg(&file).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}\n{}", compiled.stdout, compiled.stderr);
    let observed = run(&mut Command::new(binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(
        observed
            .stdout
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        host_expected
    );
}
