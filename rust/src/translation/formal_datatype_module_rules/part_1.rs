// Generated from complete js/src/translation/formal-datatype-module-rules.js. Do not edit by hand.
use super::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result;
use super::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result;
use super::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result;
use super::source_ml_temporary_plan_formal_datatype_module_closure_loop9;
use super::source_ml_temporary_plan_formal_datatype_module_closure_loop10;
use super::source_ml_temporary_plan_formal_datatype_module_prefixes_loop1;
use super::source_ml_temporary_plan_formal_datatype_module_prefixes_loop2;

/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript `Number::toString`: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
    #[must_use]
    pub fn js_number(x: f64) -> String {
        if x.is_nan() {
            return String::from("NaN");
        }
        if x.is_infinite() {
            return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" });
        }
        if x == 0.0 {
            return String::from("0");
        }
        let (mut digits, n) = decimal(&format!("{:e}", x.abs()));
        // Rust's shortest digits round a tie away from zero; ECMAScript takes the even neighbour.
        let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
        let rest = &exact[digits.len().min(exact.len())..];
        if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
            let last = digits.as_bytes()[digits.len() - 1] - b'0';
            let down = exact[..digits.len()].to_string();
            let other = if digits == down {
                increment(&down)
            } else {
                Some(down)
            };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");
        let body = if k <= n && n <= 21 {
            format!("{digits}{}", "0".repeat((n - k) as usize))
        } else if 0 < n && n <= 21 {
            format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
        } else if -6 < n && n <= 0 {
            format!("0.{}{digits}", "0".repeat((-n) as usize))
        } else {
            let sign = if n - 1 < 0 { '-' } else { '+' };
            let power = (n - 1).abs();
            if k == 1 {
                format!("{digits}e{sign}{power}")
            } else {
                format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
            }
        };
        if x < 0.0 { format!("-{body}") } else { body }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    #[must_use]
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// `SameValue`, as `Object.is` and `assert.strictEqual` compare: NaN equals
    /// NaN, and 0 differs from -0.
    #[must_use]
    pub const fn same_value(a: f64, b: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            a.is_nan() && b.is_nan()
        } else {
            a.to_bits() == b.to_bits()
        }
    }

    /// A decimal digit string plus one, or None when it gains a digit.
    fn increment(digits: &str) -> Option<String> {
        let mut bytes = digits.as_bytes().to_vec();
        for index in (0..bytes.len()).rev() {
            if bytes[index] == b'9' {
                bytes[index] = b'0';
            } else {
                bytes[index] += 1;
                return String::from_utf8(bytes).ok();
            }
        }
        None
    }

    /// The significant digits without trailing zeros, and the exponent n with
    /// value 0.digits × 10^n, of Rust's {:e} text.
    fn decimal(text: &str) -> (String, i32) {
        let (mantissa, exponent) = text.split_once('e').expect("exponent");
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let digits = digits.trim_end_matches('0').to_string();
        let digits = if digits.is_empty() {
            String::from("0")
        } else {
            digits
        };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
    }
}

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_formal_datatype_module_prefixes(
    paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    groups: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (({
        let ml_reference_array1 = paths.clone();
        ml_reference_array1.borrow().len() as f64
    }) != ({
        let ml_reference_array2 = groups.clone();
        ml_reference_array2.borrow().len() as f64
    })) || (({
        let ml_reference_array3 = paths.clone();
        ml_reference_array3.borrow().len() as f64
    }) > 8192f64)
    {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let common = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let total = 0f64;
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s1 =
                            self::source_ml_temporary_plan_formal_datatype_module_prefixes_loop1(
                                paths.clone(),
                                groups.clone(),
                                common.clone(),
                                total,
                                index,
                            );
                        match source_ml_source1_s1 {
                            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Done => {
                                {
                                    let index_2 = 0f64;
                                    {
                                        let source_ml_source1_s2 = self::source_ml_temporary_plan_formal_datatype_module_prefixes_loop2(paths.clone(), groups.clone(), common.clone(), index_2);
                                        match source_ml_source1_s2 {
                                            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Done => {
                                                {
                                                    let depths = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                    {
                                                        let index_3 = 0f64;
                                                        {
                                                            let source_ml_source1_s3 = self::source_ml_temporary_plan_formal_datatype_module_prefixes_loop3(groups.clone(), common.clone(), depths.clone(), index_3);
                                                            match source_ml_source1_s3 {
                                                                self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Done => {
                                                                    depths.clone()
                                                                }
                                                                self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Return(source_ml_temporary_result3) => {
                                                                    source_ml_temporary_result3.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(source_ml_temporary_result4) => {
                                                source_ml_temporary_result4.clone()
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(source_ml_temporary_result6) => {
                                source_ml_temporary_result6.clone()
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
pub fn plan_formal_datatype_module_closure(
    paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (({
        let ml_reference_array4 = paths.clone();
        ml_reference_array4.borrow().len() as f64
    }) != ({
        let ml_reference_array5 = depths.clone();
        ml_reference_array5.borrow().len() as f64
    })) || (({
        let ml_reference_array6 = paths.clone();
        ml_reference_array6.borrow().len() as f64
    }) > 8192f64)
    {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let boundaries = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::map(
                {
                    (std::rc::Rc::new(std::cell::RefCell::new(Vec::<(String, Option<f64>)>::new())))
                        .borrow()
                        .clone()
                },
                |a, b| a == b,
            )));
            {
                let budget = 0f64;
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s4 =
                            self::source_ml_temporary_plan_formal_datatype_module_closure_loop9(
                                paths.clone(),
                                depths.clone(),
                                boundaries.clone(),
                                budget,
                                index,
                            );
                        match source_ml_source1_s4 {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Done => {
                                {
                                    let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let source_ml_source1_s5 = self::source_ml_temporary_plan_formal_datatype_module_closure_loop10(paths.clone(), depths.clone(), boundaries.clone(), result.clone(), index_2);
                                            match source_ml_source1_s5 {
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done => {
                                                    result.clone()
                                                }
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(source_ml_temporary_result10) => {
                                                    source_ml_temporary_result10.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(source_ml_temporary_result12) => {
                                source_ml_temporary_result12.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result {
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Done,
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_prefixes_loop3(
    mut groups: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut common: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result {
    loop {
        return if index
            < ({
                let ml_reference_array8 = groups.clone();
                ml_reference_array8.borrow().len() as f64
            }) {
            {
                let group = {
                    let source_ml_source1_array_read_value6 = groups.clone();
                    {
                        let source_ml_source1_array_read_index7 = index;
                        {
                            let ml_reference_array9 = source_ml_source1_array_read_value6;
                            let ml_reference_position9 = source_ml_source1_array_read_index7;
                            self::ml_array::optional_at(
                                &*ml_reference_array9.borrow(),
                                self::ml_array::number_index(ml_reference_position9),
                            )
                        }
                    }
                };
                if {
                    let source_ml_source1_comparison_operand8 = group.clone();
                    {
                        let source_ml_source1_comparison_operand9 = None::<()>;
                        match (
                            source_ml_source1_comparison_operand8,
                            source_ml_source1_comparison_operand9,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } {
                    self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let source_ml_temporary_plan_formal_datatype_module_prefixes_ignored4 = {
                            let ml_method_owner10 = depths.clone();
                            let ml_method_argument10_0 = ({
                                let source_ml_source1_array_read_value10 = common.clone();
                                {
                                    let source_ml_source1_array_read_index11 =
                                        group.clone().expect("a proven present value");
                                    {
                                        let ml_reference_array11 =
                                            source_ml_source1_array_read_value10;
                                        let ml_reference_position11 =
                                            source_ml_source1_array_read_index11;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array11.borrow(),
                                            self::ml_array::number_index(ml_reference_position11),
                                        )
                                    }
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| -1f64);
                            let ml_inserted = vec![ml_method_argument10_0];
                            let mut ml_method_values10 = ml_method_owner10.borrow_mut();
                            let ml_length = ml_method_values10
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4_294_967_295usize)
                                .expect("Invalid array length");
                            ml_method_values10.extend(ml_inserted);
                            ml_length as f64
                        };
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
            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result {
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Done,
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_prefixes_loop5(
    mut path: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    mut representative: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    mut depth: f64,
) -> f64 {
    loop {
        return if ((depth
            < ({
                let source_ml_source1_nullish_field_receiver14 = (path.clone(),);
                if match (source_ml_source1_nullish_field_receiver14.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present15 =
                            ((source_ml_source1_nullish_field_receiver14.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array12 = source_ml_source1_nullish_field_present15;
                            ml_reference_array12.borrow().len() as f64
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (String::from(
                            "Cannot read properties of undefined (reading 'length')",
                        ))
                        .encode_utf16()
                        .collect(),
                        message_present: true,
                    })
                    .raise()
                }
            }))
            && (depth
                < ({
                    let source_ml_source1_nullish_field_receiver18 = (representative.clone(),);
                    if match (source_ml_source1_nullish_field_receiver18.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present19 =
                                ((source_ml_source1_nullish_field_receiver18.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            {
                                let ml_reference_array13 =
                                    source_ml_source1_nullish_field_present19;
                                ml_reference_array13.borrow().len() as f64
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (String::from(
                                "Cannot read properties of undefined (reading 'length')",
                            ))
                            .encode_utf16()
                            .collect(),
                            message_present: true,
                        })
                        .raise()
                    }
                })))
            && {
                let source_ml_source1_comparison_operand30 = {
                    let source_ml_source1_nullish_field_receiver20 = (path.clone(), depth);
                    {
                        let source_ml_source1_computed_property_key21 =
                            (source_ml_source1_nullish_field_receiver20.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver20.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present22 =
                                    ((source_ml_source1_nullish_field_receiver20.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_array_read_value23 =
                                        source_ml_source1_nullish_field_present22;
                                    {
                                        let source_ml_source1_array_read_index24 =
                                            source_ml_source1_computed_property_key21;
                                        {
                                            let ml_reference_array14 =
                                                source_ml_source1_array_read_value23;
                                            let ml_reference_position14 =
                                                source_ml_source1_array_read_index24;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array14.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position14,
                                                ),
                                            )
                                        }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (format!(
                                    "Cannot read properties of undefined (reading '{}')",
                                    self::ml_number::js_number(
                                        (source_ml_source1_nullish_field_receiver20.clone()).1
                                    )
                                ))
                                .encode_utf16()
                                .collect(),
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand31 = {
                        let source_ml_source1_nullish_field_receiver25 =
                            (representative.clone(), depth);
                        {
                            let source_ml_source1_computed_property_key26 =
                                (source_ml_source1_nullish_field_receiver25.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver25.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present27 =
                                        ((source_ml_source1_nullish_field_receiver25.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_array_read_value28 =
                                            source_ml_source1_nullish_field_present27;
                                        {
                                            let source_ml_source1_array_read_index29 =
                                                source_ml_source1_computed_property_key26;
                                            {
                                                let ml_reference_array15 =
                                                    source_ml_source1_array_read_value28;
                                                let ml_reference_position15 =
                                                    source_ml_source1_array_read_index29;
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array15.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position15,
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (format!(
                                        "Cannot read properties of undefined (reading '{}')",
                                        self::ml_number::js_number(
                                            (source_ml_source1_nullish_field_receiver25.clone()).1
                                        )
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand30,
                        source_ml_source1_comparison_operand31,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
            {
                let depth_2 = depth + 1f64;
                {
                    depth = depth_2;
                    continue;
                }
            }
        } else {
            depth
        };
    }
}
