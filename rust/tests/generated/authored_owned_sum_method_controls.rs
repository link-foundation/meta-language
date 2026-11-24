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

pub fn read_rust_primitive_owner_path(segments: Vec<String>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        {
            let owner = String::from("");
            {
                let owner_4 = if ((((segments.len() as f64)) == (2f64)) && !absolute) {
                    {
                        let owner_2 = (self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""));
                        owner_2.clone()
                    }
                } else {
                    if (((((segments.len() as f64)) == (4f64)) && ({
                        let source_ml_source1_comparison_operand1 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
                        {
                            let source_ml_source1_comparison_operand2 = String::from("std");
                            match (source_ml_source1_comparison_operand1.clone(), source_ml_source1_comparison_operand2.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } || {
                        let source_ml_source1_comparison_operand3 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
                        {
                            let source_ml_source1_comparison_operand4 = String::from("core");
                            match (source_ml_source1_comparison_operand3.clone(), source_ml_source1_comparison_operand4.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    })) && {
                        let source_ml_source1_comparison_operand5 = self::ml_array::optional_at(&segments, self::ml_array::number_index(1f64));
                        {
                            let source_ml_source1_comparison_operand6 = String::from("primitive");
                            match (source_ml_source1_comparison_operand5.clone(), source_ml_source1_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    }) {
                        {
                            let owner_3 = (self::ml_array::optional_at(&segments, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| String::from(""));
                            owner_3.clone()
                        }
                    } else {
                        owner.clone()
                    }
                };
                if self::ml_array::includes(&vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize")], &owner_4.clone(), 0f64, false, |a, b| a == b) {
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
    ((method == "from") && self::ml_array::includes(&vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize"), String::from("f64")], &owner.clone(), 0f64, false, |a, b| a == b))
}

pub fn read_rust_primitive_from_owner(segments: Vec<String>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        if ((((((segments.len() as f64)) == (2f64)) && !absolute) && {
            let source_ml_source1_comparison_operand7 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand8 = String::from("f64");
                match (source_ml_source1_comparison_operand7.clone(), source_ml_source1_comparison_operand8.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) || ((((((segments.len() as f64)) == (4f64)) && ({
            let source_ml_source1_comparison_operand9 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand10 = String::from("std");
                match (source_ml_source1_comparison_operand9.clone(), source_ml_source1_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } || {
            let source_ml_source1_comparison_operand11 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand12 = String::from("core");
                match (source_ml_source1_comparison_operand11.clone(), source_ml_source1_comparison_operand12.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) && {
            let source_ml_source1_comparison_operand13 = self::ml_array::optional_at(&segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source1_comparison_operand14 = String::from("primitive");
                match (source_ml_source1_comparison_operand13.clone(), source_ml_source1_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && {
            let source_ml_source1_comparison_operand15 = self::ml_array::optional_at(&segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source1_comparison_operand16 = String::from("f64");
                match (source_ml_source1_comparison_operand15.clone(), source_ml_source1_comparison_operand16.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) {
            String::from("f64")
        } else {
            self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
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
                if ((target == "f64") && self::ml_array::includes(&vec![String::from("i8"), String::from("i16"), String::from("i32"), String::from("u8"), String::from("u16"), String::from("u32")], &source.clone(), 0f64, false, |a, b| a == b)) {
                    String::from("floating exact")
                } else {
                    {
                        let admitted = (((((((((source == "u8") && self::ml_array::includes(&vec![String::from("u16"), String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b)) || ((source == "u16") && self::ml_array::includes(&vec![String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i32"), String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u32") && self::ml_array::includes(&vec![String::from("u64"), String::from("u128"), String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u64") && self::ml_array::includes(&vec![String::from("u128"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i8") && self::ml_array::includes(&vec![String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i16") && self::ml_array::includes(&vec![String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i32") && self::ml_array::includes(&vec![String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i64") && (target == "i128")));
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

pub fn read_rust_primitive_default_owner(segments: Vec<String>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if {
        let source_ml_source1_comparison_operand17 = self::ml_array::optional_at(&segments, self::ml_array::number_index(((segments.len() as f64) - 1f64)));
        {
            let source_ml_source1_comparison_operand18 = String::from("default");
            match (source_ml_source1_comparison_operand17.clone(), source_ml_source1_comparison_operand18.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    } {
        self::read_rust_primitive_from_owner(segments, ordinary, shadowed, absolute)
    } else {
        String::from("")
    }
}

pub fn read_rust_primitive_default_term(owner: String) -> String {
    if (owner == "bool") {
        String::from("false")
    } else {
        if (owner == "f64") {
            String::from("positive floating zero")
        } else {
            if self::ml_array::includes(&vec![String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize")], &owner.clone(), 0f64, false, |a, b| a == b) {
                String::from("integer zero")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_primitive_default_import(path: String, ordinary: bool, shadowed: bool) -> bool {
    ((ordinary && !shadowed) && ((path == "std::default::Default") || (path == "core::default::Default")))
}

pub fn read_rust_owned_standard_sum_method_arity(owner: String, method: String) -> f64 {
    if (owner == "std::option::Option") {
        if ((method == "unwrap_or") || (method == "ok_or")) {
            1f64
        } else {
            if (method == "unwrap_or_default") {
                0f64
            } else {
                (-1f64)
            }
        }
    } else {
        if (owner == "std::result::Result") {
            if (method == "unwrap_or") {
                1f64
            } else {
                if (((method == "ok") || (method == "err")) || (method == "unwrap_or_default")) {
                    0f64
                } else {
                    (-1f64)
                }
            }
        } else {
            (-1f64)
        }
    }
}

pub fn read_rust_owned_standard_sum_method_receiver_mode(owner: String, method: String, role: String) -> String {
    if ((self::read_rust_owned_standard_sum_method_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        if (role == "owned") {
            String::from("owned")
        } else {
            if ((role == "copy") && ((method == "unwrap_or_default") || ((owner == "std::option::Option") && (method == "ok_or")))) {
                String::from("copy")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_owned_standard_sum_method_call_production(role: String, function_: bool, recorded: bool, identity: bool, wrappers: f64, borrowed: bool) -> bool {
    ((((((role == "owned") && function_) && recorded) && identity) && (wrappers == (0f64))) && !borrowed)
}

pub fn read_rust_owned_standard_sum_method_version_error(owner: String, method: String, minor: f64) -> String {
    if ((self::read_rust_owned_standard_sum_method_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("the consuming owned standard sum method is outside this profile")
    } else {
        if ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64))) {
            String::from("")
        } else {
            String::from("the consuming owned standard sum method source version is uncovered")
        }
    }
}

pub fn read_rust_owned_standard_sum_method_result(owner: String, method: String) -> String {
    if ((self::read_rust_owned_standard_sum_method_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        if ((method == "unwrap_or") || (method == "unwrap_or_default")) {
            String::from("payload")
        } else {
            if (method == "ok_or") {
                String::from("std::result::Result")
            } else {
                String::from("std::option::Option")
            }
        }
    }
}

pub fn plan_rust_owned_standard_sum_method_arguments(owner: String, method: String) -> Vec<f64> {
    if ((self::read_rust_owned_standard_sum_method_arity(owner.clone(), method.clone())) < (0f64)) {
        Vec::<f64>::new()
    } else {
        if (method == "ok_or") {
            vec![0f64, (-1f64)]
        } else {
            if (method == "ok") {
                vec![0f64]
            } else {
                if (method == "err") {
                    vec![1f64]
                } else {
                    Vec::<f64>::new()
                }
            }
        }
    }
}

pub fn plan_rust_owned_standard_sum_method_branch(owner: String, variant: String, method: String) -> Vec<String> {
    {
        let source_ml_source1_s19 = if (owner == "std::option::Option") {
            if (method == "unwrap_or") {
                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if (variant == "Some") {
                    vec![String::from(""), String::from("payload")]
                } else {
                    if (variant == "None") {
                        vec![String::from(""), String::from("argument")]
                    } else {
                        Vec::<String>::new()
                    }
                })
            } else {
                if (method == "unwrap_or_default") {
                    self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if (variant == "Some") {
                        vec![String::from(""), String::from("payload")]
                    } else {
                        if (variant == "None") {
                            vec![String::from(""), String::from("standard default")]
                        } else {
                            Vec::<String>::new()
                        }
                    })
                } else {
                    if (method == "ok_or") {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(if (variant == "Some") {
                            vec![String::from("Ok"), String::from("payload")]
                        } else {
                            if (variant == "None") {
                                vec![String::from("Err"), String::from("argument")]
                            } else {
                                Vec::<String>::new()
                            }
                        })
                    } else {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next
        };
        match source_ml_source1_s19.clone() {
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next => {
                {
                    let source_ml_source1_s20 = if (owner == "std::result::Result") {
                        if (method == "unwrap_or") {
                            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if (variant == "Ok") {
                                vec![String::from(""), String::from("payload")]
                            } else {
                                if (variant == "Err") {
                                    vec![String::from(""), String::from("argument")]
                                } else {
                                    Vec::<String>::new()
                                }
                            })
                        } else {
                            if (method == "unwrap_or_default") {
                                self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if (variant == "Ok") {
                                    vec![String::from(""), String::from("payload")]
                                } else {
                                    if (variant == "Err") {
                                        vec![String::from(""), String::from("standard default")]
                                    } else {
                                        Vec::<String>::new()
                                    }
                                })
                            } else {
                                if (method == "ok") {
                                    self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if (variant == "Ok") {
                                        vec![String::from("Some"), String::from("payload")]
                                    } else {
                                        if (variant == "Err") {
                                            vec![String::from("None"), String::from("empty")]
                                        } else {
                                            Vec::<String>::new()
                                        }
                                    })
                                } else {
                                    if (method == "err") {
                                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(if (variant == "Err") {
                                            vec![String::from("Some"), String::from("payload")]
                                        } else {
                                            if (variant == "Ok") {
                                                vec![String::from("None"), String::from("empty")]
                                            } else {
                                                Vec::<String>::new()
                                            }
                                        })
                                    } else {
                                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next
                    };
                    match source_ml_source1_s20.clone() {
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next => {
                            Vec::<String>::new()
                        }
                        self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
            self::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1::MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(source_ml_temporary_result2_2) => {
                source_ml_temporary_result2_2
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_method_capture_error(role: String, wrappers: f64, borrowed: bool, tracking: bool, before: String, after: String, available: bool, produced: bool) -> String {
    if ((((((((role == "owned") && (wrappers == (0f64))) && !borrowed) && tracking) && (after != "")) && (after != before)) && available) && produced) {
        String::from("")
    } else {
        String::from("the consuming value needs its actual move into a fresh tracked owned temporary")
    }
}

pub fn accept_rust_owned_standard_sum_method_argument(role: String) -> bool {
    ((role == "copy") || (role == "owned"))
}

pub fn accept_rust_owned_standard_sum_method_production(role: String, receiver: bool, argument: bool) -> bool {
    (((role == "owned") && receiver) && argument)
}

pub fn read_rust_owned_standard_sum_default_term(kind: String, owner_kind: String, owner: String, storage: String, wrappers: f64, proven: bool) -> String {
    if (!proven || (wrappers != (0f64))) {
        String::from("")
    } else {
        if ((((kind == "string") && (owner_kind == "library")) && (storage == "owned string")) && self::ml_array::includes(&vec![String::from("alloc::string::String"), String::from("std::string::String")], &owner.clone(), 0f64, false, |a, b| a == b)) {
            String::from("empty owned string")
        } else {
            if ((((kind == "array") && (owner_kind == "library")) && (storage == "owned vector")) && self::ml_array::includes(&vec![String::from("alloc::vec::Vec"), String::from("std::vec::Vec")], &owner.clone(), 0f64, false, |a, b| a == b)) {
                String::from("empty owned vector")
            } else {
                if ((kind == "named") && (owner == "std::option::Option")) {
                    String::from("empty actual Option")
                } else {
                    if ((kind == "unit") && (owner_kind == "unit")) {
                        String::from("unit")
                    } else {
                        {
                            let primitive = if (owner_kind == "primitive") {
                                self::read_rust_primitive_default_term(owner.clone())
                            } else {
                                String::from("")
                            };
                            if ((((kind == "bool") && (primitive == "false")) || ((kind == "float") && (primitive == "positive floating zero"))) || ((kind == "fixed") && (primitive == "integer zero"))) {
                                primitive.clone()
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

pub fn read_rust_standard_sum_clone_source_error(owner: String, role: String, wrappers: f64, borrowed: bool, tracking: bool, available: bool, produced: bool) -> String {
    if ((!(self::ml_array::includes(&vec![String::from("std::option::Option"), String::from("std::result::Result")], &owner.clone(), 0f64, false, |a, b| a == b)) || (wrappers != (0f64))) || borrowed) {
        String::from("standard sum clone needs its actual direct ordinary value source")
    } else {
        if ((role == "copy") || (((role == "owned") && tracking) && (available || produced))) {
            String::from("")
        } else {
            String::from("standard sum clone needs an available actual owner or an actual fresh producer, independently of its type")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2 {
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Next,
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1 {
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Next,
    MlTemporaryPlanRustOwnedStandardSumMethodBranchJoin1Return(Vec<String>),
}

fn ml_main() {
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::option::Option"), String::from("ok_or"), String::from("copy"))) == "copy"), "assertion 1");
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::option::Option"), String::from("ok_or"), String::from("owned"))) == "owned"), "assertion 2");
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::result::Result"), String::from("ok"), String::from("copy"))) == ""), "assertion 3");
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::option::Option"), String::from("unwrap_or"), String::from("copy"))) == ""), "assertion 4");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), true, true, true, 0f64, false)) == (true)), "assertion 5");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), true, false, true, 0f64, false)) == (false)), "assertion 6");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), false, true, true, 0f64, false)) == (false)), "assertion 7");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), true, true, false, 0f64, false)) == (false)), "assertion 8");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), true, true, true, 1f64, false)) == (false)), "assertion 9");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("owned"), true, true, true, 0f64, true)) == (false)), "assertion 10");
    assert!(((self::accept_rust_owned_standard_sum_method_call_production(String::from("copy"), true, true, true, 0f64, false)) == (false)), "assertion 11");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::option::Option"), String::from("unwrap_or")), 1f64), "assertion 12");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::option::Option"), String::from("ok_or")), 1f64), "assertion 13");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::result::Result"), String::from("unwrap_or")), 1f64), "assertion 14");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::result::Result"), String::from("ok")), 0f64), "assertion 15");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::result::Result"), String::from("err")), 0f64), "assertion 16");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::option::Option"), String::from("unwrap_or_else")), (-1f64)), "assertion 17");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("user::Option"), String::from("unwrap_or")), (-1f64)), "assertion 18");
    assert!(((self::read_rust_owned_standard_sum_method_version_error(String::from("std::option::Option"), String::from("unwrap_or"), 90f64)) == ""), "assertion 19");
    assert!(((self::read_rust_owned_standard_sum_method_version_error(String::from("std::result::Result"), String::from("ok"), 99f64)) == ""), "assertion 20");
    assert!(((self::read_rust_owned_standard_sum_method_version_error(String::from("std::option::Option"), String::from("unwrap_or"), 89f64)) == "the consuming owned standard sum method source version is uncovered"), "assertion 21");
    assert!(((self::read_rust_owned_standard_sum_method_version_error(String::from("std::result::Result"), String::from("err"), 100f64)) == "the consuming owned standard sum method source version is uncovered"), "assertion 22");
    assert!(((self::read_rust_owned_standard_sum_method_result(String::from("std::option::Option"), String::from("unwrap_or"))) == "payload"), "assertion 23");
    assert!(((self::read_rust_owned_standard_sum_method_result(String::from("std::option::Option"), String::from("ok_or"))) == "std::result::Result"), "assertion 24");
    assert!(((self::read_rust_owned_standard_sum_method_result(String::from("std::result::Result"), String::from("ok"))) == "std::option::Option"), "assertion 25");
    assert!(((self::read_rust_owned_standard_sum_method_result(String::from("std::result::Result"), String::from("err"))) == "std::option::Option"), "assertion 26");
    assert!(((self::plan_rust_owned_standard_sum_method_arguments(String::from("std::option::Option"), String::from("ok_or"))) == (vec![0f64, (-1f64)])), "assertion 27");
    assert!(((self::plan_rust_owned_standard_sum_method_arguments(String::from("std::result::Result"), String::from("ok"))) == (vec![0f64])), "assertion 28");
    assert!(((self::plan_rust_owned_standard_sum_method_arguments(String::from("std::result::Result"), String::from("err"))) == (vec![1f64])), "assertion 29");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("Some"), String::from("unwrap_or"))) == (vec![String::from(""), String::from("payload")])), "assertion 30");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("None"), String::from("unwrap_or"))) == (vec![String::from(""), String::from("argument")])), "assertion 31");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Ok"), String::from("unwrap_or"))) == (vec![String::from(""), String::from("payload")])), "assertion 32");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Err"), String::from("unwrap_or"))) == (vec![String::from(""), String::from("argument")])), "assertion 33");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("Some"), String::from("ok_or"))) == (vec![String::from("Ok"), String::from("payload")])), "assertion 34");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("None"), String::from("ok_or"))) == (vec![String::from("Err"), String::from("argument")])), "assertion 35");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Ok"), String::from("ok"))) == (vec![String::from("Some"), String::from("payload")])), "assertion 36");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Err"), String::from("ok"))) == (vec![String::from("None"), String::from("empty")])), "assertion 37");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Ok"), String::from("err"))) == (vec![String::from("None"), String::from("empty")])), "assertion 38");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Err"), String::from("err"))) == (vec![String::from("Some"), String::from("payload")])), "assertion 39");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("Other"), String::from("unwrap_or"))) == (Vec::<String>::new())), "assertion 40");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from("old"), String::from("new"), true, true)) == ""), "assertion 41");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from(""), String::from("new"), true, true)) == ""), "assertion 42");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from("same"), String::from("same"), true, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 43");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from(""), String::from(""), true, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 44");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, true, true, String::from("old"), String::from("new"), true, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 45");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 1f64, false, true, String::from("old"), String::from("new"), true, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 46");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, false, String::from("old"), String::from("new"), true, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 47");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from("old"), String::from("new"), false, true)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 48");
    assert!(((self::read_rust_owned_standard_sum_method_capture_error(String::from("owned"), 0f64, false, true, String::from("old"), String::from("new"), true, false)) == "the consuming value needs its actual move into a fresh tracked owned temporary"), "assertion 49");
    assert!(((self::accept_rust_owned_standard_sum_method_argument(String::from("copy"))) == (true)), "assertion 50");
    assert!(((self::accept_rust_owned_standard_sum_method_argument(String::from("owned"))) == (true)), "assertion 51");
    assert!(((self::accept_rust_owned_standard_sum_method_argument(String::from("reference"))) == (false)), "assertion 52");
    assert!(((self::accept_rust_owned_standard_sum_method_production(String::from("owned"), true, true)) == (true)), "assertion 53");
    assert!(((self::accept_rust_owned_standard_sum_method_production(String::from("copy"), true, true)) == (false)), "assertion 54");
    assert!(((self::accept_rust_owned_standard_sum_method_production(String::from("owned"), false, true)) == (false)), "assertion 55");
    assert!(((self::accept_rust_owned_standard_sum_method_production(String::from("owned"), true, false)) == (false)), "assertion 56");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("library"), String::from("std::string::String"), String::from("owned string"), 0f64, true)) == "empty owned string"), "assertion 57");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("library"), String::from("alloc::string::String"), String::from("owned string"), 0f64, true)) == "empty owned string"), "assertion 58");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("array"), String::from("library"), String::from("std::vec::Vec"), String::from("owned vector"), 0f64, true)) == "empty owned vector"), "assertion 59");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("array"), String::from("library"), String::from("alloc::vec::Vec"), String::from("owned vector"), 0f64, true)) == "empty owned vector"), "assertion 60");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("named"), String::from("declaration"), String::from("std::option::Option"), String::from(""), 0f64, true)) == "empty actual Option"), "assertion 61");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("unit"), String::from("unit"), String::from(""), String::from(""), 0f64, true)) == "unit"), "assertion 62");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("fixed"), String::from("primitive"), String::from("u8"), String::from(""), 0f64, true)) == "integer zero"), "assertion 63");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("fixed"), String::from("primitive"), String::from("usize"), String::from(""), 0f64, true)) == "integer zero"), "assertion 64");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("float"), String::from("primitive"), String::from("f64"), String::from(""), 0f64, true)) == "positive floating zero"), "assertion 65");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("bool"), String::from("primitive"), String::from("bool"), String::from(""), 0f64, true)) == "false"), "assertion 66");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("library"), String::from("std::string::String"), String::from("owned string"), 1f64, true)) == ""), "assertion 67");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("declaration"), String::from("std::string::String"), String::from("owned string"), 0f64, true)) == ""), "assertion 68");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("array"), String::from("library"), String::from("std::vec::Vec"), String::from("fixed array"), 0f64, true)) == ""), "assertion 69");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("array"), String::from("library"), String::from("local::Vec"), String::from("owned vector"), 0f64, true)) == ""), "assertion 70");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("named"), String::from("declaration"), String::from("local::Option"), String::from(""), 0f64, true)) == ""), "assertion 71");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("named"), String::from("declaration"), String::from("std::result::Result"), String::from(""), 0f64, true)) == ""), "assertion 72");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("fixed"), String::from("declaration"), String::from("u8"), String::from(""), 0f64, true)) == ""), "assertion 73");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("library"), String::from("std::string::String"), String::from("owned string"), 0f64, false)) == ""), "assertion 74");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::option::Option"), String::from("unwrap_or_default")), 0f64), "assertion 75");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_method_arity(String::from("std::result::Result"), String::from("unwrap_or_default")), 0f64), "assertion 76");
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::option::Option"), String::from("unwrap_or_default"), String::from("copy"))) == "copy"), "assertion 77");
    assert!(((self::read_rust_owned_standard_sum_method_receiver_mode(String::from("std::result::Result"), String::from("unwrap_or_default"), String::from("owned"))) == "owned"), "assertion 78");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::option::Option"), String::from("None"), String::from("unwrap_or_default"))) == (vec![String::from(""), String::from("standard default")])), "assertion 79");
    assert!(((self::plan_rust_owned_standard_sum_method_branch(String::from("std::result::Result"), String::from("Err"), String::from("unwrap_or_default"))) == (vec![String::from(""), String::from("standard default")])), "assertion 80");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("string"), String::from("primitive"), String::from("u8"), String::from(""), 0f64, true)) == ""), "assertion 81");
    assert!(((self::read_rust_owned_standard_sum_default_term(String::from("fixed"), String::from("primitive"), String::from("bool"), String::from(""), 0f64, true)) == ""), "assertion 82");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("copy"), 0f64, false, false, false, false)) == ""), "assertion 83");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("owned"), 0f64, false, true, true, false)) == ""), "assertion 84");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::result::Result"), String::from("owned"), 0f64, false, true, false, true)) == ""), "assertion 85");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("owned"), 0f64, false, true, false, false)) == "standard sum clone needs an available actual owner or an actual fresh producer, independently of its type"), "assertion 86");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("owned"), 0f64, false, false, false, true)) == "standard sum clone needs an available actual owner or an actual fresh producer, independently of its type"), "assertion 87");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from(""), 0f64, false, true, true, true)) == "standard sum clone needs an available actual owner or an actual fresh producer, independently of its type"), "assertion 88");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("local::Option"), String::from("owned"), 0f64, false, true, true, true)) == "standard sum clone needs its actual direct ordinary value source"), "assertion 89");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("owned"), 1f64, false, true, true, true)) == "standard sum clone needs its actual direct ordinary value source"), "assertion 90");
    assert!(((self::read_rust_standard_sum_clone_source_error(String::from("std::option::Option"), String::from("owned"), 0f64, true, true, true, true)) == "standard sum clone needs its actual direct ordinary value source"), "assertion 91");
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
fn all_literal_owned_sum_method_controls() { main(); }
