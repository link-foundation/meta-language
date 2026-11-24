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

pub fn read_rust_primitive_owner_path(segments: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        {
            let owner = String::from("");
            {
                let owner_4 = if ((({ let ml_reference_array7 = segments.clone(); let ml_reference_projection7 = ml_reference_array7.borrow().len() as f64; ml_reference_projection7 }) == (2f64)) && !absolute) {
                    {
                        let owner_2 = ({
                            let source_ml_source1_array_read_value7 = segments.clone();
                            {
                                let source_ml_source1_array_read_index8 = 0f64;
                                { let ml_reference_array8 = source_ml_source1_array_read_value7.clone(); let ml_reference_position8 = source_ml_source1_array_read_index8; let ml_reference_projection8 = self::ml_array::optional_at(&*ml_reference_array8.borrow(), self::ml_array::number_index(ml_reference_position8)); ml_reference_projection8 }
                            }
                        }).clone().unwrap_or_else(|| String::from(""));
                        owner_2.clone()
                    }
                } else {
                    if (((({ let ml_reference_array9 = segments.clone(); let ml_reference_projection9 = ml_reference_array9.borrow().len() as f64; ml_reference_projection9 }) == (4f64)) && ({
                        let source_ml_source1_comparison_operand11 = {
                            let source_ml_source1_array_read_value9 = segments.clone();
                            {
                                let source_ml_source1_array_read_index10 = 0f64;
                                { let ml_reference_array10 = source_ml_source1_array_read_value9.clone(); let ml_reference_position10 = source_ml_source1_array_read_index10; let ml_reference_projection10 = self::ml_array::optional_at(&*ml_reference_array10.borrow(), self::ml_array::number_index(ml_reference_position10)); ml_reference_projection10 }
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand12 = String::from("std");
                            match (source_ml_source1_comparison_operand11.clone(), source_ml_source1_comparison_operand12.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } || {
                        let source_ml_source1_comparison_operand15 = {
                            let source_ml_source1_array_read_value13 = segments.clone();
                            {
                                let source_ml_source1_array_read_index14 = 0f64;
                                { let ml_reference_array11 = source_ml_source1_array_read_value13.clone(); let ml_reference_position11 = source_ml_source1_array_read_index14; let ml_reference_projection11 = self::ml_array::optional_at(&*ml_reference_array11.borrow(), self::ml_array::number_index(ml_reference_position11)); ml_reference_projection11 }
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand16 = String::from("core");
                            match (source_ml_source1_comparison_operand15.clone(), source_ml_source1_comparison_operand16.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    })) && {
                        let source_ml_source1_comparison_operand19 = {
                            let source_ml_source1_array_read_value17 = segments.clone();
                            {
                                let source_ml_source1_array_read_index18 = 1f64;
                                { let ml_reference_array12 = source_ml_source1_array_read_value17.clone(); let ml_reference_position12 = source_ml_source1_array_read_index18; let ml_reference_projection12 = self::ml_array::optional_at(&*ml_reference_array12.borrow(), self::ml_array::number_index(ml_reference_position12)); ml_reference_projection12 }
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand20 = String::from("primitive");
                            match (source_ml_source1_comparison_operand19.clone(), source_ml_source1_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    }) {
                        {
                            let owner_3 = ({
                                let source_ml_source1_array_read_value21 = segments.clone();
                                {
                                    let source_ml_source1_array_read_index22 = 2f64;
                                    { let ml_reference_array13 = source_ml_source1_array_read_value21.clone(); let ml_reference_position13 = source_ml_source1_array_read_index22; let ml_reference_projection13 = self::ml_array::optional_at(&*ml_reference_array13.borrow(), self::ml_array::number_index(ml_reference_position13)); ml_reference_projection13 }
                                }
                            }).clone().unwrap_or_else(|| String::from(""));
                            owner_3.clone()
                        }
                    } else {
                        owner.clone()
                    }
                };
                if self::ml_array::includes(&{ let ml_reference_value14 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize")]))).borrow().clone(); ml_reference_value14 }, &owner_4.clone(), 0f64, false, |a, b| a == b) {
                    owner_4.clone()
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn accept_rust_source_signature_model(model: String) -> bool {
    ((model == "") || (model == "Rust source function signature 1"))
}

pub fn accept_rust_source_signature_wrapper(wrapper: String) -> bool {
    ((wrapper == "reference") || (wrapper == "box"))
}

pub fn accept_rust_source_signature_owner_kind(kind: String) -> bool {
    ((((((kind == "primitive") || (kind == "declaration")) || (kind == "library")) || (kind == "tuple")) || (kind == "unit")) || (kind == "slice"))
}

pub fn accept_rust_source_signature_catalog_owner(kind: String) -> bool {
    ((((kind == "data") || (kind == "module")) || (kind == "externalCrate")) || (kind == "ctor"))
}

pub fn accept_rust_source_signature_primitive(kind: String, path: String, bits: String, signed: bool) -> bool {
    if (path == "f64") {
        (kind == "float")
    } else {
        if (path == "bool") {
            (kind == "bool")
        } else {
            if (path == "str") {
                (kind == "string")
            } else {
                if (path == "i8") {
                    (((kind == "fixed") && (bits == "8")) && (signed == (true)))
                } else {
                    if (path == "u8") {
                        (((kind == "fixed") && (bits == "8")) && (signed == (false)))
                    } else {
                        if (path == "i16") {
                            (((kind == "fixed") && (bits == "16")) && (signed == (true)))
                        } else {
                            if (path == "u16") {
                                (((kind == "fixed") && (bits == "16")) && (signed == (false)))
                            } else {
                                if (path == "i32") {
                                    (((kind == "fixed") && (bits == "32")) && (signed == (true)))
                                } else {
                                    if (path == "u32") {
                                        (((kind == "fixed") && (bits == "32")) && (signed == (false)))
                                    } else {
                                        if (path == "i64") {
                                            (((kind == "fixed") && (bits == "64")) && (signed == (true)))
                                        } else {
                                            if (path == "u64") {
                                                (((kind == "fixed") && (bits == "64")) && (signed == (false)))
                                            } else {
                                                if (path == "i128") {
                                                    (((kind == "fixed") && (bits == "128")) && (signed == (true)))
                                                } else {
                                                    if (path == "u128") {
                                                        (((kind == "fixed") && (bits == "128")) && (signed == (false)))
                                                    } else {
                                                        if (path == "usize") {
                                                            (((kind == "fixed") && (bits == "64")) && (signed == (false)))
                                                        } else {
                                                            if (path == "isize") {
                                                                (((kind == "fixed") && (bits == "64")) && (signed == (true)))
                                                            } else {
                                                                false
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

pub fn read_rust_primitive_selection_call_role(path: String, ordinary: bool, shadowed: bool) -> String {
    if (ordinary && !shadowed) {
        if (path == "std::cmp::min") {
            String::from("min")
        } else {
            if (path == "std::cmp::max") {
                String::from("max")
            } else {
                String::from("")
            }
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_primitive_selection_comparison(method: String, ordinary: bool) -> String {
    if ordinary {
        if (method == "min") {
            String::from("le")
        } else {
            if (method == "max") {
                String::from("gt")
            } else {
                String::from("")
            }
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_primitive_selection_condition(method: String, kind: String) -> String {
    if (kind == "bool") {
        if (method == "min") {
            String::from("not")
        } else {
            if (method == "max") {
                String::from("identity")
            } else {
                String::from("")
            }
        }
    } else {
        if (kind == "fixed") {
            self::read_rust_primitive_selection_comparison(method.clone(), true)
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_primitive_selection_receiver(kind: String, bits: String, signed: bool, owner: String, path: String, children: f64, proven: bool, layers: f64) -> bool {
    (((((((kind == "fixed") || (kind == "bool")) && (owner == "primitive")) && (children == (0f64))) && proven) && (layers == (0f64))) && self::accept_rust_source_signature_primitive(kind.clone(), path.clone(), bits.clone(), signed))
}

pub fn accept_rust_primitive_selection_argument(expected: String, bits: String, signed: bool, owner: String, actual: String, actual_bits: String, actual_signed: bool, actual_owner: String, layers: f64, proven: bool, known: bool, literal: bool) -> bool {
    if (layers != (0f64)) {
        false
    } else {
        if (((literal && (expected == "fixed")) && (actual == "")) && (actual_owner == "")) {
            true
        } else {
            (((((known && proven) && (expected == actual)) && (bits == actual_bits)) && (signed == actual_signed)) && (owner == actual_owner))
        }
    }
}

pub fn accept_rust_primitive_from_method(owner: String, method: String) -> bool {
    ((method == "from") && self::ml_array::includes(&{ let ml_reference_value15 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize"), String::from("f64")]))).borrow().clone(); ml_reference_value15 }, &owner.clone(), 0f64, false, |a, b| a == b))
}

pub fn read_rust_primitive_from_owner(segments: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        if ((((({ let ml_reference_array16 = segments.clone(); let ml_reference_projection16 = ml_reference_array16.borrow().len() as f64; ml_reference_projection16 }) == (2f64)) && !absolute) && {
            let source_ml_source1_comparison_operand25 = {
                let source_ml_source1_array_read_value23 = segments.clone();
                {
                    let source_ml_source1_array_read_index24 = 0f64;
                    { let ml_reference_array17 = source_ml_source1_array_read_value23.clone(); let ml_reference_position17 = source_ml_source1_array_read_index24; let ml_reference_projection17 = self::ml_array::optional_at(&*ml_reference_array17.borrow(), self::ml_array::number_index(ml_reference_position17)); ml_reference_projection17 }
                }
            };
            {
                let source_ml_source1_comparison_operand26 = String::from("f64");
                match (source_ml_source1_comparison_operand25.clone(), source_ml_source1_comparison_operand26.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) || ((((({ let ml_reference_array18 = segments.clone(); let ml_reference_projection18 = ml_reference_array18.borrow().len() as f64; ml_reference_projection18 }) == (4f64)) && ({
            let source_ml_source1_comparison_operand29 = {
                let source_ml_source1_array_read_value27 = segments.clone();
                {
                    let source_ml_source1_array_read_index28 = 0f64;
                    { let ml_reference_array19 = source_ml_source1_array_read_value27.clone(); let ml_reference_position19 = source_ml_source1_array_read_index28; let ml_reference_projection19 = self::ml_array::optional_at(&*ml_reference_array19.borrow(), self::ml_array::number_index(ml_reference_position19)); ml_reference_projection19 }
                }
            };
            {
                let source_ml_source1_comparison_operand30 = String::from("std");
                match (source_ml_source1_comparison_operand29.clone(), source_ml_source1_comparison_operand30.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } || {
            let source_ml_source1_comparison_operand33 = {
                let source_ml_source1_array_read_value31 = segments.clone();
                {
                    let source_ml_source1_array_read_index32 = 0f64;
                    { let ml_reference_array20 = source_ml_source1_array_read_value31.clone(); let ml_reference_position20 = source_ml_source1_array_read_index32; let ml_reference_projection20 = self::ml_array::optional_at(&*ml_reference_array20.borrow(), self::ml_array::number_index(ml_reference_position20)); ml_reference_projection20 }
                }
            };
            {
                let source_ml_source1_comparison_operand34 = String::from("core");
                match (source_ml_source1_comparison_operand33.clone(), source_ml_source1_comparison_operand34.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) && {
            let source_ml_source1_comparison_operand37 = {
                let source_ml_source1_array_read_value35 = segments.clone();
                {
                    let source_ml_source1_array_read_index36 = 1f64;
                    { let ml_reference_array21 = source_ml_source1_array_read_value35.clone(); let ml_reference_position21 = source_ml_source1_array_read_index36; let ml_reference_projection21 = self::ml_array::optional_at(&*ml_reference_array21.borrow(), self::ml_array::number_index(ml_reference_position21)); ml_reference_projection21 }
                }
            };
            {
                let source_ml_source1_comparison_operand38 = String::from("primitive");
                match (source_ml_source1_comparison_operand37.clone(), source_ml_source1_comparison_operand38.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && {
            let source_ml_source1_comparison_operand41 = {
                let source_ml_source1_array_read_value39 = segments.clone();
                {
                    let source_ml_source1_array_read_index40 = 2f64;
                    { let ml_reference_array22 = source_ml_source1_array_read_value39.clone(); let ml_reference_position22 = source_ml_source1_array_read_index40; let ml_reference_projection22 = self::ml_array::optional_at(&*ml_reference_array22.borrow(), self::ml_array::number_index(ml_reference_position22)); ml_reference_projection22 }
                }
            };
            {
                let source_ml_source1_comparison_operand42 = String::from("f64");
                match (source_ml_source1_comparison_operand41.clone(), source_ml_source1_comparison_operand42.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) {
            String::from("f64")
        } else {
            self::read_rust_primitive_owner_path(segments.clone(), ordinary, shadowed, absolute)
        }
    }
}

pub fn read_rust_primitive_from_literal_owner(form: String, typed: bool, kind: String) -> String {
    if ((form != "numeral") && (form != "negative numeral")) {
        String::from("")
    } else {
        if (kind == "float") {
            String::from("f64")
        } else {
            if !typed {
                String::from("i32")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_primitive_from_source(kind: String, bits: String, signed: bool, owner: String, path: String, children: f64, proven: bool, layers: f64) -> bool {
    (((((((((kind == "float") && (bits == "")) && !signed) && (owner == "primitive")) && (path == "f64")) && (children == (0f64))) && proven) && (layers == (0f64))) || self::accept_rust_primitive_selection_receiver(kind.clone(), bits.clone(), signed, owner.clone(), path.clone(), children, proven, layers))
}

pub fn plan_rust_primitive_from(target: String, source: String) -> String {
    if (!(self::accept_rust_primitive_from_method(target.clone(), String::from("from"))) || !(self::accept_rust_primitive_from_method(source.clone(), String::from("from")))) {
        String::from("")
    } else {
        if (target == source) {
            String::from("identity")
        } else {
            if (((source == "bool") && (target != "bool")) && (target != "f64")) {
                String::from("boolean integer")
            } else {
                if ((target == "f64") && self::ml_array::includes(&{ let ml_reference_value23 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i8"), String::from("i16"), String::from("i32"), String::from("u8"), String::from("u16"), String::from("u32")]))).borrow().clone(); ml_reference_value23 }, &source.clone(), 0f64, false, |a, b| a == b)) {
                    String::from("floating exact")
                } else {
                    {
                        let admitted = (((((((((source == "u8") && self::ml_array::includes(&{ let ml_reference_value24 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u16"), String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value24 }, &target.clone(), 0f64, false, |a, b| a == b)) || ((source == "u16") && self::ml_array::includes(&{ let ml_reference_value25 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i32"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value25 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u32") && self::ml_array::includes(&{ let ml_reference_value26 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u64"), String::from("u128"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value26 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u64") && self::ml_array::includes(&{ let ml_reference_value27 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u128"), String::from("i128")]))).borrow().clone(); ml_reference_value27 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i8") && self::ml_array::includes(&{ let ml_reference_value28 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value28 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i16") && self::ml_array::includes(&{ let ml_reference_value29 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value29 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i32") && self::ml_array::includes(&{ let ml_reference_value30 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value30 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i64") && (target == "i128")));
                        if admitted {
                            String::from("integer exact")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_source_array_owner(owner: String, shadowed: bool) -> bool {
    (!shadowed && ((owner == "Vec") || (owner == "std::vec::Vec")))
}

pub fn accept_rust_source_array_element(kind: String, indirected: bool) -> bool {
    (!indirected && (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")) || (kind == "string")))
}

pub fn accept_rust_source_array_coercion(actual: String, expected: String, borrowed: bool) -> bool {
    ((actual == expected) || ((borrowed && (actual == "owned vector")) && (expected == "immutable slice")))
}

pub fn accept_rust_source_array_result_model(model: String) -> bool {
    ((model == "") || (model == "immutable fresh scalar vector results 1"))
}

pub fn accept_rust_source_array_result(model: String, kind: String, owned: bool, fresh: bool) -> bool {
    ((((model == "immutable fresh scalar vector results 1") && owned) && fresh) && ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")))
}

pub fn accept_rust_fresh_array_result_tail(kind: String, constructor: bool, children: bool) -> bool {
    (((kind == "array") && constructor) || (((kind == "let") || (kind == "if")) && children))
}

pub fn accept_rust_retained_source_signature_model(model: String) -> bool {
    (model == "Rust source function signature 2")
}

pub fn select_rust_retained_source_signature_model(model: String, retain: bool) -> String {
    if (model != "") {
        model.clone()
    } else {
        if retain {
            String::from("Rust source function signature 2")
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_retained_function_owner(model: String, owner: String, type_: String, paths: f64, children: f64, parameters: f64, reference: bool) -> bool {
    ((((((((self::accept_rust_retained_source_signature_model(model.clone()) && (owner == "function")) && (type_ == "function")) && (paths == (0f64))) && self::ml_math::is_integer(parameters)) && (parameters >= (0f64))) && (parameters <= (100000f64))) && (children == ((parameters + 1f64)))) && !reference)
}

pub fn accept_rust_nominal_clone_inherent_override(owner: String, selected_owner: String, mode: f64) -> bool {
    (((owner != "") && (owner == selected_owner)) && (mode > (0f64)))
}

pub fn read_rust_nominal_ownership_leaf(kind: String, bits: String, signed: bool, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, copy: bool, owner: String, path: String, owner_paths: f64, arguments_: f64, model: String, pointer_header: bool, parameters: f64, scope: bool, closed: bool, element: String) -> String {
    if (!scope || !closed) {
        String::from("unknown")
    } else {
        if { let ml_value = { let ml_reference_array32 = wrappers.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }; ml_value != 0f64 && !ml_value.is_nan() } {
            if { let ml_values33 = { let ml_reference_value34 = wrappers.borrow().clone(); ml_reference_value34 }; let ml_callback33 = std::rc::Rc::new(move |layer: String| -> bool { (layer == "reference") }); ml_values33.iter().cloned().enumerate().all(|(ml_index33, ml_value33)| ml_callback33(ml_value33)) } {
                String::from("copy")
            } else {
                String::from("unknown")
            }
        } else {
            if ((proven && copy) && self::accept_rust_primitive_from_source(kind.clone(), bits.clone(), signed, owner.clone(), path.clone(), arguments_, true, 0f64)) {
                String::from("copy")
            } else {
                if ((((((kind == "unit") && proven) && copy) && (owner == "unit")) && (path == "")) && (arguments_ == (0f64))) {
                    String::from("copy")
                } else {
                    if (((kind == "function") && ((proven && copy) || pointer_header)) && self::accept_rust_retained_function_owner(String::from("Rust source function signature 2"), owner.clone(), kind.clone(), owner_paths, arguments_, parameters, false)) {
                        String::from("copy")
                    } else {
                        if ((((((kind == "string") && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value35 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::string::String"), String::from("alloc::string::String")]))).borrow().clone(); ml_reference_value35 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (0f64))) && (model == "owned string")) {
                            String::from("string")
                        } else {
                            if (((((((kind == "array") && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value36 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value36 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) && self::accept_rust_source_array_element(element.clone(), false)) {
                                String::from("vector")
                            } else {
                                if ((kind == "tuple") && (owner == "tuple")) {
                                    String::from("tuple")
                                } else {
                                    if ((kind == "named") && (owner == "declaration")) {
                                        String::from("named")
                                    } else {
                                        String::from("unknown")
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

pub fn read_rust_nominal_ownership_sum_node(owner: String, arguments_: f64, provisional: bool, proven: bool, scope: bool) -> String {
    if (scope && (proven || provisional)) {
        if ((owner == "std::option::Option") && (arguments_ == (1f64))) {
            String::from("option")
        } else {
            if ((owner == "std::result::Result") && (arguments_ == (2f64))) {
                String::from("result")
            } else {
                String::from("unknown")
            }
        }
    } else {
        String::from("unknown")
    }
}

pub fn accept_rust_nominal_ownership_declaration_owner(kind: String, path: String, arguments_: f64, declared: String) -> bool {
    ((((kind == "declaration") && (declared != "")) && (path == declared)) && (arguments_ == (0f64)))
}

pub fn read_rust_nominal_ownership_graph_error(nodes: f64, edges: f64, depth: f64) -> String {
    if ((((((((!(self::ml_math::is_integer(nodes)) || !(self::ml_math::is_integer(edges))) || !(self::ml_math::is_integer(depth))) || (nodes < (0f64))) || (edges < (0f64))) || (depth < (0f64))) || (nodes > (65536f64))) || (edges > (131072f64))) || (depth > (256f64))) {
        String::from("the actual nominal ownership field graph exceeds its finite source profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_nominal_ownership_node_role(kind: String, roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if (kind == "copy") {
        String::from("copy")
    } else {
        if ((kind == "string") || (kind == "standard collection")) {
            String::from("owned")
        } else {
            if (kind == "vector") {
                if ((({ let ml_reference_array37 = roles.clone(); let ml_reference_projection37 = ml_reference_array37.borrow().len() as f64; ml_reference_projection37 }) == (1f64)) && ({
                    let source_ml_source1_comparison_operand45 = {
                        let source_ml_source1_array_read_value43 = roles.clone();
                        {
                            let source_ml_source1_array_read_index44 = 0f64;
                            { let ml_reference_array38 = source_ml_source1_array_read_value43.clone(); let ml_reference_position38 = source_ml_source1_array_read_index44; let ml_reference_projection38 = self::ml_array::optional_at(&*ml_reference_array38.borrow(), self::ml_array::number_index(ml_reference_position38)); ml_reference_projection38 }
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand46 = String::from("copy");
                        match (source_ml_source1_comparison_operand45.clone(), source_ml_source1_comparison_operand46.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                } || {
                    let source_ml_source1_comparison_operand49 = {
                        let source_ml_source1_array_read_value47 = roles.clone();
                        {
                            let source_ml_source1_array_read_index48 = 0f64;
                            { let ml_reference_array39 = source_ml_source1_array_read_value47.clone(); let ml_reference_position39 = source_ml_source1_array_read_index48; let ml_reference_projection39 = self::ml_array::optional_at(&*ml_reference_array39.borrow(), self::ml_array::number_index(ml_reference_position39)); ml_reference_projection39 }
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand50 = String::from("owned");
                        match (source_ml_source1_comparison_operand49.clone(), source_ml_source1_comparison_operand50.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                })) {
                    String::from("owned")
                } else {
                    String::from("")
                }
            } else {
                if (((kind == "option") && (({ let ml_reference_array40 = roles.clone(); let ml_reference_projection40 = ml_reference_array40.borrow().len() as f64; ml_reference_projection40 }) != (1f64))) || ((kind == "result") && (({ let ml_reference_array41 = roles.clone(); let ml_reference_projection41 = ml_reference_array41.borrow().len() as f64; ml_reference_projection41 }) != (2f64)))) {
                    String::from("")
                } else {
                    if (!(self::ml_array::includes(&{ let ml_reference_value42 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("constructor"), String::from("tuple"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value42 }, &kind.clone(), 0f64, false, |a, b| a == b)) || { let ml_values43 = { let ml_reference_value44 = roles.borrow().clone(); ml_reference_value44 }; let ml_callback43 = std::rc::Rc::new(move |role: String| -> bool { ((role != "copy") && (role != "owned")) }); ml_values43.iter().cloned().enumerate().any(|(ml_index43, ml_value43)| ml_callback43(ml_value43)) }) {
                        String::from("")
                    } else {
                        {
                            let owned = self::ml_array::includes(&{ let ml_reference_value45 = roles.borrow().clone(); ml_reference_value45 }, &String::from("owned"), 0f64, false, |a, b| a == b);
                            if ((kind == "tuple") && owned) {
                                String::from("")
                            } else {
                                if owned {
                                    String::from("owned")
                                } else {
                                    String::from("copy")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
