import {rustEscape,rustString} from './rust-string-literals.js';
import { renderWrappingFixedIntegerCast, readFixedIntegerCastStatement } from './fixed-integer-cast-rules.js';
import {checkRustRuntimeShell,emitRustRuntimeShell,refuseRustRuntimeShellComposition} from './rust-runtime-shell.js';
import { renderStringLengthExpression, renderStringMapExpression, renderStringTestExpression } from './frontend-rules.js';
import {requireStringOrderUnits,renderStringOrderExpression} from './string-order-rules.js';
// Rust emitter. Naturals and integers are `ml::Big`, an unbounded integer
// the translation carries in its own prelude, so no value is ever narrowed;
// machine integers stay Rust machine integers with checked arithmetic, which
// panics exactly where the source aborts; a JavaScript Number is an `f64`,
// printed by `ml_number::js_number` exactly as JavaScript prints it. Data types are enums whose
// data-typed fields are boxed. Every value is owned: a variable is cloned
// where it is consumed. Theorems cannot be proved in Rust: each becomes an
// executable property, checked over a bounded domain by
// `--ml-check-theorems`, while the proof stays discharged by the source kernel.

import { castMessage, overflowMessage, zeroDivisorMessage } from './aborts.js';
import {readJavaScriptNumberBitwiseOperationCode,readJavaScriptNumberBitwiseArity} from './number-bitwise-rules.js';
import {NUMBER_BITWISE_PRELUDE} from './number-bitwise-runtime.js';
import { renderCollectionIteration, renderOptionalArrayRead, renderStringPointIteration } from './array-value-rules.js';
import { renderArrayFromMapping } from './array-from-rules.js';
import { renderArrayCallbackExpression, renderArrayCallbackReferenceExpression, readArrayCallbackCheckedArgumentPosition } from './array-callback-rules.js';
import { renderPrimitiveMethodArgument, renderPrimitiveMethodExpression, requirePrimitiveMethodUtf16, renderPrimitiveArraySearchExpression, renderPrimitiveStringRepresentation, readPrimitiveArrayJoinElementForm, readPrimitiveArrayJoinOperandForm, renderPrimitiveArrayJoinElement, renderPrimitiveArrayJoinExpression } from './primitive-method-rules.js';
import { readSurfaceReferenceEmissionOperation, readSurfaceReferenceEmissionRefusal, renderSurfaceReferenceOperation } from './surface-reference-analysis-rules.js';
import { renderRecordField, renderRecordValue } from './record-emission-rules.js';
import { readUnionTargetRefusal } from './type-syntax-rules.js';
import { readCallableCapturedBindings } from './callable-source.js';
import { renderOptionalAbsentValue, renderOptionalCoalescing, renderOptionalConsoleText, renderOptionalFlattening, renderOptionalMatch, renderOptionalNumericPresent, renderOptionalPresentValue, renderOptionalPrimitiveTruth, renderOptionalProjection, renderOptionalStrictEquality, renderOptionalTypeQueryPresent, renderOptionalUnwrapping, requireOptionalNumericPredicate } from './optional-value-rules.js';
import { renderCallableApplication, renderCallableCellOperation, renderCallableFunctionReference, renderCallableLambda } from './callable-source-rules.js';
import {checkRustConstFunctionProof,renderRustConstFunctionHeader} from './rust-const-functions.js';
import {checkRustOwnedLocalStateProof} from './rust-owned-local-state.js';
import { readOwnedArrayFinalUses } from './owned-array-moves.js';
import {validateRustTypeNamespace} from './rust-type-namespace.js';
import { typeError, unsupported } from './diagnostics.js';
import { renderCollectionConstruction } from './collection-construction-rules.js';
import { sameType, typeKey } from './types.js';
import {acceptReferenceIdentityComparison, readReferenceIdentityComparisonError, renderReferenceIdentityComparison} from './reference-identity-rules.js';
import { Scope, renameFunction, renameMain, renameTheorem, tailLoop } from './ir.js';
import { EmitState, typeContains, withParts } from './emit-common.js';
import { STRING_PRELUDE } from './string-runtime.js';
import { readRustOptionalPrelude, renderRustCompoundType, renderRustReferenceType } from './structured-type-rules.js';
import { acceptConstantEmission, constantBindingForm, renderConstantBinding } from './frontend-rules.js';
import { readNumberDisplayFunction, renderAbsentNumberExpression } from './primitive-value-rules.js';
import { acceptTupleCopyType, renderTupleProjection, renderTupleValue } from './product-emission-rules.js';
import { acceptProgramTupleProductModel } from '../typed-program-link-rules.js';
import { readRustSourceHeaderEmission, renderRustSourceHeaderEmission } from './rust-source-header-emission.js';
import { RUST_BORROWED_CALL_INTERFACE_POLICY, readRustBorrowedCallInterface, renderRustBorrowedCall } from './rust-borrowed-call-interface.js';
import { acceptRustBorrowedCallInterfaceModel, renderRustBorrowedCallHeaderReference } from './rust-borrowed-call-interface-rules.js';

const KEYWORDS = new Set([
  'as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
  'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct', 'super',
  'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while', 'async', 'await', 'dyn', 'abstract', 'become', 'box',
  'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield', 'try', 'gen', 'union', 'raw',
  // Names the emitted code relies on.
  'ml', 'main', 'std', 'core', 'alloc', 'String', 'Vec', 'Box', 'Option', 'Some', 'None', 'Ok', 'Err', 'Result',
  'bool', 'str', 'char', 'u8', 'u16', 'u32', 'u64', 'u128', 'usize', 'i8', 'i16', 'i32', 'i64', 'i128', 'isize',
  'f32', 'f64', 'Big', 'Clone', 'Debug', 'PartialEq', 'Eq', 'ToString', 'ml_main', 'ml_check_theorems', 'ml_number',
]);

const PRELUDE = `/// Unbounded integers for the portable core's naturals and integers.
pub mod ml {
    use std::cmp::Ordering;
    use std::fmt;

    /// Sign and magnitude; the magnitude is little-endian base 2^32 without
    /// leading zero limbs, and zero is never negative.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Big {
        negative: bool,
        magnitude: Vec<u32>,
    }

    fn trim(mut magnitude: Vec<u32>) -> Vec<u32> {
        while magnitude.last() == Some(&0) {
            magnitude.pop();
        }
        magnitude
    }

    fn compare_magnitude(a: &[u32], b: &[u32]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0)) + u64::from(*b.get(index).unwrap_or(&0)) + carry;
            out.push(sum as u32);
            carry = sum >> 32;
        }
        if carry > 0 {
            out.push(carry as u32);
        }
        out
    }

    /// |a| - |b| where |a| >= |b|.
    fn sub_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len());
        let mut borrow = 0i64;
        for (index, limb) in a.iter().enumerate() {
            let mut difference = i64::from(*limb) - i64::from(*b.get(index).unwrap_or(&0)) - borrow;
            borrow = 0;
            if difference < 0 {
                difference += 1 << 32;
                borrow = 1;
            }
            out.push(difference as u32);
        }
        trim(out)
    }

    fn mul_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; a.len() + b.len()];
        for (i, x) in a.iter().enumerate() {
            let mut carry = 0u64;
            for (j, y) in b.iter().enumerate() {
                let product = u64::from(out[i + j]) + u64::from(*x) * u64::from(*y) + carry;
                out[i + j] = product as u32;
                carry = product >> 32;
            }
            let mut index = i + b.len();
            while carry > 0 {
                let sum = u64::from(out[index]) + carry;
                out[index] = sum as u32;
                carry = sum >> 32;
                index += 1;
            }
        }
        trim(out)
    }

    /// Quotient and remainder of magnitudes; the divisor is not zero.
    fn divmod_magnitude(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut quotient = vec![0u32; a.len()];
        if b.len() == 1 {
            let divisor = u64::from(b[0]);
            let mut remainder = 0u64;
            for index in (0..a.len()).rev() {
                let current = (remainder << 32) | u64::from(a[index]);
                quotient[index] = (current / divisor) as u32;
                remainder = current % divisor;
            }
            return (trim(quotient), trim(vec![remainder as u32]));
        }
        let mut remainder: Vec<u32> = Vec::new();
        for bit in (0..a.len() * 32).rev() {
            let mut carry = (a[bit / 32] >> (bit % 32)) & 1;
            for limb in remainder.iter_mut() {
                let next = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            if carry != 0 {
                remainder.push(carry);
            }
            if compare_magnitude(&remainder, b) != Ordering::Less {
                remainder = sub_magnitude(&remainder, b);
                quotient[bit / 32] |= 1 << (bit % 32);
            }
        }
        (trim(quotient), remainder)
    }

    impl Big {
        fn from_parts(negative: bool, magnitude: Vec<u32>) -> Self {
            let magnitude = trim(magnitude);
            Self { negative: negative && !magnitude.is_empty(), magnitude }
        }

        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text),
            };
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        pub fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        pub fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
        pub fn divmod_trunc(&self, other: &Self) -> (Self, Self) {
            let (quotient, remainder) = divmod_magnitude(&self.magnitude, &other.magnitude);
            (
                Self::from_parts(self.negative != other.negative, quotient),
                Self::from_parts(self.negative, remainder),
            )
        }
    }

    impl Ord for Big {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.negative, other.negative) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => compare_magnitude(&self.magnitude, &other.magnitude),
                (true, true) => compare_magnitude(&other.magnitude, &self.magnitude),
            }
        }
    }

    impl PartialOrd for Big {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl fmt::Display for Big {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_zero() {
                return formatter.write_str("0");
            }
            let billion = [1_000_000_000u32];
            let mut chunks = Vec::new();
            let mut rest = self.magnitude.clone();
            while !rest.is_empty() {
                let (quotient, remainder) = divmod_magnitude(&rest, &billion);
                chunks.push(remainder.first().copied().unwrap_or(0));
                rest = quotient;
            }
            let mut text = String::new();
            if self.negative {
                text.push('-');
            }
            text.push_str(&chunks.pop().unwrap_or(0).to_string());
            for chunk in chunks.iter().rev() {
                text.push_str(&format!("{chunk:09}"));
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
    pub fn divide(a: &Big, b: &Big, rounding: &str, by_zero: Option<&str>, remainder: bool) -> Big {
        if b.is_zero() {
            if let Some(message) = by_zero {
                panic!("{message}");
            }
            return if remainder { a.clone() } else { Big::zero() };
        }
        let one = Big::from_i128(1);
        let (mut quotient, rest) = a.divmod_trunc(b);
        if !rest.is_zero() {
            if rounding == "floor" && rest.is_negative() != b.is_negative() {
                quotient = quotient.sub(&one);
            }
            if rounding == "euclid" && rest.is_negative() {
                quotient = if b.is_negative() { quotient.add(&one) } else { quotient.sub(&one) };
            }
        }
        if remainder { a.sub(&quotient.mul(b)) } else { quotient }
    }

    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        if value.is_negative() {
            panic!("{message}");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}`;

