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

pub fn read_rust_owned_record_loop_operation() -> String {
    String::from("rustOwnedRecordFiniteLoop")
}

pub fn read_rust_owned_record_loop_chunk_size() -> f64 {
    8f64
}

pub fn read_rust_owned_record_loop_chunk_name(loop_: f64, chunk: f64, collision: f64) -> String {
    if ((!(self::accept_rust_owned_record_loop_count(loop_)) || !(self::accept_rust_owned_record_loop_count(chunk))) || !(self::accept_rust_owned_record_loop_count(collision))) {
        String::from("")
    } else {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_finite_record_loop_chunk_")), ({
            let source_ml_source2_string_argument1 = loop_;
            self::ml_number::js_number(source_ml_source2_string_argument1)
        }))), (String::from("_")))), ({
            let source_ml_source2_string_argument2 = chunk;
            self::ml_number::js_number(source_ml_source2_string_argument2)
        }))), (if (collision == (0f64)) {
            String::from("")
        } else {
            format!("{}{}", (String::from("_")), ({
                let source_ml_source2_string_argument3 = collision;
                self::ml_number::js_number(source_ml_source2_string_argument3)
            }))
        }))
    }
}

pub fn test_rust_owned_record_loop_chunk_name(name: String) -> bool {
    name.starts_with("ml_finite_record_loop_chunk_")
}

pub fn read_rust_owned_record_loop_helper_error(generated: bool, recursive: bool, mutual: bool, unknown: bool) -> String {
    if !generated {
        String::from("a finite record loop chunk needs its actual generated declaration")
    } else {
        if (recursive || mutual) {
            String::from("finite record loops cannot use a recursive or mutually recursive helper graph")
        } else {
            if unknown {
                String::from("finite record loops need a complete static helper call graph without dynamic application or external calls")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_record_loop_counter_error(kind: String, bits: f64, signed: bool, wrappers: f64, owner: String, operation: String) -> String {
    if ((((((kind == "fixed") && (bits == (8f64))) && !signed) && (wrappers == (0f64))) && (owner == "u8")) && ((operation == "ne") || (operation == "gt"))) {
        String::from("")
    } else {
        String::from("a finite record loop needs its actual direct u8 counter and pure nonzero head")
    }
}

pub fn plan_rust_owned_record_loop_iterations(bits: f64) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        if (bits != (8f64)) {
            result
        } else {
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1(result, index);
                    result_2
                }
            }
        }
    }
}

pub fn test_rust_owned_record_loop_continuing_exit(kind: String) -> bool {
    ((kind == "value") || (kind == "continue"))
}

pub fn accept_rust_owned_record_loop_exit(kind: String) -> bool {
    (((((kind == "value") || (kind == "continue")) || (kind == "break")) || (kind == "return")) || (kind == "abort"))
}

