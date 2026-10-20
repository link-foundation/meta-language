// Generated from complete authenticated source declarations; do not edit.
use meta_language::{ParseOutcome, StructuredDataLinksLimits};
use serde_json::Number;
fn fail(message: &str) -> String {
    format!("structured data: {message}")
}

fn write_number(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err(fail("a number must be finite binary64"));
    }
    if value.to_bits().trailing_zeros() >= 63 {
        return Ok(if value.is_sign_negative() { "-0" } else { "0" }.to_owned());
    }
    let shortest = value.abs().to_string();
    let (coefficient, exponent) = shortest.split_once(['e', 'E']).map_or(
        Ok((shortest.as_str(), 0_i32)),
        |(coefficient, exponent)| {
            exponent
                .parse::<i32>()
                .map(|exponent| (coefficient, exponent))
                .map_err(|_| fail("a number exponent is invalid"))
        },
    )?;
    let point = coefficient.find('.').unwrap_or(coefficient.len());
    let mut digits: String = coefficient
        .chars()
        .filter(|character| *character != '.')
        .collect();
    let leading = digits
        .chars()
        .take_while(|character| *character == '0')
        .count();
    digits.drain(..leading);
    while digits.ends_with('0') {
        digits.pop();
    }
    let magnitude = i32::try_from(point).map_err(|_| fail("a number is too long"))?
        - i32::try_from(leading).map_err(|_| fail("a number is too long"))?
        + exponent
        - 1;
    let mut output = if (-6..21).contains(&magnitude) {
        let position = magnitude + 1;
        if position <= 0 {
            format!(
                "0.{}{digits}",
                "0".repeat(usize::try_from(-position).unwrap())
            )
        } else {
            let position = usize::try_from(position).unwrap();
            if position >= digits.len() {
                format!("{digits}{}", "0".repeat(position - digits.len()))
            } else {
                format!("{}.{}", &digits[..position], &digits[position..])
            }
        }
    } else {
        let coefficient = if digits.len() == 1 {
            digits
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        format!(
            "{coefficient}e{}{magnitude}",
            if magnitude >= 0 { "+" } else { "" }
        )
    };
    if value.is_sign_negative() {
        output.insert(0, '-');
    }
    Ok(output)
}
fn exact_json_number(number: &Number) -> Result<f64, String> {
    let value = number
        .as_f64()
        .ok_or_else(|| fail("a JSON number is not binary64"))?;
    if let Some(integer) = number.as_i64() {
        // i128 has sufficient range for all JSON integer inputs, so
        // this comparison detects binary64 rounding without saturation.
        #[allow(clippy::cast_possible_truncation)]
        let represented = value as i128;
        if represented != i128::from(integer) {
            return Err(fail(
                "a JSON integer is not exactly representable as binary64",
            ));
        }
    } else if let Some(integer) = number.as_u64() {
        #[allow(clippy::cast_possible_truncation)]
        let represented = value as i128;
        if represented != i128::from(integer) {
            return Err(fail(
                "a JSON integer is not exactly representable as binary64",
            ));
        }
    }
    Ok(value)
}

fn next_depth(depth: usize, increment: usize) -> Result<usize, String> {
    depth
        .checked_add(increment)
        .ok_or_else(|| fail("the outcome exceeds its structural resource limit"))
}

struct CountArray {
    depth: usize,
    expected: usize,
    seen: usize,
}

struct CountSink {
    limits: StructuredDataLinksLimits,
    occurrences: usize,
    bytes: usize,
}

impl CountSink {
    fn occurrence(&mut self, depth: usize, bytes: usize) -> Result<(), String> {
        let occurrences = self
            .occurrences
            .checked_add(1)
            .ok_or_else(|| fail("the outcome exceeds its structural resource limit"))?;
        let total_bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or_else(|| fail("the outcome exceeds its scalar byte resource limit"))?;
        if occurrences > self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(fail("the outcome exceeds its structural resource limit"));
        }
        if total_bytes > self.limits.byte_limit {
            return Err(fail("the outcome exceeds its scalar byte resource limit"));
        }
        self.occurrences = occurrences;
        self.bytes = total_bytes;
        Ok(())
    }
}

