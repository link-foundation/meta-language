// Generated from complete js/src/translation/disjoint-collection-projection-rules.js. Do not edit by hand.
use super::add_fact;
use super::add_field;
use super::add_value;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_required_number_fact;
use super::render_array_callback_argument;
use super::text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_callback_call_loop9(
    operation: &self::ml_string::Text,
    mut parameter_count: f64,
    suffix: &self::ml_string::Text,
    mut clone_element: bool,
    mut arguments_text: self::ml_string::Text,
    mut position: f64,
    mut maximum: f64,
) -> self::ml_string::Text {
    loop {
        if (position < parameter_count) && (position < maximum) {
            {
                let arguments_text_3 = if position > 0f64 {
                    { arguments_text.concat(&self::ml_string::Text::from_units(vec![44, 32])) }
                } else {
                    arguments_text
                };
                {
                    let arguments_text_4 =
                        arguments_text_3.concat(&self::render_array_callback_argument(
                            operation,
                            position,
                            suffix,
                            clone_element,
                        ));
                    {
                        let position_2 = position + 1f64;
                        {
                            (arguments_text, position) = (arguments_text_4, position_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { arguments_text };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_state_method_suffix_loop10(
    text_2: &self::ml_string::Text,
    mut suffix: f64,
) -> f64 {
    loop {
        if text_2.contains(
            &(self::ml_string::Text::from_units(vec![
                109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )),
        ) || text_2.contains(
            &((self::ml_string::Text::from_units(vec![
                109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109, 101, 110, 116,
            ]))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(suffix),
            )))
            .concat(&self::ml_string::Text::from_units(vec![95])),
        ) {
            {
                let suffix_2 = suffix + 1f64;
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        }
        return { suffix };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop11(
    arguments_: &[self::ml_string::Text],
    types: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut output: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let output_2 = output.concat(
                    &((((((((self::ml_string::Text::from_units(vec![
                        108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103,
                        117, 109, 101, 110, 116,
                    ]))
                    .concat(suffix))
                    .concat(&self::ml_string::Text::from_units(vec![95])))
                    .concat(&self::ml_string::Text::from_text(
                        &self::ml_number::js_number(index),
                    )))
                    .concat(&self::ml_string::Text::from_units(vec![58, 32])))
                    .concat(
                        &(self::ml_array::optional_at(types, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                    .concat(
                        &(self::ml_array::optional_at(
                            arguments_,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                    ))
                    .concat(&self::ml_string::Text::from_units(vec![59, 32])),
                );
                {
                    let index_2 = index + 1f64;
                    {
                        (output, index) = (output_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { output.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop12(
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (((self::ml_string::Text::from_units(vec![
                                109, 108, 95, 97, 114, 114, 97, 121, 95, 97, 114, 103, 117, 109,
                                101, 110, 116,
                            ]))
                            .concat(suffix))
                            .concat(&self::ml_string::Text::from_units(vec![95])))
                            .concat(&self::ml_string::Text::from_text(
                                &self::ml_number::js_number(index),
                            )),
                        ],
                        names,
                    );
                    self::ml_array::append(&(ml_append_left), ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_state_method_expression_loop13(
    arguments_: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut inserted: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (arguments_.len() as f64) {
            {
                let inserted_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                names,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                        ],
                        inserted,
                    );
                    self::ml_array::append(&(ml_append_left), ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (inserted, index) = (inserted_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { inserted };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_roots_loop15(
    mut tag: f64,
    mut node: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values14: &[f64],
    mut source_ml_source1_temporary_index14: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index14
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values14.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values14,
                    source_ml_source1_temporary_index14.to_index(),
                );
                if ((fact / span).floor()) == tag {
                    {
                        let value = fact % span;
                        if ((value / width).floor()) == node {
                            {
                                let result_2 = self::add_value(&(result), ((value % width) - 1f64));
                                {
                                    let source_ml_source1_temporary_index14_2 =
                                        source_ml_source1_temporary_index14
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index14) =
                                            (result_2, source_ml_source1_temporary_index14_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_source1_temporary_index14_3 =
                                    source_ml_source1_temporary_index14
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index14 =
                                        source_ml_source1_temporary_index14_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index14_4 =
                            source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index14 =
                                source_ml_source1_temporary_index14_4;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_add_roots_loop18(
    mut present: Vec<f64>,
    source_ml_source1_temporary_values16: &[f64],
    mut source_ml_source1_temporary_index16: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index16
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values16.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values16,
                    source_ml_source1_temporary_index16.to_index(),
                );
                {
                    let present_2 = self::add_value(&(present), root);
                    {
                        let source_ml_source1_temporary_index16_2 =
                            source_ml_source1_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            (present, source_ml_source1_temporary_index16) =
                                (present_2, source_ml_source1_temporary_index16_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { present };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_add_roots_loop19(
    mut tag: f64,
    mut node: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values17: &[f64],
    mut source_ml_source1_temporary_index17: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index17
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values17.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values17,
                    source_ml_source1_temporary_index17.to_index(),
                );
                {
                    let result_2 = self::add_fact(
                        &(result),
                        ((((tag * span) + (node * width)) + root) + 1f64),
                    );
                    {
                        let source_ml_source1_temporary_index17_2 =
                            source_ml_source1_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index17) =
                                (result_2, source_ml_source1_temporary_index17_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_edges_loop20(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    label: &self::ml_string::Text,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            if (self::text_fact(labels, at)) == (*label) {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value51 = result;
                        {
                            let source_ml_source2_concat_value52 =
                                vec![self::read_required_number_fact(targets, at)];
                            self::ml_array::append(
                                &(self::ml_array::append(
                                    &(Vec::<f64>::new()),
                                    source_ml_source2_concat_value51,
                                )),
                                source_ml_source2_concat_value52,
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_field_loop24(
    mut field_2: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut root: f64,
    source_ml_source1_temporary_values22: &[f64],
    mut source_ml_source1_temporary_index22: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index22
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values22.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values22,
                    source_ml_source1_temporary_index22.to_index(),
                );
                if ((fact / span).floor()) == 2f64 {
                    {
                        let value = fact % span;
                        {
                            let owner = (value / (field_width * width)).floor();
                            {
                                let actual = (value / width).floor() % field_width;
                                if (owner == root) && ((actual == 0f64) || (actual == field_2)) {
                                    {
                                        let result_2 =
                                            self::add_value(&(result), ((value % width) - 1f64));
                                        {
                                            let source_ml_source1_temporary_index22_2 =
                                                source_ml_source1_temporary_index22
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (result, source_ml_source1_temporary_index22) = (
                                                    result_2,
                                                    source_ml_source1_temporary_index22_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let source_ml_source1_temporary_index22_3 =
                                            source_ml_source1_temporary_index22
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index22 =
                                                source_ml_source1_temporary_index22_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index22_4 =
                            source_ml_source1_temporary_index22.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index22 =
                                source_ml_source1_temporary_index22_4;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_field_loop23(
    facts: &[f64],
    mut field_2: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values21: &[f64],
    mut source_ml_source1_temporary_index21: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index21
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values21.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values21,
                    source_ml_source1_temporary_index21.to_index(),
                );
                if root < 0f64 {
                    {
                        let result_2 = self::add_value(&(result), -1f64);
                        {
                            let source_ml_source1_temporary_index21_2 =
                                source_ml_source1_temporary_index21
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index21) =
                                    (result_2, source_ml_source1_temporary_index21_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_values22 = facts.to_vec();
                        {
                            let source_ml_source1_temporary_index22 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_read_field_loop24(
                                    field_2,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    root,
                                    &(source_ml_source1_temporary_values22),
                                    source_ml_source1_temporary_index22,
                                );
                                {
                                    let source_ml_source1_temporary_index21_3 =
                                        source_ml_source1_temporary_index21
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index21) =
                                            (result_3, source_ml_source1_temporary_index21_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_add_field_loop28(
    mut field_2: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut owner: f64,
    source_ml_source1_temporary_values26: &[f64],
    mut source_ml_source1_temporary_index26: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index26
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values26.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values26,
                    source_ml_source1_temporary_index26.to_index(),
                );
                {
                    let result_2 = self::add_fact(
                        &(result),
                        ((((2f64 * span) + (((owner * field_width) + field_2) * width)) + root)
                            + 1f64),
                    );
                    {
                        let source_ml_source1_temporary_index26_2 =
                            source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                        {
                            (result, source_ml_source1_temporary_index26) =
                                (result_2, source_ml_source1_temporary_index26_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_add_field_loop27(
    mut field_2: f64,
    roots: &[f64],
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values25: &[f64],
    mut source_ml_source1_temporary_index25: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index25
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values25.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    source_ml_source1_temporary_values25,
                    source_ml_source1_temporary_index25.to_index(),
                );
                if owner >= 0f64 {
                    {
                        let source_ml_source1_temporary_values26 = roots.to_vec();
                        {
                            let source_ml_source1_temporary_index26 = self::ml::Big::from_i128(0);
                            {
                                let result_2 = self::source_ml_source1_temporary_add_field_loop28(
                                    field_2,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    owner,
                                    &(source_ml_source1_temporary_values26),
                                    source_ml_source1_temporary_index26,
                                );
                                {
                                    let source_ml_source1_temporary_index25_2 =
                                        source_ml_source1_temporary_index25
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index25) =
                                            (result_2, source_ml_source1_temporary_index25_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index25_3 =
                            source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index25 =
                                source_ml_source1_temporary_index25_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_copy_fields_loop32(
    mut destination: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    mut root: f64,
    source_ml_source1_temporary_values30: &[f64],
    mut source_ml_source1_temporary_index30: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index30
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values30.len() as u128))
        {
            {
                let fact = self::ml_array::at(
                    source_ml_source1_temporary_values30,
                    source_ml_source1_temporary_index30.to_index(),
                );
                if ((fact / span).floor()) == 2f64 {
                    {
                        let value = fact % span;
                        if ((value / (field_width * width)).floor()) == root {
                            {
                                let result_2 = self::add_fact(
                                    &(result),
                                    (((2f64 * span) + ((destination * field_width) * width))
                                        + (value % (field_width * width))),
                                );
                                {
                                    let source_ml_source1_temporary_index30_2 =
                                        source_ml_source1_temporary_index30
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index30) =
                                            (result_2, source_ml_source1_temporary_index30_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let source_ml_source1_temporary_index30_3 =
                                    source_ml_source1_temporary_index30
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index30 =
                                        source_ml_source1_temporary_index30_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index30_4 =
                            source_ml_source1_temporary_index30.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index30 =
                                source_ml_source1_temporary_index30_4;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_copy_fields_loop31(
    facts: &[f64],
    mut destination: f64,
    mut field_width: f64,
    mut width: f64,
    mut span: f64,
    mut result: Vec<f64>,
    source_ml_source1_temporary_values29: &[f64],
    mut source_ml_source1_temporary_index29: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_source1_temporary_index29
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values29.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values29,
                    source_ml_source1_temporary_index29.to_index(),
                );
                if root < 0f64 {
                    {
                        let result_2 = self::add_field(
                            &(result),
                            &(vec![destination]),
                            0f64,
                            &(vec![-1f64]),
                            field_width,
                            width,
                            span,
                        );
                        {
                            let source_ml_source1_temporary_index29_2 =
                                source_ml_source1_temporary_index29
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index29) =
                                    (result_2, source_ml_source1_temporary_index29_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_values30 = facts.to_vec();
                        {
                            let source_ml_source1_temporary_index30 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_source1_temporary_copy_fields_loop32(
                                    destination,
                                    field_width,
                                    width,
                                    span,
                                    result,
                                    root,
                                    &(source_ml_source1_temporary_values30),
                                    source_ml_source1_temporary_index30,
                                );
                                {
                                    let source_ml_source1_temporary_index29_3 =
                                        source_ml_source1_temporary_index29
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        (result, source_ml_source1_temporary_index29) =
                                            (result_3, source_ml_source1_temporary_index29_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop36Result {
    MlSource1TemporaryReachableLoop36Done(Vec<f64>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReachableLoop37Result {
    MlSource1TemporaryReachableLoop37Done(Vec<f64>, bool),
}
