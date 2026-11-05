// Generated from complete js/src/translation/intrinsic-initializer-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptInertBindingSequenceJoin3;
use super::MlTemporaryAcceptInertBindingSequenceLoop2Result;
use super::accept_inert_binding_sequence;
use super::ml;
use super::ml_array;
use super::read_inert_binding_children;
use super::read_inert_binding_value;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_inert_binding_sequence_loop2(
    terms: &[String],
    parents: &[f64],
    values: &[bool],
    element: &str,
    children: &[f64],
    mut value: bool,
    mut at: f64,
) -> self::MlTemporaryAcceptInertBindingSequenceLoop2Result {
    loop {
        return if at < ((children.len() as f64) - 1f64) {
            {
                let child = self::read_required_number_fact(children, at);
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let source_ml_source1_s26 = if value {
                            if !(self::read_inert_binding_value(
                                values,
                                (parents.len() as f64),
                                child,
                            )) || ((!(element).is_empty()) && (term != element))
                            {
                                self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Return(false)
                            } else {
                                self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Next
                            }
                        } else {
                            let ml_library_inequality_1 = (term != ",");
                            if ml_library_inequality_1 {
                                self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Return(false)
                            } else {
                                self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Next
                            }
                        };
                        match source_ml_source1_s26 {
                            self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Next => {
                                {
                                    let value_2 = !value;
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (value, at) = (value_2, at_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryAcceptInertBindingSequenceJoin3::MlTemporaryAcceptInertBindingSequenceJoin3Return(source_ml_temporary_result3) => {
                                self::MlTemporaryAcceptInertBindingSequenceLoop2Result::MlTemporaryAcceptInertBindingSequenceLoop2Return(source_ml_temporary_result3)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptInertBindingSequenceLoop2Result::MlTemporaryAcceptInertBindingSequenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop16Result {
    MlTemporaryReadInertModuleBindingLoop16Done(f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop16(
    terms: &[String],
    fields: &[String],
    mut key: f64,
    mut item: f64,
    mut colons: f64,
    source_ml_temporary_values7: &[f64],
    mut source_ml_temporary_index7: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop16Result {
    loop {
        return if source_ml_temporary_index7
            < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values7,
                    source_ml_temporary_index7.to_index(),
                );
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let part = self::read_required_text_fact(terms, child);
                        if ((field == "key") && (key < 0f64))
                            && (((part == "property_identifier") || (part == "string"))
                                || (part == "number"))
                        {
                            {
                                let key_2 = child;
                                {
                                    let source_ml_temporary_index7_2 = source_ml_temporary_index7
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (key, source_ml_temporary_index7) =
                                            (key_2, source_ml_temporary_index7_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (field == "value") && (item < 0f64) {
                                {
                                    let item_2 = child;
                                    {
                                        let source_ml_temporary_index7_3 =
                                            source_ml_temporary_index7
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (item, source_ml_temporary_index7) =
                                                (item_2, source_ml_temporary_index7_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if part == ":" {
                                    {
                                        let colons_2 = colons + 1f64;
                                        {
                                            let source_ml_temporary_index7_4 =
                                                source_ml_temporary_index7
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (colons, source_ml_temporary_index7) =
                                                    (colons_2, source_ml_temporary_index7_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let key_3 = -2f64;
                                        self::MlTemporaryReadInertModuleBindingLoop16Result::MlTemporaryReadInertModuleBindingLoop16Done(key_3, item, colons)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop16Result::MlTemporaryReadInertModuleBindingLoop16Done(key, item, colons)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop15(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    mut values: Vec<bool>,
    mut at: f64,
) -> Vec<bool> {
    loop {
        if at >= 0f64 {
            {
                let term = self::read_required_text_fact(terms, at);
                {
                    let children = self::read_inert_binding_children(terms, parents, at);
                    {
                        let admitted = false;
                        {
                            let admitted_8 = if ((((term == "string") || (term == "number"))
                                || (term == "true"))
                                || (term == "false"))
                                || (term == "null")
                            {
                                { true }
                            } else if term == "array" {
                                {
                                    self::accept_inert_binding_sequence(
                                        terms,
                                        parents,
                                        &(values),
                                        at,
                                        "[",
                                        "]",
                                        "",
                                    )
                                }
                            } else if term == "object" {
                                {
                                    self::accept_inert_binding_sequence(
                                        terms,
                                        parents,
                                        &(values),
                                        at,
                                        "{",
                                        "}",
                                        "pair",
                                    )
                                }
                            } else if term == "pair" {
                                {
                                    let key = -1f64;
                                    {
                                        let item = -1f64;
                                        {
                                            let colons = 0f64;
                                            {
                                                let source_ml_temporary_values7 = children;
                                                {
                                                    let source_ml_temporary_index7 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s27 = self::source_ml_temporary_read_inert_module_binding_loop16(terms, fields, key, item, colons, &(source_ml_temporary_values7), source_ml_temporary_index7);
                                                        match source_ml_source1_s27 {
                                                                        self::MlTemporaryReadInertModuleBindingLoop16Result::MlTemporaryReadInertModuleBindingLoop16Done(key_2, item_2, colons_2) => {
                                                                            {
                                                                                (((key_2 >= 0f64) && (item_2 >= 0f64)) && (colons_2 == 1f64)) && self::read_inert_binding_value(&(values), (parents.len() as f64), item_2)
                                                                            }
                                                                        }
                                                                    }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if (term == "unary_expression")
                                && ((children.len() as f64) == 2f64)
                            {
                                {
                                    let operator =
                                        self::read_required_number_fact(&(children), 0f64);
                                    {
                                        let operand =
                                            self::read_required_number_fact(&(children), 1f64);
                                        {
                                            let operation =
                                                self::read_required_text_fact(terms, operator);
                                            {
                                                ((operation == "-") || (operation == "+"))
                                                    && ((self::read_required_text_fact(
                                                        terms, operand,
                                                    )) == "number")
                                            }
                                        }
                                    }
                                }
                            } else if (term == "parenthesized_expression")
                                && ((children.len() as f64) == 3f64)
                            {
                                {
                                    (((self::read_required_text_fact(
                                        terms,
                                        self::read_required_number_fact(&(children), 0f64),
                                    )) == "(")
                                        && ((self::read_required_text_fact(
                                            terms,
                                            self::read_required_number_fact(&(children), 2f64),
                                        )) == ")"))
                                        && self::read_inert_binding_value(
                                            &(values),
                                            (parents.len() as f64),
                                            self::read_required_number_fact(&(children), 1f64),
                                        )
                                }
                            } else {
                                admitted
                            };
                            {
                                let values_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![admitted_8], values);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_2 = at - 1f64;
                                    {
                                        (values, at) = (values_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { values };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingJoin17 {
    MlTemporaryReadInertModuleBindingJoin17Next,
    MlTemporaryReadInertModuleBindingJoin17Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop19Result {
    MlTemporaryReadInertModuleBindingLoop19Done,
    MlTemporaryReadInertModuleBindingLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop19(
    terms: &[String],
    parents: &[f64],
    source_ml_temporary_values9: &[f64],
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop19Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                {
                    let part = self::read_required_text_fact(terms, child);
                    if ((part == "[") || (part == "]")) || (part == ",") {
                        {
                            let source_ml_temporary_index9_2 =
                                source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index9 = source_ml_temporary_index9_2;
                                continue;
                            }
                        }
                    }
                    {
                        if part == "array" {
                            {
                                let entries =
                                    self::read_inert_binding_children(terms, parents, child);
                                if ((entries.len() as f64) != 5f64)
                                    && ((entries.len() as f64) != 6f64)
                                {
                                    self::MlTemporaryReadInertModuleBindingLoop19Result::MlTemporaryReadInertModuleBindingLoop19Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let source_ml_temporary_index9_3 =
                                            source_ml_temporary_index9
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index9 =
                                                source_ml_temporary_index9_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadInertModuleBindingLoop19Result::MlTemporaryReadInertModuleBindingLoop19Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop19Result::MlTemporaryReadInertModuleBindingLoop19Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop18Result {
    MlTemporaryReadInertModuleBindingLoop18Done(f64, f64, f64),
    MlTemporaryReadInertModuleBindingLoop18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop18(
    terms: &[String],
    fields: &[String],
    mut constructor: f64,
    mut arguments_at: f64,
    mut news: f64,
    source_ml_temporary_values8: &[f64],
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop18Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let part = self::read_required_text_fact(terms, child);
                        if ((field == "constructor") && (part == "identifier"))
                            && (constructor < 0f64)
                        {
                            {
                                let constructor_2 = child;
                                {
                                    let source_ml_temporary_index8_2 = source_ml_temporary_index8
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (constructor, source_ml_temporary_index8) =
                                            (constructor_2, source_ml_temporary_index8_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "arguments") && (part == "arguments"))
                                && (arguments_at < 0f64)
                            {
                                {
                                    let arguments_at_2 = child;
                                    {
                                        let source_ml_temporary_index8_3 =
                                            source_ml_temporary_index8
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, source_ml_temporary_index8) =
                                                (arguments_at_2, source_ml_temporary_index8_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if part == "new" {
                                    {
                                        let news_2 = news + 1f64;
                                        {
                                            let source_ml_temporary_index8_4 =
                                                source_ml_temporary_index8
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (news, source_ml_temporary_index8) =
                                                    (news_2, source_ml_temporary_index8_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlTemporaryReadInertModuleBindingLoop18Result::MlTemporaryReadInertModuleBindingLoop18Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop18Result::MlTemporaryReadInertModuleBindingLoop18Done(constructor, arguments_at, news)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop20Result {
    MlTemporaryReadInertModuleBindingLoop20Done(f64, f64),
    MlTemporaryReadInertModuleBindingLoop20Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop20(
    terms: &[String],
    fields: &[String],
    mut callee: f64,
    mut arguments_at: f64,
    source_ml_temporary_values10: &[f64],
    mut source_ml_temporary_index10: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop20Result {
    loop {
        return if source_ml_temporary_index10
            < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values10,
                    source_ml_temporary_index10.to_index(),
                );
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let part = self::read_required_text_fact(terms, child);
                        if ((field == "function") && (part == "member_expression"))
                            && (callee < 0f64)
                        {
                            {
                                let callee_2 = child;
                                {
                                    let source_ml_temporary_index10_2 = source_ml_temporary_index10
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (callee, source_ml_temporary_index10) =
                                            (callee_2, source_ml_temporary_index10_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "arguments") && (part == "arguments"))
                                && (arguments_at < 0f64)
                            {
                                {
                                    let arguments_at_2 = child;
                                    {
                                        let source_ml_temporary_index10_3 =
                                            source_ml_temporary_index10
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, source_ml_temporary_index10) =
                                                (arguments_at_2, source_ml_temporary_index10_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                self::MlTemporaryReadInertModuleBindingLoop20Result::MlTemporaryReadInertModuleBindingLoop20Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop20Result::MlTemporaryReadInertModuleBindingLoop20Done(callee, arguments_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop14Result {
    MlTemporaryReadInertModuleBindingLoop14Done(f64, f64, f64),
    MlTemporaryReadInertModuleBindingLoop14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop14(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    source_ml_temporary_values6: &[f64],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop14Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let source_ml_temporary_index6_2 = source_ml_temporary_index6
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, source_ml_temporary_index6) =
                                            (name_2, source_ml_temporary_index6_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (field == "value") && (value < 0f64) {
                                {
                                    let value_2 = child;
                                    {
                                        let source_ml_temporary_index6_3 =
                                            source_ml_temporary_index6
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, source_ml_temporary_index6) =
                                                (value_2, source_ml_temporary_index6_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if term == "=" {
                                    {
                                        let equals_2 = equals + 1f64;
                                        {
                                            let source_ml_temporary_index6_4 =
                                                source_ml_temporary_index6
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, source_ml_temporary_index6) =
                                                    (equals_2, source_ml_temporary_index6_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlTemporaryReadInertModuleBindingLoop14Result::MlTemporaryReadInertModuleBindingLoop14Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop14Result::MlTemporaryReadInertModuleBindingLoop14Done(name, value, equals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop13Result {
    MlTemporaryReadInertModuleBindingLoop13Done(f64, f64),
    MlTemporaryReadInertModuleBindingLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop13(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    source_ml_temporary_values5: &[f64],
    mut source_ml_temporary_index5: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop13Result {
    loop {
        return if source_ml_temporary_index5
            < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values5,
                    source_ml_temporary_index5.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let source_ml_temporary_index5_2 =
                                    source_ml_temporary_index5.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, source_ml_temporary_index5) =
                                        (constants_2, source_ml_temporary_index5_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if (term == "variable_declarator") && (binding < 0f64) {
                            {
                                let binding_2 = child;
                                {
                                    let source_ml_temporary_index5_3 = source_ml_temporary_index5
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, source_ml_temporary_index5) =
                                            (binding_2, source_ml_temporary_index5_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let source_ml_temporary_index5_4 = source_ml_temporary_index5
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index5 = source_ml_temporary_index5_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlTemporaryReadInertModuleBindingLoop13Result::MlTemporaryReadInertModuleBindingLoop13Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop13Result::MlTemporaryReadInertModuleBindingLoop13Done(binding, constants)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingJoin12 {
    MlTemporaryReadInertModuleBindingJoin12Next(f64),
    MlTemporaryReadInertModuleBindingJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop21Result {
    MlTemporaryReadInertModuleBindingLoop21Done(f64, f64),
    MlTemporaryReadInertModuleBindingLoop21Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop21(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    source_ml_temporary_values4: &[f64],
    mut source_ml_temporary_index4: self::ml::Big,
) -> self::MlTemporaryReadInertModuleBindingLoop21Result {
    loop {
        return if source_ml_temporary_index4
            < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values4,
                    source_ml_temporary_index4.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let source_ml_temporary_index4_2 =
                                    source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, source_ml_temporary_index4) =
                                        (exports_2, source_ml_temporary_index4_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if (term == "lexical_declaration") && (selected < 0f64) {
                            {
                                let selected_2 = child;
                                {
                                    let source_ml_temporary_index4_3 = source_ml_temporary_index4
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, source_ml_temporary_index4) =
                                            (selected_2, source_ml_temporary_index4_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let source_ml_temporary_index4_4 = source_ml_temporary_index4
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index4 = source_ml_temporary_index4_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlTemporaryReadInertModuleBindingLoop21Result::MlTemporaryReadInertModuleBindingLoop21Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop21Result::MlTemporaryReadInertModuleBindingLoop21Done(selected, exports)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadInertModuleBindingLoop11Result {
    MlTemporaryReadInertModuleBindingLoop11Done,
    MlTemporaryReadInertModuleBindingLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_module_binding_loop11(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlTemporaryReadInertModuleBindingLoop11Result {
    loop {
        return if at < (terms.len() as f64) {
            {
                let parent = self::read_required_number_fact(parents, at);
                {
                    let start = self::read_required_number_fact(starts, at);
                    {
                        let end = self::read_required_number_fact(ends, at);
                        if !(((((start >= 0f64) && ((start % 1f64) == 0f64)) && (end >= start))
                            && ((end % 1f64) == 0f64))
                            && (end <= source_bytes))
                            || ((self::read_required_number_fact(errors, at)) != 0f64)
                        {
                            self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Return(Vec::<f64>::new())
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        } else if (!(((parent >= 0f64) && ((parent % 1f64) == 0f64))
                            && (parent < at))
                            || (start < (self::read_required_number_fact(starts, parent))))
                            || (end > (self::read_required_number_fact(ends, parent)))
                        {
                            self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Return(Vec::<f64>::new())
                        } else {
                            {
                                let at_3 = at + 1f64;
                                {
                                    at = at_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptInertModuleBindingScopeLoop23Result {
    MlTemporaryAcceptInertModuleBindingScopeLoop23Done,
    MlTemporaryAcceptInertModuleBindingScopeLoop23Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptInertModuleBindingScopeLoop24Result {
    MlTemporaryAcceptInertModuleBindingScopeLoop24Done,
    MlTemporaryAcceptInertModuleBindingScopeLoop24Return(bool),
}
