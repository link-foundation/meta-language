//! Actual source arguments/types into ordinary Boolean/value/error IR.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, Language, LitValue, Node, Result, SArrayItem,
    STRING, Span, Type, UNIT, binary_node, plain_binary, text_lit, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_own_property_rules as rules;
use crate::translation::types::TypeField;
fn equal(left: Expr, right: Expr) -> Expr {
    Expr::new(
        binary_node(
            BinaryOp::Eq,
            Box::new(left),
            Box::new(right),
            Some(STRING),
            None,
            None,
            None,
        ),
        BOOL,
    )
}
fn both(left: Expr, right: Expr) -> Expr {
    plain_binary(BinaryOp::And, left, right, BOOL)
}
fn either(left: Expr, right: Expr) -> Expr {
    plain_binary(BinaryOp::Or, left, right, BOOL)
}
fn boolean(value: bool) -> Expr {
    Expr::lit(BOOL, LitValue::Bool(value))
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
fn failure() -> Expr {
    Expr::new(Node::Math { op: crate::translation::source_exception_rules::read_java_script_source_exception_operation("TypeError"), args: vec![text_lit(STRING, rules::read_java_script_object_coercion_error()), boolean(true)] }, BOOL)
}

impl Checker {
    fn own_property_membership(
        &mut self,
        receiver: Expr,
        key: Expr,
        form: &str,
        span: Option<Span>,
    ) -> Result<Expr> {
        if form == "record" {
            let Type::Record { fields, open, .. } = &receiver.ty else {
                unreachable!("a selected record retains its fields")
            };
            if *open {
                return Err(unsupported(
                    "own property membership",
                    "the actual open record needs its complete runtime property domain",
                    span,
                ));
            }
            return Ok(fields.iter().fold(boolean(false), |result, field| {
                either(
                    result,
                    equal(key.clone(), text_lit(STRING, field.name.clone())),
                )
            }));
        }
        if form == "empty" {
            return Ok(boolean(false));
        }
        if form != "indexed" {
            return Err(unsupported(
                "own property membership",
                "the actual callable, nominal value, proxy or exotic object needs its complete own-property protocol",
                span,
            ));
        }
        let name = self.fresh_name("ml_own_property_index");
        let index = Expr::var(name.clone(), FLOAT);
        let number = self.materialize_number_conversion(key.clone(), "convertJavaScriptNumber");
        let length = match &receiver.ty {
            Type::Tuple { elements } => text_lit(FLOAT, elements.len().to_string()),
            _ => Expr::new(
                Node::Length {
                    array: Box::new(Self::reference_value(receiver.clone())),
                },
                FLOAT,
            ),
        };
        let canonical = equal(
            super::string_search::primitive_text(index.clone(), span)?,
            key.clone(),
        );
        let integral = Expr::new(
            Node::Math {
                op: "isInteger".to_owned(),
                args: vec![index.clone()],
            },
            BOOL,
        );
        let bounds = both(
            comparison(BinaryOp::Ge, index.clone(), text_lit(FLOAT, "0")),
            comparison(BinaryOp::Lt, index.clone(), length),
        );
        let array_bound = if receiver.ty == Type::String {
            boolean(true)
        } else {
            comparison(
                BinaryOp::Lt,
                index,
                text_lit(FLOAT, rules::read_java_script_array_property_index_limit()),
            )
        };
        let selected = either(
            equal(key, text_lit(STRING, "length")),
            both(canonical, both(integral, both(bounds, array_bound))),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(number),
                body: Box::new(selected),
            },
            BOOL,
        ))
    }
    fn converted_own_property(
        &mut self,
        receiver: Expr,
        key: Expr,
        span: Option<Span>,
    ) -> Result<Expr> {
        let element = match &receiver.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_own_property_receiver_form(receiver.ty.kind(), element);
        if form == "absent" {
            return Ok(failure());
        }
        if form == "guarded" {
            let Type::Option { element, .. } = &receiver.ty else {
                unreachable!("a guarded actual receiver has its element")
            };
            let present_type = element.as_ref().clone();
            let condition =
                Self::javascript_optional_presence(receiver.clone(), &present_type, span)?;
            let present = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(receiver),
                },
                present_type,
            );
            let success = self.converted_own_property(present, key, span)?;
            return Ok(Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(success),
                    otherwise: Box::new(failure()),
                },
                BOOL,
            ));
        }
        let name = self.fresh_name("ml_own_property_key");
        let text = super::string_search::primitive_text(key, span)?;
        let ty = Type::Record {
            fields: vec![TypeField {
                name: "key".to_owned(),
                ty: STRING,
            }],
            open: false,
            reference: false,
            ordered: false,
        };
        let read = Expr::new(
            Node::RecordField {
                object: Box::new(Expr::var(name.clone(), ty.clone())),
                field: "key".to_owned(),
                index: 0,
            },
            STRING,
        );
        let body = self.own_property_membership(receiver, read, &form, span)?;
        let record = Expr::new(
            Node::Record {
                entries: vec![RecordEntry {
                    name: "key".to_owned(),
                    value: text,
                }],
                order: None,
            },
            ty,
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(record),
                body: Box::new(body),
            },
            BOOL,
        ))
    }
    pub(super) fn javascript_own_property(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.iter().any(|item| item.spread) {
            return Err(unsupported(
                "Object.hasOwn arguments",
                "the actual argument iterator effects must be represented",
                span,
            ));
        }
        let mut values = args
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        while values.len() < 2 {
            values.push(Expr::new(Node::Unit, UNIT));
        }
        let name = self.fresh_name("ml_own_property_operands");
        let width = (values.len() - 1).to_string().len();
        let field = |index: usize| format!("argument{index:0width$}");
        let ty = Type::Record {
            fields: values
                .iter()
                .enumerate()
                .map(|(index, value)| TypeField {
                    name: field(index),
                    ty: value.ty.clone(),
                })
                .collect(),
            open: false,
            reference: false,
            ordered: false,
        };
        let read = |index: usize| {
            Expr::new(
                Node::RecordField {
                    object: Box::new(Expr::var(name.clone(), ty.clone())),
                    field: field(index),
                    index,
                },
                values[index].ty.clone(),
            )
        };
        let body = self.converted_own_property(read(0), read(1), span)?;
        let record = Expr::new(
            Node::Record {
                entries: values
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| RecordEntry {
                        name: field(index),
                        value,
                    })
                    .collect(),
                order: None,
            },
            ty,
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(record),
                body: Box::new(body),
            },
            BOOL,
        ))
    }
}
