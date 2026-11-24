//! Actual type/expression adapters for shared string construction definitions.
use super::{
    BOOL, BinaryOp, Checker, Env, Expr, FLOAT, Language, LitValue, Node, Result, SArrayItem,
    STRING, Span, Type, array, plain_binary, type_error, unsupported,
};
use crate::translation::string_construction_rules as rules;
use crate::translation::types::option;

fn number(value: impl ToString) -> Expr {
    Expr::lit(FLOAT, LitValue::Text(value.to_string()))
}
fn text(value: &str) -> Expr {
    Expr::lit(STRING, LitValue::Text(value.to_owned()))
}
fn column(items: Vec<Expr>) -> Expr {
    Expr::new(Node::Array { items }, array(FLOAT))
}
fn operation(name: &str, value: Expr, ty: Type) -> Expr {
    Expr::new(
        Node::Math {
            op: name.to_owned(),
            args: vec![value],
        },
        ty,
    )
}
fn binding(name: String, value: Expr, body: Expr) -> Expr {
    let ty = body.ty.clone();
    Expr::new(
        Node::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        },
        ty,
    )
}

impl Checker {
    pub(super) fn javascript_string_construction(
        &mut self,
        op: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.iter().any(|item| item.spread) {
            return Err(unsupported(
                "spread string construction arguments",
                "the complete source iterator effects require their represented operations",
                span,
            ));
        }
        let values = args
            .iter()
            .map(|item| self.expr(&item.value, env, path, None, false))
            .collect::<Result<Vec<_>>>()?;
        if values
            .iter()
            .any(|value| !super::number_conversions::accepts(&value.ty))
        {
            return Err(unsupported(
                "string construction primitive conversion",
                "the actual object or symbol primitive conversion needs its represented operation",
                span,
            ));
        }
        let names = values
            .iter()
            .map(|_| self.fresh_name("ml_character_argument"))
            .collect::<Vec<_>>();
        let converted = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                self.materialize_number_conversion(
                    Expr::var(names[index].clone(), value.ty.clone()),
                    "coerceJavaScriptNumber",
                )
            })
            .collect::<Vec<_>>();
        let mut body = if op == "constructJavaScriptStringFromCharacterCodes" {
            operation(
                "convertJavaScriptCharacterCodesToString",
                column(converted.clone()),
                STRING,
            )
        } else {
            text("")
        };
        if op == "constructJavaScriptStringFromCodePoints" {
            for value in converted.into_iter().rev() {
                let input_name = self.fresh_name("ml_character_number");
                let input = Expr::var(input_name.clone(), FLOAT);
                let result_name = self.fresh_name("ml_character_units");
                let result = Expr::var(result_name.clone(), array(FLOAT));
                let at = Expr::new(
                    Node::Coalesce {
                        left: Box::new(Expr::new(
                            Node::OptionalIndex {
                                array: Box::new(result.clone()),
                                index: Box::new(number(0)),
                            },
                            option(FLOAT, "undefined"),
                        )),
                        right: Box::new(number(0)),
                    },
                    FLOAT,
                );
                let rendered = Expr::new(
                    Node::ToString {
                        arg: Box::new(input.clone()),
                        console: false,
                    },
                    STRING,
                );
                let message = plain_binary(
                    BinaryOp::Concat,
                    text("Invalid code point "),
                    rendered,
                    STRING,
                );
                let failure = Expr::new(
                    Node::Math {
                        op: "raiseJavaScriptRangeError".to_owned(),
                        args: vec![message, Expr::lit(BOOL, LitValue::Bool(true))],
                    },
                    STRING,
                );
                let suffix = Expr::new(
                    Node::Method {
                        op: "slice".to_owned(),
                        object: Box::new(result),
                        args: vec![Some(number(1)), Some(number("Infinity"))],
                    },
                    array(FLOAT),
                );
                let character =
                    operation("convertJavaScriptCharacterCodesToString", suffix, STRING);
                let success = plain_binary(BinaryOp::Concat, character, body, STRING);
                let mut condition = plain_binary(BinaryOp::Eq, at, number(-1), BOOL);
                if let Node::Binary { domain, .. } = &mut condition.node {
                    *domain = Some(FLOAT);
                }
                let branches = Expr::new(
                    Node::If {
                        cond: Box::new(condition),
                        then: Box::new(success),
                        otherwise: Box::new(failure),
                    },
                    STRING,
                );
                let converted_point = operation(
                    "convertJavaScriptCodePointUnitsResult",
                    column(vec![input]),
                    array(FLOAT),
                );
                body = binding(
                    input_name,
                    value,
                    binding(result_name, converted_point, branches),
                );
            }
        }
        for (index, value) in values.into_iter().enumerate().rev() {
            body = binding(names[index].clone(), value, body);
        }
        Ok(body)
    }
    pub(super) fn javascript_string_construction_value(
        &mut self,
        op: &str,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        if self.language != Language::JavaScript || args.len() != 1 || args[0].spread {
            return Err(type_error(
                "a JavaScript string construction intrinsic needs one Number column",
                span,
            ));
        }
        let value = self.expr(&args[0].value, env, path, Some(&array(FLOAT)), false)?;
        if value.ty != array(FLOAT) {
            return Err(type_error(
                "a JavaScript string construction intrinsic needs one Number column",
                span,
            ));
        }
        Ok(operation(
            op,
            value,
            if rules::read_java_script_string_construction_form(op) == "characters" {
                STRING
            } else {
                array(FLOAT)
            },
        ))
    }
}
