// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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

pub fn plan_rust_nominal_ownership_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let count = { let ml_reference_array46 = kinds.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 };
        {
            let size = { let ml_reference_array47 = owners.clone(); let ml_reference_projection47 = ml_reference_array47.borrow().len() as f64; ml_reference_projection47 };
            if (((((((size > (4096f64)) || (({ let ml_reference_array48 = copies.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }) != size)) || (({ let ml_reference_array49 = clones.clone(); let ml_reference_projection49 = ml_reference_array49.borrow().len() as f64; ml_reference_projection49 }) != size)) || (({ let ml_reference_array50 = roots.clone(); let ml_reference_projection50 = ml_reference_array50.borrow().len() as f64; ml_reference_projection50 }) != size)) || (({ let ml_reference_array51 = ends.clone(); let ml_reference_projection51 = ml_reference_array51.borrow().len() as f64; ml_reference_projection51 }) != count)) || (({ let ml_reference_array52 = targets.clone(); let ml_reference_projection52 = ml_reference_array52.borrow().len() as f64; ml_reference_projection52 }) != count)) || ((self::read_rust_nominal_ownership_graph_error(count, { let ml_reference_array53 = edges.clone(); let ml_reference_projection53 = ml_reference_array53.borrow().len() as f64; ml_reference_projection53 }, 0f64)) != "")) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")]))
            } else {
                {
                    let previous = 0f64;
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s51 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(ends.clone(), edges.clone(), targets.clone(), count, size, previous, at);
                            match source_ml_source1_s51.clone() {
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(previous_2) => {
                                    if (previous_2 != ({ let ml_reference_array54 = edges.clone(); let ml_reference_projection54 = ml_reference_array54.borrow().len() as f64; ml_reference_projection54 })) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership edges")]))
                                    } else {
                                        {
                                            let owner = 0f64;
                                            {
                                                let source_ml_source1_s52 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), roots.clone(), count, size, owner);
                                                match source_ml_source1_s52.clone() {
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done => {
                                                        {
                                                            let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values55 = { let ml_reference_value56 = owners.borrow().clone(); ml_reference_value56 }; let ml_callback55 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> String { if self::read_required_boolean_fact(copies.clone(), index) {
                                                                String::from("copy")
                                                            } else {
                                                                String::from("")
                                                            } }) }; ml_values55.iter().cloned().enumerate().map(|(ml_index55, ml_value55)| ml_callback55(ml_value55, (ml_index55 as f64))).collect::<Vec<_>>() }));
                                                            {
                                                                let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values57 = { let ml_reference_value58 = owners.borrow().clone(); ml_reference_value58 }; let ml_callback57 = { let copies = copies.clone(); std::rc::Rc::new(move |x_: String, index: f64| -> bool { self::read_required_boolean_fact(copies.clone(), index) }) }; ml_values57.iter().cloned().enumerate().map(|(ml_index57, ml_value57)| ml_callback57(ml_value57, (ml_index57 as f64))).collect::<Vec<_>>() }));
                                                                {
                                                                    let changed = true;
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let source_ml_source1_s53 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, known.clone(), implemented.clone(), changed, work);
                                                                            match source_ml_source1_s53.clone() {
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done => {
                                                                                    {
                                                                                        let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                        {
                                                                                            let owner_2 = 0f64;
                                                                                            {
                                                                                                let result_2 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(size, known.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                result_2.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result4) => {
                                                                                    source_ml_temporary_result4.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(source_ml_temporary_result10) => {
                                                        source_ml_temporary_result10.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result10_2) => {
                                    source_ml_temporary_result10_2.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_nominal_clone_error(ordinary: bool, role: String, actual_clone: bool, wrappers: f64, borrowed: bool, tracking: bool, available: bool, produced: bool) -> String {
    if (!ordinary || !actual_clone) {
        String::from("nominal Clone needs its actual complete derived implementation")
    } else {
        if ((wrappers != (0f64)) || borrowed) {
            String::from("nominal Clone needs an actual direct source value")
        } else {
            if ((role == "copy") || (((role == "owned") && tracking) && (available || produced))) {
                String::from("")
            } else {
                String::from("nominal Clone needs an available value or actual producer independently of its type")
            }
        }
    }
}

pub fn read_rust_nominal_record_update_error(role: String) -> String {
    if (role == "owned") {
        String::from("owned nominal record updates need their actual partial-field move interpretation")
    } else {
        String::from("")
    }
}

pub fn read_rust_nominal_clone_implementation_error(implemented: bool) -> String {
    if implemented {
        String::from("")
    } else {
        String::from("Clone needs each actual nominal Clone implementation independently of ownership")
    }
}

pub fn plan_rust_nominal_pattern_transfers(identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, deferred: bool) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let size = { let ml_reference_array59 = identities.clone(); let ml_reference_projection59 = ml_reference_array59.borrow().len() as f64; ml_reference_projection59 };
        if ((((size > (65536f64)) || (({ let ml_reference_array60 = fields.clone(); let ml_reference_projection60 = ml_reference_array60.borrow().len() as f64; ml_reference_projection60 }) != size)) || (({ let ml_reference_array61 = owned.clone(); let ml_reference_projection61 = ml_reference_array61.borrow().len() as f64; ml_reference_projection61 }) != size)) || (({ let ml_reference_array62 = produced.clone(); let ml_reference_projection62 = ml_reference_array62.borrow().len() as f64; ml_reference_projection62 }) != size)) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete nominal pattern transfer facts")]))
        } else {
            {
                let plan = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("transfers")]));
                {
                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s54 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop12(identities.clone(), fields.clone(), owned.clone(), produced.clone(), deferred, size, plan.clone(), seen.clone(), index);
                            match source_ml_source1_s54.clone() {
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Done(plan_2) => {
                                    plan_2.clone()
                                }
                                self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(source_ml_temporary_result12) => {
                                    source_ml_temporary_result12.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_nominal_pattern_alternative_error(same: bool) -> String {
    if same {
        String::from("")
    } else {
        String::from("every alternative needs the same actual nominal field moves, owners and parent lineage")
    }
}

pub fn accept_rust_nominal_vector_element_leaf(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    ((({ let ml_reference_array63 = wrappers.clone(); let ml_reference_projection63 = ml_reference_array63.borrow().len() as f64; ml_reference_projection63 }) == (0f64)) && self::accept_rust_source_array_element(kind.clone(), false))
}

pub fn plan_rust_nominal_vector_element_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    {
        let checked = self::plan_rust_nominal_ownership_owners(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone());
        if (((self::read_required_text_fact(checked.clone(), 0f64)) != "profiles") || (({ let ml_reference_array64 = leaves.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 }) != ({ let ml_reference_array65 = kinds.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 }))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()))
        } else {
            {
                let known = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values66 = { let ml_reference_value67 = owners.borrow().clone(); ml_reference_value67 }; let ml_callback66 = std::rc::Rc::new(move || -> bool { false }); ml_values66.iter().cloned().enumerate().map(|(ml_index66, ml_value66)| ml_callback66()).collect::<Vec<_>>() }));
                {
                    let changed = true;
                    {
                        let work = 0f64;
                        {
                            let source_ml_source1_s55 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop14(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), known.clone(), changed, work);
                            match source_ml_source1_s55.clone() {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done => {
                                    known.clone()
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(source_ml_temporary_result14) => {
                                    source_ml_temporary_result14.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_nominal_vector_element(profile: String, clone: bool, fields: bool) -> bool {
    (((profile == "copy") && clone) && fields)
}

pub fn read_rust_nominal_vector_element_admission(legacy: bool, nominal: bool, header: bool, collecting: bool, complete: bool, kind: String, wrappers: f64) -> String {
    if (legacy || nominal) {
        String::from("admit")
    } else {
        if ((((header && collecting) && !complete) && (kind == "named")) && (wrappers == (0f64))) {
            String::from("defer")
        } else {
            String::from("refuse")
        }
    }
}

pub fn read_rust_nominal_constructor_error(actual: bool, fields: f64, arguments_: f64) -> String {
    if (((((actual && self::ml_math::is_integer(fields)) && self::ml_math::is_integer(arguments_)) && (fields >= (0f64))) && (fields <= (65536f64))) && (fields == arguments_)) {
        String::from("")
    } else {
        String::from("nominal construction needs its actual constructor owner and exact finite field arity")
    }
}

pub fn read_rust_nominal_ownership_vector_template_node(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, element: String, scope: bool) -> String {
    if ((((((((scope && (kind == "array")) && (({ let ml_reference_array68 = wrappers.clone(); let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 }) == (0f64))) && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value69 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value69 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) && (element == "named")) {
        String::from("pending vector")
    } else {
        String::from("unknown")
    }
}

pub fn plan_rust_nominal_ownership_vector_nodes(kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if (((({ let ml_reference_array70 = kinds.clone(); let ml_reference_projection70 = ml_reference_array70.borrow().len() as f64; ml_reference_projection70 }) != ({ let ml_reference_array71 = ends.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 })) || (({ let ml_reference_array72 = kinds.clone(); let ml_reference_projection72 = ml_reference_array72.borrow().len() as f64; ml_reference_projection72 }) != ({ let ml_reference_array73 = targets.clone(); let ml_reference_projection73 = ml_reference_array73.borrow().len() as f64; ml_reference_projection73 }))) || ((self::read_rust_nominal_ownership_graph_error({ let ml_reference_array74 = kinds.clone(); let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 }, { let ml_reference_array75 = edges.clone(); let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 }, 0f64)) != "")) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
            {
                let previous = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s56 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop19(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), elements.clone(), result.clone(), previous, at);
                        match source_ml_source1_s56.clone() {
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(result_2, previous_2) => {
                                if (previous_2 == ({ let ml_reference_array76 = edges.clone(); let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 })) {
                                    result_2.clone()
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
                                }
                            }
                            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(source_ml_temporary_result19) => {
                                source_ml_temporary_result19.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_nominal_verified_vector_clone(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, element: bool) -> bool {
    ((((((((kind == "array") && (({ let ml_reference_array77 = wrappers.clone(); let ml_reference_projection77 = ml_reference_array77.borrow().len() as f64; ml_reference_projection77 }) == (0f64))) && proven) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value78 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value78 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) && element)
}

pub fn plan_rust_owned_recursive_nominal_owners(owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    {
        let checked = self::plan_rust_nominal_ownership_owners(owners.clone(), copies.clone(), clones.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone());
        if ((((self::read_required_text_fact(checked.clone(), 0f64)) != "profiles") || (({ let ml_reference_array79 = leaves.clone(); let ml_reference_projection79 = ml_reference_array79.borrow().len() as f64; ml_reference_projection79 }) != ({ let ml_reference_array80 = kinds.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }))) || ((self::read_rust_nominal_ownership_graph_error({ let ml_reference_array81 = kinds.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }, { let ml_reference_array82 = edges.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 }, 0f64)) != "")) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual recursive datatype graph")]))
        } else {
            {
                let count = { let ml_reference_array83 = kinds.clone(); let ml_reference_projection83 = ml_reference_array83.borrow().len() as f64; ml_reference_projection83 };
                {
                    let size = { let ml_reference_array84 = owners.clone(); let ml_reference_projection84 = ml_reference_array84.borrow().len() as f64; ml_reference_projection84 };
                    {
                        let eligible = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values85 = { let ml_reference_value86 = owners.borrow().clone(); ml_reference_value86 }; let ml_callback85 = std::rc::Rc::new(move || -> bool { true }); ml_values85.iter().cloned().enumerate().map(|(ml_index85, ml_value85)| ml_callback85()).collect::<Vec<_>>() }));
                        {
                            let copy_nodes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let work = 0f64;
                                {
                                    let at = 0f64;
                                    {
                                        let copy_nodes_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop21(copies.clone(), kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, copy_nodes.clone(), at);
                                        {
                                            let owner = 0f64;
                                            {
                                                let source_ml_source1_s57 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop23(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), count, size, work, owner);
                                                match source_ml_source1_s57.clone() {
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(work_2) => {
                                                        {
                                                            let changed = true;
                                                            {
                                                                let source_ml_source1_s58 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop24(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), copy_nodes_2.clone(), work_2, changed);
                                                                match source_ml_source1_s58.clone() {
                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(work_3, changed_2) => {
                                                                        {
                                                                            let inhabited = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values87 = { let ml_reference_value88 = owners.borrow().clone(); ml_reference_value88 }; let ml_callback87 = std::rc::Rc::new(move || -> bool { false }); ml_values87.iter().cloned().enumerate().map(|(ml_index87, ml_value87)| ml_callback87()).collect::<Vec<_>>() }));
                                                                            {
                                                                                let ranks = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values89 = { let ml_reference_value90 = owners.borrow().clone(); ml_reference_value90 }; let ml_callback89 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values89.iter().cloned().enumerate().map(|(ml_index89, ml_value89)| ml_callback89()).collect::<Vec<_>>() }));
                                                                                {
                                                                                    let witnesses = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values91 = { let ml_reference_value92 = owners.borrow().clone(); ml_reference_value92 }; let ml_callback91 = std::rc::Rc::new(move || -> f64 { (-1f64) }); ml_values91.iter().cloned().enumerate().map(|(ml_index91, ml_value91)| ml_callback91()).collect::<Vec<_>>() }));
                                                                                    {
                                                                                        let rank = 0f64;
                                                                                        {
                                                                                            let changed_3 = true;
                                                                                            {
                                                                                                let source_ml_source1_s59 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop25(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, eligible.clone(), work_3, changed_3, inhabited.clone(), ranks.clone(), witnesses.clone(), rank);
                                                                                                match source_ml_source1_s59.clone() {
                                                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(work_4, changed_4) => {
                                                                                                        {
                                                                                                            let implemented = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values93 = { let ml_reference_value94 = owners.borrow().clone(); ml_reference_value94 }; let ml_callback93 = { let eligible = eligible.clone(); let inhabited = inhabited.clone(); let copies = copies.clone(); let clones = clones.clone(); std::rc::Rc::new(move |x_: String, owner_2: f64| -> bool { ((self::read_required_boolean_fact(eligible.clone(), owner_2) && self::read_required_boolean_fact(inhabited.clone(), owner_2)) && (self::read_required_boolean_fact(copies.clone(), owner_2) || self::read_required_boolean_fact(clones.clone(), owner_2))) }) }; ml_values93.iter().cloned().enumerate().map(|(ml_index93, ml_value93)| ml_callback93(ml_value93, (ml_index93 as f64))).collect::<Vec<_>>() }));
                                                                                                            {
                                                                                                                let changed_5 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_s60 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop26(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), leaves.clone(), count, size, work_4, changed_5, implemented.clone());
                                                                                                                    match source_ml_source1_s60.clone() {
                                                                                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done => {
                                                                                                                            {
                                                                                                                                let result = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles")]));
                                                                                                                                {
                                                                                                                                    let owner_2 = 0f64;
                                                                                                                                    {
                                                                                                                                        let result_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop27(copies.clone(), size, eligible.clone(), inhabited.clone(), ranks.clone(), witnesses.clone(), implemented.clone(), result.clone(), owner_2);
                                                                                                                                        result_2.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(source_ml_temporary_result27) => {
                                                                                                                            source_ml_temporary_result27.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(source_ml_temporary_result31) => {
                                                                                                        source_ml_temporary_result31.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(source_ml_temporary_result39) => {
                                                                        source_ml_temporary_result39.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(source_ml_temporary_result43) => {
                                                        source_ml_temporary_result43.clone()
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

pub fn accept_rust_owned_box_type_node(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owner: String, path: String, arguments_: f64) -> bool {
    (((((({ let ml_reference_array95 = wrappers.clone(); let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 }) > (0f64)) && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box")) && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value96 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value96 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64)))
}

pub fn accept_rust_owned_nominal_header_type(data: bool, complete: bool, role: String) -> bool {
    ((data && complete) && ((role == "copy") || (role == "owned")))
}

pub fn read_rust_owned_container_type_node(kind: String, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, proven: bool, owner: String, path: String, arguments_: f64, model: String, leaf: String, copy: bool, nominal: String, scope: bool) -> String {
    if (!scope || !proven) {
        String::from("unknown")
    } else {
        if self::accept_rust_owned_box_type_node(wrappers.clone(), owner.clone(), path.clone(), arguments_) {
            String::from("box")
        } else {
            if { let ml_value = { let ml_reference_array98 = wrappers.clone(); let ml_reference_projection98 = ml_reference_array98.borrow().len() as f64; ml_reference_projection98 }; ml_value != 0f64 && !ml_value.is_nan() } {
                if { let ml_values99 = { let ml_reference_value100 = wrappers.borrow().clone(); ml_reference_value100 }; let ml_callback99 = std::rc::Rc::new(move |value: String| -> bool { (value == "reference") }); ml_values99.iter().cloned().enumerate().all(|(ml_index99, ml_value99)| ml_callback99(ml_value99)) } {
                    String::from("reference")
                } else {
                    String::from("unknown")
                }
            } else {
                if ((kind == "named") && ((nominal == "copy") || (nominal == "owned"))) {
                    String::from("named")
                } else {
                    if (((((kind == "array") && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value101 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value101 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) && (model == "owned vector")) {
                        String::from("vector")
                    } else {
                        if (((leaf == "copy") && copy) && self::ml_array::includes(&{ let ml_reference_value102 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("fixed"), String::from("float"), String::from("unit")]))).borrow().clone(); ml_reference_value102 }, &kind.clone(), 0f64, false, |a, b| a == b)) {
                            String::from("copy")
                        } else {
                            if (leaf == "string") {
                                String::from("string")
                            } else {
                                if (((kind == "tuple") && (leaf == "tuple")) && copy) {
                                    String::from("tuple")
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

pub fn read_rust_owned_container_sum_node(owner: String, arguments_: f64, proven: bool, provisional: bool, wrappers: f64) -> String {
    if ((proven && !provisional) && (wrappers == (0f64))) {
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

pub fn read_rust_owned_container_node_role(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<String>>>, nominal: String) -> String {
    if ((kind == "copy") || (kind == "reference")) {
        String::from("copy")
    } else {
        if (kind == "string") {
            String::from("owned")
        } else {
            if (kind == "named") {
                if ((nominal == "copy") || (nominal == "owned")) {
                    nominal.clone()
                } else {
                    String::from("")
                }
            } else {
                if { let ml_values103 = { let ml_reference_value104 = children.borrow().clone(); ml_reference_value104 }; let ml_callback103 = std::rc::Rc::new(move |value: String| -> bool { ((value != "copy") && (value != "owned")) }); ml_values103.iter().cloned().enumerate().any(|(ml_index103, ml_value103)| ml_callback103(ml_value103)) } {
                    String::from("")
                } else {
                    if ((kind == "box") || (kind == "vector")) {
                        if (({ let ml_reference_array105 = children.clone(); let ml_reference_projection105 = ml_reference_array105.borrow().len() as f64; ml_reference_projection105 }) == (1f64)) {
                            String::from("owned")
                        } else {
                            String::from("")
                        }
                    } else {
                        if (kind == "tuple") {
                            if { let ml_values106 = { let ml_reference_value107 = children.borrow().clone(); ml_reference_value107 }; let ml_callback106 = std::rc::Rc::new(move |value: String| -> bool { (value == "copy") }); ml_values106.iter().cloned().enumerate().all(|(ml_index106, ml_value106)| ml_callback106(ml_value106)) } {
                                String::from("copy")
                            } else {
                                String::from("")
                            }
                        } else {
                            if (((kind == "option") && (({ let ml_reference_array108 = children.clone(); let ml_reference_projection108 = ml_reference_array108.borrow().len() as f64; ml_reference_projection108 }) == (1f64))) || ((kind == "result") && (({ let ml_reference_array109 = children.clone(); let ml_reference_projection109 = ml_reference_array109.borrow().len() as f64; ml_reference_projection109 }) == (2f64)))) {
                                if self::ml_array::includes(&{ let ml_reference_value110 = children.borrow().clone(); ml_reference_value110 }, &String::from("owned"), 0f64, false, |a, b| a == b) {
                                    String::from("owned")
                                } else {
                                    String::from("copy")
                                }
                            } else {
                                String::from("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_container_clone(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, nominal: bool) -> bool {
    (((((kind == "copy") || (kind == "reference")) || (kind == "string")) || ((kind == "named") && nominal)) || (self::ml_array::includes(&{ let ml_reference_value111 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("tuple"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value111 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values112 = { let ml_reference_value113 = children.borrow().clone(); ml_reference_value113 }; let ml_callback112 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values112.iter().cloned().enumerate().all(|(ml_index112, ml_value112)| ml_callback112(ml_value112)) }))
}

pub fn read_rust_owned_box_canonical_owner(owner: String) -> String {
    if ((owner == "Box") || (owner == "std::boxed::Box")) {
        String::from("std::boxed::Box")
    } else {
        if (owner == "alloc::boxed::Box") {
            String::from("alloc::boxed::Box")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_box_constructor_error(owner: String, payload: String, arguments_: f64) -> String {
    if ((self::ml_array::includes(&{ let ml_reference_value114 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value114 }, &owner.clone(), 0f64, false, |a, b| a == b) && ((payload == "copy") || (payload == "owned"))) && (arguments_ == (1f64))) {
        String::from("")
    } else {
        String::from("Box construction needs its authentic standard owner and one actual admitted payload")
    }
}

pub fn read_rust_owned_box_move_error(tracking: bool, available: bool, produced: bool) -> String {
    if (tracking && (available || produced)) {
        String::from("")
    } else {
        String::from("moving a Box needs its actual available binding or completed allocation producer")
    }
}

pub fn accept_rust_owned_nominal_vector_element(role: String, closed: bool) -> bool {
    ((role == "owned") && closed)
}

pub fn accept_rust_owned_container_source(ordinary: bool, minor: f64) -> bool {
    (((ordinary && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor < (100f64)))
}

pub fn read_rust_owned_container_leaf_role(kind: String, bits: String, signed: bool, proven: bool, owner: String, path: String, arguments_: f64, model: String) -> String {
    if !proven {
        String::from("")
    } else {
        if (self::accept_rust_primitive_from_source(kind.clone(), bits.clone(), signed, owner.clone(), path.clone(), arguments_, proven, 0f64) || ((((kind == "unit") && (owner == "unit")) && (path == "")) && (arguments_ == (0f64)))) {
            String::from("copy")
        } else {
            if (((((kind == "string") && (owner == "library")) && self::ml_array::includes(&{ let ml_reference_value115 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::string::String"), String::from("alloc::string::String")]))).borrow().clone(); ml_reference_value115 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (0f64))) && (model == "owned string")) {
                String::from("owned")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_owned_box_intrinsic_copy_value(kind: String, actual_copy: bool) -> bool {
    (self::ml_array::includes(&{ let ml_reference_value116 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("num"), String::from("bool"), String::from("unit"), String::from("str")]))).borrow().clone(); ml_reference_value116 }, &kind.clone(), 0f64, false, |a, b| a == b) || ((kind == "tuple") && actual_copy))
}

pub fn accept_rust_owned_box_value_layers(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    ((((((({ let ml_reference_array117 = wrappers.clone(); let ml_reference_projection117 = ml_reference_array117.borrow().len() as f64; ml_reference_projection117 }) > (0f64)) && ((self::read_required_text_fact(wrappers.clone(), 0f64)) == "box")) && (({ let ml_reference_array118 = owners.clone(); let ml_reference_projection118 = ml_reference_array118.borrow().len() as f64; ml_reference_projection118 }) > (0f64))) && (({ let ml_reference_array119 = owners.clone(); let ml_reference_projection119 = ml_reference_array119.borrow().len() as f64; ml_reference_projection119 }) <= (256f64))) && (({ let ml_reference_array120 = owners.clone(); let ml_reference_projection120 = ml_reference_array120.borrow().len() as f64; ml_reference_projection120 }) <= ({ let ml_reference_array121 = wrappers.clone(); let ml_reference_projection121 = ml_reference_array121.borrow().len() as f64; ml_reference_projection121 }))) && { let ml_values122 = { let ml_reference_value123 = owners.borrow().clone(); ml_reference_value123 }; let ml_callback122 = std::rc::Rc::new(move |owner: String| -> bool { self::ml_array::includes(&{ let ml_reference_value124 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::boxed::Box"), String::from("alloc::boxed::Box")]))).borrow().clone(); ml_reference_value124 }, &owner.clone(), 0f64, false, |a, b| a == b) }); ml_values122.iter().cloned().enumerate().all(|(ml_index122, ml_value122)| ml_callback122(ml_value122)) })
}

pub fn read_rust_owned_container_child_model(kind: String, owner: String, path: String, arguments_: f64) -> String {
    if ((((owner == "library") && (kind == "string")) && self::ml_array::includes(&{ let ml_reference_value125 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::string::String"), String::from("alloc::string::String")]))).borrow().clone(); ml_reference_value125 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (0f64))) {
        String::from("owned string")
    } else {
        if ((((owner == "library") && (kind == "array")) && self::ml_array::includes(&{ let ml_reference_value126 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value126 }, &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_ == (1f64))) {
            String::from("owned vector")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_container_runtime_metadata_cost(role: String, units: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_integer(units)) || (units < (0f64))) || (units > (8388608f64))) || !(self::ml_array::includes(&{ let ml_reference_value127 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("origin"), String::from("list"), String::from("binding"), String::from("layer"), String::from("owner"), String::from("intrinsic copy"), String::from("payload loans"), String::from("payload field loans"), String::from("value loans"), String::from("value field loans"), String::from("place loans"), String::from("loans"), String::from("temporary loans")]))).borrow().clone(); ml_reference_value127 }, &role.clone(), 0f64, false, |a, b| a == b))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        if (role == "origin") {
            std::rc::Rc::new(std::cell::RefCell::new(vec![16f64, (256f64 + units)]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, ((role.encode_utf16().count() as f64) + units)]))
        }
    }
}

pub fn read_rust_owned_box_clone_error(actual: bool, clone: bool, readonly: bool, closed: bool, available: bool, produced: bool) -> String {
    if (!actual || !clone) {
        String::from("Box Clone needs its actual admitted payload Clone implementation")
    } else {
        if (readonly && !closed) {
            String::from("a borrowed Box Clone needs its closed payload lifetime projection")
        } else {
            if ((readonly || available) || produced) {
                String::from("")
            } else {
                String::from("Box Clone needs its actual available source value or actual producer")
            }
        }
    }
}

pub fn accept_rust_owned_box_layer_clone(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    if (!(self::accept_rust_owned_box_value_layers(wrappers.clone(), owners.clone())) || (({ let ml_reference_array128 = owners.clone(); let ml_reference_projection128 = ml_reference_array128.borrow().len() as f64; ml_reference_projection128 }) != ({ let ml_reference_array129 = copied.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 }))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s61 = self::source_ml_temporary_accept_rust_owned_box_layer_clone_loop48(wrappers.clone(), owners.clone(), copied.clone(), at);
                match source_ml_source1_s61.clone() {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done => {
                        false
                    }
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(source_ml_temporary_result48) => {
                        source_ml_temporary_result48
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_standard_sum_box_payload(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: f64, owner: String, path: String, arguments_: f64, boxes: f64) -> bool {
    ((((self::accept_rust_owned_box_type_node(wrappers.clone(), owner.clone(), path.clone(), arguments_) && { let ml_values130 = { let ml_reference_value131 = wrappers.borrow().clone(); ml_reference_value131 }; let ml_callback130 = std::rc::Rc::new(move |layer: String| -> bool { (layer == "box") }); ml_values130.iter().cloned().enumerate().all(|(ml_index130, ml_value130)| ml_callback130(ml_value130)) }) && (lifetimes == (0f64))) && self::ml_math::is_integer(boxes)) && (boxes == ({ let ml_reference_array132 = wrappers.clone(); let ml_reference_projection132 = ml_reference_array132.borrow().len() as f64; ml_reference_projection132 })))
}

pub fn accept_rust_owned_container_value_loan_role(role: String) -> bool {
    self::ml_array::includes(&{ let ml_reference_value133 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("value loans"), String::from("value field loans"), String::from("payload loans"), String::from("payload field loans")]))).borrow().clone(); ml_reference_value133 }, &role.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_rust_owned_box_payload_error(closed: bool, intrinsic: bool, borrowed: bool, produced: bool) -> String {
    if (closed || ((intrinsic || produced) && !borrowed)) {
        String::from("")
    } else {
        String::from("stored Box payload references or callable captures need their complete lifetime/drop profile")
    }
}

pub fn read_rust_owned_container_depth_error(wrappers: f64, layers: f64) -> String {
    if (((((!(self::ml_math::is_integer(wrappers)) || !(self::ml_math::is_integer(layers))) || (wrappers < (0f64))) || (layers < (0f64))) || (wrappers > (256f64))) || (layers > (256f64))) {
        String::from("owned container source syntax and value lineage exceed their finite depth profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_owned_container_closed(kind: String, children: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, nominal: bool) -> bool {
    ((((kind == "copy") || (kind == "string")) || ((kind == "named") && nominal)) || (self::ml_array::includes(&{ let ml_reference_value134 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("tuple"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value134 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values135 = { let ml_reference_value136 = children.borrow().clone(); ml_reference_value136 }; let ml_callback135 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values135.iter().cloned().enumerate().all(|(ml_index135, ml_value135)| ml_callback135(ml_value135)) }))
}

pub fn read_rust_owned_box_type_error(selected: bool, header: bool, complete: bool, role: String, closed: bool) -> String {
    if (((!selected || header) || !complete) || ((role == "owned") && closed)) {
        String::from("")
    } else {
        String::from("an owned Box source type needs its complete closed payload ownership graph")
    }
}

pub fn accept_rust_owned_box_call_result(actual: bool, function_: bool, owner: bool, type_: bool, recorded: bool) -> bool {
    ((((actual && function_) && owner) && type_) && recorded)
}

pub fn plan_rust_owned_nominal_runtime_facts(selected: bool, old_role: String, old_clone: bool, role: String, clone: bool, closed: bool, rank: f64, constructor: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if ((old_role == "copy") || (old_role == "owned")) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![old_role.clone(), if old_clone {
            String::from("Clone")
        } else {
            String::from("")
        }]))
    } else {
        if ((((((((!selected || (role != "owned")) || !closed) || !(self::ml_math::is_integer(rank))) || (rank < (1f64))) || (rank > (65536f64))) || !(self::ml_math::is_integer(constructor))) || (constructor < (0f64))) || (constructor > (65536f64))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), if clone {
                String::from("Clone")
            } else {
                String::from("")
            }]))
        }
    }
}

pub fn read_rust_owned_vector_signature_phase(selected: bool, header: bool, complete: bool, kind: String, wrappers: f64, owner: String, path: String, arguments_: f64, element: String, profile: String, closed: bool) -> String {
    if ((((((!selected || (kind != "array")) || (wrappers != (0f64))) || (owner != "library")) || !(self::ml_array::includes(&{ let ml_reference_value137 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value137 }, &path.clone(), 0f64, false, |a, b| a == b))) || (arguments_ != (1f64))) || (element != "named")) {
        String::from("")
    } else {
        if (header && !complete) {
            String::from("defer")
        } else {
            if ((complete && (profile == "owned")) && closed) {
                String::from("admit")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_vector_element_production_error(role: String, same: bool, transferred: bool, available: bool, produced: bool, collecting: bool) -> String {
    if (!(self::ml_array::includes(&{ let ml_reference_value138 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("owned")]))).borrow().clone(); ml_reference_value138 }, &role.clone(), 0f64, false, |a, b| a == b)) || !same) {
        String::from("owned vector construction needs each actual complete matching element type")
    } else {
        if ((((collecting || (role == "copy")) || transferred) || available) || produced) {
            String::from("")
        } else {
            String::from("an owned vector element needs its actual available binding or completed producer")
        }
    }
}

pub fn accept_rust_owned_vector_call_result(actual: bool, function_: bool, owner: bool, type_: bool, recorded: bool) -> bool {
    ((((actual && function_) && owner) && type_) && recorded)
}

pub fn accept_rust_owned_vector_named_element(role: String, closed: bool, wrappers: f64) -> bool {
    (((wrappers == (0f64)) && closed) && ((role == "copy") || (role == "owned")))
}

pub fn read_rust_owned_vector_element_count_error(count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (65536f64))) {
        String::from("owned vector construction exceeds its finite actual element profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_vector_named_admission(previous: String, actual: bool) -> String {
    if ((previous == "refuse") && actual) {
        String::from("admit")
    } else {
        previous.clone()
    }
}

pub fn accept_rust_owned_vector_produced_value(node: String, role: String, closed: bool, produced: bool) -> bool {
    ((((node == "vector") && (role == "owned")) && closed) && produced)
}

pub fn accept_rust_owned_vector_function_result(phase: String, actual: bool, owner: bool, type_: bool, value: bool, borrowed: bool, collecting: bool) -> bool {
    ((((((phase == "admit") && actual) && owner) && type_) && (collecting || value)) && !borrowed)
}

pub fn read_rust_owned_vector_empty_constructor_disposition(phase: String, owner: String, method: String, arguments_: f64) -> String {
    if (((phase != "admit") || !(self::ml_array::includes(&{ let ml_reference_value139 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Vec"), String::from("std::vec::Vec"), String::from("alloc::vec::Vec")]))).borrow().clone(); ml_reference_value139 }, &owner.clone(), 0f64, false, |a, b| a == b))) || (method != "new")) {
        String::from("")
    } else {
        if (arguments_ != (0f64)) {
            String::from("arity")
        } else {
            String::from("construct")
        }
    }
}

pub fn read_rust_owned_box_clone_source_error(collecting: bool, actual: bool, clone: bool, readonly: bool, closed: bool, available: bool, produced: bool) -> String {
    {
        let error = self::read_rust_owned_box_clone_error(actual, clone, readonly, closed, available, produced);
        if (((collecting && actual) && clone) && (!readonly || closed)) {
            String::from("")
        } else {
            error.clone()
        }
    }
}

pub fn read_rust_owned_vector_clone_error(collecting: bool, actual: bool, clone: bool, closed: bool, readonly: bool, available: bool, produced: bool) -> String {
    if ((!actual || !clone) || !closed) {
        String::from("vector Clone needs its actual closed element Clone implementation")
    } else {
        if (((collecting || readonly) || available) || produced) {
            String::from("")
        } else {
            String::from("vector Clone needs its actual available source value or actual producer")
        }
    }
}

pub fn accept_rust_owned_argument_read_check(wrappers: f64, transferred: bool) -> bool {
    ((wrappers > (0f64)) && !transferred)
}

pub fn read_rust_owned_nominal_field_lifetime_error(field: bool, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if !field {
        String::from("")
    } else {
        {
            let references = { let ml_reference_array142 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values140 = { let ml_reference_value141 = wrappers.borrow().clone(); ml_reference_value141 }; let ml_callback140 = std::rc::Rc::new(move |layer: String| -> bool { (layer == "reference") }); ml_values140.iter().cloned().enumerate().filter_map(|(ml_index140, ml_value140)| if ml_callback140(ml_value140.clone()) { Some(ml_value140) } else { None }).collect::<Vec<_>>() })); let ml_reference_projection142 = ml_reference_array142.borrow().len() as f64; ml_reference_projection142 };
            if ((references != ({ let ml_reference_array143 = lifetimes.clone(); let ml_reference_projection143 = ml_reference_array143.borrow().len() as f64; ml_reference_projection143 })) || { let ml_values144 = { let ml_reference_value145 = lifetimes.borrow().clone(); ml_reference_value145 }; let ml_callback144 = std::rc::Rc::new(move |lifetime: String| -> bool { (lifetime != "static") }); ml_values144.iter().cloned().enumerate().any(|(ml_index144, ml_value144)| ml_callback144(ml_value144)) }) {
                String::from("a nongeneric datatype reference field requires its explicit static lifetime")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_vector_element_role(kind: String, copy: bool, direct: bool, fields: bool, actual: String) -> String {
    if ((actual == "copy") || (actual == "owned")) {
        actual.clone()
    } else {
        if ((((kind == "") && copy) && direct) && !fields) {
            String::from("copy")
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_owned_vector_element_type_match(actual: String, expected: String, same: bool, copy: bool) -> bool {
    (same || (((actual == "") && (expected == "fixed")) && copy))
}

pub fn accept_rust_owned_vector_nominal_type_identity(left: String, right: String, references: bool) -> bool {
    ((references && (left != "")) && (left == right))
}

pub fn accept_rust_owned_box_pointer_owner_child(kind: String, path: String, arguments_count: f64) -> bool {
    (((kind == "library") && (path == "alloc::boxed::Box")) && (arguments_count == (1f64)))
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop4(mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owner: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    loop {
        return if (owner < size) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value146 = result.borrow().clone(); ml_reference_value146 }, vec![self::read_required_text_fact(known.clone(), owner)])));
                {
                    let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value147 = result_2.borrow().clone(); ml_reference_value147 }, vec![if self::read_required_boolean_fact(implemented.clone(), owner) {
                        String::from("Clone")
                    } else {
                        String::from("")
                    }])));
                    {
                        let owner_2 = (owner + 1f64);
                        {
                            (result, owner) = (result_3.clone(), owner_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(bool, bool),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut ready: bool, mut clone: bool, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let ready_3 = if ((self::read_required_text_fact(roles.clone(), child)) == "") {
                        {
                            let ready_2 = false;
                            ready_2
                        }
                    } else {
                        ready
                    };
                    if !(self::read_required_boolean_fact(clone_fields.clone(), child)) {
                        {
                            let clone_2 = false;
                            {
                                let edge_2 = (edge + 1f64);
                                {
                                    (ready, clone, edge) = (ready_3, clone_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let edge_3 = (edge + 1f64);
                            {
                                (ready, edge) = (ready_3, edge_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(ready, clone)
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop6(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if self::read_required_boolean_fact(copies.clone(), owner) {
                {
                    let owner_2 = (owner + 1f64);
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            } else {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if (root == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let end = self::read_required_number_fact(ends.clone(), root);
                            {
                                let ready = (end > start);
                                {
                                    let clone = true;
                                    {
                                        let edge = start;
                                        {
                                            let source_ml_source1_s62 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop7(edges.clone(), roles.clone(), clone_fields.clone(), end, ready, clone, edge);
                                            match source_ml_source1_s62.clone() {
                                                self::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop7Done(ready_2, clone_2) => {
                                                    {
                                                        let changed_3 = if (ready_2 && ((self::read_required_text_fact(known.clone(), owner)) == "")) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored8 = { let ml_array_owner148 = known.clone(); let ml_array_argument148_0: f64 = owner; let ml_array_argument148_1: f64 = 1f64; let ml_array_argument148_2: String = String::from("owned"); let ml_array_length = ml_array_owner148.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument148_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument148_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument148_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner148.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_2 = true;
                                                                    changed_2
                                                                }
                                                            }
                                                        } else {
                                                            changed
                                                        };
                                                        if (((ready_2 && clone_2) && self::read_required_boolean_fact(clones.clone(), owner)) && !(self::read_required_boolean_fact(implemented.clone(), owner))) {
                                                            {
                                                                let source_ml_temporary_plan_rust_nominal_ownership_owners_ignored9 = { let ml_array_owner149 = implemented.clone(); let ml_array_argument149_0: f64 = owner; let ml_array_argument149_1: f64 = 1f64; let ml_array_argument149_2: bool = true; let ml_array_length = ml_array_owner149.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument149_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument149_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument149_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner149.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                                                {
                                                                    let changed_4 = true;
                                                                    {
                                                                        let owner_3 = (owner + 1f64);
                                                                        {
                                                                            (changed, owner) = (changed_4, owner_3);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let owner_4 = (owner + 1f64);
                                                                {
                                                                    (changed, owner) = (changed_3, owner_4);
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
        } else {
            changed
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(f64, std::rc::Rc<std::cell::RefCell<Vec<String>>>, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop10(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut child_roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut child_clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                {
                    let child_roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value150 = child_roles.borrow().clone(); ml_reference_value150 }, vec![self::read_required_text_fact(roles.clone(), child)])));
                    {
                        let child_clones_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value151 = child_clones.borrow().clone(); ml_reference_value151 }, vec![self::read_required_boolean_fact(clone_fields.clone(), child)])));
                        {
                            let edge_2 = (edge + 1f64);
                            {
                                (child_roles, child_clones, edge) = (child_roles_2.clone(), child_clones_2.clone(), edge_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(child_roles.clone(), child_clones.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop5(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut roles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut clone_fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal ownership graph reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let child_roles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                        {
                            let child_clones = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let start = if (at == (0f64)) {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let edge = start;
                                    {
                                        let source_ml_source1_s63 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop10(ends.clone(), edges.clone(), roles.clone(), clone_fields.clone(), at, child_roles.clone(), child_clones.clone(), edge);
                                        match source_ml_source1_s63.clone() {
                                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop10Done(child_roles_2, child_clones_2) => {
                                                {
                                                    let target = self::read_required_number_fact(targets.clone(), at);
                                                    {
                                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                                        {
                                                            let roles_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value152 = roles.borrow().clone(); ml_reference_value152 }, vec![if (target >= (0f64)) {
                                                                self::read_required_text_fact(known.clone(), target)
                                                            } else {
                                                                self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())
                                                            }])));
                                                            {
                                                                let clone_fields_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value153 = clone_fields.borrow().clone(); ml_reference_value153 }, vec![if (target >= (0f64)) {
                                                                    self::read_required_boolean_fact(implemented.clone(), target)
                                                                } else {
                                                                    ((((kind == "copy") || (kind == "string")) || (kind == "standard collection")) || ((self::ml_array::includes(&{ let ml_reference_value154 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("tuple"), String::from("constructor"), String::from("option"), String::from("result")]))).borrow().clone(); ml_reference_value154 }, &kind.clone(), 0f64, false, |a, b| a == b) && { let ml_values155 = { let ml_reference_value156 = child_clones_2.borrow().clone(); ml_reference_value156 }; let ml_callback155 = std::rc::Rc::new(move |value: bool| -> bool { value }); ml_values155.iter().cloned().enumerate().all(|(ml_index155, ml_value155)| ml_callback155(ml_value155)) }) && ((self::read_rust_nominal_ownership_node_role(kind.clone(), child_roles_2.clone())) != "")))
                                                                }])));
                                                                {
                                                                    let at_2 = (at + 1f64);
                                                                    {
                                                                        (work, roles, clone_fields, at) = (work_2, roles_2.clone(), clone_fields_2.clone(), at_2);
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
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(work, roles.clone(), clone_fields.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop3(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut known: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut work: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let roles = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
                    {
                        let clone_fields = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_s64 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop5(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), count, known.clone(), implemented.clone(), work, roles.clone(), clone_fields.clone(), at);
                                match source_ml_source1_s64.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Done(work_2, roles_2, clone_fields_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop6(copies.clone(), clones.clone(), ends.clone(), edges.clone(), roots.clone(), size, known.clone(), implemented.clone(), changed_2, roles_2.clone(), clone_fields_2.clone(), owner);
                                                {
                                                    (changed, work) = (changed_3, work_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop5Return(source_ml_temporary_result9) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Return(source_ml_temporary_result9.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop2(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut owner: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result {
    loop {
        return if (owner < size) {
            {
                let name = self::read_required_text_fact(owners.clone(), owner);
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    if ((((((name == "") || ((self::ml_array::index_of(&{ let ml_reference_value157 = owners.borrow().clone(); ml_reference_value157 }, &name.clone(), 0f64, false, |a, b| a == b)) != owner)) || !(self::ml_math::is_integer(root))) || (root < (0f64))) || (root >= count)) || ((self::read_required_text_fact(kinds.clone(), root)) != "nominal")) {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal declaration roots")])))
                    } else {
                        if (self::read_required_boolean_fact(copies.clone(), owner) && !(self::read_required_boolean_fact(clones.clone(), owner))) {
                            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("actual nominal Copy must retain its complete Clone derivation")])))
                        } else {
                            {
                                let owner_2 = (owner + 1f64);
                                {
                                    owner = owner_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(f64),
    MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result {
    MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done,
    MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop11(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut end: f64, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child >= at)) {
                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("nominal field edges need actual postorder identities")])))
                } else {
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_owners_loop1(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut previous: f64, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result {
    loop {
        return if (at < count) {
            {
                let end = self::read_required_number_fact(ends.clone(), at);
                {
                    let target = self::read_required_number_fact(targets.clone(), at);
                    if (((((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array158 = edges.clone(); let ml_reference_projection158 = ml_reference_array158.borrow().len() as f64; ml_reference_projection158 }))) || !(self::ml_math::is_integer(target))) || (target < ((-1f64)))) || (target >= size)) {
                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("incomplete actual nominal ownership graph")])))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s65 = self::source_ml_temporary_plan_rust_nominal_ownership_owners_loop11(edges.clone(), at, end, edge);
                                match source_ml_source1_s65.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Done => {
                                        {
                                            let previous_2 = end;
                                            {
                                                let at_2 = (at + 1f64);
                                                {
                                                    (previous, at) = (previous_2, at_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop11Return(source_ml_temporary_result11) => {
                                        self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Return(source_ml_temporary_result11.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Result::MlTemporaryPlanRustNominalOwnershipOwnersLoop1Done(previous)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop12Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop12Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
    MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalPatternTransfersLoop13Result {
    MlTemporaryPlanRustNominalPatternTransfersLoop13Done,
    MlTemporaryPlanRustNominalPatternTransfersLoop13Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop13(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut size: f64, mut index: f64, mut identity: String, mut peer: f64) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result {
    loop {
        return if (peer < size) {
            if ((((peer != index) && ((self::read_required_text_fact(identities.clone(), peer)) == identity)) && self::read_required_boolean_fact(owned.clone(), peer)) && !(self::read_required_boolean_fact(fields.clone(), peer))) {
                self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a whole nominal alias cannot also move its owned fields")])))
            } else {
                {
                    let peer_2 = (peer + 1f64);
                    {
                        peer = peer_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_pattern_transfers_loop12(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut fields: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owned: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut produced: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut deferred: bool, mut size: f64, mut plan: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result {
    loop {
        return if (index < size) {
            if (!(self::read_required_boolean_fact(fields.clone(), index)) || !(self::read_required_boolean_fact(owned.clone(), index))) {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            } else {
                if deferred {
                    self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("owned nominal guarded fields need their deferred partial-move interpretation")])))
                } else {
                    {
                        let identity = self::read_required_text_fact(identities.clone(), index);
                        if ((identity == "") && !(self::read_required_boolean_fact(produced.clone(), index))) {
                            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a nominal field move needs its actual available parent or actual producer")])))
                        } else {
                            {
                                let peer = 0f64;
                                {
                                    let source_ml_source1_s66 = self::source_ml_temporary_plan_rust_nominal_pattern_transfers_loop13(identities.clone(), fields.clone(), owned.clone(), size, index, identity.clone(), peer);
                                    match source_ml_source1_s66.clone() {
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Done => {
                                            if ((identity != "") && !(self::ml_array::includes(&{ let ml_reference_value159 = seen.borrow().clone(); ml_reference_value159 }, &identity.clone(), 0f64, false, |a, b| a == b))) {
                                                {
                                                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value160 = seen.borrow().clone(); ml_reference_value160 }, vec![identity.clone()])));
                                                    {
                                                        let plan_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value161 = plan.borrow().clone(); ml_reference_value161 }, vec![{
                                                            let source_ml_source1_string_argument67 = index;
                                                            self::ml_number::js_number(source_ml_source1_string_argument67)
                                                        }])));
                                                        {
                                                            let index_3 = (index + 1f64);
                                                            {
                                                                (plan, seen, index) = (plan_2.clone(), seen_2.clone(), index_3);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let index_4 = (index + 1f64);
                                                    {
                                                        index = index_4;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanRustNominalPatternTransfersLoop13Result::MlTemporaryPlanRustNominalPatternTransfersLoop13Return(source_ml_temporary_result13) => {
                                            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Return(source_ml_temporary_result13.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalPatternTransfersLoop12Result::MlTemporaryPlanRustNominalPatternTransfersLoop12Done(plan.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done,
    MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop16(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < ({ let ml_reference_array162 = owners.clone(); let ml_reference_projection162 = ml_reference_array162.borrow().len() as f64; ml_reference_projection162 })) {
            if (((!(self::read_required_boolean_fact(known.clone(), owner)) && self::read_required_boolean_fact(copies.clone(), owner)) && self::read_required_boolean_fact(clones.clone(), owner)) && self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(roots.clone(), owner))) {
                {
                    let source_ml_temporary_plan_rust_nominal_vector_element_owners_ignored17 = { let ml_array_owner163 = known.clone(); let ml_array_argument163_0: f64 = owner; let ml_array_argument163_1: f64 = 1f64; let ml_array_argument163_2: bool = true; let ml_array_length = ml_array_owner163.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument163_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument163_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument163_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner163.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = (owner + 1f64);
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_3 = (owner + 1f64);
                    {
                        owner = owner_3;
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
pub enum MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result {
    MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop18(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut actual: bool, mut end: f64, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let actual_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (actual, edge) = (actual_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            actual
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result {
    loop {
        return if (at < ({ let ml_reference_array164 = kinds.clone(); let ml_reference_projection164 = ml_reference_array164.borrow().len() as f64; ml_reference_projection164 })) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new())))
                } else {
                    {
                        let target = self::read_required_number_fact(targets.clone(), at);
                        {
                            let kind = self::read_required_text_fact(kinds.clone(), at);
                            {
                                let actual = if (target >= (0f64)) {
                                    self::read_required_boolean_fact(known.clone(), target)
                                } else {
                                    if self::ml_array::includes(&{ let ml_reference_value165 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("string")]))).borrow().clone(); ml_reference_value165 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                        self::read_required_boolean_fact(leaves.clone(), at)
                                    } else {
                                        self::ml_array::includes(&{ let ml_reference_value166 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("constructor"), String::from("tuple"), String::from("option"), String::from("result"), String::from("nominal")]))).borrow().clone(); ml_reference_value166 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                    }
                                };
                                {
                                    let start = if (at == (0f64)) {
                                        0f64
                                    } else {
                                        self::read_required_number_fact(ends.clone(), (at - 1f64))
                                    };
                                    {
                                        let end = self::read_required_number_fact(ends.clone(), at);
                                        {
                                            let actual_3 = if ((kind == "nominal") && (start == end)) {
                                                {
                                                    let actual_2 = false;
                                                    actual_2
                                                }
                                            } else {
                                                actual
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let actual_4 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop18(edges.clone(), safe.clone(), actual_3, end, edge);
                                                    {
                                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value167 = safe.borrow().clone(); ml_reference_value167 }, vec![actual_4])));
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (work, safe, at) = (work_2, safe_2.clone(), at_2);
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
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(work, safe.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_vector_element_owners_loop14(mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut clones: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut known: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut work: f64) -> self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s68 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop15(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), known.clone(), work, safe.clone(), at);
                            match source_ml_source1_s68.clone() {
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Done(work_2, safe_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_nominal_vector_element_owners_loop16(owners.clone(), copies.clone(), clones.clone(), roots.clone(), known.clone(), changed_2, safe_2.clone(), owner);
                                            {
                                                (changed, work) = (changed_3, work_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop15Return(source_ml_temporary_result17) => {
                                    self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Return(source_ml_temporary_result17.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Result::MlTemporaryPlanRustNominalVectorElementOwnersLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(std::rc::Rc<std::cell::RefCell<Vec<String>>>, f64),
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result {
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done,
    MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop20(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut end: f64, mut edge: f64) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result {
    loop {
        return if (edge < end) {
            {
                let child = self::read_required_number_fact(edges.clone(), edge);
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child >= at)) {
                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                } else {
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop19(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut elements: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut previous: f64, mut at: f64) -> self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result {
    loop {
        return if (at < ({ let ml_reference_array168 = kinds.clone(); let ml_reference_projection168 = ml_reference_array168.borrow().len() as f64; ml_reference_projection168 })) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let end = self::read_required_number_fact(ends.clone(), at);
                    if ((!(self::ml_math::is_integer(end)) || (end < previous)) || (end > ({ let ml_reference_array169 = edges.clone(); let ml_reference_projection169 = ml_reference_array169.borrow().len() as f64; ml_reference_projection169 }))) {
                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())))
                    } else {
                        {
                            let edge = previous;
                            {
                                let source_ml_source1_s69 = self::source_ml_temporary_plan_rust_nominal_ownership_vector_nodes_loop20(edges.clone(), at, end, edge);
                                match source_ml_source1_s69.clone() {
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Done => {
                                        {
                                            let selected = kind.clone();
                                            {
                                                let selected_4 = if (kind == "pending vector") {
                                                    {
                                                        let selected_2 = String::from("unknown");
                                                        if (end == ((previous + 1f64))) {
                                                            {
                                                                let child = self::read_required_number_fact(edges.clone(), previous);
                                                                {
                                                                    let target = self::read_required_number_fact(targets.clone(), child);
                                                                    if ((((((self::read_required_text_fact(kinds.clone(), child)) == "reference") && self::ml_math::is_integer(target)) && (target >= (0f64))) && (target < ({ let ml_reference_array170 = elements.clone(); let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 }))) && self::read_required_boolean_fact(elements.clone(), target)) {
                                                                        {
                                                                            let selected_3 = String::from("vector");
                                                                            selected_3.clone()
                                                                        }
                                                                    } else {
                                                                        selected_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            selected_2.clone()
                                                        }
                                                    }
                                                } else {
                                                    selected.clone()
                                                };
                                                {
                                                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value171 = result.borrow().clone(); ml_reference_value171 }, vec![selected_4.clone()])));
                                                    {
                                                        let previous_2 = end;
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (result, previous, at) = (result_2.clone(), previous_2, at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop20Return(source_ml_temporary_result20) => {
                                        self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Return(source_ml_temporary_result20.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Result::MlTemporaryPlanRustNominalOwnershipVectorNodesLoop19Done(result.clone(), previous)
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop22(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut copy: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(copy_nodes.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let copy_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (copy, edge) = (copy_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            copy
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop21(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    loop {
        return if (at < count) {
            {
                let kind = self::read_required_text_fact(kinds.clone(), at);
                {
                    let target = self::read_required_number_fact(targets.clone(), at);
                    {
                        let start = if (at == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                        };
                        {
                            let end = self::read_required_number_fact(ends.clone(), at);
                            {
                                let copy = if (target >= (0f64)) {
                                    self::read_required_boolean_fact(copies.clone(), target)
                                } else {
                                    (kind == "copy")
                                };
                                {
                                    let copy_4 = if (kind == "tuple") {
                                        {
                                            let copy_2 = true;
                                            {
                                                let edge = start;
                                                {
                                                    let copy_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop22(edges.clone(), copy_nodes.clone(), end, copy_2, edge);
                                                    copy_3
                                                }
                                            }
                                        }
                                    } else {
                                        copy
                                    };
                                    {
                                        let copy_nodes_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value172 = copy_nodes.borrow().clone(); ml_reference_value172 }, vec![copy_4])));
                                        {
                                            let at_2 = (at + 1f64);
                                            {
                                                (copy_nodes, at) = (copy_nodes_2.clone(), at_2);
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
            copy_nodes.clone()
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop27(mut copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owner: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    loop {
        return if (owner < size) {
            {
                let admitted = (self::read_required_boolean_fact(eligible.clone(), owner) && self::read_required_boolean_fact(inhabited.clone(), owner));
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value173 = result.borrow().clone(); ml_reference_value173 }, vec![if admitted {
                        if self::read_required_boolean_fact(copies.clone(), owner) {
                            String::from("copy")
                        } else {
                            String::from("owned")
                        }
                    } else {
                        String::from("")
                    }])));
                    {
                        let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value174 = result_2.borrow().clone(); ml_reference_value174 }, vec![if self::read_required_boolean_fact(implemented.clone(), owner) {
                            String::from("Clone")
                        } else {
                            String::from("")
                        }])));
                        {
                            let result_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value175 = result_3.borrow().clone(); ml_reference_value175 }, vec![if admitted {
                                String::from("closed")
                            } else {
                                String::from("")
                            }])));
                            {
                                let result_5 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value176 = result_4.borrow().clone(); ml_reference_value176 }, vec![{
                                    let source_ml_source1_string_argument70 = self::read_required_number_fact(ranks.clone(), owner);
                                    self::ml_number::js_number(source_ml_source1_string_argument70)
                                }])));
                                {
                                    let result_6 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value177 = result_5.borrow().clone(); ml_reference_value177 }, vec![{
                                        let source_ml_source1_string_argument71 = self::read_required_number_fact(witnesses.clone(), owner);
                                        self::ml_number::js_number(source_ml_source1_string_argument71)
                                    }])));
                                    {
                                        let owner_2 = (owner + 1f64);
                                        {
                                            (result, owner) = (result_6.clone(), owner_2);
                                            continue;
                                        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done,
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop29(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut changed: bool, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(implemented.clone(), owner) && !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(roots.clone(), owner)))) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored30 = { let ml_array_owner178 = implemented.clone(); let ml_array_argument178_0: f64 = owner; let ml_array_argument178_1: f64 = 1f64; let ml_array_argument178_2: bool = false; let ml_array_length = ml_array_owner178.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument178_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument178_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument178_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner178.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = (owner + 1f64);
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_3 = (owner + 1f64);
                    {
                        owner = owner_3;
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            if !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop28(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut work: f64, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let kind = self::read_required_text_fact(kinds.clone(), at);
                        {
                            let target = self::read_required_number_fact(targets.clone(), at);
                            {
                                let start = if (at == (0f64)) {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let value = if (target >= (0f64)) {
                                        self::read_required_boolean_fact(implemented.clone(), target)
                                    } else {
                                        if (kind == "standard collection") {
                                            true
                                        } else {
                                            if ((kind == "copy") || (kind == "string")) {
                                                self::read_required_boolean_fact(leaves.clone(), at)
                                            } else {
                                                self::ml_array::includes(&{ let ml_reference_value179 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("pending vector"), String::from("option"), String::from("result"), String::from("constructor"), String::from("nominal"), String::from("tuple")]))).borrow().clone(); ml_reference_value179 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                            }
                                        }
                                    };
                                    {
                                        let edge = start;
                                        {
                                            let value_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop31(ends.clone(), edges.clone(), values.clone(), at, value, edge);
                                            {
                                                let values_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value180 = values.borrow().clone(); ml_reference_value180 }, vec![value_2])));
                                                {
                                                    let at_2 = (at + 1f64);
                                                    {
                                                        (work, values, at) = (work_2, values_2.clone(), at_2);
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
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(work, values.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop26(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut work: f64, mut changed: bool, mut implemented: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s72 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop28(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, implemented.clone(), values.clone(), at);
                            match source_ml_source1_s72.clone() {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Done(work_2, values_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop29(roots.clone(), size, changed_2, implemented.clone(), values_2.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop28Return(source_ml_temporary_result30) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Return(source_ml_temporary_result30.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64, mut root: f64, mut start: f64, mut edge: f64) -> bool {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), root))) {
            if self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored35 = { let ml_array_owner181 = inhabited.clone(); let ml_array_argument181_0: f64 = owner; let ml_array_argument181_1: f64 = 1f64; let ml_array_argument181_2: bool = true; let ml_array_length = ml_array_owner181.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument181_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument181_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument181_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner181.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored36 = { let ml_array_owner182 = ranks.clone(); let ml_array_argument182_0: f64 = owner; let ml_array_argument182_1: f64 = 1f64; let ml_array_argument182_2: f64 = rank; let ml_array_length = ml_array_owner182.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument182_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument182_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument182_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner182.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                        {
                            let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored37 = { let ml_array_owner183 = witnesses.clone(); let ml_array_argument183_0: f64 = owner; let ml_array_argument183_1: f64 = 1f64; let ml_array_argument183_2: f64 = (edge - start); let ml_array_length = ml_array_owner183.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument183_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument183_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument183_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner183.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
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

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(eligible.clone(), owner) && !(self::read_required_boolean_fact(inhabited.clone(), owner))) {
                {
                    let root = self::read_required_number_fact(roots.clone(), owner);
                    {
                        let start = if (root == (0f64)) {
                            0f64
                        } else {
                            self::read_required_number_fact(ends.clone(), (root - 1f64))
                        };
                        {
                            let edge = start;
                            {
                                let changed_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop34(ends.clone(), edges.clone(), changed, inhabited.clone(), ranks.clone(), witnesses.clone(), rank, values.clone(), owner, root, start, edge);
                                {
                                    let owner_2 = (owner + 1f64);
                                    {
                                        (changed, owner) = (changed_2, owner_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_3 = (owner + 1f64);
                    {
                        owner = owner_3;
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop38(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop39(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if self::read_required_boolean_fact(values.clone(), self::read_required_number_fact(edges.clone(), edge)) {
                {
                    let value_2 = true;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut work: f64, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let kind = self::read_required_text_fact(kinds.clone(), at);
                        {
                            let target = self::read_required_number_fact(targets.clone(), at);
                            {
                                let start = if (at == (0f64)) {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let end = self::read_required_number_fact(ends.clone(), at);
                                    {
                                        let value = if (target >= (0f64)) {
                                            self::read_required_boolean_fact(inhabited.clone(), target)
                                        } else {
                                            if (kind == "standard collection") {
                                                true
                                            } else {
                                                if ((kind == "copy") || (kind == "string")) {
                                                    self::read_required_boolean_fact(leaves.clone(), at)
                                                } else {
                                                    self::ml_array::includes(&{ let ml_reference_value184 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("vector"), String::from("pending vector"), String::from("option")]))).borrow().clone(); ml_reference_value184 }, &kind.clone(), 0f64, false, |a, b| a == b)
                                                }
                                            }
                                        };
                                        {
                                            let value_4 = if (((kind == "constructor") || (kind == "tuple")) || (kind == "box")) {
                                                {
                                                    let value_2 = true;
                                                    {
                                                        let edge = start;
                                                        {
                                                            let value_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop38(edges.clone(), values.clone(), end, value_2, edge);
                                                            value_3
                                                        }
                                                    }
                                                }
                                            } else {
                                                value
                                            };
                                            {
                                                let value_7 = if ((kind == "nominal") || (kind == "result")) {
                                                    {
                                                        let value_5 = false;
                                                        {
                                                            let edge_2 = start;
                                                            {
                                                                let value_6 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop39(edges.clone(), values.clone(), end, value_5, edge_2);
                                                                value_6
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    value_4
                                                };
                                                {
                                                    let values_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value185 = values.borrow().clone(); ml_reference_value185 }, vec![value_7])));
                                                    {
                                                        let at_2 = (at + 1f64);
                                                        {
                                                            (work, values, at) = (work_2, values_2.clone(), at_2);
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
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work, values.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop25(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut changed: bool, mut inhabited: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut ranks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut witnesses: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut rank: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result {
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
                                let source_ml_source1_s73 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop32(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, work, inhabited.clone(), values.clone(), at);
                                match source_ml_source1_s73.clone() {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Done(work_2, values_2) => {
                                        {
                                            let owner = 0f64;
                                            {
                                                let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop33(ends.clone(), edges.clone(), roots.clone(), size, eligible.clone(), changed_2, inhabited.clone(), ranks.clone(), witnesses.clone(), rank_2, values_2.clone(), owner);
                                                {
                                                    (work, changed, rank) = (work_2, changed_3, rank_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop32Return(source_ml_temporary_result37) => {
                                        self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Return(source_ml_temporary_result37.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop25Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(f64, bool),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop41(mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut changed: bool, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut owner: f64) -> bool {
    loop {
        return if (owner < size) {
            if (self::read_required_boolean_fact(eligible.clone(), owner) && !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(roots.clone(), owner)))) {
                {
                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored42 = { let ml_array_owner186 = eligible.clone(); let ml_array_argument186_0: f64 = owner; let ml_array_argument186_1: f64 = 1f64; let ml_array_argument186_2: bool = false; let ml_array_length = ml_array_owner186.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument186_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument186_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument186_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner186.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let changed_2 = true;
                        {
                            let owner_2 = (owner + 1f64);
                            {
                                (changed, owner) = (changed_2, owner_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let owner_3 = (owner + 1f64);
                    {
                        owner = owner_3;
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
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(f64, std::rc::Rc<std::cell::RefCell<Vec<bool>>>),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut end: f64, mut value: bool, mut edge: f64) -> bool {
    loop {
        return if (edge < end) {
            if !(self::read_required_boolean_fact(safe.clone(), self::read_required_number_fact(edges.clone(), edge))) {
                {
                    let value_2 = false;
                    {
                        let edge_2 = (edge + 1f64);
                        {
                            (value, edge) = (value_2, edge_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let edge_3 = (edge + 1f64);
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        } else {
            value
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop40(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut safe: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result {
    loop {
        return if (at < count) {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let kind = self::read_required_text_fact(kinds.clone(), at);
                        {
                            let target = self::read_required_number_fact(targets.clone(), at);
                            {
                                let start = if (at == (0f64)) {
                                    0f64
                                } else {
                                    self::read_required_number_fact(ends.clone(), (at - 1f64))
                                };
                                {
                                    let end = self::read_required_number_fact(ends.clone(), at);
                                    {
                                        let value = if (target >= (0f64)) {
                                            self::read_required_boolean_fact(eligible.clone(), target)
                                        } else {
                                            if (kind == "standard collection") {
                                                true
                                            } else {
                                                if ((kind == "copy") || (kind == "string")) {
                                                    self::read_required_boolean_fact(leaves.clone(), at)
                                                } else {
                                                    (self::ml_array::includes(&{ let ml_reference_value187 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("vector"), String::from("pending vector"), String::from("option"), String::from("result"), String::from("constructor"), String::from("nominal")]))).borrow().clone(); ml_reference_value187 }, &kind.clone(), 0f64, false, |a, b| a == b) || ((kind == "tuple") && self::read_required_boolean_fact(copy_nodes.clone(), at)))
                                                }
                                            }
                                        };
                                        {
                                            let value_3 = if (((((((kind == "box") || (kind == "vector")) || (kind == "pending vector")) || (kind == "option")) && (((end - start)) != (1f64))) || ((kind == "result") && (((end - start)) != (2f64)))) || ((kind == "nominal") && (start == end))) {
                                                {
                                                    let value_2 = false;
                                                    value_2
                                                }
                                            } else {
                                                value
                                            };
                                            {
                                                let edge = start;
                                                {
                                                    let value_4 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop43(edges.clone(), safe.clone(), end, value_3, edge);
                                                    {
                                                        let safe_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value188 = safe.borrow().clone(); ml_reference_value188 }, vec![value_4])));
                                                        {
                                                            let at_2 = (at + 1f64);
                                                            {
                                                                (work, safe, at) = (work_2, safe_2.clone(), at_2);
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(work, safe.clone())
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop24(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut leaves: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut size: f64, mut eligible: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut copy_nodes: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut work: f64, mut changed: bool) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s74 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop40(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), leaves.clone(), count, eligible.clone(), copy_nodes.clone(), work, safe.clone(), at);
                            match source_ml_source1_s74.clone() {
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Done(work_2, safe_2) => {
                                    {
                                        let owner = 0f64;
                                        {
                                            let changed_3 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop41(roots.clone(), size, eligible.clone(), changed_2, safe_2.clone(), owner);
                                            {
                                                (work, changed) = (work_2, changed_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop40Return(source_ml_temporary_result42) => {
                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Return(source_ml_temporary_result42.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop24Done(work, changed)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop44(mut count: f64, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    loop {
        return if (at < count) {
            {
                let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value189 = seen.borrow().clone(); ml_reference_value189 }, vec![false])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (seen, at) = (seen_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            seen.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result {
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(f64),
    MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc<std::cell::RefCell<Vec<String>>>),
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut edge: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (edge < (self::read_required_number_fact(ends.clone(), at))) {
            {
                let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value190 = pending.borrow().clone(); ml_reference_value190 }, vec![self::read_required_number_fact(edges.clone(), edge)])));
                {
                    let edge_2 = (edge + 1f64);
                    {
                        (pending, edge) = (pending_2.clone(), edge_2);
                        continue;
                    }
                }
            }
        } else {
            pending.clone()
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut work: f64, mut owner: f64, mut pending: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result {
    loop {
        return if { let ml_value = { let ml_reference_array192 = pending.clone(); let ml_reference_projection192 = ml_reference_array192.borrow().len() as f64; ml_reference_projection192 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let work_2 = (work + 1f64);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let at = self::read_required_number_fact(pending.clone(), ({ let ml_reference_array193 = pending.clone(); let ml_reference_projection193 = ml_reference_array193.borrow().len() as f64; ml_reference_projection193 } - 1f64));
                        {
                            let pending_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value194 = pending.borrow().clone(); ml_reference_value194 }, 0f64, (-1f64))));
                            if self::read_required_boolean_fact(seen.clone(), at) {
                                {
                                    (work, pending) = (work_2, pending_2.clone());
                                    continue;
                                }
                            } else {
                                {
                                    let source_ml_temporary_plan_rust_owned_recursive_nominal_owners_ignored46 = { let ml_array_owner195 = seen.clone(); let ml_array_argument195_0: f64 = at; let ml_array_argument195_1: f64 = 1f64; let ml_array_argument195_2: bool = true; let ml_array_length = ml_array_owner195.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument195_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument195_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument195_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner195.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                    {
                                        let kind = self::read_required_text_fact(kinds.clone(), at);
                                        {
                                            let target = self::read_required_number_fact(targets.clone(), at);
                                            if (target >= (0f64)) {
                                                if (target == owner) {
                                                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a recursive source datatype size cycle needs an actual Box or Vec indirection")])))
                                                } else {
                                                    {
                                                        let pending_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value196 = pending_2.borrow().clone(); ml_reference_value196 }, vec![self::read_required_number_fact(roots.clone(), target)])));
                                                        {
                                                            (work, pending) = (work_2, pending_3.clone());
                                                            continue;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (((kind == "box") || (kind == "vector")) || (kind == "pending vector")) {
                                                    {
                                                        (work, pending) = (work_2, pending_2.clone());
                                                        continue;
                                                    }
                                                } else {
                                                    {
                                                        let start = if (at == (0f64)) {
                                                            0f64
                                                        } else {
                                                            self::read_required_number_fact(ends.clone(), (at - 1f64))
                                                        };
                                                        {
                                                            let edge = start;
                                                            {
                                                                let pending_4 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop47(ends.clone(), edges.clone(), pending_2.clone(), at, edge);
                                                                {
                                                                    (work, pending) = (work_2, pending_4.clone());
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
        } else {
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(work)
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop23(mut kinds: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut size: f64, mut work: f64, mut owner: f64) -> self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result {
    loop {
        return if (owner < size) {
            {
                let work_2 = (work + count);
                if (work_2 > (1048576f64)) {
                    self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("recursive datatype reasoning exceeds its finite source profile")])))
                } else {
                    {
                        let pending = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(roots.clone(), owner)]));
                        {
                            let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                            {
                                let at = 0f64;
                                {
                                    let seen_2 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop44(count, seen.clone(), at);
                                    {
                                        let source_ml_source1_s75 = self::source_ml_temporary_plan_rust_owned_recursive_nominal_owners_loop45(kinds.clone(), ends.clone(), edges.clone(), targets.clone(), roots.clone(), work_2, owner, pending.clone(), seen_2.clone());
                                        match source_ml_source1_s75.clone() {
                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Done(work_3) => {
                                                {
                                                    let owner_2 = (owner + 1f64);
                                                    {
                                                        (work, owner) = (work_3, owner_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop45Return(source_ml_temporary_result45) => {
                                                self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Return(source_ml_temporary_result45.clone())
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
            self::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Result::MlTemporaryPlanRustOwnedRecursiveNominalOwnersLoop23Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result {
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done,
    MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_box_layer_clone_loop48(mut wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut copied: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut at: f64) -> self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result {
    loop {
        return if (at < ({ let ml_reference_array197 = owners.clone(); let ml_reference_projection197 = ml_reference_array197.borrow().len() as f64; ml_reference_projection197 })) {
            if ((self::read_required_text_fact(wrappers.clone(), at)) != "box") {
                self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(false)
            } else {
                if self::read_required_boolean_fact(copied.clone(), at) {
                    self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Return(true)
                } else {
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
            self::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Result::MlTemporaryAcceptRustOwnedBoxLayerCloneLoop48Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_owned_container_source(true, 90f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_owned_container_source(true, 99f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_owned_container_source(true, 100f64)) == (false)), "assertion 3");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box")])), String::from("library"), String::from("std::boxed::Box"), 1f64)) == (true)), "assertion 4");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box")])), String::from("declaration"), String::from("local::Box"), 1f64)) == (false)), "assertion 5");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box")])), String::from("library"), String::from("std::boxed::Box"), 1f64)) == (false)), "assertion 6");
    assert!(((self::accept_rust_owned_standard_sum_box_payload(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("box")])), 0f64, String::from("library"), String::from("std::boxed::Box"), 1f64, 2f64)) == (true)), "assertion 7");
    assert!(((self::accept_rust_owned_standard_sum_box_payload(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("reference")])), 1f64, String::from("library"), String::from("std::boxed::Box"), 1f64, 1f64)) == (false)), "assertion 8");
    assert!(((self::read_rust_owned_box_type_error(true, false, true, String::from("owned"), true)) == ""), "assertion 9");
    assert!(((self::read_rust_owned_box_type_error(true, true, true, String::from(""), false)) == ""), "assertion 10");
    assert!(((self::read_rust_owned_box_type_error(true, false, true, String::from(""), false)) == "an owned Box source type needs its complete closed payload ownership graph"), "assertion 11");
    assert!(((self::accept_rust_owned_box_call_result(true, true, true, true, true)) == (true)), "assertion 12");
    assert!(((self::accept_rust_owned_box_call_result(true, true, false, true, true)) == (false)), "assertion 13");
    assert!(((self::accept_rust_owned_box_call_result(true, true, true, true, false)) == (false)), "assertion 14");
    assert!(((self::read_rust_owned_box_move_error(true, true, false)) == ""), "assertion 15");
    assert!(((self::read_rust_owned_box_payload_error(false, true, true, false)) == "stored Box payload references or callable captures need their complete lifetime/drop profile"), "assertion 16");
    assert!(((self::read_rust_owned_box_payload_error(true, false, false, false)) == ""), "assertion 17");
    assert!(((self::read_rust_owned_container_depth_error(256f64, 256f64)) == ""), "assertion 18");
    assert!(((self::read_rust_owned_container_depth_error(257f64, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 19");
    assert!(((self::read_rust_owned_container_depth_error(f64::NAN, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 20");
    assert!(((self::read_rust_owned_container_depth_error(0f64, f64::INFINITY)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 21");
    assert!(((self::read_rust_owned_container_depth_error(1.5f64, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 22");
    assert!(((self::read_rust_owned_container_runtime_metadata_cost(String::from("owner"), 4f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 9f64])))), "assertion 23");
    assert!(((self::read_rust_owned_container_runtime_metadata_cost(String::from("origin"), 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![16f64, 256f64])))), "assertion 24");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), false, true, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), String::from("")])))), "assertion 25");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, true, 2f64, 1f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), String::from("Clone")])))), "assertion 26");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(false, String::from(""), false, String::from("owned"), true, true, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 27");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, false, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 28");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, true, (-1f64), 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 29");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(false, String::from("copy"), true, String::from(""), false, false, (-1f64), (-1f64))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("Clone")])))), "assertion 30");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("owned"), true, 0f64)) == (true)), "assertion 31");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("copy"), false, 0f64)) == (false)), "assertion 32");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("owned"), true, 1f64)) == (false)), "assertion 33");
    assert!(((self::read_rust_owned_vector_named_admission(String::from("refuse"), true)) == "admit"), "assertion 34");
    assert!(((self::read_rust_owned_vector_named_admission(String::from("defer"), true)) == "defer"), "assertion 35");
    assert!(((self::read_rust_owned_vector_signature_phase(true, true, false, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from(""), false)) == "defer"), "assertion 36");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from("owned"), true)) == "admit"), "assertion 37");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("declaration"), String::from("local::Vec"), 1f64, String::from("named"), String::from("owned"), true)) == ""), "assertion 38");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from("owned"), false)) == ""), "assertion 39");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, true, false, false)) == ""), "assertion 40");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, false, false, false)) == "an owned vector element needs its actual available binding or completed producer"), "assertion 41");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, false, false, true)) == ""), "assertion 42");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), false, true, true, true, true)) == "owned vector construction needs each actual complete matching element type"), "assertion 43");
    assert!(((self::accept_rust_owned_vector_call_result(true, true, true, true, true)) == (true)), "assertion 44");
    assert!(((self::accept_rust_owned_vector_call_result(true, true, true, true, false)) == (false)), "assertion 45");
    assert!(((self::accept_rust_owned_vector_produced_value(String::from("vector"), String::from("owned"), true, true)) == (true)), "assertion 46");
    assert!(((self::accept_rust_owned_vector_produced_value(String::from("vector"), String::from("owned"), true, false)) == (false)), "assertion 47");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, true, false, false)) == (true)), "assertion 48");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, false, false, true)) == (true)), "assertion 49");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, false, false, false)) == (false)), "assertion 50");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, true, true, true)) == (false)), "assertion 51");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("std::vec::Vec"), String::from("new"), 0f64)) == "construct"), "assertion 52");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("local::Vec"), String::from("new"), 0f64)) == ""), "assertion 53");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("std::vec::Vec"), String::from("new"), 1f64)) == "arity"), "assertion 54");
    assert!(((self::read_rust_owned_vector_clone_error(false, true, false, true, true, true, true)) == "vector Clone needs its actual closed element Clone implementation"), "assertion 55");
    assert!(((self::read_rust_owned_vector_clone_error(true, true, true, true, false, false, false)) == ""), "assertion 56");
    assert!(((self::read_rust_owned_vector_clone_error(false, true, true, true, false, false, false)) == "vector Clone needs its actual available source value or actual producer"), "assertion 57");
    assert!(((self::read_rust_owned_box_clone_source_error(true, true, true, false, true, false, false)) == ""), "assertion 58");
    assert!(((self::read_rust_owned_box_clone_source_error(true, true, false, false, true, true, true)) == "Box Clone needs its actual admitted payload Clone implementation"), "assertion 59");
    assert!(((self::accept_rust_owned_argument_read_check(1f64, true)) == (false)), "assertion 60");
    assert!(((self::accept_rust_owned_argument_read_check(1f64, false)) == (true)), "assertion 61");
    assert!(((self::read_rust_owned_vector_element_count_error(65536f64)) == ""), "assertion 62");
    assert!(((self::read_rust_owned_vector_element_count_error(65537f64)) == "owned vector construction exceeds its finite actual element profile"), "assertion 63");
    assert!(((self::read_rust_owned_vector_element_count_error(f64::NAN)) == "owned vector construction exceeds its finite actual element profile"), "assertion 64");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(true, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])))) == "a nongeneric datatype reference field requires its explicit static lifetime"), "assertion 65");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(true, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("static")])))) == ""), "assertion 66");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(false, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])))) == ""), "assertion 67");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Tree")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box"), String::from("constructor"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 2f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from("Clone"), String::from("closed"), String::from("1"), String::from("1")])))), "assertion 68");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Tree")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from(""), String::from("closed"), String::from("1"), String::from("0")])))), "assertion 69");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Bad")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a recursive source datatype size cycle needs an actual Box or Vec indirection")])))), "assertion 70");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Endless")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from(""), String::from(""), String::from(""), String::from("-1"), String::from("-1")])))), "assertion 71");
    assert!(((self::read_rust_owned_vector_element_role(String::from(""), true, true, false, String::from(""))) == "copy"), "assertion 72");
    assert!(((self::read_rust_owned_vector_element_role(String::from(""), false, true, false, String::from(""))) == ""), "assertion 73");
    assert!(((self::read_rust_owned_vector_element_role(String::from("named"), true, true, false, String::from(""))) == ""), "assertion 74");
    assert!(((self::read_rust_owned_vector_element_role(String::from("named"), false, true, false, String::from("owned"))) == "owned"), "assertion 75");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("fixed"), false, true)) == (true)), "assertion 76");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("fixed"), false, false)) == (false)), "assertion 77");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("named"), false, true)) == (false)), "assertion 78");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("foreign.Message"), true)) == (true)), "assertion 79");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("local.Message"), true)) == (false)), "assertion 80");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from(""), String::from(""), true)) == (false)), "assertion 81");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("foreign.Message"), false)) == (false)), "assertion 82");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("alloc::boxed::Box"), 1f64)) == (true)), "assertion 83");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("declaration"), String::from("alloc::boxed::Box"), 1f64)) == (false)), "assertion 84");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("alloc::boxed::Box"), 2f64)) == (false)), "assertion 85");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("std::boxed::Box"), 1f64)) == (false)), "assertion 86");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_owned_recursive_container_controls() { main(); }
