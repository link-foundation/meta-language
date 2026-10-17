//! Paired owned primitive paths and independent native value observations.
//! The compiler invocations here run in CI; local checks use JavaScript only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::ir::{Decl, LitValue, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_primitive_owner_model};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, TypedProgramLinksEmissionOptions,
    create_typed_program_links_with_rust_primitive_owner_model, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, encode_typed_program_links,
};
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-primitive-owners.lino"
    ))
    .unwrap()
}
fn proof<'a>(program: &'a Program, name: &str) -> &'a Value {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("a checked primitive function exists")
    };
    function.source_primitive_owners.as_ref().unwrap()
}
fn case<'a>(data: &'a Value, name: &str) -> &'a Value {
    data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == name)
        .unwrap()
}
fn field(network: &LinkNetwork, owner: LinkId, label: &str) -> LinkId {
    let label = match label {
        "declarations" => "declaration mapping",
        "params" => "parameters",
        other => other,
    };
    network
        .links()
        .find_map(|link| match link.references() {
            [parent, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *parent == owner
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap()
}
fn archive(case: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(case["root"].as_u64().unwrap()),
    )
}

#[test]
fn actual_standard_owners_and_ordered_import_namespaces_survive_paired_archives() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    let signature = data["signatureModel"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 45);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let surface = parse_rust_with_primitive_owner_model(
            case["source"].as_str().unwrap(),
            model,
            signature,
        )
        .unwrap_or_else(|failure| panic!("{id}: {failure}"));
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let checked = check_program(&surface).unwrap();
        let native = create_typed_program_links_with_rust_primitive_owner_model(
            case["source"].as_str().unwrap(),
            "Rust",
            model,
            Some(signature),
            Some(&json!({"project":id})),
        )
        .unwrap();
        let native_network = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        let (shared, shared_root) = archive(case);
        for (network, root) in [(&native_network, native.root), (&shared, shared_root)] {
            let decoded = decode_typed_program_links(network, root).unwrap();
            assert_eq!(proof(&decoded, "apply"), proof(&checked, "apply"), "{id}");
            for target in ["JavaScript", "Rust"] {
                assert_eq!(
                    emit_typed_program_links(network, root, target)
                        .unwrap()
                        .code,
                    case[if target == "Rust" {
                        "rust"
                    } else {
                        "javascript"
                    }]
                    .as_str()
                    .unwrap(),
                    "{id}: {target}"
                );
            }
            assert!(network.links().all(|link| {
                !link
                    .metadata()
                    .definition()
                    .is_some_and(|text| text.contains(case["source"].as_str().unwrap()))
            }));
        }
        assert!(
            parse_rust(case["source"].as_str().unwrap())
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{id}"
        );
    }
    for (id, names) in [
        ("two-alias-source-order", ["Zebra", "Alpha"]),
        ("two-alias-reverse-order", ["Alpha", "Zebra"]),
        ("later-import-source-order", ["Zebra", "Alpha"]),
    ] {
        let surface = parse_rust_with_primitive_owner_model(
            case(&data, id)["source"].as_str().unwrap(),
            model,
            signature,
        )
        .unwrap();
        let checked = check_program(&surface).unwrap();
        for function in ["apply", "other"] {
            assert_eq!(
                proof(&checked, function)["namespace"]["imports"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|alias| alias["name"].as_str().unwrap())
                    .collect::<Vec<_>>(),
                names,
                "{id}: {function}"
            );
        }
    }
}

