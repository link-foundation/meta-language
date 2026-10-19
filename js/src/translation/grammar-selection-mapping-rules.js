// Portable source-mapping decisions. Native code is generated from this module
// and the actual imported Unicode decoder; it is never a second decoder port.
import { decodeUnicodeEscape } from './frontend-rules.js';

/** Existing identity selections keep their matching context. The new mapping
 * needs a syntactic physical CST and one common source plane per selection.
 * @param {string[]} mappings @param {boolean} inToken @returns {number}
 */
export function readGrammarSelectionMappingContext(mappings, inToken) {
  let mapped = false;
  for (const mapping of mappings) if (mapping === 'json-string') mapped = true;
  if (!mapped) return 0;
  if (inToken) return 2;
  for (const mapping of mappings) if (mapping !== 'json-string') return 2;
  return 1;
}

/** @param {number} code @returns {number} */
export function readGrammarSelectionJsonShortEscape(code) {
  if (code === 34 || code === 47 || code === 92) return code;
  if (code === 98) return 8;
  if (code === 102) return 12;
  if (code === 110) return 10;
  if (code === 114) return 13;
  if (code === 116) return 9;
  return -1;
}

/** One fixed-size plan over actual Unicode scalars of a complete quoted string.
 * [0, scalar, next] emits a scalar; [3, next] ends at the closing quote.
 * [1, position] is malformed JSON; [2, position] needs non-scalar text support.
 * Returning one part avoids repeated whole-array copies in JavaScript.
 * @param {number[]} codes @param {number} index @returns {number[]}
 */
export function readGrammarSelectionJsonStringPart(codes, index) {
  const last = codes.length - 1;
  if (codes.length < 2 || codes[0] !== 34 || codes[last] !== 34
      || !Number.isInteger(index) || index < 1 || index > last) return [1, 0];
  if (index === last) return [3, index];
  const code = codes[index] ?? -1;
  if (!Number.isInteger(code) || code < 32 || code > 1114111
      || code >= 55296 && code <= 57343 || code === 34) return [1, index];
  if (code !== 92) return [0, code, index + 1];
  const escaped = codes[index + 1] ?? -1;
  if (escaped !== 117) {
    const value = readGrammarSelectionJsonShortEscape(escaped);
    if (value < 0 || index + 2 > last) return [1, index];
    return [0, value, index + 2];
  }
  // JSON has fixed \uXXXX escapes, not JavaScript's \u{...} or \xNN.
  if (codes[index + 2] === 123) return [1, index];
  const decoded = decodeUnicodeEscape(codes.slice(index, index + 12), true);
  if (decoded.$ === 'malformed') return [1, index];
  if (decoded.$ === 'unsupported') return [2, index];
  if (index + decoded.end > last) return [1, index];
  return [0, decoded.code, index + decoded.end];
}

/** Reserve a linear upper bound before building input columns/output/maps.
 * A part has at most twelve input scalars and a fixed-size result. These cells
 * stay charged with the parse's existing memory and step budgets.
 * @param {number} bytes @param {boolean} steps @returns {number}
 */
export function readGrammarSelectionMappingReservation(bytes, steps) {
  if (!Number.isInteger(bytes) || bytes < 0 || bytes > 4294967295) return -1;
  return (steps ? 32 : 12) * bytes + 32;
}

/** Intervals are ordered [inputStart,inputEnd,outputStart,outputEnd] columns.
 * Diagnostics inside a decoded scalar identify its complete source spelling;
 * an output end boundary points at the original closing quote.
 * @param {number[]} intervals @param {number} offset @returns {number}
 */
export function readGrammarSelectionMappedOffset(intervals, offset) {
  if (intervals.length < 8 || intervals.length % 4 !== 0
      || !Number.isInteger(offset) || offset < 0) return -1;
  let index = 0;
  while (index + 3 < intervals.length) {
    const start = intervals[index + 2] ?? -1;
    const end = intervals[index + 3] ?? -1;
    if (start <= offset && offset < end) return intervals[index] ?? -1;
    index += 4;
  }
  const last = intervals.length - 4;
  return offset === intervals[last + 2] ? intervals[last] ?? -1 : -1;
}

/** Cover a decoded point/range with whole original source spellings.
 * Sorted intervals admit binary search; the result is
 * [displayAnchor, coveringInputStart, coveringInputEnd], never a claim that
 * an interior UTF-8 byte has one exact corresponding physical source byte.
 * @param {number[][]} intervals @param {number} start @param {number} end @returns {number[]}
 */
