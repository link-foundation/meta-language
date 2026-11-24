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
        let source_ml_source2_s1 = if operation == "unicodeCodeUnitLessThan" {
            if target == "JavaScript" {
                self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Return(format!("(({}) < ({}))", left.to_owned(), right.to_owned()))
            } else if target == "Rust" {
                self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::lt({}, {})", left.to_owned(), right.to_owned()))
            } else {
                self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Next
            }
        } else {
            self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Next
        };
        match source_ml_source2_s1 {
            self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Next => {
                {
                    let source_ml_source2_s2 = if operation == "unicodeCodeUnitLessThanOrEqual" {
                        if target == "JavaScript" {
                            self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Return(format!("(({}) <= ({}))", left.to_owned(), right.to_owned()))
                        } else if target == "Rust" {
                                self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::le({}, {})", left.to_owned(), right.to_owned()))
                            } else {
                                self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Next
                            }
                    } else {
                        self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Next
                    };
                    match source_ml_source2_s2 {
                        self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Next => {
                            {
                                let source_ml_source2_s3 = if operation == "unicodeCodeUnitGreaterThan" {
                                    if target == "JavaScript" {
                                        self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Return(format!("(({}) > ({}))", left.to_owned(), right.to_owned()))
                                    } else if target == "Rust" {
                                            self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::gt({}, {})", left.to_owned(), right.to_owned()))
                                        } else {
                                            self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Next
                                        }
                                } else {
                                    self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Next
                                };
                                match source_ml_source2_s3 {
                                    self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Next => {
                                        {
                                            let source_ml_source2_s4 = if operation == "unicodeCodeUnitGreaterThanOrEqual" {
                                                if target == "JavaScript" {
                                                    self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Return(format!("(({}) >= ({}))", left.to_owned(), right.to_owned()))
                                                } else if target == "Rust" {
                                                        self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Return(format!("<crate::ml_string::Text as std::cmp::PartialOrd>::ge({}, {})", left.to_owned(), right.to_owned()))
                                                    } else {
                                                        self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Next
                                                    }
                                            } else {
                                                self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Next
                                            };
                                            match source_ml_source2_s4 {
                                                self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Next => {
                                                    String::new()
                                                }
                                                self::MlSource1TemporaryRenderStringOrderExpressionJoin4::MlSource1TemporaryRenderStringOrderExpressionJoin4Return(source_ml_source1_temporary_result4) => {
                                                    source_ml_source1_temporary_result4
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryRenderStringOrderExpressionJoin3::MlSource1TemporaryRenderStringOrderExpressionJoin3Return(source_ml_source1_temporary_result4_2) => {
                                        source_ml_source1_temporary_result4_2
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryRenderStringOrderExpressionJoin2::MlSource1TemporaryRenderStringOrderExpressionJoin2Return(source_ml_source1_temporary_result4_3) => {
                            source_ml_source1_temporary_result4_3
                        }
                    }
                }
            }
            self::MlSource1TemporaryRenderStringOrderExpressionJoin1::MlSource1TemporaryRenderStringOrderExpressionJoin1Return(source_ml_source1_temporary_result4_4) => {
                source_ml_source1_temporary_result4_4
            }
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
pub enum MlSource1TemporaryRenderStringOrderExpressionJoin4 {
    MlSource1TemporaryRenderStringOrderExpressionJoin4Next,
    MlSource1TemporaryRenderStringOrderExpressionJoin4Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderStringOrderExpressionJoin3 {
    MlSource1TemporaryRenderStringOrderExpressionJoin3Next,
    MlSource1TemporaryRenderStringOrderExpressionJoin3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderStringOrderExpressionJoin2 {
    MlSource1TemporaryRenderStringOrderExpressionJoin2Next,
    MlSource1TemporaryRenderStringOrderExpressionJoin2Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRenderStringOrderExpressionJoin1 {
    MlSource1TemporaryRenderStringOrderExpressionJoin1Next,
    MlSource1TemporaryRenderStringOrderExpressionJoin1Return(String),
}
