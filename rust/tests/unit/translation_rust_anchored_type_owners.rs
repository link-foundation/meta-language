//! External coverage of actual parsed Rust type and value namespaces.
use meta_language::translation::check::check_program;
use meta_language::translation::ir::{Decl, Node};
use meta_language::translation::rust::{parse_rust, parse_rust_with_source_signature_model};
use meta_language::translation::source_signatures::SourceOwner;
use meta_language::translation::surface::{SItem, SProgram};
use meta_language::translation::types::Type;

fn source_with_header(owner: &str, deep: bool) -> String {
    let header = format!("fn header(value:{owner})->bool{{true}}");
    let declaration = if deep {
        format!("mod deep{{{header}}}")
    } else {
        header
    };
    format!(
        "enum f64{{Only}}mod nested{{fn f64()->bool{{true}}{declaration}fn value()->bool{{f64()}}}}fn main(){{}}"
    )
}

fn parsed_header(owner: &str, deep: bool) -> (SProgram, SourceOwner) {
    let surface = parse_rust_with_source_signature_model(
        &source_with_header(owner, deep),
        "Rust source function signature 1",
    )
    .unwrap();
    let SItem::Module(nested) = surface
        .items
        .iter()
        .find(|item| item.name() == "nested")
        .unwrap()
    else {
        panic!("actual source declares the nested module")
    };
    let container = if deep {
        let SItem::Module(module) = nested
            .items
            .iter()
            .find(|item| item.name() == "deep")
            .unwrap()
        else {
            panic!("actual source declares the deep module")
        };
        module
    } else {
        nested
    };
    let SItem::Fn(header) = container
        .items
        .iter()
        .find(|item| item.name() == "header")
        .unwrap()
    else {
        panic!("actual source declares the header function")
    };
    let owner = header.source_signature.as_ref().unwrap().parameters[0]
        .owner
        .clone();
    (surface, owner)
}

#[test]
fn anchored_type_owner_lookup_never_falls_through_a_value_namespace_hit() {
    // Preserve the original unqualified root-data lookup despite the nested value function.
    let (_, unqualified) = parsed_header("f64", false);
    assert_eq!(unqualified.kind, "declaration");
    assert_eq!(unqualified.path, ["f64"]);

    // An explicit self anchor must not fall back to the root type after a value-only hit.
    let (surface, local) = parsed_header("self::f64", false);
    assert_eq!(local.path, ["self", "f64"]);
    assert_eq!(
        check_program(&surface).unwrap_err().reason,
        "unknown type self.f64"
    );

    // The same guard applies to super from the nested.deep declaration scope.
    let (surface, parent) = parsed_header("super::f64", true);
    assert_eq!(parent.path, ["super", "f64"]);
    assert_eq!(
        check_program(&surface).unwrap_err().reason,
        "unknown type super.f64"
    );

    // An explicit crate anchor resolves the actual root data owner.
    let (surface, root) = parsed_header("crate::f64", false);
    assert_eq!(root.path, ["f64"]);
    let checked = check_program(&surface).unwrap();
    let Some(Decl::Fn(header)) = checked.declaration("nested.header") else {
        panic!("the checked header function exists")
    };
    assert_eq!(header.params[0].ty, Type::Data { name: "f64".into() });

    // The same source's unqualified value call still targets the nested function.
    let checked =
        check_program(&parse_rust(&source_with_header("crate::f64", false)).unwrap()).unwrap();
    let Some(Decl::Fn(function)) = checked.declaration("nested.value") else {
        panic!("the checked value function exists")
    };
    let Node::Call { func, args } = &function.body.node else {
        panic!("actual source contains a checked call")
    };
    assert_eq!(func, "nested.f64");
    assert_eq!(args.as_slice(), []);
}
