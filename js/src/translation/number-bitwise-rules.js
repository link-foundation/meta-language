// Number-only operations. The checked source generates the native algorithms.

/** @param {number} value @returns {number} */
export function normalizeJavaScriptUnsignedInteger(value) {
  if (!Number.isFinite(value) || value === 0) return 0;
  const integral = value < 0 ? -Math.floor(-value) : Math.floor(value);
  const remainder = integral % 4294967296;
  if (remainder < 0) return remainder + 4294967296;
  // A negative exact multiple produces -0 under %, but ToUint32 produces +0.
  if (remainder === 0) return 0;
  return remainder;
}

/** @param {number} value @returns {number} */
export function normalizeJavaScriptSignedInteger(value) {
  const unsigned = normalizeJavaScriptUnsignedInteger(value);
  return unsigned >= 2147483648 ? unsigned - 4294967296 : unsigned;
}

/** @param {string} operation @returns {number} */
export function readJavaScriptNumberBitwiseOperationCode(operation) {
  if (operation === 'conjoinJavaScriptNumberBits') return 0;
  if (operation === 'disjoinJavaScriptNumberBits') return 1;
  if (operation === 'excludeEqualJavaScriptNumberBits') return 2;
  if (operation === 'shiftJavaScriptNumberBitsLeft') return 3;
  if (operation === 'shiftJavaScriptNumberBitsRight') return 4;
  if (operation === 'shiftJavaScriptNumberBitsRightUnsigned') return 5;
  if (operation === 'complementJavaScriptNumberBits') return 6;
  return -1;
}

/** @param {string} symbol @returns {string} */
export function readJavaScriptNumberBitwiseOperation(symbol) {
  if (symbol === '&') return 'conjoinJavaScriptNumberBits';
  if (symbol === '|') return 'disjoinJavaScriptNumberBits';
  if (symbol === '^') return 'excludeEqualJavaScriptNumberBits';
  if (symbol === '<<') return 'shiftJavaScriptNumberBitsLeft';
  if (symbol === '>>') return 'shiftJavaScriptNumberBitsRight';
  if (symbol === '>>>') return 'shiftJavaScriptNumberBitsRightUnsigned';
  if (symbol === '~') return 'complementJavaScriptNumberBits';
  return '';
}

/** @param {string} operation @returns {string} */
export function readJavaScriptNumberBitwiseSymbol(operation) {
  if (operation === 'conjoinJavaScriptNumberBits') return '&';
  if (operation === 'disjoinJavaScriptNumberBits') return '|';
  if (operation === 'excludeEqualJavaScriptNumberBits') return '^';
  if (operation === 'shiftJavaScriptNumberBitsLeft') return '<<';
  if (operation === 'shiftJavaScriptNumberBitsRight') return '>>';
  if (operation === 'shiftJavaScriptNumberBitsRightUnsigned') return '>>>';
  if (operation === 'complementJavaScriptNumberBits') return '~';
  return '';
}

/** @param {string} operation @returns {number} */
export function readJavaScriptNumberBitwiseArity(operation) {
  const code = readJavaScriptNumberBitwiseOperationCode(operation);
  if (code === 6) return 1;
  if (code >= 0) return 2;
  return 0;
}

/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedNumberBitwiseOperation(kind, operation) {
  return kind === 'math' && readJavaScriptNumberBitwiseArity(operation) > 0;
}

/** @param {string} operation @param {number} count @returns {boolean} */
export function acceptJavaScriptNumberBitwiseOperandCount(operation, count) {
  const arity = readJavaScriptNumberBitwiseArity(operation);
  return arity > 0 && count === arity;
}

/** @param {string} symbol @param {number} level @returns {boolean} */
export function acceptJavaScriptNumberBitwisePrecedence(symbol, level) {
  return level === 0 && symbol === '|'
    || level === 1 && symbol === '^'
    || level === 2 && symbol === '&';
}

/** @param {number} operation @param {number} left @param {number} right @returns {number} */
export function applyJavaScriptNumberBitwiseOperation(operation, left, right) {
  if (operation === 6) return -1 - normalizeJavaScriptSignedInteger(left);
  if (operation === 3 || operation === 4 || operation === 5) {
    const count = normalizeJavaScriptUnsignedInteger(right) % 32;
    let factor = 1;
    for (let position = 0; position < count; position += 1) factor *= 2;
    if (operation === 3) return normalizeJavaScriptSignedInteger(normalizeJavaScriptUnsignedInteger(left) * factor);
    if (operation === 4) return Math.floor(normalizeJavaScriptSignedInteger(left) / factor);
    return Math.floor(normalizeJavaScriptUnsignedInteger(left) / factor);
  }
  if (!(operation === 0 || operation === 1 || operation === 2)) return NaN;
  let first = normalizeJavaScriptUnsignedInteger(left);
  let second = normalizeJavaScriptUnsignedInteger(right);
  let result = 0;
  let weight = 1;
  for (let position = 0; position < 32; position += 1) {
    const firstBit = first % 2;
    const secondBit = second % 2;
    if (operation === 0 && firstBit === 1 && secondBit === 1
      || operation === 1 && (firstBit === 1 || secondBit === 1)
      || operation === 2 && firstBit !== secondBit) result += weight;
    first = (first - firstBit) / 2;
    second = (second - secondBit) / 2;
    weight *= 2;
  }
  return normalizeJavaScriptSignedInteger(result);
}

/** @param {number} left @param {number} right @returns {number} */
export function conjoinJavaScriptNumberBits(left, right) { return applyJavaScriptNumberBitwiseOperation(0, left, right); }
/** @param {number} left @param {number} right @returns {number} */
export function disjoinJavaScriptNumberBits(left, right) { return applyJavaScriptNumberBitwiseOperation(1, left, right); }
/** @param {number} left @param {number} right @returns {number} */
export function excludeEqualJavaScriptNumberBits(left, right) { return applyJavaScriptNumberBitwiseOperation(2, left, right); }
/** @param {number} left @param {number} right @returns {number} */
export function shiftJavaScriptNumberBitsLeft(left, right) { return applyJavaScriptNumberBitwiseOperation(3, left, right); }
/** @param {number} left @param {number} right @returns {number} */
export function shiftJavaScriptNumberBitsRight(left, right) { return applyJavaScriptNumberBitwiseOperation(4, left, right); }
/** @param {number} left @param {number} right @returns {number} */
export function shiftJavaScriptNumberBitsRightUnsigned(left, right) { return applyJavaScriptNumberBitwiseOperation(5, left, right); }
/** @param {number} left @returns {number} */
export function complementJavaScriptNumberBits(left) { return applyJavaScriptNumberBitwiseOperation(6, left, 0); }
