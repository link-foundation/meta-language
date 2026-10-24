//! Actual literal keys and String operands into ordinary checked field/unit IR.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, Node, Result, SExpr, SNode, STRING, Span, Type,
    binary_node, plain_binary, text_lit, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_literal_property_rules as rules;
use crate::translation::types::{TypeField, option};

pub(super) fn literal_index_source(node: &SExpr, kind: &str) -> Option<SExpr> {
    let SNode::Index {
        object,
        index,
        optional,
        proven_in_bounds,
    } = &node.node
    else {
        return None;
    };
    let SNode::Str { value } = &index.node else {
        return None;
    };
    let units = value.encode_utf16().map(f64::from).collect::<Vec<_>>();
    let position = rules::read_java_script_literal_property_index(&units);
    let source = if rules::read_java_script_literal_property_form(kind, position) == "index" {
        SNode::Index {
            object: object.clone(),
            index: Box::new(SExpr::new(
                SNode::Num {
                    value: position.to_string(),
                    ty: None,
                    negative: false,
                    unit: false,
                },
                index.span,
            )),
            optional: optional.clone(),
            proven_in_bounds: proven_in_bounds.clone(),
        }
    } else {
        SNode::Field {
            object: object.clone(),
            field: value.clone(),
        }
    };
    Some(SExpr::new(source, node.span))
}

fn comparison(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::new(
        binary_node(
            op,
            Box::new(left),
            Box::new(right),
            Some(FLOAT),
            None,
            None,
            None,
        ),
        BOOL,
    )
}
impl Checker {
    fn string_index_number(&mut self, value: Expr, span: Option<Span>) -> Result<Expr> {
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_string_index_number_form(value.ty.kind(), element);
        match form.as_str() {
            "identity" => Ok(value),
            "convert" => Ok(self.materialize_number_conversion(value, "convertJavaScriptNumber")),
            "absent" => Ok(text_lit(FLOAT, "NaN")),
            "optional" => {
                let Type::Option { element, .. } = &value.ty else {
                    unreachable!("a selected optional key has its actual element")
                };
                let present_type = element.as_ref().clone();
                let condition =
                    Self::javascript_optional_presence(value.clone(), &present_type, span)?;
                let present = Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(value),
                    },
                    present_type,
                );
                let number = self.string_index_number(present, span)?;
                Ok(Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(number),
                        otherwise: Box::new(text_lit(FLOAT, "NaN")),
                    },
                    FLOAT,
                ))
            }
            _ => Err(unsupported(
                "indexed String key",
                "the actual property key needs its represented coercion, named property and prototype behavior",
                span,
            )),
        }
    }
    pub(super) fn javascript_string_index(
        &mut self,
        receiver: Expr,
        source: (&SExpr, Option<Span>),
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let (index, span) = source;
        let key = self.expr(index, env, path, None, false)?;
        let capture = self.fresh_name("ml_string_index_operands");
        let captured_type = Type::Record {
            fields: vec![
                TypeField {
                    name: "base".to_owned(),
                    ty: receiver.ty.clone(),
                },
                TypeField {
                    name: "key".to_owned(),
                    ty: key.ty.clone(),
                },
            ],
            open: false,
            reference: false,
        };
        let read = |field: &str, index: usize, ty: Type| {
            Expr::new(
                Node::RecordField {
                    object: Box::new(Expr::var(capture.clone(), captured_type.clone())),
                    field: field.to_owned(),
                    index,
                },
                ty,
            )
        };
        let text = read("base", 0, receiver.ty.clone());
        let raw = read("key", 1, key.ty.clone());
        let name = self.fresh_name("ml_string_index_number");
        let at = Expr::var(name.clone(), FLOAT);
        let number = self.string_index_number(raw, index.span.or(span))?;
        let integral = Expr::new(
            Node::Math {
                op: "isInteger".to_owned(),
                args: vec![at.clone()],
            },
            BOOL,
        );
        let bounds = plain_binary(
            BinaryOp::And,
            comparison(BinaryOp::Ge, at.clone(), text_lit(FLOAT, "0")),
            comparison(
                BinaryOp::Lt,
                at.clone(),
                Expr::new(
                    Node::Length {
                        array: Box::new(text.clone()),
                    },
                    FLOAT,
                ),
            ),
            BOOL,
        );
        let condition = plain_binary(BinaryOp::And, integral, bounds, BOOL);
        let ty = option(STRING, "undefined");
        let character =
            self.materialize_primitive_method(text, "charAt", vec![Some(at)], path, span)?;
        let selected = Expr::new(
            Node::If {
                cond: Box::new(condition),
                then: Box::new(Expr::new(
                    Node::Some {
                        value: Box::new(character),
                    },
                    ty.clone(),
                )),
                otherwise: Box::new(Expr::new(
                    Node::None {
                        absence: "undefined".to_owned(),
                    },
                    ty.clone(),
                )),
            },
            ty.clone(),
        );
        let body = Expr::new(
            Node::Let {
                name,
                value: Box::new(number),
                body: Box::new(selected),
            },
            ty.clone(),
        );
        let record = Expr::new(
            Node::Record {
                entries: vec![
                    RecordEntry {
                        name: "base".to_owned(),
                        value: receiver,
                    },
                    RecordEntry {
                        name: "key".to_owned(),
                        value: key,
                    },
                ],
            },
            captured_type,
        );
        Ok(Expr::new(
            Node::Let {
                name: capture,
                value: Box::new(record),
                body: Box::new(body),
            },
            ty,
        ))
    }
}
