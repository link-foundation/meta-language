//! Allocate existing surface terms from the automatically translated cast plan.
use super::{
    BinaryOp, FLOAT, INT, JavaScriptParser, Result, SExpr, SNode, Span, TokenKind, UnaryOp, node,
    unsupported,
};
use crate::translation::javascript_fixed_integer_cast_plan::{
    plan_java_script_fixed_integer_arguments, plan_java_script_fixed_integer_cast_expression,
};

impl JavaScriptParser {
    pub(super) fn read_fixed_integer_cast(
        &mut self,
        name: &str,
        args: &[SExpr],
        place: Span,
    ) -> Result<Option<SExpr>> {
        if name != "BigInt.asIntN" && name != "BigInt.asUintN" {
            return Ok(None);
        }
        let mut width = args.first();
        let mut sign = 1.0;
        for _ in 0..64 {
            match width.map(|arg| &arg.node) {
                Some(SNode::Unary {
                    op: UnaryOp::Neg,
                    arg,
                }) => {
                    sign = -sign;
                    width = Some(arg.as_ref());
                }
                _ => break,
            }
        }
        let bits = match width.map(|arg| &arg.node) {
            Some(SNode::Num {
                value,
                ty: Some(ty),
                negative,
                ..
            }) if *ty == FLOAT && (2..=4096).contains(&args.len()) => value
                .parse::<f64>()
                .ok()
                .map(|value| value * if *negative { -sign } else { sign }),
            _ => None,
        }
        .ok_or_else(|| {
            unsupported(
                name,
                "the width must be a supported constant Number and the value must have BigInt type",
                Some(place),
            )
        })?;
        let plan = plan_java_script_fixed_integer_cast_expression(bits, name == "BigInt.asIntN");
        if plan.is_empty() {
            return Err(unsupported(
                name,
                "constant widths whose ToIndex result is between 0 and 4096 are represented; other widths require their complete conversion and resource model",
                Some(place),
            ));
        }
        if self.record_parameter_names.is_none() {
            self.record_parameter_names = Some(
                self.cursor
                    .tokens
                    .iter()
                    .filter(|token| token.kind == TokenKind::Identifier)
                    .map(|token| token.value.clone())
                    .chain(self.externals.iter().map(|item| item.name().to_owned()))
                    .chain(self.literal_bindings.keys().cloned())
                    .chain(self.global_binding_names.iter().cloned())
                    .collect(),
            );
        }
        let captures = plan_java_script_fixed_integer_arguments(args.len() as f64);
        let mut names = Vec::new();
        while names.len() < 3 + captures.len() {
            if self.record_parameter_ordinal >= 9_007_199_254_740_991 {
                return Err(unsupported(
                    name,
                    "fresh binding names exhausted their safe ordinal range",
                    Some(place),
                ));
            }
            let candidate = format!("fixed_integer_cast_value{}", self.record_parameter_ordinal);
            self.record_parameter_ordinal += 1;
            if self
                .record_parameter_names
                .as_mut()
                .expect("initialized names")
                .insert(candidate.clone())
            {
                names.push(candidate);
            }
        }
        let mut terms: Vec<SExpr> = Vec::new();
        for instruction in plan.chunks_exact(5) {
            let term = |cell: usize| -> Result<SExpr> {
                instruction[cell].parse::<usize>().ok().and_then(|at| terms.get(at)).cloned()
                    .ok_or_else(|| unsupported(name, "the shared construction position must reference an earlier integer surface term", Some(place)))
            };
            let binding = || -> Result<String> {
                instruction[1]
                    .parse::<usize>()
                    .ok()
                    .and_then(|at| names.get(at))
                    .cloned()
                    .ok_or_else(|| {
                        unsupported(
                            name,
                            "the shared construction binding must belong to the fresh local slots",
                            Some(place),
                        )
                    })
            };
            let value = match instruction[0].as_str() {
                "input" => {
                    if captures.is_empty() {
                        args[1].clone()
                    } else {
                        node(
                            SNode::Name {
                                path: vec![names[3].clone()],
                            },
                            place,
                        )
                    }
                }
                "integer" => node(
                    SNode::Num {
                        value: instruction[1].clone(),
                        ty: Some(INT),
                        negative: false,
                        unit: false,
                    },
                    place,
                ),
                "reference" => node(
                    SNode::Name {
                        path: vec![binding()?],
                    },
                    place,
                ),
                "binary" => {
                    let op = match instruction[1].as_str() {
                        "rem" => BinaryOp::Rem,
                        "lt" => BinaryOp::Lt,
                        "add" => BinaryOp::Add,
                        "ge" => BinaryOp::Ge,
                        "sub" => BinaryOp::Sub,
                        _ => {
                            return Err(unsupported(
                                name,
                                "the shared construction operator must be an existing integer operation",
                                Some(place),
                            ));
                        }
                    };
                    node(
                        SNode::Binary {
                            op,
                            left: Box::new(term(2)?),
                            right: Box::new(term(3)?),
                            rounding: None,
                        },
                        place,
                    )
                }
                "conditional" => node(
                    SNode::If {
                        cond: Box::new(term(2)?),
                        then: Box::new(term(3)?),
                        otherwise: Box::new(term(4)?),
                    },
                    place,
                ),
                "let" => node(
                    SNode::Let {
                        name: binding()?,
                        ty: None,
                        value: Box::new(term(2)?),
                        body: Box::new(term(3)?),
                    },
                    place,
                ),
                _ => {
                    return Err(unsupported(
                        name,
                        "the shared construction instruction must allocate an existing integer surface term",
                        Some(place),
                    ));
                }
            };
            terms.push(value);
        }
        let mut body = terms
            .pop()
            .expect("a complete shared cast plan has a result");
        for (slot, coordinate) in captures.iter().enumerate().rev() {
            body = node(
                SNode::Let {
                    name: names[slot + 3].clone(),
                    ty: None,
                    value: Box::new(args[*coordinate as usize].clone()),
                    body: Box::new(body),
                },
                place,
            );
        }
        Ok(Some(body))
    }
}
