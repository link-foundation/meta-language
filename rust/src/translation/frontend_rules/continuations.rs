// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin5 {
    MlDecodeUnicodeEscapeJoin5Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop6Result {
    MlDecodeUnicodeEscapeLoop6Done(f64, f64),
}

pub fn ml_decode_unicode_escape_loop6(
    mut units: Vec<f64>,
    mut index: f64,
    mut low: f64,
    mut offset: f64,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop6Result {
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
                        let digit_5 = if ((unit.unwrap_or(f64::NAN) >= (48f64))
                            && (unit.unwrap_or(f64::NAN) <= (57f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 48f64) }
                        } else if ((unit.unwrap_or(f64::NAN) >= (65f64))
                            && (unit.unwrap_or(f64::NAN) <= (70f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 55f64) }
                        } else if ((unit.unwrap_or(f64::NAN) >= (97f64))
                            && (unit.unwrap_or(f64::NAN) <= (102f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop6Result::MlDecodeUnicodeEscapeLoop6Done(low, offset)
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
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop6Result::MlDecodeUnicodeEscapeLoop6Done(low, offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin4 {
    MlDecodeUnicodeEscapeJoin4Next(f64),
    MlDecodeUnicodeEscapeJoin4Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop3Result {
    MlDecodeUnicodeEscapeLoop3Done(f64, f64, f64),
}

pub fn ml_decode_unicode_escape_loop3(
    mut units: Vec<f64>,
    mut fixed: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
    mut braced: bool,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop3Result {
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
                        let digit_5 = if ((unit.unwrap_or(f64::NAN) >= (48f64))
                            && (unit.unwrap_or(f64::NAN) <= (57f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 48f64) }
                        } else if ((unit.unwrap_or(f64::NAN) >= (65f64))
                            && (unit.unwrap_or(f64::NAN) <= (70f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 55f64) }
                        } else if ((unit.unwrap_or(f64::NAN) >= (97f64))
                            && (unit.unwrap_or(f64::NAN) <= (102f64)))
                        {
                            { (unit.unwrap_or(f64::NAN) - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop3Result::MlDecodeUnicodeEscapeLoop3Done(index, code, digits)
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
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop3Result::MlDecodeUnicodeEscapeLoop3Done(index, code, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin2 {
    MlDecodeUnicodeEscapeJoin2Next(f64, bool),
    MlDecodeUnicodeEscapeJoin2Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin1 {
    MlDecodeUnicodeEscapeJoin1Next(f64),
    MlDecodeUnicodeEscapeJoin1Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptArgumentCountLoop7Result {
    MlAcceptArgumentCountLoop7Done,
    MlAcceptArgumentCountLoop7Return(bool),
}

pub fn ml_accept_argument_count_loop7(
    mut defaults: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptArgumentCountLoop7Result {
    loop {
        return if (index < (defaults.len() as f64)) {
            if crate::translation::frontend_rules::ml_array::optional_at(
                &defaults,
                crate::translation::frontend_rules::ml_array::number_index(index),
            )
            .unwrap_or_default()
            {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop7Result::MlAcceptArgumentCountLoop7Return(false)
        } else {
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop7Result::MlAcceptArgumentCountLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop8Result {
    MlFindDefaultParameterReferenceLoop8Done,
    MlFindDefaultParameterReferenceLoop8Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceJoin9 {
    MlFindDefaultParameterReferenceJoin9Next,
    MlFindDefaultParameterReferenceJoin9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop10Result {
    MlFindDefaultParameterReferenceLoop10Done,
    MlFindDefaultParameterReferenceLoop10Return(f64),
}

pub fn ml_find_default_parameter_reference_loop10(
    mut parameters: Vec<String>,
    mut at: f64,
    mut value: Option<String>,
    mut parameter_index: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop10Result {
    loop {
        return if (parameter_index < (parameters.len() as f64)) {
            if match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &parameters,
                    crate::translation::frontend_rules::ml_array::number_index(parameter_index),
                ),
                value.clone(),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            } {
                crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop10Result::MlFindDefaultParameterReferenceLoop10Return(at)
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
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop10Result::MlFindDefaultParameterReferenceLoop10Done
        };
    }
}

pub fn ml_find_default_parameter_reference_loop8(
    mut kinds: Vec<String>,
    mut values: Vec<String>,
    mut parameters: Vec<String>,
    mut at: f64,
    mut depth: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop8Result {
    loop {
        return if (((at < (values.len() as f64))
            && !(match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &kinds,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                ),
                String::from("eof"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }))
            && ((depth > (0f64))
                || (!(match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    String::from(","),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) && !(match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    String::from(")"),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))))
        {
            {
                let value = crate::translation::frontend_rules::ml_array::optional_at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                );
                {
                    let depth_3 = if ((match (value.clone(), String::from("(")) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } || match (value.clone(), String::from("[")) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }) || match (value.clone(), String::from("{")) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }) {
                        { (depth + 1f64) }
                    } else {
                        depth
                    };
                    {
                        let depth_5 = if ((match (value.clone(), String::from(")")) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } || match (value.clone(), String::from("]")) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) || match (value.clone(), String::from("}")) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) {
                            { (depth_3 - 1f64) }
                        } else {
                            depth_3
                        };
                        {
                            let ml_s19 = if (match (
                                crate::translation::frontend_rules::ml_array::optional_at(
                                    &kinds,
                                    crate::translation::frontend_rules::ml_array::number_index(at),
                                ),
                                String::from("identifier"),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            } && !((at > (0f64))
                                && match (
                                    crate::translation::frontend_rules::ml_array::optional_at(
                                        &values,
                                        crate::translation::frontend_rules::ml_array::number_index(
                                            (at - 1f64),
                                        ),
                                    ),
                                    String::from("."),
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })) {
                                {
                                    let parameter_index = 0f64;
                                    {
                                        let ml_s20 = crate::translation::frontend_rules::ml_find_default_parameter_reference_loop10(parameters.clone(), at, value.clone(), parameter_index);
                                        match ml_s20.clone() {
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop10Result::MlFindDefaultParameterReferenceLoop10Done => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin9::MlFindDefaultParameterReferenceJoin9Next
                                            }
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop10Result::MlFindDefaultParameterReferenceLoop10Return(ml_result10) => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin9::MlFindDefaultParameterReferenceJoin9Return(ml_result10)
                                            }
                                        }
                                    }
                                }
                            } else {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin9::MlFindDefaultParameterReferenceJoin9Next
                            };
                            match ml_s19.clone() {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin9::MlFindDefaultParameterReferenceJoin9Next => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            (at, depth) = (at_2, depth_5);
                                            continue;
                                        }
                                    }
                                }
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin9::MlFindDefaultParameterReferenceJoin9Return(ml_result9) => {
                                    crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop8Result::MlFindDefaultParameterReferenceLoop8Return(ml_result9)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop8Result::MlFindDefaultParameterReferenceLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndLoop11Result {
    MlRegularExpressionEndLoop11Done,
    MlRegularExpressionEndLoop11Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndJoin12 {
    MlRegularExpressionEndJoin12Next(f64, bool, bool),
    MlRegularExpressionEndJoin12Return(f64),
}

pub fn ml_regular_expression_end_loop13(mut units: Vec<f64>, mut index: f64) -> f64 {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let flag = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if (((flag.unwrap_or(f64::NAN) >= (65f64))
                    && (flag.unwrap_or(f64::NAN) <= (90f64)))
                    || ((flag.unwrap_or(f64::NAN) >= (97f64))
                        && (flag.unwrap_or(f64::NAN) <= (122f64))))
                {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                index
            }
        } else {
            index
        };
    }
}

pub fn ml_regular_expression_end_loop11(
    mut units: Vec<f64>,
    mut index: f64,
    mut escaped: bool,
    mut character_class: bool,
) -> crate::translation::frontend_rules::MlRegularExpressionEndLoop11Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if (((match (unit, 10f64) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } || match (unit, 13f64) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) || match (unit, 8232f64) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) || match (unit, 8233f64) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) {
                    crate::translation::frontend_rules::MlRegularExpressionEndLoop11Result::MlRegularExpressionEndLoop11Return(-1f64)
                } else {
                    {
                        let ml_s21 = if escaped {
                            {
                                let escaped_2 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index, escaped_2, character_class)
                            }
                        } else if match (unit, 92f64) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            {
                                let escaped_3 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index, escaped_3, character_class)
                            }
                        } else if match (unit, 91f64) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            {
                                let character_class_2 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index, escaped, character_class_2)
                            }
                        } else if match (unit, 93f64) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            {
                                let character_class_3 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index, escaped, character_class_3)
                            }
                        } else if (match (unit, 47f64) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } && !character_class)
                        {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    let index_3 = crate::translation::frontend_rules::ml_regular_expression_end_loop13(units.clone(), index_2);
                                    crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Return(index_3)
                                }
                            }
                        } else {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index, escaped, character_class)
                        };
                        match ml_s21.clone() {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Next(index_4, escaped_4, character_class_4) => {
                                {
                                    let index_5 = (index_4 + 1f64);
                                    {
                                        (index, escaped, character_class) = (index_5, escaped_4, character_class_4);
                                        continue;
                                    }
                                }
                            }
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin12::MlRegularExpressionEndJoin12Return(ml_result12) => {
                                crate::translation::frontend_rules::MlRegularExpressionEndLoop11Result::MlRegularExpressionEndLoop11Return(ml_result12)
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlRegularExpressionEndLoop11Result::MlRegularExpressionEndLoop11Done
        };
    }
}

pub fn ml_find_binding_run_end_loop14(mut terms: Vec<String>, mut end: f64) -> f64 {
    loop {
        return if (end < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::optional_at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if ((!(match (term.clone(), String::from("function_declaration")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) && !(match (term.clone(), String::from("lexical_declaration")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })) && !(match (term.clone(), String::from("export_statement")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })) {
                    end
                } else {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptBindingScopeLoop15Result {
    MlAcceptBindingScopeLoop15Done,
    MlAcceptBindingScopeLoop15Return(bool),
}

pub fn ml_accept_binding_scope_loop15(
    mut statuses: Vec<String>,
    mut reasons: Vec<String>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptBindingScopeLoop15Result {
    loop {
        return if (index < (statuses.len() as f64)) {
            if (match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &statuses,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                ),
                String::from("carried"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } && match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &reasons,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                ),
                String::from("type"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) {
                crate::translation::frontend_rules::MlAcceptBindingScopeLoop15Result::MlAcceptBindingScopeLoop15Return(true)
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
            crate::translation::frontend_rules::MlAcceptBindingScopeLoop15Result::MlAcceptBindingScopeLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptModuleBindingScopeLoop16Result {
    MlAcceptModuleBindingScopeLoop16Done(f64),
    MlAcceptModuleBindingScopeLoop16Return(bool),
}

pub fn ml_accept_module_binding_scope_loop16(
    mut terms: Vec<String>,
    mut declarations: f64,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop16Result {
    loop {
        return if (index < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::optional_at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if (match (term.clone(), String::new()) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
                if ((!(match (term.clone(), String::from("function_declaration")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) && !(match (term.clone(), String::from("lexical_declaration")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })) && !(match (term.clone(), String::from("export_statement")) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })) {
                    crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop16Result::MlAcceptModuleBindingScopeLoop16Return(false)
                } else {
                    {
                        let declarations_2 = (declarations + 1f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                (declarations, index) = (declarations_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop16Result::MlAcceptModuleBindingScopeLoop16Done(declarations)
        };
    }
}

pub fn ml_find_documentation_parameter_range_loop17(mut units: Vec<f64>, mut start: f64) -> f64 {
    loop {
        return if ((start < (units.len() as f64))
            && (match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(start),
                ),
                32f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } || match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(start),
                ),
                9f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

pub fn ml_find_documentation_parameter_range_loop19(
    mut units: Vec<f64>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (units.len() as f64)) {
            {
                let code = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if ((((((code.unwrap_or(f64::NAN) >= (65f64))
                    && (code.unwrap_or(f64::NAN) <= (90f64)))
                    || ((code.unwrap_or(f64::NAN) >= (97f64))
                        && (code.unwrap_or(f64::NAN) <= (122f64))))
                    || match (code, 95f64) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                    || match (code, 36f64) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                    || (((end > start) && (code.unwrap_or(f64::NAN) >= (48f64)))
                        && (code.unwrap_or(f64::NAN) <= (57f64))))
                {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
                end
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDocumentationParameterRangeJoin20 {
    MlFindDocumentationParameterRangeJoin20Next,
    MlFindDocumentationParameterRangeJoin20Return(Vec<f64>),
}

pub fn ml_find_documentation_parameter_range_loop21(mut units: Vec<f64>, mut after: f64) -> f64 {
    loop {
        return if ((after < (units.len() as f64))
            && (match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(after),
                ),
                32f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } || match (
                crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(after),
                ),
                9f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })) {
            {
                let after_2 = (after + 1f64);
                {
                    after = after_2;
                    continue;
                }
            }
        } else {
            after
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDocumentationParameterRangeJoin18 {
    MlFindDocumentationParameterRangeJoin18Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop22(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )
            .unwrap_or(f64::NAN)
                >= (48f64))
                && (crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )
                .unwrap_or(f64::NAN)
                    <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    95f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

pub fn ml_read_rust_number_bounds_loop25(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )
            .unwrap_or(f64::NAN)
                >= (97f64))
                && (crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )
                .unwrap_or(f64::NAN)
                    <= (122f64)))
                || ((crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )
                .unwrap_or(f64::NAN)
                    >= (48f64))
                    && (crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    )
                    .unwrap_or(f64::NAN)
                        <= (57f64)))))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRustNumberBoundsJoin24 {
    MlReadRustNumberBoundsJoin24Next(f64, bool),
}

pub fn ml_read_rust_number_bounds_loop26(units: &[f64], mut at: f64) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )
            .unwrap_or(f64::NAN)
                >= (48f64))
                && (crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )
                .unwrap_or(f64::NAN)
                    <= (57f64)))
                || match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    ),
                    95f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}
