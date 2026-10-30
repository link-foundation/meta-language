//! Source/AST transport of the complete paired JavaScript-derived byte lambda.
use super::{
    FLOAT, Flavor, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, unsupported,
};
use crate::translation::rust_string_byte_view_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use crate::translation::surface::SParam;
use serde_json::Value;
use std::sync::OnceLock;
fn instantiate(value: &Value, span: Option<Span>) -> Value {
    match value {
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| instantiate(item, span)).collect())
        }
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        if key == "span" {
                            serde_json::to_value(span).expect("source span serializes")
                        } else {
                            instantiate(value, span)
                        },
                    )
                })
                .collect(),
        ),
        value => value.clone(),
    }
}
fn template(span: Option<Span>) -> Result<SExpr> {
    static POLICY: OnceLock<Value> = OnceLock::new();
    let policy = POLICY.get_or_init(|| {
        crate::read_structured_data_lino(include_str!(
            "../../../data/rust-string-byte-template.lino"
        ))
        .expect("the paired UTF8 source template is Links data")
    });
    if policy["schemaVersion"] != 1
        || policy["model"] != "portable Rust scalar string UTF8 byte lambda 1"
        || policy.pointer("/template/k").and_then(Value::as_str) != Some("lambda")
    {
        return Err(unsupported(
            "Rust string byte view",
            "the packaged UTF8 source template is malformed",
            span,
        ));
    }
    serde_json::from_value(instantiate(&policy["template"], span)).map_err(|_| {
        unsupported(
            "Rust string byte view",
            "the packaged UTF8 source lambda is not representable",
            span,
        )
    })
}
impl RustParser {
    pub(super) fn string_byte_view(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !rules::accept_rust_string_byte_view_candidate(
            &method.value,
            self.ordinary_completion_context(),
        ) {
            return Ok(None);
        }
        let mut actual = self.origins.origin(receiver, path);
        let error = rules::read_rust_string_byte_view_error(
            &method.value,
            args.len() as f64,
            actual
                .ty
                .as_ref()
                .map_or("", crate::translation::types::Type::kind),
            actual.text_source_model.unwrap_or(""),
            actual.wrappers.len() as f64,
            actual.proven,
            actual.wrappers.iter().all(|layer| layer == "reference"),
        );
        if !error.is_empty() {
            return Err(unsupported("Rust string byte view", error, range));
        }
        self.origins.check_owned_origin(&actual, range)?;
        if method.value == "into_bytes" {
            self.origins.consume_owned_binding(&actual, range)?;
        } else {
            actual = self.origins.capture_owned_borrow(actual);
        }
        let element = super::rust_fixed_type("u8").expect("standard byte type");
        let ty = crate::translation::types::array(element.clone());
        let name = self.fresh_product_binding();
        let storage = rules::read_rust_string_byte_view_storage(&method.value);
        let layers = rules::read_rust_string_byte_view_layers(&method.value) as usize;
        let encoded = SExpr::new(
            SNode::App {
                func: Box::new(template(range)?),
                args: vec![receiver.clone()],
            },
            range,
        );
        let byte = SExpr::new(
            SNode::Cast {
                arg: Box::new(SExpr::new(
                    SNode::Name {
                        path: vec![name.clone()],
                    },
                    range,
                )),
                to: element.clone(),
                from: Some(FLOAT),
                flavor: Flavor::Exact,
            },
            range,
        );
        let mapping = SExpr::new(
            SNode::Lambda {
                params: vec![SParam {
                    name,
                    ty: Some(FLOAT),
                    optional: false,
                    default_value: None,
                    span: range,
                    guard: None,
                    rocq_type: None,
                }],
                ret: Some(element),
                captures: Vec::new(),
                assigned_captures: None,
                body: Box::new(byte),
            },
            range,
        );
        let result = SExpr::new(
            SNode::Method {
                object: Box::new(encoded),
                op: "map".to_owned(),
                args: vec![Some(mapping)],
            },
            range,
        );
        let mut origin = SourceOrigin::with_type(ty, path, vec!["reference".to_owned(); layers]);
        origin.array_source_model = Some(if storage == "owned vector" {
            "owned vector"
        } else {
            "immutable slice"
        });
        origin.array_element_copy_evidence = true;
        origin.source_lifetimes = if layers == 0 {
            Vec::new()
        } else {
            vec![actual.source_lifetimes.first().cloned().unwrap_or(None)]
        };
        origin.source_owner = Some(SourceOwner {
            kind: if storage == "owned vector" {
                "library"
            } else {
                "slice"
            }
            .to_owned(),
            path: if storage == "owned vector" {
                vec!["std".to_owned(), "vec".to_owned(), "Vec".to_owned()]
            } else {
                Vec::new()
            },
            arguments: vec![SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["u8".to_owned()],
                arguments: Vec::new(),
            }],
        });
        if layers > 0 {
            origin.source_binding_identity = actual.source_binding_identity.clone();
            origin.source_loan_binding_identities =
                crate::translation::rust_owned_binding_rules::project_rust_owned_loan_bindings(
                    actual.source_binding_identity.as_deref().unwrap_or(""),
                    &actual.source_loan_binding_identities,
                );
        }
        origin.source_byte_iterator = method.value == "bytes";
        origin.fresh_array_value = range.filter(|_| storage == "owned vector");
        if storage == "owned vector" {
            self.origins.mark_vector_constructor(&result, origin);
        } else {
            self.origins.mark(&result, origin);
        }
        if method.value == "bytes" {
            return self.iterator_reduction(&result, method, args, range, path);
        }
        Ok(Some(result))
    }
}
