//! Generic projection of the shared source term plan into existing typed ASTs.
use super::{
    BOOL, BinaryOp, FLOAT, Result, RustParser, SExpr, SNode, Span, Type, UnaryOp, type_error,
};
use crate::translation::rust_integer_float_plan_rules as rules;
use std::collections::BTreeMap;
#[derive(Clone)]
struct PlanTerm {
    node: SExpr,
    ty: Type,
}
fn coordinate(value: f64, limit: usize, range: Option<Span>) -> Result<usize> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 || value > 4096.0 {
        return Err(type_error(
            "invalid shared integer floating source coordinate",
            range,
        ));
    }
    let index = format!("{:.0}", value.abs())
        .parse::<usize>()
        .map_err(|_| type_error("invalid shared integer floating source coordinate", range))?;
    if index >= limit {
        return Err(type_error(
            "the shared integer floating source plan lost an earlier term or binding slot",
            range,
        ));
    }
    Ok(index)
}
fn term(values: &[PlanTerm], index: f64, range: Option<Span>) -> Result<PlanTerm> {
    Ok(values[coordinate(index, values.len(), range)?].clone())
}
fn name(
    parser: &mut RustParser,
    names: &mut BTreeMap<usize, String>,
    slot: f64,
    range: Option<Span>,
) -> Result<String> {
    let index = coordinate(slot, 129, range)?;
    Ok(names
        .entry(index)
        .or_insert_with(|| parser.fresh_product_binding())
        .clone())
}
impl RustParser {
    pub(super) fn project_integer_float_plan(
        &mut self,
        input: SExpr,
        source_type: &Type,
        range: Option<Span>,
    ) -> Result<SExpr> {
        let Type::Fixed { bits, .. } = source_type else {
            return Err(type_error(
                "the exact integer floating source plan requires an actual fixed source type",
                range,
            ));
        };
        let plan = rules::plan_rust_integer_float_conversion(f64::from(*bits));
        if plan.is_empty() || plan.len() % 5 != 0 || plan.len() > 4096 {
            return Err(type_error(
                "the exact integer floating source plan exceeds its declared domain",
                range,
            ));
        }
        let mut values = Vec::<PlanTerm>::new();
        let mut names = BTreeMap::new();
        for row in plan.chunks_exact(5) {
            let operation = rules::read_rust_integer_float_plan_operation(row[0]);
            let value = match operation.as_str() {
                "input" => PlanTerm {
                    node: input.clone(),
                    ty: source_type.clone(),
                },
                "fixed zero" | "fixed two" | "floating zero" | "floating one" | "floating two" => {
                    let ty = if operation.starts_with("fixed ") {
                        source_type.clone()
                    } else {
                        FLOAT
                    };
                    let magnitude = if operation.ends_with("zero") {
                        "0"
                    } else if operation.ends_with("one") {
                        "1"
                    } else {
                        "2"
                    };
                    PlanTerm {
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
                "fixed reference" | "floating reference" => PlanTerm {
                    node: SExpr::new(
                        SNode::Name {
                            path: vec![name(self, &mut names, row[1], range)?],
                        },
                        range,
                    ),
                    ty: if operation == "fixed reference" {
                        source_type.clone()
                    } else {
                        FLOAT
                    },
                },
                "equal" | "less" | "remainder" | "division" | "addition" | "multiplication" => {
                    let left = term(&values, row[1], range)?;
                    let right = term(&values, row[2], range)?;
                    let (op, ty) = match operation.as_str() {
                        "equal" => (BinaryOp::Eq, BOOL),
                        "less" => (BinaryOp::Lt, BOOL),
                        "remainder" => (BinaryOp::Rem, source_type.clone()),
                        "division" => (BinaryOp::Div, source_type.clone()),
                        "addition" => (BinaryOp::Add, FLOAT),
                        _ => (BinaryOp::Mul, FLOAT),
                    };
                    PlanTerm {
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
                "negation" => PlanTerm {
                    node: SExpr::new(
                        SNode::Unary {
                            op: UnaryOp::Neg,
                            arg: Box::new(term(&values, row[1], range)?.node),
                        },
                        range,
                    ),
                    ty: FLOAT,
                },
                "conditional" => {
                    let cond = term(&values, row[1], range)?;
                    let yes = term(&values, row[2], range)?;
                    let no = term(&values, row[3], range)?;
                    PlanTerm {
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
                    let item = term(&values, row[2], range)?;
                    let body = term(&values, row[3], range)?;
                    PlanTerm {
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
                _ => {
                    return Err(type_error(
                        "the shared integer floating source plan has an unknown source term",
                        range,
                    ));
                }
            };
            values.push(value);
        }
        let result = values.pop().ok_or_else(|| {
            type_error(
                "the shared integer floating source plan lost its complete result",
                range,
            )
        })?;
        if result.ty != FLOAT {
            return Err(type_error(
                "the shared integer floating source plan lost its actual binary64 result",
                range,
            ));
        }
        Ok(result.node)
    }
}
