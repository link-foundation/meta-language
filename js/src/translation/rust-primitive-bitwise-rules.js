// Total primitive Rust bit operations. Shift overflow is a separate effect.

/** @param {string} symbol @returns {string} */
export function readRustPrimitiveBitwiseOperation(symbol) {
  if (symbol === '&') return 'conjoinRustPrimitiveBits';
  if (symbol === '|') return 'disjoinRustPrimitiveBits';
  if (symbol === '^') return 'excludeEqualRustPrimitiveBits';
  if (symbol === '!') return 'complementRustPrimitiveBits';
  return '';
}
/** @param {string} operation @returns {string} */
export function readRustPrimitiveBitwiseSymbol(operation) {
  if (operation === 'conjoinRustPrimitiveBits') return '&';
  if (operation === 'disjoinRustPrimitiveBits') return '|';
  if (operation === 'excludeEqualRustPrimitiveBits') return '^';
  if (operation === 'complementRustPrimitiveBits') return '!';
  return '';
}
/** @param {string} operation @returns {number} */
export function readRustPrimitiveBitwiseArity(operation) {
  const symbol = readRustPrimitiveBitwiseSymbol(operation);
  return symbol === '!' ? 1 : symbol === '' ? 0 : 2;
}
/** @param {string} expression @param {string} operation @returns {boolean} */
export function acceptLinkedRustPrimitiveBitwiseOperation(expression, operation) {
  return expression === 'math' && readRustPrimitiveBitwiseArity(operation) > 0;
}
/** @param {string} kind @param {number} bits @returns {boolean} */
export function acceptRustPrimitiveBitwiseType(kind, bits) {
  return kind === 'bool' || kind === 'fixed' && (bits === 8 || bits === 16 || bits === 32 || bits === 64 || bits === 128);
}
/** @param {string} symbol @param {number} level @returns {boolean} */
export function acceptRustPrimitiveBitwisePrecedence(symbol, level) {
  return level === 0 && symbol === '|' || level === 1 && symbol === '^' || level === 2 && symbol === '&';
}
/** @param {string} operation @returns {string} */
export function readRustPrimitiveBitwiseIntegerOperation(operation) {
  if (operation === 'conjoinRustPrimitiveBits') return 'conjoinBigIntegerBits';
  if (operation === 'disjoinRustPrimitiveBits') return 'disjoinBigIntegerBits';
  if (operation === 'excludeEqualRustPrimitiveBits') return 'excludeEqualBigIntegerBits';
  if (operation === 'complementRustPrimitiveBits') return 'complementBigIntegerBits';
  return '';
}
/** @param {string} target @param {boolean} signed @param {string} argument @returns {string} */
export function renderRustPrimitiveBitwiseIntegerArgument(target, signed, argument) {
  if (!signed && target === 'Lean') return '(Int.ofNat (' + argument + '))';
  if (!signed && target === 'Rocq') return '(Z.of_N (' + argument + '))';
  return argument;
}
/** The supplied integer expression comes from the common bitwise definition.
 * @param {string} target @param {string} operation @param {string} kind
 * @param {number} bits @param {boolean} signed @param {string[]} arguments_
 * @param {string} integer @param {string} modulus @returns {string} */
export function renderRustPrimitiveBitwiseExpression(target, operation, kind, bits, signed, arguments_, integer, modulus) {
  const symbol = readRustPrimitiveBitwiseSymbol(operation);
  if (!acceptRustPrimitiveBitwiseType(kind, bits) || arguments_.length !== readRustPrimitiveBitwiseArity(operation)) return '';
  const left = arguments_[0] ?? '', right = arguments_[1] ?? '';
  if (target === 'Rust') return symbol === '!' ? '(!(' + left + '))' : '((' + left + ') ' + symbol + ' (' + right + '))';
  if (kind === 'bool') {
    if (target === 'JavaScript') return symbol === '!' ? '(!(' + left + '))'
      : '((ml_bit_left, ml_bit_right) => ' + (symbol === '&' ? 'ml_bit_left && ml_bit_right' : symbol === '|' ? 'ml_bit_left || ml_bit_right' : 'ml_bit_left !== ml_bit_right') + ')(' + left + ', ' + right + ')';
    if (target === 'Rocq') return '(' + (symbol === '&' ? 'andb' : symbol === '|' ? 'orb' : symbol === '^' ? 'xorb' : 'negb')
      + ' ' + left + (symbol === '!' ? '' : ' ' + right) + ')';
    if (target === 'Lean') return symbol === '!' ? '(!(' + left + '))'
      : '((fun (ml_bit_left ml_bit_right : Bool) => ' + (symbol === '&' ? 'ml_bit_left && ml_bit_right' : symbol === '|' ? 'ml_bit_left || ml_bit_right' : 'ml_bit_left != ml_bit_right') + ') (' + left + ') (' + right + '))';
    return '';
  }
  if (target === 'JavaScript') return symbol === '!' ? 'BigInt.' + (signed ? 'asIntN' : 'asUintN') + '(' + bits + ', ~(' + left + '))'
    : '((' + left + ') ' + symbol + ' (' + right + '))';
  if (target === 'Lean') return signed ? integer : '(Int.toNat (Int.emod ' + integer + ' (' + modulus + ' : Int)))';
  if (target === 'Rocq') return signed ? integer : '(Z.to_N (Z.modulo ' + integer + ' (' + modulus + '%Z)))';
  return '';
}
