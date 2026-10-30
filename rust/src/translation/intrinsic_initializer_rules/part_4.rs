// Generated from complete js/src/translation/intrinsic-initializer-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptInertModuleBindingScopeLoop23Result;
use super::MlTemporaryAcceptInertModuleBindingScopeLoop24Result;
use super::children_of_intrinsic_initializer;
use super::intrinsic_initializer_deferred_body;
use super::intrinsic_initializer_sequence;
use super::intrinsic_initializer_value;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_inert_module_binding_scope_loop24(
    mut owner: f64,
    source_ml_temporary_values22: &[String],
    mut source_ml_temporary_index22: self::ml::Big,
) -> self::MlTemporaryAcceptInertModuleBindingScopeLoop24Result {
    loop {
        return if source_ml_temporary_index22
            < (self::ml::Big::from_u128(source_ml_temporary_values22.len() as u128))
        {
            {
                let binding = self::ml_array::at(
                    source_ml_temporary_values22,
                    source_ml_temporary_index22.to_index(),
                );
                if (((owner == 1f64) && (binding == "Map"))
                    || ((owner == 2f64) && (binding == "Set")))
                    || ((owner == 3f64) && (binding == "Object"))
                {
                    self::MlTemporaryAcceptInertModuleBindingScopeLoop24Result::MlTemporaryAcceptInertModuleBindingScopeLoop24Return(false)
                } else {
                    {
                        let source_ml_temporary_index22_2 =
                            source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index22 = source_ml_temporary_index22_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptInertModuleBindingScopeLoop24Result::MlTemporaryAcceptInertModuleBindingScopeLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_inert_module_binding_scope_loop23(
    forms: &[String],
    owners: &[f64],
    bindings: &[String],
    mut at: f64,
) -> self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result {
    loop {
        return if at < (forms.len() as f64) {
            {
                let form = self::read_required_text_fact(forms, at);
                {
                    let owner = self::read_required_number_fact(owners, at);
                    if form == "owned inert value binding" {
                        if (((0f64..=3f64).contains(&owner)) && ((owner % 1f64) == 0f64)) {
                            {
                                let source_ml_temporary_values22 = bindings.to_vec();
                                {
                                    let source_ml_temporary_index22 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s28 = self::source_ml_temporary_accept_inert_module_binding_scope_loop24(owner, &(source_ml_temporary_values22), source_ml_temporary_index22);
                                        match source_ml_source1_s28 {
                                            self::MlTemporaryAcceptInertModuleBindingScopeLoop24Result::MlTemporaryAcceptInertModuleBindingScopeLoop24Done => {
                                                {
                                                    let at_2 = at + 1f64;
                                                    {
                                                        at = at_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlTemporaryAcceptInertModuleBindingScopeLoop24Result::MlTemporaryAcceptInertModuleBindingScopeLoop24Return(source_ml_temporary_result24) => {
                                                self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Return(source_ml_temporary_result24)
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Return(false)
                        }
                    } else if (owner != -1f64)
                        || (((((((form != "comment") && (form != "named function declaration"))
                            && (form != "named import"))
                            && (form != "primitive literal binding"))
                            && (form != "empty class declaration"))
                            && (form != "deferred named class declaration"))
                            && (form != "deferred function value binding"))
                    {
                        self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Return(false)
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
        } else {
            self::MlTemporaryAcceptInertModuleBindingScopeLoop23Result::MlTemporaryAcceptInertModuleBindingScopeLoop23Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_children_of_intrinsic_initializer_loop25(
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
                        let source_ml_source1_concat_value29 = result;
                        {
                            let source_ml_source1_concat_value30 = vec![at];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value29,
                                ),
                                source_ml_source1_concat_value30,
                            )
                        }
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
pub enum MlTemporaryIntrinsicInitializerSequenceLoop26Result {
    MlTemporaryIntrinsicInitializerSequenceLoop26Done,
    MlTemporaryIntrinsicInitializerSequenceLoop26Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryIntrinsicInitializerSequenceJoin27 {
    MlTemporaryIntrinsicInitializerSequenceJoin27Next,
    MlTemporaryIntrinsicInitializerSequenceJoin27Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_intrinsic_initializer_sequence_loop26(
    terms: &[String],
    values: &[bool],
    mut object: bool,
    children: &[f64],
    mut expect_value: bool,
    mut index: f64,
) -> self::MlTemporaryIntrinsicInitializerSequenceLoop26Result {
    loop {
        return if index < ((children.len() as f64) - 1f64) {
            {
                let child = self::read_required_number_fact(children, index);
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let source_ml_source1_s31 = if expect_value {
                            if !(self::intrinsic_initializer_value(
                                values,
                                (terms.len() as f64),
                                child,
                            )) || ((object && (term != "pair")) && (term != "method_definition"))
                            {
                                self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Return(false)
                            } else {
                                self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Next
                            }
                        } else {
                            let ml_library_inequality_1 = (term != ",");
                            if ml_library_inequality_1 {
                                self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Return(false)
                            } else {
                                self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Next
                            }
                        };
                        match source_ml_source1_s31 {
                            self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Next => {
                                {
                                    let expect_value_2 = !expect_value;
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (expect_value, index) = (expect_value_2, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryIntrinsicInitializerSequenceJoin27::MlTemporaryIntrinsicInitializerSequenceJoin27Return(source_ml_temporary_result27) => {
                                self::MlTemporaryIntrinsicInitializerSequenceLoop26Result::MlTemporaryIntrinsicInitializerSequenceLoop26Return(source_ml_temporary_result27)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryIntrinsicInitializerSequenceLoop26Result::MlTemporaryIntrinsicInitializerSequenceLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result {
    MlTemporaryIntrinsicInitializerDeferredBodyLoop29Done(f64, f64, f64),
    MlTemporaryIntrinsicInitializerDeferredBodyLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_intrinsic_initializer_deferred_body_loop29(
    terms: &[String],
    fields: &[String],
    kind: &str,
    mut parameters: f64,
    mut body: f64,
    mut names: f64,
    source_ml_temporary_values28: &[f64],
    mut source_ml_temporary_index28: self::ml::Big,
) -> self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result {
    loop {
        return if source_ml_temporary_index28
            < (self::ml::Big::from_u128(source_ml_temporary_values28.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values28,
                    source_ml_temporary_index28.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if ((field == "parameters") && (term == "formal_parameters"))
                            || ((field == "parameter") && (term == "identifier"))
                        {
                            {
                                let parameters_2 = parameters + 1f64;
                                {
                                    let source_ml_temporary_index28_2 = source_ml_temporary_index28
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (parameters, source_ml_temporary_index28) =
                                            (parameters_2, source_ml_temporary_index28_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if field == "body" {
                                {
                                    let body_2 = body + 1f64;
                                    {
                                        let source_ml_temporary_index28_3 =
                                            source_ml_temporary_index28
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (body, source_ml_temporary_index28) =
                                                (body_2, source_ml_temporary_index28_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if field == "name" {
                                    if ((((kind == "method_definition")
                                        && (term != "property_identifier"))
                                        && (term != "string"))
                                        && (term != "number"))
                                        || ((kind != "method_definition") && (term != "identifier"))
                                    {
                                        self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Return(false)
                                    } else {
                                        {
                                            let names_2 = names + 1f64;
                                            {
                                                let source_ml_temporary_index28_4 =
                                                    source_ml_temporary_index28
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (names, source_ml_temporary_index28) =
                                                        (names_2, source_ml_temporary_index28_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else if (((((term != "function") && (term != "=>"))
                                    && (term != "async"))
                                    && (term != "*"))
                                    && (term != "get"))
                                    && (term != "set")
                                {
                                    self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Return(false)
                                } else {
                                    {
                                        let source_ml_temporary_index28_5 =
                                            source_ml_temporary_index28
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index28 =
                                                source_ml_temporary_index28_5;
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
            self::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Result::MlTemporaryIntrinsicInitializerDeferredBodyLoop29Done(parameters, body, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop41Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop41Done(f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop41(
    terms: &[String],
    fields: &[String],
    mut key: f64,
    mut item: f64,
    mut colons: f64,
    source_ml_temporary_values33: &[f64],
    mut source_ml_temporary_index33: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop41Result {
    loop {
        return if source_ml_temporary_index33
            < (self::ml::Big::from_u128(source_ml_temporary_values33.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values33,
                    source_ml_temporary_index33.to_index(),
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
                                    let source_ml_temporary_index33_2 = source_ml_temporary_index33
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (key, source_ml_temporary_index33) =
                                            (key_2, source_ml_temporary_index33_2);
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
                                        let source_ml_temporary_index33_3 =
                                            source_ml_temporary_index33
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (item, source_ml_temporary_index33) =
                                                (item_2, source_ml_temporary_index33_3);
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
                                            let source_ml_temporary_index33_4 =
                                                source_ml_temporary_index33
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (colons, source_ml_temporary_index33) =
                                                    (colons_2, source_ml_temporary_index33_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let key_3 = -2f64;
                                        self::MlTemporaryReadIntrinsicModuleInitializerLoop41Result::MlTemporaryReadIntrinsicModuleInitializerLoop41Done(key_3, item, colons)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop41Result::MlTemporaryReadIntrinsicModuleInitializerLoop41Done(key, item, colons)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop40(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    mut count: f64,
    mut values: Vec<bool>,
    mut at: f64,
) -> Vec<bool> {
    loop {
        if at >= 0f64 {
            {
                let term = self::read_required_text_fact(terms, at);
                {
                    let children = self::children_of_intrinsic_initializer(terms, parents, at);
                    {
                        let admitted = false;
                        {
                            let admitted_9 = if (((((term == "string") || (term == "number"))
                                || (term == "true"))
                                || (term == "false"))
                                || (term == "null"))
                                || (term == "regex")
                            {
                                { true }
                            } else if term == "array" {
                                {
                                    self::intrinsic_initializer_sequence(
                                        terms,
                                        parents,
                                        &(values),
                                        at,
                                        "[",
                                        "]",
                                        false,
                                    )
                                }
                            } else if term == "object" {
                                {
                                    self::intrinsic_initializer_sequence(
                                        terms,
                                        parents,
                                        &(values),
                                        at,
                                        "{",
                                        "}",
                                        true,
                                    )
                                }
                            } else if (((term == "function_expression")
                                || (term == "generator_function"))
                                || (term == "arrow_function"))
                                || (term == "method_definition")
                            {
                                {
                                    self::intrinsic_initializer_deferred_body(
                                        terms, fields, parents, at,
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
                                                let source_ml_temporary_values33 = children;
                                                {
                                                    let source_ml_temporary_index33 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s32 = self::source_ml_temporary_read_intrinsic_module_initializer_loop41(terms, fields, key, item, colons, &(source_ml_temporary_values33), source_ml_temporary_index33);
                                                        match source_ml_source1_s32 {
                                                                            self::MlTemporaryReadIntrinsicModuleInitializerLoop41Result::MlTemporaryReadIntrinsicModuleInitializerLoop41Done(key_2, item_2, colons_2) => {
                                                                                {
                                                                                    (((key_2 >= 0f64) && (item_2 >= 0f64)) && (colons_2 == 1f64)) && self::intrinsic_initializer_value(&(values), count, item_2)
                                                                                }
                                                                            }
                                                                        }
                                                    }
                                                }
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
                                        && self::intrinsic_initializer_value(
                                            &(values),
                                            count,
                                            self::read_required_number_fact(&(children), 1f64),
                                        )
                                }
                            } else if (term == "unary_expression")
                                && ((children.len() as f64) == 2f64)
                            {
                                {
                                    let operation = self::read_required_text_fact(
                                        terms,
                                        self::read_required_number_fact(&(children), 0f64),
                                    );
                                    {
                                        ((operation == "+") || (operation == "-"))
                                            && ((self::read_required_text_fact(
                                                terms,
                                                self::read_required_number_fact(&(children), 1f64),
                                            )) == "number")
                                    }
                                }
                            } else {
                                admitted
                            };
                            {
                                let values_2 = {
                                    let source_ml_source1_concat_value33 = values;
                                    {
                                        let source_ml_source1_concat_value34 = vec![admitted_9];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<bool>::new(),
                                                source_ml_source1_concat_value33,
                                            ),
                                            source_ml_source1_concat_value34,
                                        )
                                    }
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
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop43Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop43Done,
    MlTemporaryReadIntrinsicModuleInitializerLoop43Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop43(
    terms: &[String],
    parents: &[f64],
    source_ml_temporary_values35: &[f64],
    mut source_ml_temporary_index35: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result {
    loop {
        return if source_ml_temporary_index35
            < (self::ml::Big::from_u128(source_ml_temporary_values35.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values35,
                    source_ml_temporary_index35.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    if ((term == "[") || (term == "]")) || (term == ",") {
                        {
                            let source_ml_temporary_index35_2 =
                                source_ml_temporary_index35.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index35 = source_ml_temporary_index35_2;
                                continue;
                            }
                        }
                    }
                    {
                        if term == "array" {
                            {
                                let pair =
                                    self::children_of_intrinsic_initializer(terms, parents, child);
                                if ((pair.len() as f64) != 5f64) && ((pair.len() as f64) != 6f64) {
                                    self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result::MlTemporaryReadIntrinsicModuleInitializerLoop43Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let source_ml_temporary_index35_3 =
                                            source_ml_temporary_index35
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index35 =
                                                source_ml_temporary_index35_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result::MlTemporaryReadIntrinsicModuleInitializerLoop43Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop43Result::MlTemporaryReadIntrinsicModuleInitializerLoop43Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop42Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop42Done(f64, f64, f64),
    MlTemporaryReadIntrinsicModuleInitializerLoop42Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop42(
    terms: &[String],
    fields: &[String],
    mut callee: f64,
    mut arguments_at: f64,
    mut news: f64,
    source_ml_temporary_values34: &[f64],
    mut source_ml_temporary_index34: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop42Result {
    loop {
        return if source_ml_temporary_index34
            < (self::ml::Big::from_u128(source_ml_temporary_values34.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values34,
                    source_ml_temporary_index34.to_index(),
                );
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if (((field == "constructor") && (term == "identifier"))
                            || ((field == "function") && (term == "member_expression")))
                            && (callee < 0f64)
                        {
                            {
                                let callee_2 = child;
                                {
                                    let source_ml_temporary_index34_2 = source_ml_temporary_index34
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (callee, source_ml_temporary_index34) =
                                            (callee_2, source_ml_temporary_index34_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "arguments") && (term == "arguments"))
                                && (arguments_at < 0f64)
                            {
                                {
                                    let arguments_at_2 = child;
                                    {
                                        let source_ml_temporary_index34_3 =
                                            source_ml_temporary_index34
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, source_ml_temporary_index34) =
                                                (arguments_at_2, source_ml_temporary_index34_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if term == "new" {
                                    {
                                        let news_2 = news + 1f64;
                                        {
                                            let source_ml_temporary_index34_4 =
                                                source_ml_temporary_index34
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (news, source_ml_temporary_index34) =
                                                    (news_2, source_ml_temporary_index34_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlTemporaryReadIntrinsicModuleInitializerLoop42Result::MlTemporaryReadIntrinsicModuleInitializerLoop42Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop42Result::MlTemporaryReadIntrinsicModuleInitializerLoop42Done(callee, arguments_at, news)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop39Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop39Done(f64, f64, f64),
    MlTemporaryReadIntrinsicModuleInitializerLoop39Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop39(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    source_ml_temporary_values32: &[f64],
    mut source_ml_temporary_index32: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop39Result {
    loop {
        return if source_ml_temporary_index32
            < (self::ml::Big::from_u128(source_ml_temporary_values32.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values32,
                    source_ml_temporary_index32.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let source_ml_temporary_index32_2 = source_ml_temporary_index32
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, source_ml_temporary_index32) =
                                            (name_2, source_ml_temporary_index32_2);
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
                                        let source_ml_temporary_index32_3 =
                                            source_ml_temporary_index32
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, source_ml_temporary_index32) =
                                                (value_2, source_ml_temporary_index32_3);
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
                                            let source_ml_temporary_index32_4 =
                                                source_ml_temporary_index32
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, source_ml_temporary_index32) =
                                                    (equals_2, source_ml_temporary_index32_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlTemporaryReadIntrinsicModuleInitializerLoop39Result::MlTemporaryReadIntrinsicModuleInitializerLoop39Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop39Result::MlTemporaryReadIntrinsicModuleInitializerLoop39Done(name, value, equals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop38Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop38Done(f64, f64),
    MlTemporaryReadIntrinsicModuleInitializerLoop38Return(Vec<f64>),
}
