//! Checked exact literal facts into the generated integer-domain partition plan.
use super::{
    BinaryOp, Checker, Column, NPat, Result, Row, SExpr, SNode, SPattern, Span, Type, type_error,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::rust_integer_range_rules as rules;
use crate::translation::types::fixed_bounds;

fn literal_text(pattern: &NPat) -> Option<&str> {
    if let NPat::Lit { key, .. } = pattern {
        key.rsplit_once(':').map(|(_, value)| value)
    } else {
        None
    }
}
impl Checker {
    fn integer_range_endpoint(
        &mut self,
        endpoint: SPattern,
        ty: &Type,
        span: Option<Span>,
    ) -> Result<String> {
        let pattern = self.normalise_pattern(NPat::Surface(endpoint), ty, span)?;
        literal_text(&pattern).map(str::to_owned).ok_or_else(|| {
            type_error(
                "an integer range endpoint requires an actual checked literal",
                span,
            )
        })
    }
    pub(super) fn normalise_integer_range(
        &mut self,
        lower: Option<SPattern>,
        upper: Option<SPattern>,
        inclusive: bool,
        ty: &Type,
        span: Option<Span>,
    ) -> Result<NPat> {
        let Type::Fixed { bits, signed } = ty else {
            return Err(type_error(
                "an integer range pattern requires an actual fixed-width integer scrutinee",
                span,
            ));
        };
        let (minimum, maximum) = fixed_bounds(*bits, *signed);
        let lower = match lower {
            Some(value) => self.integer_range_endpoint(value, ty, span)?,
            None => minimum.to_string(),
        };
        let upper = match upper {
            Some(value) => {
                let value = self.integer_range_endpoint(value, ty, span)?;
                if inclusive {
                    rules::read_rust_integer_range_successor(&value)
                } else {
                    value
                }
            }
            None => rules::read_rust_integer_range_successor(&maximum.to_string()),
        };
        let error = rules::read_rust_integer_range_bounds_error(&lower, &upper);
        if !error.is_empty() {
            return Err(type_error(&error, span));
        }
        Ok(NPat::Interval { lower, upper })
    }
    pub(super) fn compile_integer_range_column(
        &mut self,
        columns: &[Column],
        rows: &[Row],
        pivot: usize,
        span: Option<Span>,
    ) -> Result<SExpr> {
        let column = &columns[pivot];
        let Type::Fixed { bits, signed } = &column.ty else {
            return Err(type_error(
                "an integer range pattern requires an actual fixed-width integer scrutinee",
                span,
            ));
        };
        let (minimum, maximum) = fixed_bounds(*bits, *signed);
        let (mut kinds, mut lowers, mut uppers) = (Vec::new(), Vec::new(), Vec::new());
        for row in rows {
            match &row.patterns[pivot] {
                NPat::Interval { lower, upper } => {
                    kinds.push("interval".to_owned());
                    lowers.push(lower.clone());
                    uppers.push(upper.clone());
                }
                pattern @ NPat::Lit { .. } => {
                    let value = literal_text(pattern).expect("checked integer literal key");
                    kinds.push("lit".to_owned());
                    lowers.push(value.to_owned());
                    uppers.push(rules::read_rust_integer_range_successor(value));
                }
                NPat::Wild => {
                    kinds.push("wild".to_owned());
                    lowers.push(String::new());
                    uppers.push(String::new());
                }
                NPat::Bind(_) => {
                    kinds.push("bind".to_owned());
                    lowers.push(String::new());
                    uppers.push(String::new());
                }
                _ => {
                    return Err(type_error(
                        "integer range compilation requires its complete checked domain partition",
                        span,
                    ));
                }
            }
        }
        let cuts = rules::read_rust_integer_range_cuts(
            &minimum.to_string(),
            &rules::read_rust_integer_range_successor(&maximum.to_string()),
            &lowers,
            &uppers,
        );
        let plan = rules::plan_rust_integer_range_rows(&kinds, &lowers, &uppers, &cuts);
        if plan.first() != Some(&0.0) {
            return Err(type_error(
                if plan.first() == Some(&-2.0) {
                    "non-exhaustive match"
                } else {
                    "integer range compilation requires its complete checked domain partition"
                },
                span,
            ));
        }
        let remaining = columns
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != pivot)
            .map(|(_, column)| column.clone())
            .collect::<Vec<_>>();
        let mut at = 2;
        let mut bodies = Vec::new();
        for _ in 0..cuts.len() - 1 {
            let count = number_index(plan[at]).expect("shared integer cell row count");
            at += 1;
            let mut selected = Vec::new();
            for _ in 0..count {
                let row = &rows[number_index(plan[at]).expect("shared actual integer source row")];
                at += 1;
                let mut binds = row.binds.clone();
                if let NPat::Bind(name) = &row.patterns[pivot] {
                    binds.push((name.clone(), column.name.clone()));
                }
                selected.push(Row {
                    patterns: row
                        .patterns
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index != pivot)
                        .map(|(_, pattern)| pattern.clone())
                        .collect(),
                    body: row.body.clone(),
                    binds,
                });
            }
            bodies.push(self.compile_rows(&remaining, selected, span)?);
        }
        let mut body = bodies.pop().expect("complete nonempty integer domain");
        for (cell, then) in bodies.into_iter().enumerate().rev() {
            let text = &cuts[cell + 1];
            let negative = text.starts_with('-');
            let value = text.strip_prefix('-').unwrap_or(text).to_owned();
            body = SExpr::new(
                SNode::If {
                    cond: Box::new(SExpr::new(
                        SNode::Binary {
                            op: BinaryOp::Lt,
                            left: Box::new(SExpr::name(&column.name)),
                            right: Box::new(SExpr::new(
                                SNode::Num {
                                    value,
                                    ty: Some(column.ty.clone()),
                                    negative,
                                    unit: false,
                                },
                                span,
                            )),
                            rounding: None,
                        },
                        span,
                    )),
                    then: Box::new(then),
                    otherwise: Box::new(body),
                },
                span,
            );
        }
        Ok(body)
    }
}
