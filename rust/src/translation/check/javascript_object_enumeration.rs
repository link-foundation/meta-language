//! Actual source arguments into ordinary order/key definitions and source errors.
use super::{
    Checker, Env, Expr, LitValue, Node, Result, SArrayItem, Span, Type, array, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::types::TypeField;

fn failure(result: &Type) -> Expr {
    Expr::new(Node::Math{op:crate::translation::source_exception_rules::read_java_script_source_exception_operation("TypeError"),args:vec![super::text_lit(Type::String,crate::translation::javascript_own_property_rules::read_java_script_object_coercion_error()),Expr::lit(Type::Bool,LitValue::Bool(true))]},result.clone())
}
impl Checker {
    fn enumerate_object_keys(&mut self, object: Expr, span: Option<Span>) -> Result<Expr> {
        let result = array(Type::String);
        let element = if let Type::Option { element, .. } = &object.ty {
            element.kind()
        } else {
            ""
        };
        let form=crate::translation::javascript_enumeration_rules::read_java_script_own_enumeration_receiver_form(object.ty.kind(),element);
        if form == "absent" {
            return Ok(failure(&result));
        }
        match &object.ty {
            Type::Unit => Ok(failure(&result)),
            Type::Option { element, .. } if form == "guarded" => {
                let ty = element.as_ref().clone();
                let condition = Self::javascript_optional_presence(object.clone(), &ty, span)?;
                let present = Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(object),
                    },
                    ty,
                );
                Ok(Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(self.enumerate_object_keys(present, span)?),
                        otherwise: Box::new(failure(&result)),
                    },
                    result,
                ))
            }
            Type::Record {
                fields,
                open,
                ordered,
                ..
            } if form == "record" => {
                if *open || !*ordered {
                    return Err(unsupported(
                        "Object.keys record",
                        "every actual own field and its value order must be retained",
                        span,
                    ));
                }
                let names = crate::translation::ordered_record_values::property_names(
                    fields.iter().map(|field| field.name.as_str()),
                );
                let order = Expr::new(
                    Node::Method {
                        object: Box::new(Self::reference_value(object)),
                        op: "readJavaScriptRecordPropertyOrder".to_owned(),
                        args: Vec::new(),
                    },
                    array(Type::Float),
                );
                Ok(Expr::new(
                    Node::Math {
                        op: "readJavaScriptRecordPropertyKeyTexts".to_owned(),
                        args: vec![names, order],
                    },
                    result,
                ))
            }
            Type::Array { .. } | Type::Tuple { .. } | Type::String if form == "indexed" => {
                let length = if let Type::Tuple { elements } = &object.ty {
                    super::text_lit(Type::Float, elements.len().to_string())
                } else {
                    Expr::new(
                        Node::Length {
                            array: Box::new(Self::reference_value(object)),
                        },
                        Type::Float,
                    )
                };
                Ok(Expr::new(
                    Node::Math {
                        op: "createJavaScriptDensePropertyKeyTexts".to_owned(),
                        args: vec![length],
                    },
                    result,
                ))
            }
            Type::Float
            | Type::Fixed { .. }
            | Type::Int
            | Type::Nat
            | Type::Bool
            | Type::Map { .. }
            | Type::Set { .. }
                if form == "empty" =>
            {
                Ok(Expr::new(Node::Array { items: Vec::new() }, result))
            }
            _ => Err(unsupported(
                "Object.keys receiver",
                "the actual nominal, callable, Symbol, proxy or exotic own-property protocol must be represented",
                span,
            )),
        }
    }
    pub(super) fn javascript_object_enumeration(
        &mut self,
        operation: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let form =
            crate::translation::javascript_enumeration_rules::read_java_script_enumeration_form(
                operation,
            );
        if self.language != super::Language::JavaScript
            || args.iter().any(|argument| argument.spread)
        {
            return Err(unsupported(
                &format!("Object.{form} arguments"),
                "every actual iterator argument effect must be represented",
                span,
            ));
        }
        if form != "keys" {
            return Err(unsupported(
                &format!("Object.{form}"),
                "the actual value/entry element and mutable pair domains require their complete enumeration operations",
                span,
            ));
        }
        let mut values = args
            .iter()
            .map(|argument| self.expr(&argument.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        if values.is_empty() {
            values.push(Expr::new(Node::Unit, Type::Unit));
        }
        let name = self.fresh_name("ml_enumeration_operands");
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
            ordered: false,
            open: false,
            reference: false,
        };
        let object = Expr::new(
            Node::RecordField {
                object: Box::new(Expr::var(&name, ty.clone())),
                field: field(0),
                index: 0,
            },
            values[0].ty.clone(),
        );
        let body = self.enumerate_object_keys(object, span)?;
        let result = body.ty.clone();
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
            result,
        ))
    }
}
