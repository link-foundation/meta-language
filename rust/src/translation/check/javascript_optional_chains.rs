//! Actual receiver/binding/continuation transport into ordinary typed core IR.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, Language, Node, Result, SArrayItem, STRING, Span, Type,
    UNIT, binary_node, type_error, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::javascript_optional_chain_rules as rules;
use crate::translation::types::{TypeField, option};

fn absent(ty: Type) -> Expr {
    Expr::new(
        Node::None {
            absence: "undefined".to_owned(),
        },
        ty,
    )
}
impl Checker {
    pub(super) fn javascript_optional_presence(
        receiver: Expr,
        present: &Type,
        span: Option<Span>,
    ) -> Result<Expr> {
        if rules::read_java_script_optional_chain_presence_form(present.kind()) == "truthy" {
            return Ok(Expr::new(
                Node::Truthy {
                    arg: Box::new(receiver),
                },
                BOOL,
            ));
        }
        let tag = crate::translation::frontend_rules::read_type_query_result(present.kind());
        if tag.is_empty() {
            return Err(unsupported(
                "optional chain receiver",
                "the present source value needs its explicit nullish observation protocol",
                span,
            ));
        }
        let presence = Expr::new(
            Node::OptionalTypeOf {
                arg: Box::new(receiver),
                present: tag.clone(),
            },
            STRING,
        );
        let literal = super::text_lit(STRING, tag);
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
        Ok(condition)
    }

    pub(super) fn javascript_optional_chain(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let malformed = || {
            type_error(
                "an optional chain needs its actual receiver, lexical binding and complete continuation",
                span,
            )
        };
        let argument = args.first().ok_or_else(malformed)?;
        let super::SNode::Let {
            name,
            ty: None,
            value: source,
            body: continuation_source,
        } = &argument.value.node
        else {
            return Err(malformed());
        };
        #[allow(clippy::cast_precision_loss)]
        let argument_count = args.len() as f64;
        if self.language != Language::JavaScript
            || !rules::accept_java_script_optional_chain_operands(
                argument_count,
                args.iter().any(|arg| arg.spread),
                name,
            )
        {
            return Err(type_error(
                "an optional chain needs its actual receiver, lexical binding and complete continuation",
                span,
            ));
        }
        let value = self.expr(source, env, path, None, false)?;
        let element = match &value.ty {
            Type::Option { element, .. } => element.kind(),
            _ => "",
        };
        let form = rules::read_java_script_optional_chain_form(value.ty.kind(), element);
        let binding = self.fresh_name("ml_optional_chain_receiver");
        let field = rules::read_java_script_optional_chain_field();
        let ty = Type::Record {
            fields: vec![TypeField {
                name: field.clone(),
                ty: value.ty.clone(),
            }],
            open: false,
            reference: false,
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
            absent(option(UNIT, "undefined"))
        } else {
            let present = match &value.ty {
                Type::Option { element, .. } if form == "guarded" => element.as_ref().clone(),
                _ => value.ty.clone(),
            };
            let actual = if form == "guarded" {
                Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(receiver.clone()),
                    },
                    present.clone(),
                )
            } else {
                receiver.clone()
            };
            let mut scope = env.clone();
            scope.bind_local(name, present.clone());
            let continuation = self.expr(continuation_source, &scope, path, None, false)?;
            let result = continuation.ty.clone();
            let tail = Expr::new(
                Node::Let {
                    name: name.to_owned(),
                    value: Box::new(actual),
                    body: Box::new(continuation),
                },
                result.clone(),
            );
            if form == "direct" {
                tail
            } else {
                let outer = option(result.clone(), "undefined");
                let result_form = rules::read_java_script_optional_chain_result_form(result.kind());
                let success = if result_form == "undefined" {
                    self.coerce(tail, &outer, span)?
                } else {
                    Expr::new(
                        Node::Some {
                            value: Box::new(tail),
                        },
                        outer.clone(),
                    )
                };
                let condition = Self::javascript_optional_presence(receiver, &present, span)?;
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
