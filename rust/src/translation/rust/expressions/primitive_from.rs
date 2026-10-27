//! Actual primitive origin, capture, conditional and cast AST transport only.
use super::{
    BOOL, FLOAT, Flavor, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Type, UnaryOp,
    rust_fixed_type, type_error, unsupported,
};
use crate::translation::rust_primitive_from_rules as rules;
use crate::translation::source_signatures::SourceOwner;
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
impl RustParser {
    pub(super) fn project_primitive_from(
        &mut self,
        mut receiver: SExpr,
        target: &str,
        range: Option<Span>,
        path: &[String],
    ) -> Result<SExpr> {
        let (form, typed) = match &receiver.node {
            SNode::Num { ty, .. } => ("numeral", ty.is_some()),
            SNode::Unary {
                op: UnaryOp::Neg,
                arg,
            } => match &arg.node {
                SNode::Num { ty, .. } => ("negative numeral", ty.is_some()),
                _ => ("", true),
            },
            _ => ("", true),
        };
        let literal_kind = match &receiver.node {
            SNode::Num { ty: Some(ty), .. } => ty.kind(),
            SNode::Unary { arg, .. } => match &arg.node {
                SNode::Num { ty: Some(ty), .. } => ty.kind(),
                _ => "",
            },
            _ => "",
        };
        let literal_owner =
            rules::read_rust_primitive_from_literal_owner(form, typed, literal_kind);
        if !literal_owner.is_empty() {
            let Some(ty) = (if literal_owner == "f64" {
                Some(FLOAT)
            } else {
                rust_fixed_type(&literal_owner)
            }) else {
                return Err(type_error(
                    "the primitive From literal lost its actual default type",
                    range,
                ));
            };
            match &mut receiver.node {
                SNode::Num { ty: actual, .. } => *actual = Some(ty.clone()),
                SNode::Unary { arg, .. } => {
                    if let SNode::Num { ty: actual, .. } = &mut arg.node {
                        *actual = Some(ty.clone());
                    }
                }
                _ => {}
            }
            let mut origin = SourceOrigin::with_type(ty, path, Vec::new());
            origin.source_owner = Some(SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![literal_owner],
                arguments: Vec::new(),
            });
            self.origins.mark(&receiver, origin);
        }
        let origin = self.origins.origin(&receiver, path);
        let owner = origin.source_owner.clone().or_else(|| {
            let primitive = match (&receiver.node, &origin.ty) {
                (SNode::Bool { .. }, _) => Some("bool"),
                (SNode::Num { .. }, Some(Type::Float)) => Some("f64"),
                _ => None,
            };
            primitive.map(|primitive| SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![primitive.to_owned()],
                arguments: Vec::new(),
            })
        });
        let (bits, signed) = match &origin.ty {
            Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
            _ => (String::new(), false),
        };
        if !rules::accept_rust_primitive_from_source(
            origin.ty.as_ref().map_or("", Type::kind),
            &bits,
            signed,
            owner.as_ref().map_or("", |owner| owner.kind.as_str()),
            &owner
                .as_ref()
                .map_or_else(String::new, |owner| owner.path.join("::")),
            owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            origin.proven,
            count(origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust primitive From",
                "the actual argument must retain a proven direct standard primitive source type",
                range,
            ));
        }
        let plan = rules::plan_rust_primitive_from(
            target,
            &owner
                .as_ref()
                .map_or_else(String::new, |owner| owner.path.join("::")),
        );
        if plan.is_empty() {
            return Err(unsupported(
                "Rust primitive From",
                "the selected standard primitive From implementation must exist for the actual source owner",
                range,
            ));
        }
        let to = match target {
            "bool" => BOOL,
            "f64" => FLOAT,
            _ => rust_fixed_type(target).ok_or_else(|| {
                type_error("the primitive From target lost its actual type", range)
            })?,
        };
        let source_type = origin
            .ty
            .clone()
            .ok_or_else(|| type_error("the primitive From source lost its actual type", range))?;
        let name = self.fresh_product_binding();
        let local = SExpr::new(
            SNode::Name {
                path: vec![name.clone()],
            },
            receiver.span.or(range),
        );
        self.origins.mark(&local, origin);
        let body = match plan.as_str() {
            "identity" => local,
            "boolean integer" => SExpr::new(
                SNode::If {
                    cond: Box::new(local),
                    then: Box::new(SExpr::new(
                        SNode::Num {
                            value: "1".to_owned(),
                            ty: Some(to.clone()),
                            negative: false,
                            unit: false,
                        },
                        range,
                    )),
                    otherwise: Box::new(SExpr::new(
                        SNode::Num {
                            value: "0".to_owned(),
                            ty: Some(to.clone()),
                            negative: false,
                            unit: false,
                        },
                        range,
                    )),
                },
                range,
            ),
            "integer exact" | "floating exact" => SExpr::new(
                SNode::Cast {
                    arg: Box::new(local),
                    to: to.clone(),
                    from: None,
                    flavor: Flavor::Exact,
                },
                range,
            ),
            _ => {
                return Err(type_error(
                    "the shared primitive From plan lost its actual interpretation",
                    range,
                ));
            }
        };
        let result = SExpr::new(
            SNode::Let {
                name,
                ty: Some(source_type.into()),
                value: Box::new(receiver),
                body: Box::new(body),
            },
            range,
        );
        let mut result_origin = SourceOrigin::with_type(to, path, Vec::new());
        result_origin.source_owner = Some(SourceOwner {
            kind: "primitive".to_owned(),
            path: vec![target.to_owned()],
            arguments: Vec::new(),
        });
        self.origins.mark(&result, result_origin);
        Ok(result)
    }
}
