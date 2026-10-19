// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_order_operation(language: &str, operation: &str, kind: &str) -> String {
    if (language != "JavaScript") || (kind != "string") {
        String::new()
    } else if operation == "lt" {
        String::from("unicodeCodeUnitLessThan")
    } else if operation == "le" {
        String::from("unicodeCodeUnitLessThanOrEqual")
    } else if operation == "gt" {
        String::from("unicodeCodeUnitGreaterThan")
    } else if operation == "ge" {
        String::from("unicodeCodeUnitGreaterThanOrEqual")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_string_order_units(expression: &str, operation: &str) -> bool {
    (expression == "stringTest")
        && ((((operation == "unicodeCodeUnitLessThan")
            || (operation == "unicodeCodeUnitLessThanOrEqual"))
            || (operation == "unicodeCodeUnitGreaterThan"))
            || (operation == "unicodeCodeUnitGreaterThanOrEqual"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_string_order_expression(
    target: &str,
    operation: &str,
    left: &str,
    right: &str,
) -> String {
    {
        let ml_s1 = if operation == "unicodeCodeUnitLessThan" {
            if target == "JavaScript" {
                self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Return(
                    format!("(({}) < ({}))", left.to_owned(), right.to_owned()),
                )
            } else if target == "Rust" {
                self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Return(
                    format!(
                        "<crate::ml_string::Text as std::cmp::PartialOrd>::lt({}, {})",
                        left.to_owned(),
                        right.to_owned()
                    ),
                )
            } else {
                self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Next
            }
        } else {
            self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Next
        };
        match ml_s1 {
            self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Next => {
                let ml_s2 = if operation == "unicodeCodeUnitLessThanOrEqual" {
                    if target == "JavaScript" {
                        self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Return(format!("(({}) <= ({}))", left.to_owned(), right.to_owned()))
                    } else if target == "Rust" {
                        self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::le({}, {})", left.to_owned(), right.to_owned()))
                    } else {
                        self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Next
                    }
                } else {
                    self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Next
                };
                match ml_s2 {
                        self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Next => {
                            {
                                let ml_s3 = if operation == "unicodeCodeUnitGreaterThan" {
                                    if target == "JavaScript" {
                                        self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Return(format!("(({}) > ({}))", left.to_owned(), right.to_owned()))
                                    } else if target == "Rust" {
                                            self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::gt({}, {})", left.to_owned(), right.to_owned()))
                                        } else {
                                            self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Next
                                        }
                                } else {
                                    self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Next
                                };
                                match ml_s3 {
                                    self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Next => {
                                        {
                                            let ml_s4 = if operation == "unicodeCodeUnitGreaterThanOrEqual" {
                                                if target == "JavaScript" {
                                                    self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Return(format!("(({}) >= ({}))", left.to_owned(), right.to_owned()))
                                                } else if target == "Rust" {
                                                        self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::ge({}, {})", left.to_owned(), right.to_owned()))
                                                    } else {
                                                        self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Next
                                                    }
                                            } else {
                                                self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Next
                                            };
                                            match ml_s4 {
                                                self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Next => {
                                                    String::new()
                                                }
                                                self::MlRenderStringOrderExpressionJoin4::MlRenderStringOrderExpressionJoin4Return(ml_result4) => {
                                                    ml_result4
                                                }
                                            }
                                        }
                                    }
                                    self::MlRenderStringOrderExpressionJoin3::MlRenderStringOrderExpressionJoin3Return(ml_result4_2) => {
                                        ml_result4_2
                                    }
                                }
                            }
                        }
                        self::MlRenderStringOrderExpressionJoin2::MlRenderStringOrderExpressionJoin2Return(ml_result4_3) => {
                            ml_result4_3
                        }
                    }
            }
            self::MlRenderStringOrderExpressionJoin1::MlRenderStringOrderExpressionJoin1Return(
                ml_result4_4,
            ) => ml_result4_4,
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_order_refusal(target: &str, operation: &str) -> String {
    if (!(self::require_string_order_units("stringTest", operation)) || (target == "JavaScript"))
        || (target == "Rust")
    {
        String::new()
    } else {
        String::from(
            "this target needs a UTF-16 operational string encoding for lexical comparison",
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlRenderStringOrderExpressionJoin4 {
    MlRenderStringOrderExpressionJoin4Next,
    MlRenderStringOrderExpressionJoin4Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlRenderStringOrderExpressionJoin3 {
    MlRenderStringOrderExpressionJoin3Next,
    MlRenderStringOrderExpressionJoin3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlRenderStringOrderExpressionJoin2 {
    MlRenderStringOrderExpressionJoin2Next,
    MlRenderStringOrderExpressionJoin2Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlRenderStringOrderExpressionJoin1 {
    MlRenderStringOrderExpressionJoin1Next,
    MlRenderStringOrderExpressionJoin1Return(String),
}
