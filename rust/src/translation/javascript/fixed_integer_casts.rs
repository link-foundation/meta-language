//! Allocate existing surface terms from the automatically translated cast plan.
use super::{
    BinaryOp, FLOAT, INT, JavaScriptParser, Result, SExpr, SNode, Span, TokenKind, node,
    unsupported,
};
use crate::translation::fixed_integer_cast_rules::plan_fixed_integer_cast_expression;

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
        let bits = match args.first().map(|arg| &arg.node) {
            Some(SNode::Num {
                value,
                ty: Some(ty),
                negative: false,
                ..
            }) if *ty == FLOAT && args.len() == 2 => value.parse::<f64>().ok(),
            _ => None,
        }
        .ok_or_else(|| {
            unsupported(
                name,
                "the width must be a supported constant Number and the value must have BigInt type",
                Some(place),
            )
        })?;
        let plan = plan_fixed_integer_cast_expression(bits, name == "BigInt.asIntN");
        if plan.is_empty() {
            return Err(unsupported(
                name,
                "a width of 8, 16, 32, 64 or 128 is represented; other widths need an integer-domain extension",
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
        let mut names = Vec::new();
        while names.len() < 3 {
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
                "input" => args[1].clone(),
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
        Ok(terms.pop())
    }
}