const NUMBER_PRELUDE = `/// JavaScript's Number operations that Rust's f64 does not share.
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
}`;

const MATH_PRELUDE = `/// The Math functions that Rust's f64 methods do not share with JavaScript.
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
}`;

const LEGACY_ARRAY_PRELUDE = `/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
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
}`;

const ARRAY_PRELUDE = `/// Reads of JavaScript arrays, which the portable core never mutates.
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
}`;

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_]/gu, '_');
  if (/^[0-9]/u.test(result) || result.startsWith('__') || result === '_' || result === '') result = `x${result}`;
  if (KEYWORDS.has(result)) result = `${result}_`;
  return result;
}

function snake(name) {
  return ident(name
    .replace(/([a-z0-9])([A-Z])/gu, '$1_$2')
    .replace(/([A-Z]+)([A-Z][a-z])/gu, '$1_$2')
    .toLowerCase());
}

function camel(name) {
  const parts = name.split(/[^A-Za-z0-9]+|_/u).filter(Boolean);
  return ident(parts.map((part) => part[0].toUpperCase() + part.slice(1)).join('') || 'T');
}

function isCopyType(type, productModel) {
  if (type.kind === 'tuple' && productModel !== 'immutable value product') return false;
  return acceptTupleCopyType(type.kind, type.kind === 'tuple' ? type.elements.map(item => isCopyType(item, productModel)) : []);
}
const OPTIONAL_PRELUDE = readRustOptionalPrelude();

/** The exact target runtime data shared with the native Rust host adapter. */
export function readRustArrayPrelude(standard = false) {
  return standard ? ARRAY_PRELUDE : LEGACY_ARRAY_PRELUDE;
}

export function emitRust(program, options = {}) {
  validateRustTypeNamespace(program);
  checkRustConstFunctionProof(program);
  checkRustOwnedLocalStateProof(program);
  const runtimeShell=checkRustRuntimeShell(program);
  if(runtimeShell){if(options.sourceHeaderModel!==undefined)refuseRustRuntimeShellComposition('target header changes require an independent runtime composition proof');return emitRustRuntimeShell(program,runtimeShell);}
  const borrowedCalls = acceptRustBorrowedCallInterfaceModel(options.sourceHeaderModel) ? readRustBorrowedCallInterface(program, options.sourceHeaderModel) : null;
  const sourceHeaders = borrowedCalls?.headers ?? (options.sourceHeaderModel === undefined ? null : readRustSourceHeaderEmission(program, options.sourceHeaderModel));
  const state = new EmitState(program, 'Rust', snake, KEYWORDS, {
    ctorStyle: 'data',
    typeName: camel,
    valueName: snake,
    ctorName: camel,
    moduleSegment: snake,
    typeSpace: true,
    modulesShareTypeSpace: true,
  });
  // A local may not shadow another item of the module that the program calls or reads.
  for (const external of program.externals ?? []) state.claimed.add(externalName(external.k, external.name));
  return new RustEmitter(program, state, sourceHeaders, borrowedCalls?.targets, options.arrayConsumption === true, options.recordDefinitionOwnership === true).file();
}

/** The Rust name of another item of the module: a function's in snake case, a constant's as it is written. */
function externalName(kind, name) {
  return kind === 'fn' ? snake(name) : name;
}

/** Actual normal emitter target identities, shared by optional ownership checks. */
export function readRustExternalTargets(program) {
  const externals = new Map((program.externals ?? []).filter(({name}) => !program.declarations.has(name)).map(entry => [entry.name, entry]));
  return [...externals.values()].map(entry => ({source: entry.name, kind: entry.k, target: externalName(entry.k, entry.name)}));
}

/**
 * A program whose only effect binds one top-level constant, as Rust items: a
 * Number, boolean or machine integer literal is a `const` of its type, a
 * string literal a `&str` const, and any other value a `static` computed once,
 * when it is first read, as a JavaScript module computes it once. Returns null
 * for a program with declarations or other effects.
 */
export function emitRustConstants(program) {
  const effects = program.main?.effects ?? [];
  const effect = effects[0];
  const legalName = typeof effect?.name === 'string' && !effect.name.startsWith('ml_') && !KEYWORDS.has(effect.name) && /^[A-Za-z_][A-Za-z0-9_]*$/u.test(effect.name);
  if (!acceptConstantEmission(effect?.constant ?? false, effects.length, program.declarations.size, legalName ?? false)) return null;
  const state = new EmitState(program, 'Rust', snake, KEYWORDS, {
    ctorStyle: 'data', typeName: camel, valueName: snake, ctorName: camel, moduleSegment: snake, typeSpace: true, modulesShareTypeSpace: true,
  });
  for (const external of program.externals ?? []) state.claimed.add(externalName(external.k, external.name));
  const emitter = new RustEmitter(program, state);
  const [{ name, value }] = effects;
  const form = constantBindingForm(value.k, value.type.kind);
  const definition = renderConstantBinding(form, name,
    form === 'string' ? '&str' : emitter.type(value.type),
    form === 'string' ? rustString(String(value.value)) : emitter.expr(value));
  const preludes = [
    ...(emitter.usesBig ? [PRELUDE] : []),
    ...(emitter.usesNumber ? [NUMBER_PRELUDE] : []),
    ...(emitter.usesMath ? [MATH_PRELUDE] : []),
    ...(emitter.usesArray ? [readRustArrayPrelude(emitter.usesStandardArray)] : []),
    ...(emitter.usesNumberBitwise ? [NUMBER_BITWISE_PRELUDE] : []),
  ];
  return withParts({ language: 'Rust', text: [...preludes, definition].join('\n\n'), entry: null }, preludes, [definition]);
}

