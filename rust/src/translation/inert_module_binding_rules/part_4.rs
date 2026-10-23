// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptInertModuleBindingScopeLoop23Result;
use super::MlAcceptInertModuleBindingScopeLoop24Result;
use super::accept_numeric_module_binding_number_literal;
use super::ml;
use super::ml_accept_inert_module_binding_scope_loop24;
use super::ml_array;
use super::read_numeric_module_binding_children;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inert_module_binding_scope_loop23(
    forms: &[String],
    owners: &[f64],
    bindings: &[String],
    mut at: f64,
) -> self::MlAcceptInertModuleBindingScopeLoop23Result {
    loop {
        return if at < (forms.len() as f64) {
            {
                let form = self::read_required_text_fact(forms, at);
                {
                    let owner = self::read_required_number_fact(owners, at);
                    if form == "owned inert value binding" {
                        if (((0f64..=3f64).contains(&owner)) && ((owner % 1f64) == 0f64)) {
                            {
                                let ml_values22 = bindings.to_vec();
                                {
                                    let ml_index22 = self::ml::Big::from_i128(0);
                                    {
                                        let ml_s3333 =
                                            self::ml_accept_inert_module_binding_scope_loop24(
                                                owner,
                                                &(ml_values22),
                                                ml_index22,
                                            );
                                        match ml_s3333 {
                                            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Done => {
                                                {
                                                    let at_2 = at + 1f64;
                                                    {
                                                        at = at_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Return(ml_result24) => {
                                                self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(ml_result24)
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(false)
                        }
                    } else if (owner != -1f64)
                        || (((((((form != "comment") && (form != "named function declaration"))
                            && (form != "named import"))
                            && (form != "primitive literal binding"))
                            && (form != "empty class declaration"))
                            && (form != "deferred named class declaration"))
                            && (form != "deferred function value binding"))
                    {
                        self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(false)
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
            self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_collection_binding_children_loop25(
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingJoin35 {
    MlReadEmptyStandardCollectionModuleBindingJoin35Next,
    MlReadEmptyStandardCollectionModuleBindingJoin35Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop34Result {
    MlReadEmptyStandardCollectionModuleBindingLoop34Done(f64, f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop34Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop34(
    terms: &[String],
    fields: &[String],
    mut constructor: f64,
    mut arguments_at: f64,
    mut news: f64,
    ml_values29: &[f64],
    mut ml_index29: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop34Result {
    loop {
        return if ml_index29 < (self::ml::Big::from_u128(ml_values29.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values29, ml_index29.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "constructor") && (term == "identifier"))
                            && (constructor < 0f64)
                        {
                            {
                                let constructor_2 = child;
                                {
                                    let ml_index29_2 = ml_index29.add(&self::ml::Big::from_i128(1));
                                    {
                                        (constructor, ml_index29) = (constructor_2, ml_index29_2);
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
                                        let ml_index29_3 =
                                            ml_index29.add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, ml_index29) =
                                                (arguments_at_2, ml_index29_3);
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
                                            let ml_index29_4 =
                                                ml_index29.add(&self::ml::Big::from_i128(1));
                                            {
                                                (news, ml_index29) = (news_2, ml_index29_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Done(constructor, arguments_at, news)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop33Result {
    MlReadEmptyStandardCollectionModuleBindingLoop33Done(f64, f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop33Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop33(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values28: &[f64],
    mut ml_index28: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop33Result {
    loop {
        return if ml_index28 < (self::ml::Big::from_u128(ml_values28.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values28, ml_index28.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index28_2 = ml_index28.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index28) = (name_2, ml_index28_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "value") && (term == "new_expression")) && (value < 0f64)
                            {
                                {
                                    let value_2 = child;
                                    {
                                        let ml_index28_3 =
                                            ml_index28.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index28) = (value_2, ml_index28_3);
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
                                            let ml_index28_4 =
                                                ml_index28.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index28) = (equals_2, ml_index28_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Done(name, value, equals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop32Result {
    MlReadEmptyStandardCollectionModuleBindingLoop32Done(f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop32Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop32(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    ml_values27: &[f64],
    mut ml_index27: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop32Result {
    loop {
        return if ml_index27 < (self::ml::Big::from_u128(ml_values27.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values27, ml_index27.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let ml_index27_2 = ml_index27.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, ml_index27) = (constants_2, ml_index27_2);
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
                                    let ml_index27_3 = ml_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, ml_index27) = (binding_2, ml_index27_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index27_4 = ml_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index27 = ml_index27_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Done(binding, constants)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingJoin31 {
    MlReadEmptyStandardCollectionModuleBindingJoin31Next(f64),
    MlReadEmptyStandardCollectionModuleBindingJoin31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop36Result {
    MlReadEmptyStandardCollectionModuleBindingLoop36Done(f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop36Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop36(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values26: &[f64],
    mut ml_index26: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop36Result {
    loop {
        return if ml_index26 < (self::ml::Big::from_u128(ml_values26.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values26, ml_index26.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index26_2 = ml_index26.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index26) = (exports_2, ml_index26_2);
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
                                    let ml_index26_3 = ml_index26.add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, ml_index26) = (selected_2, ml_index26_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index26_4 = ml_index26.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index26 = ml_index26_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Done(selected, exports)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop30Result {
    MlReadEmptyStandardCollectionModuleBindingLoop30Done,
    MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop30(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop30Result {
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
                            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
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
                            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
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
            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_children_loop37(
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
pub enum MlAcceptNumericModuleBindingNumberLiteralLoop39Result {
    MlAcceptNumericModuleBindingNumberLiteralLoop39Done,
    MlAcceptNumericModuleBindingNumberLiteralLoop39Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_numeric_module_binding_number_literal_loop39(
    text: &str,
    ml_values38: &[String],
    mut ml_index38: self::ml::Big,
) -> self::MlAcceptNumericModuleBindingNumberLiteralLoop39Result {
    loop {
        return if ml_index38 < (self::ml::Big::from_u128(ml_values38.len() as u128)) {
            {
                let prefix = self::ml_array::at(ml_values38, ml_index38.to_index());
                if text.starts_with(prefix.as_str()) {
                    self::MlAcceptNumericModuleBindingNumberLiteralLoop39Result::MlAcceptNumericModuleBindingNumberLiteralLoop39Return(false)
                } else {
                    {
                        let ml_index38_2 = ml_index38.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index38 = ml_index38_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptNumericModuleBindingNumberLiteralLoop39Result::MlAcceptNumericModuleBindingNumberLiteralLoop39Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_loop47(
    terms: &[String],
    parents: &[f64],
    texts: &[String],
    mut values: Vec<bool>,
    mut at: f64,
) -> Vec<bool> {
    loop {
        if at >= 0f64 {
            {
                let term = self::read_required_text_fact(terms, at);
                {
                    let children = self::read_numeric_module_binding_children(terms, parents, at);
                    {
                        let admitted = false;
                        {
                            let admitted_6 = if term == "number" {
                                {
                                    ((children.len() as f64) == 0f64)
                                        && self::accept_numeric_module_binding_number_literal(
                                            &(self::read_required_text_fact(texts, at)),
                                        )
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
                                        && self::read_required_boolean_fact(
                                            &(values),
                                            (((terms.len() as f64) - 1f64)
                                                - self::read_required_number_fact(
                                                    &(children),
                                                    1f64,
                                                )),
                                        )
                                }
                            } else if (term == "unary_expression")
                                && ((children.len() as f64) == 2f64)
                            {
                                {
                                    let operator = self::read_required_text_fact(
                                        terms,
                                        self::read_required_number_fact(&(children), 0f64),
                                    );
                                    {
                                        (((operator == "+") || (operator == "-"))
                                            || (operator == "~"))
                                            && self::read_required_boolean_fact(
                                                &(values),
                                                (((terms.len() as f64) - 1f64)
                                                    - self::read_required_number_fact(
                                                        &(children),
                                                        1f64,
                                                    )),
                                            )
                                    }
                                }
                            } else if (term == "binary_expression")
                                && ((children.len() as f64) == 3f64)
                            {
                                {
                                    let operator_2 = self::read_required_text_fact(
                                        terms,
                                        self::read_required_number_fact(&(children), 1f64),
                                    );
                                    {
                                        let numeric = (((((((((((operator_2 == "+")
                                            || (operator_2 == "-"))
                                            || (operator_2 == "*"))
                                            || (operator_2 == "/"))
                                            || (operator_2 == "%"))
                                            || (operator_2 == "**"))
                                            || (operator_2 == "<<"))
                                            || (operator_2 == ">>"))
                                            || (operator_2 == ">>>"))
                                            || (operator_2 == "&"))
                                            || (operator_2 == "|"))
                                            || (operator_2 == "^");
                                        {
                                            (numeric
                                                && self::read_required_boolean_fact(
                                                    &(values),
                                                    (((terms.len() as f64) - 1f64)
                                                        - self::read_required_number_fact(
                                                            &(children),
                                                            0f64,
                                                        )),
                                                ))
                                                && self::read_required_boolean_fact(
                                                    &(values),
                                                    (((terms.len() as f64) - 1f64)
                                                        - self::read_required_number_fact(
                                                            &(children),
                                                            2f64,
                                                        )),
                                                )
                                        }
                                    }
                                }
                            } else {
                                admitted
                            };
                            {
                                let values_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![admitted_6], values);
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
pub enum MlReadNumericModuleBindingLoop46Result {
    MlReadNumericModuleBindingLoop46Done(f64, f64, f64),
    MlReadNumericModuleBindingLoop46Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_loop46(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values42: &[f64],
    mut ml_index42: self::ml::Big,
) -> self::MlReadNumericModuleBindingLoop46Result {
    loop {
        return if ml_index42 < (self::ml::Big::from_u128(ml_values42.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values42, ml_index42.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index42_2 = ml_index42.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index42) = (name_2, ml_index42_2);
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
                                        let ml_index42_3 =
                                            ml_index42.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index42) = (value_2, ml_index42_3);
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
                                            let ml_index42_4 =
                                                ml_index42.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index42) = (equals_2, ml_index42_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadNumericModuleBindingLoop46Result::MlReadNumericModuleBindingLoop46Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadNumericModuleBindingLoop46Result::MlReadNumericModuleBindingLoop46Done(
                name, value, equals,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingLoop45Result {
    MlReadNumericModuleBindingLoop45Done(f64, f64),
    MlReadNumericModuleBindingLoop45Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_loop45(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    ml_values41: &[f64],
    mut ml_index41: self::ml::Big,
) -> self::MlReadNumericModuleBindingLoop45Result {
    loop {
        return if ml_index41 < (self::ml::Big::from_u128(ml_values41.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values41, ml_index41.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let ml_index41_2 = ml_index41.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, ml_index41) = (constants_2, ml_index41_2);
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
                                    let ml_index41_3 = ml_index41.add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, ml_index41) = (binding_2, ml_index41_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index41_4 = ml_index41.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index41 = ml_index41_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadNumericModuleBindingLoop45Result::MlReadNumericModuleBindingLoop45Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadNumericModuleBindingLoop45Result::MlReadNumericModuleBindingLoop45Done(
                binding, constants,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingJoin44 {
    MlReadNumericModuleBindingJoin44Next(f64),
    MlReadNumericModuleBindingJoin44Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingLoop48Result {
    MlReadNumericModuleBindingLoop48Done(f64, f64),
    MlReadNumericModuleBindingLoop48Return(Vec<f64>),
}
