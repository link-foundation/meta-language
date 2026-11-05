// Shared fixed integer cast semantics. Constants are powers of two generated
// from each admitted width; source values retain their declared integer domain.

/** @param {number} bits @returns {boolean} */
export function acceptFixedIntegerCastWidth(bits) {
  return bits === 8 || bits === 16 || bits === 32 || bits === 64 || bits === 128;
}

/** @param {number} bits @returns {string} */
export function readFixedIntegerCastModulus(bits) {
  if (bits === 8) return '256';
  if (bits === 16) return '65536';
  if (bits === 32) return '4294967296';
  if (bits === 64) return '18446744073709551616';
  if (bits === 128) return '340282366920938463463374607431768211456';
  return '';
}

/** @param {number} bits @returns {string} */
export function readFixedIntegerCastSignedLimit(bits) {
  if (bits === 8) return '128';
  if (bits === 16) return '32768';
  if (bits === 32) return '2147483648';
  if (bits === 64) return '9223372036854775808';
  if (bits === 128) return '170141183460469231731687303715884105728';
  return '';
}

/** @param {string} language @param {boolean} widening @param {string} flavor @returns {string} */
export function readFixedIntegerCastFlavor(language, widening, flavor) {
  if (widening) return 'exact';
  return language === 'Rust' || flavor === 'wrapping' ? 'wrapping' : '';
}

/** @returns {string} */
export function readFixedIntegerCastStatement() {
  return 'Fixed width integer casts retain low bits at the target width and interpret its sign bit for signed targets, without integer rounding through binary64.';
}

/** Complete mathematical operation; no binary64 integer conversion.
 * @param {bigint} value @param {number} bits @param {boolean} signed @returns {bigint}
 */
export function wrapFixedIntegerBits(value, bits, signed) {
  if (!acceptFixedIntegerCastWidth(bits)) throw new RangeError('unsupported fixed integer cast width');
  let modulus = 1n;
  for (let bit = 0; bit < bits; bit += 1) modulus *= 2n;
  let residue = value % modulus;
  if (residue < 0n) residue += modulus;
  return signed && residue >= modulus / 2n ? residue - modulus : residue;
}

/** Postorder construction of the same operation using existing integer terms.
 * Each instruction occupies five cells: kind, value/operator, first, second,
 * third. References and let names use three fresh caller-owned binding slots.
 * The value operand occurs once, and the result remains an unbounded integer.
 * @param {number} bits @param {boolean} signed @returns {string[]}
 */
export function planFixedIntegerCastExpression(bits, signed) {
  const modulus = readFixedIntegerCastModulus(bits);
  if (modulus === '') return [];
  return [
    'input', '', '', '', '',
    'integer', modulus, '', '', '',
    'integer', '0', '', '', '',
    'reference', '0', '', '', '',
    'binary', 'rem', '3', '1', '',
    'reference', '1', '', '', '',
    'binary', 'lt', '5', '2', '',
    'binary', 'add', '5', '1', '',
    'conditional', '', '6', '7', '5',
    'reference', '2', '', '', '',
    'integer', readFixedIntegerCastSignedLimit(bits), '', '', '',
    'binary', 'ge', '9', '10', '',
    'binary', 'sub', '9', '1', '',
    'conditional', '', '11', '12', '9',
    'let', '2', '8', signed ? '13' : '9', '',
    'let', '1', '4', '14', '',
    'let', '0', '0', '15', '',
  ];
}

/** Render the same modular operation in each target's integer domain.
 * The actual argument appears once, preserving all its effects.
 * @param {string} target @param {string} argument @param {number} bits
 * @param {boolean} signed @param {boolean} sourceSigned @returns {string}
 */
export function renderWrappingFixedIntegerCast(target, argument, bits, signed, sourceSigned) {
  const modulus = readFixedIntegerCastModulus(bits), limit = readFixedIntegerCastSignedLimit(bits);
  if (modulus === '') return '';
  if (target === 'Rust') return '((' + argument + ') as ' + (signed ? 'i' : 'u') + bits + ')';
  if (target === 'JavaScript') return 'BigInt.' + (signed ? 'asIntN' : 'asUintN') + '(' + bits + ', ' + argument + ')';
  if (target === 'Lean') {
    const value = sourceSigned ? '(' + argument + ')' : '(Int.ofNat (' + argument + '))';
    const residue = '(Int.emod ' + value + ' (' + modulus + ' : Int))';
    return signed ? '(let ml_cast_value : Int := ' + residue + '; if ml_cast_value < (' + limit
      + ' : Int) then ml_cast_value else ml_cast_value - (' + modulus + ' : Int))' : '(Int.toNat ' + residue + ')';
  }
  if (target === 'Rocq') {
    const value = sourceSigned ? '(' + argument + ')' : '(Z.of_N (' + argument + '))';
    const residue = '(Z.modulo ' + value + ' (' + modulus + '%Z))';
    return signed ? '(let ml_cast_value : Z := ' + residue + ' in if Z.ltb ml_cast_value (' + limit
      + '%Z) then ml_cast_value else Z.sub ml_cast_value (' + modulus + '%Z))' : '(Z.to_N ' + residue + ')';
  }
  return '';
}

