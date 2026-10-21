//! Source-selected metadata and the existing checked allocate/read boundary.

use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Node};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::SurfaceRawMetadata;
use meta_language::translation::surface_reference_analysis_rules::{
    accept_surface_reference_closure_token, accept_surface_reference_mutating_method,
    render_surface_reference_operation, require_surface_reference_token,
};
use meta_language::{LinkNetwork, decode_typed_program_links, encode_typed_program_links};
use serde_json::{Value, json};

const SELECTOR: &str = "const marker=()=>0; function select(){let value=0;value+=1;return value;}";

#[test]
fn actual_source_selection_recursively_marks_nested_structural_types() {
    let source = "/** @param {{items: Map<string, Set<number[]>>}} value @returns {{items: Map<string, Set<number[]>>}} */ function identity(value){return value;}";
    let plain = parse_javascript(source).unwrap();
    assert!(plain.reference_semantics.is_none());
    let selected = parse_javascript(&format!("{source}{SELECTOR}")).unwrap();
    assert_eq!(
        selected
            .reference_semantics
            .as_ref()
            .and_then(SurfaceRawMetadata::as_boolean),
        Some(true)
    );
    let value = serde_json::to_value(&selected).unwrap();
    for position in [
        &value["items"][0]["params"][0]["type"],
        &value["items"][0]["ret"],
    ] {
        assert_eq!(position["kind"], "record");
        assert_eq!(position["reference"], true);
        let map = &position["fields"][0]["type"];
        assert_eq!(map["kind"], "map");
        assert_eq!(map["reference"], true);
        assert_eq!(map["value"]["kind"], "set");
        assert_eq!(map["value"]["reference"], true);
        assert_eq!(map["value"]["element"]["kind"], "array");
        assert_eq!(map["value"]["element"]["reference"], true);
        assert!(map["key"].get("reference").is_none());
    }
}

#[test]
fn late_structural_equality_selection_marks_actual_types_and_preserves_reference_identity() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::{
        BinaryLinksOptions, decode_link_network_binary, encode_link_network_binary,
    };
    use sha2::{Digest, Sha256};
    use std::{fs, process::Command};

    // This unreleased implementation epoch keeps the exact predecessor input.
    // The previous full test, missing-reference assertion and refusal remain in
    // the certified source archive; current behavior requires the actual new policy.
    assert_eq!(
        env!("CARGO_PKG_VERSION"),
        "1.0.0",
        "uncovered package version; supported [1.0.0,1.0.1)"
    );
    let profile = meta_language::read_structured_data_lino(include_str!(
        "../../src/data/reference-identity-rules.lino"
    ))
    .unwrap();
    assert_eq!(
        profile["model"],
        "same-domain structural reference identity equality 1"
    );
    assert_eq!(
        profile["assertionVersions"]["Node"],
        ">=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
    );
    assert_eq!(profile["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
    assert_eq!(
        profile["sourceClosure"]["sourceSha256"],
        "31c61031a2b2bc4c856f85687f8195bb60f51edc823344711962966df39aae1e",
        "the explicit current implementation source epoch is required"
    );
    let definition = profile["sourceClosure"]["source"].as_str().unwrap();
    assert_eq!(
        definition.len(),
        usize::try_from(profile["sourceClosure"]["bytes"].as_u64().unwrap()).unwrap()
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(definition.as_bytes())),
        profile["sourceClosure"]["sourceSha256"].as_str().unwrap()
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
            .map(|part| part.parse::<u32>().unwrap())
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
    let source = "function same(left,right){return left===right;} console.log(same([1],[1]));";
    let original = observe(source);
    assert!(original.status.success());
    assert_eq!(original.stdout, b"false\n");
    let surface = parse_javascript(source).unwrap();
    let value = serde_json::to_value(&surface).unwrap();
    assert_eq!(value["referenceSemantics"], true);
    assert_eq!(value["items"][0]["params"][0]["type"]["kind"], "array");
    assert_eq!(value["items"][0]["params"][0]["type"]["reference"], true);
    let program = check_program(&surface).unwrap();
    let Decl::Fn(function) = program.declaration("same").unwrap() else {
        panic!("the actual function remains");
    };
    let Node::Binary {
        left,
        right,
        domain: Some(domain),
        ..
    } = &function.body.node
    else {
        panic!("reference equality retains its actual Binary domain");
    };
    assert!(left.ty.is_reference());
    assert!(right.ty.is_reference());
    assert!(domain.is_reference());
    assert_eq!(&left.ty, domain);
    assert_eq!(left.ty, right.ty);
    let javascript = emit_javascript(&program).unwrap().text;
    let translated = observe(&javascript);
    assert_eq!(translated.status.code(), original.status.code());
    assert_eq!(translated.stdout, original.stdout);
    assert_eq!(translated.stderr, original.stderr);
    let rust = emit_rust(&program).unwrap().text;
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, program);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
        let replay = observe(&emit_javascript(&restored).unwrap().text);
        assert_eq!(replay.status.code(), original.status.code());
        assert_eq!(replay.stdout, original.stdout);
        assert_eq!(replay.stderr, original.stderr);
    }
    // CI only: compile the complete actual emitted program once after both
    // archives have independently required its exact bytes.
    let directory = TemporaryDirectory::new("late-reference-identity");
    let input = directory.0.join("observed.rs");
    let binary = directory
        .0
        .join(format!("observed{}", std::env::consts::EXE_SUFFIX));
    fs::write(&input, &rust).unwrap();
    let mut command = Command::new("rustc");
    command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        command.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(command.arg(&input).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let native = run(&mut Command::new(binary));
    assert!(native.ok, "{}", native.stderr);
    assert_eq!(native.stdout.as_bytes(), original.stdout.as_slice());
}

