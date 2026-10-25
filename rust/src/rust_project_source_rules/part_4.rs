// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryExpandRustProjectSourceEventsJoin10;
use super::MlTemporaryExpandRustProjectSourceEventsLoop11Result;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::module_ancestors;
use super::number_column;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_project_module_directory;
use super::select_rust_project_module_source;
use super::source_ml_temporary_expand_rust_project_source_events_loop11;
use super::text_column;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_expand_rust_project_source_events_loop9(
    paths: &[self::ml_string::Text],
    sources: &[self::ml_string::Text],
    values: &[Vec<self::ml_string::Text>],
    starts: &[Vec<f64>],
    ends: &[Vec<f64>],
    mut depth_limit: f64,
    mut expansion_limit: f64,
    mut output_unit_limit: f64,
    plans: &[Vec<f64>],
    mut frames: Vec<f64>,
    mut directories: Vec<self::ml_string::Text>,
    mut owners: Vec<self::ml_string::Text>,
    mut events: Vec<self::ml_string::Text>,
    mut expansions: f64,
    mut output_units: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return {
            let ml_library_condition_2 = {
                let ml_value = frames.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_2 {
                {
                    let at = (frames.len() as f64) - 4f64;
                    {
                        let file = self::read_required_number_fact(&(frames), at);
                        {
                            let next = self::read_required_number_fact(&(frames), (at + 1f64));
                            {
                                let offset =
                                    self::read_required_number_fact(&(frames), (at + 2f64));
                                {
                                    let depth =
                                        self::read_required_number_fact(&(frames), (at + 3f64));
                                    {
                                        let frames_2 = self::ml_array::slice(&frames, 0f64, at);
                                        {
                                            let directory = self::read_required_text_fact(
                                                &(directories),
                                                ((directories.len() as f64) - 1f64),
                                            );
                                            {
                                                let owner = self::read_required_text_fact(
                                                    &(owners),
                                                    ((owners.len() as f64) - 1f64),
                                                );
                                                {
                                                    let directories_2 = self::ml_array::slice(
                                                        &directories,
                                                        0f64,
                                                        -1f64,
                                                    );
                                                    {
                                                        let owners_2 = self::ml_array::slice(
                                                            &owners, 0f64, -1f64,
                                                        );
                                                        {
                                                            let source =
                                                                self::read_required_text_fact(
                                                                    sources, file,
                                                                );
                                                            {
                                                                let path =
                                                                    self::read_required_text_fact(
                                                                        paths, file,
                                                                    );
                                                                {
                                                                    let plan = self::number_column(
                                                                        plans, file,
                                                                    );
                                                                    {
                                                                        let tokens =
                                                                            self::text_column(
                                                                                values, file,
                                                                            );
                                                                        {
                                                                            let first =
                                                                                self::number_column(
                                                                                    starts, file,
                                                                                );
                                                                            {
                                                                                let last = self::number_column(ends, file);
                                                                                {
                                                                                    let events_3 = if (next == 0f64) && (offset == 0f64) {
                                                                                    {
                                                                                        { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![102, 105, 108, 101]), path.clone(), self::ml_string::Text::from_units(vec![48]), {
                                                                                            let source_ml_source1_string_argument19 = source.length();
                                                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument19))
                                                                                        }, self::ml_string::Text::from_units(vec![]), owner.clone(), self::ml_string::Text::from_units(vec![])], events); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                                    }
                                                                                } else {
                                                                                    events
                                                                                };
                                                                                    if (next * 5f64)
                                                                                        >= (plan
                                                                                            .len()
                                                                                            as f64)
                                                                                    {
                                                                                        if ((output_units + source.length()) - offset) > output_unit_limit {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                    } else {
                                                                                        {
                                                                                            let events_4 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101]), path.clone(), {
                                                                                                let source_ml_source1_string_argument20 = offset;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument20))
                                                                                            }, {
                                                                                                let source_ml_source1_string_argument21 = source.length();
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument21))
                                                                                            }, {
                                                                                                let source_ml_source1_string_range_operand22 = source.clone();
                                                                                                {
                                                                                                    let source_ml_source1_string_range_operand23 = offset;
                                                                                                    {
                                                                                                        let source_ml_source1_string_range_position24 = source_ml_source1_string_range_operand23;
                                                                                                        {
                                                                                                            let source_ml_source1_string_range_position25 = f64::INFINITY;
                                                                                                            source_ml_source1_string_range_operand22.slice(source_ml_source1_string_range_position24, source_ml_source1_string_range_position25)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }, owner.clone(), self::ml_string::Text::from_units(vec![])], events_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                            {
                                                                                                let output_units_2 = output_units + (source.length() - offset);
                                                                                                {
                                                                                                    let source_ml_source1_s28 = if depth > 0f64 {
                                                                                                        if (output_units_2 + 2f64) > output_unit_limit {
                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                        } else {
                                                                                                            {
                                                                                                                let events_5 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 97, 114, 121]), path.clone(), {
                                                                                                                    let source_ml_source1_string_argument29 = source.length();
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument29))
                                                                                                                }, {
                                                                                                                    let source_ml_source1_string_argument30 = source.length();
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument30))
                                                                                                                }, self::ml_string::Text::from_units(vec![10, 125]), owner.clone(), self::ml_string::Text::from_units(vec![])], events_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                {
                                                                                                                    let output_units_3 = output_units_2 + 2f64;
                                                                                                                    self::MlTemporaryExpandRustProjectSourceEventsJoin10::MlTemporaryExpandRustProjectSourceEventsJoin10Next(events_5, output_units_3)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlTemporaryExpandRustProjectSourceEventsJoin10::MlTemporaryExpandRustProjectSourceEventsJoin10Next(events_4, output_units_2)
                                                                                                    };
                                                                                                    match source_ml_source1_s28 {
                                                                                                        self::MlTemporaryExpandRustProjectSourceEventsJoin10::MlTemporaryExpandRustProjectSourceEventsJoin10Next(events_6, output_units_4) => {
                                                                                                            {
                                                                                                                (frames, directories, owners, events, output_units) = (frames_2, directories_2, owners_2, events_6, output_units_4);
                                                                                                                continue;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    } else {
                                                                                        {
                                                                                            let keyword = self::read_required_number_fact(&(plan), (next * 5f64));
                                                                                            {
                                                                                                let name = self::read_required_text_fact(&(tokens), self::read_required_number_fact(&(plan), ((next * 5f64) + 1f64)));
                                                                                                {
                                                                                                    let delimiter = self::read_required_number_fact(&(plan), ((next * 5f64) + 2f64));
                                                                                                    {
                                                                                                        let ancestors = self::module_ancestors(&(plan), &(tokens), next);
                                                                                                        {
                                                                                                            let qualified = ((owner.concat(&self::ml_string::Text::from_units(vec![58, 58]))).concat(&{
let ml_library_condition_1 =  { let ml_value = ancestors.len() as f64; ml_value != 0f64 && !ml_value.is_nan() };
if ml_library_condition_1 {
                                                                                                            (self::ml_string::join(&ancestors, &self::ml_string::Text::from_units(vec![58, 58]))).concat(&self::ml_string::Text::from_units(vec![58, 58]))
                                                                                                        } else {
                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                        }
})).concat(&name);
                                                                                                            if ((depth + (ancestors.len() as f64)) + 1f64) > depth_limit {
                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                        } else {
                                                                                                            {
                                                                                                                let module_directory = self::ml_string::join(&{ let ml_values1 = self::ml_array::append(vec![directory.clone()], ancestors.clone()); let ml_callback1 = std::rc::Rc::new(move |part: self::ml_string::Text| -> bool { part != (self::ml_string::Text::from_units(vec![])) }); ml_values1.iter().cloned().enumerate().filter_map(|(ml_index1, ml_value1)| if ml_callback1(ml_value1.clone()) { Some(ml_value1) } else { None }).collect::<Vec<_>>() }, &self::ml_string::Text::from_units(vec![47]));
                                                                                                                {
                                                                                                                    let external = (self::read_required_text_fact(&(tokens), delimiter)) == (self::ml_string::Text::from_units(vec![59]));
                                                                                                                    {
                                                                                                                        let provider = if external {
                                                                                                                            self::select_rust_project_module_source(&(module_directory), &(name), paths)
                                                                                                                        } else {
                                                                                                                            path.clone()
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let events_7 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101]), path.clone(), {
                                                                                                                                let source_ml_source1_string_argument31 = self::read_required_number_fact(&(first), keyword);
                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument31))
                                                                                                                            }, {
                                                                                                                                let source_ml_source1_string_argument32 = self::read_required_number_fact(&(last), (self::read_required_number_fact(&(plan), ((next * 5f64) + 3f64)) - 1f64));
                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument32))
                                                                                                                            }, self::ml_string::Text::from_units(vec![]), qualified.clone(), provider.clone()], events_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                            if !external {
                                                                                                                                {
                                                                                                                                    let frames_3 = { let (ml_append_right, ml_append_left) = (vec![file, (next + 1f64), offset, depth], frames_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                    {
                                                                                                                                        let directories_3 = { let (ml_append_right, ml_append_left) = (vec![directory.clone()], directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                        {
                                                                                                                                            let owners_3 = { let (ml_append_right, ml_append_left) = (vec![owner.clone()], owners_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                            {
                                                                                                                                                (frames, directories, owners, events) = (frames_3, directories_3, owners_3, events_7);
                                                                                                                                                continue;
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
{
                                                                                                                                {
                                                                                                                                    let expansions_2 = expansions + 1f64;
                                                                                                                                    if expansions_2 > expansion_limit {
                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let beginning = self::read_required_number_fact(&(first), delimiter);
                                                                                                                                            {
                                                                                                                                                let after = self::read_required_number_fact(&(last), delimiter);
                                                                                                                                                if ((beginning < offset) || (after < beginning)) || (after > (source.length())) {
                                                                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 97, 110, 103, 101])).0, message_present: true }).raise()
                                                                                                                                                } else if (((output_units + beginning) - offset) + 2f64) > output_unit_limit {
                                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let events_8 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101]), path.clone(), {
                                                                                                                                                                let source_ml_source1_string_argument33 = offset;
                                                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument33))
                                                                                                                                                            }, {
                                                                                                                                                                let source_ml_source1_string_argument34 = beginning;
                                                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument34))
                                                                                                                                                            }, {
                                                                                                                                                                let source_ml_source1_string_range_operand35 = source.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_string_range_operand36 = offset;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_string_range_operand37 = beginning;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_string_range_position38 = source_ml_source1_string_range_operand36;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_string_range_position39 = source_ml_source1_string_range_operand37;
                                                                                                                                                                                source_ml_source1_string_range_operand35.slice(source_ml_source1_string_range_position38, source_ml_source1_string_range_position39)
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }, owner.clone(), self::ml_string::Text::from_units(vec![])], events_7); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                            {
                                                                                                                                                                let events_9 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 97, 114, 121]), path.clone(), {
                                                                                                                                                                    let source_ml_source1_string_argument40 = beginning;
                                                                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument40))
                                                                                                                                                                }, {
                                                                                                                                                                    let source_ml_source1_string_argument41 = after;
                                                                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument41))
                                                                                                                                                                }, self::ml_string::Text::from_units(vec![123, 10]), qualified.clone(), provider.clone()], events_8); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                {
                                                                                                                                                                    let output_units_5 = output_units + ((beginning - offset) + 2f64);
                                                                                                                                                                    {
                                                                                                                                                                        let child = self::ml_array::index_of(paths, &provider.clone(), 0f64, false, |a, b| a == b);
                                                                                                                                                                        if child == file {
                                                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: ((self::ml_string::Text::from_units(vec![99, 121, 99, 108, 105, 99, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 102, 105, 108, 101, 58, 32])).concat(&provider)).0, message_present: true }).raise()
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let active = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_s42 = self::source_ml_temporary_expand_rust_project_source_events_loop11(&(frames_2), &(provider), child, active);
                                                                                                                                                                                    match source_ml_source1_s42 {
                                                                                                                                                                                        self::MlTemporaryExpandRustProjectSourceEventsLoop11Result::MlTemporaryExpandRustProjectSourceEventsLoop11Done => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let frames_4 = { let (ml_append_right, ml_append_left) = (vec![file, (next + 1f64), after, depth], frames_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                {
                                                                                                                                                                                                    let directories_4 = { let (ml_append_right, ml_append_left) = (vec![directory.clone()], directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let owners_4 = { let (ml_append_right, ml_append_left) = (vec![owner.clone()], owners_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let frames_5 = { let (ml_append_right, ml_append_left) = (vec![child, 0f64, 0f64, ((depth + (ancestors.len() as f64)) + 1f64)], frames_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let directories_5 = { let (ml_append_right, ml_append_left) = (vec![self::read_rust_project_module_directory(&(provider), false)], directories_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let owners_5 = { let (ml_append_right, ml_append_left) = (vec![qualified.clone()], owners_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        (frames, directories, owners, events, expansions, output_units) = (frames_5, directories_5, owners_5, events_9, expansions_2, output_units_5);
                                                                                                                                                                                                                        continue;
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
                    }
                }
            } else {
                events
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceProjectCommandPlanLoop12Result {
    MlTemporaryReadRustSourceProjectCommandPlanLoop12Done(
        self::ml_string::Text,
        self::ml_string::Text,
        bool,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceProjectCommandPlanJoin13 {
    MlTemporaryReadRustSourceProjectCommandPlanJoin13Next(
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_project_command_plan_loop12(
    args: &[self::ml_string::Text],
    mut entry: self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut binary: bool,
    mut library: bool,
    mut position: f64,
) -> self::MlTemporaryReadRustSourceProjectCommandPlanLoop12Result {
    loop {
        return if position < (args.len() as f64) {
            {
                let option = self::read_required_text_fact(args, position);
                if option
                    == (self::ml_string::Text::from_units(vec![45, 45, 98, 105, 110, 97, 114, 121]))
                {
                    if binary {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 45, 45, 98, 105,
                                110, 97, 114, 121,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    } else {
                        {
                            let binary_2 = true;
                            {
                                let position_2 = position + 1f64;
                                {
                                    (binary, position) = (binary_2, position_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else if option
                    == (self::ml_string::Text::from_units(vec![
                        45, 45, 108, 105, 98, 114, 97, 114, 121,
                    ]))
                {
                    if library {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 45, 45, 108, 105,
                                98, 114, 97, 114, 121,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    } else {
                        {
                            let library_2 = true;
                            {
                                let position_3 = position + 1f64;
                                {
                                    (library, position) = (library_2, position_3);
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((option
                    != (self::ml_string::Text::from_units(vec![45, 45, 101, 110, 116, 114, 121])))
                    && (option != (self::ml_string::Text::from_units(vec![45, 45, 116, 111]))))
                    || ((position + 1f64) >= (args.len() as f64))
                {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: ((self::ml_string::Text::from_units(vec![
                            105, 110, 118, 97, 108, 105, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                            112, 114, 111, 106, 101, 99, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                            116, 58, 32,
                        ]))
                        .concat(&option))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let value = self::read_required_text_fact(args, (position + 1f64));
                        if !(!value.0.is_empty())
                            || value.starts_with(&self::ml_string::Text::from_units(vec![45, 45]))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: ((self::ml_string::Text::from_units(vec![
                                    109, 105, 115, 115, 105, 110, 103, 32, 115, 111, 117, 114, 99,
                                    101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 111, 112, 116,
                                    105, 111, 110, 32, 118, 97, 108, 117, 101, 58, 32,
                                ]))
                                .concat(&option))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let source_ml_source1_s43 = if option
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 101, 110, 116, 114, 121,
                                    ])) {
                                    {
                                        let ml_library_inequality_2 =
                                            entry != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_2 {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        100, 117, 112, 108, 105, 99, 97, 116, 101,
                                                        32, 45, 45, 101, 110, 116, 114, 121,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        } else {
                                            {
                                                let entry_2 = value.clone();
                                                self::MlTemporaryReadRustSourceProjectCommandPlanJoin13::MlTemporaryReadRustSourceProjectCommandPlanJoin13Next(entry_2, target.clone())
                                            }
                                        }
                                    }
                                } else {
                                    let ml_library_inequality_1 =
                                        target != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    100, 117, 112, 108, 105, 99, 97, 116, 101, 32,
                                                    45, 45, 116, 111,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    } else {
                                        {
                                            let target_2 = value.clone();
                                            self::MlTemporaryReadRustSourceProjectCommandPlanJoin13::MlTemporaryReadRustSourceProjectCommandPlanJoin13Next(entry.clone(), target_2)
                                        }
                                    }
                                };
                                match source_ml_source1_s43 {
                                            self::MlTemporaryReadRustSourceProjectCommandPlanJoin13::MlTemporaryReadRustSourceProjectCommandPlanJoin13Next(entry_3, target_3) => {
                                                {
                                                    let position_4 = position + 1f64;
                                                    {
                                                        let position_5 = position_4 + 1f64;
                                                        {
                                                            (entry, target, position) = (entry_3.clone(), target_3.clone(), position_5);
                                                            continue;
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
            self::MlTemporaryReadRustSourceProjectCommandPlanLoop12Result::MlTemporaryReadRustSourceProjectCommandPlanLoop12Done(entry.clone(), target.clone(), binary, library)
        };
    }
}
