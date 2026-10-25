// Generated from complete js/src/translation/intrinsic-initializer-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptInertModuleBindingScopeLoop23Result;
use super::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result;
use super::MlTemporaryIntrinsicInitializerSequenceLoop26Result;
use super::MlTemporaryReadInertModuleBindingJoin12;
use super::MlTemporaryReadInertModuleBindingJoin17;
use super::MlTemporaryReadInertModuleBindingLoop11Result;
use super::MlTemporaryReadInertModuleBindingLoop13Result;
use super::MlTemporaryReadInertModuleBindingLoop14Result;
use super::MlTemporaryReadInertModuleBindingLoop18Result;
use super::MlTemporaryReadInertModuleBindingLoop19Result;
use super::MlTemporaryReadInertModuleBindingLoop20Result;
use super::MlTemporaryReadInertModuleBindingLoop21Result;
use super::MlTemporaryReadIntrinsicModuleInitializerJoin37;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop36Result;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop38Result;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop39Result;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop42Result;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop43Result;
use super::MlTemporaryReadIntrinsicModuleInitializerLoop44Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_inert_binding_argument;
use super::read_inert_binding_children;
use super::read_inert_binding_value;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_accept_inert_module_binding_scope_loop23;
use super::source_ml_temporary_accept_intrinsic_module_initializer_scope_loop46;
use super::source_ml_temporary_children_of_intrinsic_initializer_loop25;
use super::source_ml_temporary_intrinsic_initializer_deferred_body_loop29;
use super::source_ml_temporary_intrinsic_initializer_sequence_loop26;
use super::source_ml_temporary_read_inert_module_binding_loop11;
use super::source_ml_temporary_read_inert_module_binding_loop13;
use super::source_ml_temporary_read_inert_module_binding_loop14;
use super::source_ml_temporary_read_inert_module_binding_loop15;
use super::source_ml_temporary_read_inert_module_binding_loop18;
use super::source_ml_temporary_read_inert_module_binding_loop19;
use super::source_ml_temporary_read_inert_module_binding_loop20;
use super::source_ml_temporary_read_inert_module_binding_loop21;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop36;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop38;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop39;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop40;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop42;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop43;
use super::source_ml_temporary_read_intrinsic_module_initializer_loop44;

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
                let source_ml_source1_s2 =
                    self::source_ml_temporary_read_inert_module_binding_loop11(
                        terms,
                        parents,
                        starts,
                        ends,
                        errors,
                        source_bytes,
                        at,
                    );
                match source_ml_source1_s2 {
                    self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Done => {
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
                                        let source_ml_source1_s4 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                            {
                                                let children = self::read_inert_binding_children(terms, parents, declaration);
                                                {
                                                    let selected = -1f64;
                                                    {
                                                        let exports = 0f64;
                                                        {
                                                            let source_ml_temporary_values4 = children;
                                                            {
                                                                let source_ml_temporary_index4 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source1_s5 = self::source_ml_temporary_read_inert_module_binding_loop21(terms, selected, exports, &(source_ml_temporary_values4), source_ml_temporary_index4);
                                                                    match source_ml_source1_s5 {
                                                                        self::MlTemporaryReadInertModuleBindingLoop21Result::MlTemporaryReadInertModuleBindingLoop21Done(selected_2, exports_2) => {
                                                                            if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                                self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Return(Vec::<f64>::new())
                                                                            } else {
                                                                                {
                                                                                    let declaration_2 = selected_2;
                                                                                    self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Next(declaration_2)
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlTemporaryReadInertModuleBindingLoop21Result::MlTemporaryReadInertModuleBindingLoop21Return(source_ml_temporary_result21) => {
                                                                            self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Return(source_ml_temporary_result21)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Next(declaration)
                                        };
                                        match source_ml_source1_s4 {
                                            self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Next(declaration_3) => {
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
                                                                let source_ml_temporary_values5 = self::read_inert_binding_children(terms, parents, declaration_3);
                                                                {
                                                                    let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s6 = self::source_ml_temporary_read_inert_module_binding_loop13(terms, binding, constants, &(source_ml_temporary_values5), source_ml_temporary_index5);
                                                                        match source_ml_source1_s6 {
                                                                            self::MlTemporaryReadInertModuleBindingLoop13Result::MlTemporaryReadInertModuleBindingLoop13Done(binding_2, constants_2) => {
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
                                                                                                    let source_ml_temporary_values6 = self::read_inert_binding_children(terms, parents, binding_2);
                                                                                                    {
                                                                                                        let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let source_ml_source1_s7 = self::source_ml_temporary_read_inert_module_binding_loop14(terms, fields, name, value, equals, &(source_ml_temporary_values6), source_ml_temporary_index6);
                                                                                                            match source_ml_source1_s7 {
                                                                                                                self::MlTemporaryReadInertModuleBindingLoop14Result::MlTemporaryReadInertModuleBindingLoop14Done(name_2, value_2, equals_2) => {
                                                                                                                    if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains("\\") {
                                                                                                                        Vec::<f64>::new()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let values = Vec::<bool>::new();
                                                                                                                            {
                                                                                                                                let at_2 = (terms.len() as f64) - 1f64;
                                                                                                                                {
                                                                                                                                    let values_2 = self::source_ml_temporary_read_inert_module_binding_loop15(terms, fields, parents, values, at_2);
                                                                                                                                    if self::read_inert_binding_value(&(values_2), (parents.len() as f64), value_2) {
                                                                                                                                        vec![name_2, 0f64]
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let term = self::read_required_text_fact(terms, value_2);
                                                                                                                                            {
                                                                                                                                                let children_2 = self::read_inert_binding_children(terms, parents, value_2);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_s11 = if term == "new_expression" {
                                                                                                                                                        {
                                                                                                                                                            let constructor = -1f64;
                                                                                                                                                            {
                                                                                                                                                                let arguments_at = -1f64;
                                                                                                                                                                {
                                                                                                                                                                    let news = 0f64;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_temporary_values8 = children_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_s12 = self::source_ml_temporary_read_inert_module_binding_loop18(terms, fields, constructor, arguments_at, news, &(source_ml_temporary_values8), source_ml_temporary_index8);
                                                                                                                                                                                match source_ml_source1_s12 {
                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingLoop18Result::MlTemporaryReadInertModuleBindingLoop18Done(constructor_2, arguments_at_2, news_2) => {
                                                                                                                                                                                        if ((news_2 != 1f64) || (constructor_2 < 0f64)) || (arguments_at_2 < 0f64) {
                                                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                        } else {
                                                                                                                                                                                            {
                                                                                                                                                                                                let owner = self::read_required_text_fact(texts, constructor_2);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let argument = self::read_inert_binding_argument(terms, parents, &(values_2), arguments_at_2);
                                                                                                                                                                                                    if (argument < 0f64) || ((self::read_required_text_fact(terms, argument)) != "array") {
                                                                                                                                                                                                        self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                    } else if owner == "Set" {
                                                                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(vec![name_2, 2f64])
                                                                                                                                                                                                        } else if owner == "Map" {
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_temporary_values9 = self::read_inert_binding_children(terms, parents, argument);
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source1_s13 = self::source_ml_temporary_read_inert_module_binding_loop19(terms, parents, &(source_ml_temporary_values9), source_ml_temporary_index9);
                                                                                                                                                                                                                            match source_ml_source1_s13 {
                                                                                                                                                                                                                                self::MlTemporaryReadInertModuleBindingLoop19Result::MlTemporaryReadInertModuleBindingLoop19Done => {
                                                                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(vec![name_2, 1f64])
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                self::MlTemporaryReadInertModuleBindingLoop19Result::MlTemporaryReadInertModuleBindingLoop19Return(source_ml_temporary_result19) => {
                                                                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(source_ml_temporary_result19)
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Next
                                                                                                                                                                                                            }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingLoop18Result::MlTemporaryReadInertModuleBindingLoop18Return(source_ml_temporary_result19_2) => {
                                                                                                                                                                                        self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(source_ml_temporary_result19_2)
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
                                                                                                                                                                        let source_ml_temporary_values10 = children_2;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_s14 = self::source_ml_temporary_read_inert_module_binding_loop20(terms, fields, callee, arguments_at_3, &(source_ml_temporary_values10), source_ml_temporary_index10);
                                                                                                                                                                                match source_ml_source1_s14 {
                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingLoop20Result::MlTemporaryReadInertModuleBindingLoop20Done(callee_2, arguments_at_4) => {
                                                                                                                                                                                        if (callee_2 < 0f64) || (arguments_at_4 < 0f64) {
                                                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                        } else {
                                                                                                                                                                                            {
                                                                                                                                                                                                let member = self::read_inert_binding_children(terms, parents, callee_2);
                                                                                                                                                                                                {
let ml_library_inequality_1 = (member.len() as f64) != 3f64 ;
if ml_library_inequality_1 {
                                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let object = self::read_required_number_fact(&(member), 0f64);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let dot = self::read_required_number_fact(&(member), 1f64);
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let property = self::read_required_number_fact(&(member), 2f64);
                                                                                                                                                                                                                if (((((self::read_required_text_fact(terms, object)) != "identifier") || ((self::read_required_text_fact(texts, object)) != "Object")) || ((self::read_required_text_fact(terms, dot)) != ".")) || ((self::read_required_text_fact(terms, property)) != "property_identifier")) || ((self::read_required_text_fact(texts, property)) != "freeze") {
                                                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(Vec::<f64>::new())
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let argument_2 = self::read_inert_binding_argument(terms, parents, &(values_2), arguments_at_4);
                                                                                                                                                                                                                        if (argument_2 >= 0f64) && (((self::read_required_text_fact(terms, argument_2)) == "array") || ((self::read_required_text_fact(terms, argument_2)) == "object")) {
                                                                                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(vec![name_2, 3f64])
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Next
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
                                                                                                                                                                                    self::MlTemporaryReadInertModuleBindingLoop20Result::MlTemporaryReadInertModuleBindingLoop20Return(source_ml_temporary_result20) => {
                                                                                                                                                                                        self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(source_ml_temporary_result20)
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Next
                                                                                                                                                        };
                                                                                                                                                    match source_ml_source1_s11 {
                                                                                                                                                        self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Next => {
                                                                                                                                                            Vec::<f64>::new()
                                                                                                                                                        }
                                                                                                                                                        self::MlTemporaryReadInertModuleBindingJoin17::MlTemporaryReadInertModuleBindingJoin17Return(source_ml_temporary_result17) => {
                                                                                                                                                            source_ml_temporary_result17
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
                                                                                                                self::MlTemporaryReadInertModuleBindingLoop14Result::MlTemporaryReadInertModuleBindingLoop14Return(source_ml_temporary_result20_2) => {
                                                                                                                    source_ml_temporary_result20_2
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
                                                                            self::MlTemporaryReadInertModuleBindingLoop13Result::MlTemporaryReadInertModuleBindingLoop13Return(source_ml_temporary_result20_3) => {
                                                                                source_ml_temporary_result20_3
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
                                            self::MlTemporaryReadInertModuleBindingJoin12::MlTemporaryReadInertModuleBindingJoin12Return(source_ml_temporary_result20_4) => {
                                                source_ml_temporary_result20_4
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                    }
                    self::MlTemporaryReadInertModuleBindingLoop11Result::MlTemporaryReadInertModuleBindingLoop11Return(source_ml_temporary_result21_2) => {
                        source_ml_temporary_result21_2
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
                    let source_ml_source1_s15 =
                        self::source_ml_temporary_accept_inert_module_binding_scope_loop23(
                            forms, owners, bindings, at,
                        );
                    match source_ml_source1_s15 {
                    self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Done => {
                        true
                    }
                    self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Return(source_ml_temporary_result23) => {
                        source_ml_temporary_result23
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn children_of_intrinsic_initializer(
    terms: &[String],
    parents: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::source_ml_temporary_children_of_intrinsic_initializer_loop25(
                    terms, parents, owner, result, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn intrinsic_initializer_value(values: &[bool], count: f64, at: f64) -> bool {
    ((at >= 0f64) && (at < count))
        && self::read_required_boolean_fact(values, ((count - 1f64) - at))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn intrinsic_initializer_sequence(
    terms: &[String],
    parents: &[f64],
    values: &[bool],
    owner: f64,
    open: &str,
    close: &str,
    object: bool,
) -> bool {
    {
        let children = self::children_of_intrinsic_initializer(terms, parents, owner);
        if (((children.len() as f64) < 2f64)
            || ((self::read_required_text_fact(
                terms,
                self::read_required_number_fact(&(children), 0f64),
            )) != open))
            || ((self::read_required_text_fact(
                terms,
                self::read_required_number_fact(&(children), ((children.len() as f64) - 1f64)),
            )) != close)
        {
            false
        } else {
            {
                let expect_value = true;
                {
                    let index = 1f64;
                    {
                        let source_ml_source1_s16 =
                            self::source_ml_temporary_intrinsic_initializer_sequence_loop26(
                                terms,
                                values,
                                object,
                                &(children),
                                expect_value,
                                index,
                            );
                        match source_ml_source1_s16 {
                            self::MlTemporaryIntrinsicInitializerSequenceLoop26Result::MlTemporaryIntrinsicInitializerSequenceLoop26Done => {
                                true
                            }
                            self::MlTemporaryIntrinsicInitializerSequenceLoop26Result::MlTemporaryIntrinsicInitializerSequenceLoop26Return(source_ml_temporary_result26) => {
                                source_ml_temporary_result26
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn intrinsic_initializer_deferred_body(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    owner: f64,
) -> bool {
    {
        let kind = self::read_required_text_fact(terms, owner);
        {
            let parameters = 0f64;
            {
                let body = 0f64;
                {
                    let names = 0f64;
                    {
                        let source_ml_temporary_values28 =
                            self::children_of_intrinsic_initializer(terms, parents, owner);
                        {
                            let source_ml_temporary_index28 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s17 = self::source_ml_temporary_intrinsic_initializer_deferred_body_loop29(terms, fields, &(kind), parameters, body, names, &(source_ml_temporary_values28), source_ml_temporary_index28);
                                match source_ml_source1_s17 {
                                    self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Done(parameters_2, body_2, names_2) => {
                                        (((parameters_2 == 1f64) && (body_2 == 1f64)) && (names_2 < 2f64)) && ((kind != "method_definition") || (names_2 == 1f64))
                                    }
                                    self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Return(source_ml_temporary_result29) => {
                                        source_ml_temporary_result29
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_intrinsic_module_initializer(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    texts: &[String],
    errors: &[f64],
    source_bytes: f64,
) -> Vec<f64> {
    {
        let count = terms.len() as f64;
        if ((((((((((count < 1f64) || (count > 4096f64)) || ((fields.len() as f64) != count))
            || ((parents.len() as f64) != count))
            || ((starts.len() as f64) != count))
            || ((ends.len() as f64) != count))
            || ((texts.len() as f64) != count))
            || ((errors.len() as f64) != count))
            || !(self::ml_math::is_integer(source_bytes)))
            || (source_bytes < 0f64))
            || (source_bytes > 65_536f64)
        {
            Vec::<f64>::new()
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s18 =
                        self::source_ml_temporary_read_intrinsic_module_initializer_loop36(
                            terms,
                            parents,
                            starts,
                            ends,
                            errors,
                            source_bytes,
                            count,
                            at,
                        );
                    match source_ml_source1_s18 {
                        self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Done => {
                            {
                                let top = self::children_of_intrinsic_initializer(terms, parents, 0f64);
                                {
let ml_library_inequality_5 = (top.len() as f64) != 1f64 ;
if ml_library_inequality_5 {
                                    Vec::<f64>::new()
                                } else {
                                    {
                                        let declaration = self::read_required_number_fact(&(top), 0f64);
                                        {
                                            let source_ml_source1_s20 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                                {
                                                    let selected = -1f64;
                                                    {
                                                        let exports = 0f64;
                                                        {
                                                            let source_ml_temporary_values30 = self::children_of_intrinsic_initializer(terms, parents, declaration);
                                                            {
                                                                let source_ml_temporary_index30 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source1_s21 = self::source_ml_temporary_read_intrinsic_module_initializer_loop44(terms, selected, exports, &(source_ml_temporary_values30), source_ml_temporary_index30);
                                                                    match source_ml_source1_s21 {
                                                                        self::MlTemporaryReadIntrinsicModuleInitializerLoop44Result::MlTemporaryReadIntrinsicModuleInitializerLoop44Done(selected_2, exports_2) => {
                                                                            if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                                self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Return(Vec::<f64>::new())
                                                                            } else {
                                                                                {
                                                                                    let declaration_2 = selected_2;
                                                                                    self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Next(declaration_2)
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlTemporaryReadIntrinsicModuleInitializerLoop44Result::MlTemporaryReadIntrinsicModuleInitializerLoop44Return(source_ml_temporary_result44) => {
                                                                            self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Return(source_ml_temporary_result44)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Next(declaration)
                                            };
                                            match source_ml_source1_s20 {
                                                self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Next(declaration_3) => {
                                                    {
let ml_library_inequality_4 = (self::read_required_text_fact(terms, declaration_3)) != "lexical_declaration" ;
if ml_library_inequality_4 {
                                                        Vec::<f64>::new()
                                                    } else {
                                                        {
                                                            let binding = -1f64;
                                                            {
                                                                let constants = 0f64;
                                                                {
                                                                    let source_ml_temporary_values31 = self::children_of_intrinsic_initializer(terms, parents, declaration_3);
                                                                    {
                                                                        let source_ml_temporary_index31 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s22 = self::source_ml_temporary_read_intrinsic_module_initializer_loop38(terms, binding, constants, &(source_ml_temporary_values31), source_ml_temporary_index31);
                                                                            match source_ml_source1_s22 {
                                                                                self::MlTemporaryReadIntrinsicModuleInitializerLoop38Result::MlTemporaryReadIntrinsicModuleInitializerLoop38Done(binding_2, constants_2) => {
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
                                                                                                        let source_ml_temporary_values32 = self::children_of_intrinsic_initializer(terms, parents, binding_2);
                                                                                                        {
                                                                                                            let source_ml_temporary_index32 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source1_s23 = self::source_ml_temporary_read_intrinsic_module_initializer_loop39(terms, fields, name, value, equals, &(source_ml_temporary_values32), source_ml_temporary_index32);
                                                                                                                match source_ml_source1_s23 {
                                                                                                                    self::MlTemporaryReadIntrinsicModuleInitializerLoop39Result::MlTemporaryReadIntrinsicModuleInitializerLoop39Done(name_2, value_2, equals_2) => {
                                                                                                                        if ((((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64)) || (((self::read_required_text_fact(texts, name_2)).encode_utf16().count() as f64) == 0f64)) || (self::read_required_text_fact(texts, name_2)).contains("\\") {
                                                                                                                            Vec::<f64>::new()
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let values = Vec::<bool>::new();
                                                                                                                                {
                                                                                                                                    let at_2 = count - 1f64;
                                                                                                                                    {
                                                                                                                                        let values_2 = self::source_ml_temporary_read_intrinsic_module_initializer_loop40(terms, fields, parents, count, values, at_2);
                                                                                                                                        if self::intrinsic_initializer_value(&(values_2), count, value_2) {
                                                                                                                                            vec![name_2, 0f64]
                                                                                                                                        } else {
                                                                                                                                            {
                                                                                                                                                let kind = self::read_required_text_fact(terms, value_2);
                                                                                                                                                {
                                                                                                                                                    let children = self::children_of_intrinsic_initializer(terms, parents, value_2);
                                                                                                                                                    {
                                                                                                                                                        let callee = -1f64;
                                                                                                                                                        {
                                                                                                                                                            let arguments_at = -1f64;
                                                                                                                                                            {
                                                                                                                                                                let news = 0f64;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_temporary_values34 = children;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_temporary_index34 = self::ml::Big::from_i128(0);
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_s24 = self::source_ml_temporary_read_intrinsic_module_initializer_loop42(terms, fields, callee, arguments_at, news, &(source_ml_temporary_values34), source_ml_temporary_index34);
                                                                                                                                                                            match source_ml_source1_s24 {
                                                                                                                                                                                self::MlTemporaryReadIntrinsicModuleInitializerLoop42Result::MlTemporaryReadIntrinsicModuleInitializerLoop42Done(callee_2, arguments_at_2, news_2) => {
                                                                                                                                                                                    if (callee_2 < 0f64) || (arguments_at_2 < 0f64) {
                                                                                                                                                                                        Vec::<f64>::new()
                                                                                                                                                                                    } else {
                                                                                                                                                                                        {
                                                                                                                                                                                            let arguments_children = self::children_of_intrinsic_initializer(terms, parents, arguments_at_2);
                                                                                                                                                                                            if ((arguments_children.len() as f64) != 3f64) && ((arguments_children.len() as f64) != 4f64) {
                                                                                                                                                                                                Vec::<f64>::new()
                                                                                                                                                                                            } else {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let first = self::read_required_number_fact(&(arguments_children), 0f64);
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let argument = self::read_required_number_fact(&(arguments_children), 1f64);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let last = self::read_required_number_fact(&(arguments_children), ((arguments_children.len() as f64) - 1f64));
                                                                                                                                                                                                            if (
 (((self::read_required_text_fact(terms, first)) != "(") || ((self::read_required_text_fact(terms, last)) != ")")) || !(self::intrinsic_initializer_value(&(values_2), count, argument)) 
) || (
 ((arguments_children.len() as f64) == 4f64) && ((self::read_required_text_fact(terms, self::read_required_number_fact(&(arguments_children), 2f64))) != ",") 
) {
                                                                                                                                                                                                                    Vec::<f64>::new()
                                                                                                                                                                                                                } else if (kind == "new_expression") && (news_2 == 1f64) {
                                                                                                                                                                                                                        {
let ml_library_inequality_3 = (self::read_required_text_fact(terms, argument)) != "array" ;
if ml_library_inequality_3 {
                                                                                                                                                                                                                            Vec::<f64>::new()
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let owner = self::read_required_text_fact(texts, callee_2);
                                                                                                                                                                                                                                if owner == "Set" {
                                                                                                                                                                                                                                    vec![name_2, 2f64]
                                                                                                                                                                                                                                } else {
let ml_library_inequality_2 = owner != "Map" ;
if ml_library_inequality_2 {
                                                                                                                                                                                                                                        Vec::<f64>::new()
                                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let source_ml_temporary_values35 = self::children_of_intrinsic_initializer(terms, parents, argument);
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let source_ml_temporary_index35 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let source_ml_source1_s25 = self::source_ml_temporary_read_intrinsic_module_initializer_loop43(terms, parents, &(source_ml_temporary_values35), source_ml_temporary_index35);
                                                                                                                                                                                                                                                    match source_ml_source1_s25 {
                                                                                                                                                                                                                                                        self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result::MlTemporaryReadIntrinsicModuleInitializerLoop43Done => {
                                                                                                                                                                                                                                                            vec![name_2, 1f64]
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result::MlTemporaryReadIntrinsicModuleInitializerLoop43Return(source_ml_temporary_result43) => {
                                                                                                                                                                                                                                                            source_ml_temporary_result43
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
                                                                                                                                                                                                                    } else if (kind != "call_expression") || (news_2 != 0f64) {
                                                                                                                                                                                                                            Vec::<f64>::new()
                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let member = self::children_of_intrinsic_initializer(terms, parents, callee_2);
                                                                                                                                                                                                                                {
let ml_library_inequality_1 = (member.len() as f64) != 3f64 ;
if ml_library_inequality_1 {
                                                                                                                                                                                                                                    Vec::<f64>::new()
                                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let object = self::read_required_number_fact(&(member), 0f64);
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let dot = self::read_required_number_fact(&(member), 1f64);
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let property = self::read_required_number_fact(&(member), 2f64);
                                                                                                                                                                                                                                                if (
 (((((self::read_required_text_fact(terms, object)) != "identifier") || ((self::read_required_text_fact(texts, object)) != "Object")) || ((self::read_required_text_fact(terms, dot)) != ".")) || ((self::read_required_text_fact(terms, property)) != "property_identifier")) || ((self::read_required_text_fact(texts, property)) != "freeze") 
) || (
 ((self::read_required_text_fact(terms, argument)) != "array") && ((self::read_required_text_fact(terms, argument)) != "object") 
) {
                                                                                                                                                                                                                                                        Vec::<f64>::new()
                                                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                                                        vec![name_2, 3f64]
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
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                                self::MlTemporaryReadIntrinsicModuleInitializerLoop42Result::MlTemporaryReadIntrinsicModuleInitializerLoop42Return(source_ml_temporary_result43_2) => {
                                                                                                                                                                                    source_ml_temporary_result43_2
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
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlTemporaryReadIntrinsicModuleInitializerLoop39Result::MlTemporaryReadIntrinsicModuleInitializerLoop39Return(source_ml_temporary_result43_3) => {
                                                                                                                        source_ml_temporary_result43_3
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
                                                                                self::MlTemporaryReadIntrinsicModuleInitializerLoop38Result::MlTemporaryReadIntrinsicModuleInitializerLoop38Return(source_ml_temporary_result43_4) => {
                                                                                    source_ml_temporary_result43_4
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
                                                self::MlTemporaryReadIntrinsicModuleInitializerJoin37::MlTemporaryReadIntrinsicModuleInitializerJoin37Return(source_ml_temporary_result43_5) => {
                                                    source_ml_temporary_result43_5
                                                }
                                            }
                                        }
                                    }
                                }
}
                            }
                        }
                        self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Return(source_ml_temporary_result44_2) => {
                            source_ml_temporary_result44_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_intrinsic_module_initializer_scope(
    forms: &[String],
    owners: &[f64],
    bindings: &[String],
) -> bool {
    {
        let translated = Vec::<String>::new();
        {
            let source_ml_temporary_values45 = forms.to_vec();
            {
                let source_ml_temporary_index45 = self::ml::Big::from_i128(0);
                {
                    let translated_2 =
                        self::source_ml_temporary_accept_intrinsic_module_initializer_scope_loop46(
                            translated,
                            &(source_ml_temporary_values45),
                            source_ml_temporary_index45,
                        );
                    self::accept_inert_module_binding_scope(&(translated_2), owners, bindings)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_inert_binding_children_loop1(
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
pub enum MlTemporaryAcceptInertBindingSequenceLoop2Result {
    MlTemporaryAcceptInertBindingSequenceLoop2Done,
    MlTemporaryAcceptInertBindingSequenceLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptInertBindingSequenceJoin3 {
    MlTemporaryAcceptInertBindingSequenceJoin3Next,
    MlTemporaryAcceptInertBindingSequenceJoin3Return(bool),
}
