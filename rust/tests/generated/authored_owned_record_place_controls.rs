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

pub fn read_rust_owned_record_place_depth_limit() -> f64 {
    256f64
}

pub fn read_rust_owned_record_place_token_count(forms: Vec<String>) -> f64 {
    {
        let depth = 0f64;
        {
            let atom = true;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_read_rust_owned_record_place_token_count_loop1(forms, depth, atom, at);
                    match source_ml_source1_s1.clone() {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done => {
                            (-1f64)
                        }
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result {
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done,
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(f64),
}

pub fn source_ml_temporary_read_rust_owned_record_place_token_count_loop1(mut forms: Vec<String>, mut depth: f64, mut atom: bool, mut at: f64) -> self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result {
    loop {
        return if (at < ((forms.len() as f64))) {
            {
                let form = self::ml_array::optional_at(&forms, self::ml_array::number_index(at));
                if atom {
                    if {
                        let source_ml_source1_comparison_operand2 = form.clone();
                        {
                            let source_ml_source1_comparison_operand3 = String::from("(");
                            match (source_ml_source1_comparison_operand2.clone(), source_ml_source1_comparison_operand3.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } {
                        {
                            let depth_2 = (depth + 1f64);
                            if (depth_2 >= (self::read_rust_owned_record_place_depth_limit())) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return((-1f64))
                            } else {
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (depth, at) = (depth_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if {
                            let source_ml_source1_comparison_operand4 = form.clone();
                            {
                                let source_ml_source1_comparison_operand5 = String::from("name");
                                match (source_ml_source1_comparison_operand4.clone(), source_ml_source1_comparison_operand5.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        } {
                            {
                                let atom_2 = false;
                                {
                                    let at_3 = (at + 1f64);
                                    {
                                        (atom, at) = (atom_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return((-1f64))
                        }
                    }
                } else {
                    if {
                        let source_ml_source1_comparison_operand6 = form.clone();
                        {
                            let source_ml_source1_comparison_operand7 = String::from(".");
                            match (source_ml_source1_comparison_operand6.clone(), source_ml_source1_comparison_operand7.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } {
                        {
                            let at_4 = (at + 1f64);
                            if ({
                                let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(&forms, self::ml_array::number_index(at_4));
                                {
                                    let source_ml_source1_comparison_operand9 = String::from("name");
                                    !(match (source_ml_source1_comparison_operand8.clone(), source_ml_source1_comparison_operand9.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }
                            } && {
                                let source_ml_source1_comparison_operand10 = self::ml_array::optional_at(&forms, self::ml_array::number_index(at_4));
                                {
                                    let source_ml_source1_comparison_operand11 = String::from("number");
                                    !(match (source_ml_source1_comparison_operand10.clone(), source_ml_source1_comparison_operand11.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }
                            }) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return((-1f64))
                            } else {
                                {
                                    let at_5 = (at_4 + 1f64);
                                    {
                                        at = at_5;
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if {
                            let source_ml_source1_comparison_operand12 = form.clone();
                            {
                                let source_ml_source1_comparison_operand13 = String::from(")");
                                match (source_ml_source1_comparison_operand12.clone(), source_ml_source1_comparison_operand13.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        } {
                            if (depth == (0f64)) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return((-1f64))
                            } else {
                                {
                                    let depth_3 = (depth - 1f64);
                                    {
                                        let at_6 = (at + 1f64);
                                        {
                                            (depth, at) = (depth_3, at_6);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            if {
                                let source_ml_source1_comparison_operand14 = form.clone();
                                {
                                    let source_ml_source1_comparison_operand15 = String::from("}");
                                    match (source_ml_source1_comparison_operand14.clone(), source_ml_source1_comparison_operand15.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                }
                            } {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(if (depth == (0f64)) {
                                    at
                                } else {
                                    (-1f64)
                                })
                            } else {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return((-1f64))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done
        };
    }
}

fn ml_main() {
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_depth_limit(), 256f64), "assertion 1");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("name"), String::from("}")]), 1f64), "assertion 2");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("("), String::from("name"), String::from(")"), String::from("}")]), 3f64), "assertion 3");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("("), String::from("name"), String::from("."), String::from("name"), String::from(")"), String::from("."), String::from("name"), String::from("}")]), 7f64), "assertion 4");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("name"), String::from("."), String::from("number"), String::from("}")]), 3f64), "assertion 5");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("("), String::from("("), String::from("name"), String::from(")"), String::from(")"), String::from("}")]), 5f64), "assertion 6");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("name"), String::from("("), String::from(")"), String::from("}")]), (-1f64)), "assertion 7");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("{"), String::from("name"), String::from("}"), String::from("}")]), (-1f64)), "assertion 8");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("("), String::from("name"), String::from("}")]), (-1f64)), "assertion 9");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("name"), String::from("."), String::from("}")]), (-1f64)), "assertion 10");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("number"), String::from("}")]), (-1f64)), "assertion 11");
    assert!(self::ml_number::same_value(self::read_rust_owned_record_place_token_count(vec![String::from("name"), String::from("other"), String::from("name"), String::from("}")]), (-1f64)), "assertion 12");
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
fn all_literal_owned_record_place_controls() { main(); }
