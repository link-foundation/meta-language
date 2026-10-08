// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin4 {
    MlDecodeUnicodeEscapeJoin4Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop5Result {
    MlDecodeUnicodeEscapeLoop5Done(f64, f64),
}

pub fn ml_decode_unicode_escape_loop5(
    mut units: Vec<f64>,
    mut index: f64,
    mut low: f64,
    mut offset: f64,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result {
    loop {
        return if (offset < (6f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index + offset),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if (48f64..=57f64).contains(&unit) {
                            { (unit - 48f64) }
                        } else if (65f64..=70f64).contains(&unit) {
                            { (unit - 55f64) }
                        } else if (97f64..=102f64).contains(&unit) {
                            { (unit - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result::MlDecodeUnicodeEscapeLoop5Done(low, offset)
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
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop5Result::MlDecodeUnicodeEscapeLoop5Done(low, offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin3 {
    MlDecodeUnicodeEscapeJoin3Next(f64),
    MlDecodeUnicodeEscapeJoin3Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop2Result {
    MlDecodeUnicodeEscapeLoop2Done(f64, f64, f64),
}

pub fn ml_decode_unicode_escape_loop2(
    mut units: Vec<f64>,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
    mut braced: bool,
) -> crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result {
    loop {
        return if ((index < (units.len() as f64)) && (digits < (if braced { 6f64 } else { 4f64 })))
        {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                {
                    let digit = (-1f64);
                    {
                        let digit_5 = if (48f64..=57f64).contains(&unit) {
                            { (unit - 48f64) }
                        } else if (65f64..=70f64).contains(&unit) {
                            { (unit - 55f64) }
                        } else if (97f64..=102f64).contains(&unit) {
                            { (unit - 87f64) }
                        } else {
                            digit
                        };
                        if (digit_5 < (0f64)) {
                            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result::MlDecodeUnicodeEscapeLoop2Done(index, code, digits)
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
            crate::translation::frontend_rules::MlDecodeUnicodeEscapeLoop2Result::MlDecodeUnicodeEscapeLoop2Done(index, code, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin1 {
    MlDecodeUnicodeEscapeJoin1Next(f64, bool),
    MlDecodeUnicodeEscapeJoin1Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptArgumentCountLoop6Result {
    MlAcceptArgumentCountLoop6Done,
    MlAcceptArgumentCountLoop6Return(bool),
}

pub fn ml_accept_argument_count_loop6(
    mut defaults: Vec<bool>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result {
    loop {
        return if (index < (defaults.len() as f64)) {
            if (crate::translation::frontend_rules::ml_array::at(
                &defaults,
                crate::translation::frontend_rules::ml_array::number_index(index),
            )) {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Return(false)
        } else {
            crate::translation::frontend_rules::MlAcceptArgumentCountLoop6Result::MlAcceptArgumentCountLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop7Result {
    MlFindDefaultParameterReferenceLoop7Done,
    MlFindDefaultParameterReferenceLoop7Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceJoin8 {
    MlFindDefaultParameterReferenceJoin8Next,
    MlFindDefaultParameterReferenceJoin8Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindDefaultParameterReferenceLoop9Result {
    MlFindDefaultParameterReferenceLoop9Done,
    MlFindDefaultParameterReferenceLoop9Return(f64),
}

pub fn ml_find_default_parameter_reference_loop9(
    mut parameters: Vec<String>,
    mut at: f64,
    mut value: String,
    mut parameter_index: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result {
    loop {
        return if (parameter_index < (parameters.len() as f64)) {
            if ((crate::translation::frontend_rules::ml_array::at(
                &parameters,
                crate::translation::frontend_rules::ml_array::number_index(parameter_index),
            )) == value)
            {
                crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Return(at)
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
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Done
        };
    }
}

pub fn ml_find_default_parameter_reference_loop7(
    mut kinds: Vec<String>,
    mut values: Vec<String>,
    mut parameters: Vec<String>,
    mut at: f64,
    mut depth: f64,
) -> crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result {
    loop {
        return if (((at < (values.len() as f64))
            && ((crate::translation::frontend_rules::ml_array::at(
                &kinds,
                crate::translation::frontend_rules::ml_array::number_index(at),
            )) != "eof"))
            && ((depth > (0f64))
                || (((crate::translation::frontend_rules::ml_array::at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                )) != ",")
                    && ((crate::translation::frontend_rules::ml_array::at(
                        &values,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    )) != ")"))))
        {
            {
                let value = crate::translation::frontend_rules::ml_array::at(
                    &values,
                    crate::translation::frontend_rules::ml_array::number_index(at),
                );
                {
                    let depth_3 = if (((value == "(") || (value == "[")) || (value == "{")) {
                        { (depth + 1f64) }
                    } else {
                        depth
                    };
                    {
                        let depth_5 = if (((value == ")") || (value == "]")) || (value == "}")) {
                            { (depth_3 - 1f64) }
                        } else {
                            depth_3
                        };
                        {
                            let ml_s15 = if (((crate::translation::frontend_rules::ml_array::at(
                                &kinds,
                                crate::translation::frontend_rules::ml_array::number_index(at),
                            )) == "identifier")
                                && !((at > (0f64))
                                    && ((crate::translation::frontend_rules::ml_array::at(
                                        &values,
                                        crate::translation::frontend_rules::ml_array::number_index(
                                            (at - 1f64),
                                        ),
                                    )) == ".")))
                            {
                                {
                                    let parameter_index = 0f64;
                                    {
                                        let ml_s16 = crate::translation::frontend_rules::ml_find_default_parameter_reference_loop9(parameters.clone(), at, value.clone(), parameter_index);
                                        match ml_s16.clone() {
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Done => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next
                                            }
                                            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop9Result::MlFindDefaultParameterReferenceLoop9Return(ml_result9) => {
                                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Return(ml_result9)
                                            }
                                        }
                                    }
                                }
                            } else {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next
                            };
                            match ml_s15.clone() {
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Next => {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            (at, depth) = (at_2, depth_5);
                                            continue;
                                        }
                                    }
                                }
                                crate::translation::frontend_rules::MlFindDefaultParameterReferenceJoin8::MlFindDefaultParameterReferenceJoin8Return(ml_result8) => {
                                    crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Return(ml_result8)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlFindDefaultParameterReferenceLoop7Result::MlFindDefaultParameterReferenceLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndLoop10Result {
    MlRegularExpressionEndLoop10Done,
    MlRegularExpressionEndLoop10Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRegularExpressionEndJoin11 {
    MlRegularExpressionEndJoin11Next(f64, bool, bool),
    MlRegularExpressionEndJoin11Return(f64),
}

pub fn ml_regular_expression_end_loop12(mut units: Vec<f64>, mut index: f64) -> f64 {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let flag = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ((65f64..=90f64).contains(&flag) || (97f64..=122f64).contains(&flag)) {
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

pub fn ml_regular_expression_end_loop10(
    mut units: Vec<f64>,
    mut index: f64,
    mut escaped: bool,
    mut character_class: bool,
) -> crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ((((unit == (10f64)) || (unit == (13f64))) || (unit == (8232f64)))
                    || (unit == (8233f64)))
                {
                    crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Return(-1f64)
                } else {
                    {
                        let ml_s17 = if escaped {
                            {
                                let escaped_2 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped_2, character_class)
                            }
                        } else if (unit == (92f64)) {
                            {
                                let escaped_3 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped_3, character_class)
                            }
                        } else if (unit == (91f64)) {
                            {
                                let character_class_2 = true;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class_2)
                            }
                        } else if (unit == (93f64)) {
                            {
                                let character_class_3 = false;
                                crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class_3)
                            }
                        } else if ((unit == (47f64)) && !character_class) {
                            {
                                let index_2 = (index + 1f64);
                                {
                                    let index_3 = crate::translation::frontend_rules::ml_regular_expression_end_loop12(units.clone(), index_2);
                                    crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Return(index_3)
                                }
                            }
                        } else {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index, escaped, character_class)
                        };
                        match ml_s17.clone() {
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Next(index_4, escaped_4, character_class_4) => {
                                {
                                    let index_5 = (index_4 + 1f64);
                                    {
                                        (index, escaped, character_class) = (index_5, escaped_4, character_class_4);
                                        continue;
                                    }
                                }
                            }
                            crate::translation::frontend_rules::MlRegularExpressionEndJoin11::MlRegularExpressionEndJoin11Return(ml_result11) => {
                                crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Return(ml_result11)
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlRegularExpressionEndLoop10Result::MlRegularExpressionEndLoop10Done
        };
    }
}

pub fn ml_find_binding_run_end_loop13(mut terms: Vec<String>, mut end: f64) -> f64 {
    loop {
        return if (end < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if (((term != "function_declaration") && (term != "lexical_declaration"))
                    && (term != "export_statement"))
                {
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
pub enum MlAcceptBindingScopeLoop14Result {
    MlAcceptBindingScopeLoop14Done,
    MlAcceptBindingScopeLoop14Return(bool),
}

pub fn ml_accept_binding_scope_loop14(
    mut statuses: Vec<String>,
    mut reasons: Vec<String>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result {
    loop {
        return if (index < (statuses.len() as f64)) {
            if (((crate::translation::frontend_rules::ml_array::at(
                &statuses,
                crate::translation::frontend_rules::ml_array::number_index(index),
            )) == "carried")
                && ((crate::translation::frontend_rules::ml_array::at(
                    &reasons,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                )) == "type"))
            {
                crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Return(true)
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
            crate::translation::frontend_rules::MlAcceptBindingScopeLoop14Result::MlAcceptBindingScopeLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptModuleBindingScopeLoop15Result {
    MlAcceptModuleBindingScopeLoop15Done(f64),
    MlAcceptModuleBindingScopeLoop15Return(bool),
}

pub fn ml_accept_module_binding_scope_loop15(
    mut terms: Vec<String>,
    mut declarations: f64,
    mut index: f64,
) -> crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result {
    loop {
        return if (index < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if term.is_empty() {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
                if (((term != "function_declaration") && (term != "lexical_declaration"))
                    && (term != "export_statement"))
                {
                    crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Return(false)
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
            crate::translation::frontend_rules::MlAcceptModuleBindingScopeLoop15Result::MlAcceptModuleBindingScopeLoop15Done(declarations)
        };
    }
}

pub fn ml_find_documentation_parameter_range_loop16(mut units: Vec<f64>, mut start: f64) -> f64 {
    loop {
        return if ((start < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(start),
            )) == (32f64))
                || ((crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(start),
                )) == (9f64))))
        {
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

pub fn ml_find_documentation_parameter_range_loop18(
    mut units: Vec<f64>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (units.len() as f64)) {
            {
                let code = crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if (((((65f64..=90f64).contains(&code) || (97f64..=122f64).contains(&code))
                    || (code == (95f64)))
                    || (code == (36f64)))
                    || (((end > start) && (code >= (48f64))) && (code <= (57f64))))
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
pub enum MlFindDocumentationParameterRangeJoin19 {
    MlFindDocumentationParameterRangeJoin19Next,
    MlFindDocumentationParameterRangeJoin19Return(Vec<f64>),
}

pub fn ml_find_documentation_parameter_range_loop20(mut units: Vec<f64>, mut after: f64) -> f64 {
    loop {
        return if ((after < (units.len() as f64))
            && (((crate::translation::frontend_rules::ml_array::at(
                &units,
                crate::translation::frontend_rules::ml_array::number_index(after),
            )) == (32f64))
                || ((crate::translation::frontend_rules::ml_array::at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(after),
                )) == (9f64))))
        {
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
pub enum MlFindDocumentationParameterRangeJoin17 {
    MlFindDocumentationParameterRangeJoin17Next(f64, bool),
}

const fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(ml_main)
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
