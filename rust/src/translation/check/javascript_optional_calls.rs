//! Actual operands/types into core Let/If/Apply/optional/value-record terms.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, Language, Node, Result, SArrayItem, SNode, STRING, Span,
    Type, UNIT, binary_node, type_error, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_optional_call_rules as rules;
use crate::translation::types::TypeField;
fn absent(ty: Type) -> Expr {
    Expr::new(
        Node::None {
            absence: "undefined".to_owned(),
        },
        ty,
    )
}
impl Checker {
    pub(super) fn javascript_optional_call(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript
            || args.is_empty()
            || args.iter().any(|argument| argument.spread)
        {
            return Err(type_error(
                "an optional call needs its actual receiver and ordered non-spread arguments",
                span,
            ));
        }
        let source = &args[0].value;
        let path_length = match &source.node {
            SNode::Name { path } | SNode::FunctionValue { path } => path.len(),
            _ => 0,
        };
        if !rules::accept_java_script_optional_call_head(source.node.kind(), {
            #[allow(clippy::cast_precision_loss)]
            let position = path_length as f64;
            position
        }) {
            return Err(unsupported(
                "optional method call",
                "the property reference needs its complete original receiver and chain extent",
                span,
            ));
        }
        let value = self.expr(source, env, path, None, false)?;
        let value = Self::reference_value(value);
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_optional_call_form(value.ty.kind(), element);
        if form == "unsupported" {
            return Err(unsupported(
                "optional callable",
                "this source call needs its actual callable or nullish value and complete receiver protocol",
                span,
            ));
        }
        let binding = self.fresh_name("ml_optional_call_receiver");
        let field = rules::read_java_script_optional_call_field();
        let ty = Type::Record {
            fields: vec![TypeField {
                name: field.clone(),
                ty: value.ty.clone(),
            }],
            open: false,
            reference: false,
            ordered: false,
        };
        let receiver = Expr::new(
            Node::RecordField {
                object: Box::new(Expr::var(binding.clone(), ty.clone())),
                field: field.clone(),
                index: 0,
            },
            value.ty.clone(),
        );
        let body = if form == "absent" {
            absent(crate::translation::types::option(UNIT, "undefined"))
        } else {
            let signature = match &value.ty {
                Type::Option { element, .. } => element.as_ref().clone(),
                _ => value.ty.clone(),
            };
            let function = if form == "direct" {
                receiver.clone()
            } else {
                Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(receiver.clone()),
                    },
                    signature,
                )
            };
            let arguments = args[1..]
                .iter()
                .map(|argument| argument.value.clone())
                .collect::<Vec<_>>();
            let call = self.checked_callable_application(function, &arguments, env, path, span)?;
            if form == "direct" {
                call
            } else {
                let ret = call.ty.clone();
                let outer = crate::translation::types::option(ret.clone(), "undefined");
                let result_form = rules::read_java_script_optional_call_result_form(ret.kind());
                let success = if result_form == "undefined" {
                    self.coerce(call, &outer, span)?
                } else {
                    Expr::new(
                        Node::Some {
                            value: Box::new(call),
                        },
                        outer.clone(),
                    )
                };
                let presence = Expr::new(
                    Node::OptionalTypeOf {
                        arg: Box::new(receiver),
                        present: "function".to_owned(),
                    },
                    STRING,
                );
                let literal = super::text_lit(STRING, "function");
                let condition = Expr::new(
                    binary_node(
                        BinaryOp::Eq,
                        Box::new(presence),
                        Box::new(literal),
                        Some(STRING),
                        None,
                        None,
                        None,
                    ),
                    BOOL,
                );
                let branch = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(success),
                        otherwise: Box::new(absent(outer.clone())),
                    },
                    outer,
                );
                if result_form == "flatten" {
                    super::optional_values::flatten_option(branch)
                } else {
                    branch
                }
            }
        };
        let result = body.ty.clone();
        let record = Expr::new(
            Node::Record {
                entries: vec![RecordEntry { name: field, value }],
                order: None,
            },
            ty,
        );
        Ok(Expr::new(
            Node::Let {
                name: binding,
                value: Box::new(record),
                body: Box::new(body),
            },
            result,
        ))
    }
}
