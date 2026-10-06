//! The JavaScript runtime helpers a translation may use.

/// Runtime helpers, declared in the order they are written to the file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Helper {
    NatSub,
    Fixed,
    Divide,
    ToNatChecked,
    Abort,
    Assert,
    Equal,
    ShowNumber,
    At,
    Forall,
    Domains,
}

impl Helper {
    pub(super) const fn text(self) -> &'static str {
        match self {
            Self::NatSub => {
                "function ml_natSub(a, b) {
  return a > b ? a - b : 0n;
}"
            }
            Self::Fixed => {
                "function ml_fixed(value, min, max, message) {
  if (value < min || value > max) throw new RangeError(message);
  return value;
}"
            }
            Self::Divide => {
                "// Integer division with the source's rounding; by zero it aborts with the message `zero`
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
}"
            }
            Self::ToNatChecked => {
                "function ml_toNatChecked(value, message) {
  if (value < 0n) throw new RangeError(message);
  return value;
}"
            }
            Self::Abort => {
                "function ml_abort(message) {
  throw new Error(message);
}"
            }
            Self::Assert => {
                "function ml_assert(holds, statement) {
  if (!holds) throw new Error(`assertion failed: ${statement}`);
}"
            }
            Self::Equal => {
                "function ml_equal(left, right) {
  if (typeof left !== 'object' || left === null) return left === right;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => ml_equal(left[key], right[key]));
}"
            }
            Self::ShowNumber => {
                "// console.log prints -0 as -0, where String(-0) is \"0\".
function ml_showNumber(value) {
  return Object.is(value, -0) ? '-0' : String(value);
}"
            }
            Self::At => {
                "// An element of an array; a read outside it, undefined in JavaScript, aborts.
function ml_at(values, index) {
  const at = Number(index);
  if (!Number.isInteger(at) || at < 0 || at >= values.length) throw new RangeError(`array index ${index} out of range`);
  return values[at];
}"
            }
            Self::Forall => {
                "function ml_forall(values, property) {
  return values.every(property);
}"
            }
            Self::Domains => {
                "// Bounded domains for executable theorem checks.
const ml_nat = [0n, 1n, 2n, 3n, 4n, 5n, 6n];
const ml_int = [-4n, -3n, -2n, -1n, 0n, 1n, 2n, 3n, 4n];
const ml_small_nat = [0n, 1n, 2n];
const ml_small_int = [-1n, 0n, 1n];
function ml_product(lists) {
  return lists.reduce((rows, list) => rows.flatMap((row) => list.map((value) => [...row, value])), [[]]);
}"
            }
        }
    }
}
