// Generated from complete js/src/translation/reference-array-projection-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_reference_array_projection_index(kind: &str, type_: &str, children: &[bool]) -> bool {
    if ((type_ != "float") && (type_ != "int")) && (type_ != "nat") {
        false
    } else if (kind == "var") || (kind == "lit") {
        (children.len() as f64) == 0f64
    } else {
        {
            let ml_s1 = if (kind == "unary") || (kind == "cast") {
                {
                    let ml_library_inequality_2 = (children.len() as f64) != 1f64;
                    if ml_library_inequality_2 {
                        self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Return(false)
                    } else {
                        self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Next
                    }
                }
            } else if kind == "binary" {
                {
                    let ml_library_inequality_1 = (children.len() as f64) != 2f64;
                    if ml_library_inequality_1 {
                        self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Return(false)
                    } else {
                        self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Next
                    }
                }
            } else if kind == "math" {
                if (children.len() as f64) == 0f64 {
                    self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Return(false)
                } else {
                    self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Next
                }
            } else {
                self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Return(false)
            };
            match ml_s1 {
                    self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Next => {
                        {
                            let index = 0f64;
                            {
                                let ml_s2 = self::ml_accept_reference_array_projection_index_loop2(children, index);
                                match ml_s2 {
                                    self::MlAcceptReferenceArrayProjectionIndexLoop2Result::MlAcceptReferenceArrayProjectionIndexLoop2Done => {
                                        true
                                    }
                                    self::MlAcceptReferenceArrayProjectionIndexLoop2Result::MlAcceptReferenceArrayProjectionIndexLoop2Return(ml_result2) => {
                                        ml_result2
                                    }
                                }
                            }
                        }
                    }
                    self::MlAcceptReferenceArrayProjectionIndexJoin1::MlAcceptReferenceArrayProjectionIndexJoin1Return(ml_result2_2) => {
                        ml_result2_2
                    }
                }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_array_projection(
    operation: &str,
    operand: &str,
    kind: &str,
    reference: bool,
    result: &str,
    index: bool,
) -> String {
    if ((operand != "referenceRead") || (kind != "array")) || !reference {
        String::new()
    } else if operation == "length" {
        if ((result == "float") || (result == "int")) || (result == "nat") {
            operation.to_owned()
        } else {
            String::new()
        }
    } else if index && ((operation == "index") || (operation == "optionalIndex")) {
        operation.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_reference_array_source_operands(
    language: &str,
    operand: &str,
    kind: &str,
    reference: bool,
) -> bool {
    (((language == "JavaScript") && (operand == "referenceRead")) && (kind == "array")) && reference
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_reference_array_projection_suffix(initial: f64, operands: &[String]) -> f64 {
    {
        let suffix = initial;
        {
            let occupied = true;
            { self::ml_read_reference_array_projection_suffix_loop3(operands, suffix, occupied) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_reference_array_projection(
    operation: &str,
    receiver: &str,
    position: &str,
    index: &str,
    result: &str,
    suffix: &str,
) -> String {
    {
        let owner = format!("ml_reference_array{}", suffix.to_owned());
        {
            let key = format!("ml_reference_position{}", suffix.to_owned());
            {
                let value = format!("ml_reference_projection{}", suffix.to_owned());
                {
                    let opening = format!("{{ let {owner} = {}; ", receiver.to_owned());
                    if operation == "length" {
                        {
                            let length = if result == "float" {
                                format!("{owner}.borrow().len() as f64")
                            } else {
                                format!("crate::ml::Big::from_u128({owner}.borrow().len() as u128)")
                            };
                            format!("{opening}let {value} = {length}; {value} }}")
                        }
                    } else if (operation != "index") && (operation != "optionalIndex") {
                        String::new()
                    } else {
                        {
                            let converted = if index == "float" {
                                format!("crate::ml_array::number_index({key})")
                            } else {
                                format!("{key}.to_index()")
                            };
                            {
                                let method = if operation == "index" {
                                    String::from("at")
                                } else {
                                    String::from("optional_at")
                                };
                                format!(
                                    "{opening}let {key} = {}; let {value} = crate::ml_array::{method}(&*{owner}.borrow(), {converted}); {value} }}",
                                    position.to_owned()
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptReferenceArrayProjectionIndexLoop2Result {
    MlAcceptReferenceArrayProjectionIndexLoop2Done,
    MlAcceptReferenceArrayProjectionIndexLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_reference_array_projection_index_loop2(
    children: &[bool],
    mut index: f64,
) -> self::MlAcceptReferenceArrayProjectionIndexLoop2Result {
    loop {
        return if index < (children.len() as f64) {
            if (match (
                self::ml_array::optional_at(children, self::ml_array::number_index(index)),
                true,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlAcceptReferenceArrayProjectionIndexLoop2Result::MlAcceptReferenceArrayProjectionIndexLoop2Return(false)
            }
        } else {
            self::MlAcceptReferenceArrayProjectionIndexLoop2Result::MlAcceptReferenceArrayProjectionIndexLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptReferenceArrayProjectionIndexJoin1 {
    MlAcceptReferenceArrayProjectionIndexJoin1Next,
    MlAcceptReferenceArrayProjectionIndexJoin1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_reference_array_projection_suffix_loop4(
    operands: &[String],
    mut suffix: f64,
    mut occupied: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (operands.len() as f64) {
            {
                let text =
                    (self::ml_array::optional_at(operands, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (text.contains(
                    (format!("ml_reference_array{}", self::ml_number::js_number(suffix))).as_str(),
                ) || text.contains(
                    (format!(
                        "ml_reference_position{}",
                        self::ml_number::js_number(suffix)
                    ))
                    .as_str(),
                )) || text.contains(
                    (format!(
                        "ml_reference_projection{}",
                        self::ml_number::js_number(suffix)
                    ))
                    .as_str(),
                ) {
                    {
                        let occupied_2 = true;
                        {
                            let index_2 = index + 1f64;
                            {
                                (occupied, index) = (occupied_2, index_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { occupied };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_reference_array_projection_suffix_loop3(
    operands: &[String],
    mut suffix: f64,
    mut occupied: bool,
) -> f64 {
    loop {
        if occupied {
            {
                let occupied_2 = false;
                {
                    let index = 0f64;
                    {
                        let occupied_3 = self::ml_read_reference_array_projection_suffix_loop4(
                            operands, suffix, occupied_2, index,
                        );
                        if occupied_3 {
                            {
                                let suffix_2 = suffix + 1f64;
                                {
                                    (suffix, occupied) = (suffix_2, occupied_3);
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                occupied = occupied_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { suffix };
    }
}