#[test]
fn source_selected_allocation_and_member_reads_survive_source_free_checked_replay() {
    let source = "/** @param {number[]} values @returns {number[][]} */ function nested(values){return [values];}/** @param {number[]} values @returns {number} */ function size(values){return values.length;}";
    let surface = parse_javascript(&format!("{source}{SELECTOR}")).unwrap();
    let checked = check_program(&surface).unwrap();
    assert!(checked.reference_semantics);
    let Decl::Fn(nested) = checked.declaration("nested").unwrap() else {
        panic!("the actual function is retained")
    };
    let Node::ReferenceAllocate { value } = &nested.body.node else {
        panic!("the selected array is allocated")
    };
    assert!(nested.ret.is_reference());
    assert!(!value.ty.is_reference());
    assert!(value.ty.element().unwrap().is_reference());
    let Decl::Fn(size) = checked.declaration("size").unwrap() else {
        panic!("the actual function is retained")
    };
    let Node::Length { array } = &size.body.node else {
        panic!("the source member read is checked")
    };
    assert!(matches!(array.node, Node::ReferenceRead { .. }));
    assert!(!array.ty.is_reference());
    let encoded = serde_json::to_value(&size.body).unwrap();
    assert_eq!(
        encoded["array"]["type"].get("reference"),
        Some(&json!(false))
    );
    let mut missing = encoded["array"].clone();
    missing["type"].as_object_mut().unwrap().remove("reference");
    assert!(serde_json::from_value::<meta_language::translation::ir::Expr>(missing).is_err());
    let packet = encode_typed_program_links(&checked).unwrap();
    let restored = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&restored, packet.root).unwrap(),
        checked
    );
    let javascript = emit_javascript(&checked).unwrap();
    assert!(javascript.text.contains(".length"));
    let rust = emit_rust(&checked).unwrap();
    assert!(
        rust.text
            .contains("std::rc::Rc::new(std::cell::RefCell::new(")
    );
    assert!(rust.text.contains("ml_reference_value1"));
    assert!(rust.text.contains(".borrow().clone()"));
}

