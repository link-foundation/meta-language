// meta-language:self-translation:v1 source=Rust target=JavaScript sha256=da0c0d5d9c4becb536fa0500205f3f1b72a3fd5bd064ac3b3a47ff26f3794c2a bytes=410

// meta-language:prelude begin
function ml_fixed(value, min, max, message) {
  if (value < min || value > max) throw new RangeError(message);
  return value;
}

// Integer division with the source's rounding; by zero it aborts with the message `zero`
// or, when that is null, is total (x / 0 = 0, x % 0 = x); a machine-integer quotient
// out of [min, max] aborts with `overflow`, the remainder too.
function ml_divide(a, b, rounding, zero, remainder, bounds) {
  if (b === 0n) {
    if (zero !== null) throw new RangeError(zero);
    return remainder ? a : 0n;
  }
  let q = a / b;
  const r = a - q * b;
  if (r !== 0n) {
    if (rounding === 'floor' && (r < 0n) !== (b < 0n)) q -= 1n;
    if (rounding === 'euclid' && r < 0n) q = b > 0n ? q - 1n : q + 1n;
  }
  if (bounds && (q < bounds[0] || q > bounds[1])) throw new RangeError(bounds[2]);
  return remainder ? a - q * b : q;
}
// meta-language:prelude end

// meta-language:carried Rust use_declaration (unsupported)
// | //! Geometry the portable core translates, beside items it carries.
// | use std::fmt;

// meta-language:translated Rust function_item items=1 sha256=4e67b151b68acd3b23bc900e731787b4c19d7d20dce533d7872bc1933abc5cd4
// | // The squared distance of a point from the origin.
// | pub fn norm2(x: i64, y: i64) -> i64 {
// |     x * x + y * y
// | }
export function norm2(x, y) {
  ml_fixed(x, -9223372036854775808n, 9223372036854775807n, "i64 argument x out of range");
  ml_fixed(y, -9223372036854775808n, 9223372036854775807n, "i64 argument y out of range");
  return ml_fixed((ml_fixed((x * x), -9223372036854775808n, 9223372036854775807n, "attempt to multiply with overflow") + ml_fixed((y * y), -9223372036854775808n, 9223372036854775807n, "attempt to multiply with overflow")), -9223372036854775808n, 9223372036854775807n, "attempt to add with overflow");
}

// meta-language:translated Rust function_item items=1 sha256=d9e422326c9d675f968dee7d90cd2a048836f1b53f62a3bef1ed005b36743740
// | fn is_even(n: u32) -> bool {
// |     n % 2 == 0
// | }
function is_even(n) {
  ml_fixed(n, 0n, 4294967295n, "u32 argument n out of range");
  return (ml_divide(n, 2n, 'trunc', "attempt to calculate the remainder with a divisor of zero", true, [0n, 4294967295n, "attempt to calculate the remainder with overflow"]) === 0n);
}

// meta-language:carried Rust struct_item (syntax)
// | pub struct Point {
// |     x: i32,
// | }

// meta-language:carried Rust impl_item (unsupported)
// | impl fmt::Display for Point {
// |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// |         write!(f, "({})", self.x)
// |     }
// | }