#[test]
fn every_original_refusal_and_unrepresented_linked_field_stays_outside_the_profile() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    let signature = data["signatureModel"].as_str().unwrap();
    assert_eq!(data["invalid"].as_array().unwrap().len(), 20);
    for case in data["invalid"].as_array().unwrap() {
        let failure = parse_rust_with_primitive_owner_model(
            case["source"].as_str().unwrap(),
            model,
            signature,
        )
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
        assert_eq!(
            failure.kind.as_str(),
            case["kind"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            failure.reason,
            case["reason"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            serde_json::to_value(failure.span).unwrap(),
            case["span"],
            "{}",
            case["id"]
        );
    }
    let original = case(&data, "qualified-standard-floor");
    for (label, value, parameter) in [
        ("generated", Value::Null, false),
        ("generated", json!(0), false),
        ("generated", json!(""), false),
        ("optional", json!(false), true),
        ("optional", Value::Null, true),
        ("span", Value::Null, true),
        ("async", json!(true), false),
        ("unsafe", json!(false), false),
        ("opaque", Value::Null, false),
    ] {
        let (mut network, root) = archive(original);
        let body = network.link(root).unwrap().references()[0];
        let directory = field(&network, body, "declarations");
        let pair = network.link(directory).unwrap().references()[0];
        let function = network.link(pair).unwrap().references()[1];
        let owner = if parameter {
            network
                .link(field(&network, function, "params"))
                .unwrap()
                .references()[0]
        } else {
            function
        };
        let (term, definition) = match value {
            Value::Null => ("null value", String::new()),
            Value::Bool(value) => ("boolean value", value.to_string()),
            Value::Number(value) => ("number value", value.to_string()),
            Value::String(value) => ("string value", value),
            _ => unreachable!(),
        };
        let scalar = network.insert_link(
            [],
            LinkMetadata::new()
                .with_language("meta-language typed program")
                .with_named(true)
                .with_term(term)
                .with_definition(definition),
        );
        network.insert_field(owner, label, scalar);
        let reloaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&reloaded, root, target).is_err(),
                "{label}: {target}"
            );
        }
    }
    let (mut network, root) = archive(original);
    let variable = network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, value]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some("construct kind")
                    && network
                        .link(*value)
                        .and_then(|value| value.metadata().definition())
                        == Some("var") =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .unwrap();
    let boolean = network.insert_link(
        [],
        LinkMetadata::new()
            .with_language("meta-language typed program")
            .with_named(true)
            .with_term("boolean value")
            .with_definition("true"),
    );
    network.insert_field(variable, "borrowEscape", boolean);
    let reloaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    for target in ["JavaScript", "Rust"] {
        assert!(emit_typed_program_links(&reloaded, root, target).is_err());
    }
    assert!(parse_rust_with_primitive_owner_model("fn main(){}", "unknown", "").is_err());
    assert!(
        create_typed_program_links_with_rust_primitive_owner_model(
            "function f(){}",
            "JavaScript",
            model,
            None,
            None
        )
        .is_err()
    );
}

#[test]
fn source_free_owner_contradictions_refuse_and_a_scalar_meaning_edit_keeps_owned_provenance() {
    let data = fixture();
    for (field, replacement) in [
        (
            "namespace",
            json!({"declaredTypeOwners":["Float"],"externalPrimitiveRoots":["std","core"],"imports":[]}),
        ),
        (
            "result",
            json!({"sourcePath":["Float"],"resolvedPath":["crate","Float"],"alias":"Float","wrappers":[],"lifetimes":[]}),
        ),
        ("model", json!("unknown")),
    ] {
        let (network, root) = archive(case(&data, "imported-core-INFINITY"));
        let mut program = decode_typed_program_links(&network, root).unwrap();
        let Decl::Fn(function) = &mut program.declarations[0] else {
            panic!("apply exists")
        };
        function.source_primitive_owners.as_mut().unwrap()[field] = replacement;
        assert!(encode_typed_program_links(&program).is_err(), "{field}");
    }
    let (network, root) = archive(case(&data, "editable-value-body"));
    let mut program = decode_typed_program_links(&network, root).unwrap();
    let before = proof(&program, "apply").clone();
    let Decl::Fn(function) = &mut program.declarations[0] else {
        panic!("apply exists")
    };
    let Node::Binary { right, .. } = &mut function.body.node else {
        panic!("actual addition exists")
    };
    right.node = Node::Lit {
        value: LitValue::Text("2.0".into()),
    };
    let edited = encode_typed_program_links(&program).unwrap();
    let reloaded = LinkNetwork::from_lino(&edited.network.to_lino()).unwrap();
    assert_eq!(
        proof(
            &decode_typed_program_links(&reloaded, edited.root).unwrap(),
            "apply"
        ),
        &before
    );
    let directory = TemporaryDirectory::new("edited-primitive-owner");
    let script = directory.0.join("edited.js");
    fs::write(
        &script,
        format!(
            "{}\nconsole.log(apply(3));",
            emit_typed_program_links(&reloaded, edited.root, "JavaScript")
                .unwrap()
                .code
        ),
    )
    .unwrap();
    let result = run(Command::new("node").arg(script));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout, "5\n");
}