class RustEmitter {
  constructor(program, state, sourceHeaders = null, borrowedCallTargets = null, arrayConsumption = false, recordDefinitionOwnership = false) {
    this.program = program;
    this.state = state;
    this.definitionOwnership = recordDefinitionOwnership ? { schemaVersion: 1, blocks: [], externalTargets: [] } : null;
    this.sourceHeaders = sourceHeaders;
    this.borrowedCallTargets = borrowedCallTargets;
    // Historical low-level profiles keep copies; ordinary source and Links
    // translation explicitly select final binding consumption.
    this.arrayConsumption = arrayConsumption;
    // The other items of the module the program calls or reads, which it does not declare.
    this.externals = new Map((program.externals ?? []).filter(({ name }) => !program.declarations.has(name)).map((external) => [external.name, external]));
    if (this.definitionOwnership !== null) this.definitionOwnership.externalTargets = readRustExternalTargets(program);
    this.temporaries = 0;
    this.theoremChecks = [];
    this.usesBig = false;
    this.usesNumber = false;
    this.usesArray = false;
    this.usesStandardArray = false;
    this.usesMath = false;
    this.usesNumberBitwise = false;
    this.usesOptional = false;
    const needsUnits = (node) => {
      if (!node || typeof node !== 'object') return false;
      if (requireStringOrderUnits(node.k ?? '', node.op ?? '')) return true;
      if (requirePrimitiveMethodUtf16(node.k ?? '', node.k === 'method' ? node.object.type.kind : '')) return true;
      return Object.entries(node).some(([key, value]) => key !== 'type' && key !== 'span' && (Array.isArray(value) ? value.some(needsUnits) : needsUnits(value)));
    };
    this.utf16Strings = Boolean(program.utf16Strings) || [...program.declarations.values()].some(needsUnits) || needsUnits(program.main);
    if (this.utf16Strings) this.state.encode('utf16-strings', 'strings are sequences of UTF-16 code units, retaining isolated surrogates and JavaScript offsets; output replaces isolated surrogates as Node writes UTF-8');
  }

  file() {
    const tree = { items: [], modules: new Map() };
    for (const entry of this.program.declarations.values()) {
      let node = tree;
      for (const segment of entry.modulePath) {
        if (!node.modules.has(segment)) node.modules.set(segment, { items: [], modules: new Map() });
        node = node.modules.get(segment);
      }
      node.items.push(entry);
    }
    const body = [...(this.program.imports ?? []).map((entry) => this.use(entry)), ...this.moduleBody(tree, [])];
    const main = this.program.main ? this.main(this.program.main) : null;
    const runner = this.theoremRunner();
    const entry = this.entry(Boolean(main));
    const text = [
      `// Translated from ${this.program.sourceLanguage} by meta-language: portable core, Rust target.`,
      '#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]',
      '',
      ...(this.usesBig ? [PRELUDE, ''] : []),
      ...(this.usesNumber ? [NUMBER_PRELUDE, ''] : []),
      ...(this.usesMath ? [MATH_PRELUDE, ''] : []),
      ...(this.usesArray ? [readRustArrayPrelude(this.usesStandardArray), ''] : []),
      ...(this.utf16Strings ? [STRING_PRELUDE, ''] : []),
      ...(this.usesOptional ? [OPTIONAL_PRELUDE, ''] : []),
      ...(this.usesNumberBitwise ? [NUMBER_BITWISE_PRELUDE, ''] : []),
      ...body.flatMap((block) => [block, '']),
      ...[main, runner, entry].filter(Boolean).flatMap((block) => [block, '']),
    ].join('\n');
    const preludes = [
      ...(this.usesBig ? [PRELUDE] : []),
      ...(this.usesNumber ? [NUMBER_PRELUDE] : []),
      ...(this.usesMath ? [MATH_PRELUDE] : []),
      ...(this.usesArray ? [readRustArrayPrelude(this.usesStandardArray)] : []),
      ...(this.utf16Strings ? [STRING_PRELUDE] : []),
      ...(this.usesOptional ? [OPTIONAL_PRELUDE] : []),
      ...(this.usesNumberBitwise ? [NUMBER_BITWISE_PRELUDE] : []),
    ];
    return withParts({
      language: 'Rust',
      text,
      mappings: this.state.mappings,
      assumptions: this.state.assumptionList(),
      encodings: this.state.encodingList(),
      theorems: this.state.theorems,
      entry: main ? 'main' : null,
    }, preludes, body, this.definitionOwnership);
  }

  /**
   * An import of items of another module of the crate, `use crate::m::{a, b as c};`.
   * A function is named in snake case, as its own translation names it, and
   * any other item as it is written.
   */
  use({ module, names }) {
    const kinds = new Map((this.program.externals ?? []).map((external) => [external.name, external.k]));
    const items = names.map(({ imported, local }) => {
      const kind = kinds.get(local);
      const from = externalName(kind, imported);
      const to = externalName(kind, local);
      return from === to ? from : `${from} as ${to}`;
    });
    const path = ['crate', ...module.map(snake)].join('::');
    return items.length === 1 ? `use ${path}::${items[0]};` : `use ${path}::{${items.join(', ')}};`;
  }

  /** A call of another item of the module, or a read of one of its constants. */
  external(e, { k, literal }) {
    if (k === 'fn') return `${externalName(k, e.fn)}(${e.args.map((arg) => this.expr(arg)).join(', ')})`;
    // A literal Number, boolean or machine integer is a const, a literal string a &str const, and any other value a static.
    if (literal && isCopyType(e.type, e.productModel)) return e.fn;
    if (literal && e.type.kind === 'string') return renderPrimitiveStringRepresentation(this.utf16Strings, 'constant', e.fn, '', '');
    return isCopyType(e.type, e.productModel) ? `*${e.fn}` : `${e.fn}.clone()`;
  }

  moduleBody(node, path) {
    const blocks = node.items.map((entry, index) => {
      const mappingStart = this.state.mappings.length;
      const definition = this.declaration(entry);
      if (this.definitionOwnership !== null && path.length === 0) {
        this.definitionOwnership.blocks.push({source: entry.fullName, kind: entry.k,
          definitionIndex: (this.program.imports?.length ?? 0) + index,
          definition, sourceSpan: entry.span ? {...entry.span} : null,
          mappingStart, mappingEnd: this.state.mappings.length,
          targets: this.state.mappings.slice(mappingStart).map(mapping => mapping.target)});
      }
      return definition;
    });
    for (const [segment, child] of node.modules) {
      const inner = this.moduleBody(child, [...path, segment]);
      blocks.push(`pub mod ${this.state.moduleName([...path, segment])} {\n${indent(inner.join('\n\n'), 1)}\n}`);
    }
    return blocks;
  }

  declaration(entry) {
    if (entry.k === 'data') return this.data(entry);
    if (entry.k === 'fn') return this.fn(entry);
    return this.theorem(entry);
  }

  type(type) {
    if (type.reference) return renderRustReferenceType(this.type({ ...type, reference: false }), true);
    switch (type.kind) {
      case 'nat':
      case 'int':
        this.usesBig = true;
        this.state.encode('unbounded-integers', 'naturals and integers are ml::Big, an arbitrary-precision integer defined in the translated program; naturals stay non-negative because natural subtraction truncates and conversions to naturals are checked');
        return 'crate::ml::Big';
      case 'fixed':
        return typeKey(type);
      case 'float':
        this.floats();
        return 'f64';
      case 'bool':
        return 'bool';
      case 'string':
        return renderPrimitiveStringRepresentation(this.utf16Strings, 'type', '', '', '');
      case 'option':
        if (type.absence === 'both') {
          this.usesOptional = true;
          return renderRustCompoundType('option', 'both', [this.type(type.element)]);
        }
        return renderRustCompoundType('option', type.absence, [this.type(type.element)]);
      case 'tuple':
        return renderRustCompoundType('tuple', '', type.elements.map((element) => this.type(element)));
      case 'record':
        return renderRustCompoundType('record', '', type.fields.map((field) => this.type(field.type)));
      case 'function':
        return renderRustCompoundType('function', this.type(type.ret), type.params.map((param) => this.type(param)));
      case 'set':
        return renderRustCompoundType('set', '', [this.type(type.element)]);
      case 'map':
        return renderRustCompoundType('map', '', [this.type(type.key), this.type(type.value)]);
      case 'cell':
        return renderRustCompoundType('cell', '', [this.type(type.element)]);
      case 'unit':
        return '()';
      case 'data':
        return `crate::${this.state.ref(type.name, '::')}`;
      case 'array':
        this.state.encode('arrays', 'a JavaScript array, which the portable core never mutates, is a Rust Vec; a read outside it panics');
        return renderRustCompoundType('array', '', [this.type(type.element)]);
      case 'union':
        throw typeError(readUnionTargetRefusal('Rust'));
      default:
        throw new Error(`no Rust type for ${type.kind}`);
    }
  }

  fieldType(type) {
    return type.kind === 'data' ? `Box<${this.type(type)}>` : this.type(type);
  }

