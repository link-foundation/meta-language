// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptInertModuleBindingScopeLoop23Result;
use super::MlAcceptNumericModuleBindingNumberLiteralLoop39Result;
use super::MlReadEmptyStandardCollectionModuleBindingJoin31;
use super::MlReadEmptyStandardCollectionModuleBindingJoin35;
use super::MlReadEmptyStandardCollectionModuleBindingLoop30Result;
use super::MlReadEmptyStandardCollectionModuleBindingLoop32Result;
use super::MlReadEmptyStandardCollectionModuleBindingLoop33Result;
use super::MlReadEmptyStandardCollectionModuleBindingLoop34Result;
use super::MlReadEmptyStandardCollectionModuleBindingLoop36Result;
use super::MlReadInertModuleBindingJoin12;
use super::MlReadInertModuleBindingJoin17;
use super::MlReadInertModuleBindingLoop11Result;
use super::MlReadInertModuleBindingLoop13Result;
use super::MlReadInertModuleBindingLoop14Result;
use super::MlReadInertModuleBindingLoop18Result;
use super::MlReadInertModuleBindingLoop19Result;
use super::MlReadInertModuleBindingLoop20Result;
use super::MlReadInertModuleBindingLoop21Result;
use super::MlReadNumericModuleBindingJoin44;
use super::MlReadNumericModuleBindingLoop43Result;
use super::MlReadNumericModuleBindingLoop45Result;
use super::MlReadNumericModuleBindingLoop46Result;
use super::MlReadNumericModuleBindingLoop48Result;
use super::ml;
use super::ml_accept_inert_module_binding_scope_loop23;
use super::ml_accept_numeric_module_binding_number_literal_loop39;
use super::ml_read_empty_collection_binding_children_loop25;
use super::ml_read_empty_standard_collection_module_binding_loop30;
use super::ml_read_empty_standard_collection_module_binding_loop32;
use super::ml_read_empty_standard_collection_module_binding_loop33;
use super::ml_read_empty_standard_collection_module_binding_loop34;
use super::ml_read_empty_standard_collection_module_binding_loop36;
use super::ml_read_inert_module_binding_loop11;
use super::ml_read_inert_module_binding_loop13;
use super::ml_read_inert_module_binding_loop14;
use super::ml_read_inert_module_binding_loop15;
use super::ml_read_inert_module_binding_loop18;
use super::ml_read_inert_module_binding_loop19;
use super::ml_read_inert_module_binding_loop20;
use super::ml_read_inert_module_binding_loop21;
use super::ml_read_numeric_module_binding_children_loop37;
use super::ml_read_numeric_module_binding_loop43;
use super::ml_read_numeric_module_binding_loop45;
use super::ml_read_numeric_module_binding_loop46;
use super::ml_read_numeric_module_binding_loop47;
use super::ml_read_numeric_module_binding_loop48;
use super::read_inert_binding_argument;
use super::read_inert_binding_children;
use super::read_inert_binding_value;
use super::read_required_boolean_fact;
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
                let ml_s22 = self::ml_read_inert_module_binding_loop11(
                    terms,
                    parents,
                    starts,
                    ends,
                    errors,
                    source_bytes,
                    at,
                );
                match ml_s22 {
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
                                        let ml_s44 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
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
                                                                    let ml_s55 = self::ml_read_inert_module_binding_loop21(terms, selected, exports, &(ml_values4), ml_index4);
                                                                    match ml_s55 {
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
                                        match ml_s44 {
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
                                                                        let ml_s66 = self::ml_read_inert_module_binding_loop13(terms, binding, constants, &(ml_values5), ml_index5);
                                                                        match ml_s66 {
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
                                                                                                            let ml_s77 = self::ml_read_inert_module_binding_loop14(terms, fields, name, value, equals, &(ml_values6), ml_index6);
                                                                                                            match ml_s77 {
                                                                                                                self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Done(name_2, value_2, equals_2) => {
                                                                                                                    if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains('\\') {
                                                                                                                        Vec::<f64>::new()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let values = Vec::<bool>::new();
                                                                                                                            {
                                                                                                                                let at_2 = (terms.len() as f64) - 1f64;
                                                                                                                                {
                                                                                                                                    let accepted_literal_values = self::ml_read_inert_module_binding_loop15(terms, fields, parents, values, at_2);
                                                                                                                                    if self::read_inert_binding_value(&(accepted_literal_values), (parents.len() as f64), value_2) {
                                                                                                                                        vec![name_2, 0f64]
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let term = self::read_required_text_fact(terms, value_2);
                                                                                                                                            {
                                                                                                                                                let children_2 = self::read_inert_binding_children(terms, parents, value_2);
                                                                                                                                                {
                                                                                                                                                    let ml_s1111 = if term == "new_expression" {
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
                                                                                                                                                                                let ml_s1122 = self::ml_read_inert_module_binding_loop18(terms, fields, constructor, arguments_at, news, &(ml_values8), ml_index8);
                                                                                                                                                                                match ml_s1122 {
                                                                                                                                                                                    self::MlReadInertModuleBindingLoop18Result::MlReadInertModuleBindingLoop18Done(constructor_2, arguments_at_2, news_2) => {
                                                                                                                                                                                        if ((news_2 != 1f64) || (constructor_2 < 0f64)) || (arguments_at_2 < 0f64) {
                                                                                                                                                                                            self::MlReadInertModuleBindingJoin17::MlReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                        } else {
                                                                                                                                                                                            {
                                                                                                                                                                                                let owner = self::read_required_text_fact(texts, constructor_2);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let argument = self::read_inert_binding_argument(terms, parents, &(accepted_literal_values), arguments_at_2);
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
                                                                                                                                                                                                                            let ml_s1133 = self::ml_read_inert_module_binding_loop19(terms, parents, &(ml_values9), ml_index9);
                                                                                                                                                                                                                            match ml_s1133 {
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
                                                                                                                                                                                let ml_s1144 = self::ml_read_inert_module_binding_loop20(terms, fields, callee, arguments_at_3, &(ml_values10), ml_index10);
                                                                                                                                                                                match ml_s1144 {
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
                                                                                                                                                                                                                        let argument_2 = self::read_inert_binding_argument(terms, parents, &(accepted_literal_values), arguments_at_4);
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
                                                                                                                                                    match ml_s1111 {
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
                    let ml_s1155 = self::ml_accept_inert_module_binding_scope_loop23(
                        forms, owners, bindings, at,
                    );
                    match ml_s1155 {
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
pub fn read_empty_collection_binding_children(
    terms: &[String],
    parents: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::ml_read_empty_collection_binding_children_loop25(
                    terms, parents, owner, result, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_empty_standard_collection_module_binding(
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
                let ml_s1166 = self::ml_read_empty_standard_collection_module_binding_loop30(
                    terms,
                    parents,
                    starts,
                    ends,
                    errors,
                    source_bytes,
                    at,
                );
                match ml_s1166 {
                    self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Done => {
                        {
                            let top = self::read_empty_collection_binding_children(terms, parents, 0f64);
                            {
let ml_library_inequality_2 = (top.len() as f64) != 1f64 ;
if ml_library_inequality_2 {
                                Vec::<f64>::new()
                            } else {
                                {
                                    let declaration = self::read_required_number_fact(&(top), 0f64);
                                    {
                                        let ml_s1188 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                            {
                                                let selected = -1f64;
                                                {
                                                    let exports = 0f64;
                                                    {
                                                        let ml_values26 = self::read_empty_collection_binding_children(terms, parents, declaration);
                                                        {
                                                            let ml_index26 = self::ml::Big::from_i128(0);
                                                            {
                                                                let ml_s1199 = self::ml_read_empty_standard_collection_module_binding_loop36(terms, selected, exports, &(ml_values26), ml_index26);
                                                                match ml_s1199 {
                                                                    self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Done(selected_2, exports_2) => {
                                                                        if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                            self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Return(Vec::<f64>::new())
                                                                        } else {
                                                                            {
                                                                                let declaration_2 = selected_2;
                                                                                self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Next(declaration_2)
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Return(ml_result36) => {
                                                                        self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Return(ml_result36)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Next(declaration)
                                        };
                                        match ml_s1188 {
                                            self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Next(declaration_3) => {
                                                {
let ml_library_inequality_1 = (self::read_required_text_fact(terms, declaration_3)) != "lexical_declaration" ;
if ml_library_inequality_1 {
                                                    Vec::<f64>::new()
                                                } else {
                                                    {
                                                        let binding = -1f64;
                                                        {
                                                            let constants = 0f64;
                                                            {
                                                                let ml_values27 = self::read_empty_collection_binding_children(terms, parents, declaration_3);
                                                                {
                                                                    let ml_index27 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let ml_s2200 = self::ml_read_empty_standard_collection_module_binding_loop32(terms, binding, constants, &(ml_values27), ml_index27);
                                                                        match ml_s2200 {
                                                                            self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Done(binding_2, constants_2) => {
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
                                                                                                    let ml_values28 = self::read_empty_collection_binding_children(terms, parents, binding_2);
                                                                                                    {
                                                                                                        let ml_index28 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let ml_s2211 = self::ml_read_empty_standard_collection_module_binding_loop33(terms, fields, name, value, equals, &(ml_values28), ml_index28);
                                                                                                            match ml_s2211 {
                                                                                                                self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Done(name_2, value_2, equals_2) => {
                                                                                                                    if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains('\\') {
                                                                                                                        Vec::<f64>::new()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let constructor = -1f64;
                                                                                                                            {
                                                                                                                                let arguments_at = -1f64;
                                                                                                                                {
                                                                                                                                    let news = 0f64;
                                                                                                                                    {
                                                                                                                                        let ml_values29 = self::read_empty_collection_binding_children(terms, parents, value_2);
                                                                                                                                        {
                                                                                                                                            let ml_index29 = self::ml::Big::from_i128(0);
                                                                                                                                            {
                                                                                                                                                let ml_s2222 = self::ml_read_empty_standard_collection_module_binding_loop34(terms, fields, constructor, arguments_at, news, &(ml_values29), ml_index29);
                                                                                                                                                match ml_s2222 {
                                                                                                                                                    self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Done(constructor_2, arguments_at_2, news_2) => {
                                                                                                                                                        if (news_2 != 1f64) || (constructor_2 < 0f64) {
                                                                                                                                                            Vec::<f64>::new()
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let ml_s2233 = if arguments_at_2 >= 0f64 {
                                                                                                                                                                    {
                                                                                                                                                                        let children = self::read_empty_collection_binding_children(terms, parents, arguments_at_2);
                                                                                                                                                                        if (((children.len() as f64) != 2f64) || ((self::read_required_text_fact(terms, self::read_required_number_fact(&(children), 0f64))) != "(")) || ((self::read_required_text_fact(terms, self::read_required_number_fact(&(children), 1f64))) != ")") {
                                                                                                                                                                            self::MlReadEmptyStandardCollectionModuleBindingJoin35::MlReadEmptyStandardCollectionModuleBindingJoin35Return(Vec::<f64>::new())
                                                                                                                                                                        } else {
                                                                                                                                                                            self::MlReadEmptyStandardCollectionModuleBindingJoin35::MlReadEmptyStandardCollectionModuleBindingJoin35Next
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlReadEmptyStandardCollectionModuleBindingJoin35::MlReadEmptyStandardCollectionModuleBindingJoin35Next
                                                                                                                                                                };
                                                                                                                                                                match ml_s2233 {
                                                                                                                                                                    self::MlReadEmptyStandardCollectionModuleBindingJoin35::MlReadEmptyStandardCollectionModuleBindingJoin35Next => {
                                                                                                                                                                        {
                                                                                                                                                                            let owner = self::read_required_text_fact(texts, constructor_2);
                                                                                                                                                                            if owner == "Map" {
                                                                                                                                                                                vec![name_2, 1f64]
                                                                                                                                                                            } else if owner == "Set" {
                                                                                                                                                                                    vec![name_2, 2f64]
                                                                                                                                                                                } else {
                                                                                                                                                                                    Vec::<f64>::new()
                                                                                                                                                                                }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    self::MlReadEmptyStandardCollectionModuleBindingJoin35::MlReadEmptyStandardCollectionModuleBindingJoin35Return(ml_result35) => {
                                                                                                                                                                        ml_result35
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Return(ml_result35_2) => {
                                                                                                                                                        ml_result35_2
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
                                                                                                                self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Return(ml_result35_3) => {
                                                                                                                    ml_result35_3
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
                                                                            self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Return(ml_result35_4) => {
                                                                                ml_result35_4
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
                                            self::MlReadEmptyStandardCollectionModuleBindingJoin31::MlReadEmptyStandardCollectionModuleBindingJoin31Return(ml_result35_5) => {
                                                ml_result35_5
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                    }
                    self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(ml_result36_2) => {
                        ml_result36_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_numeric_module_binding_children(
    terms: &[String],
    parents: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::ml_read_numeric_module_binding_children_loop37(
                    terms, parents, owner, result, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_numeric_module_binding_number_literal(text: &str) -> bool {
    if ((text.encode_utf16().count() as f64) == 0f64) || text.contains('n') {
        false
    } else {
        {
            let ml_values38 = vec![
                String::from("00"),
                String::from("01"),
                String::from("02"),
                String::from("03"),
                String::from("04"),
                String::from("05"),
                String::from("06"),
                String::from("07"),
                String::from("08"),
                String::from("09"),
            ];
            {
                let ml_index38 = self::ml::Big::from_i128(0);
                {
                    let ml_s2244 = self::ml_accept_numeric_module_binding_number_literal_loop39(
                        text,
                        &(ml_values38),
                        ml_index38,
                    );
                    match ml_s2244 {
                        self::MlAcceptNumericModuleBindingNumberLiteralLoop39Result::MlAcceptNumericModuleBindingNumberLiteralLoop39Done => {
                            true
                        }
                        self::MlAcceptNumericModuleBindingNumberLiteralLoop39Result::MlAcceptNumericModuleBindingNumberLiteralLoop39Return(ml_result39) => {
                            ml_result39
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_numeric_module_binding(
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
                let ml_s2255 = self::ml_read_numeric_module_binding_loop43(
                    terms,
                    parents,
                    starts,
                    ends,
                    errors,
                    source_bytes,
                    at,
                );
                match ml_s2255 {
                    self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Done => {
                        {
                            let top = self::read_numeric_module_binding_children(terms, parents, 0f64);
                            {
let ml_library_inequality_2 = (top.len() as f64) != 1f64 ;
if ml_library_inequality_2 {
                                Vec::<f64>::new()
                            } else {
                                {
                                    let declaration = self::read_required_number_fact(&(top), 0f64);
                                    {
                                        let ml_s2277 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                            {
                                                let selected = -1f64;
                                                {
                                                    let exports = 0f64;
                                                    {
                                                        let ml_values40 = self::read_numeric_module_binding_children(terms, parents, declaration);
                                                        {
                                                            let ml_index40 = self::ml::Big::from_i128(0);
                                                            {
                                                                let ml_s2288 = self::ml_read_numeric_module_binding_loop48(terms, selected, exports, &(ml_values40), ml_index40);
                                                                match ml_s2288 {
                                                                    self::MlReadNumericModuleBindingLoop48Result::MlReadNumericModuleBindingLoop48Done(selected_2, exports_2) => {
                                                                        if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                            self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Return(Vec::<f64>::new())
                                                                        } else {
                                                                            {
                                                                                let declaration_2 = selected_2;
                                                                                self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Next(declaration_2)
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlReadNumericModuleBindingLoop48Result::MlReadNumericModuleBindingLoop48Return(ml_result48) => {
                                                                        self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Return(ml_result48)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Next(declaration)
                                        };
                                        match ml_s2277 {
                                            self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Next(declaration_3) => {
                                                {
let ml_library_inequality_1 = (self::read_required_text_fact(terms, declaration_3)) != "lexical_declaration" ;
if ml_library_inequality_1 {
                                                    Vec::<f64>::new()
                                                } else {
                                                    {
                                                        let binding = -1f64;
                                                        {
                                                            let constants = 0f64;
                                                            {
                                                                let ml_values41 = self::read_numeric_module_binding_children(terms, parents, declaration_3);
                                                                {
                                                                    let ml_index41 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let ml_s2299 = self::ml_read_numeric_module_binding_loop45(terms, binding, constants, &(ml_values41), ml_index41);
                                                                        match ml_s2299 {
                                                                            self::MlReadNumericModuleBindingLoop45Result::MlReadNumericModuleBindingLoop45Done(binding_2, constants_2) => {
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
                                                                                                    let ml_values42 = self::read_numeric_module_binding_children(terms, parents, binding_2);
                                                                                                    {
                                                                                                        let ml_index42 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let ml_s3300 = self::ml_read_numeric_module_binding_loop46(terms, fields, name, value, equals, &(ml_values42), ml_index42);
                                                                                                            match ml_s3300 {
                                                                                                                self::MlReadNumericModuleBindingLoop46Result::MlReadNumericModuleBindingLoop46Done(name_2, value_2, equals_2) => {
                                                                                                                    if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains('\\') {
                                                                                                                        Vec::<f64>::new()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let values = Vec::<bool>::new();
                                                                                                                            {
                                                                                                                                let at_2 = (terms.len() as f64) - 1f64;
                                                                                                                                {
                                                                                                                                    let numeric_literal_decisions = self::ml_read_numeric_module_binding_loop47(terms, parents, texts, values, at_2);
                                                                                                                                    if self::read_required_boolean_fact(&(numeric_literal_decisions), (((terms.len() as f64) - 1f64) - value_2)) {
                                                                                                                                        vec![name_2, 0f64]
                                                                                                                                    } else {
                                                                                                                                        Vec::<f64>::new()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlReadNumericModuleBindingLoop46Result::MlReadNumericModuleBindingLoop46Return(ml_result47) => {
                                                                                                                    ml_result47
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
                                                                            self::MlReadNumericModuleBindingLoop45Result::MlReadNumericModuleBindingLoop45Return(ml_result47_2) => {
                                                                                ml_result47_2
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
                                            self::MlReadNumericModuleBindingJoin44::MlReadNumericModuleBindingJoin44Return(ml_result47_3) => {
                                                ml_result47_3
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                    }
                    self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Return(ml_result48_2) => {
                        ml_result48_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_numeric_module_binding_signature(
    kind: &str,
    type_: &str,
    literal: bool,
    source_name: &str,
    signature_name: &str,
) -> bool {
    ((((kind == "const") && (type_ == "float")) && !literal)
        && ((source_name.encode_utf16().count() as f64) > 0f64))
        && (source_name == signature_name)
}
