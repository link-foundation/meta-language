// Generated from complete js/src/translation/record-closure-parameter-rules.js. Do not edit by hand.
use super::continues_java_script_directive_expression;
use super::ml_string;
use super::read_required_boolean_fact;
use super::read_required_text_fact;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result {
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Done,
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop38(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let current = self::read_required_text_fact(kinds, at);
                if current == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-2f64)
                } else if current
                    == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
                {
                    {
                        let raw_ = self::read_required_text_fact(raws, at);
                        {
                            let strict = (raw_
                                == (self::ml_string::Text::from_units(vec![
                                    34, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116, 34,
                                ])))
                                || (raw_
                                    == (self::ml_string::Text::from_units(vec![
                                        39, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116, 39,
                                    ])));
                            {
                                let next = at + 1f64;
                                if next >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-2f64)
                                } else {
                                    {
                                        let kind = self::read_required_text_fact(kinds, next);
                                        {
                                            let value = self::read_required_text_fact(values, next);
                                            if (kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && (value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        59,
                                                    ])))
                                            {
                                                if strict {
                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(at)
                                                } else {
                                                    {
                                                        let at_2 = next + 1f64;
                                                        {
                                                            at = at_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                            } else if (kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && (value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        125,
                                                    ])))
                                            {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(if strict {
                                                            at
                                                        } else {
                                                            -1f64
                                                        })
                                            } else if kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 111, 102,
                                                ]))
                                            {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-2f64)
                                            } else if (self::read_required_boolean_fact(
                                                line_breaks,
                                                next,
                                            )) {
                                                if self::continues_java_script_directive_expression(&(kind), &(value)) {
                                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-1f64)
                                                                } else if ((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((value == (self::ml_string::Text::from_units(vec![123]))) || (value == (self::ml_string::Text::from_units(vec![43, 43])))) || (value == (self::ml_string::Text::from_units(vec![45, 45]))))) {
                                                                        if strict {
                                                                            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(at)
                                                                        } else {
                                                                            {
                                                                                let at_3 = next;
                                                                                {
                                                                                    at = at_3;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-2f64)
                                                                    }
                                            } else {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-1f64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Return(-1f64)
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop38Done
        };
    }
}
