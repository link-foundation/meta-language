// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptInertBindingSequenceLoop2Result;
use super::accept_inert_binding_sequence;
use super::ml;
use super::ml_array;
use super::read_inert_binding_children;
use super::read_inert_binding_value;
use super::read_required_number_fact;
use super::read_required_text_fact;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertBindingSequenceJoin3 {
    MlAcceptInertBindingSequenceJoin3Next,
    MlAcceptInertBindingSequenceJoin3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inert_binding_sequence_loop2(
    terms: &[String],
    parents: &[f64],
    values: &[bool],
    element: &str,
    children: &[f64],
    mut value: bool,
    mut at: f64,
) -> self::MlAcceptInertBindingSequenceLoop2Result {
    loop {
        return if at < ((children.len() as f64) - 1f64) {
            {
                let child = self::read_required_number_fact(children, at);
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let ml_s3311 = if value {
                            if !(self::read_inert_binding_value(
                                values,
                                (parents.len() as f64),
                                child,
                            )) || ((!(element).is_empty()) && (term != element))
                            {
                                self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Return(false)
                            } else {
                                self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Next
                            }
                        } else {
                            let ml_library_inequality_1 = (term != ",");
                            if ml_library_inequality_1 {
                                self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Return(false)
                            } else {
                                self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Next
                            }
                        };
                        match ml_s3311 {
                            self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Next => {
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
                            self::MlAcceptInertBindingSequenceJoin3::MlAcceptInertBindingSequenceJoin3Return(ml_result3) => {
                                self::MlAcceptInertBindingSequenceLoop2Result::MlAcceptInertBindingSequenceLoop2Return(ml_result3)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInertBindingSequenceLoop2Result::MlAcceptInertBindingSequenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop16Result {
    MlReadInertModuleBindingLoop16Done(f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop16(
    terms: &[String],
    fields: &[String],
    mut key: f64,
    mut item: f64,
    mut colons: f64,
    ml_values7: &[f64],
    mut ml_index7: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop16Result {
    loop {
        return if ml_index7 < (self::ml::Big::from_u128(ml_values7.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values7, ml_index7.to_index());
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
                                    let ml_index7_2 = ml_index7.add(&self::ml::Big::from_i128(1));
                                    {
                                        (key, ml_index7) = (key_2, ml_index7_2);
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
                                        let ml_index7_3 =
                                            ml_index7.add(&self::ml::Big::from_i128(1));
                                        {
                                            (item, ml_index7) = (item_2, ml_index7_3);
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
                                            let ml_index7_4 =
                                                ml_index7.add(&self::ml::Big::from_i128(1));
                                            {
                                                (colons, ml_index7) = (colons_2, ml_index7_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let key_3 = -2f64;
                                        self::MlReadInertModuleBindingLoop16Result::MlReadInertModuleBindingLoop16Done(key_3, item, colons)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop16Result::MlReadInertModuleBindingLoop16Done(
                key, item, colons,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop15(
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
                                                let ml_values7 = children;
                                                {
                                                    let ml_index7 = self::ml::Big::from_i128(0);
                                                    {
                                                        let ml_s3322 = self::ml_read_inert_module_binding_loop16(terms, fields, key, item, colons, &(ml_values7), ml_index7);
                                                        match ml_s3322 {
                                                                        self::MlReadInertModuleBindingLoop16Result::MlReadInertModuleBindingLoop16Done(key_2, item_2, colons_2) => {
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
pub enum MlReadInertModuleBindingJoin17 {
    MlReadInertModuleBindingJoin17Next,
    MlReadInertModuleBindingJoin17Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop19Result {
    MlReadInertModuleBindingLoop19Done,
    MlReadInertModuleBindingLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop19(
    terms: &[String],
    parents: &[f64],
    ml_values9: &[f64],
    mut ml_index9: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop19Result {
    loop {
        return if ml_index9 < (self::ml::Big::from_u128(ml_values9.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values9, ml_index9.to_index());
                {
                    let part = self::read_required_text_fact(terms, child);
                    if ((part == "[") || (part == "]")) || (part == ",") {
                        {
                            let ml_index9_2 = ml_index9.add(&self::ml::Big::from_i128(1));
                            {
                                ml_index9 = ml_index9_2;
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
                                    self::MlReadInertModuleBindingLoop19Result::MlReadInertModuleBindingLoop19Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let ml_index9_3 =
                                            ml_index9.add(&self::ml::Big::from_i128(1));
                                        {
                                            ml_index9 = ml_index9_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlReadInertModuleBindingLoop19Result::MlReadInertModuleBindingLoop19Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop19Result::MlReadInertModuleBindingLoop19Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop18Result {
    MlReadInertModuleBindingLoop18Done(f64, f64, f64),
    MlReadInertModuleBindingLoop18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop18(
    terms: &[String],
    fields: &[String],
    mut constructor: f64,
    mut arguments_at: f64,
    mut news: f64,
    ml_values8: &[f64],
    mut ml_index8: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop18Result {
    loop {
        return if ml_index8 < (self::ml::Big::from_u128(ml_values8.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values8, ml_index8.to_index());
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
                                    let ml_index8_2 = ml_index8.add(&self::ml::Big::from_i128(1));
                                    {
                                        (constructor, ml_index8) = (constructor_2, ml_index8_2);
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
                                        let ml_index8_3 =
                                            ml_index8.add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, ml_index8) =
                                                (arguments_at_2, ml_index8_3);
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
                                            let ml_index8_4 =
                                                ml_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                (news, ml_index8) = (news_2, ml_index8_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadInertModuleBindingLoop18Result::MlReadInertModuleBindingLoop18Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop18Result::MlReadInertModuleBindingLoop18Done(
                constructor,
                arguments_at,
                news,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop20Result {
    MlReadInertModuleBindingLoop20Done(f64, f64),
    MlReadInertModuleBindingLoop20Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop20(
    terms: &[String],
    fields: &[String],
    mut callee: f64,
    mut arguments_at: f64,
    ml_values10: &[f64],
    mut ml_index10: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop20Result {
    loop {
        return if ml_index10 < (self::ml::Big::from_u128(ml_values10.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values10, ml_index10.to_index());
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
                                    let ml_index10_2 = ml_index10.add(&self::ml::Big::from_i128(1));
                                    {
                                        (callee, ml_index10) = (callee_2, ml_index10_2);
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
                                        let ml_index10_3 =
                                            ml_index10.add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, ml_index10) =
                                                (arguments_at_2, ml_index10_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Done(
                callee,
                arguments_at,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop14Result {
    MlReadInertModuleBindingLoop14Done(f64, f64, f64),
    MlReadInertModuleBindingLoop14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop14(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values6: &[f64],
    mut ml_index6: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop14Result {
    loop {
        return if ml_index6 < (self::ml::Big::from_u128(ml_values6.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values6, ml_index6.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index6_2 = ml_index6.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index6) = (name_2, ml_index6_2);
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
                                        let ml_index6_3 =
                                            ml_index6.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index6) = (value_2, ml_index6_3);
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
                                            let ml_index6_4 =
                                                ml_index6.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index6) = (equals_2, ml_index6_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Done(
                name, value, equals,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop13Result {
    MlReadInertModuleBindingLoop13Done(f64, f64),
    MlReadInertModuleBindingLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop13(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    ml_values5: &[f64],
    mut ml_index5: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop13Result {
    loop {
        return if ml_index5 < (self::ml::Big::from_u128(ml_values5.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values5, ml_index5.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let ml_index5_2 = ml_index5.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, ml_index5) = (constants_2, ml_index5_2);
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
                                    let ml_index5_3 = ml_index5.add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, ml_index5) = (binding_2, ml_index5_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index5_4 = ml_index5.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index5 = ml_index5_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Done(
                binding, constants,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingJoin12 {
    MlReadInertModuleBindingJoin12Next(f64),
    MlReadInertModuleBindingJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop21Result {
    MlReadInertModuleBindingLoop21Done(f64, f64),
    MlReadInertModuleBindingLoop21Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop21(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values4: &[f64],
    mut ml_index4: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop21Result {
    loop {
        return if ml_index4 < (self::ml::Big::from_u128(ml_values4.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values4, ml_index4.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index4_2 = ml_index4.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index4) = (exports_2, ml_index4_2);
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
                                    let ml_index4_3 = ml_index4.add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, ml_index4) = (selected_2, ml_index4_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index4_4 = ml_index4.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index4 = ml_index4_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Done(
                selected, exports,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop11Result {
    MlReadInertModuleBindingLoop11Done,
    MlReadInertModuleBindingLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop11(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadInertModuleBindingLoop11Result {
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
                            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
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
                            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
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
            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertModuleBindingScopeLoop23Result {
    MlAcceptInertModuleBindingScopeLoop23Done,
    MlAcceptInertModuleBindingScopeLoop23Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertModuleBindingScopeLoop24Result {
    MlAcceptInertModuleBindingScopeLoop24Done,
    MlAcceptInertModuleBindingScopeLoop24Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inert_module_binding_scope_loop24(
    mut owner: f64,
    ml_values22: &[String],
    mut ml_index22: self::ml::Big,
) -> self::MlAcceptInertModuleBindingScopeLoop24Result {
    loop {
        return if ml_index22 < (self::ml::Big::from_u128(ml_values22.len() as u128)) {
            {
                let binding = self::ml_array::at(ml_values22, ml_index22.to_index());
                if (((owner == 1f64) && (binding == "Map"))
                    || ((owner == 2f64) && (binding == "Set")))
                    || ((owner == 3f64) && (binding == "Object"))
                {
                    self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Return(false)
                } else {
                    {
                        let ml_index22_2 = ml_index22.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index22 = ml_index22_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Done
        };
    }
}