#[test]
fn generated_token_and_render_decisions_retain_existing_source_boundaries() {
    assert!(accept_surface_reference_closure_token("punct", "=>", ""));
    assert!(!accept_surface_reference_closure_token("string", "=>", ""));
    assert!(accept_surface_reference_mutating_method(
        "push",
        &["push".to_owned()]
    ));
    let window = ["punct", "=", "identifier", "field", ".", "number", "1", ";"].map(str::to_owned);
    assert!(require_surface_reference_token(&window, false, false));
    let declaration = [
        "punct",
        "=",
        "identifier",
        "value",
        "const",
        "number",
        "1",
        ";",
    ]
    .map(str::to_owned);
    assert!(!require_surface_reference_token(&declaration, true, false));
    assert_eq!(
        render_surface_reference_operation(
            "JavaScript",
            "referenceRead",
            "original".to_owned(),
            "unused"
        ),
        "original"
    );
    assert_eq!(
        render_surface_reference_operation(
            "Rust",
            "referenceRead",
            "owner".to_owned(),
            "temporary"
        ),
        "{ let temporary = owner.borrow().clone(); temporary }"
    );
    let expected: Value = json!({"kind":"array","element":{"kind":"bool"},"reference":true});
    let ty: meta_language::translation::types::Type =
        serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(ty.with_reference(false).with_reference(true)).unwrap(),
        expected
    );
}

#[test]
fn forged_checked_receiver_and_allocation_flags_retain_exact_rust_emission_refusals() {
    let source = "/** @param {number[]} values @returns {number} */ function size(values){return values.length;} function numbers(){return [1];}";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    let mut receiver = checked.clone();
    let Decl::Fn(size) = receiver
        .declarations
        .iter_mut()
        .find(|declaration| declaration.full_name() == "size")
        .unwrap()
    else {
        panic!("the actual size function is retained")
    };
    let Node::Length { array } = &mut size.body.node else {
        panic!("the actual receiver read is checked")
    };
    array.ty = array.ty.with_reference(true);
    let receiver_span = size.body.span;
    let error = emit_rust(&receiver).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert_eq!(
        error.reason,
        "reference operation: this operation needs the shared reference and slot runtime before target emission"
    );
    assert_eq!(error.span, receiver_span);

    let mut allocation = checked;
    let Decl::Fn(numbers) = allocation
        .declarations
        .iter_mut()
        .find(|declaration| declaration.full_name() == "numbers")
        .unwrap()
    else {
        panic!("the actual numbers function is retained")
    };
    assert!(matches!(numbers.body.node, Node::Array { .. }));
    numbers.body.ty = numbers.body.ty.with_reference(true);
    let allocation_span = numbers.body.span;
    let error = emit_rust(&allocation).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert_eq!(
        error.reason,
        "reference allocation: this allocation needs the shared reference and slot runtime before target emission"
    );
    assert_eq!(error.span, allocation_span);
}

#[test]
fn default_selection_excludes_only_actual_eliminated_private_append_operations() {
    let source = "/** @returns {number[]} */ function build(){const values=[1];values.push(2);return values;}";
    let surface = parse_javascript(source).unwrap();
    assert!(surface.reference_semantics.is_none());
    let checked = check_program(&surface).unwrap();
    assert!(!checked.reference_semantics);
    assert!(
        emit_rust(&checked)
            .unwrap()
            .text
            .contains("ml_array::append(")
    );
    let packet = encode_typed_program_links(&checked).unwrap();
    let restored = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&restored, packet.root).unwrap(),
        checked
    );

    let mutation = format!(
        "{source}/** @param {{number[]}} other @returns {{number[]}} */ function mutate(other){{other[0]=1;return other;}}"
    );
    let selected = parse_javascript(&mutation).unwrap();
    assert_eq!(
        selected
            .reference_semantics
            .as_ref()
            .and_then(SurfaceRawMetadata::as_boolean),
        Some(true)
    );
    let cells = format!("{source}{SELECTOR}");
    let selected = parse_javascript(&cells).unwrap();
    assert_eq!(
        selected
            .reference_semantics
            .as_ref()
            .and_then(SurfaceRawMetadata::as_boolean),
        Some(true)
    );
}
