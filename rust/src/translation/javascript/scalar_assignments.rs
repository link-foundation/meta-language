//! Mechanical materialization of the shared source-term construction plan.
use super::{
    BinaryOp, JavaScriptParser, Result, SArrayItem, SExpr, SNode, Span, Token, TokenKind, UnaryOp,
    node, type_error, unsupported,
};
use crate::translation::{
    frontend_rules::ml_array::number_index, scalar_assignment_rules as rules,
};

fn materialize(
    cell: SExpr,
    right: Option<SExpr>,
    operator: &str,
    postfix: bool,
    names: &[String; 2],
    place: Span,
) -> Result<SExpr> {
    let plan = rules::plan_scalar_assignment_expression(operator, postfix);
    let operation = rules::read_scalar_assignment_operation(operator);
    if plan.is_empty() || !plan.len().is_multiple_of(4) {
        return Err(unsupported(
            "scalar assignment expression",
            "this operator needs its complete source operation",
            Some(place),
        ));
    }
    let mut nodes: Vec<SExpr> = Vec::new();
    for instruction in plan.as_chunks::<4>().0 {
        let code = number_index(instruction[0]).ok_or_else(|| {
            type_error(
                "invalid scalar assignment construction operation",
                Some(place),
            )
        })?;
        let first = number_index(instruction[1]).ok_or_else(|| {
            type_error(
                "invalid scalar assignment construction operand",
                Some(place),
            )
        })?;
        let second = number_index(instruction[2]).ok_or_else(|| {
            type_error(
                "invalid scalar assignment construction operand",
                Some(place),
            )
        })?;
        let third = number_index(instruction[3]).ok_or_else(|| {
            type_error(
                "invalid scalar assignment construction operand",
                Some(place),
            )
        })?;
        let actual = |index: usize| {
            nodes.get(index).cloned().map(Box::new).ok_or_else(|| {
                type_error(
                    "a scalar assignment plan needs an earlier actual source term",
                    Some(place),
                )
            })
        };
        let value = match code {
            0 => {
                if first == 0 {
                    cell.clone()
                } else {
                    right.clone().ok_or_else(|| {
                        type_error(
                            "scalar assignment source transport lost an actual operand",
                            Some(place),
                        )
                    })?
                }
            }
            1 => node(
                SNode::Name {
                    path: vec![
                        names
                            .get(first)
                            .ok_or_else(|| {
                                type_error("unknown scalar assignment temporary", Some(place))
                            })?
                            .clone(),
                    ],
                },
                place,
            ),
            2 => node(
                SNode::CellRead {
                    cell: actual(first)?,
                },
                place,
            ),
            3 => node(
                SNode::Num {
                    value: first.to_string(),
                    ty: None,
                    negative: false,
                    unit: true,
                },
                place,
            ),
            4 => node(
                SNode::Binary {
                    op: serde_json::from_value::<BinaryOp>(serde_json::Value::String(
                        operation.clone(),
                    ))
                    .map_err(|error| type_error(error.to_string(), Some(place)))?,
                    left: actual(second)?,
                    right: actual(third)?,
                    rounding: None,
                },
                place,
            ),
            5 => node(
                SNode::Math {
                    op: operation.clone(),
                    name: operator.to_owned(),
                    args: vec![
                        SArrayItem::new(*actual(second)?, false),
                        SArrayItem::new(*actual(third)?, false),
                    ],
                },
                place,
            ),
            6 => node(
                SNode::Coalesce {
                    left: actual(first)?,
                    right: actual(second)?,
                },
                place,
            ),
            7 => node(
                SNode::Let {
                    name: names
                        .get(first)
                        .ok_or_else(|| type_error("unknown scalar assignment binder", Some(place)))?
                        .clone(),
                    ty: None,
                    value: actual(second)?,
                    body: actual(third)?,
                },
                place,
            ),
            8 => node(
                SNode::CellWrite {
                    cell: actual(first)?,
                    value: actual(second)?,
                    body: actual(third)?,
                },
                place,
            ),
            9 => node(
                SNode::If {
                    cond: actual(first)?,
                    then: actual(second)?,
                    otherwise: actual(third)?,
                },
                place,
            ),
            10 => node(
                SNode::Unary {
                    op: UnaryOp::Neg,
                    arg: actual(first)?,
                },
                place,
            ),
            _ => {
                return Err(type_error(
                    "unknown scalar assignment construction operation",
                    Some(place),
                ));
            }
        };
        nodes.push(value);
    }
    nodes
        .pop()
        .ok_or_else(|| type_error("a scalar assignment plan has no root", Some(place)))
}

impl JavaScriptParser {
    pub(super) fn scalar_assignment_expression(
        &mut self,
        target: SExpr,
        operator: &Token,
        postfix: bool,
        read_right: Option<fn(&mut Self) -> Result<SExpr>>,
    ) -> Result<SExpr> {
        let target_span = target.span.unwrap_or_else(|| operator.span());
        let mut place = Span::new(
            target_span.start.min(operator.start),
            target_span.end.max(operator.end),
        );
        let name = match &target.node {
            SNode::Name { path } if path.len() == 1 => path[0].clone(),
            _ => {
                return Err(unsupported(
                    "assignment expression target",
                    "this target needs its actual reference and property state operation",
                    Some(place),
                ));
            }
        };
        let token = self
            .cursor
            .tokens
            .iter()
            .find(|token| {
                token.kind == TokenKind::Identifier
                    && token.value == name
                    && token.start >= target_span.start
                    && token.end <= target_span.end
            })
            .cloned()
            .ok_or_else(|| {
                unsupported(
                    "assignment expression source",
                    "the target needs its actual identifier token",
                    Some(place),
                )
            })?;
        crate::translation::lexer::require_java_script_identifier_role(&token, true)?;
        self.lexical.read(&token, true, place);
        if self.scope.tdz.contains(&name) {
            return Err(unsupported(
                &format!("assignment of {name} before its declaration"),
                "the binding is in its temporal dead zone, where assigning it throws a ReferenceError",
                Some(place),
            ));
        }
        if !self.scope.locals.contains(&name) {
            return Err(unsupported(
                &format!("assignment of {name}"),
                "only local variables declared with let, and parameters, are assignable",
                Some(place),
            ));
        }
        if !self.scope.mutable.contains(&name) {
            return Err(unsupported(
                &format!("assignment of constant {name}"),
                "assigning a const binding throws a TypeError; declare it with let",
                Some(place),
            ));
        }
        if rules::read_scalar_assignment_operation(&operator.value).is_empty() {
            return Err(unsupported(
                "scalar assignment expression",
                "the selected source profile needs the complete shared cell operation",
                Some(place),
            ));
        }
        self.reference_state.enabled = true;
        self.reference_state.forced = true;
        let right = read_right.map(|read| read(self)).transpose()?;
        place.end = place.end.max(
            right
                .as_ref()
                .and_then(|right| right.span)
                .map_or(0, |span| span.end),
        );
        self.generated_count += 1;
        let names = [
            format!(
                "{}temporary_assignment_previous{}",
                self.generated_prefix, self.generated_count
            ),
            format!(
                "{}temporary_assignment_updated{}",
                self.generated_prefix, self.generated_count
            ),
        ];
        materialize(
            node(SNode::Name { path: vec![name] }, token.span()),
            right,
            &operator.value,
            postfix,
            &names,
            place,
        )
    }
}
