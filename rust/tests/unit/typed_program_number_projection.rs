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
