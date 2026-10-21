// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlReadDeclarationDeferredBindingLoop60Result;
use super::MlReadDeclarationDeferredBindingLoop61Result;
use super::accept_deferred_declaration_parameters;
use super::ml;
use super::ml_array;
use super::read_deferred_declaration_children;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_declaration_deferred_binding_loop61(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    texts: &[String],
    mut method_name: f64,
    mut parameters: f64,
    mut method_body: f64,
    ml_values52: &[f64],
    mut ml_index52: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop61Result {
    loop {
        return if ml_index52 < (self::ml::Big::from_u128(ml_values52.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values52, ml_index52.to_index());
                {
                    let part = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if ((part == "property_identifier") && (field == "name"))
                            && (method_name < 0f64)
                        {
                            {
                                let method_name_2 = child;
                                {
                                    let ml_index52_2 = ml_index52.add(&self::ml::Big::from_i128(1));
                                    {
                                        (method_name, ml_index52) = (method_name_2, ml_index52_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (part == "formal_parameters") && (field == "parameters") {
                                if (self::accept_deferred_declaration_parameters(
                                    terms, parents, texts, child,
                                )) {
                                    {
                                        let parameters_2 = parameters + 1f64;
                                        {
                                            let ml_index52_3 =
                                                ml_index52.add(&self::ml::Big::from_i128(1));
                                            {
                                                (parameters, ml_index52) =
                                                    (parameters_2, ml_index52_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(Vec::<f64>::new())
                                }
                            } else {
                                if (part == "statement_block") && (field == "body") {
                                    {
                                        let method_body_2 = method_body + 1f64;
                                        {
                                            let ml_index52_4 =
                                                ml_index52.add(&self::ml::Big::from_i128(1));
                                            {
                                                (method_body, ml_index52) =
                                                    (method_body_2, ml_index52_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Done(method_name, parameters, method_body)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop60(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    texts: &[String],
    mut constructors: f64,
    ml_values51: &[f64],
    mut ml_index51: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop60Result {
    loop {
        return if ml_index51 < (self::ml::Big::from_u128(ml_values51.len() as u128)) {
            {
                let member = self::ml_array::at(ml_values51, ml_index51.to_index());
                {
                    let term = self::read_required_text_fact(terms, member);
                    if ((term == "{") || (term == "}")) || (term == ";") {
                        {
                            let ml_index51_2 = ml_index51.add(&self::ml::Big::from_i128(1));
                            {
                                ml_index51 = ml_index51_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let ml_library_inequality_1 = term != "method_definition";
                            if ml_library_inequality_1 {
                                self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(Vec::<f64>::new())
                            } else {
                                {
                                    let method_name = -1f64;
                                    {
                                        let parameters = 0f64;
                                        {
                                            let method_body = 0f64;
                                            {
                                                let ml_values52 =
                                                    self::read_deferred_declaration_children(
                                                        terms, parents, member,
                                                    );
                                                {
                                                    let ml_index52 = self::ml::Big::from_i128(0);
                                                    {
                                                        let ml_s42 = self::ml_read_declaration_deferred_binding_loop61(terms, fields, parents, texts, method_name, parameters, method_body, &(ml_values52), ml_index52);
                                                        match ml_s42 {
                                                        self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Done(method_name_2, parameters_2, method_body_2) => {
                                                            if ((method_name_2 < 0f64) || (parameters_2 != 1f64)) || (method_body_2 != 1f64) {
                                                                self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(Vec::<f64>::new())
                                                            } else {
                                                                {
                                                                    let text = self::read_required_text_fact(texts, method_name_2);
                                                                    if ((text.encode_utf16().count() as f64) == 0f64) || text.contains("\\") {
                                                                        self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(Vec::<f64>::new())
                                                                    } else {
                                                                        {
                                                                            let constructors_3 = if text == "constructor" {
                                                                                {
                                                                                    constructors + 1f64
                                                                                }
                                                                            } else {
                                                                                constructors
                                                                            };
                                                                            if constructors_3 > 1f64 {
                                                                                self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(Vec::<f64>::new())
                                                                            } else {
                                                                                {
                                                                                    let ml_index51_3 = ml_index51.add(&self::ml::Big::from_i128(1));
                                                                                    {
                                                                                        (constructors, ml_index51) = (constructors_3, ml_index51_3);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(ml_result61) => {
                                                            self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(ml_result61)
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
        } else {
            self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop59Result {
    MlReadDeclarationDeferredBindingLoop59Done(f64, f64, f64),
    MlReadDeclarationDeferredBindingLoop59Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop59(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut body: f64,
    mut classes: f64,
    ml_values50: &[f64],
    mut ml_index50: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop59Result {
    loop {
        return if ml_index50 < (self::ml::Big::from_u128(ml_values50.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values50, ml_index50.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if term == "class" {
                            {
                                let classes_2 = classes + 1f64;
                                {
                                    let ml_index50_2 = ml_index50.add(&self::ml::Big::from_i128(1));
                                    {
                                        (classes, ml_index50) = (classes_2, ml_index50_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((term == "identifier") && (field == "name")) && (name < 0f64) {
                                {
                                    let name_2 = child;
                                    {
                                        let ml_index50_3 =
                                            ml_index50.add(&self::ml::Big::from_i128(1));
                                        {
                                            (name, ml_index50) = (name_2, ml_index50_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if ((term == "class_body") && (field == "body")) && (body < 0f64) {
                                    {
                                        let body_2 = child;
                                        {
                                            let ml_index50_4 =
                                                ml_index50.add(&self::ml::Big::from_i128(1));
                                            {
                                                (body, ml_index50) = (body_2, ml_index50_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    if term == ";" {
                                        {
                                            let ml_index50_5 =
                                                ml_index50.add(&self::ml::Big::from_i128(1));
                                            {
                                                ml_index50 = ml_index50_5;
                                                continue;
                                            }
                                        }
                                    }
                                    {
                                        self::MlReadDeclarationDeferredBindingLoop59Result::MlReadDeclarationDeferredBindingLoop59Return(Vec::<f64>::new())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop59Result::MlReadDeclarationDeferredBindingLoop59Done(name, body, classes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingJoin54 {
    MlReadDeclarationDeferredBindingJoin54Next(f64),
    MlReadDeclarationDeferredBindingJoin54Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop62Result {
    MlReadDeclarationDeferredBindingLoop62Done(f64, f64),
    MlReadDeclarationDeferredBindingLoop62Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop62(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values46: &[f64],
    mut ml_index46: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop62Result {
    loop {
        return if ml_index46 < (self::ml::Big::from_u128(ml_values46.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values46, ml_index46.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index46_2 = ml_index46.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index46) = (exports_2, ml_index46_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if (term == "class_declaration") || (term == "lexical_declaration") {
                            if selected >= 0f64 {
                                self::MlReadDeclarationDeferredBindingLoop62Result::MlReadDeclarationDeferredBindingLoop62Return(Vec::<f64>::new())
                            } else {
                                {
                                    let selected_2 = child;
                                    {
                                        let ml_index46_3 =
                                            ml_index46.add(&self::ml::Big::from_i128(1));
                                        {
                                            (selected, ml_index46) = (selected_2, ml_index46_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if term == ";" {
                            {
                                let ml_index46_4 = ml_index46.add(&self::ml::Big::from_i128(1));
                                {
                                    ml_index46 = ml_index46_4;
                                    continue;
                                }
                            }
                        } else {
                            self::MlReadDeclarationDeferredBindingLoop62Result::MlReadDeclarationDeferredBindingLoop62Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop62Result::MlReadDeclarationDeferredBindingLoop62Done(selected, exports)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop53Result {
    MlReadDeclarationDeferredBindingLoop53Done,
    MlReadDeclarationDeferredBindingLoop53Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop53(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadDeclarationDeferredBindingLoop53Result {
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
                            self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Return(Vec::<f64>::new())
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        } else if (((parent >= 0f64) && ((parent % 1f64) == 0f64)) && (parent < at))
                        {
                            if (start < (self::read_required_number_fact(starts, parent)))
                                || (end > (self::read_required_number_fact(ends, parent)))
                            {
                                self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Return(Vec::<f64>::new())
                            } else {
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        at = at_3;
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptDeclarationDeferredModuleScopeLoop64Result {
    MlAcceptDeclarationDeferredModuleScopeLoop64Done,
    MlAcceptDeclarationDeferredModuleScopeLoop64Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_declaration_deferred_module_scope_loop64(
    ml_values63: &[String],
    mut ml_index63: self::ml::Big,
) -> self::MlAcceptDeclarationDeferredModuleScopeLoop64Result {
    loop {
        return if ml_index63 < (self::ml::Big::from_u128(ml_values63.len() as u128)) {
            {
                let form = self::ml_array::at(ml_values63, ml_index63.to_index());
                if ((((((form != "comment") && (form != "named function declaration"))
                    && (form != "named import"))
                    && (form != "primitive literal binding"))
                    && (form != "empty class declaration"))
                    && (form != "deferred named class declaration"))
                    && (form != "deferred function value binding")
                {
                    self::MlAcceptDeclarationDeferredModuleScopeLoop64Result::MlAcceptDeclarationDeferredModuleScopeLoop64Return(false)
                } else {
                    {
                        let ml_index63_2 = ml_index63.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index63 = ml_index63_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptDeclarationDeferredModuleScopeLoop64Result::MlAcceptDeclarationDeferredModuleScopeLoop64Done
        };
    }
}
