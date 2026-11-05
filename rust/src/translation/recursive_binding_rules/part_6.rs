// Generated from complete js/src/translation/recursive-binding-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27;
use super::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result;
use super::accept_java_script_source_binding_name;
use super::contains_java_script_recursive_object_binding;
use super::find_java_script_pattern_default_expression_end;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_recursive_object_bindings;
use super::read_java_script_object_binding_tree;
use super::read_java_script_recursive_object_binding_positions;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_recursive_object_formals_loop28(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    positions: &[f64],
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (positions.len() as f64) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value158 = names;
                    {
                        let source_ml_source2_concat_value159 =
                            vec![self::read_required_text_fact(
                                values,
                                self::read_required_number_fact(positions, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value158,
                            ),
                            source_ml_source2_concat_value159,
                        )
                    }
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_recursive_object_formals_loop26(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut maximum_depth: f64,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![41]))))
        {
            {
                let source_ml_source2_s167 = if ((self::read_required_text_fact(kinds, at))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && ((self::read_required_text_fact(values, at))
                        == (self::ml_string::Text::from_units(vec![123])))
                {
                    {
                        let tree = self::plan_java_script_recursive_object_bindings(
                            kinds,
                            values,
                            at,
                            reserved,
                            &(names),
                            maximum_depth,
                        );
                        if (self::read_required_number_fact(&(tree), 0f64)) < 0f64 {
                            self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Return(tree)
                        } else {
                            {
                                let positions =
                                    self::read_java_script_recursive_object_binding_positions(
                                        &(tree),
                                    );
                                {
                                    let index = 0f64;
                                    {
                                        let names_2 = self::source_ml_source1_temporary_plan_java_script_recursive_object_formals_loop28(values, names, &(positions), index);
                                        {
                                            let instructions_2 = {
                                                let source_ml_source2_concat_value168 =
                                                    instructions;
                                                {
                                                    let source_ml_source2_concat_value169 =
                                                        vec![1f64, slot, (tree.len() as f64)];
                                                    {
                                                        let source_ml_source2_concat_value170 =
                                                            tree.clone();
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value168), source_ml_source2_concat_value169), source_ml_source2_concat_value170)
                                                    }
                                                }
                                            };
                                            {
                                                let at_2 =
                                                    self::read_required_number_fact(&(tree), 0f64);
                                                self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Next(at_2, names_2, instructions_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if ((self::read_required_text_fact(kinds, at))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && ((self::read_required_text_fact(values, at))
                        == (self::ml_string::Text::from_units(vec![91])))
                {
                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Return(vec![-3f64, at])
                } else if (self::accept_java_script_source_binding_name(
                    &(self::read_required_text_fact(kinds, at)),
                    &(self::read_required_text_fact(values, at)),
                    self::ml_array::includes(
                        &names,
                        &self::read_required_text_fact(values, at),
                        0f64,
                        false,
                        |a, b| a == b,
                    ),
                    reserved,
                )) {
                    {
                        let names_3 = {
                            let source_ml_source2_concat_value171 = names;
                            {
                                let source_ml_source2_concat_value172 =
                                    vec![self::read_required_text_fact(values, at)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value171,
                                    ),
                                    source_ml_source2_concat_value172,
                                )
                            }
                        };
                        {
                            let instructions_3 = {
                                let source_ml_source2_concat_value173 = instructions;
                                {
                                    let source_ml_source2_concat_value174 = vec![0f64, slot, at];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value173,
                                        ),
                                        source_ml_source2_concat_value174,
                                    )
                                }
                            };
                            {
                                let at_3 = at + 1f64;
                                self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Next(at_3, names_3, instructions_3)
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Return(vec![-8f64, at])
                };
                match source_ml_source2_s167 {
                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Next(at_4, names_4, instructions_4) => {
                        if at_4 >= (kinds.len() as f64) {
                            self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(vec![-1f64, at_4])
                        } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![61]))) {
                                self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(vec![-2f64, at_4])
                            } else {
                                {
                                    let slot_2 = slot + 1f64;
                                    if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![44]))) {
                                        {
                                            let at_5 = at_4 + 1f64;
                                            {
                                                (at, slot, names, instructions) = (at_5, slot_2, names_4, instructions_4);
                                                continue;
                                            }
                                        }
                                    }
{
                                        if (((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![41])))) {
                                            {
                                                (at, slot, names, instructions) = (at_4, slot_2, names_4, instructions_4);
                                                continue;
                                            }
                                        }
{
                                            self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(vec![-1f64, at_4])
                                        }
                                    }
                                }
                            }
                    }
                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsJoin27Return(source_ml_source1_temporary_result27) => {
                        self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(source_ml_source1_temporary_result27)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Done(at, slot, instructions)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_object_formal_binding_positions_loop29(
    plan: &[f64],
    mut at: f64,
    mut result: Vec<f64>,
) -> Vec<f64> {
    loop {
        if at < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, at);
                if form == 0f64 {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value175 = result;
                            {
                                let source_ml_source2_concat_value176 =
                                    vec![self::read_required_number_fact(plan, (at + 2f64))];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value175,
                                    ),
                                    source_ml_source2_concat_value176,
                                )
                            }
                        };
                        {
                            let at_2 = at + 3f64;
                            {
                                (at, result) = (at_2, result_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let count = self::read_required_number_fact(plan, (at + 2f64));
                        {
                            let tree =
                                self::ml_array::slice(plan, (at + 3f64), ((at + 3f64) + count));
                            {
                                let result_3 = {
                                    let source_ml_source2_concat_value177 = result;
                                    {
                                        let source_ml_source2_concat_value178 = self::read_java_script_recursive_object_binding_positions(&(tree));
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value177,
                                            ),
                                            source_ml_source2_concat_value178,
                                        )
                                    }
                                };
                                {
                                    let at_3 = at + (3f64 + count);
                                    {
                                        (at, result) = (at_3, result_3);
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result {
    MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Done,
    MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_recursive_object_formals_loop30(
    plan: &[f64],
    mut at: f64,
) -> self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result {
    loop {
        return if at < (plan.len() as f64) {
            if (self::read_required_number_fact(plan, at)) == 0f64 {
                {
                    let at_2 = at + 3f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let count = self::read_required_number_fact(plan, (at + 2f64));
                    if self::contains_java_script_recursive_object_binding(
                        &(self::ml_array::slice(plan, (at + 3f64), ((at + 3f64) + count))),
                    ) {
                        self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Return(true)
                    } else {
                        {
                            let at_3 = at + (3f64 + count);
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result {
    MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Done(f64, Vec<f64>),
    MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_binding_tree_loop35(
    mut rows: Vec<f64>,
    children: &[f64],
    mut owner: f64,
    mut count: f64,
    mut base: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let offset = 2f64 + (index * 8f64);
                {
                    let old = self::read_required_number_fact(children, (offset + 1f64));
                    {
                        let parent = if old == 0f64 {
                            owner
                        } else {
                            (base + old) - 1f64
                        };
                        {
                            let rows_2 = {
                                let source_ml_source2_concat_value179 = rows;
                                {
                                    let source_ml_source2_concat_value180 = vec![
                                        self::read_required_number_fact(children, offset),
                                        parent,
                                    ];
                                    {
                                        let source_ml_source2_concat_value181 =
                                            self::ml_array::slice(
                                                children,
                                                (offset + 2f64),
                                                (offset + 8f64),
                                            );
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value179,
                                                ),
                                                source_ml_source2_concat_value180,
                                            ),
                                            source_ml_source2_concat_value181,
                                        )
                                    }
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (rows, index) = (rows_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { rows };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34 {
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33 {
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Next(f64, f64, f64, f64, Vec<f64>),
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32 {
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Next(f64, bool),
    MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_binding_tree_loop31(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut depth: f64,
    mut maximum_depth: f64,
    mut at: f64,
    mut rows: Vec<f64>,
) -> self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![93]))))
        {
            {
                let token = at;
                if ((self::read_required_text_fact(kinds, at))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && ((self::read_required_text_fact(values, at))
                        == (self::ml_string::Text::from_units(vec![44])))
                {
                    {
                        let rows_2 = {
                            let source_ml_source2_concat_value182 = rows;
                            {
                                let source_ml_source2_concat_value183 =
                                    vec![5f64, 0f64, at, -1f64, -1f64, -1f64, -1f64, -1f64];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value182,
                                    ),
                                    source_ml_source2_concat_value183,
                                )
                            }
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (at, rows) = (at_2, rows_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let rest = false;
                        {
                            let source_ml_source2_s184 = if ((self::read_required_text_fact(
                                kinds, at,
                            ))
                                == (self::ml_string::Text::from_units(vec![
                                    112, 117, 110, 99, 116,
                                ])))
                                && ((self::read_required_text_fact(values, at))
                                    == (self::ml_string::Text::from_units(vec![46, 46, 46])))
                            {
                                {
                                    let rest_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        if at_3 >= (kinds.len() as f64) {
                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Return(vec![-1f64, at_3])
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Next(at_3, rest_2)
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Next(at, rest)
                            };
                            match source_ml_source2_s184 {
                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Next(at_4, rest_3) => {
                                    {
                                        let binding = at_4;
                                        {
                                            let open = -1f64;
                                            {
                                                let close = -1f64;
                                                {
                                                    let children = Vec::<f64>::new();
                                                    {
                                                        let source_ml_source2_s185 = if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![123]))) || ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![91])))) {
                                                            {
                                                                let open_2 = at_4;
                                                                {
                                                                    let children_2 = self::read_java_script_object_binding_tree(kinds, values, at_4, reserved, (depth + 1f64), maximum_depth, true);
                                                                    if (self::read_required_number_fact(&(children_2), 0f64)) < 0f64 {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Return(children_2)
                                                                    } else {
                                                                        {
                                                                            let at_5 = self::read_required_number_fact(&(children_2), 0f64);
                                                                            {
                                                                                let close_2 = at_5 - 1f64;
                                                                                {
                                                                                    let binding_2 = -1f64;
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Next(at_5, binding_2, open_2, close_2, children_2)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else if (self::accept_java_script_source_binding_name(&(self::read_required_text_fact(kinds, at_4)), &(self::read_required_text_fact(values, at_4)), false, reserved)) {
                                                                {
                                                                    let at_6 = at_4 + 1f64;
                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Next(at_6, binding, open, close, children)
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Return(vec![-8f64, at_4])
                                                            };
                                                        match source_ml_source2_s185 {
                                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Next(at_7, binding_3, open_3, close_3, children_3) => {
                                                                if at_7 >= (kinds.len() as f64) {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(vec![-1f64, at_7])
                                                                } else {
                                                                    {
                                                                        let first = -1f64;
                                                                        {
                                                                            let last = -1f64;
                                                                            {
                                                                                let source_ml_source2_s186 = if ((self::read_required_text_fact(kinds, at_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                    if rest_3 {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Return(vec![-6f64, at_7])
                                                                                    } else {
                                                                                        {
                                                                                            let first_2 = at_7 + 1f64;
                                                                                            {
                                                                                                let last_2 = self::find_java_script_pattern_default_expression_end(kinds, values, first_2, &(self::ml_string::Text::from_units(vec![93])));
                                                                                                if last_2 < 0f64 {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Return(vec![-1f64, first_2])
                                                                                                } else {
                                                                                                    {
                                                                                                        let at_8 = last_2;
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Next(at_8, first_2, last_2)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Next(at_7, first, last)
                                                                                };
                                                                                match source_ml_source2_s186 {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Next(at_9, first_3, last_3) => {
                                                                                        {
                                                                                            let owner = (rows.len() as f64) / 8f64;
                                                                                            {
                                                                                                let rows_3 = {
                                                                                                    let source_ml_source2_concat_value187 = rows;
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value188 = vec![if rest_3 {
                                                                                                            4f64
                                                                                                        } else {
                                                                                                            3f64
                                                                                                        }, 0f64, token, binding_3, first_3, last_3, open_3, close_3];
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value187), source_ml_source2_concat_value188)
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let rows_5 = {
let ml_library_condition_1 =  { let ml_value = children_3.len() as f64; ml_value != 0f64 && !ml_value.is_nan() };
if ml_library_condition_1 {
                                                                                                        {
                                                                                                            let count = self::read_required_number_fact(&(children_3), 1f64);
                                                                                                            {
                                                                                                                let base = (rows_3.len() as f64) / 8f64;
                                                                                                                {
                                                                                                                    let index = 1f64;
                                                                                                                    {
                                                                                                                        self::source_ml_source1_temporary_read_java_script_array_binding_tree_loop35(rows_3, &(children_3), owner, count, base, index)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        rows_3
                                                                                                    }
};
                                                                                                    if rest_3 {
                                                                                                        if ((self::read_required_text_fact(kinds, at_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) || ((self::read_required_text_fact(values, at_9)) != (self::ml_string::Text::from_units(vec![93]))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(vec![-6f64, at_9])
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Done(at_9, rows_5)
                                                                                                        }
                                                                                                    } else {
                                                                                                        if ((self::read_required_text_fact(kinds, at_9)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_9)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                                            {
                                                                                                                let at_10 = at_9 + 1f64;
                                                                                                                {
                                                                                                                    (at, rows) = (at_10, rows_5);
                                                                                                                    continue;
                                                                                                                }
                                                                                                            }
                                                                                                        }
{
                                                                                                            if (((self::read_required_text_fact(kinds, at_9)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_9)) == (self::ml_string::Text::from_units(vec![93])))) {
                                                                                                                {
                                                                                                                    (at, rows) = (at_9, rows_5);
                                                                                                                    continue;
                                                                                                                }
                                                                                                            }
{
                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(vec![-1f64, at_9])
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin34Return(source_ml_source1_temporary_result35) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(source_ml_source1_temporary_result35)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin33Return(source_ml_source1_temporary_result35_2) => {
                                                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(source_ml_source1_temporary_result35_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32::MlSource1TemporaryReadJavaScriptArrayBindingTreeJoin32Return(source_ml_source1_temporary_result35_3) => {
                                    self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(source_ml_source1_temporary_result35_3)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Done(at, rows)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result {
    MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Done,
    MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_recursive_bindings_loop36(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    positions: &[f64],
    mut names: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result {
    loop {
        return if index < (positions.len() as f64) {
            {
                let at = self::read_required_number_fact(positions, index);
                {
                    let name = self::read_required_text_fact(values, at);
                    if (self::accept_java_script_source_binding_name(
                        &(self::read_required_text_fact(kinds, at)),
                        &(name),
                        self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| a == b),
                        reserved,
                    )) {
                        {
                            let names_2 = {
                                let source_ml_source2_concat_value189 = names;
                                {
                                    let source_ml_source2_concat_value190 = vec![name.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source2_concat_value189,
                                        ),
                                        source_ml_source2_concat_value190,
                                    )
                                }
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
                    {
                        self::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Return(vec![-8f64, at])
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_array_element_index_loop37(
    plan: &[f64],
    mut row: f64,
    mut owner: f64,
    mut position: f64,
    mut index: f64,
) -> f64 {
    loop {
        if index < row {
            {
                let at = 2f64 + (index * 8f64);
                {
                    let role = self::read_required_number_fact(plan, at);
                    if ((self::read_required_number_fact(plan, (at + 1f64))) == owner)
                        && (((role == 3f64) || (role == 4f64)) || (role == 5f64))
                    {
                        {
                            let position_2 = position + 1f64;
                            {
                                let index_2 = index + 1f64;
                                {
                                    (position, index) = (position_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let index_3 = index + 1f64;
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { position };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result {
    MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Done,
    MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_advanced_array_binding_loop38(
    plan: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result {
    loop {
        return if index < (self::read_required_number_fact(plan, 1f64)) {
            {
                let at = 2f64 + (index * 8f64);
                {
                    let role = self::read_required_number_fact(plan, at);
                    if (((((role == 1f64) || (role == 2f64)) || (role == 4f64)) || (role == 5f64))
                        || ((self::read_required_number_fact(plan, (at + 4f64))) >= 0f64))
                        || ((self::read_required_number_fact(plan, (at + 6f64))) >= 0f64)
                    {
                        self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Return(true)
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result {
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40 {
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop41(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    positions: &[f64],
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (positions.len() as f64) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value191 = names;
                    {
                        let source_ml_source2_concat_value192 =
                            vec![self::read_required_text_fact(
                                values,
                                self::read_required_number_fact(positions, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value191,
                            ),
                            source_ml_source2_concat_value192,
                        )
                    }
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
