// Generated from complete js/src/translation/fixed-integer-cast-rules.js. Do not edit by hand.
/// Unbounded integers for the portable core's naturals and integers.
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
        a.len()
            .cmp(&b.len())
            .then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0))
                + u64::from(*b.get(index).unwrap_or(&0))
                + carry;
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
            for limb in &mut remainder {
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
            Self {
                negative: negative && !magnitude.is_empty(),
                magnitude,
            }
        }

        #[must_use]
        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        #[must_use]
        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        #[must_use]
        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        #[must_use]
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = text
                .strip_prefix('-')
                .map_or((false, text), |rest| (true, rest));
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        #[must_use]
        pub const fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        #[must_use]
        pub const fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        #[must_use]
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self
                .magnitude
                .iter()
                .rev()
                .fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        #[must_use]
        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        #[must_use]
        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(
                    self.negative,
                    add_magnitude(&self.magnitude, &other.magnitude),
                );
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(
                    other.negative,
                    sub_magnitude(&other.magnitude, &self.magnitude),
                ),
                _ => Self::from_parts(
                    self.negative,
                    sub_magnitude(&self.magnitude, &other.magnitude),
                ),
            }
        }

        #[must_use]
        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        #[must_use]
        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(
                self.negative != other.negative,
                mul_magnitude(&self.magnitude, &other.magnitude),
            )
        }

        /// Division rounding toward zero; the divisor is not zero.
        #[must_use]
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
                std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}"))
                    .expect("writing to a String cannot fail");
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    #[must_use]
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    #[must_use]
    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
    #[must_use]
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
                quotient = if b.is_negative() {
                    quotient.add(&one)
                } else {
                    quotient.sub(&one)
                };
            }
        }
        if remainder {
            a.sub(&quotient.mul(b))
        } else {
            quotient
        }
    }

    #[must_use]
    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        assert!(!value.is_negative(), "{message}");
        value
    }

    #[must_use]
    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() {
            Big::zero()
        } else {
            value
        }
    }

    /// Bounded domains for executable theorem checks.
    #[must_use]
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_fixed_integer_cast_width(bits: f64) -> bool {
    ((((bits == 8f64) || (bits == 16f64)) || (bits == 32f64)) || (bits == 64f64))
        || (bits == 128f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_modulus(bits: f64) -> String {
    if bits == 8f64 {
        String::from("256")
    } else if bits == 16f64 {
        String::from("65536")
    } else if bits == 32f64 {
        String::from("4294967296")
    } else if bits == 64f64 {
        String::from("18446744073709551616")
    } else if bits == 128f64 {
        String::from("340282366920938463463374607431768211456")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_signed_limit(bits: f64) -> String {
    if bits == 8f64 {
        String::from("128")
    } else if bits == 16f64 {
        String::from("32768")
    } else if bits == 32f64 {
        String::from("2147483648")
    } else if bits == 64f64 {
        String::from("9223372036854775808")
    } else if bits == 128f64 {
        String::from("170141183460469231731687303715884105728")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_flavor(language: &str, widening: bool, flavor: &str) -> String {
    if widening {
        String::from("exact")
    } else if (language == "Rust") || (flavor == "wrapping") {
        String::from("wrapping")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_statement() -> String {
    String::from(
        "Fixed width integer casts retain low bits at the target width and interpret its sign bit for signed targets, without integer rounding through binary64.",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn wrap_fixed_integer_bits(value: self::ml::Big, bits: f64, signed: bool) -> self::ml::Big {
    if (self::accept_fixed_integer_cast_width(bits)) {
        {
            let modulus = self::ml::Big::from_i128(1);
            {
                let bit = 0f64;
                {
                    let modulus_2 = self::ml_wrap_fixed_integer_bits_loop1(bits, modulus, bit);
                    {
                        let residue = self::ml::divide(
                            &value,
                            &modulus_2,
                            "trunc",
                            Some("Division by zero"),
                            true,
                        );
                        {
                            let residue_3 = if residue < (self::ml::Big::from_i128(0)) {
                                { residue.add(&modulus_2) }
                            } else {
                                residue.clone()
                            };
                            if signed
                                && (residue_3
                                    >= (self::ml::divide(
                                        &modulus_2,
                                        &self::ml::Big::from_i128(2),
                                        "trunc",
                                        Some("Division by zero"),
                                        false,
                                    )))
                            {
                                residue_3.sub(&modulus_2)
                            } else {
                                residue_3.clone()
                            }
                        }
                    }
                }
            }
        }
    } else {
        panic!("{}", "unsupported fixed integer cast width")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_fixed_integer_cast_expression(bits: f64, signed: bool) -> Vec<String> {
    {
        let modulus = self::read_fixed_integer_cast_modulus(bits);
        if (modulus).is_empty() {
            Vec::<String>::new()
        } else {
            vec![
                String::from("input"),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::from("integer"),
                modulus,
                String::new(),
                String::new(),
                String::new(),
                String::from("integer"),
                String::from("0"),
                String::new(),
                String::new(),
                String::new(),
                String::from("reference"),
                String::from("0"),
                String::new(),
                String::new(),
                String::new(),
                String::from("binary"),
                String::from("rem"),
                String::from("3"),
                String::from("1"),
                String::new(),
                String::from("reference"),
                String::from("1"),
                String::new(),
                String::new(),
                String::new(),
                String::from("binary"),
                String::from("lt"),
                String::from("5"),
                String::from("2"),
                String::new(),
                String::from("binary"),
                String::from("add"),
                String::from("5"),
                String::from("1"),
                String::new(),
                String::from("conditional"),
                String::new(),
                String::from("6"),
                String::from("7"),
                String::from("5"),
                String::from("reference"),
                String::from("2"),
                String::new(),
                String::new(),
                String::new(),
                String::from("integer"),
                self::read_fixed_integer_cast_signed_limit(bits),
                String::new(),
                String::new(),
                String::new(),
                String::from("binary"),
                String::from("ge"),
                String::from("9"),
                String::from("10"),
                String::new(),
                String::from("binary"),
                String::from("sub"),
                String::from("9"),
                String::from("1"),
                String::new(),
                String::from("conditional"),
                String::new(),
                String::from("11"),
                String::from("12"),
                String::from("9"),
                String::from("let"),
                String::from("2"),
                String::from("8"),
                if signed {
                    String::from("13")
                } else {
                    String::from("9")
                },
                String::new(),
                String::from("let"),
                String::from("1"),
                String::from("4"),
                String::from("14"),
                String::new(),
                String::from("let"),
                String::from("0"),
                String::from("0"),
                String::from("15"),
                String::new(),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_wrapping_fixed_integer_cast(
    target: &str,
    argument: &str,
    bits: f64,
    signed: bool,
    source_signed: bool,
) -> String {
    {
        let modulus = self::read_fixed_integer_cast_modulus(bits);
        {
            let limit = self::read_fixed_integer_cast_signed_limit(bits);
            if (modulus).is_empty() {
                String::new()
            } else if target == "Rust" {
                format!(
                    "(({}) as {}{})",
                    argument.to_owned(),
                    if signed {
                        String::from("i")
                    } else {
                        String::from("u")
                    },
                    self::ml_number::js_number(bits)
                )
            } else if target == "JavaScript" {
                format!(
                    "BigInt.{}({}, {})",
                    if signed {
                        String::from("asIntN")
                    } else {
                        String::from("asUintN")
                    },
                    self::ml_number::js_number(bits),
                    argument.to_owned()
                )
            } else if target == "Lean" {
                {
                    let value = if source_signed {
                        format!("({})", argument.to_owned())
                    } else {
                        format!("(Int.ofNat ({}))", argument.to_owned())
                    };
                    {
                        let residue = format!("(Int.emod {value} ({modulus} : Int))");
                        if signed {
                            format!(
                                "(let ml_cast_value : Int := {residue}; if ml_cast_value < ({limit} : Int) then ml_cast_value else ml_cast_value - ({modulus} : Int))"
                            )
                        } else {
                            format!("(Int.toNat {residue})")
                        }
                    }
                }
            } else if target == "Rocq" {
                {
                    let value_2 = if source_signed {
                        format!("({})", argument.to_owned())
                    } else {
                        format!("(Z.of_N ({}))", argument.to_owned())
                    };
                    {
                        let residue_2 = format!("(Z.modulo {value_2} ({modulus}%Z))");
                        if signed {
                            format!(
                                "(let ml_cast_value : Z := {residue_2} in if Z.ltb ml_cast_value ({limit}%Z) then ml_cast_value else Z.sub ml_cast_value ({modulus}%Z))"
                            )
                        } else {
                            format!("(Z.to_N {residue_2})")
                        }
                    }
                }
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_wrap_fixed_integer_bits_loop1(
    mut bits: f64,
    mut modulus: self::ml::Big,
    mut bit: f64,
) -> self::ml::Big {
    loop {
        if bit < bits {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let bit_2 = bit + 1f64;
                    {
                        (modulus, bit) = (modulus_2, bit_2);
                        continue;
                    }
                }
            }
        }
        return { modulus.clone() };
    }
}
