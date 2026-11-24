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

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn read_required_number_fact(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array1 = values.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value1 = values.clone();
            {
                let source_ml_source1_array_read_index2 = index;
                { let ml_reference_array2 = source_ml_source1_array_read_value1.clone(); let ml_reference_position2 = source_ml_source1_array_read_index2; let ml_reference_projection2 = self::ml_array::optional_at(&*ml_reference_array2.borrow(), self::ml_array::number_index(ml_reference_position2)); ml_reference_projection2 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: std::rc::Rc<std::cell::RefCell<Vec<String>>>, index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array3 = values.clone(); let ml_reference_projection3 = ml_reference_array3.borrow().len() as f64; ml_reference_projection3 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value3 = values.clone();
            {
                let source_ml_source1_array_read_index4 = index;
                { let ml_reference_array4 = source_ml_source1_array_read_value3.clone(); let ml_reference_position4 = source_ml_source1_array_read_index4; let ml_reference_projection4 = self::ml_array::optional_at(&*ml_reference_array4.borrow(), self::ml_array::number_index(ml_reference_position4)); ml_reference_projection4 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array5 = values.clone(); let ml_reference_projection5 = ml_reference_array5.borrow().len() as f64; ml_reference_projection5 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value5 = values.clone();
            {
                let source_ml_source1_array_read_index6 = index;
                { let ml_reference_array6 = source_ml_source1_array_read_value5.clone(); let ml_reference_position6 = source_ml_source1_array_read_index6; let ml_reference_projection6 = self::ml_array::optional_at(&*ml_reference_array6.borrow(), self::ml_array::number_index(ml_reference_position6)); ml_reference_projection6 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_callable_data_traversal_error(depth: f64, visits: f64) -> String {
    if (((((!(self::ml_math::is_integer(depth)) || !(self::ml_math::is_integer(visits))) || (depth < (0f64))) || (visits < (0f64))) || (depth > (256f64))) || (visits > (65536f64))) {
        String::from("callable data type analysis exceeds its finite source profile")
    } else {
        String::from("")
    }
}

pub fn accept_callable_data_node(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    ((kind == "function") || { let ml_values7 = { let ml_reference_value8 = children.borrow().clone(); ml_reference_value8 }; let ml_callback7 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values7.iter().cloned().enumerate().any(|(ml_index7, ml_value7)| ml_callback7(ml_value7)) })
}

pub fn accept_callable_data_constructive_node(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    if (kind == "function") {
        ((({ let ml_reference_array9 = children.clone(); let ml_reference_projection9 = ml_reference_array9.borrow().len() as f64; ml_reference_projection9 }) > (0f64)) && { let ml_values10 = { let ml_reference_value11 = children.borrow().clone(); ml_reference_value11 }; let ml_callback10 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values10.iter().cloned().enumerate().all(|(ml_index10, ml_value10)| ml_callback10(ml_value10)) })
    } else {
        if (kind == "data") {
            ((({ let ml_reference_array12 = children.clone(); let ml_reference_projection12 = ml_reference_array12.borrow().len() as f64; ml_reference_projection12 }) > (0f64)) && { let ml_values13 = { let ml_reference_value14 = children.borrow().clone(); ml_reference_value14 }; let ml_callback13 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values13.iter().cloned().enumerate().all(|(ml_index13, ml_value13)| ml_callback13(ml_value13)) })
        } else {
            if (((kind == "constructor") || (kind == "tuple")) || (kind == "record")) {
                { let ml_values15 = { let ml_reference_value16 = children.borrow().clone(); ml_reference_value16 }; let ml_callback15 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values15.iter().cloned().enumerate().all(|(ml_index15, ml_value15)| ml_callback15(ml_value15)) }
            } else {
                (self::ml_array::includes(&{ let ml_reference_value17 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("nat"), String::from("int"), String::from("fixed"), String::from("float"), String::from("bool"), String::from("string"), String::from("unit"), String::from("output"), String::from("option"), String::from("array"), String::from("set"), String::from("map")]))).borrow().clone(); ml_reference_value17 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values18 = { let ml_reference_value19 = children.borrow().clone(); ml_reference_value19 }; let ml_callback18 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values18.iter().cloned().enumerate().all(|(ml_index18, ml_value18)| ml_callback18(ml_value18)) })
            }
        }
    }
}

pub fn read_callable_data_profile_error(callable: bool, acyclic: bool, constructive: bool) -> String {
    if !callable {
        String::from("")
    } else {
        if !acyclic {
            String::from("recursive callable data needs an independent positivity and constructive default profile")
        } else {
            if !constructive {
                String::from("callable data needs actual constructive defaults for every referenced datatype and callable result")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_callable_data_derivation(target: String, callable: bool, float: bool, output: bool, undecidable: bool) -> String {
    if (target == "Rust") {
        if callable {
            String::from("Clone")
        } else {
            if float {
                String::from("Clone, Debug, PartialEq")
            } else {
                String::from("Clone, Debug, PartialEq, Eq")
            }
        }
    } else {
        if (target == "Lean") {
            if (callable || output) {
                String::from("Inhabited")
            } else {
                if undecidable {
                    String::from("Repr, Inhabited")
                } else {
                    String::from("Repr, DecidableEq, Inhabited")
                }
            }
        } else {
            String::from("")
        }
    }
}

pub fn read_callable_data_observation_error(operation: String, callable: bool) -> String {
    if (callable && self::ml_array::includes(&{ let ml_reference_value20 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("display"), String::from("comparison"), String::from("proof")]))).borrow().clone(); ml_reference_value20 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
        String::from("callable values and aggregates retain no portable identity, structural equality, debug display or proof observation")
    } else {
        String::from("")
    }
}

pub fn read_callable_data_recursive_plan(kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, root: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    self::read_datatype_constructive_plan(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), root, false)
}

pub fn read_formal_datatype_constructive_plan(kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, root: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    self::read_datatype_constructive_plan(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), root, true)
}

pub fn read_datatype_constructive_plan(kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, root: f64, mutual: bool) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let count = { let ml_reference_array21 = kinds.clone(); let ml_reference_projection21 = ml_reference_array21.borrow().len() as f64; ml_reference_projection21 };
        if (((((((((count == (0f64)) || (count > (65536f64))) || (({ let ml_reference_array22 = edges.clone(); let ml_reference_projection22 = ml_reference_array22.borrow().len() as f64; ml_reference_projection22 }) > (65536f64))) || (({ let ml_reference_array23 = roots.clone(); let ml_reference_projection23 = ml_reference_array23.borrow().len() as f64; ml_reference_projection23 }) > count)) || (({ let ml_reference_array24 = ends.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) != count)) || (({ let ml_reference_array25 = references.clone(); let ml_reference_projection25 = ml_reference_array25.borrow().len() as f64; ml_reference_projection25 }) != count)) || !(self::ml_math::is_integer(root))) || (root < (0f64))) || (root >= count)) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete callable data graph")]))
        } else {
            {
                let previous = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s7 = self::source_ml_temporary_read_datatype_constructive_plan_loop1(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), count, previous, at);
                        match source_ml_source1_s7.clone() {
                            self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Done(previous_2) => {
                                if ((previous_2 != ({ let ml_reference_array26 = edges.clone(); let ml_reference_projection26 = ml_reference_array26.borrow().len() as f64; ml_reference_projection26 })) || { let ml_values27 = { let ml_reference_value28 = roots.borrow().clone(); ml_reference_value28 }; let ml_callback27 = { let count = count; let kinds = kinds.clone(); std::rc::Rc::new(move |value: f64| -> bool { (((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value >= count)) || ((self::read_required_text_fact(kinds.clone(), value)) != "data")) }) }; ml_values27.iter().cloned().enumerate().any(|(ml_index27, ml_value27)| ml_callback27(ml_value27)) }) {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete callable data declaration roots")]))
                                } else {
                                    {
                                        let callable = self::ml_array::includes(&{ let ml_reference_value29 = kinds.borrow().clone(); ml_reference_value29 }, &String::from("function"), 0f64, false, |a, b| a == b);
                                        if (!callable && !mutual) {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profile"), String::from("false"), String::from("false"), String::from("true"), String::from("-1")]))
                                        } else {
                                            {
                                                let work = 0f64;
                                                {
                                                    let recursive = false;
                                                    {
                                                        let declaration = 0f64;
                                                        {
                                                            let source_ml_source1_s8 = self::source_ml_temporary_read_datatype_constructive_plan_loop2(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), mutual, count, work, recursive, declaration);
                                                            match source_ml_source1_s8.clone() {
                                                                self::MlTemporaryReadDatatypeConstructivePlanLoop2Result::MlTemporaryReadDatatypeConstructivePlanLoop2Done(work_2, recursive_2) => {
                                                                    {
                                                                        let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values30 = { let ml_reference_value31 = roots.borrow().clone(); ml_reference_value31 }; let ml_callback30 = std::rc::Rc::new(move || -> bool { false }); ml_values30.iter().cloned().enumerate().map(|(ml_index30, ml_value30)| ml_callback30()).collect::<Vec<_>>() }));
                                                                        {
                                                                            let witnesses = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values32 = { let ml_reference_value33 = roots.borrow().clone(); ml_reference_value33 }; let ml_callback32 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values32.iter().cloned().enumerate().map(|(ml_index32, ml_value32)| ml_callback32()).collect::<Vec<_>>() }));
                                                                            {
                                                                                let ranks = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values34 = { let ml_reference_value35 = roots.borrow().clone(); ml_reference_value35 }; let ml_callback34 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values34.iter().cloned().enumerate().map(|(ml_index34, ml_value34)| ml_callback34()).collect::<Vec<_>>() }));
                                                                                {
                                                                                    let changed = true;
                                                                                    {
                                                                                        let rank = 0f64;
                                                                                        {
                                                                                            let source_ml_source1_s9 = self::source_ml_temporary_read_datatype_constructive_plan_loop3(kinds.clone(), ends.clone(), edges.clone(), references.clone(), roots.clone(), count, work_2, known.clone(), witnesses.clone(), ranks.clone(), changed, rank);
                                                                                            match source_ml_source1_s9.clone() {
                                                                                                self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Done => {
                                                                                                    if !({ let ml_values36 = { let ml_reference_value37 = known.borrow().clone(); ml_reference_value37 }; let ml_callback36 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values36.iter().cloned().enumerate().all(|(ml_index36, ml_value36)| ml_callback36(ml_value36)) }) {
                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data has no finite constructive witness for every referenced datatype")]))
                                                                                                    } else {
                                                                                                        if mutual {
                                                                                                            {
                                                                                                                let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("witnesses")]));
                                                                                                                {
                                                                                                                    let at_2 = 0f64;
                                                                                                                    {
                                                                                                                        let source_ml_source1_s10 = self::source_ml_temporary_read_datatype_constructive_plan_loop4(roots.clone(), witnesses.clone(), ranks.clone(), result.clone(), at_2);
                                                                                                                        match source_ml_source1_s10.clone() {
                                                                                                                            self::MlTemporaryReadDatatypeConstructivePlanLoop4Result::MlTemporaryReadDatatypeConstructivePlanLoop4Done => {
                                                                                                                                result.clone()
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            {
                                                                                                                let declaration_2 = self::read_required_number_fact(references.clone(), root);
                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profile"), String::from("true"), if recursive_2 {
                                                                                                                    String::from("true")
                                                                                                                } else {
                                                                                                                    String::from("false")
                                                                                                                }, String::from("true"), {
                                                                                                                    let source_ml_source1_string_argument11 = if (declaration_2 < (0f64)) {
                                                                                                                        (-1f64)
                                                                                                                    } else {
                                                                                                                        self::read_required_number_fact(witnesses.clone(), declaration_2)
                                                                                                                    };
                                                                                                                    self::ml_number::js_number(source_ml_source1_string_argument11)
                                                                                                                }]))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Return(source_ml_temporary_result6) => {
                                                                                                    source_ml_temporary_result6.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryReadDatatypeConstructivePlanLoop2Result::MlTemporaryReadDatatypeConstructivePlanLoop2Return(source_ml_temporary_result15) => {
                                                                    source_ml_temporary_result15.clone()
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
                            self::MlTemporaryReadDatatypeConstructivePlanLoop1Result::MlTemporaryReadDatatypeConstructivePlanLoop1Return(source_ml_temporary_result29) => {
                                source_ml_temporary_result29.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_callable_data_lean_default(name: String, constructor: String, fields: f64) -> String {
    {
        let value = constructor.clone();
        {
            let at = 0f64;
            {
                let value_2 = self::source_ml_temporary_render_callable_data_lean_default_loop31(fields, value.clone(), at);
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("instance : Inhabited ")), name)), (String::from(" := ⟨")))), value_2)), (String::from("⟩")))
            }
        }
    }
}

pub fn read_formal_datatype_lean_block_derivation(callable: bool, output: bool, undecidable: bool) -> String {
    if (callable || output) {
        String::from("")
    } else {
        if undecidable {
            String::from("Repr")
        } else {
            String::from("Repr, DecidableEq")
        }
    }
}

pub fn render_formal_datatype_rocq_mutual_block(definitions: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    format!("{}{}", (String::from("#[warnings=\"-register-all\"]\n")), (({ let ml_reference_value38 = definitions.borrow().clone(); ml_reference_value38 }).join((String::from("\n")).as_str())))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop4Result {
    MlTemporaryReadDatatypeConstructivePlanLoop4Done,
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop4(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut at: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop4Result {
    loop {
        return if (at < ({ let ml_reference_array39 = roots.clone(); let ml_reference_projection39 = ml_reference_array39.borrow().len() as f64; ml_reference_projection39 })) {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored5 = { let ml_method_owner40 = result.clone(); let ml_method_argument40_0 = {
                    let source_ml_source1_string_argument12 = self::read_required_number_fact(witnesses.clone(), at);
                    self::ml_number::js_number(source_ml_source1_string_argument12)
                }; let ml_inserted = vec![ml_method_argument40_0]; let mut ml_method_values40 = ml_method_owner40.borrow_mut(); let ml_length = ml_method_values40.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values40.extend(ml_inserted); ml_length as f64 };
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored6 = { let ml_method_owner41 = result.clone(); let ml_method_argument41_0 = {
                        let source_ml_source1_string_argument13 = self::read_required_number_fact(ranks.clone(), at);
                        self::ml_number::js_number(source_ml_source1_string_argument13)
                    }; let ml_inserted = vec![ml_method_argument41_0]; let mut ml_method_values41 = ml_method_owner41.borrow_mut(); let ml_length = ml_method_values41.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values41.extend(ml_inserted); ml_length as f64 };
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop4Result::MlTemporaryReadDatatypeConstructivePlanLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop3Result {
    MlTemporaryReadDatatypeConstructivePlanLoop3Done,
    MlTemporaryReadDatatypeConstructivePlanLoop3Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop9(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut changed: bool, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut declaration: f64, mut at: f64, mut start: f64, mut edge: f64) -> bool {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            if self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)) {
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored10 = { let ml_array_owner42 = known.clone(); let ml_array_argument42_0: f64 = declaration; let ml_array_argument42_1: f64 = 1f64; let ml_array_argument42_2: bool = true; let ml_array_length = ml_array_owner42.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument42_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument42_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument42_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner42.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let source_ml_temporary_read_datatype_constructive_plan_ignored11 = { let ml_array_owner43 = witnesses.clone(); let ml_array_argument43_0: f64 = declaration; let ml_array_argument43_1: f64 = 1f64; let ml_array_argument43_2: f64 = (edge - start); let ml_array_length = ml_array_owner43.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument43_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument43_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument43_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner43.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                        {
                            let source_ml_temporary_read_datatype_constructive_plan_ignored12 = { let ml_array_owner44 = ranks.clone(); let ml_array_argument44_0: f64 = declaration; let ml_array_argument44_1: f64 = 1f64; let ml_array_argument44_2: f64 = rank; let ml_array_length = ml_array_owner44.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument44_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument44_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument44_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner44.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                            {
                                let changed_2 = true;
                                changed_2
                            }
                        }
                    }
                }
            } else {
                {
                    let edge_2 = (edge + 1f64);
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            changed
        };
    }
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop8(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut changed: bool, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut declaration: f64) -> bool {
    loop {
        return if (declaration < ({ let ml_reference_array45 = roots.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 })) {
            if !(self::read_required_boolean_fact(known.clone(), declaration)) {
                {
                    let at = self::read_required_number_fact(roots.clone(), declaration);
                    {
                        let start = if (at == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let edge = start;
                            {
                                let changed_2 = self::source_ml_temporary_read_datatype_constructive_plan_loop9(ends.clone(), edges.clone(), known.clone(), witnesses.clone(), ranks.clone(), changed, rank, values.clone(), declaration, at, start, edge);
                                {
                                    let declaration_2 = (declaration + 1f64);
                                    {
                                        (changed, declaration) = (changed_2, declaration_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let declaration_3 = (declaration + 1f64);
                    {
                        declaration = declaration_3;
                        continue;
                    }
                }
            }
        } else {
            changed
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop7Result {
    MlTemporaryReadDatatypeConstructivePlanLoop7Done(f64),
    MlTemporaryReadDatatypeConstructivePlanLoop7Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop13Result {
    MlTemporaryReadDatatypeConstructivePlanLoop13Done,
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop13(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut edge: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop13Result {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored15 = { let ml_method_owner46 = children.clone(); let ml_method_argument46_0 = self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)); let ml_inserted = vec![ml_method_argument46_0]; let mut ml_method_values46 = ml_method_owner46.borrow_mut(); let ml_length = ml_method_values46.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values46.extend(ml_inserted); ml_length as f64 };
                {
                    let edge_2 = (edge + 1f64);
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop13Result::MlTemporaryReadDatatypeConstructivePlanLoop13Done
        };
    }
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop7(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut work: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop7Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("callable data graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let start = if (at == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let children = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let edge = start;
                                {
                                    let source_ml_source1_s14 = self::source_ml_temporary_read_datatype_constructive_plan_loop13(ends.clone(), edges.clone(), values.clone(), at, children.clone(), edge);
                                    match source_ml_source1_s14.clone() {
                                        self::MlTemporaryReadDatatypeConstructivePlanLoop13Result::MlTemporaryReadDatatypeConstructivePlanLoop13Done => {
                                            {
                                                let reference = self::read_required_number_fact(references.clone(), at);
                                                {
                                                    let kind = self::read_required_text_fact(kinds.clone(), at);
                                                    {
                                                        let source_ml_temporary_read_datatype_constructive_plan_ignored14 = { let ml_method_owner47 = values.clone(); let ml_method_argument47_0 = if (reference >= (0f64)) {
                                                            self::read_required_boolean_fact(known.clone(), reference)
                                                        } else {
                                                            if (kind == "data") {
                                                                { let ml_values48 = { let ml_reference_value49 = children.borrow().clone(); ml_reference_value49 }; let ml_callback48 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values48.iter().cloned().enumerate().any(|(ml_index48, ml_value48)| ml_callback48(ml_value48)) }
                                                            } else {
                                                                if (kind == "function") {
                                                                    ((({ let ml_reference_array50 = children.clone(); let ml_reference_projection50 = ml_reference_array50.borrow().len() as f64; ml_reference_projection50 }) > (0f64)) && self::read_required_boolean_fact(children.clone(), ({ let ml_reference_array51 = children.clone(); let ml_reference_projection51 = ml_reference_array51.borrow().len() as f64; ml_reference_projection51 } - 1f64)))
                                                                } else {
                                                                    if self::ml_array::includes(&{ let ml_reference_value52 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("array"), String::from("option"), String::from("set"), String::from("map")]))).borrow().clone(); ml_reference_value52 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                                                        true
                                                                    } else {
                                                                        self::accept_callable_data_constructive_node(kind.clone(), children.clone())
                                                                    }
                                                                }
                                                            }
                                                        }; let ml_inserted = vec![ml_method_argument47_0]; let mut ml_method_values47 = ml_method_owner47.borrow_mut(); let ml_length = ml_method_values47.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values47.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (work, at) = (work_2, at_2);
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
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Done(work)
        };
    }
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop3(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut references: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut work: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut changed: bool, mut rank: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop3Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let rank_2 = (rank + 1f64);
                    {
                        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s15 = self::source_ml_temporary_read_datatype_constructive_plan_loop7(kinds.clone(), ends.clone(), edges.clone(), references.clone(), count, work, known.clone(), values.clone(), at);
                                match source_ml_source1_s15.clone() {
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Done(work_2) => {
                                        {
                                            let declaration = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_read_datatype_constructive_plan_loop8(ends.clone(), edges.clone(), roots.clone(), known.clone(), witnesses.clone(), ranks.clone(), changed_2, rank_2, values.clone(), declaration);
                                                {
                                                    (work, changed, rank) = (work_2, changed_3, rank_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadDatatypeConstructivePlanLoop7Result::MlTemporaryReadDatatypeConstructivePlanLoop7Return(source_ml_temporary_result12) => {
                                        self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Return(source_ml_temporary_result12.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop3Result::MlTemporaryReadDatatypeConstructivePlanLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop2Result {
    MlTemporaryReadDatatypeConstructivePlanLoop2Done(f64, bool),
    MlTemporaryReadDatatypeConstructivePlanLoop2Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop17Result {
    MlTemporaryReadDatatypeConstructivePlanLoop17Done(f64, bool),
    MlTemporaryReadDatatypeConstructivePlanLoop17Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadDatatypeConstructivePlanLoop25Result {
    MlTemporaryReadDatatypeConstructivePlanLoop25Done,
}

pub fn source_ml_temporary_read_datatype_constructive_plan_loop25(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut domains: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut crossings: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut domain: bool, mut crossed: bool, mut end: f64, mut child: f64) -> self::MlTemporaryReadDatatypeConstructivePlanLoop25Result {
    loop {
        return if (child < end) {
            {
                let source_ml_temporary_read_datatype_constructive_plan_ignored26 = { let ml_method_owner53 = pending.clone(); let ml_method_argument53_0 = self::read_required_number_fact(edges.clone(), child); let ml_inserted = vec![ml_method_argument53_0]; let mut ml_method_values53 = ml_method_owner53.borrow_mut(); let ml_length = ml_method_values53.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values53.extend(ml_inserted); ml_length as f64 };
                {
                    let source_ml_temporary_read_datatype_constructive_plan_ignored27 = { let ml_method_owner54 = domains.clone(); let ml_method_argument54_0 = (domain || (((self::read_required_text_fact(kinds.clone(), at)) == "function") && (child < ((end - 1f64))))); let ml_inserted = vec![ml_method_argument54_0]; let mut ml_method_values54 = ml_method_owner54.borrow_mut(); let ml_length = ml_method_values54.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values54.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_temporary_read_datatype_constructive_plan_ignored28 = { let ml_method_owner55 = crossings.clone(); let ml_method_argument55_0 = crossed; let ml_inserted = vec![ml_method_argument55_0]; let mut ml_method_values55 = ml_method_owner55.borrow_mut(); let ml_length = ml_method_values55.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values55.extend(ml_inserted); ml_length as f64 };
                        {
                            let child_2 = (child + 1f64);
                            {
                                child = child_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadDatatypeConstructivePlanLoop25Result::MlTemporaryReadDatatypeConstructivePlanLoop25Done
        };
    }
}