  data(entry) {
    if (typeContains(this.program, { kind: 'data', name: entry.fullName }, 'function')) throw unsupported('callable data field', 'callable fields need their own identity, debug and equality encodings before this target derives structural traits', entry.span);
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    this.state.encode('data', 'a data type is an enum with one tuple variant per constructor; data-typed fields are boxed, and values are compared structurally');
    const variants = entry.ctors.map((ctor) => {
      const local = this.state.ctorLocal(entry.fullName, ctor.name);
      if (!ctor.fields.length) return `    ${local},`;
      return `    ${local}(${ctor.fields.map((field) => this.fieldType(field.type)).join(', ')}),`;
    });
    // f64 is not `Eq`, so a data type that holds a Number, directly or through another data type, derives `PartialEq` alone.
    const derives = this.holdsFloat(entry.fullName) ? 'Clone, Debug, PartialEq' : 'Clone, Debug, PartialEq, Eq';
    return [`#[derive(${derives})]`, `pub enum ${name} {`, ...variants, '}'].join('\n');
  }

  holdsFloat(name, seen = new Set()) {
    return typeContains(this.program, { kind: 'data', name }, 'float', seen);
  }

  fn(entry) {
    const { params, body } = renameFunction(entry, snake, this.state.localReserved());
    this.arrayFinalUses = this.arrayConsumption ? readOwnedArrayFinalUses(body) : new Set();
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const sourceHeader = this.sourceHeaders?.get(entry.fullName);
    if (sourceHeader) {
      this.arrayFinalUses = new Set();
      if (this.borrowedCallTargets) {
        // The fresh scalar names cannot capture a parameter, local binder or
        // free variable of any later argument. Keep checked input names intact.
        const reserved = this.state.localReserved();
        for (const parameter of params) reserved.add(parameter.name);
        const pending = [body];
        for (let at = 0; at < pending.length; at += 1) {
          const node = pending[at];
          if (node.k === 'var' || node.k === 'let') reserved.add(node.name);
          for (const [key, value] of Object.entries(node)) {
            if (key === 'type' || key === 'domain' || key === 'span') continue;
            if (Array.isArray(value)) pending.push(...value.filter(item => item?.k));
            else if (value?.k) pending.push(value);
          }
        }
        this.borrowedCallScope = new Scope(snake, reserved);
        const text = this.expr(body);
        this.borrowedCallScope = null;
        return renderRustSourceHeaderEmission(sourceHeader, params, name, text, indent, renderRustBorrowedCallHeaderReference);
      }
      return renderRustSourceHeaderEmission(sourceHeader, params, name, this.expr(body), indent);
    }
    const loop = tailLoop(entry, body);
    if (loop) {
      // A lifted loop runs as a loop: each iteration assigns the parameters their next values.
      this.loopParams = params.map((param) => param.name);
      this.arrayFinalUses = this.arrayConsumption ? readOwnedArrayFinalUses(loop) : new Set();
      const text = this.expr(loop);
      this.loopParams = null;
      const binders = params.map((param) => `mut ${param.name}: ${this.type(param.type)}`).join(', ');
      return `pub fn ${name}(${binders}) -> ${this.type(entry.ret)} {\n${indent(`loop {\n${indent(`return ${text};`, 1)}\n}`, 1)}\n}`;
    }
    const binders = params.map((param) => `${param.name}: ${this.type(param.type)}`).join(', ');
    return renderRustConstFunctionHeader(entry,`pub fn ${name}(${binders}) -> ${this.type(entry.ret)} {\n${indent(this.expr(body), 1)}\n}`);
  }

