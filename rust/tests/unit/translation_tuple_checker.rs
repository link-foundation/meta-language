use meta_language::translation::{
    check::check_program,
    ir::{Decl, Node},
    primitive_value_rules::{accept_tuple_index, read_tuple_position},
    surface::SProgram,
    types::Type,
};
use serde_json::{Value, json};

fn checked(
    body: Value,
    ret: Value,
) -> meta_language::translation::diagnostics::Result<meta_language::translation::ir::Expr> {
    let surface: SProgram = serde_json::from_value(json!({
        "language": "JavaScript",
        "items": [{"k": "fn", "name": "choose", "params": [], "ret": ret, "body": body}],
        "main": null
    }))
    .unwrap();
    let program = check_program(&surface)?;
    let function = program
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Decl::Fn(function) if function.full_name == "choose" => Some(function),
            _ => None,
        })
        .unwrap();
    Ok(function.body.clone())
}

fn product() -> Value {
    json!({"k": "tuple", "frozen": true, "items": [
        {"k": "num", "value": "5", "type": {"kind": "float"}}, {"k": "bool", "value": true}
    ]})
}

#[test]
fn positional_products_preserve_types_and_safe_constant_projection() {
    let ret = json!({"kind": "tuple", "elements": [{"kind": "float"}, {"kind": "bool"}]});
    let expression = checked(product(), ret).unwrap();
    let Node::Tuple { items, frozen, .. } = &expression.node else {
        panic!("not a product")
    };
    assert!(*frozen);
    assert_eq!(
        items.iter().map(|item| &item.ty).collect::<Vec<_>>(),
        [&Type::Float, &Type::Bool]
    );
    assert_eq!(expression.children().len(), 2);
    let projected = checked(
        json!({"k": "tupleIndex", "object": product(), "index": 1}),
        json!({"kind": "bool"}),
    )
    .unwrap();
    assert_eq!(projected.ty, Type::Bool);
    assert_eq!(projected.children().len(), 1);
    assert!(matches!(projected.node, Node::TupleIndex { index: 1, .. }));
    assert!(
        Type::Tuple {
            elements: vec![
                Type::Bool,
                Type::Tuple {
                    elements: vec![Type::Float, Type::Unit]
                }
            ]
        }
        .immutable_product_type()
    );
    assert!(
        !Type::Tuple {
            elements: vec![Type::String]
        }
        .immutable_product_type()
    );
    assert_ne!(Type::Tuple { elements: vec![] }.key(), Type::Unit.key());
}

#[test]
fn selected_product_model_is_retained_and_refuses_a_contradictory_freeze_edit() {
    let ty = json!({"kind": "tuple", "elements": [{"kind": "float"}, {"kind": "bool"}]});
    let mut source = product();
    source["productModel"] = json!("immutable value product");
    let expression = checked(source.clone(), ty.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(&expression).unwrap()["productModel"],
        "immutable value product"
    );
    assert_eq!(expression.map_children(&mut Clone::clone), expression);
    assert!(
        matches!(&expression.node, Node::Tuple { product_model: Some(model), .. } if model == "immutable value product")
    );
    let round_trip: meta_language::translation::ir::Expr =
        serde_json::from_value(serde_json::to_value(&expression).unwrap()).unwrap();
    assert_eq!(round_trip, expression);
    source["frozen"] = json!(false);
    assert!(checked(source.clone(), ty.clone()).is_err());
    source.as_object_mut().unwrap().remove("productModel");
    assert!(checked(source, ty).is_ok());
    for invalid in [Value::Null, json!(7), json!(true)] {
        let mut malformed = serde_json::to_value(&expression).unwrap();
        malformed["productModel"] = invalid;
        let error = serde_json::from_value::<meta_language::translation::ir::Expr>(malformed)
            .unwrap_err()
            .to_string();
        assert!(error.contains("tuple product model"), "{error}");
    }
}

#[test]
fn products_reject_wrong_shape_and_mutable_inner_values_without_changing_legacy_tuples() {
    assert_eq!(read_tuple_position(-0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(read_tuple_position(1.0), 1.0);
    for index in [
        -1.0,
        0.5,
        2.0,
        f64::NAN,
        f64::INFINITY,
        9_007_199_254_740_992.0,
    ] {
        assert!(!accept_tuple_index(index, 2.0));
    }
    assert!(
        checked(
            json!({"k": "tupleIndex", "object": product(), "index": 2}),
            json!({"kind": "float"})
        )
        .is_err()
    );
    assert!(
        checked(
            product(),
            json!({"kind": "tuple", "elements": [{"kind": "float"}]})
        )
        .is_err()
    );
    assert!(
        checked(
            json!({"k": "tuple", "frozen": true, "items": [{"k": "str", "value": "owned"}]}),
            json!({"kind": "tuple", "elements": [{"kind": "string"}]})
        )
        .is_err()
    );
    let ret =
        json!({"kind": "tuple", "elements": [{"kind": "tuple", "elements": [{"kind": "float"}]}]});
    let mutable_inner = json!({"k": "tuple", "frozen": true, "items": [{"k": "tuple", "items": [{"k": "num", "value": "5", "type": {"kind": "float"}}]}]});
    assert!(checked(mutable_inner, ret).is_err());
    let inner_type =
        json!({"kind": "tuple", "elements": [{"kind": "tuple", "elements": [{"kind": "float"}]}]});
    let wrapped = |frozen| json!({"k": "tuple", "frozen": true, "items": [{"k": "let", "name": "inner", "value": {"k": "tuple", "frozen": frozen, "items": [{"k": "num", "value": "5", "type": {"kind": "float"}}]}, "body": {"k": "name", "path": ["inner"]}}]});
    assert!(checked(wrapped(false), inner_type.clone()).is_err());
    assert!(checked(wrapped(true), inner_type).is_ok());
    let outer_binding = |frozen| json!({"k": "let", "name": "inner", "value": {"k": "tuple", "frozen": frozen, "items": [{"k": "num", "value": "5", "type": {"kind": "float"}}]}, "body": {"k": "tuple", "frozen": true, "items": [{"k": "name", "path": ["inner"]}]}});
    let nested =
        json!({"kind": "tuple", "elements": [{"kind": "tuple", "elements": [{"kind": "float"}]}]});
    assert!(checked(outer_binding(true), nested.clone()).is_ok());
    assert!(checked(outer_binding(false), nested).is_err());
    assert!(
        checked(
            json!({"k": "tuple", "frozen": true, "items": []}),
            json!({"kind": "tuple", "elements": []})
        )
        .is_err()
    );
    let ordinary = checked(json!({"k": "array", "items": [{"spread": false, "value": {"k": "str", "value": "owned"}}]}), json!({"kind": "tuple", "elements": [{"kind": "string"}]})).unwrap();
    assert!(matches!(ordinary.node, Node::Tuple { frozen: false, .. }));
}
