// Shared scalar identifier and canonical normalization algorithms. Unicode
// property/decomposition tables are source-derived Links data, never host ICU.

/** @param {number} value @returns {boolean} */
export function acceptUnicodeScalar(value) {
  return Number.isSafeInteger(value) && value >= 0 && value <= 1114111
    && !(value >= 55296 && value <= 57343);
}
/** @param {number} first @param {number} second @returns {number} */
export function decodeUnicodeSourceScalar(first, second) {
  if (first >= 55296 && first <= 56319 && second >= 56320 && second <= 57343)
    return 65536 + (first - 55296) * 1024 + second - 56320;
  return acceptUnicodeScalar(first) && first <= 65535 ? first : -1;
}
/** @param {number} scalar @returns {number} */
export function readUnicodeSourceScalarWidth(scalar) { return scalar > 65535 ? 2 : 1; }
/** @param {number[]} ranges @param {number} scalar @returns {boolean} */
export function acceptUnicodePropertyScalar(ranges, scalar) {
  if (!acceptUnicodeScalar(scalar)) return false;
  let low = 0;
  let high = Math.floor(ranges.length / 2);
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const first = ranges[middle * 2] ?? -1;
    const last = ranges[middle * 2 + 1] ?? -1;
    if (scalar < first) high = middle;
    else if (scalar > last) low = middle + 1;
    else return true;
  }
  return false;
}
/** @param {string} language @param {boolean} first @param {number} scalar @param {number[]} starts @param {number[]} continuations @returns {boolean} */
export function acceptSourceIdentifierScalar(language, first, scalar, starts, continuations) {
  if (!acceptUnicodeScalar(scalar)) return false;
  if (language !== 'JavaScript' && language !== 'Rust') return false;
  if (scalar === 95 || language === 'JavaScript' && scalar === 36) return true;
  if (scalar >= 65 && scalar <= 90 || scalar >= 97 && scalar <= 122) return true;
  if (!first && scalar >= 48 && scalar <= 57) return true;
  if (scalar < 128) return false;
  if (language === 'JavaScript' && !first && (scalar === 8204 || scalar === 8205)) return true;
  return acceptUnicodePropertyScalar(first ? starts : continuations, scalar);
}
/** @param {string} name @param {boolean} admitted @returns {string} */
export function readRustSourceRawIdentifierError(name, admitted) {
  return !admitted || name === '_' || name === 'crate' || name === 'self' || name === 'Self' || name === 'super'
    ? 'invalid Rust raw identifier r#' + name : '';
}
/** @param {number[]} rows @param {number} stride @param {number} key @returns {number} */
export function findUnicodeTableRow(rows, stride, key) {
  let low = 0;
  let high = Math.floor(rows.length / stride);
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const value = rows[middle * stride] ?? -1;
    if (key < value) high = middle;
    else if (key > value) low = middle + 1;
    else return middle * stride;
  }
  return -1;
}
/** @param {number[]} classes @param {number} scalar @returns {number} */
export function readUnicodeCombiningClass(classes, scalar) {
  const row = findUnicodeTableRow(classes, 2, scalar);
  return row < 0 ? 0 : classes[row + 1] ?? 0;
}
/** @param {number[]} pairs @param {number} first @param {number} second @returns {number} */
export function composeUnicodeScalars(pairs, first, second) {
  // Unicode 17.0 UAX #15, algorithmic Hangul composition; no syllable table.
  if (first >= 4352 && first < 4371 && second >= 4449 && second < 4470)
    return 44032 + ((first - 4352) * 21 + second - 4449) * 28;
  if (first >= 44032 && first < 55204 && (first - 44032) % 28 === 0
      && second > 4519 && second < 4547) return first + second - 4519;
  const row = findUnicodeTableRow(pairs, 2, first * 1114112 + second);
  return row < 0 ? -1 : pairs[row + 1] ?? -1;
}
/** @param {number[]} input @param {number[]} classes @param {number[]} decompositions @param {number[]} values @param {number[]} pairs @param {number} limit @returns {number[]} */
export function normalizeUnicodeScalars(input, classes, decompositions, values, pairs, limit) {
  if (!Number.isSafeInteger(limit) || limit < 1 || input.length > limit)
    throw new Error('Unicode normalization resource limit exceeded');
  const stack = [];
  for (let index = input.length; index > 0; index -= 1) stack.push(input[index - 1] ?? -1);
  const ordered = [];
  let steps = 0;
  while (stack.length > 0) {
    steps += 1;
    if (steps > limit) throw new Error('Unicode normalization resource limit exceeded');
    const scalar = stack.pop() ?? -1;
    if (!acceptUnicodeScalar(scalar)) throw new Error('Unicode normalization requires scalar values');
    if (scalar >= 44032 && scalar < 55204) {
      const syllable = scalar - 44032;
      const tail = syllable % 28;
      if (tail !== 0) stack.push(4519 + tail);
      stack.push(4449 + Math.floor(syllable / 28) % 21);
      stack.push(4352 + Math.floor(syllable / 588));
    } else {
      const row = findUnicodeTableRow(decompositions, 3, scalar);
      if (row >= 0) {
        const offset = decompositions[row + 1] ?? -1;
        const count = decompositions[row + 2] ?? -1;
        if (offset < 0 || count < 1 || offset + count > values.length || stack.length + count > limit)
          throw new Error('invalid Unicode canonical decomposition table');
        for (let index = count; index > 0; index -= 1) stack.push(values[offset + index - 1] ?? -1);
      } else {
        const combining = readUnicodeCombiningClass(classes, scalar);
        let position = ordered.length;
        ordered.push(scalar);
        // Stable canonical ordering stops at every starter (class zero).
        while (combining > 0 && position > 0) {
          steps += 1;
          if (steps > limit) throw new Error('Unicode normalization resource limit exceeded');
          const previous = ordered[position - 1] ?? -1;
          if (readUnicodeCombiningClass(classes, previous) <= combining) break;
          if (position < ordered.length) ordered.splice(position, 1, previous);
          position -= 1;
        }
        if (position < ordered.length) ordered.splice(position, 1, scalar);
      }
    }
  }
  const result = [];
  let starter = -1;
  let previousClass = 0;
  for (let index = 0; index < ordered.length; index += 1) {
    const scalar = ordered[index] ?? -1;
    const combining = readUnicodeCombiningClass(classes, scalar);
    const composition = starter < 0 ? -1 : composeUnicodeScalars(pairs, result[starter] ?? -1, scalar);
    if (composition >= 0 && (previousClass === 0 || previousClass < combining)) {
      if (starter >= 0 && starter < result.length) result.splice(starter, 1, composition);
    } else {
      if (combining === 0) starter = result.length;
      result.push(scalar);
      previousClass = combining;
    }
  }
  return result;
}
