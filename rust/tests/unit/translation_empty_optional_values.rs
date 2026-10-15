//! Exact empty optional values retain their type and absence through native adapters.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Node};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::types::Type;
use meta_language::{
    LinkNetwork, create_typed_program_links, decode_typed_program_links, encode_typed_program_links,
};
use std::fs;
use std::process::Command;

#[test]
fn the_empty_value_preserves_its_number_element_and_undefined_policy() {
    let surface = parse_javascript("function empty(){return undefined;}").unwrap();
    let program = check_program(&surface).unwrap();
    let Some(Decl::Fn(function)) = program.declaration("empty") else {
        panic!("the actual empty declaration exists")
    };
    let expected = Type::Option {
        element: Box::new(Type::Float),
        absence: "undefined".into(),
    };
    assert_eq!(function.ret, expected);
    assert_eq!(function.body.ty, expected);
    assert_eq!(
        function.body.node,
        Node::None {
            absence: "undefined".into()
        }
    );
    assert_eq!(
        function.body.children(),
        [] as [&meta_language::translation::ir::Expr; 0]
    );
    let unchanged = function
        .body
        .map_children(&mut |_| panic!("an empty optional value has no operands"));
    assert_eq!(unchanged, function.body);
    let encoded = encode_typed_program_links(&program).unwrap();
    let network = LinkNetwork::from_lino(&encoded.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&network, encoded.root).unwrap(),
        program
    );
    let imported =
        create_typed_program_links("function empty(){return undefined;}", "JavaScript").unwrap();
    assert_eq!(
        decode_typed_program_links(&imported.network, imported.root).unwrap(),
        program
    );
    assert!(
        emit_javascript(&program)
            .unwrap()
            .definitions
            .join("\n")
            .contains("return undefined;")
    );
    assert!(emit_lean(&program).unwrap().text.contains("none"));
    assert!(emit_rocq(&program).unwrap().text.contains("(@None float)"));
    let rust = emit_rust(&program).unwrap().definitions.join("\n\n");
    assert!(rust.contains("None::<f64>"));
    let directory = TemporaryDirectory::new("empty-optional-number");
    let source = directory.0.join("empty.rs");
    let binary = directory.0.join("empty");
    fs::write(
        &source,
        format!("{rust}\n#[test] fn exact_empty_value(){{assert!(empty().is_none());}}\n"),
    )
    .unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg("--test")
        .arg(&source)
        .arg("-o")
        .arg(&binary));
    assert!(
        compiled.ok,
        "empty value compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(Command::new(&binary).args(["--exact", "exact_empty_value"]));
    assert!(
        observed.ok,
        "empty value observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert!(observed.stdout.contains("1 passed"), "{}", observed.stdout);
}
#[test]
fn empty_tags_elements_and_source_owner_shadowing_keep_their_boundary() {
    let packet =
        create_typed_program_links("function empty(){return undefined;}", "JavaScript").unwrap();
    let original = decode_typed_program_links(&packet.network, packet.root).unwrap();
    for (element, absence) in [
        (Type::Bool, "undefined"),
        (Type::Float, "null"),
        (Type::Float, "both"),
    ] {
        let mut forged = original.clone();
        let Decl::Fn(function) = &mut forged.declarations[0] else {
            unreachable!()
        };
        let ty = Type::Option {
            element: Box::new(element),
            absence: absence.into(),
        };
        function.ret = ty.clone();
        function.body.ty = ty;
        function.body.node = Node::None {
            absence: absence.into(),
        };
        assert!(encode_typed_program_links(&forged).is_err());
    }
    assert!(parse_javascript("function empty(){return null;}").is_err());
    assert!(parse_javascript("function empty(){return undefined;const undefined=1;}").is_err());
    let shadowed = check_program(
        &parse_javascript("function shadowed(undefined){return undefined;}").unwrap(),
    )
    .unwrap();
    let Some(Decl::Fn(function)) = shadowed.declaration("shadowed") else {
        unreachable!()
    };
    assert!(matches!(function.body.node, Node::Var { .. }));
}