impl OutcomeWireSink for CountSink {
    type Node = ();
    type Object = usize;
    type Array = CountArray;

    fn begin_object(&mut self, depth: usize) -> Result<usize, String> {
        self.occurrence(depth, 0)?;
        Ok(depth)
    }

    fn object_field<F>(&mut self, object: &mut usize, key: &str, visit: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<(), String>,
    {
        let child_depth = next_depth(*object, 2)?;
        self.occurrence(next_depth(*object, 1)?, 0)?;
        self.string(key, child_depth)?;
        visit(self, child_depth)
    }

    fn end_object(&mut self, _object: usize) -> Result<(), String> {
        Ok(())
    }

    fn begin_array(&mut self, count: usize, depth: usize) -> Result<CountArray, String> {
        self.occurrence(depth, 0)?;
        Ok(CountArray {
            depth,
            expected: count,
            seen: 0,
        })
    }

    fn array_item<F>(&mut self, array: &mut CountArray, visit: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<(), String>,
    {
        if array.seen >= array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        visit(self, next_depth(array.depth, 1)?)?;
        array.seen += 1;
        Ok(())
    }

    fn end_array(&mut self, array: CountArray) -> Result<(), String> {
        if array.seen != array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        Ok(())
    }

    fn null(&mut self, depth: usize) -> Result<(), String> {
        self.occurrence(depth, 0)
    }

    fn boolean(&mut self, value: bool, depth: usize) -> Result<(), String> {
        self.occurrence(depth, if value { 4 } else { 5 })
    }

    fn string(&mut self, value: &str, depth: usize) -> Result<(), String> {
        self.occurrence(depth, value.len())
    }

    fn usize(&mut self, value: usize, depth: usize) -> Result<(), String> {
        let integer = u64::try_from(value).map_err(|_| fail("a JSON integer is not binary64"))?;
        self.json_number(&serde_json::Number::from(integer), depth)
    }

    fn i64(&mut self, value: i64, depth: usize) -> Result<(), String> {
        self.json_number(&serde_json::Number::from(value), depth)
    }

    fn json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<(), String> {
        // Existing formatter's binary64 scratch is bounded constant work, not
        // an input-sized output tree. No replacement numerical algorithm.
        let spelling = write_number(exact_json_number(value)?)?;
        self.occurrence(depth, spelling.len())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeJsonNumberObservation {
    pub wire_occurrence: usize,
    pub signed: bool,
    pub unsigned: bool,
    pub floating: bool,
    pub signed_decimal: Option<String>,
    pub unsigned_decimal: Option<String>,
    pub floating_bits: Option<String>,
}

struct ObservationSink {
    limits: StructuredDataLinksLimits,
    occurrences: usize,
    bytes: usize,
    observations: Vec<NativeJsonNumberObservation>,
}

impl ObservationSink {
    fn occurrence(&mut self, depth: usize, bytes: usize) -> Result<(), String> {
        let occurrences = self
            .occurrences
            .checked_add(1)
            .ok_or_else(|| fail("the outcome exceeds its structural resource limit"))?;
        let total_bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or_else(|| fail("the outcome exceeds its scalar byte resource limit"))?;
        if occurrences > self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(fail("the outcome exceeds its structural resource limit"));
        }
        if total_bytes > self.limits.byte_limit {
            return Err(fail("the outcome exceeds its scalar byte resource limit"));
        }
        self.occurrences = occurrences;
        self.bytes = total_bytes;
        Ok(())
    }
}

impl OutcomeWireSink for ObservationSink {
    type Node = ();
    type Object = usize;
    type Array = CountArray;

    fn begin_object(&mut self, depth: usize) -> Result<usize, String> {
        self.occurrence(depth, 0)?;
        Ok(depth)
    }

    fn object_field<F>(&mut self, object: &mut usize, key: &str, visit: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<(), String>,
    {
        let child_depth = next_depth(*object, 2)?;
        self.occurrence(next_depth(*object, 1)?, 0)?;
        self.string(key, child_depth)?;
        visit(self, child_depth)
    }

    fn end_object(&mut self, _object: usize) -> Result<(), String> {
        Ok(())
    }

    fn begin_array(&mut self, count: usize, depth: usize) -> Result<CountArray, String> {
        self.occurrence(depth, 0)?;
        Ok(CountArray {
            depth,
            expected: count,
            seen: 0,
        })
    }

    fn array_item<F>(&mut self, array: &mut CountArray, visit: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self, usize) -> Result<(), String>,
    {
        if array.seen >= array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        visit(self, next_depth(array.depth, 1)?)?;
        array.seen += 1;
        Ok(())
    }

    fn end_array(&mut self, array: CountArray) -> Result<(), String> {
        if array.seen != array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        Ok(())
    }

    fn null(&mut self, depth: usize) -> Result<(), String> {
        self.occurrence(depth, 0)
    }

    fn boolean(&mut self, value: bool, depth: usize) -> Result<(), String> {
        self.occurrence(depth, if value { 4 } else { 5 })
    }

    fn string(&mut self, value: &str, depth: usize) -> Result<(), String> {
        self.occurrence(depth, value.len())
    }

    fn usize(&mut self, value: usize, depth: usize) -> Result<(), String> {
        let integer = u64::try_from(value).map_err(|_| fail("a JSON integer is not binary64"))?;
        self.json_number(&serde_json::Number::from(integer), depth)
    }

    fn i64(&mut self, value: i64, depth: usize) -> Result<(), String> {
        self.json_number(&serde_json::Number::from(value), depth)
    }

    fn json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<(), String> {
        // Existing formatter's binary64 scratch is bounded constant work, not
        // an input-sized output tree. No replacement numerical algorithm.
        let spelling = write_number(exact_json_number(value)?)?;
        self.occurrence(depth, spelling.len())
    }

    fn native_json_number(
        &mut self,
        value: &serde_json::Number,
        depth: usize,
    ) -> Result<(), String> {
        // These primitive facts belong to this original borrowed Number before
        // the retained projection validator can erase its representation.
        let signed = value.is_i64();
        let unsigned = value.is_u64();
        let floating = value.is_f64();
        let signed_decimal = value.as_i64().map(|integer| integer.to_string());
        let unsigned_decimal = value.as_u64().map(|integer| integer.to_string());
        let floating_bits = value
            .as_f64()
            .map(|number| format!("{:016x}", number.to_bits()));
        self.json_number(value, depth)?;
        self.observations
            .try_reserve(1)
            .map_err(|_| fail("unable to reserve bounded original number observations"))?;
        self.observations.push(NativeJsonNumberObservation {
            wire_occurrence: self.occurrences,
            signed,
            unsigned,
            floating,
            signed_decimal,
            unsigned_decimal,
            floating_bits,
        });
        Ok(())
    }
}

pub fn capture_original_json_number_observations(
    outcome: &ParseOutcome,
    limits: StructuredDataLinksLimits,
) -> Result<Vec<NativeJsonNumberObservation>, String> {
    // This new finite observation profile does not change any existing API.
    if limits.node_limit > 100_000 || limits.byte_limit > 16 * 1024 * 1024 {
        return Err(fail(
            "original number observation profile exceeds its explicit resource bounds",
        ));
    }

    if limits.node_limit == 0 || limits.depth_limit == 0 || limits.byte_limit == 0 {
        return Err(fail("resource limits must be positive"));
    }
    if limits.depth_limit > 256 {
        return Err(fail(
            "the borrowed outcome profile requires a depth limit at most 256",
        ));
    }
    let mut count = CountSink {
        limits,
        occurrences: 0,
        bytes: 0,
    };
    wire_parse_outcome(outcome, &mut count, 1)?;
    // Count the schema root LAST as in the ordered plan. The whole counting
    // pass still finishes before any input-sized output allocation.
    count.occurrence(0, 1)?;
    // The complete allocation-free original counting pass finishes first.
    let mut observed = ObservationSink {
        limits,
        occurrences: 0,
        bytes: 0,
        observations: Vec::new(),
    };
    wire_parse_outcome(outcome, &mut observed, 1)?;
    observed.occurrence(0, 1)?;
    Ok(observed.observations)
}

include!("native_json_number_observation_visitors.rs");
