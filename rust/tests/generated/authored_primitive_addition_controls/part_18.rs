pub fn convert_java_script_string_big_integer_units(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1650 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop114(units.clone(), index);
            match source_ml_source2_s1650.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Done => {
                    {
                        let start = 0f64;
                        {
                            let end = { let ml_reference_array252 = units.clone(); let ml_reference_projection252 = ml_reference_array252.borrow().len() as f64; ml_reference_projection252 };
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop115(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop116(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0)]))
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = ({ let ml_reference_array253 = units.clone(); let ml_reference_position253 = start_2; let ml_reference_projection253 = self::ml_array::optional_at(&*ml_reference_array253.borrow(), self::ml_array::number_index(ml_reference_position253)); ml_reference_projection253 }).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s1653 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s1653.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117Next(start_4, negative_3, signed_2) => {
                                                                if (start_4 == end_2) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s1660 = if ((!signed_2 && ((({ let ml_reference_array254 = units.clone(); let ml_reference_position254 = start_4; let ml_reference_projection254 = self::ml_array::optional_at(&*ml_reference_array254.borrow(), self::ml_array::number_index(ml_reference_position254)); ml_reference_projection254 }).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = ({
                                                                                            let source_ml_source2_array_read_index1666 = (start_4 + 1f64);
                                                                                            { let ml_reference_array255 = units.clone(); let ml_reference_position255 = source_ml_source2_array_read_index1666; let ml_reference_projection255 = self::ml_array::optional_at(&*ml_reference_array255.borrow(), self::ml_array::number_index(ml_reference_position255)); ml_reference_projection255 }
                                                                                        }).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let selected = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (selected != (0f64)) {
                                                                                                {
                                                                                                    let base_2 = if (selected == (16f64)) {
                                                                                                        self::ml::Big::from_i128(16)
                                                                                                    } else {
                                                                                                        if (selected == (8f64)) {
                                                                                                            self::ml::Big::from_i128(8)
                                                                                                        } else {
                                                                                                            self::ml::Big::from_i128(2)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let start_5 = (start_4 + 2f64);
                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118Next(start_5, selected, base_2.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s1660.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118Next(start_6, radix_2, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                                        } else {
                                                                                            {
                                                                                                let digits = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)]));
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s1667 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop119(units.clone(), end_2, radix_2, base_3.clone(), digits.clone(), value.clone(), start_6);
                                                                                                        match source_ml_source2_s1667.clone() {
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Done(value_2) => {
                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![if negative_3 {
                                                                                                                    value_2.neg()
                                                                                                                } else {
                                                                                                                    value_2.clone()
                                                                                                                }]))
                                                                                                            }
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Return(source_ml_source1_temporary_result119) => {
                                                                                                                source_ml_source1_temporary_result119.clone()
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Return(source_ml_source1_temporary_result119_2) => {
                    source_ml_source1_temporary_result119_2.clone()
                }
            }
        }
    }
}

pub fn convert_java_script_utf8_big_integer(bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    self::convert_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes.clone()))
}

pub fn convert_java_script_number_big_integer(value: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if !(self::ml_math::is_integer(value)) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
    } else {
        {
            let negative = (value < (0f64));
            {
                let value_2 = value.abs();
                {
                    let integer = self::ml::Big::from_i128(0);
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop120(value_2, integer.clone(), weight.clone());
                            std::rc::Rc::new(std::cell::RefCell::new(vec![if negative {
                                integer_2.neg()
                            } else {
                                integer_2.clone()
                            }]))
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_boolean_big_integer(value: bool) -> self::ml::Big {
    if value {
        self::ml::Big::from_i128(1)
    } else {
        self::ml::Big::from_i128(0)
    }
}

pub fn compare_java_script_number_big_integer(value: f64, integer: self::ml::Big) -> f64 {
    if value.is_nan() {
        2f64
    } else {
        if (value == (f64::INFINITY)) {
            1f64
        } else {
            if (value == (f64::NEG_INFINITY)) {
                (-1f64)
            } else {
                {
                    let floor = value.floor();
                    {
                        let lower = ({
                            let source_ml_source2_array_read_value1668 = self::convert_java_script_number_big_integer(floor);
                            {
                                let source_ml_source2_array_read_index1669 = 0f64;
                                { let ml_reference_array256 = source_ml_source2_array_read_value1668.clone(); let ml_reference_position256 = source_ml_source2_array_read_index1669; let ml_reference_projection256 = self::ml_array::optional_at(&*ml_reference_array256.borrow(), self::ml_array::number_index(ml_reference_position256)); ml_reference_projection256 }
                            }
                        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                        if (lower < integer) {
                            (-1f64)
                        } else {
                            if (lower > integer) {
                                1f64
                            } else {
                                if (value == floor) {
                                    0f64
                                } else {
                                    1f64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_string_big_integer_units(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, integer: self::ml::Big) -> f64 {
    {
        let result = self::convert_java_script_string_big_integer_units(units.clone());
        if (({ let ml_reference_array257 = result.clone(); let ml_reference_projection257 = ml_reference_array257.borrow().len() as f64; ml_reference_projection257 }) != (1f64)) {
            2f64
        } else {
            {
                let value = ({
                    let source_ml_source2_array_read_index1671 = 0f64;
                    { let ml_reference_array258 = result.clone(); let ml_reference_position258 = source_ml_source2_array_read_index1671; let ml_reference_projection258 = self::ml_array::optional_at(&*ml_reference_array258.borrow(), self::ml_array::number_index(ml_reference_position258)); ml_reference_projection258 }
                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                if (value < integer) {
                    (-1f64)
                } else {
                    if (value > integer) {
                        1f64
                    } else {
                        0f64
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_utf8_big_integer(bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, integer: self::ml::Big) -> f64 {
    self::compare_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes.clone()), integer.clone())
}

pub fn divide_java_script_big_integer(left: self::ml::Big, right: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if (right == (self::ml::Big::from_i128(0))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
    } else {
        {
            let remaining = if (left < (self::ml::Big::from_i128(0))) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if (right < (self::ml::Big::from_i128(0))) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor.clone();
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s1672 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop121(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s1672.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s1673 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop122(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s1673.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Done(remaining_2, quotient_2) => {
                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![if (((left < (self::ml::Big::from_i128(0)))) != ((right < (self::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (self::ml::Big::from_i128(0))) {
                                                        remaining_2.neg()
                                                    } else {
                                                        remaining_2.clone()
                                                    }]))
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

pub fn power_java_script_big_integer(base: self::ml::Big, exponent: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if (exponent < (self::ml::Big::from_i128(0))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop123(result.clone(), base.clone(), exponent.clone());
                std::rc::Rc::new(std::cell::RefCell::new(vec![result_2.clone()]))
            }
        }
    }
}

pub fn normalize_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if (value <= (self::ml::Big::from_i128(0))) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(0)]))
    } else {
        {
            let source_ml_source2_s1674 = self::source_ml_source1_temporary_normalize_number_power_bound_loop124(value.clone(), exponent.clone(), scale.clone(), upward);
            match source_ml_source2_s1674.clone() {
                self::MlSource1TemporaryNormalizeNumberPowerBoundLoop124Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop124Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s1675 = self::source_ml_source1_temporary_normalize_number_power_bound_loop125(value_2.clone(), exponent_2.clone(), scale.clone());
                        match source_ml_source2_s1675.clone() {
                            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop125Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop125Done(value_3, exponent_3) => {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![value_3.clone(), exponent_3.clone()]))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn divide_number_power_bound(numerator: self::ml::Big, denominator: self::ml::Big, upward: bool) -> self::ml::Big {
    {
        let result = self::divide_java_script_big_integer(numerator.clone(), denominator.clone());
        {
            let quotient = ({
                let source_ml_source2_array_read_index1677 = 0f64;
                { let ml_reference_array259 = result.clone(); let ml_reference_position259 = source_ml_source2_array_read_index1677; let ml_reference_projection259 = self::ml_array::optional_at(&*ml_reference_array259.borrow(), self::ml_array::number_index(ml_reference_position259)); ml_reference_projection259 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let remainder = ({
                    let source_ml_source2_array_read_index1679 = 1f64;
                    { let ml_reference_array260 = result.clone(); let ml_reference_position260 = source_ml_source2_array_read_index1679; let ml_reference_projection260 = self::ml_array::optional_at(&*ml_reference_array260.borrow(), self::ml_array::number_index(ml_reference_position260)); ml_reference_projection260 }
                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                quotient.add(&if (upward && (remainder != (self::ml::Big::from_i128(0)))) {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                })
            }
        }
    }
}

pub fn root_number_power_integer(value: self::ml::Big) -> self::ml::Big {
    if (value <= (self::ml::Big::from_i128(0))) {
        self::ml::Big::from_i128(0)
    } else {
        {
            let root = self::ml::Big::from_i128(1);
            {
                let root_2 = self::source_ml_source1_temporary_root_number_power_integer_loop126(root.clone(), value.clone());
                {
                    let root_3 = root_2.mul(&self::ml::Big::from_i128(2));
                    self::source_ml_source1_temporary_root_number_power_integer_loop127(value.clone(), root_3.clone())
                }
            }
        }
    }
}

pub fn multiply_number_power_interval(left: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, right: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, scale: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound((({
            let source_ml_source2_array_read_index1681 = 0f64;
            { let ml_reference_array261 = left.clone(); let ml_reference_position261 = source_ml_source2_array_read_index1681; let ml_reference_projection261 = self::ml_array::optional_at(&*ml_reference_array261.borrow(), self::ml_array::number_index(ml_reference_position261)); ml_reference_projection261 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&({
            let source_ml_source2_array_read_index1683 = 0f64;
            { let ml_reference_array262 = right.clone(); let ml_reference_position262 = source_ml_source2_array_read_index1683; let ml_reference_projection262 = self::ml_array::optional_at(&*ml_reference_array262.borrow(), self::ml_array::number_index(ml_reference_position262)); ml_reference_projection262 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false), (({
            let source_ml_source2_array_read_index1685 = 1f64;
            { let ml_reference_array263 = left.clone(); let ml_reference_position263 = source_ml_source2_array_read_index1685; let ml_reference_projection263 = self::ml_array::optional_at(&*ml_reference_array263.borrow(), self::ml_array::number_index(ml_reference_position263)); ml_reference_projection263 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&({
            let source_ml_source2_array_read_index1687 = 1f64;
            { let ml_reference_array264 = right.clone(); let ml_reference_position264 = source_ml_source2_array_read_index1687; let ml_reference_projection264 = self::ml_array::optional_at(&*ml_reference_array264.borrow(), self::ml_array::number_index(ml_reference_position264)); ml_reference_projection264 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound((({
                let source_ml_source2_array_read_index1689 = 2f64;
                { let ml_reference_array265 = left.clone(); let ml_reference_position265 = source_ml_source2_array_read_index1689; let ml_reference_projection265 = self::ml_array::optional_at(&*ml_reference_array265.borrow(), self::ml_array::number_index(ml_reference_position265)); ml_reference_projection265 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&({
                let source_ml_source2_array_read_index1691 = 2f64;
                { let ml_reference_array266 = right.clone(); let ml_reference_position266 = source_ml_source2_array_read_index1691; let ml_reference_projection266 = self::ml_array::optional_at(&*ml_reference_array266.borrow(), self::ml_array::number_index(ml_reference_position266)); ml_reference_projection266 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true), (({
                let source_ml_source2_array_read_index1693 = 3f64;
                { let ml_reference_array267 = left.clone(); let ml_reference_position267 = source_ml_source2_array_read_index1693; let ml_reference_projection267 = self::ml_array::optional_at(&*ml_reference_array267.borrow(), self::ml_array::number_index(ml_reference_position267)); ml_reference_projection267 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&({
                let source_ml_source2_array_read_index1695 = 3f64;
                { let ml_reference_array268 = right.clone(); let ml_reference_position268 = source_ml_source2_array_read_index1695; let ml_reference_projection268 = self::ml_array::optional_at(&*ml_reference_array268.borrow(), self::ml_array::number_index(ml_reference_position268)); ml_reference_projection268 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true);
            std::rc::Rc::new(std::cell::RefCell::new(vec![({
                let source_ml_source2_array_read_index1697 = 0f64;
                { let ml_reference_array269 = lower.clone(); let ml_reference_position269 = source_ml_source2_array_read_index1697; let ml_reference_projection269 = self::ml_array::optional_at(&*ml_reference_array269.borrow(), self::ml_array::number_index(ml_reference_position269)); ml_reference_projection269 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1699 = 1f64;
                { let ml_reference_array270 = lower.clone(); let ml_reference_position270 = source_ml_source2_array_read_index1699; let ml_reference_projection270 = self::ml_array::optional_at(&*ml_reference_array270.borrow(), self::ml_array::number_index(ml_reference_position270)); ml_reference_projection270 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1701 = 0f64;
                { let ml_reference_array271 = upper.clone(); let ml_reference_position271 = source_ml_source2_array_read_index1701; let ml_reference_projection271 = self::ml_array::optional_at(&*ml_reference_array271.borrow(), self::ml_array::number_index(ml_reference_position271)); ml_reference_projection271 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1703 = 1f64;
                { let ml_reference_array272 = upper.clone(); let ml_reference_position272 = source_ml_source2_array_read_index1703; let ml_reference_projection272 = self::ml_array::optional_at(&*ml_reference_array272.borrow(), self::ml_array::number_index(ml_reference_position272)); ml_reference_projection272 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]))
        }
    }
}

pub fn root_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let source_ml_source2_s1704 = if ((self::ml::divide(&exponent, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) != (self::ml::Big::from_i128(0))) {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    self::MlSource1TemporaryRootNumberPowerBoundJoin128::MlSource1TemporaryRootNumberPowerBoundJoin128Next(value_2.clone(), exponent_2.clone())
                }
            }
        } else {
            self::MlSource1TemporaryRootNumberPowerBoundJoin128::MlSource1TemporaryRootNumberPowerBoundJoin128Next(value.clone(), exponent.clone())
        };
        match source_ml_source2_s1704.clone() {
            self::MlSource1TemporaryRootNumberPowerBoundJoin128::MlSource1TemporaryRootNumberPowerBoundJoin128Next(value_3, exponent_3) => {
                {
                    let square = value_3.mul(&scale);
                    {
                        let root = self::root_number_power_integer(square.clone());
                        {
                            let root_3 = if (upward && ((root.mul(&root)) < square)) {
                                {
                                    let root_2 = root.add(&self::ml::Big::from_i128(1));
                                    root_2.clone()
                                }
                            } else {
                                root.clone()
                            };
                            self::normalize_number_power_bound(root_3.clone(), self::ml::divide(&exponent_3, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false), scale.clone(), upward)
                        }
                    }
                }
            }
        }
    }
}

pub fn root_number_power_interval(interval: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, scale: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let lower = self::root_number_power_bound(({
            let source_ml_source2_array_read_index1706 = 0f64;
            { let ml_reference_array273 = interval.clone(); let ml_reference_position273 = source_ml_source2_array_read_index1706; let ml_reference_projection273 = self::ml_array::optional_at(&*ml_reference_array273.borrow(), self::ml_array::number_index(ml_reference_position273)); ml_reference_projection273 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
            let source_ml_source2_array_read_index1708 = 1f64;
            { let ml_reference_array274 = interval.clone(); let ml_reference_position274 = source_ml_source2_array_read_index1708; let ml_reference_projection274 = self::ml_array::optional_at(&*ml_reference_array274.borrow(), self::ml_array::number_index(ml_reference_position274)); ml_reference_projection274 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), false);
        {
            let upper = self::root_number_power_bound(({
                let source_ml_source2_array_read_index1710 = 2f64;
                { let ml_reference_array275 = interval.clone(); let ml_reference_position275 = source_ml_source2_array_read_index1710; let ml_reference_projection275 = self::ml_array::optional_at(&*ml_reference_array275.borrow(), self::ml_array::number_index(ml_reference_position275)); ml_reference_projection275 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1712 = 3f64;
                { let ml_reference_array276 = interval.clone(); let ml_reference_position276 = source_ml_source2_array_read_index1712; let ml_reference_projection276 = self::ml_array::optional_at(&*ml_reference_array276.borrow(), self::ml_array::number_index(ml_reference_position276)); ml_reference_projection276 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), true);
            std::rc::Rc::new(std::cell::RefCell::new(vec![({
                let source_ml_source2_array_read_index1714 = 0f64;
                { let ml_reference_array277 = lower.clone(); let ml_reference_position277 = source_ml_source2_array_read_index1714; let ml_reference_projection277 = self::ml_array::optional_at(&*ml_reference_array277.borrow(), self::ml_array::number_index(ml_reference_position277)); ml_reference_projection277 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1716 = 1f64;
                { let ml_reference_array278 = lower.clone(); let ml_reference_position278 = source_ml_source2_array_read_index1716; let ml_reference_projection278 = self::ml_array::optional_at(&*ml_reference_array278.borrow(), self::ml_array::number_index(ml_reference_position278)); ml_reference_projection278 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1718 = 0f64;
                { let ml_reference_array279 = upper.clone(); let ml_reference_position279 = source_ml_source2_array_read_index1718; let ml_reference_projection279 = self::ml_array::optional_at(&*ml_reference_array279.borrow(), self::ml_array::number_index(ml_reference_position279)); ml_reference_projection279 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1720 = 1f64;
                { let ml_reference_array280 = upper.clone(); let ml_reference_position280 = source_ml_source2_array_read_index1720; let ml_reference_projection280 = self::ml_array::optional_at(&*ml_reference_array280.borrow(), self::ml_array::number_index(ml_reference_position280)); ml_reference_projection280 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]))
        }
    }
}

pub fn invert_number_power_interval(interval: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, scale: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), ({
            let source_ml_source2_array_read_index1722 = 2f64;
            { let ml_reference_array281 = interval.clone(); let ml_reference_position281 = source_ml_source2_array_read_index1722; let ml_reference_projection281 = self::ml_array::optional_at(&*ml_reference_array281.borrow(), self::ml_array::number_index(ml_reference_position281)); ml_reference_projection281 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), false), (({
            let source_ml_source2_array_read_index1724 = 3f64;
            { let ml_reference_array282 = interval.clone(); let ml_reference_position282 = source_ml_source2_array_read_index1724; let ml_reference_projection282 = self::ml_array::optional_at(&*ml_reference_array282.borrow(), self::ml_array::number_index(ml_reference_position282)); ml_reference_projection282 }
        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), ({
                let source_ml_source2_array_read_index1726 = 0f64;
                { let ml_reference_array283 = interval.clone(); let ml_reference_position283 = source_ml_source2_array_read_index1726; let ml_reference_projection283 = self::ml_array::optional_at(&*ml_reference_array283.borrow(), self::ml_array::number_index(ml_reference_position283)); ml_reference_projection283 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), true), (({
                let source_ml_source2_array_read_index1728 = 1f64;
                { let ml_reference_array284 = interval.clone(); let ml_reference_position284 = source_ml_source2_array_read_index1728; let ml_reference_projection284 = self::ml_array::optional_at(&*ml_reference_array284.borrow(), self::ml_array::number_index(ml_reference_position284)); ml_reference_projection284 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), true);
            std::rc::Rc::new(std::cell::RefCell::new(vec![({
                let source_ml_source2_array_read_index1730 = 0f64;
                { let ml_reference_array285 = lower.clone(); let ml_reference_position285 = source_ml_source2_array_read_index1730; let ml_reference_projection285 = self::ml_array::optional_at(&*ml_reference_array285.borrow(), self::ml_array::number_index(ml_reference_position285)); ml_reference_projection285 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1732 = 1f64;
                { let ml_reference_array286 = lower.clone(); let ml_reference_position286 = source_ml_source2_array_read_index1732; let ml_reference_projection286 = self::ml_array::optional_at(&*ml_reference_array286.borrow(), self::ml_array::number_index(ml_reference_position286)); ml_reference_projection286 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1734 = 0f64;
                { let ml_reference_array287 = upper.clone(); let ml_reference_position287 = source_ml_source2_array_read_index1734; let ml_reference_projection287 = self::ml_array::optional_at(&*ml_reference_array287.borrow(), self::ml_array::number_index(ml_reference_position287)); ml_reference_projection287 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                let source_ml_source2_array_read_index1736 = 1f64;
                { let ml_reference_array288 = upper.clone(); let ml_reference_position288 = source_ml_source2_array_read_index1736; let ml_reference_projection288 = self::ml_array::optional_at(&*ml_reference_array288.borrow(), self::ml_array::number_index(ml_reference_position288)); ml_reference_projection288 }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]))
        }
    }
}

pub fn decompose_number_power_magnitude(value: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let exponent = self::ml::Big::from_i128(0);
        {
            let source_ml_source2_s1737 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop129(value, exponent.clone());
            match source_ml_source2_s1737.clone() {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s1738 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop130(value_2, exponent_2.clone());
                        match source_ml_source2_s1738.clone() {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Done(value_3, exponent_3) => {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![({
                                    let source_ml_source2_array_read_value1739 = self::convert_java_script_number_big_integer((value_3 * 4503599627370496f64));
                                    {
                                        let source_ml_source2_array_read_index1740 = 0f64;
                                        { let ml_reference_array289 = source_ml_source2_array_read_value1739.clone(); let ml_reference_position289 = source_ml_source2_array_read_index1740; let ml_reference_projection289 = self::ml_array::optional_at(&*ml_reference_array289.borrow(), self::ml_array::number_index(ml_reference_position289)); ml_reference_projection289 }
                                    }
                                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), exponent_3.sub(&self::ml::Big::from_i128(52))]))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn round_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, precision: self::ml::Big) -> f64 {
    if ((value == (self::ml::Big::from_i128(0))) || (exponent < (self::ml::Big::from_i128(-1076)))) {
        0f64
    } else {
        if (exponent > (self::ml::Big::from_i128(1024))) {
            f64::INFINITY
        } else {
            {
                let shift = exponent.sub(&precision);
                if (shift >= (self::ml::Big::from_i128(0))) {
                    self::convert_string_number_rational(value.mul(&({
                        let source_ml_source2_array_read_value1741 = self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.clone());
                        {
                            let source_ml_source2_array_read_index1742 = 0f64;
                            { let ml_reference_array290 = source_ml_source2_array_read_value1741.clone(); let ml_reference_position290 = source_ml_source2_array_read_index1742; let ml_reference_projection290 = self::ml_array::optional_at(&*ml_reference_array290.borrow(), self::ml_array::number_index(ml_reference_position290)); ml_reference_projection290 }
                        }
                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), self::ml::Big::from_i128(1))
                } else {
                    self::convert_string_number_rational(value.clone(), ({
                        let source_ml_source2_array_read_value1743 = self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.neg());
                        {
                            let source_ml_source2_array_read_index1744 = 0f64;
                            { let ml_reference_array291 = source_ml_source2_array_read_value1743.clone(); let ml_reference_position291 = source_ml_source2_array_read_index1744; let ml_reference_projection291 = self::ml_array::optional_at(&*ml_reference_array291.borrow(), self::ml_array::number_index(ml_reference_position291)); ml_reference_projection291 }
                        }
                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))
                }
            }
        }
    }
}

pub fn power_java_script_number(base: f64, exponent: f64) -> f64 {
    if exponent.is_nan() {
        f64::NAN
    } else {
        if (exponent == (0f64)) {
            1f64
        } else {
            if base.is_nan() {
                f64::NAN
            } else {
                {
                    let magnitude = base.abs();
                    if !(exponent.is_finite()) {
                        if (magnitude == (1f64)) {
                            f64::NAN
                        } else {
                            if (magnitude > (1f64)) {
                                if (exponent > (0f64)) {
                                    f64::INFINITY
                                } else {
                                    0f64
                                }
                            } else {
                                if (exponent > (0f64)) {
                                    0f64
                                } else {
                                    f64::INFINITY
                                }
                            }
                        }
                    } else {
                        {
                            let odd = (self::ml_math::is_integer(exponent) && (((exponent.abs() % 2f64)) == (1f64)));
                            {
                                let negative = (((base < (0f64)) || ((base == (0f64)) && (((1f64 / base)) < (0f64)))) && odd);
                                if (base == (0f64)) {
                                    {
                                        let value = if (exponent > (0f64)) {
                                            0f64
                                        } else {
                                            f64::INFINITY
                                        };
                                        if negative {
                                            (-value)
                                        } else {
                                            value
                                        }
                                    }
                                } else {
                                    if !(base.is_finite()) {
                                        {
                                            let value_2 = if (exponent > (0f64)) {
                                                f64::INFINITY
                                            } else {
                                                0f64
                                            };
                                            if negative {
                                                (-value_2)
                                            } else {
                                                value_2
                                            }
                                        }
                                    } else {
                                        if ((base < (0f64)) && !(self::ml_math::is_integer(exponent))) {
                                            f64::NAN
                                        } else {
                                            if (magnitude == (1f64)) {
                                                if negative {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                }
                                            } else {
                                                {
                                                    let input = self::decompose_number_power_magnitude(magnitude);
                                                    {
                                                        let power = self::decompose_number_power_magnitude(exponent.abs());
                                                        {
                                                            let precision = self::ml::Big::from_i128(192);
                                                            self::source_ml_source1_temporary_power_java_script_number_loop131(exponent, negative, input.clone(), power.clone(), precision.clone())
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

pub fn accept_java_script_exponentiation_base(prefix: self::ml_string::Text, parenthesized: bool) -> bool {
    (parenthesized || !(self::ml_array::includes(&{ let ml_reference_value292 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43]), self::ml_string::Text::from_units(vec![45]), self::ml_string::Text::from_units(vec![33]), self::ml_string::Text::from_units(vec![126]), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]), self::ml_string::Text::from_units(vec![118, 111, 105, 100]), self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116])]))).borrow().clone(); ml_reference_value292 }, &prefix.clone(), 0f64, false, |a, b| a == b)))
}

pub fn read_java_script_big_integer_exponent_message() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101])
}

pub fn render_java_script_big_integer_power(target: self::ml_string::Text, base: self::ml_string::Text, exponent: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 32])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_nonzero_java_script_integer_literal(value: self::ml_string::Text) -> bool {
    (((value != (self::ml_string::Text::from_units(vec![]))) && (value != (self::ml_string::Text::from_units(vec![48])))) && (value != (self::ml_string::Text::from_units(vec![45, 48]))))
}

pub fn require_java_script_big_integer_division(language: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, literal: self::ml_string::Text) -> bool {
    ((((language == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) && (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) && ((operation == (self::ml_string::Text::from_units(vec![100, 105, 118]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 109]))))) && !(self::accept_nonzero_java_script_integer_literal(literal.clone())))
}

pub fn read_java_script_big_integer_division_message() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])
}

pub fn render_java_script_big_integer_division(target: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 100, 105, 118, 105, 100, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                } else {
                    if (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                    } else {
                        if (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) {
                            self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                        } else {
                            self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_error_class(form: self::ml_string::Text) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
    } else {
        if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
        } else {
            if ((form == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn render_java_script_big_integer_conversion_message(form: self::ml_string::Text, value: self::ml_string::Text) -> self::ml_string::Text {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form.clone());
        if (({ let ml_reference_array293 = pieces.clone(); let ml_reference_projection293 = ml_reference_array293.borrow().len() as f64; ml_reference_projection293 }) == (2f64)) {
            ((({
                let source_ml_source2_array_read_index1746 = 0f64;
                { let ml_reference_array294 = pieces.clone(); let ml_reference_position294 = source_ml_source2_array_read_index1746; let ml_reference_projection294 = self::ml_array::optional_at(&*ml_reference_array294.borrow(), self::ml_array::number_index(ml_reference_position294)); ml_reference_projection294 }
            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(&value)).concat(&({
                let source_ml_source2_array_read_index1748 = 1f64;
                { let ml_reference_array295 = pieces.clone(); let ml_reference_position295 = source_ml_source2_array_read_index1748; let ml_reference_projection295 = self::ml_array::optional_at(&*ml_reference_array295.borrow(), self::ml_array::number_index(ml_reference_position295)); ml_reference_projection295 }
            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_big_integer_conversion_message_pieces(form: self::ml_string::Text) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32]), self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114])]))
    } else {
        if (((form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (form == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32]), self::ml_string::Text::from_units(vec![32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116])]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
        }
    }
}

pub fn read_java_script_big_integer_builtin_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_big_integer_result_operation(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 86, 97, 108, 117, 101]))) {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                    self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                        self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114]))) {
                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 111, 111, 108, 101, 97, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101]))) {
                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116]))) {
                                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_linked_java_script_big_integer_result(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_java_script_big_integer_result_operation(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_java_script_big_integer_intrinsic_operand_kinds(operation: self::ml_string::Text) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if ((form == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 115, 105, 111, 110]))) || (form == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114])))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 116])]))
        } else {
            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114]))) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])]))
            } else {
                if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101]))) {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 110, 116])]))
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101]))) {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![105, 110, 116])]))
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])]))
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])]))
                            } else {
                                if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 111, 111, 108])]))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