  theorem(entry) {
    const { binders, prop } = renameTheorem(entry, snake, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    this.state.theorem(entry, name, { closedGoal: binders.length === 0, discharge: 'source-kernel', check: 'bounded' });
    this.theoremChecks.push({ ref: `crate::${this.state.ref(entry.fullName, '::')}`, binders, source: entry.fullName });
    const params = binders.map((binder) => `${binder.name}: ${this.type(binder.type)}`).join(', ');
    return `/// Executable form of theorem ${entry.fullName}.\npub fn ${name}(${params}) -> bool {\n${indent(this.prop(prop), 1)}\n}`;
  }

  theoremRunner() {
    if (!this.theoremChecks.length) return null;
    this.state.encode('theorem-properties', 'each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel');
    const checks = this.theoremChecks.map(({ ref, binders, source }) => {
      const call = `${ref}(${binders.map((binder) => this.own(binder.name, binder.type)).join(', ')})`;
      const holds = binders.reduceRight(
        (inner, binder) => `${this.domain(binder.type, 3)}.into_iter().all(|${binder.name}| ${inner})`,
        call,
      );
      return [
        `    if !(${holds}) {`,
        `        panic!("theorem ${formatEscape(source)} fails on a bounded input");`,
        '    }',
        `    println!("theorem ${formatEscape(source)}: holds on the bounded domain");`,
      ].join('\n');
    });
    return `fn ml_check_theorems() {\n${checks.join('\n')}\n}`;
  }

  entry(hasMain) {
    const run = this.theoremChecks.length
      ? (hasMain ? 'if check { ml_check_theorems() } else { ml_main() }' : 'if check { ml_check_theorems() }')
      : (hasMain ? 'ml_main()' : '');
    return [
      '// Deep recursion in the source is not bounded by a small native stack.',
      'fn main() {',
      '    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");',
      '    let worker = std::thread::Builder::new()',
      '        .stack_size(1 << 28)',
      `        .spawn(move || { ${run} })`,
      '        .expect("spawn the program thread");',
      '    if worker.join().is_err() {',
      '        std::process::exit(101);',
      '    }',
      '}',
    ].join('\n');
  }

  /** Values of a type up to a constructor depth, as a Rust `Vec`. */
  domain(type, depth) {
    switch (type.kind) {
      case 'nat':
        this.usesBig = true;
        return depth >= 3 ? 'crate::ml::range(0, 6)' : 'crate::ml::range(0, 2)';
      case 'int':
        this.usesBig = true;
        return depth >= 3 ? 'crate::ml::range(-4, 4)' : 'crate::ml::range(-1, 1)';
      case 'fixed':
        return `vec![${(type.signed ? [-1, 0, 1] : [0, 1, 2]).map((value) => `${value}${typeKey(type)}`).join(', ')}]`;
      case 'bool':
        return 'vec![false, true]';
      case 'string':
        return renderPrimitiveStringRepresentation(this.utf16Strings, 'examples', '', '', '');
      case 'unit':
        return 'vec![()]';
      case 'data': {
        const entry = this.program.declarations.get(type.name);
        const lines = [`let mut values: Vec<${this.type(type)}> = Vec::new();`];
        for (const ctor of entry.ctors) {
          const recursive = ctor.fields.some((field) => field.type.kind === 'data');
          if (recursive && depth <= 1) continue;
          const names = ctor.fields.map((_, index) => `f${index}`);
          const args = ctor.fields.map((field, index) => (field.type.kind === 'data' ? `Box::new(${names[index]}.clone())` : `${names[index]}.clone()`));
          const head = `crate::${this.state.ctorRef(entry.fullName, ctor.name, '::')}`;
          let statement = `values.push(${args.length ? `${head}(${args.join(', ')})` : head});`;
          for (let index = ctor.fields.length - 1; index >= 0; index -= 1) {
            const field = ctor.fields[index];
            statement = `for ${names[index]} in ${this.domain(field.type, field.type.kind === 'data' ? depth - 1 : 1)} {\n${indent(statement, 1)}\n}`;
          }
          lines.push(statement);
        }
        lines.push('values');
        return `{\n${indent(lines.join('\n'), 1)}\n}`;
      }
      default:
        throw new Error(`no Rust domain for ${type.kind}`);
    }
  }

  prop(prop) {
    switch (prop.p) {
      case 'forall': {
        const body = this.prop(prop.body);
        return prop.binders.reduceRight(
          (inner, binder) => `${this.domain(binder.type, 3)}.into_iter().all(|${binder.name}| ${inner})`,
          body,
        );
      }
      case 'and':
        return `(${this.prop(prop.left)} && ${this.prop(prop.right)})`;
      case 'or':
        return `(${this.prop(prop.left)} || ${this.prop(prop.right)})`;
      case 'implies':
        return `(!${this.prop(prop.left)} || ${this.prop(prop.right)})`;
      case 'not':
        return `!${this.prop(prop.arg)}`;
      case 'bool':
        return this.expr(prop.expr);
      default: {
        if (prop.sameValue) {
          this.usesNumber = true;
          const same = `crate::ml_number::same_value(${this.expr(prop.left)}, ${this.expr(prop.right)})`;
          return prop.p === 'eq' ? same : `!${same}`;
        }
        const operator = { eq: '==', ne: '!=', lt: '<', le: '<=', gt: '>', ge: '>=' }[prop.p];
        // Comparisons take their operands by reference, so they read them in place.
        return `(${this.compared(prop.left)} ${operator} ${this.compared(prop.right)})`;
      }
    }
  }

  /** An owned copy of a variable. */
  own(name, type) {
    return isCopyType(type) ? name : `${name}.clone()`;
  }

  /** An expression whose value is only read: variables are borrowed, not cloned. */
  borrow(e) {
    if (e.type?.kind === 'float') return this.expr(e);
    return e.k === 'var' ? `&${e.name}` : `&${this.expr(e)}`;
  }

  /** A comparison operand: read in place, a string literal as a `&str`. */
  compared(e) {
    if (this.utf16Strings && e.type.kind === 'string') return this.receiver(e);
    if (e.k === 'lit' && e.type.kind === 'string') return rustString(String(e.value));
    if (e.k === 'call' && this.externals.get(e.fn)?.literal && e.type.kind === 'string') return e.fn;
    return this.receiver(e);
  }

  /** A method receiver: `&self` methods borrow a variable in place. */
  receiver(e) {
    return e.k === 'var' ? e.name : `(${this.expr(e)})`;
  }

  /** Read nested product fields in place; clone only an ownership-bearing result. */
  tuplePlace(e) {
    const object = e.object.k === 'tupleIndex' ? this.tuplePlace(e.object)
      : e.object.k === 'var' ? e.object.name : this.expr(e.object);
    return renderTupleProjection('Rust', object, String(e.index));
  }

  tupleProjection(e) {
    const text = this.tuplePlace(e);
    return isCopyType(e.type, e.productModel) ? text : `(${text}).clone()`;
  }

  lambda(e) {
    const captured = readCallableCapturedBindings(e.body, e.params.map(param => param.name));
    const copies = [...captured].map(([name, type]) => `let ${name} = ${this.own(name, type)};`);
    return renderCallableLambda('Rust', e.params.map(param => param.name), e.params.map(param => this.type(param.type)), this.type(e.type.ret), this.expr(e.body), copies);
  }

  optionalMatch(arg, binder, present, nullCase, undefinedCase) {
    if (arg.type.absence === 'both') this.usesOptional = true;
    return renderOptionalMatch('Rust', arg.type.absence, this.expr(arg), binder, present, nullCase, undefinedCase);
  }

  truthy(arg) {
    if (arg.type.kind === 'option') return this.optionalMatch(arg, 'ml_present', this.truthy({ k: 'var', name: 'ml_present', type: arg.type.element }), 'false', 'false');
    const value = this.receiver(arg);
    const evaluated = ['int', 'nat', 'fixed', 'string'].includes(arg.type.kind) ? '' : this.expr(arg);
    return renderOptionalPrimitiveTruth('Rust', arg.type.kind, evaluated, value, this.utf16Strings, Boolean(arg.type.signed));
  }

  strictEqual(e) {
    const typeA = e.left.type.kind === 'option' ? e.left.type.element : e.left.type;
    const typeB = e.right.type.kind === 'option' ? e.right.type.element : e.right.type;
    if (e.left.type.absence === 'both' || e.right.type.absence === 'both') this.usesOptional = true;
    const compare = typeKey(typeA) === typeKey(typeB) ? 'ml_left == ml_right' : 'false';
    return renderOptionalStrictEquality('Rust', e.left.type.kind, e.left.type.absence ?? '', e.right.type.kind, e.right.type.absence ?? '', compare, this.expr(e.left), this.expr(e.right), e.negate);
  }

  method(e) {
    if (e.object.type.kind === 'string') {
      const object = this.receiver(e.object);
      const argumentsText = e.args.map((arg, index) => renderPrimitiveMethodArgument('Rust', 'string', e.op, index, arg === null,
        arg === null ? 'None' : arg.type.kind === 'string' ? this.borrow(arg) : this.expr(arg)));
      return renderPrimitiveMethodExpression('Rust', 'string', e.op, object, argumentsText.join(', '));
    }
    if (e.object.type.kind === 'array') {
      this.usesArray = true;
      if (['slice', 'indexOf', 'lastIndexOf', 'includes'].includes(e.op)) this.usesStandardArray = true;
      if (e.op === 'join') {
        const stringElement = e.object.type.element.kind === 'string';
        const operand = (value, role) => readPrimitiveArrayJoinOperandForm(this.utf16Strings, stringElement, role) === 'borrow' ? this.borrow(value) : this.receiver(value);
        const text = (name, type) => {
          const form = readPrimitiveArrayJoinElementForm(type.kind);
          const empty = form === 'empty' || form === 'optional' ? this.literal({ type: { kind: 'string' }, value: '' }) : '';
          if (form === 'optional' && type.absence === 'both') this.usesOptional = true;
          const present = form === 'optional' ? text('ml_present', type.element) : form === 'empty' ? ''
            : this.expr({ k: 'toString', arg: { k: 'var', name, type }, type: { kind: 'string' } });
          return renderPrimitiveArrayJoinElement(type.kind, type.absence ?? '', name, present, empty);
        };
        const object = operand(e.object, 'object');
        const separator = operand(e.args[0], 'separator');
        return renderPrimitiveArrayJoinExpression(this.utf16Strings, stringElement, object, separator,
          stringElement ? '' : text('ml_value', e.object.type.element));
      }
      if (e.op === 'slice') return renderPrimitiveMethodExpression('Rust', 'array', e.op, this.borrow(e.object), e.args.map((arg) => this.expr(arg)).join(', '));
      if (['indexOf', 'lastIndexOf', 'includes'].includes(e.op)) {
        const element = e.object.type.element;
        return renderPrimitiveArraySearchExpression(e.op, this.borrow(e.object), this.expr(e.args[0]), this.expr(e.args[1]),
          element.kind === 'option' ? element.element.kind : element.kind, element.kind === 'option' ? element.absence : '');
      }
      this.temporaries += 1;
      const suffix = this.temporaries;
      const fn = e.args[0];
      if (e.op === 'map') return renderArrayFromMapping('Rust', this.expr(e.object), this.expr(fn), fn.type.params.length, String(suffix));
      const parameterCount = fn.type.params.length;
      const object = this.expr(e.object);
      const callbackText = this.expr(fn);
      const hasInitial = readArrayCallbackCheckedArgumentPosition(e.op, e.args.length !== 1, 0) === 1;
      const initial = hasInitial ? this.expr(e.args[1]) : '';
      const referenceResult = fn.type.ret.kind === 'array' && fn.type.ret.reference === true;
      const rendered = renderArrayCallbackReferenceExpression(e.op, object, callbackText, parameterCount, String(suffix), referenceResult)
        || renderArrayCallbackExpression(e.op, object, callbackText, parameterCount, String(suffix), initial, hasInitial);
      if (rendered) return rendered;
    }
    throw unsupported(`method call .${e.op}()`, 'the Rust target has no operational encoding for this method', e.span);
  }

  expr(e) {
    if (e.k === 'ctor' && e.frozen && this.program.sourceLanguage === 'JavaScript') throw unsupported('explicit data object freeze', 'this target needs a JavaScript property descriptor model to retain explicit constructor freezing', e.span);
    const statefulReceiver = e.array?.type.reference || e.object?.type.reference;
    const referenceOperation = readSurfaceReferenceEmissionOperation(e.k, Boolean(statefulReceiver), Boolean(e.type?.reference));
    if (referenceOperation) throw unsupported(referenceOperation, readSurfaceReferenceEmissionRefusal(referenceOperation), e.span);
    switch (e.k) {
      case 'lit':
        return this.literal(e);
      case 'unit':
        return '()';
      case 'var':
        return this.arrayFinalUses?.has(e) ? e.name : this.own(e.name, e.type);
      case 'call':
        if (this.borrowedCallTargets) {
          const signature = this.borrowedCallTargets.get(e.fn);
          const values = e.args.map(argument => this.expr(argument));
          const names = values.map(() => this.borrowedCallScope.fresh(RUST_BORROWED_CALL_INTERFACE_POLICY.argumentName));
          return renderRustBorrowedCall(signature, `crate::${this.state.ref(e.fn, '::')}`, values, names, indent);
        }
        if (this.externals.has(e.fn)) return this.external(e, this.externals.get(e.fn));
        return `crate::${this.state.ref(e.fn, '::')}(${e.args.map((arg) => this.expr(arg)).join(', ')})`;
      case 'functionRef':
        return renderCallableFunctionReference('Rust', `crate::${this.state.ref(e.fn, '::')}`);
      case 'apply':
        return renderCallableApplication('Rust', this.expr(e.fn), e.args.map(arg => this.expr(arg)));
      case 'lambda':
        return this.lambda(e);
      case 'method':
        return this.method(e);
      case 'record':
        return renderRecordValue('Rust', [], e.entries.map(entry => this.expr(entry.value)));
      case 'cellAllocate':
        return renderCallableCellOperation('Rust', e.k, '', this.expr(e.value), '', '', '');
      case 'cellRead':
        return renderCallableCellOperation('Rust', e.k, this.receiver(e.cell), '', '', '', '');
      case 'cellWrite': {
        this.temporaries += 1;
        const cell = `ml_cell${this.temporaries}`;
        const value = `ml_value${this.temporaries}`;
        return renderCallableCellOperation('Rust', e.k, this.expr(e.cell), this.expr(e.value), this.expr(e.body), cell, value);
      }
      case 'referenceAllocate':
        return renderSurfaceReferenceOperation('Rust', e.k, this.expr(e.value), '');
      case 'referenceRead': {
        this.temporaries += 1;
        const value = `ml_reference_value${this.temporaries}`;
        return renderSurfaceReferenceOperation('Rust', e.k, this.receiver(e.reference), value);
      }
      case 'recordField':
        return renderRecordField('Rust', this.expr(e.object), '', String(e.index));
      case 'tuple':
        if (Object.hasOwn(e, 'productModel') && !acceptProgramTupleProductModel(e.productModel, e.frozen === true)) throw unsupported('immutable tuple product model', 'the declared immutable value product requires its retained frozen facet', e.span);
        return renderTupleValue('Rust', e.items.map((item) => this.expr(item)), e.frozen === true);
      case 'tupleIndex':
        return this.tupleProjection(e);
      case 'none':
        if (e.type.absence === 'both') this.usesOptional = true;
        return renderOptionalAbsentValue('Rust', e.type.absence, e.absence, e.type.absence === 'both' ? '' : this.type(e.type.element));
      case 'some':
        if (e.type.absence === 'both') this.usesOptional = true;
        return renderOptionalPresentValue('Rust', e.type.absence, this.expr(e.value));
      case 'nullishNumber': {
        const element = e.arg.type.element;
        const present = renderOptionalNumericPresent('Rust', element.kind, 'arithmetic');
        return this.optionalMatch(e.arg, 'ml_present', present, renderAbsentNumberExpression('Rust', 'arithmetic', 'null'), renderAbsentNumberExpression('Rust', 'arithmetic', 'undefined'));
      }
      case 'optionalIndexNumber':
        return this.optionalMatch(e.arg, 'ml_present', renderOptionalNumericPresent('Rust', e.arg.type.element.kind, 'array-index'), renderAbsentNumberExpression('Rust', 'array-index', 'null'), renderAbsentNumberExpression('Rust', 'array-index', 'undefined'));
      case 'truthy':
        return this.truthy(e.arg);
      case 'strictEqual':
        return this.strictEqual(e);
      case 'optionalText': {
        const element = e.arg.type.element;
        let present = element.kind === 'unit' ? this.literal({ type: { kind: 'string' }, value: 'undefined' }) : this.expr({ k: 'toString', arg: { k: 'var', name: 'ml_present', type: element }, console: e.console, type: { kind: 'string' } });
        present = renderOptionalConsoleText('Rust', present, ['int', 'nat', 'fixed'].includes(element.kind), e.console, this.utf16Strings);
        return this.optionalMatch(e.arg, 'ml_present', present, this.literal({ type: { kind: 'string' }, value: 'null' }), this.literal({ type: { kind: 'string' }, value: 'undefined' }));
      }
      case 'optionalTypeOf':
        return this.optionalMatch(e.arg, 'ml_present', renderOptionalTypeQueryPresent('Rust', this.literal({ type: { kind: 'string' }, value: e.present ?? 'undefined' })), this.literal({ type: { kind: 'string' }, value: 'object' }), this.literal({ type: { kind: 'string' }, value: 'undefined' }));
      case 'unwrapOption':
        if (e.arg.type.absence === 'both') this.usesOptional = true;
        return renderOptionalUnwrapping('Rust', e.arg.type.absence, e.arg.type.absence === 'both' ? this.expr(e.arg) : '', e.arg.type.absence === 'both' ? '' : this.receiver(e.arg), '');
      case 'optionalPredicate': {
        const present = requireOptionalNumericPredicate(e.arg.type.element.kind) ? this.math({ k: 'math', op: e.op, args: [{ k: 'var', name: 'ml_present', type: { kind: 'float' } }], type: { kind: 'bool' } }) : 'false';
        return this.optionalMatch(e.arg, 'ml_present', present, 'false', 'false');
      }
      case 'flattenOption': {
        if (e.type.absence === 'both') this.usesOptional = true;
        return renderOptionalFlattening('Rust', e.value.type.absence, e.value.type.element.absence, e.type.absence, this.expr(e.value));
      }
      case 'coalesce':
        if (e.left.type.absence === 'both') this.usesOptional = true;
        return renderOptionalCoalescing('Rust', e.left.type.absence, e.left.type.absence === 'both' ? this.expr(e.left) : '', e.left.type.absence === 'both' ? '' : this.receiver(e.left), this.expr(e.right));
      case 'optionalField': {
        if (e.object.type.absence === 'both') this.usesOptional = true;
        return renderOptionalProjection('Rust', e.object.type.absence, e.object.type.absence === 'both' ? this.expr(e.object) : '', e.object.type.absence === 'both' ? '' : this.receiver(e.object), 'ml_record', `ml_record.${e.index}`, '');
      }
      case 'optionalProject': {
        const element = e.object.type.element;
        const length = element.kind === 'string' ? this.utf16Strings ? 'ml_value.length()' : renderStringLengthExpression('Rust', 'ml_value') : '(ml_value.len() as f64)';
        if (e.object.type.absence === 'both') this.usesOptional = true;
        return renderOptionalProjection('Rust', e.object.type.absence, e.object.type.absence === 'both' ? this.expr(e.object) : '', e.object.type.absence === 'both' ? '' : this.receiver(e.object), 'ml_value', length, '');
      }
      case 'optionalIndex': {
        this.usesArray = true;
        this.usesStandardArray = true;
        const index = e.index.type.kind === 'float' ? this.expr(e.index) : this.receiver(e.index);
        return renderOptionalArrayRead('Rust', e.index.type.kind, this.borrow(e.array), index);
      }
      case 'stringPoints':
        return renderStringPointIteration('Rust', this.utf16Strings, this.receiver(e.object));
      case 'set':
      case 'map': {
        this.usesArray = true;
        this.usesStandardArray = true;
        const key = e.k === 'set' ? e.type.element : e.type.key;
        const values = this.expr(e.k === 'set' ? e.items : e.entries);
        return renderCollectionConstruction('Rust', e.k, key.kind, values, '', '', '');
      }
      case 'collectionSize':
        return `(${this.receiver(e.object)}.len() as f64)`;
      case 'collectionRead': {
        const keyType = e.object.type.kind === 'map' ? e.object.type.key : e.object.type.element;
        const equal = keyType.kind === 'float' ? '(ml_key == &ml_search || (ml_key.is_nan() && ml_search.is_nan()))' : 'ml_key == &ml_search';
        const binds = e.object.type.kind === 'map' ? '(ml_key, ml_value)' : 'ml_key';
        return `{ let ml_collection = ${this.expr(e.object)}; let ml_search = ${this.expr(e.key)}; ml_collection.iter().${e.op === 'get' ? `find_map(|${binds}| if ${equal} { Some(ml_value.clone()) } else { None })` : `any(|${binds}| ${equal})`} }`;
      }
      case 'collectionItems': {
        const map = e.object.type.kind === 'map';
        const value = e.op === 'entries' && map || !map && e.op !== 'entries' ? this.expr(e.object) : this.receiver(e.object);
        return renderCollectionIteration('Rust', map, e.op, value, '', '');
      }
      case 'ctor': {
        const entry = this.program.declarations.get(e.data);
        const ctor = entry.ctors.find((candidate) => candidate.name === e.ctor);
        const head = `crate::${this.state.ctorRef(e.data, e.ctor, '::')}`;
        if (!e.args.length) return head;
        const args = e.args.map((arg, index) => (ctor.fields[index].type.kind === 'data' ? `Box::new(${this.expr(arg)})` : this.expr(arg)));
        return `${head}(${args.join(', ')})`;
      }
      case 'unary':
        if (e.op === 'not') return `!${this.receiver(e.arg)}`;
        if (e.type.kind === 'fixed') return this.checked(`${this.receiver(e.arg)}.checked_neg()`, e.type, 'neg');
        if (e.type.kind === 'float') return `(-${this.receiver(e.arg)})`;
        return `${this.receiver(e.arg)}.neg()`;
      case 'binary':
        return this.binary(e);
      case 'if':
        return `if ${this.expr(e.cond)} {\n${indent(this.expr(e.then), 1)}\n} else {\n${indent(this.expr(e.else), 1)}\n}`;
      case 'let':
        return `{\n${indent(`let ${e.name} = ${this.expr(e.value)};\n${this.expr(e.body)}`, 1)}\n}`;
      case 'match':
        return this.match(e);
      case 'print':
        return `{\n${indent(`println!("{}", ${this.expr(e.text)});\n${this.expr(e.body)}`, 1)}\n}`;
      case 'toString':
        if (e.arg.type.kind === 'string') return this.expr(e.arg);
        if (e.arg.type.kind === 'data' || e.arg.type.kind === 'unit') {
          throw unsupported('output of structured values', `a ${e.arg.type.kind} value has no portable textual form`, e.arg.span);
        }
        if (e.arg.type.kind === 'float') {
          this.usesNumber = true;
          const text = `crate::ml_number::${readNumberDisplayFunction(e.arg.type.kind, Boolean(e.console))}(${this.expr(e.arg)})`;
          return renderPrimitiveStringRepresentation(this.utf16Strings, 'text', text, '', '');
        }
        return renderPrimitiveStringRepresentation(this.utf16Strings, 'text', `${this.receiver(e.arg)}.to_string()`, '', '');
      case 'stringMap':
        if (this.utf16Strings) {
          return renderPrimitiveStringRepresentation(true, 'map', this.receiver(e.string), '', e.op);
        }
        return renderStringMapExpression('Rust', e.op, this.receiver(e.string));
      case 'stringTest': {
        if (requireStringOrderUnits('stringTest', e.op)) return renderStringOrderExpression('Rust', e.op, this.borrow(e.string), this.borrow(e.search));
        if (this.utf16Strings) {
          return renderPrimitiveStringRepresentation(true, 'test', this.receiver(e.string), this.borrow(e.search), e.op);
        }
        const search = e.search.k === 'lit' ? rustString(String(e.search.value)) : `${this.receiver(e.search)}.as_str()`;
        return renderStringTestExpression('Rust', e.op, this.receiver(e.string), search);
      }
      case 'cast':
        return this.cast(e);
      case 'abort':
        return `panic!("{}", ${rustString(e.message)})`;
      case 'array':
        return e.items.length ? `vec![${e.items.map((item) => this.expr(item)).join(', ')}]` : `Vec::<${this.type(e.type.element)}>::new()`;
      case 'append':
        this.usesArray = true;
        if (this.arrayFinalUses?.has(e.left)) {
          // The right value may inspect the left array before its final move.
          return `{ let (ml_append_right, ml_append_left) = (${this.expr(e.right)}, ${this.expr(e.left)}); crate::ml_array::append(ml_append_left, ml_append_right) }`;
        }
        return `crate::ml_array::append(${this.expr(e.left)}, ${this.expr(e.right)})`;
      case 'index': {
        this.usesArray = true;
        if (!e.provenInBounds) this.state.arrayRead();
        const index = e.index.type.kind === 'float'
          ? `crate::ml_array::number_index(${this.expr(e.index)})`
          : `${this.receiver(e.index)}.to_index()`;
        return `crate::ml_array::at(${this.borrow(e.array)}, ${index})`;
      }
      case 'length':
        if (e.array.type.kind === 'string') return this.utf16Strings ? renderPrimitiveStringRepresentation(true, 'length', this.receiver(e.array), '', '') : renderStringLengthExpression('Rust', this.receiver(e.array));
        if (e.type.kind === 'float') return `(${this.receiver(e.array)}.len() as f64)`;
        this.usesBig = true;
        return `crate::ml::Big::from_u128(${this.receiver(e.array)}.len() as u128)`;
      case 'math':
        return this.math(e);
      case 'recur': {
        // A variable the next iteration keeps as it is, an array most of all, is not copied.
        const kept = e.args.map((arg, index) => arg.k === 'var' && arg.name === this.loopParams[index]);
        const args = e.args.map((arg) => this.expr(arg)).filter((_, index) => !kept[index]);
        const names = this.loopParams.filter((_, index) => !kept[index]);
        const assign = names.length === 1 ? `${names[0]} = ${args[0]};\n` : names.length ? `(${names.join(', ')}) = (${args.join(', ')});\n` : '';
        return `{\n${indent(`${assign}continue;`, 1)}\n}`;
      }
      default:
        throw new Error(`no Rust expression for ${e.k}`);
    }
  }

  /** Math and Number functions: f64 methods where they agree with JavaScript, ml_math otherwise. */
  math(e) {
    this.floats();
    const bitwiseCode=readJavaScriptNumberBitwiseOperationCode(e.op);
    if(bitwiseCode>=0){
      if(e.args.length!==readJavaScriptNumberBitwiseArity(e.op))throw unsupported('Number bitwise arity','the operation requires its exact checked operand inventory',e.span);
      this.usesNumberBitwise=true;
      return `crate::ml_number_bitwise::${snake(e.op)}(${e.args.map(arg=>this.expr(arg)).join(', ')})`;
    }
    const [arg] = e.args;
    switch (e.op) {
      case 'abs':
      case 'floor':
      case 'ceil':
      case 'trunc':
      case 'sqrt':
        return `${this.receiver(arg)}.${e.op}()`;
      case 'isFinite':
        return `${this.receiver(arg)}.is_finite()`;
      case 'isNaN':
        return `${this.receiver(arg)}.is_nan()`;
      case 'max':
      case 'min':
        if (!e.args.length) return e.op === 'max' ? 'f64::NEG_INFINITY' : 'f64::INFINITY';
        this.usesMath = true;
        return e.args.map((value) => this.expr(value)).reduce((left, right) => `crate::ml_math::${e.op}(${left}, ${right})`);
      default: {
        this.usesMath = true;
        const name = { maxOf: 'max_of', minOf: 'min_of', isInteger: 'is_integer', isSafeInteger: 'is_safe_integer' }[e.op] ?? e.op;
        return `crate::ml_math::${name}(${e.op.endsWith('Of') ? this.borrow(arg) : this.expr(arg)})`;
      }
    }
  }

  literal(e) {
    switch (e.type.kind) {
      case 'nat':
      case 'int': {
        this.usesBig = true;
        const value = BigInt(e.value);
        const small = value >= -(1n << 126n) && value < (1n << 126n);
        return small ? `crate::ml::Big::from_i128(${e.value})` : `crate::ml::Big::parse("${e.value}")`;
      }
      case 'fixed':
        return e.value.startsWith('-') ? `(${e.value}${typeKey(e.type)})` : `${e.value}${typeKey(e.type)}`;
      case 'float':
        this.floats();
        return rustFloat(e.value);
      case 'bool':
        return String(e.value);
      case 'string':
        return renderPrimitiveStringRepresentation(this.utf16Strings, 'literal', this.utf16Strings
          ? Array.from({ length: String(e.value).length }, (_, index) => String(e.value).charCodeAt(index)).join(', ')
          : rustString(String(e.value)), '', '');
      default:
        throw new Error(`no Rust literal for ${e.type.kind}`);
    }
  }

  /** A machine-integer result out of range panics, with Rust's own message. */
  checked(text, type, op) {
    this.machineInteger(type);
    return `${text}.expect(${rustString(overflowMessage(op))})`;
  }

  machineInteger(type) {
    this.state.encode(`machine-integer:${typeKey(type)}`, `${typeKey(type)} stays a Rust ${typeKey(type)}; its arithmetic is checked and panics where the source aborts, with the source's message`);
  }

  binary(e) {
    const sameDomain = sameType(e.left.type, e.right.type);
    const domainPresent = e.domain !== undefined && e.domain !== null;
    const refusal = readReferenceIdentityComparisonError(e.op, e.left.type.kind, e.right.type.kind, e.left.type.reference === true, e.right.type.reference === true, sameDomain, domainPresent, e.domain?.reference === true, domainPresent && sameType(e.domain, e.left.type));
    if (refusal) throw unsupported('reference comparison', refusal, e.span);
    if (acceptReferenceIdentityComparison(e.op, e.left.type.kind, e.right.type.kind, e.left.type.reference === true, e.right.type.reference === true, sameDomain)) {
      return renderReferenceIdentityComparison(e.op, this.borrow(e.left), this.borrow(e.right));
    }
    switch (e.op) {
      case 'and':
        return `(${this.expr(e.left)} && ${this.expr(e.right)})`;
      case 'or':
        return `(${this.expr(e.left)} || ${this.expr(e.right)})`;
      case 'concat':
        return renderPrimitiveStringRepresentation(this.utf16Strings, 'concat', this.receiver(e.left), this.utf16Strings ? this.borrow(e.right) : this.receiver(e.right), '');
      case 'eq':
      case 'ne':
      case 'lt':
      case 'le':
      case 'gt':
      case 'ge': {
        const operator = { eq: '==', ne: '!=', lt: '<', le: '<=', gt: '>', ge: '>=' }[e.op];
        return `(${this.compared(e.left)} ${operator} ${this.compared(e.right)})`;
      }
      default:
        if (e.semantics === 'ieee') return this.floatArithmetic(e);
        return e.type.kind === 'fixed' ? this.fixedArithmetic(e) : this.bigArithmetic(e);
    }
  }

  floats() {
    this.state.encode('floats', 'a JavaScript Number is an f64 with the same IEEE-754 arithmetic; % on f64 is the same truncated remainder, and ml_number::js_number prints a value as JavaScript does');
  }

  /** f64 arithmetic is IEEE-754 binary64 arithmetic, as JavaScript's; Rust's `%` is fmod, as JavaScript's. */
  floatArithmetic(e) {
    const operator = { add: '+', sub: '-', mul: '*', div: '/', rem: '%' }[e.op];
    return `(${this.expr(e.left)} ${operator} ${this.expr(e.right)})`;
  }

  fixedArithmetic(e) {
    const left = this.receiver(e.left);
    const right = this.expr(e.right);
    if (e.op === 'div' || e.op === 'rem') {
      if (e.rounding !== 'trunc' && e.rounding !== 'euclid' && e.type.signed) {
        throw unsupported(`${e.rounding} division on ${typeKey(e.type)}`, 'Rust machine integers divide with truncating or Euclidean rounding only', e.span);
      }
      // Rust integer division always panics by zero and on overflow, with the source's messages.
      this.machineInteger(e.type);
      if (e.rounding === 'euclid') return `${left}.${e.op}_euclid(${right})`;
      return `(${left} ${e.op === 'div' ? '/' : '%'} ${right})`;
    }
    return this.checked(`${left}.checked_${e.op}(${right})`, e.type, e.op);
  }

  bigArithmetic(e) {
    const left = this.receiver(e.left);
    const right = this.borrow(e.right);
    switch (e.op) {
      case 'add':
        return `${left}.add(${right})`;
      case 'mul':
        return `${left}.mul(${right})`;
      case 'sub':
        if (e.semantics === 'truncated') return `crate::ml::nat_sub(${this.borrow(e.left)}, ${right})`;
        return `${left}.sub(${right})`;
      case 'div':
      case 'rem': {
        const zero = e.byZero === 'abort' ? `Some(${rustString(zeroDivisorMessage(e.op, e.type))})` : 'None';
        return `crate::ml::divide(${this.borrow(e.left)}, ${right}, "${e.rounding}", ${zero}, ${e.op === 'rem'})`;
      }
      default:
        throw new Error(`no Rust operator ${e.op}`);
    }
  }

  match(e) {
    const fallback = e.cases.find((kase) => kase.pattern.k === 'wild' || kase.pattern.k === 'bind');
    if (e.scrutinee.type.kind === 'data') {
      const entry = this.program.declarations.get(e.scrutinee.type.name);
      const arms = e.cases.map((kase) => {
        const { pattern } = kase;
        if (pattern.k === 'wild') return `_ => ${block(this.expr(kase.body))}`;
        if (pattern.k === 'bind') return `${pattern.name} => ${block(this.expr(kase.body))}`;
        const ctor = entry.ctors.find((candidate) => candidate.name === pattern.ctor);
        const head = `crate::${this.state.ctorRef(pattern.data, pattern.ctor, '::')}`;
        const binds = pattern.binds.map((bind) => bind ?? '_');
        const unbox = pattern.binds
          .map((bind, index) => (bind && ctor.fields[index].type.kind === 'data' ? `let ${bind} = *${bind};` : null))
          .filter(Boolean);
        const text = binds.length ? `${head}(${binds.join(', ')})` : head;
        return `${text} => ${block([...unbox, this.expr(kase.body)].join('\n'))}`;
      });
      return `match ${this.expr(e.scrutinee)} {\n${indent(arms.join('\n'), 1)}\n}`;
    }
    const natural = e.cases.some((kase) => kase.pattern.k === 'natZero' || kase.pattern.k === 'natSucc');
    if (e.scrutinee.type.kind !== 'nat' && !natural) {
      const kase = fallback;
      const pattern = kase.pattern.k === 'bind' ? kase.pattern.name : '_';
      return `match ${this.expr(e.scrutinee)} {\n${indent(`${pattern} => ${block(this.expr(kase.body))}`, 1)}\n}`;
    }
    let subject;
    const lines = [];
    if (e.scrutinee.k === 'var') subject = e.scrutinee.name;
    else {
      this.temporaries += 1;
      subject = `__subject${this.temporaries}`;
      lines.push(`let ${subject} = ${this.expr(e.scrutinee)};`);
    }
    const fallbackBody = (kase) => [
      ...(kase.pattern.k === 'bind' ? [`let ${kase.pattern.name} = ${subject}.clone();`] : []),
      this.expr(kase.body),
    ].join('\n');
    const zero = e.cases.find((kase) => kase.pattern.k === 'natZero');
    const succ = e.cases.find((kase) => kase.pattern.k === 'natSucc');
    const zeroText = zero ? this.expr(zero.body) : fallbackBody(fallback);
    const machine = e.scrutinee.type.kind === 'fixed';
    // On an unsigned machine integer the successor case holds a value of at least 1, so `- 1` cannot wrap.
    const predecessor = machine ? `${subject} - 1` : `crate::ml::pred(&${subject})`;
    const succText = succ
      ? `let ${succ.pattern.name} = ${predecessor};\n${this.expr(succ.body)}`
      : fallbackBody(fallback);
    lines.push(`if ${machine ? `${subject} == 0` : `${subject}.is_zero()`} {\n${indent(zeroText, 1)}\n} else {\n${indent(succText, 1)}\n}`);
    return lines.length === 1 ? lines[0] : `{\n${indent(lines.join('\n'), 1)}\n}`;
  }

  cast(e) {
    const arg = this.expr(e.arg);
    if (e.flavor === 'wrapping') {
      if (e.from.kind !== 'fixed' || e.to.kind !== 'fixed') throw unsupported('wrapping integer cast', 'requires actual fixed integer source and target types', e.span);
      const code = renderWrappingFixedIntegerCast('Rust', arg, e.to.bits, e.to.signed, e.from.signed);
      if (!code) throw unsupported('wrapping integer cast', 'the integer width needs an exact target encoding', e.span);
      this.state.encode('fixed integer casts', readFixedIntegerCastStatement());
      return code;
    }
    if (e.from.kind === 'fixed' && e.to.kind === 'fixed') return `${typeKey(e.to)}::from(${arg})`;
    if (e.flavor === 'exact') return arg;
    if (e.flavor === 'clamp') return `crate::ml::clamp_nat(${arg})`;
    return `crate::ml::to_nat_checked(${arg}, ${rustString(castMessage(e))})`;
  }

  main(main) {
    const { effects } = renameMain(main, snake, this.state.localReserved());
    const lines = [];
    let assertion = 0;
    for (const effect of effects) {
      if (effect.k === 'print') lines.push(`println!("{}", ${this.expr(effect.expr)});`);
      else if (effect.k === 'let') lines.push(`let ${effect.name} = ${this.expr(effect.value)};`);
      else {
        assertion += 1;
        lines.push(`assert!(${this.prop(effect.prop)}, "assertion ${assertion}");`);
        this.state.assertionTheorem(`assertion ${assertion}`, effect);
      }
    }
    if (main.sequentialAsync) this.state.encode('sequential-async', 'an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order');
    this.state.encode('program-output', 'main prints the lines the source program prints, in order, with println!');
    return `fn ml_main() {\n${indent(lines.join('\n'), 1)}\n}`;
  }
}

/** A Number's canonical JavaScript text as an f64 expression. */
function rustFloat(value) {
  const special = { NaN: 'f64::NAN', Infinity: 'f64::INFINITY', '-Infinity': 'f64::NEG_INFINITY' }[value];
  if (special) return special;
  return value.startsWith('-') ? `(${value}f64)` : `${value}f64`;
}

function block(text) {
  return `{\n${indent(text, 1)}\n}`;
}

/** Text inside a `format!`-style literal, where braces are doubled. */
function formatEscape(text) {
  return rustEscape(text).replace(/\{/gu, '{{').replace(/\}/gu, '}}');
}

function indent(text, depth) {
  const pad = '    '.repeat(depth);
  return text.split('\n').map((line) => (line ? `${pad}${line}` : line)).join('\n');
}
