// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeJoin5 {
    MlSource1TemporaryDecodeUnicodeEscapeJoin5Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeLoop6Result {
    MlSource1TemporaryDecodeUnicodeEscapeLoop6Done(f64, f64),
}

pub fn source_ml_source1_temporary_decode_unicode_escape_loop6(
    mut units: Vec<f64>,
    mut index: f64,
    mut low: f64,
    mut offset: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop6Result {
    loop {
        return if (offset < (6f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index + offset),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if ({
                            let source_ml_source2_comparison_operand124 = unit.clone();
                            {
                                let source_ml_source2_comparison_operand125 = 48f64;
                                if match (
                                    source_ml_source2_comparison_operand124.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) >= source_ml_source2_comparison_operand125)
                                } else {
                                    ((source_ml_source2_comparison_operand124
                                        .clone()
                                        .expect("a proven present value"))
                                        >= source_ml_source2_comparison_operand125)
                                }
                            }
                        } && {
                            let source_ml_source2_comparison_operand126 = unit.clone();
                            {
                                let source_ml_source2_comparison_operand127 = 57f64;
                                if match (
                                    source_ml_source2_comparison_operand126.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) <= source_ml_source2_comparison_operand127)
                                } else {
                                    ((source_ml_source2_comparison_operand126
                                        .clone()
                                        .expect("a proven present value"))
                                        <= source_ml_source2_comparison_operand127)
                                }
                            }
                        }) {
                            {
                                (match unit.clone() {
                                    Some(ml_present) => ml_present,
                                    None => f64::NAN,
                                } - 48f64)
                            }
                        } else {
                            if ({
                                let source_ml_source2_comparison_operand128 = unit.clone();
                                {
                                    let source_ml_source2_comparison_operand129 = 65f64;
                                    if match (
                                        source_ml_source2_comparison_operand128.clone(),
                                        None::<f64>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        ((f64::NAN) >= source_ml_source2_comparison_operand129)
                                    } else {
                                        ((source_ml_source2_comparison_operand128
                                            .clone()
                                            .expect("a proven present value"))
                                            >= source_ml_source2_comparison_operand129)
                                    }
                                }
                            } && {
                                let source_ml_source2_comparison_operand130 = unit.clone();
                                {
                                    let source_ml_source2_comparison_operand131 = 70f64;
                                    if match (
                                        source_ml_source2_comparison_operand130.clone(),
                                        None::<f64>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        ((f64::NAN) <= source_ml_source2_comparison_operand131)
                                    } else {
                                        ((source_ml_source2_comparison_operand130
                                            .clone()
                                            .expect("a proven present value"))
                                            <= source_ml_source2_comparison_operand131)
                                    }
                                }
                            }) {
                                {
                                    let digit_3 = (match unit.clone() {
                                        Some(ml_present) => ml_present,
                                        None => f64::NAN,
                                    } - 55f64);
                                    digit_3
                                }
                            } else {
                                if ({
                                    let source_ml_source2_comparison_operand132 = unit.clone();
                                    {
                                        let source_ml_source2_comparison_operand133 = 97f64;
                                        if match (
                                            source_ml_source2_comparison_operand132.clone(),
                                            None::<f64>,
                                        ) {
                                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            ((f64::NAN) >= source_ml_source2_comparison_operand133)
                                        } else {
                                            ((source_ml_source2_comparison_operand132
                                                .clone()
                                                .expect("a proven present value"))
                                                >= source_ml_source2_comparison_operand133)
                                        }
                                    }
                                } && {
                                    let source_ml_source2_comparison_operand134 = unit.clone();
                                    {
                                        let source_ml_source2_comparison_operand135 = 102f64;
                                        if match (
                                            source_ml_source2_comparison_operand134.clone(),
                                            None::<f64>,
                                        ) {
                                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            ((f64::NAN) <= source_ml_source2_comparison_operand135)
                                        } else {
                                            ((source_ml_source2_comparison_operand134
                                                .clone()
                                                .expect("a proven present value"))
                                                <= source_ml_source2_comparison_operand135)
                                        }
                                    }
                                }) {
                                    {
                                        let digit_4 = (match unit.clone() {
                                            Some(ml_present) => ml_present,
                                            None => f64::NAN,
                                        } - 87f64);
                                        digit_4
                                    }
                                } else {
                                    digit
                                }
                            }
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop6Result::MlSource1TemporaryDecodeUnicodeEscapeLoop6Done(low, offset)
                        } else {
                            {
                                let low_2 = ((low * 16f64) + digit_5);
                                {
                                    let offset_2 = (offset + 1f64);
                                    {
                                        (low, offset) = (low_2, offset_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop6Result::MlSource1TemporaryDecodeUnicodeEscapeLoop6Done(low, offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeJoin4 {
    MlSource1TemporaryDecodeUnicodeEscapeJoin4Next(f64),
    MlSource1TemporaryDecodeUnicodeEscapeJoin4Return(
        Box<crate::translation::frontend_rules::UnicodeEscape>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeLoop3Result {
    MlSource1TemporaryDecodeUnicodeEscapeLoop3Done(f64, f64, f64),
}

pub fn source_ml_source1_temporary_decode_unicode_escape_loop3(
    mut units: Vec<f64>,
    mut fixed: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
    mut braced: bool,
) -> crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop3Result {
    loop {
        return if ((index < (units.len() as f64)) && (digits < (if braced { 6f64 } else { fixed })))
        {
            {
                let unit = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if ({
                            let source_ml_source2_comparison_operand136 = unit.clone();
                            {
                                let source_ml_source2_comparison_operand137 = 48f64;
                                if match (
                                    source_ml_source2_comparison_operand136.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) >= source_ml_source2_comparison_operand137)
                                } else {
                                    ((source_ml_source2_comparison_operand136
                                        .clone()
                                        .expect("a proven present value"))
                                        >= source_ml_source2_comparison_operand137)
                                }
                            }
                        } && {
                            let source_ml_source2_comparison_operand138 = unit.clone();
                            {
                                let source_ml_source2_comparison_operand139 = 57f64;
                                if match (
                                    source_ml_source2_comparison_operand138.clone(),
                                    None::<f64>,
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    ((f64::NAN) <= source_ml_source2_comparison_operand139)
                                } else {
                                    ((source_ml_source2_comparison_operand138
                                        .clone()
                                        .expect("a proven present value"))
                                        <= source_ml_source2_comparison_operand139)
                                }
                            }
                        }) {
                            {
                                (match unit.clone() {
                                    Some(ml_present) => ml_present,
                                    None => f64::NAN,
                                } - 48f64)
                            }
                        } else {
                            if ({
                                let source_ml_source2_comparison_operand140 = unit.clone();
                                {
                                    let source_ml_source2_comparison_operand141 = 65f64;
                                    if match (
                                        source_ml_source2_comparison_operand140.clone(),
                                        None::<f64>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        ((f64::NAN) >= source_ml_source2_comparison_operand141)
                                    } else {
                                        ((source_ml_source2_comparison_operand140
                                            .clone()
                                            .expect("a proven present value"))
                                            >= source_ml_source2_comparison_operand141)
                                    }
                                }
                            } && {
                                let source_ml_source2_comparison_operand142 = unit.clone();
                                {
                                    let source_ml_source2_comparison_operand143 = 70f64;
                                    if match (
                                        source_ml_source2_comparison_operand142.clone(),
                                        None::<f64>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        ((f64::NAN) <= source_ml_source2_comparison_operand143)
                                    } else {
                                        ((source_ml_source2_comparison_operand142
                                            .clone()
                                            .expect("a proven present value"))
                                            <= source_ml_source2_comparison_operand143)
                                    }
                                }
                            }) {
                                {
                                    let digit_3 = (match unit.clone() {
                                        Some(ml_present) => ml_present,
                                        None => f64::NAN,
                                    } - 55f64);
                                    digit_3
                                }
                            } else {
                                if ({
                                    let source_ml_source2_comparison_operand144 = unit.clone();
                                    {
                                        let source_ml_source2_comparison_operand145 = 97f64;
                                        if match (
                                            source_ml_source2_comparison_operand144.clone(),
                                            None::<f64>,
                                        ) {
                                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            ((f64::NAN) >= source_ml_source2_comparison_operand145)
                                        } else {
                                            ((source_ml_source2_comparison_operand144
                                                .clone()
                                                .expect("a proven present value"))
                                                >= source_ml_source2_comparison_operand145)
                                        }
                                    }
                                } && {
                                    let source_ml_source2_comparison_operand146 = unit.clone();
                                    {
                                        let source_ml_source2_comparison_operand147 = 102f64;
                                        if match (
                                            source_ml_source2_comparison_operand146.clone(),
                                            None::<f64>,
                                        ) {
                                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            ((f64::NAN) <= source_ml_source2_comparison_operand147)
                                        } else {
                                            ((source_ml_source2_comparison_operand146
                                                .clone()
                                                .expect("a proven present value"))
                                                <= source_ml_source2_comparison_operand147)
                                        }
                                    }
                                }) {
                                    {
                                        let digit_4 = (match unit.clone() {
                                            Some(ml_present) => ml_present,
                                            None => f64::NAN,
                                        } - 87f64);
                                        digit_4
                                    }
                                } else {
                                    digit
                                }
                            }
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop3Result::MlSource1TemporaryDecodeUnicodeEscapeLoop3Done(index, code, digits)
                        } else {
                            {
                                let code_2 = ((code * 16f64) + digit_5);
                                {
                                    let digits_2 = (digits + 1f64);
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            (index, code, digits) = (index_2, code_2, digits_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryDecodeUnicodeEscapeLoop3Result::MlSource1TemporaryDecodeUnicodeEscapeLoop3Done(index, code, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeJoin2 {
    MlSource1TemporaryDecodeUnicodeEscapeJoin2Next(f64, bool),
    MlSource1TemporaryDecodeUnicodeEscapeJoin2Return(
        Box<crate::translation::frontend_rules::UnicodeEscape>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeUnicodeEscapeJoin1 {
    MlSource1TemporaryDecodeUnicodeEscapeJoin1Next(f64),
    MlSource1TemporaryDecodeUnicodeEscapeJoin1Return(
        Box<crate::translation::frontend_rules::UnicodeEscape>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptArgumentCountLoop7Result {
    MlSource1TemporaryAcceptArgumentCountLoop7Done,
    MlSource1TemporaryAcceptArgumentCountLoop7Return(bool),
}

pub fn source_ml_source1_temporary_accept_argument_count_loop7(
    mut defaults: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryAcceptArgumentCountLoop7Result {
    loop {
        return if (index < (defaults.len() as f64)) {
            if !(match crate::translation::frontend_rules::ml_array::optional_at(
                &defaults,
                crate::translation::frontend_rules::ml_array::number_index(index),
            ) {
                Some(ml_present) => ml_present,
                None => false,
            }) {
                crate::translation::frontend_rules::MlSource1TemporaryAcceptArgumentCountLoop7Result::MlSource1TemporaryAcceptArgumentCountLoop7Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryAcceptArgumentCountLoop7Result::MlSource1TemporaryAcceptArgumentCountLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindDefaultParameterReferenceLoop8Result {
    MlSource1TemporaryFindDefaultParameterReferenceLoop8Done,
    MlSource1TemporaryFindDefaultParameterReferenceLoop8Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindDefaultParameterReferenceJoin9 {
    MlSource1TemporaryFindDefaultParameterReferenceJoin9Next,
    MlSource1TemporaryFindDefaultParameterReferenceJoin9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindDefaultParameterReferenceLoop10Result {
    MlSource1TemporaryFindDefaultParameterReferenceLoop10Done,
    MlSource1TemporaryFindDefaultParameterReferenceLoop10Return(f64),
}

pub fn source_ml_source1_temporary_find_default_parameter_reference_loop10(
    mut parameters: Vec<String>,
    mut at: f64,
    mut value: Option<String>,
    mut parameter_index: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop10Result
{
    loop {
        return if (parameter_index < (parameters.len() as f64)) {
            if {
                let source_ml_source2_comparison_operand148 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &parameters,
                        crate::translation::frontend_rules::ml_array::number_index(parameter_index),
                    );
                {
                    let source_ml_source2_comparison_operand149 = value.clone();
                    match (
                        source_ml_source2_comparison_operand148.clone(),
                        source_ml_source2_comparison_operand149.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop10Result::MlSource1TemporaryFindDefaultParameterReferenceLoop10Return(at)
            } else {
                {
                    let parameter_index_2 = (parameter_index + 1f64);
                    {
                        parameter_index = parameter_index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop10Result::MlSource1TemporaryFindDefaultParameterReferenceLoop10Done
        };
    }
}

pub fn source_ml_source1_temporary_find_default_parameter_reference_loop8(
    mut kinds: Vec<String>,
    mut values: Vec<String>,
    mut parameters: Vec<String>,
    mut at: f64,
    mut depth: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop8Result
{
    loop {
        return if (((at < (values.len() as f64)) && {
            let source_ml_source2_comparison_operand150 =
                crate::translation::frontend_rules::ml_array::optional_at(
                    &kinds,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                );
            {
                let source_ml_source2_comparison_operand151 = String::from("eof");
                !(match (
                    source_ml_source2_comparison_operand150.clone(),
                    source_ml_source2_comparison_operand151.clone(),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        }) && ((depth > (0f64))
            || ({
                let source_ml_source2_comparison_operand152 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand153 = String::from(",");
                    !(match (
                        source_ml_source2_comparison_operand152.clone(),
                        source_ml_source2_comparison_operand153.clone(),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } && {
                let source_ml_source2_comparison_operand154 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand155 = String::from(")");
                    !(match (
                        source_ml_source2_comparison_operand154.clone(),
                        source_ml_source2_comparison_operand155.clone(),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            }))) {
            {
                let value = crate::translation::frontend_rules::ml_array::optional_at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                );
                {
                    let depth_3 = if (({
                        let source_ml_source2_comparison_operand156 = value.clone();
                        {
                            let source_ml_source2_comparison_operand157 = String::from("(");
                            match (
                                source_ml_source2_comparison_operand156.clone(),
                                source_ml_source2_comparison_operand157.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } || {
                        let source_ml_source2_comparison_operand158 = value.clone();
                        {
                            let source_ml_source2_comparison_operand159 = String::from("[");
                            match (
                                source_ml_source2_comparison_operand158.clone(),
                                source_ml_source2_comparison_operand159.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    }) || {
                        let source_ml_source2_comparison_operand160 = value.clone();
                        {
                            let source_ml_source2_comparison_operand161 = String::from("{");
                            match (
                                source_ml_source2_comparison_operand160.clone(),
                                source_ml_source2_comparison_operand161.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    }) {
                        { (depth + 1f64) }
                    } else {
                        depth
                    };
                    {
                        let depth_5 = if (({
                            let source_ml_source2_comparison_operand162 = value.clone();
                            {
                                let source_ml_source2_comparison_operand163 = String::from(")");
                                match (
                                    source_ml_source2_comparison_operand162.clone(),
                                    source_ml_source2_comparison_operand163.clone(),
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_source2_comparison_operand164 = value.clone();
                            {
                                let source_ml_source2_comparison_operand165 = String::from("]");
                                match (
                                    source_ml_source2_comparison_operand164.clone(),
                                    source_ml_source2_comparison_operand165.clone(),
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }) || {
                            let source_ml_source2_comparison_operand166 = value.clone();
                            {
                                let source_ml_source2_comparison_operand167 = String::from("}");
                                match (
                                    source_ml_source2_comparison_operand166.clone(),
                                    source_ml_source2_comparison_operand167.clone(),
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }) {
                            { (depth_3 - 1f64) }
                        } else {
                            depth_3
                        };
                        {
                            let source_ml_source2_s173 = if ({
                                let source_ml_source2_comparison_operand174 =
                                    crate::translation::frontend_rules::ml_array::optional_at(
                                        &kinds,
                                        crate::translation::frontend_rules::ml_array::number_index(
                                            at,
                                        ),
                                    );
                                {
                                    let source_ml_source2_comparison_operand175 =
                                        String::from("identifier");
                                    match (
                                        source_ml_source2_comparison_operand174.clone(),
                                        source_ml_source2_comparison_operand175.clone(),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                }
                            } && !((at > (0f64)) && {
                                let source_ml_source2_comparison_operand176 =
                                    crate::translation::frontend_rules::ml_array::optional_at(
                                        &values,
                                        crate::translation::frontend_rules::ml_array::number_index(
                                            (at - 1f64),
                                        ),
                                    );
                                {
                                    let source_ml_source2_comparison_operand177 = String::from(".");
                                    match (
                                        source_ml_source2_comparison_operand176.clone(),
                                        source_ml_source2_comparison_operand177.clone(),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                }
                            })) {
                                {
                                    let parameter_index = 0f64;
                                    {
                                        let source_ml_source2_s178 = crate::translation::frontend_rules::source_ml_source1_temporary_find_default_parameter_reference_loop10(parameters.clone(), at, value.clone(), parameter_index);
                                        match source_ml_source2_s178.clone() {
                                            crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop10Result::MlSource1TemporaryFindDefaultParameterReferenceLoop10Done => {
                                                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceJoin9::MlSource1TemporaryFindDefaultParameterReferenceJoin9Next
                                            }
                                            crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop10Result::MlSource1TemporaryFindDefaultParameterReferenceLoop10Return(source_ml_source1_temporary_result10) => {
                                                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceJoin9::MlSource1TemporaryFindDefaultParameterReferenceJoin9Return(source_ml_source1_temporary_result10)
                                            }
                                        }
                                    }
                                }
                            } else {
                                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceJoin9::MlSource1TemporaryFindDefaultParameterReferenceJoin9Next
                            };
                            match source_ml_source2_s173.clone() {
                                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceJoin9::MlSource1TemporaryFindDefaultParameterReferenceJoin9Next => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            (at, depth) = (at_2, depth_5);
                                            continue;
                                        }
                                    }
                                }
                                crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceJoin9::MlSource1TemporaryFindDefaultParameterReferenceJoin9Return(source_ml_source1_temporary_result9) => {
                                    crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop8Result::MlSource1TemporaryFindDefaultParameterReferenceLoop8Return(source_ml_source1_temporary_result9)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryFindDefaultParameterReferenceLoop8Result::MlSource1TemporaryFindDefaultParameterReferenceLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRegularExpressionEndLoop11Result {
    MlSource1TemporaryRegularExpressionEndLoop11Done,
    MlSource1TemporaryRegularExpressionEndLoop11Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRegularExpressionEndJoin12 {
    MlSource1TemporaryRegularExpressionEndJoin12Next(f64, bool, bool),
    MlSource1TemporaryRegularExpressionEndJoin12Return(f64),
}

pub fn source_ml_source1_temporary_regular_expression_end_loop13(
    mut units: Vec<f64>,
    mut index: f64,
) -> f64 {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let flag = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if !(({
                    let source_ml_source2_comparison_operand179 = flag.clone();
                    {
                        let source_ml_source2_comparison_operand180 = 65f64;
                        if match (source_ml_source2_comparison_operand179.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand180)
                        } else {
                            ((source_ml_source2_comparison_operand179
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand180)
                        }
                    }
                } && {
                    let source_ml_source2_comparison_operand181 = flag.clone();
                    {
                        let source_ml_source2_comparison_operand182 = 90f64;
                        if match (source_ml_source2_comparison_operand181.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) <= source_ml_source2_comparison_operand182)
                        } else {
                            ((source_ml_source2_comparison_operand181
                                .clone()
                                .expect("a proven present value"))
                                <= source_ml_source2_comparison_operand182)
                        }
                    }
                }) || ({
                    let source_ml_source2_comparison_operand183 = flag.clone();
                    {
                        let source_ml_source2_comparison_operand184 = 97f64;
                        if match (source_ml_source2_comparison_operand183.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand184)
                        } else {
                            ((source_ml_source2_comparison_operand183
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand184)
                        }
                    }
                } && {
                    let source_ml_source2_comparison_operand185 = flag.clone();
                    {
                        let source_ml_source2_comparison_operand186 = 122f64;
                        if match (source_ml_source2_comparison_operand185.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) <= source_ml_source2_comparison_operand186)
                        } else {
                            ((source_ml_source2_comparison_operand185
                                .clone()
                                .expect("a proven present value"))
                                <= source_ml_source2_comparison_operand186)
                        }
                    }
                })) {
                    index
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            index
        };
    }
}
