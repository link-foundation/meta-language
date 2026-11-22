pub fn source_ml_source1_temporary_join_utf16_text_loop86(mut separator: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut offset: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (offset < ({ let ml_reference_array531 = separator.clone(); let ml_reference_projection531 = ml_reference_array531.borrow().len() as f64; ml_reference_projection531 })) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value532 = result.borrow().clone(); ml_reference_value532 }, vec![({ let ml_reference_array533 = separator.clone(); let ml_reference_position533 = offset; let ml_reference_projection533 = self::ml_array::optional_at(&*ml_reference_array533.borrow(), self::ml_array::number_index(ml_reference_position533)); ml_reference_projection533 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2.clone(), offset_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop87(mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut value: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut offset: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (offset < ({ let ml_reference_array534 = value.clone(); let ml_reference_projection534 = ml_reference_array534.borrow().len() as f64; ml_reference_projection534 })) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value535 = result.borrow().clone(); ml_reference_value535 }, vec![({ let ml_reference_array536 = value.clone(); let ml_reference_position536 = offset; let ml_reference_projection536 = self::ml_array::optional_at(&*ml_reference_array536.borrow(), self::ml_array::number_index(ml_reference_position536)); ml_reference_projection536 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2.clone(), offset_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop85(mut values: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut separator: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array537 = values.clone(); let ml_reference_projection537 = ml_reference_array537.borrow().len() as f64; ml_reference_projection537 })) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop86(separator.clone(), result.clone(), offset);
                            result_2.clone()
                        }
                    }
                } else {
                    result.clone()
                };
                {
                    let value = ({ let ml_reference_array538 = values.clone(); let ml_reference_position538 = at; let ml_reference_projection538 = self::ml_array::optional_at(&*ml_reference_array538.borrow(), self::ml_array::number_index(ml_reference_position538)); ml_reference_projection538 }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_source1_temporary_join_utf16_text_loop87(result_3.clone(), value.clone(), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4.clone(), at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_trim_utf16_text_loop88(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit(({ let ml_reference_array539 = text.clone(); let ml_reference_position539 = first; let ml_reference_projection539 = self::ml_array::optional_at(&*ml_reference_array539.borrow(), self::ml_array::number_index(ml_reference_position539)); ml_reference_projection539 }).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

pub fn source_ml_source1_temporary_trim_utf16_text_loop89(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit(({
            let source_ml_source2_array_read_index3545 = (last - 1f64);
            { let ml_reference_array540 = text.clone(); let ml_reference_position540 = source_ml_source2_array_read_index3545; let ml_reference_projection540 = self::ml_array::optional_at(&*ml_reference_array540.borrow(), self::ml_array::number_index(ml_reference_position540)); ml_reference_projection540 }
        }).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

pub fn source_ml_source1_temporary_character_units_utf16_text_loop90(mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array541 = values.clone(); let ml_reference_projection541 = ml_reference_array541.borrow().len() as f64; ml_reference_projection541 })) {
            {
                let value = ({ let ml_reference_array542 = values.clone(); let ml_reference_position542 = at; let ml_reference_projection542 = self::ml_array::optional_at(&*ml_reference_array542.borrow(), self::ml_array::number_index(ml_reference_position542)); ml_reference_projection542 }).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value543 = result.borrow().clone(); ml_reference_value543 }, vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }])));
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2.clone(), at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result {
    MlSource1TemporaryCodePointUnitsUtf16TextLoop91Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryCodePointUnitsUtf16TextLoop91Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_code_point_units_utf16_text_loop91(mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result {
    loop {
        return if (at < ({ let ml_reference_array544 = values.clone(); let ml_reference_projection544 = ml_reference_array544.borrow().len() as f64; ml_reference_projection544 })) {
            {
                let value = ({ let ml_reference_array545 = values.clone(); let ml_reference_position545 = at; let ml_reference_projection545 = self::ml_array::optional_at(&*ml_reference_array545.borrow(), self::ml_array::number_index(ml_reference_position545)); ml_reference_projection545 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Return(std::rc::Rc::new(std::cell::RefCell::new(vec![at])))
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value546 = result.borrow().clone(); ml_reference_value546 }, vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }])));
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2.clone(), at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value547 = result.borrow().clone(); ml_reference_value547 }, vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))])));
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3.clone(), at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Done(result.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDisplayPointsUtf16TextJoin93 {
    MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(f64, f64),
}

pub fn source_ml_source1_temporary_display_points_utf16_text_loop92(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array548 = text.clone(); let ml_reference_projection548 = ml_reference_array548.borrow().len() as f64; ml_reference_projection548 })) {
            {
                let point = ({ let ml_reference_array549 = text.clone(); let ml_reference_position549 = at; let ml_reference_projection549 = self::ml_array::optional_at(&*ml_reference_array549.borrow(), self::ml_array::number_index(ml_reference_position549)); ml_reference_projection549 }).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source2_s3554 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = ({
                                let source_ml_source2_array_read_index3556 = (at + 1f64);
                                { let ml_reference_array550 = text.clone(); let ml_reference_position550 = source_ml_source2_array_read_index3556; let ml_reference_projection550 = self::ml_array::optional_at(&*ml_reference_array550.borrow(), self::ml_array::number_index(ml_reference_position550)); ml_reference_projection550 }
                            }).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin93::MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlSource1TemporaryDisplayPointsUtf16TextJoin93::MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlSource1TemporaryDisplayPointsUtf16TextJoin93::MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(at, point_4)
                            }
                        } else {
                            self::MlSource1TemporaryDisplayPointsUtf16TextJoin93::MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(at, point)
                        }
                    };
                    match source_ml_source2_s3554.clone() {
                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin93::MlSource1TemporaryDisplayPointsUtf16TextJoin93Next(at_3, point_5) => {
                            {
                                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value551 = result.borrow().clone(); ml_reference_value551 }, vec![point_5])));
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2.clone(), at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_display_utf8_from_utf16_text_loop94(mut bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array552 = points.clone(); let ml_reference_projection552 = ml_reference_array552.borrow().len() as f64; ml_reference_projection552 })) {
            {
                let point = ({ let ml_reference_array553 = points.clone(); let ml_reference_position553 = at; let ml_reference_projection553 = self::ml_array::optional_at(&*ml_reference_array553.borrow(), self::ml_array::number_index(ml_reference_position553)); ml_reference_projection553 }).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value554 = bytes.borrow().clone(); ml_reference_value554 }, vec![point])));
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value555 = bytes.borrow().clone(); ml_reference_value555 }, vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))])));
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3.clone(), at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value556 = bytes.borrow().clone(); ml_reference_value556 }, vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))])));
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4.clone(), at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value557 = bytes.borrow().clone(); ml_reference_value557 }, vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))])));
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5.clone(), at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Result {
    MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
}

pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop95(mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Result {
    loop {
        return if (at < ({ let ml_reference_array558 = replacement.clone(); let ml_reference_projection558 = ml_reference_array558.borrow().len() as f64; ml_reference_projection558 })) {
            {
                let next = ({
                    let source_ml_source2_array_read_index3560 = (at + 1f64);
                    { let ml_reference_array559 = replacement.clone(); let ml_reference_position559 = source_ml_source2_array_read_index3560; let ml_reference_projection559 = self::ml_array::optional_at(&*ml_reference_array559.borrow(), self::ml_array::number_index(ml_reference_position559)); ml_reference_projection559 }
                }).clone().unwrap_or_else(|| (-1f64));
                if (((({ let ml_reference_array560 = replacement.clone(); let ml_reference_position560 = at; let ml_reference_projection560 = self::ml_array::optional_at(&*ml_reference_array560.borrow(), self::ml_array::number_index(ml_reference_position560)); ml_reference_projection560 }).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value561 = result.borrow().clone(); ml_reference_value561 }, vec![0f64, start, at])));
                                result_2.clone()
                            }
                        } else {
                            result.clone()
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value562 = result_3.borrow().clone(); ml_reference_value562 }, vec![0f64, at, (at + 1f64)])));
                                    result_4.clone()
                                }
                            } else {
                                {
                                    let result_5 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value563 = result_3.borrow().clone(); ml_reference_value563 }, vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64])));
                                    result_5.clone()
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    (result, start, at) = (result_6.clone(), at_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Done(result.clone(), start)
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop97(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array564 = text.clone(); let ml_reference_projection564 = ml_reference_array564.borrow().len() as f64; ml_reference_projection564 })) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value565 = result.borrow().clone(); ml_reference_value565 }, vec![({ let ml_reference_array566 = text.clone(); let ml_reference_position566 = at; let ml_reference_projection566 = self::ml_array::optional_at(&*ml_reference_array566.borrow(), self::ml_array::number_index(ml_reference_position566)); ml_reference_projection566 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop96Result {
    MlSource1TemporaryReplaceUtf16TextLoop96Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop98(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut found: f64, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < found) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value567 = result.borrow().clone(); ml_reference_value567 }, vec![({ let ml_reference_array568 = text.clone(); let ml_reference_position568 = at; let ml_reference_projection568 = self::ml_array::optional_at(&*ml_reference_array568.borrow(), self::ml_array::number_index(ml_reference_position568)); ml_reference_projection568 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop100(mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut last: f64, mut source: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < last) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value569 = result.borrow().clone(); ml_reference_value569 }, vec![({ let ml_reference_array570 = source.clone(); let ml_reference_position570 = at; let ml_reference_projection570 = self::ml_array::optional_at(&*ml_reference_array570.borrow(), self::ml_array::number_index(ml_reference_position570)); ml_reference_projection570 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop99(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut found: f64, mut instruction: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (instruction < ({ let ml_reference_array571 = plan.clone(); let ml_reference_projection571 = ml_reference_array571.borrow().len() as f64; ml_reference_projection571 })) {
            {
                let kind = ({ let ml_reference_array572 = plan.clone(); let ml_reference_position572 = instruction; let ml_reference_projection572 = self::ml_array::optional_at(&*ml_reference_array572.borrow(), self::ml_array::number_index(ml_reference_position572)); ml_reference_projection572 }).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        ({
                            let source_ml_source2_array_read_index3574 = (instruction + 1f64);
                            { let ml_reference_array573 = plan.clone(); let ml_reference_position573 = source_ml_source2_array_read_index3574; let ml_reference_projection573 = self::ml_array::optional_at(&*ml_reference_array573.borrow(), self::ml_array::number_index(ml_reference_position573)); ml_reference_projection573 }
                        }).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + { let ml_reference_array574 = search.clone(); let ml_reference_projection574 = ml_reference_array574.borrow().len() as f64; ml_reference_projection574 })
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            ({
                                let source_ml_source2_array_read_index3576 = (instruction + 2f64);
                                { let ml_reference_array575 = plan.clone(); let ml_reference_position575 = source_ml_source2_array_read_index3576; let ml_reference_projection575 = self::ml_array::optional_at(&*ml_reference_array575.borrow(), self::ml_array::number_index(ml_reference_position575)); ml_reference_projection575 }
                            }).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + { let ml_reference_array576 = search.clone(); let ml_reference_projection576 = ml_reference_array576.borrow().len() as f64; ml_reference_projection576 })
                                } else {
                                    { let ml_reference_array577 = text.clone(); let ml_reference_projection577 = ml_reference_array577.borrow().len() as f64; ml_reference_projection577 }
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.clone()
                            } else {
                                text.clone()
                            };
                            {
                                let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop100(result.clone(), last, source.clone(), first);
                                {
                                    let instruction_2 = (instruction + 3f64);
                                    {
                                        (result, instruction) = (result_2.clone(), instruction_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop96(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut all: bool, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut from: f64, mut searching: bool) -> self::MlSource1TemporaryReplaceUtf16TextLoop96Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text.clone(), search.clone(), from);
                if (found < (0f64)) {
                    {
                        searching = false;
                        continue;
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop98(text.clone(), result.clone(), found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop99(text.clone(), search.clone(), replacement.clone(), result_2.clone(), plan.clone(), found, instruction);
                                    {
                                        let start_2 = (found + { let ml_reference_array578 = search.clone(); let ml_reference_projection578 = ml_reference_array578.borrow().len() as f64; ml_reference_projection578 });
                                        {
                                            let searching_2 = (all && !(((({ let ml_reference_array579 = search.clone(); let ml_reference_projection579 = ml_reference_array579.borrow().len() as f64; ml_reference_projection579 }) == (0f64)) && (found == ({ let ml_reference_array580 = text.clone(); let ml_reference_projection580 = ml_reference_array580.borrow().len() as f64; ml_reference_projection580 })))));
                                            {
                                                let from_2 = (start_2 + if (({ let ml_reference_array581 = search.clone(); let ml_reference_projection581 = ml_reference_array581.borrow().len() as f64; ml_reference_projection581 }) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3.clone(), start_2, from_2, searching_2);
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
            self::MlSource1TemporaryReplaceUtf16TextLoop96Result::MlSource1TemporaryReplaceUtf16TextLoop96Done(result.clone(), start)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bool),
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop101(mut operation: self::ml_string::Text, mut receiver: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut formed: bool, mut at: f64) -> self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result {
    loop {
        return if (at < length) {
            {
                let unit = self::read_java_script_semantic_text_unit(receiver.clone(), at);
                {
                    let high = ((unit >= (55296f64)) && (unit <= (56319f64)));
                    {
                        let low = ((unit >= (56320f64)) && (unit <= (57343f64)));
                        {
                            let next = if (((at + 1f64)) < length) {
                                self::read_java_script_semantic_text_unit(receiver.clone(), (at + 1f64))
                            } else {
                                (-1f64)
                            };
                            if ((high && (next >= (56320f64))) && (next <= (57343f64))) {
                                {
                                    let units_4 = if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) {
                                        {
                                            let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value582 = units.borrow().clone(); ml_reference_value582 }, vec![unit])));
                                            {
                                                let units_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value583 = units_2.borrow().clone(); ml_reference_value583 }, vec![next])));
                                                units_3.clone()
                                            }
                                        }
                                    } else {
                                        units.clone()
                                    };
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            let at_3 = (at_2 + 1f64);
                                            {
                                                (units, at) = (units_4.clone(), at_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if (high || low) {
                                    if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) {
                                        self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false)))))
                                    } else {
                                        {
                                            let units_5 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value584 = units.borrow().clone(); ml_reference_value584 }, vec![65533f64])));
                                            {
                                                let at_4 = (at + 1f64);
                                                {
                                                    (units, formed, at) = (units_5.clone(), false, at_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) {
                                        {
                                            let units_6 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value585 = units.borrow().clone(); ml_reference_value585 }, vec![unit])));
                                            {
                                                let at_5 = (at + 1f64);
                                                {
                                                    (units, at) = (units_6.clone(), at_5);
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let at_6 = (at + 1f64);
                                            {
                                                at = at_6;
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
        } else {
            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Done(units.clone(), formed)
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop102(mut receiver: self::JavaScriptSemanticValue, mut high: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < high) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value586 = units.borrow().clone(); ml_reference_value586 }, vec![self::read_java_script_semantic_text_unit(receiver.clone(), at)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop103(mut receiver: self::JavaScriptSemanticValue, mut second: self::JavaScriptSemanticValue, mut needle_length: f64, mut start: f64, mut matches: bool, mut at: f64) -> bool {
    loop {
        return if (matches && (at < needle_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver.clone(), (start + at))) != (self::read_java_script_semantic_text_unit(second.clone(), at))) {
                {
                    let at_2 = (at + 1f64);
                    {
                        (matches, at) = (false, at_2);
                        continue;
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Done,
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop105(mut receiver: self::JavaScriptSemanticValue, mut second: self::JavaScriptSemanticValue, mut needle_length: f64, mut source_ml_source1_shadow_start_1: f64, mut matches: bool, mut at: f64) -> bool {
    loop {
        return if (matches && (at < needle_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver.clone(), (source_ml_source1_shadow_start_1 + at))) != (self::read_java_script_semantic_text_unit(second.clone(), at))) {
                {
                    let at_2 = (at + 1f64);
                    {
                        (matches, at) = (false, at_2);
                        continue;
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop104(mut receiver: self::JavaScriptSemanticValue, mut second: self::JavaScriptSemanticValue, mut length: f64, mut needle_length: f64, mut source_ml_source1_shadow_start_1: f64) -> self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result {
    loop {
        return if (source_ml_source1_shadow_start_1 <= ((length - needle_length))) {
            {
                let matches = true;
                {
                    let at = 0f64;
                    {
                        let matches_2 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop105(receiver.clone(), second.clone(), needle_length, source_ml_source1_shadow_start_1, matches, at);
                        if matches_2 {
                            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true)))))
                        } else {
                            {
                                let source_ml_source1_shadow_start_1_2 = (source_ml_source1_shadow_start_1 + 1f64);
                                {
                                    source_ml_source1_shadow_start_1 = source_ml_source1_shadow_start_1_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result {
    MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Done,
    MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, bool)>>),
}