pub fn read_rust_owned_record_loop_progress_error(exits: Vec<String>, counts: Vec<f64>) -> String {
    {
        let budget_error = String::from("the complete record loop exits exceed their finite source budget");
        if (((((exits.len() as f64)) != ((counts.len() as f64))) || (((exits.len() as f64)) == (0f64))) || (((exits.len() as f64)) > (256f64))) {
            budget_error.clone()
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source2_s4 = self::source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2(exits, counts, budget_error.clone(), index);
                    match source_ml_source2_s4.clone() {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done => {
                            String::from("")
                        }
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_record_loop_count(value: f64) -> bool {
    ((((value >= (0f64)) && (value <= (9007199254740991f64))) && (((value % 1f64)) == (0f64))) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

pub fn read_rust_owned_record_loop_write_error(overlaps: bool, exact_decrement: bool) -> String {
    if (overlaps && !exact_decrement) {
        String::from("a record loop cannot replace or alias-write its progress counter")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_record_loop_invariant_error(before: String, after: String) -> String {
    if (before == after) {
        String::from("")
    } else {
        String::from("each continuing record loop path must preserve every actual outer ownership and loan state")
    }
}

pub fn read_rust_owned_record_loop_signal(kind: String) -> String {
    if (kind == "break") {
        String::from("LoopBreak")
    } else {
        if (kind == "continue") {
            String::from("LoopContinue")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_record_loop_leaf_kind(kind: String) -> String {
    if (kind == "break") {
        String::from("broken")
    } else {
        if (kind == "continue") {
            String::from("continued")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_record_loop_exit_disposition(constructor: String) -> String {
    if (constructor == "FunctionResult") {
        String::from("return")
    } else {
        if (constructor == "LoopBreak") {
            String::from("break")
        } else {
            if ((constructor == "LoopContinue") || (constructor == "StatementContinuation")) {
                String::from("continue")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_record_loop_expansion_error(used: f64, nodes: f64, text: f64, copies: f64) -> String {
    if (((((((!(self::accept_rust_owned_record_loop_count(used)) || !(self::accept_rust_owned_record_loop_count(nodes))) || !(self::accept_rust_owned_record_loop_count(text))) || !(self::accept_rust_owned_record_loop_count(copies))) || (copies > (255f64))) || (nodes > (100000f64))) || (used > ((100000f64 - (nodes * copies))))) || (text > ((16777216f64 / (copies + 1f64))))) {
        String::from("the complete finite record loop expansion exceeds its source budget")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_record_loop_expansion_reservation(nodes: f64, text: f64) -> Vec<f64> {
    if (((!(self::accept_rust_owned_record_loop_count(nodes)) || !(self::accept_rust_owned_record_loop_count(text))) || (nodes > (100000f64))) || (text > (16777216f64))) {
        Vec::<f64>::new()
    } else {
        vec![((nodes * 2f64) + 64f64), (text * 2f64)]
    }
}

pub fn read_rust_owned_record_nested_loop_error(admitted: bool, returns: f64, overlap: bool) -> String {
    if (!admitted || !(self::accept_rust_owned_record_loop_count(returns))) {
        String::from("a nested loop needs its actual complete finite source admission")
    } else {
        if (returns != (0f64)) {
            String::from("a nested function return needs its complete enclosing loop exit continuation")
        } else {
            if overlap {
                String::from("a nested loop cannot write its enclosing progress counter")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_owned_record_nested_loop_depth_error(depth: f64) -> String {
    if (!(self::accept_rust_owned_record_loop_count(depth)) || (depth > (8f64))) {
        String::from("the actual nested finite loops exceed their source depth budget")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_record_nested_completion_error(admitted: bool, overlap: bool) -> String {
    if !admitted {
        String::from("a nested loop needs its actual complete finite source admission")
    } else {
        if overlap {
            String::from("a nested loop cannot write its enclosing progress counter")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_owned_record_loop_footprint_error(nodes: f64, text: f64) -> String {
    if (((!(self::accept_rust_owned_record_loop_count(nodes)) || !(self::accept_rust_owned_record_loop_count(text))) || (nodes > (100000f64))) || (text > (16777216f64))) {
        String::from("the complete loop write footprint exceeds its source budget")
    } else {
        String::from("")
    }
}

pub fn read_rust_owned_record_loop_residual_count_error(count: f64) -> String {
    if ((self::accept_rust_owned_record_loop_count(count) && (count >= (1f64))) && (count <= (256f64))) {
        String::from("")
    } else {
        String::from("a propagated loop exit needs every actual produced residual branch within its source budget")
    }
}

pub fn source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1(mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (255f64)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![index], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done,
    MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(String),
}

pub fn source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2(mut exits: Vec<String>, mut counts: Vec<f64>, mut budget_error: String, mut index: f64) -> self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result {
    loop {
        return if (index < ((exits.len() as f64))) {
            {
                let exit = self::ml_array::optional_at(&exits, self::ml_array::number_index(index));
                {
                    let count = self::ml_array::optional_at(&counts, self::ml_array::number_index(index));
                    if {
                        let source_ml_source2_comparison_operand5 = exit.clone();
                        {
                            let source_ml_source2_comparison_operand6 = None::<()>;
                            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                    } else {
                        if {
                            let source_ml_source2_comparison_operand7 = count.clone();
                            {
                                let source_ml_source2_comparison_operand8 = None::<()>;
                                match (source_ml_source2_comparison_operand7.clone(), source_ml_source2_comparison_operand8.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                            }
                        } {
                            self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                        } else {
                            if ((!(self::accept_rust_owned_record_loop_exit(exit.clone().expect("a proven present value"))) || !(self::accept_rust_owned_record_loop_count(count.clone().expect("a proven present value")))) || ((count.clone().expect("a proven present value")) > (256f64))) {
                                self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(budget_error.clone())
                            } else {
                                if (self::test_rust_owned_record_loop_continuing_exit(exit.clone().expect("a proven present value")) && ((count.clone().expect("a proven present value")) != (1f64))) {
                                    self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(String::from("every continuing record loop path must decrement its actual counter exactly once"))
                                } else {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            index = index_2;
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
            self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_owned_record_loop_count(0f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_owned_record_loop_count(255f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_owned_record_loop_count((-1f64))) == (false)), "assertion 3");
    assert!(((self::accept_rust_owned_record_loop_count(0.5f64)) == (false)), "assertion 4");
    assert!(((self::accept_rust_owned_record_loop_count(9007199254740992f64)) == (false)), "assertion 5");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("value"), String::from("break")], vec![1f64, 0f64])) == ""), "assertion 6");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("continue")], vec![0f64])) == "every continuing record loop path must decrement its actual counter exactly once"), "assertion 7");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("break")], vec![(-1f64)])) == "the complete record loop exits exceed their finite source budget"), "assertion 8");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, 100f64, 20f64, 255f64)) == ""), "assertion 9");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, 100001f64, 0f64, 0f64)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 10");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, 1f64, 0f64, 256f64)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 11");
    assert!(((self::read_rust_owned_record_loop_chunk_name(0f64, 1f64, 0f64)) == "ml_finite_record_loop_chunk_0_1"), "assertion 12");
    assert!(((self::read_rust_owned_record_loop_chunk_name((-1f64), 0f64, 0f64)) == ""), "assertion 13");
    assert!(((self::read_rust_owned_record_loop_expansion_reservation(100f64, 20f64)) == (vec![264f64, 40f64])), "assertion 14");
    assert!(((self::read_rust_owned_record_loop_expansion_reservation((-1f64), 20f64)) == (Vec::<f64>::new())), "assertion 15");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("unclassified")], vec![0f64])) == "the complete record loop exits exceed their finite source budget"), "assertion 16");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("return"), String::from("break"), String::from("continue")], vec![0f64, 0f64, 1f64])) == ""), "assertion 17");
    assert!(((self::accept_rust_owned_record_loop_count(f64::NAN)) == (false)), "assertion 18");
    assert!(((self::accept_rust_owned_record_loop_count(f64::INFINITY)) == (false)), "assertion 19");
    assert!(((self::accept_rust_owned_record_loop_count(f64::NEG_INFINITY)) == (false)), "assertion 20");
    assert!(((self::accept_rust_owned_record_loop_count((-0f64))) == (false)), "assertion 21");
    assert!(((self::read_rust_owned_record_loop_progress_error(vec![String::from("break")], vec![f64::NAN])) == "the complete record loop exits exceed their finite source budget"), "assertion 22");
    assert!(((self::read_rust_owned_record_loop_expansion_error(f64::NAN, 0f64, 0f64, 0f64)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 23");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, f64::INFINITY, 0f64, 0f64)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 24");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, 0f64, (-0f64), 0f64)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 25");
    assert!(((self::read_rust_owned_record_loop_expansion_error(0f64, 0f64, 0f64, f64::NEG_INFINITY)) == "the complete finite record loop expansion exceeds its source budget"), "assertion 26");
    assert!(((self::read_rust_owned_record_loop_chunk_name(f64::NAN, 0f64, 0f64)) == ""), "assertion 27");
    assert!(((self::read_rust_owned_record_loop_chunk_name(0f64, f64::INFINITY, 0f64)) == ""), "assertion 28");
    assert!(((self::read_rust_owned_record_loop_chunk_name(0f64, 0f64, (-0f64))) == ""), "assertion 29");
    assert!(((self::read_rust_owned_record_loop_expansion_reservation(f64::NAN, 0f64)) == (Vec::<f64>::new())), "assertion 30");
    assert!(((self::read_rust_owned_record_loop_expansion_reservation(0f64, (-0f64))) == (Vec::<f64>::new())), "assertion 31");
    assert!(((self::read_rust_owned_record_nested_loop_error(true, 0f64, false)) == ""), "assertion 32");
    assert!(((self::read_rust_owned_record_nested_loop_error(false, 0f64, false)) == "a nested loop needs its actual complete finite source admission"), "assertion 33");
    assert!(((self::read_rust_owned_record_nested_loop_error(true, (-1f64), false)) == "a nested loop needs its actual complete finite source admission"), "assertion 34");
    assert!(((self::read_rust_owned_record_nested_loop_error(true, 1f64, false)) == "a nested function return needs its complete enclosing loop exit continuation"), "assertion 35");
    assert!(((self::read_rust_owned_record_nested_loop_error(true, 0f64, true)) == "a nested loop cannot write its enclosing progress counter"), "assertion 36");
    assert!(((self::read_rust_owned_record_nested_loop_depth_error(8f64)) == ""), "assertion 37");
    assert!(((self::read_rust_owned_record_nested_loop_depth_error(9f64)) == "the actual nested finite loops exceed their source depth budget"), "assertion 38");
    assert!(((self::read_rust_owned_record_nested_loop_depth_error((-1f64))) == "the actual nested finite loops exceed their source depth budget"), "assertion 39");
    assert!(((self::read_rust_owned_record_nested_completion_error(true, false)) == ""), "assertion 40");
    assert!(((self::read_rust_owned_record_nested_completion_error(false, false)) == "a nested loop needs its actual complete finite source admission"), "assertion 41");
    assert!(((self::read_rust_owned_record_nested_completion_error(true, true)) == "a nested loop cannot write its enclosing progress counter"), "assertion 42");
    assert!(((self::read_rust_owned_record_loop_footprint_error(100000f64, 16777216f64)) == ""), "assertion 43");
    assert!(((self::read_rust_owned_record_loop_footprint_error(100001f64, 0f64)) == "the complete loop write footprint exceeds its source budget"), "assertion 44");
    assert!(((self::read_rust_owned_record_loop_footprint_error(0f64, 16777217f64)) == "the complete loop write footprint exceeds its source budget"), "assertion 45");
    assert!(((self::read_rust_owned_record_loop_residual_count_error(1f64)) == ""), "assertion 46");
    assert!(((self::read_rust_owned_record_loop_residual_count_error(256f64)) == ""), "assertion 47");
    assert!(((self::read_rust_owned_record_loop_residual_count_error(0f64)) == "a propagated loop exit needs every actual produced residual branch within its source budget"), "assertion 48");
    assert!(((self::read_rust_owned_record_loop_residual_count_error(257f64)) == "a propagated loop exit needs every actual produced residual branch within its source budget"), "assertion 49");
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
fn all_literal_owned_record_loop_controls() { main(); }
