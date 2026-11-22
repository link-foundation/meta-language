//! Actual typed owners for the automatically translated record-write rules.
use super::{
    Checker, Env, Expr, Node, Result, SArrayItem, SExpr, SNode, Span, Type, UNIT, type_error,
    unsupported,
};
use crate::translation::record_write_source_interfaces as rules;

impl Checker {
    pub(super) fn check_record_field_write(
        &mut self,
        object: Expr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let operator = rules::read_record_field_write_operator(operation);
        let name = rules::read_record_field_write_name(operation);
        let (field, open) = match &object.ty {
            Type::Record { fields, open, .. } => (
                fields.iter().find(|field| field.name == name).cloned(),
                *open,
            ),
            _ => (None, false),
        };
        if field.as_ref().is_some_and(|field| field.optional) && name == "__proto__" {
            return Err(unsupported(
                "shared record field assignment",
                "a potentially inherited prototype setter requires the actual prototype operation",
                span,
            ));
        }
        let field_value_type = |field: &crate::translation::types::TypeField| {
            if field.optional {
                crate::translation::javascript_constructor_presence::field_value_type(&field.ty)
                    .expect("checked optional own-field carrier")
                    .clone()
            } else {
                field.ty.clone()
            }
        };
        let value_type = field.as_ref().map(field_value_type);
        let optional_operand = match (&field, &value_type) {
            (Some(field), Some(Type::Option { element, absence })) if field.optional => {
                let operation = rules::read_optional_record_compound_binary_operation(
                    self.language.as_str(),
                    &operator,
                    element.kind(),
                    absence,
                );
                (!operation.is_empty()).then_some((operation, element.as_ref()))
            }
            _ => None,
        };
        let refusal = rules::read_record_field_source_write_refusal(
            self.language.as_str(),
            &operator,
            object.ty.kind(),
            object.ty.is_reference(),
            open,
            field.is_some(),
            optional_operand.as_ref().map_or_else(
                || value_type.as_ref().map_or("", Type::kind),
                |(_, element)| element.kind(),
            ),
        );
        if !refusal.is_empty() {
            return Err(unsupported(
                "shared record field assignment",
                &refusal,
                span,
            ));
        }
        let count = serde_json::Number::from(args.len())
            .as_f64()
            .expect("an actual arity is a Number");
        if count != rules::read_record_field_write_argument_count(&operator)
            || args.iter().any(Option::is_none)
        {
            return Err(type_error(
                "a field assignment must retain every actual operand",
                span,
            ));
        }
        let field = field.expect("the shared field rule retained its actual owner");
        let write_type = value_type
            .as_ref()
            .expect("the shared field rule retained its actual value type");
        let (binary, operand_type, optional) = if let Some((operation, element)) = optional_operand
        {
            (operation, element, true)
        } else {
            (
                rules::read_record_compound_binary_operation(
                    self.language.as_str(),
                    &operator,
                    write_type.kind(),
                ),
                write_type,
                false,
            )
        };
        if !binary.is_empty() {
            let mut inner = env.clone();
            let mut bindings = Vec::new();
            let owner_name = loop {
                let name = self.fresh_name("ml_field_owner");
                if !inner.has(&name) {
                    break name;
                }
            };
            inner.bind_local(&owner_name, object.ty.clone());
            let owner = Expr::new(
                Node::Var {
                    name: owner_name.clone(),
                },
                object.ty.clone(),
            );
            let index = match &object.ty {
                Type::Record { fields, .. } => fields
                    .iter()
                    .position(|value| value.name == name)
                    .expect("the own field was retained"),
                _ => unreachable!("the shared rule requires its actual record"),
            };
            bindings.push((owner_name, object));
            let previous_name = loop {
                let name = self.fresh_name("ml_field_previous");
                if !inner.has(&name) {
                    break name;
                }
            };
            let carrier = Expr::new(
                Node::RecordField {
                    object: Box::new(Self::reference_value(owner.clone())),
                    field: name.clone(),
                    index,
                },
                field.ty,
            );
            let previous = if optional {
                crate::translation::javascript_constructor_presence::read_value(carrier)
            } else {
                carrier
            };
            inner.bind_local(&previous_name, previous.ty.clone());
            bindings.push((previous_name.clone(), previous));
            let mut left = SExpr::name(&previous_name);
            if optional {
                let fallback = if operand_type.kind() == "float" {
                    SNode::Num {
                        value: "NaN".to_owned(),
                        ty: Some(operand_type.clone()),
                        negative: false,
                        unit: false,
                    }
                } else {
                    SNode::Str {
                        value: "undefined".to_owned(),
                    }
                };
                left = SExpr::new(
                    SNode::Coalesce {
                        left: Box::new(left),
                        right: Box::new(SExpr::new(fallback, span)),
                    },
                    span,
                );
            }
            let right = args
                .first()
                .and_then(Option::as_ref)
                .cloned()
                .unwrap_or_else(|| {
                    SExpr::new(
                        SNode::Num {
                            value: "1".to_owned(),
                            ty: Some(operand_type.clone()),
                            negative: false,
                            unit: false,
                        },
                        span,
                    )
                });
            let calculated = if binary == "exponentiation" {
                self.javascript_exponentiation(
                    &[SArrayItem::new(left, false), SArrayItem::new(right, false)],
                    span,
                    &inner,
                    path,
                    Some(operand_type),
                )?
            } else {
                let op = serde_json::from_value(serde_json::Value::String(binary))
                    .expect("the source rule supplies a registered surface binary operation");
                self.binary(
                    op,
                    &left,
                    &right,
                    None,
                    span,
                    &inner,
                    path,
                    Some(operand_type),
                    false,
                )?
            };
            let calculated = self.coerce(calculated, write_type, span)?;
            let body = Expr::new(
                Node::Method {
                    object: Box::new(owner),
                    op: rules::read_record_field_write_identity("=", &name),
                    args: vec![Some(calculated)],
                },
                UNIT,
            );
            return Ok(super::array_source_operands::wrap_array_source_operands(
                body, bindings,
            ));
        }
        let value_type = value_type.expect("the shared field rule retained its actual value type");
        let mut values = Vec::new();
        for argument in args {
            let argument = argument.as_ref().expect("the actual operand was retained");
            let value = self.expr(argument, env, path, Some(&value_type), false)?;
            values.push(Some(self.coerce(
                value,
                &value_type,
                argument.span.or(span),
            )?));
        }
        Ok(Expr::new(
            Node::Method {
                object: Box::new(object),
                op: operation.to_owned(),
                args: values,
            },
            UNIT,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn source_record_field_write(
        &mut self,
        object: &SExpr,
        index: Option<&SExpr>,
        field: Option<&str>,
        value: Option<&SExpr>,
        operator: &str,
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Expr> {
        let literal_key = index.and_then(|index| match &index.node {
            SNode::Str { value } if self.language.as_str() == "JavaScript" => Some(value.as_str()),
            _ => None,
        });
        let Some(field) = literal_key.or_else(|| field.filter(|_| index.is_none())) else {
            return Err(unsupported(
                "shared indexed assignment",
                "indexed assignment needs live slots, holes, length and property-key semantics",
                span,
            ));
        };
        let object = self.expr(object, env, path, None, false)?;
        if literal_key.is_some() && !matches!(object.ty, Type::Record { .. }) {
            return Err(unsupported(
                "shared indexed assignment",
                "indexed assignment needs live slots, holes, length and property-key semantics",
                span,
            ));
        }
        let operation = rules::read_record_field_write_identity(operator, field);
        if operation.is_empty() {
            return Err(unsupported(
                "shared record field assignment",
                "this operator needs its complete state operation",
                span,
            ));
        }
        let arguments = value
            .map(|value| vec![Some(value.clone())])
            .unwrap_or_default();
        self.check_record_field_write(object, &operation, &arguments, env, path, span)
    }
}
