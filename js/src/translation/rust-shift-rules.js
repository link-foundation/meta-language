// Rust's overflow-checked primitive shift profile. The total checked result is
// distinct from the source operator's explicit overflow-abort continuation.

/** @param {string} symbol @returns {string} */
export function readRustShiftSourceOperation(symbol) {
  return symbol === '<<' ? 'shiftRustPrimitiveBitsLeft' : symbol === '>>' ? 'shiftRustPrimitiveBitsRight' : '';
}
/** @param {string} operation @returns {string} */
export function readRustShiftCheckedOperation(operation) {
  if (operation === 'shiftRustPrimitiveBitsLeft' || operation === 'readCheckedRustLeftShift') return 'readCheckedRustLeftShift';
  if (operation === 'shiftRustPrimitiveBitsRight' || operation === 'readCheckedRustRightShift') return 'readCheckedRustRightShift';
  return '';
}
/** @param {string} operation @returns {boolean} */
export function acceptRustShiftSourceOperation(operation) {
  return operation === 'shiftRustPrimitiveBitsLeft' || operation === 'shiftRustPrimitiveBitsRight';
}
/** @param {string} expression @param {string} operation @returns {boolean} */
export function acceptLinkedRustCheckedShift(expression, operation) {
  return expression === 'math' && (operation === 'readCheckedRustLeftShift' || operation === 'readCheckedRustRightShift');
}
/** @param {string} operation @returns {string} */
export function readRustShiftOverflowMessage(operation) {
  return readRustShiftCheckedOperation(operation) === 'readCheckedRustLeftShift' ? 'attempt to shift left with overflow' : 'attempt to shift right with overflow';
}
/** @param {number} bits @returns {bigint} */
export function readRustShiftWidthBound(bits) {
  if (bits === 8) return 8n;
  if (bits === 16) return 16n;
  if (bits === 32) return 32n;
  if (bits === 64) return 64n;
  if (bits === 128) return 128n;
  return 0n;
}
/** @param {number} bits @returns {bigint} */
export function readRustShiftModulus(bits) {
  if (bits === 8) return 256n;
  if (bits === 16) return 65536n;
  if (bits === 32) return 4294967296n;
  if (bits === 64) return 18446744073709551616n;
  if (bits === 128) return 340282366920938463463374607431768211456n;
  return 0n;
}
/** Complete mathematical checked operation, with no Number conversion.
 * @param {bigint} left @param {bigint} count @param {number} bits
 * @param {boolean} signed @param {boolean} forward @returns {bigint|undefined} */
export function readCheckedRustShift(left, count, bits, signed, forward) {
  const bound = readRustShiftWidthBound(bits);
  if (count < 0n || count >= bound) return undefined;
  const value = forward ? left << count : left >> count;
  const modulus = readRustShiftModulus(bits);
  const remainder = value % modulus;
  const unsigned = remainder < 0n ? remainder + modulus : remainder;
  return signed && unsigned >= modulus / 2n ? unsigned - modulus : unsigned;
}
/** @param {string[]} arguments_ @returns {string} */
export function readRustShiftTemporarySuffix(arguments_) {
  const text = arguments_.join(' ');
  let suffix = 0;
  while (text.includes('ml_shift_left' + suffix) || text.includes('ml_shift_count' + suffix)) suffix += 1;
  return '' + suffix;
}
/** @param {string} target @param {string} argument @param {boolean} signed @returns {string} */
export function renderRustShiftIntegerArgument(target, argument, signed) {
  if (!signed && target === 'Lean') return '(Int.ofNat (' + argument + '))';
  if (!signed && target === 'Rocq') return '(Z.of_N (' + argument + '))';
  return argument;
}
/** The surrounding ordinary coalescing node carries the source abort.
 * @param {string} target @param {string} operation @param {string[]} arguments_
 * @param {number} bits @param {boolean} signed @param {boolean} countSigned
 * @param {string} modulus @returns {string} */
export function renderRustCheckedShift(target, operation, arguments_, bits, signed, countSigned, modulus) {
  if (!acceptLinkedRustCheckedShift('math', operation) || arguments_.length !== 2) return '';
  const suffix = readRustShiftTemporarySuffix(arguments_);
  const left = 'ml_shift_left' + suffix, count = 'ml_shift_count' + suffix;
  const first = arguments_[0] ?? '', second = arguments_[1] ?? '';
  const forward = operation === 'readCheckedRustLeftShift';
  if (target === 'Rust') return '{ let ' + left + ' = ' + first + '; let ' + count + ' = ' + second + '; '
    + 'if ' + (countSigned ? count + ' < 0 || ' : '') + '(' + count + ' as u128) >= ' + bits + 'u128 { None } else { Some('
    + left + '.' + (forward ? 'wrapping_shl' : 'wrapping_shr') + '(' + count + ' as u32)) } }';
  if (target === 'JavaScript') return '((' + left + ', ' + count + ') => ' + count + ' < 0n || ' + count + ' >= ' + bits
    + 'n ? undefined : BigInt.' + (signed ? 'asIntN' : 'asUintN') + '(' + bits + ', ' + left + (forward ? ' << ' : ' >> ') + count + '))(' + first + ', ' + second + ')';
  if (target === 'Lean') {
    const divisor = '(Int.ofNat (Nat.shiftLeft 1 (Int.toNat ' + count + ')))';
    const value = forward ? '(' + left + ' * ' + divisor + ')' : '(Int.ediv ' + left + ' ' + divisor + ')';
    const projected = signed ? '(if ' + value + ' % (' + modulus + ' : Int) < (' + modulus + ' : Int) / 2 then '
      + value + ' % (' + modulus + ' : Int) else ' + value + ' % (' + modulus + ' : Int) - (' + modulus + ' : Int))'
      : '(Int.toNat (Int.emod ' + value + ' (' + modulus + ' : Int)))';
    return '(let ' + left + ' : Int := ' + first + '; let ' + count + ' : Int := ' + second + '; if ' + count + ' < 0 then none else if '
      + count + ' >= (' + bits + ' : Int) then none else some ' + projected + ')';
  }
  if (target === 'Rocq') {
    const value = '(' + (forward ? 'Z.shiftl' : 'Z.shiftr') + ' ' + left + ' ' + count + ')';
    const residue = '(Z.modulo ' + value + ' (' + modulus + '%Z))';
    const projected = signed ? '(if Z.ltb ' + residue + ' (Z.div (' + modulus + '%Z) 2) then ' + residue + ' else Z.sub ' + residue + ' (' + modulus + '%Z))' : '(Z.to_N ' + residue + ')';
    return '(let ' + left + ' : Z := ' + first + ' in let ' + count + ' : Z := ' + second + ' in if orb (Z.ltb ' + count + ' 0) (Z.leb '
      + bits + ' ' + count + ') then None else Some ' + projected + ')';
  }
  return '';
}
