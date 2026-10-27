//! Generic actual source facts and typed source-term projection only.
use super::integer_float_plan::coordinate;
use super::primitive_try_from::{facts, origin};
use super::{
    BOOL, BinaryOp, Flavor, Result, RustParser, SExpr, SNode, Span, Token, Type, rust_fixed_type,
    type_error, unsupported,
};
use crate::translation::rust_integer_bit_rules as rules;
use std::collections::BTreeMap;
#[derive(Clone)]
struct Term {
    node: SExpr,
    ty: Type,
}
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn read(values: &[Term], index: f64, range: Option<Span>) -> Result<Term> {
    Ok(values[coordinate(index, values.len(), range)?].clone())
}
fn name(
    parser: &mut RustParser,
    names: &mut BTreeMap<usize, String>,
    slot: f64,
    range: Option<Span>,
) -> Result<String> {
    let index = coordinate(slot, 513, range)?;
    Ok(names
        .entry(index)
        .or_insert_with(|| parser.fresh_product_binding())
        .clone())
}
impl RustParser {
    pub(super) fn read_integer_bits(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !self.ordinary_completion_context()
            || !rules::accept_rust_integer_bit_method(&method.value)
        {
            return Ok(None);
        }
        if !args.is_empty() {
            return Err(type_error(
                format!("{} needs no actual source arguments", method.value),
                range,
            ));
        }
        if self.origins.collecting {
            return Ok(Some(SExpr::new(
                SNode::App {
                    func: Box::new(SExpr::new(
                        SNode::Name {
                            path: vec![method.value.clone()],
                        },
                        range,
                    )),
                    args: vec![receiver.clone()],
                },
                range,
            )));
        }
        let mut source = facts(self, receiver, path);
        let owner = source.path();
        if crate::translation::rust_primitive_inherent_copy_rules::accept_rust_primitive_inherent_copy(&method.value,&source.kind,&source.bits,source.signed,source.owner.as_ref().map_or("",|owner|owner.kind.as_str()),&owner,source.owner.as_ref().map_or(-1.0,|owner|count(owner.arguments.len())),source.origin.proven,count(source.origin.wrappers.len()),source.origin.wrappers.iter().all(|layer|layer=="reference"),source.origin.copy_evidence.is_some()){
   source.origin.wrappers.clear();source.origin.source_lifetimes.clear();source.origin.copy_evidence=Some(());
  }
        if !rules::accept_rust_integer_bit_receiver(
            &source.kind,
            &source.bits,
            source.signed,
            source
                .owner
                .as_ref()
                .map_or("", |owner| owner.kind.as_str()),
            &owner,
            source
                .owner
                .as_ref()
                .map_or(-1.0, |owner| count(owner.arguments.len())),
            source.origin.proven,
            count(source.origin.wrappers.len()),
        ) {
            return Err(unsupported(
                "Rust integer bit operation",
                "a proven standard fixed integer receiver with its actual source owner is required",
                range,
            ));
        }
        let source_type =
            source.origin.ty.clone().ok_or_else(|| {
                type_error("the integer bit receiver lost its source type", range)
            })?;
        let Type::Fixed { bits, .. } = &source_type else {
            return Err(type_error(
                "the integer bit receiver needs its actual fixed type",
                range,
            ));
        };
        let unsigned = Type::Fixed {
            bits: *bits,
            signed: false,
        };
        let count_type = Type::Fixed {
            bits: 32,
            signed: false,
        };
        let plan = rules::plan_rust_integer_bits(&method.value, f64::from(*bits));
        if plan.is_empty() || plan.len() % 5 != 0 || plan.len() > 16384 {
            return Err(type_error(
                "the shared integer bit source plan exceeds its declared domain",
                range,
            ));
        }
        let mut values = Vec::<Term>::new();
        let mut names = BTreeMap::new();
        for row in plan.chunks_exact(5) {
            let operation = rules::read_rust_integer_bit_plan_operation(row[0]);
            let value = match operation.as_str() {
                "unsigned input" => Term {
                    node: SExpr::new(
                        SNode::Cast {
                            arg: Box::new(receiver.clone()),
                            to: unsigned.clone(),
                            from: None,
                            flavor: Flavor::Wrapping,
                        },
                        range,
                    ),
                    ty: unsigned.clone(),
                },
                "fixed zero" | "fixed one" | "fixed two" | "count zero" | "count one" => {
                    let ty = if operation.starts_with("fixed ") {
                        unsigned.clone()
                    } else {
                        count_type.clone()
                    };
                    let magnitude = if operation.ends_with("zero") {
                        "0"
                    } else if operation.ends_with("one") {
                        "1"
                    } else {
                        "2"
                    };
                    Term {
                        node: SExpr::new(
                            SNode::Num {
                                value: magnitude.to_owned(),
                                ty: Some(ty.clone()),
                                negative: false,
                                unit: false,
                            },
                            range,
                        ),
                        ty,
                    }
                }
                "false" | "true" => Term {
                    node: SExpr::new(
                        SNode::Bool {
                            value: operation == "true",
                        },
                        range,
                    ),
                    ty: BOOL,
                },
                "fixed reference" | "count reference" | "boolean reference" => Term {
                    node: SExpr::new(
                        SNode::Name {
                            path: vec![name(self, &mut names, row[1], range)?],
                        },
                        range,
                    ),
                    ty: if operation == "fixed reference" {
                        unsigned.clone()
                    } else if operation == "count reference" {
                        count_type.clone()
                    } else {
                        BOOL
                    },
                },
                "equal"
                | "remainder"
                | "division"
                | "count addition"
                | "fixed addition"
                | "fixed multiplication" => {
                    let left = read(&values, row[1], range)?;
                    let right = read(&values, row[2], range)?;
                    let (op, ty) = match operation.as_str() {
                        "equal" => (BinaryOp::Eq, BOOL),
                        "remainder" => (BinaryOp::Rem, unsigned.clone()),
                        "division" => (BinaryOp::Div, unsigned.clone()),
                        "count addition" => (BinaryOp::Add, count_type.clone()),
                        "fixed addition" => (BinaryOp::Add, unsigned.clone()),
                        _ => (BinaryOp::Mul, unsigned.clone()),
                    };
                    Term {
                        node: SExpr::new(
                            SNode::Binary {
                                op,
                                left: Box::new(left.node),
                                right: Box::new(right.node),
                                rounding: None,
                            },
                            range,
                        ),
                        ty,
                    }
                }
                "conditional" => {
                    let cond = read(&values, row[1], range)?;
                    let yes = read(&values, row[2], range)?;
                    let no = read(&values, row[3], range)?;
                    Term {
                        ty: yes.ty,
                        node: SExpr::new(
                            SNode::If {
                                cond: Box::new(cond.node),
                                then: Box::new(yes.node),
                                otherwise: Box::new(no.node),
                            },
                            range,
                        ),
                    }
                }
                "binding" => {
                    let item = read(&values, row[2], range)?;
                    let body = read(&values, row[3], range)?;
                    Term {
                        ty: body.ty,
                        node: SExpr::new(
                            SNode::Let {
                                name: name(self, &mut names, row[1], range)?,
                                ty: Some(item.ty.into()),
                                value: Box::new(item.node),
                                body: Box::new(body.node),
                            },
                            range,
                        ),
                    }
                }
                "source cast" => Term {
                    node: SExpr::new(
                        SNode::Cast {
                            arg: Box::new(read(&values, row[1], range)?.node),
                            to: source_type.clone(),
                            from: None,
                            flavor: Flavor::Wrapping,
                        },
                        range,
                    ),
                    ty: source_type.clone(),
                },
                _ => {
                    return Err(type_error(
                        "unknown shared integer bit source operation",
                        range,
                    ));
                }
            };
            values.push(value);
        }
        let result_owner = rules::read_rust_integer_bit_result_owner(&method.value, &owner);
        let result_type = rust_fixed_type(&result_owner).ok_or_else(|| {
            type_error("the shared integer bit result lost its source type", range)
        })?;
        let result = values.pop().ok_or_else(|| {
            type_error(
                "the shared integer bit plan lost its complete result",
                range,
            )
        })?;
        if result.ty != result_type {
            return Err(type_error(
                "the shared integer bit plan lost its actual result type",
                range,
            ));
        }
        self.origins
            .mark(&result.node, origin(result_type, &result_owner, path));
        Ok(Some(result.node))
    }
}
