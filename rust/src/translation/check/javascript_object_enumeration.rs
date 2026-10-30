//! Actual source arguments into ordinary order/key definitions and source errors.
use super::{
    Checker, Env, Expr, LitValue, Node, Result, SArrayItem, Span, Type, array, unsupported,
};
use crate::translation::ir::RecordEntry;
use crate::translation::types::TypeField;

fn failure(result: &Type) -> Expr {
    Expr::new(Node::Math{op:crate::translation::source_exception_rules::read_java_script_source_exception_operation("TypeError"),args:vec![super::text_lit(Type::String,crate::translation::javascript_own_property_rules::read_java_script_object_coercion_error()),Expr::lit(Type::Bool,LitValue::Bool(true))]},result.clone())
}
fn value_element(ty: &Type, fallback: Option<&Type>, span: Option<Span>) -> Result<Type> {
    if let Type::Option { element, .. } = ty {
        return value_element(element, fallback, span);
    }
    if let Type::Array { element, .. } = ty {
        return Ok(element.as_ref().clone());
    }
    if matches!(ty, Type::String) {
        return Ok(Type::String);
    }
    let fields: Vec<&Type> = match ty {
        Type::Record { fields, .. } => fields.iter().map(|field| &field.ty).collect(),
        Type::Tuple { elements } => elements.iter().collect(),
        _ => Vec::new(),
    };
    let Some(first) = fields.first() else {
        return Ok(fallback.cloned().unwrap_or(Type::Float));
    };
    if fields.iter().any(|field| !field.same(first)) {
        return Err(unsupported(
            "heterogeneous object values",
            "every actual field needs its shared dynamic value union",
            span,
        ));
    }
    Ok((*first).clone())
}
impl Checker {
    fn record_enumeration_values(
        &mut self,
        object: Expr,
        result: &Type,
        span: Option<Span>,
    ) -> Result<Expr> {
        let name = self.fresh_name("ml_enumeration_record");
        let value = Self::reference_value(object);
        let owned = Expr::var(&name, value.ty.clone());
        let Type::Record { fields, .. } = &value.ty else {
            unreachable!("actual record enumeration owner")
        };
        let items = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                Expr::new(
                    Node::RecordField {
                        object: Box::new(owned.clone()),
                        field: field.name.clone(),
                        index,
                    },
                    field.ty.clone(),
                )
            })
            .collect();
        let values = Expr::new(Node::Array { items }, result.clone());
        let query = Expr::new(
            Node::Method {
                object: Box::new(owned),
                op: "readJavaScriptRecordPropertyOrder".to_owned(),
                args: Vec::new(),
            },
            array(Type::Float),
        );
        let body = Expr::new(
            Node::Math {
                op: "reorderJavaScriptRecordPropertyValues".to_owned(),
                args: vec![values, query],
            },
            result.clone(),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            result.clone(),
        )
        .with_span(span))
    }
    fn enumerate_object_values(
        &mut self,
        object: Expr,
        result: &Type,
        span: Option<Span>,
    ) -> Result<Expr> {
        let element = if let Type::Option { element, .. } = &object.ty {
            element.kind()
        } else {
            ""
        };
        let form=crate::translation::javascript_enumeration_rules::read_java_script_own_enumeration_receiver_form(object.ty.kind(),element);
        if form == "absent" {
            return Ok(failure(result));
        }
        match &object.ty {
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
                        then: Box::new(self.enumerate_object_values(present, result, span)?),
                        otherwise: Box::new(failure(result)),
                    },
                    result.clone(),
                ))
            }
            Type::Record { open, ordered, .. } if form == "record" => {
                if *open || !*ordered {
                    return Err(unsupported(
                        "Object.keys record",
                        "every actual own field and its value order must be retained",
                        span,
                    ));
                }
                self.record_enumeration_values(object, result, span)
            }
            Type::String if form == "indexed" => Ok(Expr::new(
                Node::Math {
                    op: "createJavaScriptStringOwnPropertyValues".to_owned(),
                    args: vec![object],
                },
                result.clone(),
            )),
            Type::Array { .. } if form == "indexed" => {
                let empty = Expr::new(Node::Array { items: Vec::new() }, result.clone());
                Ok(Expr::new(
                    Node::Append {
                        left: Box::new(empty),
                        right: Box::new(Self::reference_value(object)),
                    },
                    result.clone(),
                ))
            }
            Type::Tuple { elements } if form == "indexed" => {
                let elements = elements.clone();
                let owned = Self::reference_value(object);
                let items = elements
                    .into_iter()
                    .enumerate()
                    .map(|(index, ty)| {
                        Expr::new(
                            Node::TupleIndex {
                                object: Box::new(owned.clone()),
                                index,
                            },
                            ty,
                        )
                    })
                    .collect();
                Ok(Expr::new(Node::Array { items }, result.clone()))
            }
            _ if form == "empty" => {
                Ok(Expr::new(Node::Array { items: Vec::new() }, result.clone()))
            }
            _ => Err(unsupported(
                "Object.keys receiver",
                "the actual nominal, callable, Symbol, proxy or exotic own-property protocol must be represented",
                span,
            )),
        }
    }
    fn enumerate_object_entries(
        &mut self,
        object: Expr,
        result: &Type,
        span: Option<Span>,
    ) -> Result<Expr> {
        let element = if let Type::Option { element, .. } = &object.ty {
            element.kind()
        } else {
            ""
        };
        let receiver=crate::translation::javascript_enumeration_rules::read_java_script_own_enumeration_receiver_form(object.ty.kind(),element);
        if receiver == "absent" {
            return Ok(failure(result));
        }
        if let Type::Option { element, .. } = &object.ty
            && receiver == "guarded"
        {
            let ty = element.as_ref().clone();
            let condition = Self::javascript_optional_presence(object.clone(), &ty, span)?;
            let present = Expr::new(
                Node::UnwrapOption {
                    arg: Box::new(object),
                },
                ty,
            );
            let then = self.enumerate_object_entries(present, result, span)?;
            return Ok(Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(then),
                    otherwise: Box::new(failure(result)),
                },
                result.clone(),
            ));
        }
        let Type::Array { element, .. } = result else {
            unreachable!("entry result is an actual array")
        };
        let Type::Tuple { elements } = element.as_ref() else {
            unreachable!("entry element is an actual pair")
        };
        let values =
            self.enumerate_object_values(object.clone(), &array(elements[1].clone()), span)?;
        let keys = self.enumerate_object_keys(object, span)?;
        Ok(Expr::new(Node::Math{op:crate::translation::javascript_enumeration_rules::read_java_script_object_entry_pairing_operation(),args:vec![keys,values]},result.clone()))
    }
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
        expected: Option<&Type>,
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
        if form != "keys" && !crate::translation::javascript_enumeration_rules::accept_java_script_enumeration_value_domain(&form) {
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
        let body = if form == "keys" {
            self.enumerate_object_keys(object, span)?
        } else {
            let fallback = if let Some(Type::Array { element, .. }) = expected {
                Some(element.as_ref())
            } else {
                None
            };
            let fallback = if form == "entries" {
                match fallback {
                    Some(Type::Tuple { elements }) => elements.get(1),
                    _ => None,
                }
            } else {
                fallback
            };
            let element = value_element(&object.ty, fallback, span)?;
            if form == "entries" {
                let result = array(Type::Tuple {
                    elements: vec![Type::String, element],
                });
                self.enumerate_object_entries(object, &result, span)?
            } else {
                let result = array(element);
                self.enumerate_object_values(object, &result, span)?
            }
        };
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
