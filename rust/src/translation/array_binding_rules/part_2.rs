// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::ml_array;
use super::ml_string;
use super::read_array_binding_initializer_end;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_binding_names(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> Vec<f64> {
    if (((start < 0f64) || ((start % 1f64) != 0f64)) || (start >= (kinds.len() as f64)))
        || (start >= (values.len() as f64))
    {
        vec![-1f64]
    } else if ((self::read_required_text_fact(kinds, start))
        != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
        || ((self::read_required_text_fact(values, start))
            != (self::ml_string::Text::from_units(vec![91])))
    {
        vec![-1f64]
    } else {
        {
            let index = start + 1f64;
            {
                let positions = Vec::<f64>::new();
                {
                    let ml_s4 = self::ml_plan_java_script_array_binding_names_loop5(
                        kinds, values, index, positions,
                    );
                    match ml_s4 {
                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Done => {
                                vec![-1f64]
                            }
                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result5) => {
                                ml_result5
                            }
                        }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptJavaScriptArrayBindingNameLoop1Result {
    MlAcceptJavaScriptArrayBindingNameLoop1Done,
    MlAcceptJavaScriptArrayBindingNameLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_java_script_array_binding_name_loop1(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlAcceptJavaScriptArrayBindingNameLoop1Result {
    loop {
        return if index < (reserved.len() as f64) {
            if (self::read_required_text_fact(reserved, index)) == (*name) {
                self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingFactFieldLoop2Result {
    MlReadArrayBindingFactFieldLoop2Done,
    MlReadArrayBindingFactFieldLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_array_binding_fact_field_loop2(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut at: f64,
    mut end: f64,
) -> self::MlReadArrayBindingFactFieldLoop2Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == (*field) {
                self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Return(
                    self::read_required_number_fact(targets, at),
                )
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingInitializerEndLoop3Result {
    MlReadArrayBindingInitializerEndLoop3Done,
    MlReadArrayBindingInitializerEndLoop3Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingInitializerEndJoin4 {
    MlReadArrayBindingInitializerEndJoin4Next(Vec<self::ml_string::Text>),
    MlReadArrayBindingInitializerEndJoin4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_array_binding_initializer_end_loop3(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut index: f64,
    mut closes: Vec<self::ml_string::Text>,
) -> self::MlReadArrayBindingInitializerEndLoop3Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(-1f64)
                    } else {
                        {
                            let ml_s5 = if kind
                                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                            {
                                if ((closes.len() as f64) == 0f64)
                                    && ((value == (self::ml_string::Text::from_units(vec![44])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                {
                                    self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(if index > start {
                                        index
                                    } else {
                                        -1f64
                                    })
                                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let closes_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![if value
                                                    == (self::ml_string::Text::from_units(vec![40]))
                                                {
                                                    self::ml_string::Text::from_units(vec![41])
                                                } else if value
                                                    == (self::ml_string::Text::from_units(vec![91]))
                                                {
                                                    self::ml_string::Text::from_units(vec![93])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![125])
                                                }],
                                                closes,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_2)
                                    }
                                } else if ((value == (self::ml_string::Text::from_units(vec![41])))
                                    || (value == (self::ml_string::Text::from_units(vec![93]))))
                                    || (value == (self::ml_string::Text::from_units(vec![125])))
                                {
                                    if ((closes.len() as f64) == 0f64)
                                        || ((self::read_required_text_fact(
                                            &(closes),
                                            ((closes.len() as f64) - 1f64),
                                        )) != value)
                                    {
                                        self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(-1f64)
                                    } else {
                                        {
                                            let closes_3 = self::ml_array::slice(
                                                &closes,
                                                0f64,
                                                ((closes.len() as f64) - 1f64),
                                            );
                                            self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_3)
                                        }
                                    }
                                } else {
                                    self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes)
                                }
                            } else {
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes)
                            };
                            match ml_s5 {
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_4) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, closes) = (index_2, closes_4);
                                            continue;
                                        }
                                    }
                                }
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(ml_result4) => {
                                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(ml_result4)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesLoop5Result {
    MlPlanJavaScriptArrayBindingNamesLoop5Done,
    MlPlanJavaScriptArrayBindingNamesLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin8 {
    MlPlanJavaScriptArrayBindingNamesJoin8Next(f64),
    MlPlanJavaScriptArrayBindingNamesJoin8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin7 {
    MlPlanJavaScriptArrayBindingNamesJoin7Next(f64, Vec<f64>),
    MlPlanJavaScriptArrayBindingNamesJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin6 {
    MlPlanJavaScriptArrayBindingNamesJoin6Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_java_script_array_binding_names_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlPlanJavaScriptArrayBindingNamesLoop5Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(self::ml_array::append(vec![index], positions))
                    } else {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (value == (self::ml_string::Text::from_units(vec![44])))
                        {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let rest = false;
                                {
                                    let ml_s6 = if (kind
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        && (value
                                            == (self::ml_string::Text::from_units(vec![
                                                46, 46, 46,
                                            ]))) {
                                        {
                                            let rest_2 = true;
                                            {
                                                let index_3 = index + 1f64;
                                                self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index, rest)
                                    };
                                    match ml_s6 {
                                        self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                            } else {
                                                {
                                                    let ml_s7 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = { let (ml_append_right, ml_append_left) = (self::ml_array::slice(&nested, 1f64, f64::INFINITY), positions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = { let (ml_append_right, ml_append_left) = (vec![index_4], positions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match ml_s7 {
                                                        self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        self::ml_array::append(vec![index_7], positions_4)
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let ml_s8 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_7)
                                                                        };
                                                                        match ml_s8 {
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(self::ml_array::append(vec![index_9], positions_4))
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                            } else {
                                                                                                {
                                                                                                    let index_10 = index_9 + 1f64;
                                                                                                    {
                                                                                                        (index, positions) = (index_10, positions_4);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
}
                                                                                    }
                                                                                }
}
                                                                            }
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Return(ml_result8) => {
                                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result8)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(ml_result8_2) => {
                                                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result8_2)
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
            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Done
        };
    }
}
