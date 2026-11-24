//! Host construction of the same optional JavaScript constructor field carrier.
use super::Span;
use super::ir::{Expr, Node};
use super::surface::{SExpr, SNode};
use super::types::{Type, option};

pub(super) fn field_type(value: Type) -> Type {
    option(
        Type::Tuple {
            elements: vec![option(value, "undefined")],
        },
        "undefined",
    )
}

pub(crate) fn field_value_type(carrier: &Type) -> Option<&Type> {
    let Type::Option { element, absence } = carrier else {
        return None;
    };
    let Type::Tuple { elements } = element.as_ref() else {
        return None;
    };
    let [
        value @ Type::Option {
            absence: value_absence,
            ..
        },
    ] = elements.as_slice()
    else {
        return None;
    };
    (absence == "undefined" && matches!(value_absence.as_str(), "undefined" | "both"))
        .then_some(value)
}

pub(super) fn surface_projection(carrier: SExpr, span: Option<Span>) -> SExpr {
    let fallback = SExpr::new(
        SNode::Tuple {
            items: vec![SExpr::new(
                SNode::None {
                    absence: "undefined".to_owned(),
                },
                span,
            )],
            frozen: false,
            product_model: None,
        },
        span,
    );
    let product = SExpr::new(
        SNode::Coalesce {
            left: Box::new(carrier),
            right: Box::new(fallback),
        },
        span,
    );
    SExpr::new(
        SNode::TupleIndex {
            object: Box::new(product),
            index: 0,
        },
        span,
    )
}

pub(super) fn read_value(carrier: Expr) -> Expr {
    let Type::Option { element, .. } = &carrier.ty else {
        unreachable!("checked field presence")
    };
    let product = element.as_ref().clone();
    let value = field_value_type(&carrier.ty)
        .expect("checked field presence")
        .clone();
    let fallback = Expr::new(
        Node::Tuple {
            items: vec![Expr::new(
                Node::None {
                    absence: "undefined".to_owned(),
                },
                value.clone(),
            )],
            frozen: false,
            product_model: None,
        },
        product.clone(),
    );
    let product = Expr::new(
        Node::Coalesce {
            left: Box::new(carrier),
            right: Box::new(fallback),
        },
        product,
    );
    Expr::new(
        Node::TupleIndex {
            object: Box::new(product),
            index: 0,
        },
        value,
    )
}

pub(super) fn own_value(value: Expr, ty: &Type) -> Expr {
    let Type::Option { element, .. } = ty else {
        unreachable!("checked field presence")
    };
    let tuple = Expr::new(
        Node::Tuple {
            items: vec![value],
            frozen: false,
            product_model: None,
        },
        element.as_ref().clone(),
    );
    Expr::new(
        Node::Some {
            value: Box::new(tuple),
        },
        ty.clone(),
    )
}
