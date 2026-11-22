// Generated from complete examples/closed-source-class-composition/entry.js. Do not edit by hand.
/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript Number::toString: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
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
            let other = if digits == down { increment(&down) } else { Some(down) };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = digits.len() as i32;
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
        if x < 0.0 {
            format!("-{body}")
        } else {
            body
        }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// SameValue, as Object.is and assert.strictEqual compare: NaN equals
    /// NaN, and 0 differs from -0.
    pub fn same_value(a: f64, b: f64) -> bool {
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
        let digits = if digits.is_empty() { String::from("0") } else { digits };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_class_brand_value_0() -> () {
    ()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_class_method_0_15(source_ml_source1_class_receiver_0: std::rc::Rc<std::cell::RefCell<(std::rc::Rc<dyn Fn(String) -> String>, (), std::rc::Rc<std::cell::RefCell<Vec<String>>>)>>, name: &str) -> String {
    {
        let name_2 = self::render_source_target_identifier(name);
        {
            let candidate = self::render_source_target_identifier(&((({ let ml_reference_value1 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value1 }).0)(name_2.clone())));
            {
                let index = 2f64;
                {
                    let candidate_2 = self::source_ml_source2_temporary_ml_source1_class_method_0_15_loop1(source_ml_source1_class_receiver_0.clone(), &(name_2), candidate.clone(), index);
                    {
                        let source_ml_source2_temporary_ml_source1_class_method_0_15_ignored2 = { let ml_method_owner2 = ({ let ml_reference_value3 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value3 }).2; let ml_method_argument2_0 = candidate_2.clone(); let ml_method_key2 = ml_method_argument2_0; { let mut ml_method_values2 = ml_method_owner2.borrow_mut(); match ml_method_values2.iter().position(|ml_key| ml_key == &ml_method_key2) { Some(index) => { let _ = index; }, None => { ml_method_values2.push(ml_method_key2); } } } ml_method_owner2 };
                        candidate_2.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_class_method_0_20(source_ml_source1_class_receiver_0: std::rc::Rc<std::cell::RefCell<(std::rc::Rc<dyn Fn(String) -> String>, (), std::rc::Rc<std::cell::RefCell<Vec<String>>>)>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<dyn Fn(String) -> String>, (), std::rc::Rc<std::cell::RefCell<Vec<String>>>)>> {
    {
        let scope = self::dependency_function1(({ let ml_reference_value4 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value4 }).0, std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value5 = (std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))).borrow().clone(); ml_reference_value5 }, |a, b| a == b))));
        {
            let source_ml_source2_temporary_ml_source1_class_method_0_20_written3 = { let ml_field_owner9 = scope.clone(); let ml_field_value9 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value7 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_reference_value8 = (({ let ml_reference_value9 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value9 }).2).borrow().clone(); ml_reference_value8 }))).borrow().clone(); ml_reference_value7 }, |a, b| a == b))); ml_field_owner9.borrow_mut().2 = ml_field_value9; () };
            scope.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(ident: std::rc::Rc<dyn Fn(String) -> String>, reserved: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<dyn Fn(String) -> String>, (), std::rc::Rc<std::cell::RefCell<Vec<String>>>)>> {
    {
        let source_ml_source1_class_assigned_field_value_0_9 = ident.clone();
        {
            let source_ml_source1_class_assigned_field_value_0_12 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value10 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_reference_value11 = reserved.borrow().clone(); ml_reference_value11 }))).borrow().clone(); ml_reference_value10 }, |a, b| a == b)));
            {
                let source_ml_source1_class_receiver_0 = std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_field1 = self::source_ml_source1_class_brand_value_0();
                    {
                        let source_ml_source3_field2 = source_ml_source1_class_assigned_field_value_0_9.clone();
                        {
                            let source_ml_source3_field3 = source_ml_source1_class_assigned_field_value_0_12.clone();
                            (source_ml_source3_field2.clone(), source_ml_source3_field1, source_ml_source3_field3.clone())
                        }
                    }
                }));
                source_ml_source1_class_receiver_0.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_private_identifier(name: &str) -> bool {
    name.starts_with("ml_")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_source_target_identifier(name: &str) -> String {
    if self::accept_source_private_identifier(name) {
        format!("{}{}", (String::from("source_")), name.to_owned())
    } else {
        name.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn identity(name: &str) -> String {
    name.to_owned()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn run(name: &str) -> String {
    {
        let root = self::dependency_function1(std::rc::Rc::new(self::identity), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::set({ let ml_reference_value12 = (std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))).borrow().clone(); ml_reference_value12 }, |a, b| a == b))));
        {
            let first = {
                let source_ml_source3_class_receiver64 = root.clone();
                self::source_ml_source1_class_method_0_15(source_ml_source3_class_receiver64.clone(), name)
            };
            {
                let child = {
                    let source_ml_source3_class_receiver65 = root.clone();
                    self::source_ml_source1_class_method_0_20(source_ml_source3_class_receiver65.clone())
                };
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", first, (String::from("/")))), ({
                    let source_ml_source3_class_receiver66 = child.clone();
                    self::source_ml_source1_class_method_0_15(source_ml_source3_class_receiver66.clone(), name)
                }))), (String::from("/")))), ({
                    let source_ml_source3_class_receiver67 = root.clone();
                    self::source_ml_source1_class_method_0_15(source_ml_source3_class_receiver67.clone(), name)
                }))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source2_temporary_ml_source1_class_method_0_15_loop1(mut source_ml_source1_class_receiver_0: std::rc::Rc<std::cell::RefCell<(std::rc::Rc<dyn Fn(String) -> String>, (), std::rc::Rc<std::cell::RefCell<Vec<String>>>)>>, name: &str, mut candidate: String, mut index: f64) -> String {
    loop {
        return if { let ml_method_owner13 = ({ let ml_reference_value14 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value14 }).2; let ml_method_argument13_0 = candidate.clone(); let ml_method_key13 = ml_method_argument13_0; let ml_method_values13 = ml_method_owner13.borrow(); ml_method_values13.iter().position(|ml_key| ml_key == &ml_method_key13).is_some() } {
            {
                let candidate_2 = self::render_source_target_identifier(&((({ let ml_reference_value15 = source_ml_source1_class_receiver_0.borrow().clone(); ml_reference_value15 }).0)(format!("{}{}", (format!("{}{}", name.to_owned(), (String::from("_")))), (self::ml_number::js_number(index))))));
                {
                    let index_2 = (index + 1f64);
                    {
                        (candidate, index) = (candidate_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            candidate.clone()
        };
    }
}