export function readGrammarSelectionMappedCover(intervals, start, end) {
  if (intervals.length < 2 || !Number.isInteger(start) || !Number.isInteger(end) || start < 0 || end < start) return [];
  const closing = intervals[intervals.length - 1] ?? [];
  if (closing.length !== 4 || end > (closing[3] ?? -1)) return [];
  if (start === end && end === closing[3]) return [closing[0] ?? -1, closing[0] ?? -1, closing[1] ?? -1];
  let low = 0, high = intervals.length - 1;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const row = intervals[middle] ?? [];
    if ((row[3] ?? -1) <= start) low = middle + 1;
    else high = middle;
  }
  const first = intervals[low] ?? [];
  if (first.length !== 4 || (first[2] ?? -1) > start || start >= (first[3] ?? -1)) return [];
  if (start === end) return [first[0] ?? -1, first[0] ?? -1, first[1] ?? -1];
  high = intervals.length - 1;
  while (low < high) {
    const middle = Math.floor((low + high) / 2);
    const row = intervals[middle] ?? [];
    if ((row[3] ?? -1) < end) low = middle + 1;
    else high = middle;
  }
  const last = intervals[low] ?? [];
  if (last.length !== 4 || end > (last[3] ?? -1)) return [];
  return [first[0] ?? -1, first[0] ?? -1, last[1] ?? -1];
}

/** Shared ordered data roles used by the source-free host validator.
 * @param {string} role @returns {string}
 */
export function readGrammarSelectionMappingFieldOrder(role) {
  if (role === 'source') return 'kind|identity|encoding|byteLength|derivation';
  if (role === 'derivation') return 'operation|input|start|end';
  if (role === 'mapping') return 'kind|operation|input|output|intervals';
  return '';
}

/** @param {string} kind @param {number} identity @param {number} position
 * @param {string} encoding @param {number} length @param {number} actualLength @returns {boolean}
 */
export function acceptGrammarSelectionSourceDefinition(kind, identity, position, encoding, length, actualLength) {
  return kind === 'grammar-source' && identity === position && encoding === 'UTF-8'
    && Number.isInteger(length) && length >= 0 && length <= 4294967295 && length === actualLength;
}

/** @param {string} operation @param {number} input @param {number} output
 * @param {number} start @param {number} end @param {number} inputLength @returns {boolean}
 */
export function acceptGrammarSelectionSourceDerivation(operation, input, output, start, end, inputLength) {
  return operation === 'json-string' && Number.isInteger(input) && input >= 0 && input < output
    && Number.isInteger(start) && Number.isInteger(end) && start >= 0 && start <= end && end <= inputLength;
}

/** @param {number[]} actual @param {number[]} expected @returns {boolean} */
export function acceptGrammarSelectionSourceInterval(actual, expected) {
  return actual.length === 4 && expected.length === 4 && actual[0] === expected[0]
    && actual[1] === expected[1] && actual[2] === expected[2] && actual[3] === expected[3];
}

/** @param {number} position @param {boolean} absentDerivation @returns {boolean} */
export function acceptGrammarSelectionSourceRoot(position, absentDerivation) {
  return position === 0 ? absentDerivation : !absentDerivation;
}

/** @param {number} input @param {number} actualInput @param {number} output @param {number} count @returns {boolean} */
export function acceptGrammarSelectionMappingReference(input, actualInput, output, count) {
  return input === actualInput && Number.isInteger(output) && output > input && output < count;
}

/** @param {number} input @param {number} actualInput @param {number} start
 * @param {number} actualStart @param {number} end @param {number} actualEnd @returns {boolean}
 */
export function acceptGrammarSelectionMappingOccurrence(input, actualInput, start, actualStart, end, actualEnd) {
  return input === actualInput && start === actualStart && end === actualEnd;
}

/** @param {number} start @param {number} end @param {number} cursor
 * @param {number} byteLength @param {boolean} missing @returns {boolean}
 */
export function acceptGrammarSelectionPhysicalSpan(start, end, cursor, byteLength, missing) {
  return Number.isInteger(start) && Number.isInteger(end) && start >= 0 && end <= 4294967295
    && start === cursor && (missing ? end === cursor : end >= start && end - start === byteLength);
}

/** @param {string} kind @param {boolean} mapped @returns {string} */
export function readGrammarSelectionTreeChildren(kind, mapped) {
  if (kind === 'node') return 'children';
  if (kind === 'embed') return mapped ? 'sourceChildren' : 'root';
  if (kind === 'token' || kind === 'error') return 'leaf';
  if (kind === 'missing') return 'missing';
  return 'unsupported';
}

/** @param {string} mapping @param {boolean} physicalChildren @param {boolean} sourceMapping @returns {boolean} */
export function acceptGrammarSelectionMappingProfile(mapping, physicalChildren, sourceMapping) {
  if (mapping === 'identity') return !physicalChildren && !sourceMapping;
  return mapping === 'json-string' && physicalChildren && sourceMapping;
}

/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptGrammarSelectionMappingDescriptor(kind, operation) {
  return kind === 'source-mapping' && operation === 'json-string';
}
