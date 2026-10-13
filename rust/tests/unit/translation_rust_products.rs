//! Relocated frontend regressions; all original test names and assertions are retained.
use meta_language::translation::{
    Span,
    rust::parse_rust,
    surface::{SItem, SNode},
    types::Type,
};
#[test]
fn translation_rust_products_keep_unit_singleton_and_parenthesised_values_distinct() {
    let source = "fn unit() -> () { () }\nfn one() -> (f64,) { (5.0,) }\n\
        fn scalar() -> (f64) { (5.0) }\nfn main() { let () = unit(); \
        let (value,) = one(); let (grouped) = scalar(); }";
    let surface = parse_rust(source).expect("immutable source products parse");
    let SItem::Fn(unit) = &surface.items[0] else {
        panic!("the unit declaration is a function");
    };
    let SItem::Fn(one) = &surface.items[1] else {
        panic!("the singleton declaration is a function");
    };
    let SItem::Fn(scalar) = &surface.items[2] else {
        panic!("the scalar declaration is a function");
    };
    assert_eq!(unit.ret, Some(Type::Unit));
    assert!(matches!(unit.body.node, SNode::Unit));
    assert_eq!(
        one.ret,
        Some(Type::Tuple {
            elements: vec![Type::Float]
        })
    );
    assert!(matches!(one.body.node, SNode::Tuple { frozen: true, .. }));
    let SNode::Tuple { product_model, .. } = &one.body.node else {
        panic!("the singleton has an immutable product model");
    };
    assert_eq!(product_model.as_deref(), Some("immutable value product"));
    assert_eq!(scalar.ret, Some(Type::Float));
    assert!(matches!(scalar.body.node, SNode::Num { .. }));
    meta_language::translation::check::check_program(&surface).expect("the three forms type check");
}

#[test]
fn translation_rust_product_indices_retain_each_utf16_source_range() {
    let source = "// 🎈\nfn pick(pair: ((u64, bool), bool)) -> u64 { pair.0.0 } \
        fn main() { println!(\"x\"); }";
    let surface = parse_rust(source).expect("nested immutable products parse");
    let SItem::Fn(pick) = &surface.items[0] else {
        panic!("the projection declaration is a function");
    };
    let SNode::TupleIndex { object, index } = &pick.body.node else {
        panic!("the outer field is a positional projection");
    };
    let start = source[..source
        .find("pair.0.0")
        .expect("the source has a projection")]
        .encode_utf16()
        .count();
    assert_eq!(*index, 0);
    assert_eq!(pick.body.span, Some(Span::new(start, start + 8)));
    assert_eq!(object.span, Some(Span::new(start, start + 6)));
    assert!(matches!(object.node, SNode::TupleIndex { index: 0, .. }));
    meta_language::translation::check::check_program(&surface)
        .expect("the nested projection type checks");
}

#[test]
fn translation_rust_product_destructuring_evaluates_once_and_reserves_future_names() {
    let source = "fn make() -> ((u32, bool), ()) { ((5, true), ()) }\n\
        fn main() { let ((value, flag), ()) = make(); let ml_tuple_value0 = 9u32; \
        let ml_tuple_value1 = 10u32; println!(\"{}\", value); }";
    let surface = parse_rust(source).expect("nested irrefutable products parse");
    let value = serde_json::to_value(&surface).expect("the surface serializes");
    let effects = &value["main"]["effects"];
    assert_eq!(effects[0]["name"], "ml_tuple_value2");
    assert_eq!(effects[0]["value"]["path"][0], "make");
    assert_eq!(effects[1]["name"], "value");
    assert_eq!(effects[2]["name"], "flag");
    for index in [1, 2] {
        assert_eq!(
            effects[index]["value"]["object"]["object"]["path"][0],
            effects[0]["name"]
        );
        let name = effects[index]["name"]
            .as_str()
            .expect("the source binder has a name");
        let start = effects[index]["span"]["start"]
            .as_u64()
            .expect("binder start");
        let end = effects[index]["span"]["end"].as_u64().expect("binder end");
        assert_eq!(
            &source[usize::try_from(start).unwrap()..usize::try_from(end).unwrap()],
            name
        );
    }
    meta_language::translation::check::check_program(&surface)
        .expect("the lowered source bindings type check");
}

#[test]
fn translation_rust_product_fields_and_index_spellings_require_source_evidence() {
    let invalid = [
        "fn take(pair: (&u32, bool)) -> bool { pair.1 } fn main() {}",
        "fn take(pair: (Box<u32>, bool)) -> bool { pair.1 } fn main() {}",
        "fn take(pair: (String, bool)) -> bool { pair.1 } fn main() {}",
        "fn main() { let pointer = &1u32; let pair = (pointer, true); }",
        "fn main() { let (value, value) = (1u32, 2u32); }",
        "fn main() { let pair = (1u32, true); let value = pair.01; }",
        "fn main() { let pair = (1u32, true); let value = pair.0_0; }",
        "fn main() { let pair = (1u32, true); let value = pair.0u32; }",
        "fn main() { let pair = (1u32, true); let value = pair.9007199254740992; }",
        "fn main() { let (value,) = (1u32, true); }",
    ];
    for source in invalid {
        assert!(
            parse_rust(source)
                .and_then(|surface| meta_language::translation::check::check_program(&surface))
                .is_err(),
            "unproved source products must be refused: {source}"
        );
    }
}
