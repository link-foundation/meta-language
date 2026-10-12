//! Original tagged prototype syntax stays in the surface AST until refusal.
use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SItem, SNode, SProgram};
use serde_json::Value;

#[test]
fn tagged_prototype_definition_preserves_original_fixture_and_key_location() {
    let fixtures: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-stages.lino"
    ))
    .unwrap();
    let source = fixtures["programs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            case["name"]
                .as_str()
                .is_some_and(|name| name.ends_with("accept-ctor-proto-field"))
        })
        .unwrap()["source"]
        .as_str()
        .unwrap();
    let parsed = parse_javascript(source).expect("the original tagged object is recognized");
    let function = parsed
        .items
        .iter()
        .find_map(|item| match item {
            SItem::Fn(function) if function.name == "f" => Some(function),
            _ => None,
        })
        .unwrap();
    let SNode::CtorObject {
        fields,
        prototype_definition: Some(definition),
        ..
    } = &function.body.node
    else {
        panic!("the prototype definition must remain in the surface constructor");
    };
    assert!(!fields.iter().any(|(name, _)| name == "__proto__"));
    assert_eq!(definition.index, 1, "the tag occupies property zero");
    assert_eq!(
        &source[definition.span.start..definition.span.end],
        "__proto__"
    );
    assert!(matches!(&definition.value.node, SNode::CtorObject { tag, .. } if tag == "leaf"));
    let serialized = serde_json::to_value(&parsed).unwrap();
    let body = &serialized["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["name"] == "f")
        .unwrap()["body"];
    assert_eq!(body["prototypeDefinition"]["index"], 1);
    assert_eq!(body["prototypeDefinition"]["value"]["tag"], "leaf");
    let replay: SProgram = serde_json::from_value(serialized).unwrap();
    assert_eq!(
        replay, parsed,
        "surface serialization retains the initializer and position"
    );
    let refusal = check_program(&replay).unwrap_err();
    assert_eq!(refusal.kind, ErrorKind::Unsupported);
    assert_eq!(
        refusal.construct.as_deref(),
        Some("object prototype definition")
    );
    assert_eq!(refusal.span, Some(definition.span));
}
