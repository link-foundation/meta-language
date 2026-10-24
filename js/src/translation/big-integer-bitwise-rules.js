// Exact infinite two's-complement operations using only ordinary BigInt
// arithmetic. This complete source supplies native algorithms and target code.

/** @param {string} operation @returns {number} */
export function readBigIntegerBitwiseOperationCode(operation) {
  if (operation === 'conjoinBigIntegerBits') return 0;
  if (operation === 'disjoinBigIntegerBits') return 1;
  if (operation === 'excludeEqualBigIntegerBits') return 2;
  if (operation === 'shiftBigIntegerBitsLeft') return 3;
  if (operation === 'shiftBigIntegerBitsRight') return 4;
  if (operation === 'complementBigIntegerBits') return 6;
  return -1;
}
/** @param {string} operation @returns {string} */
export function readBigIntegerBitwiseOperation(operation) {
  if (operation === 'conjoinJavaScriptNumberBits') return 'conjoinBigIntegerBits';
  if (operation === 'disjoinJavaScriptNumberBits') return 'disjoinBigIntegerBits';
  if (operation === 'excludeEqualJavaScriptNumberBits') return 'excludeEqualBigIntegerBits';
  if (operation === 'shiftJavaScriptNumberBitsLeft') return 'shiftBigIntegerBitsLeft';
  if (operation === 'shiftJavaScriptNumberBitsRight') return 'shiftBigIntegerBitsRight';
  if (operation === 'complementJavaScriptNumberBits') return 'complementBigIntegerBits';
  return readBigIntegerBitwiseOperationCode(operation) < 0 ? '' : operation;
}
/** @param {string} operation @returns {number} */
export function readBigIntegerBitwiseArity(operation) {
  const code = readBigIntegerBitwiseOperationCode(operation);
  return code === 6 ? 1 : code < 0 ? 0 : 2;
}
/** @param {string} expression @param {string} operation @returns {boolean} */
export function acceptLinkedBigIntegerBitwiseOperation(expression, operation) {
  return expression === 'math' && readBigIntegerBitwiseArity(operation) !== 0;
}
/** @param {string[]} kinds @returns {string} */
export function readPrimitiveBitwiseDomain(kinds) {
  if (kinds.length === 0) return '';
  let number = true, integer = true;
  for (const kind of kinds) {
    number = number && kind === 'float';
    integer = integer && (kind === 'int' || kind === 'nat');
  }
  return number ? 'Number' : integer ? 'BigInt' : '';
}
/** @param {number} operation @param {bigint} left @param {bigint} right @returns {bigint} */
export function applyBigIntegerBitwiseOperation(operation, left, right) {
  if (operation === 6) return -1n - left;
  if (operation === 3 || operation === 4) {
    if (left === 0n) return 0n;
    let count = right, forward = operation === 3;
    if (count < 0n) { count = -count; forward = !forward; }
    let value = left;
    if (forward) {
      let factor = 2n;
      while (count > 0n) {
        if (count % 2n === 1n) value *= factor;
        count /= 2n;
        if (count > 0n) factor *= factor;
      }
    } else {
      while (count > 0n && value !== 0n && value !== -1n) {
        value = value < 0n ? (value - 1n) / 2n : value / 2n;
        count -= 1n;
      }
    }
    return value;
  }
  if (operation !== 0 && operation !== 1 && operation !== 2) throw new TypeError('an unknown BigInt bitwise operation');
  const negativeLeft = left < 0n, negativeRight = right < 0n;
  let first = negativeLeft ? -1n - left : left;
  let second = negativeRight ? -1n - right : right;
  let weight = 1n, result = 0n;
  while (first > 0n || second > 0n) {
    const a = negativeLeft ? 1n - first % 2n : first % 2n;
    const b = negativeRight ? 1n - second % 2n : second % 2n;
    const selected = operation === 0 ? a === 1n && b === 1n : operation === 1 ? a === 1n || b === 1n : a !== b;
    if (selected) result += weight;
    first /= 2n; second /= 2n; weight *= 2n;
  }
  const negative = operation === 0 ? negativeLeft && negativeRight : operation === 1 ? negativeLeft || negativeRight : negativeLeft !== negativeRight;
  return negative ? result - weight : result;
}
/** @param {string} target @param {string} operation @param {string[]} arguments_ @returns {string} */
export function renderBigIntegerBitwiseExpression(target, operation, arguments_) {
  const code = readBigIntegerBitwiseOperationCode(operation);
  if (code < 0 || arguments_.length !== readBigIntegerBitwiseArity(operation)) return '';
  const left = arguments_[0] ?? '', right = arguments_[1] ?? '';
  if (target === 'Rocq') {
    const operationName = code === 0 ? 'Z.land' : code === 1 ? 'Z.lor' : code === 2 ? 'Z.lxor'
      : code === 3 ? 'Z.shiftl' : code === 4 ? 'Z.shiftr' : 'Z.lnot';
    return '(' + operationName + ' ' + left + (code === 6 ? '' : ' ' + right) + ')';
  }
  if (target === 'Lean') {
    if (code === 6) return '(-1 - (' + left + '))';
    const text = left + ' ' + right;
    let suffix = 0;
    while (text.includes('ml_integer_left' + suffix) || text.includes('ml_integer_right' + suffix)) suffix += 1;
    const a = 'ml_integer_left' + suffix, b = 'ml_integer_right' + suffix;
    const am = '(Int.toNat (-1 - ' + a + '))', bm = '(Int.toNat (-1 - ' + b + '))';
    const ap = '(Int.toNat ' + a + ')', bp = '(Int.toNat ' + b + ')';
    const prefix = '(let ' + a + ' : Int := ' + left + '; let ' + b + ' : Int := ' + right + '; ';
    if (code === 3 || code === 4) {
      const count = '(Int.toNat (if ' + b + ' < 0 then -' + b + ' else ' + b + '))';
      const shiftLeft = '(' + a + ' * Int.ofNat (Nat.shiftLeft 1 ' + count + '))';
      const shiftRight = '(if ' + a + ' < 0 then -1 - Int.ofNat (Nat.shiftRight ' + am + ' ' + count
        + ') else Int.ofNat (Nat.shiftRight ' + ap + ' ' + count + '))';
      return prefix + 'if ' + a + ' = 0 then 0 else if ' + b + ' < 0 then '
        + (code === 3 ? shiftRight : shiftLeft) + ' else ' + (code === 3 ? shiftLeft : shiftRight) + ')';
    }
    const difference = '(Nat.bitwise (fun ml_bit_left ml_bit_right => ml_bit_left && !ml_bit_right) ';
    const both = code === 0 ? '(-1 - Int.ofNat (Nat.lor ' + am + ' ' + bm + '))'
      : code === 1 ? '(-1 - Int.ofNat (Nat.land ' + am + ' ' + bm + '))' : '(Int.ofNat (Nat.xor ' + am + ' ' + bm + '))';
    const leftNegative = code === 0 ? '(Int.ofNat ' + difference + bp + ' ' + am + '))'
      : code === 1 ? '(-1 - Int.ofNat ' + difference + am + ' ' + bp + '))' : '(-1 - Int.ofNat (Nat.xor ' + am + ' ' + bp + '))';
    const rightNegative = code === 0 ? '(Int.ofNat ' + difference + ap + ' ' + bm + '))'
      : code === 1 ? '(-1 - Int.ofNat ' + difference + bm + ' ' + ap + '))' : '(-1 - Int.ofNat (Nat.xor ' + ap + ' ' + bm + '))';
    const positive = '(Int.ofNat (' + (code === 0 ? 'Nat.land' : code === 1 ? 'Nat.lor' : 'Nat.xor') + ' ' + ap + ' ' + bp + '))';
    return prefix + 'if ' + a + ' < 0 then (if ' + b + ' < 0 then ' + both + ' else ' + leftNegative
      + ') else (if ' + b + ' < 0 then ' + rightNegative + ' else ' + positive + '))';
  }
  if (target === 'Rust') return 'crate::ml_big_integer_bitwise::apply_big_integer_bitwise_operation('
    + code + 'f64, ' + left + ', ' + (code === 6 ? 'ml::Big::zero()' : right) + ')';
  if (target !== 'JavaScript') return '';
  if (code === 6) return '(~(' + left + '))';
  const symbol = code === 0 ? '&' : code === 1 ? '|' : code === 2 ? '^' : code === 3 ? '<<' : '>>';
  return '((' + left + ') ' + symbol + ' (' + right + '))';
}
/** @returns {string} */
export function readBigIntegerBitwiseStatement() {
  return 'BigInt bitwise operations use exact unbounded integers with infinite sign extension, negative shift reversal and arithmetic right shifts; operands are evaluated once in source order without Number coercion or modulo-32 shift counts. Native source and the emitted integer runtime are generated from the same checked definition.';
}
