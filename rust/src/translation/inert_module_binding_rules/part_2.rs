// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptInertModuleBindingScopeLoop23Result;
use super::MlReadInertModuleBindingJoin12;
use super::MlReadInertModuleBindingLoop11Result;
use super::MlReadInertModuleBindingLoop13Result;
use super::MlReadInertModuleBindingLoop14Result;
use super::MlReadInertModuleBindingLoop21Result;
use super::accept_inert_binding_sequence;
use super::ml;
use super::ml_accept_inert_module_binding_scope_loop23;
use super::ml_array;
use super::ml_read_inert_module_binding_loop11;
use super::ml_read_inert_module_binding_loop13;
use super::ml_read_inert_module_binding_loop14;
use super::ml_read_inert_module_binding_loop20;
use super::ml_read_inert_module_binding_loop21;
use super::read_inert_binding_argument;
use super::read_inert_binding_children;
use super::read_inert_binding_value;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_inert_module_binding(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    texts: &[String],
    errors: &[f64],
    source_bytes: f64,
) -> Vec<f64> {
    if (((((((((terms.len() as f64) < 1f64) || ((terms.len() as f64) > 4096f64))
        || ((fields.len() as f64) != (terms.len() as f64)))
        || ((parents.len() as f64) != (terms.len() as f64)))
        || ((starts.len() as f64) != (terms.len() as f64)))
        || ((ends.len() as f64) != (terms.len() as f64)))
        || ((texts.len() as f64) != (terms.len() as f64)))
        || ((errors.len() as f64) != (terms.len() as f64)))
        || !(((0f64..=65_536f64).contains(&source_bytes)) && ((source_bytes % 1f64) == 0f64))
    {
        Vec::<f64>::new()
    } else {
        {
            let at = 0f64;
            {
                let ml_s2 = self::ml_read_inert_module_binding_loop11(
                    terms,
                    parents,
                    starts,
                    ends,
                    errors,
                    source_bytes,
                    at,
                );
                match ml_s2 {
                    self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Done => {
                        {
                            let top = self::read_inert_binding_children(terms, parents, 0f64);
                            {
let ml_library_inequality_3 = (top.len() as f64) != 1f64 ;
if ml_library_inequality_3 {
                                Vec::<f64>::new()
                            } else {
                                {
                                    let declaration = self::read_required_number_fact(&(top), 0f64);
                                    {
                                        let ml_s4 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                            {
                                                let children = self::read_inert_binding_children(terms, parents, declaration);
                                                {
                                                    let selected = -1f64;
                                                    {
                                                        let exports = 0f64;
                                                        {
                                                            let ml_values4 = children;
                                                            {
                                                                let ml_index4 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let ml_s5 = self::ml_read_inert_module_binding_loop21(terms, selected, exports, &(ml_values4), ml_index4);
                                                                    match ml_s5 {
                                                                        self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Done(selected_2, exports_2) => {
                                                                            if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                                self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Return(Vec::<f64>::new())
                                                                            } else {
                                                                                {
                                                                                    let declaration_2 = selected_2;
                                                                                    self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Next(declaration_2)
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Return(ml_result21) => {
                                                                            self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Return(ml_result21)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Next(declaration)
                                        };
                                        match ml_s4 {
                                            self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Next(declaration_3) => {
                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(terms, declaration_3)) != "lexical_declaration" ;
if ml_library_inequality_2 {
                                                    Vec::<f64>::new()
                                                } else {
                                                    {
                                                        let binding = -1f64;
                                                        {
                                                            let constants = 0f64;
                                                            {
                                                                let ml_values5 = self::read_inert_binding_children(terms, parents, declaration_3);
                                                                {
                                                                    let ml_index5 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let ml_s6 = self::ml_read_inert_module_binding_loop13(terms, binding, constants, &(ml_values5), ml_index5);
                                                                        match ml_s6 {
                                                                            self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Done(binding_2, constants_2) => {
                                                                                if (constants_2 != 1f64) || (binding_2 < 0f64) {
                                                                                    Vec::<f64>::new()
                                                                                } else {
                                                                                    {
                                                                                        let name = -1f64;
                                                                                        {
                                                                                            let value = -1f64;
                                                                                            {
                                                                                                let equals = 0f64;
                                                                                                {
                                                                                                    let ml_values6 = self::read_inert_binding_children(terms, parents, binding_2);
                                                                                                    {
                                                                                                        let ml_index6 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let ml_s7 = self::ml_read_inert_module_binding_loop14(terms, fields, name, value, equals, &(ml_values6), ml_index6);
                                                                                                            match ml_s7 {
                                                                                                                self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Done(name_2, value_2, equals_2) => {
                                                                                                                    if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains("\\") {
                                                                                                                        Vec::<f64>::new()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let values = Vec::<bool>::new();
                                                                                                                            {
                                                                                                                                let at_2 = (terms.len() as f64) - 1f64;
                                                                                                                                {
                                                                                                                                    let values_2 = self::ml_read_inert_module_binding_loop15(terms, fields, parents, values, at_2);
                                                                                                                                    if self::read_inert_binding_value(&(values_2), (parents.len() as f64), value_2) {
                                                                                                                                        vec![name_2, 0f64]
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let term = self::read_required_text_fact(terms, value_2);
                                                                                                                                            {
                                                                                                                                                let children_2 = self::read_inert_binding_children(terms, parents, value_2);
                                                                                                                                                {
                                                                                                                                                    let ml_s11 = if term == "new_expression" {
                                                                                                                                                        {
                                                                                                                                                            let constructor = -1f64;
                                                                                                                                                            {
                                                                                                                                                                let arguments_at = -1f64;
                                                                                                                                                                {
                                                                                                                                                                    let news = 0f64;
                                                                                                                                                                    {
                                                                                                                                                                        let ml_values8 = children_2;
                                                                                                                                                                        {
                                                                                                                                                                            let ml_index8 = self::ml::Big::from_i128(0);
                                                                                                                                                                            {
                                                                                                                                                                                let ml_s12 = self::ml_read_inert_module_binding_loop18(terms, fields, constructor, arguments_at, news, &(ml_values8), ml_index8);
                                                                                                                                                                                match ml_s12 {
                                                                                                                                                                                    self::MlReadInertModuleBindingLoop18Result::MlReadInertModuleBindingLoop18Done(constructor_2, arguments_at_2, news_2) => {
                                                                                                                                                                                        if ((news_2 != 1f64) || (constructor_2 < 0f64)) || (arguments_at_2 < 0f64) {
                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                        } else {
                                                                                                                                                                                            {
                                                                                                                                                                                                let owner = self::read_required_text_fact(texts, constructor_2);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let argument = self::read_inert_binding_argument(terms, parents, &(values_2), arguments_at_2);
                                                                                                                                                                                                    if (argument < 0f64) || ((self::read_required_text_fact(terms, argument)) != "array") {
                                                                                                                                                                                                        self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                    } else if owner == "Set" {
                                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(vec![name_2, 2f64])
                                                                                                                                                                                                        } else if owner == "Map" {
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let ml_values9 = self::read_inert_binding_children(terms, parents, argument);
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let ml_index9 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let ml_s13 = self::ml_read_inert_module_binding_loop19(terms, parents, &(ml_values9), ml_index9);
                                                                                                                                                                                                                            match ml_s13 {
                                                                                                                                                                                                                                self::MlReadInertModuleBindingLoop19Result::MlReadInertModuleBindingLoop19Done => {
                                                                                                                                                                                                                                    self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(vec![name_2, 1f64])
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                self::MlReadInertModuleBindingLoop19Result::MlReadInertModuleBindingLoop19Return(ml_result19) => {
                                                                                                                                                                                                                                    self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(ml_result19)
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Next
                                                                                                                                                                                                            }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                    self::MlReadInertModuleBindingLoop18Result::MlReadInertModuleBindingLoop18Return(ml_result19_2) => {
                                                                                                                                                                                        self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(ml_result19_2)
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    } else if term == "call_expression" {
                                                                                                                                                            {
                                                                                                                                                                let callee = -1f64;
                                                                                                                                                                {
                                                                                                                                                                    let arguments_at_3 = -1f64;
                                                                                                                                                                    {
                                                                                                                                                                        let ml_values10 = children_2;
                                                                                                                                                                        {
                                                                                                                                                                            let ml_index10 = self::ml::Big::from_i128(0);
                                                                                                                                                                            {
                                                                                                                                                                                let ml_s14 = self::ml_read_inert_module_binding_loop20(terms, fields, callee, arguments_at_3, &(ml_values10), ml_index10);
                                                                                                                                                                                match ml_s14 {
                                                                                                                                                                                    self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Done(callee_2, arguments_at_4) => {
                                                                                                                                                                                        if (callee_2 < 0f64) || (arguments_at_4 < 0f64) {
                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                        } else {
                                                                                                                                                                                            {
                                                                                                                                                                                                let member = self::read_inert_binding_children(terms, parents, callee_2);
                                                                                                                                                                                                {
let ml_library_inequality_1 = (member.len() as f64) != 3f64 ;
if ml_library_inequality_1 {
                                                                                                                                                                                                    self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let object = self::read_required_number_fact(&(member), 0f64);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let dot = self::read_required_number_fact(&(member), 1f64);
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let property = self::read_required_number_fact(&(member), 2f64);
                                                                                                                                                                                                                if (((((self::read_required_text_fact(terms, object)) != "identifier") || ((self::read_required_text_fact(texts, object)) != "Object")) || ((self::read_required_text_fact(terms, dot)) != ".")) || ((self::read_required_text_fact(terms, property)) != "property_identifier")) || ((self::read_required_text_fact(texts, property)) != "freeze") {
                                                                                                                                                                                                                    self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let argument_2 = self::read_inert_binding_argument(terms, parents, &(values_2), arguments_at_4);
                                                                                                                                                                                                                        if (argument_2 >= 0f64) && (((self::read_required_text_fact(terms, argument_2)) == "array") || ((self::read_required_text_fact(terms, argument_2)) == "object")) {
                                                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(vec![name_2, 3f64])
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Next
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
}
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                    self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Return(ml_result20) => {
                                                                                                                                                                                        self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(ml_result20)
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Next
                                                                                                                                                        };
                                                                                                                                                    match ml_s11 {
                                                                                                                                                        self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Next => {
                                                                                                                                                            Vec::<f64>::new()
                                                                                                                                                        }
                                                                                                                                                        self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(ml_result17) => {
                                                                                                                                                            ml_result17
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Return(ml_result20_2) => {
                                                                                                                    ml_result20_2
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Return(ml_result20_3) => {
                                                                                ml_result20_3
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
}
                                            }
                                            self::MlReadInertModuleBindingJoin12::MlReadInertModuleBindingJoin12Return(ml_result20_4) => {
                                                ml_result20_4
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                    }
                    self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(ml_result21_2) => {
                        ml_result21_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_inert_module_binding_scope(
    forms: &[String],
    owners: &[f64],
    bindings: &[String],
) -> bool {
    {
        let ml_library_inequality_1 = (forms.len() as f64) != (owners.len() as f64);
        if ml_library_inequality_1 {
            false
        } else {
            {
                let at = 0f64;
                {
                    let ml_s15 = self::ml_accept_inert_module_binding_scope_loop23(
                        forms, owners, bindings, at,
                    );
                    match ml_s15 {
                    self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Done => {
                        true
                    }
                    self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(ml_result23) => {
                        ml_result23
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_binding_children_loop1(
    terms: &[String],
    parents: &[f64],
    mut owner: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (terms.len() as f64) {
            if (((self::read_required_number_fact(parents, at)) == owner)
                && ((self::read_required_text_fact(terms, at)) != "comment"))
                && ((self::read_required_text_fact(terms, at)) != "whitespace")
            {
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) = (vec![at], result);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertBindingSequenceLoop2Result {
    MlAcceptInertBindingSequenceLoop2Done,
    MlAcceptInertBindingSequenceLoop2Return(bool),
}

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
                        let ml_s16 = if value {
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
                        match ml_s16 {
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
                                                        let ml_s17 = self::ml_read_inert_module_binding_loop16(terms, fields, key, item, colons, &(ml_values7), ml_index7);
                                                        match ml_s17 {
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
