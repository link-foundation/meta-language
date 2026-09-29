//! The exactly specified part of `Math` and `Number`: each function is exact
//! or correctly rounded, so every target computes the same binary64 value.

use super::{FLOAT, GLOBALS};
use super::{JavaScriptParser, Result, SArrayItem, SExpr, SNode, Token, node, span, unsupported};

/// The functions, by the path a program calls them by, and the op each is.
const MATH: [(&str, &str); 15] = [
    ("Math.abs", "abs"),
    ("Math.floor", "floor"),
    ("Math.ceil", "ceil"),
    ("Math.trunc", "trunc"),
    ("Math.round", "round"),
    ("Math.sign", "sign"),
    ("Math.sqrt", "sqrt"),
    ("Math.max", "max"),
    ("Math.min", "min"),
    ("Number.isInteger", "isInteger"),
    ("Number.isSafeInteger", "isSafeInteger"),
    ("Number.isFinite", "isFinite"),
    ("Number.isNaN", "isNaN"),
    ("isFinite", "isFinite"),
    ("isNaN", "isNaN"),
];

/// Their constants, as the source text of the Number each one is.
const CONSTANTS: [(&str, &str); 16] = [
    ("Math.PI", "3.141592653589793"),
    ("Math.E", "2.718281828459045"),
    ("Math.LN2", "0.6931471805599453"),
    ("Math.LN10", "2.302585092994046"),
    ("Math.LOG2E", "1.4426950408889634"),
    ("Math.LOG10E", "0.4342944819032518"),
    ("Math.SQRT2", "1.4142135623730951"),
    ("Math.SQRT1_2", "0.7071067811865476"),
    ("Number.MAX_SAFE_INTEGER", "9007199254740991"),
    ("Number.MIN_SAFE_INTEGER", "-9007199254740991"),
    ("Number.EPSILON", "2.220446049250313e-16"),
    ("Number.MAX_VALUE", "1.7976931348623157e+308"),
    ("Number.MIN_VALUE", "5e-324"),
    ("Number.POSITIVE_INFINITY", "Infinity"),
    ("Number.NEGATIVE_INFINITY", "-Infinity"),
    ("Number.NaN", "NaN"),
];

fn lookup(table: &[(&'static str, &'static str)], name: &str) -> Option<&'static str> {
    table
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
}

impl JavaScriptParser {
    /// A Math or Number constant, or a call of one of their functions, at the
    /// global path `name` that `token` starts; `None` for any other path.
    pub(super) fn math(&mut self, token: &Token, name: &str) -> Result<Option<SExpr>> {
        let called = self.cursor.is("(");
        if let Some(value) = lookup(&CONSTANTS, name).filter(|_| !called) {
            let negative = value.starts_with('-');
            return Ok(Some(node(
                SNode::Num {
                    value: value.trim_start_matches('-').to_owned(),
                    ty: Some(FLOAT),
                    negative,
                    unit: false,
                },
                self.to_here(token),
            )));
        }
        let Some(op) = lookup(&MATH, name).filter(|_| called) else {
            return Ok(None);
        };
        let args = self.spread_arguments(name)?;
        Ok(Some(node(
            SNode::Math {
                op: op.to_owned(),
                name: name.to_owned(),
                args,
            },
            self.to_here(token),
        )))
    }

    /// Arguments that may spread arrays, `(a, ...xs)`, as array items.
    fn spread_arguments(&mut self, context: &str) -> Result<Vec<SArrayItem>> {
        self.cursor.expect("(", Some(context))?;
        let mut args = Vec::new();
        while !self.cursor.is(")") {
            let spread = self.cursor.eat("...").is_some();
            args.push(SArrayItem {
                spread,
                value: self.expr()?,
            });
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect(")", Some(context))?;
        Ok(args)
    }
}

/// A top-level declaration may not shadow a global the translation reads as the built-in.
pub(super) fn global(token: &Token) -> Result<()> {
    if GLOBALS.contains(&token.value.as_str()) || token.value == "String" {
        return Err(unsupported(
            &format!("declaration of {}", token.value),
            &format!(
                "it shadows the JavaScript global {}, which the translation reads as the built-in; rename it",
                token.value
            ),
            Some(span(token, token)),
        ));
    }
    Ok(())
}