fn harness(case: &Value, program: &Program, original: bool) -> String {
    let Some(Decl::Fn(function)) = program.declaration("apply") else {
        panic!("apply exists")
    };
    let mut code = String::from("\n#[test]\nfn independent_primitive_observations(){\n");
    for row in case["observations"].as_array().unwrap() {
        let arguments = row["javascriptArguments"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let literal = serde_json::to_string(value.as_str().unwrap()).unwrap();
                let borrow = original
                    && !proof(program, "apply")["parameters"][index]["wrappers"]
                        .as_array()
                        .unwrap()
                        .is_empty();
                format!(
                    "{}({literal}.parse::<f64>().unwrap())",
                    if borrow { "&" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        writeln!(&mut code, "let result=apply({arguments});").unwrap();
        if function.ret.kind() == "bool" {
            code.push_str("let observed=result.to_string();\n");
        } else {
            code.push_str("let observed=if result.is_nan(){String::from(\"NaN\")}else{format!(\"{:016x}\",result.to_bits())};\n");
        }
        writeln!(
            &mut code,
            "assert_eq!(observed,{});",
            serde_json::to_string(row["expectedBits"].as_str().unwrap()).unwrap()
        )
        .unwrap();
    }
    code.push_str("}\n");
    code
}

#[test]
fn qualified_owner_facets_compose_with_retained_primitive_reference_headers() {
    let directory = TemporaryDirectory::new("primitive-owner-header");
    let options = TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some("primitive borrowed Rust function header 1".into()),
        ..TypedProgramLinksEmissionOptions::default()
    };
    for (index, (prefix, owner)) in [
        ("", "std::primitive::f64"),
        ("", "core::primitive::f64"),
        ("use std::primitive::f64 as Real;", "Real"),
        ("use core::primitive::f64 as Float;", "Float"),
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!("{prefix}fn apply(value:&{owner})->{owner}{{*value}}fn main(){{}}");
        let input = create_typed_program_links_with_rust_primitive_owner_model(
            &source,
            "Rust",
            "standard primitive Rust binary64 owner 1",
            Some("Rust source function signature 1"),
            None,
        )
        .unwrap();
        let network = LinkNetwork::from_lino(&input.network.to_lino()).unwrap();
        let program = decode_typed_program_links(&network, input.root).unwrap();
        assert_eq!(
            proof(&program, "apply")["parameters"][0]["sourcePath"],
            json!(owner.split("::").collect::<Vec<_>>())
        );
        let emitted = emit_typed_program_links_with_options(&network, input.root, "Rust", &options)
            .unwrap()
            .code;
        assert!(emitted.contains("pub fn apply(value: &f64) -> f64"));
        let observations = json!({"observations":[
            {"javascriptArguments":["-0"],"expectedBits":"8000000000000000"},
            {"javascriptArguments":["-0.5"],"expectedBits":"bfe0000000000000"},
            {"javascriptArguments":["NaN"],"expectedBits":"NaN"},
            {"javascriptArguments":["Infinity"],"expectedBits":"7ff0000000000000"}
        ]});
        for (kind, code) in [("original", source.as_str()), ("header", emitted.as_str())] {
            let path = directory.0.join(format!("header-{index}-{kind}.rs"));
            let binary = directory.0.join(format!(
                "header-{index}-{kind}{}",
                std::env::consts::EXE_SUFFIX
            ));
            fs::write(
                &path,
                format!("{code}\n{}", harness(&observations, &program, true)),
            )
            .unwrap();
            let mut build_command = Command::new("rustc");
            build_command.args([
                "--edition",
                "2024",
                "--test",
                "--crate-name",
                "primitive_owner_header",
            ]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                build_command.arg("-C").arg(format!("linker={linker}"));
            }
            build_command.arg(&path).arg("-o").arg(&binary);
            let compiled = run(&mut build_command);
            assert!(compiled.ok, "{owner}:{kind}: {}", compiled.stderr);
            let executed = run(&mut Command::new(&binary));
            assert!(
                executed.ok,
                "{owner}:{kind}: {} {}",
                executed.stdout, executed.stderr
            );
            assert!(executed.stdout.contains("1 passed"));
        }
    }
}

#[test]
fn independently_compiled_originals_and_linked_rust_preserve_non_nan_bits_and_nan_class() {
    let data = fixture();
    let directory = TemporaryDirectory::new("rust-primitive-owner");
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let (network, root) = archive(case);
        let program = decode_typed_program_links(&network, root).unwrap();
        let emitted = emit_typed_program_links(&network, root, "Rust")
            .unwrap()
            .code;
        for (kind, source, original) in [
            ("original", case["source"].as_str().unwrap(), true),
            ("linked", emitted.as_str(), false),
        ] {
            let path = directory.0.join(format!("{id}-{kind}.rs"));
            let binary = directory
                .0
                .join(format!("{id}-{kind}{}", std::env::consts::EXE_SUFFIX));
            fs::write(
                &path,
                format!("{source}\n{}", harness(case, &program, original)),
            )
            .unwrap();
            let mut build_command = Command::new("rustc");
            build_command.args([
                "--edition",
                "2024",
                "--test",
                "--crate-name",
                "primitive_owner_case",
            ]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                build_command.arg("-C").arg(format!("linker={linker}"));
            }
            build_command.arg(&path).arg("-o").arg(&binary);
            let compiled = run(&mut build_command);
            assert!(compiled.ok, "{id}:{kind}: {}", compiled.stderr);
            let executed = run(&mut Command::new(&binary));
            assert!(
                executed.ok,
                "{id}:{kind}: {} {}",
                executed.stdout, executed.stderr
            );
        }
    }
}
