// Read original lexer positions for the closed mutable callback profile.
// The ordinary source readers still type-check every captured type and body.
import {readRequiredTextFact} from './fact-column-rules.js';
import {readRequiredNumberFact} from './fact-column-rules.js';
import {readRustFnMutSourceDeclarationBudgetError} from './rust-fnmut-source-declaration-rules.js';

/** @param {string[]} values @param {number} index @returns {string} */
function tokenFact(values, index) {
  return index >= 0 && index < values.length ? readRequiredTextFact(values, index) : '';
}

/** @param {string[]} kinds @param {string[]} values @param {number} index @param {string} value @returns {boolean} */
function isPunctuation(kinds, values, index, value) {
  return tokenFact(kinds, index) === 'punct' && tokenFact(values, index) === value;
}

/** @param {string[]} kinds @param {string[]} values @param {number} index @param {string} value @returns {boolean} */
function isIdentifier(kinds, values, index, value) {
  return tokenFact(kinds, index) === 'identifier' && tokenFact(values, index) === value;
}

/** @param {string[]} kinds @param {string[]} values @param {number} start @param {string} opening @param {string} closing @returns {number} */
function closingPosition(kinds, values, start, opening, closing) {
  if (!isPunctuation(kinds, values, start, opening)) return -1;
  let depth = 1;
  for (let at = start + 1; at < kinds.length && at - start < 1000000; at += 1) {
    if (isPunctuation(kinds, values, at, opening)) depth += 1;
    if (isPunctuation(kinds, values, at, closing)) {
      depth -= 1;
      if (depth === 0) return at;
    }
  }
  return -1;
}

/** Start/end pairs of top-level comma-delimited type positions.
 * @param {string[]} kinds @param {string[]} values @param {number} start @param {number} end
 * @returns {number[]}
 */
function delimitedPositions(kinds, values, start, end) {
  const ranges = [];
  let begin = start;
  let angles = 0;
  for (let at = start; at < end; at += 1) {
    if (tokenFact(kinds, at) !== 'punct') continue;
    const value = tokenFact(values, at);
    if (value === '(' || value === '[' || value === '{') {
      const close = closingPosition(kinds, values, at, value, value === '(' ? ')' : value === '[' ? ']' : '}');
      if (close < 0 || close >= end) return [-1, at];
      at = close;
      continue;
    }
    if (value === '<') angles += 1;
    if (value === '>') angles -= 1;
    if (value === '>>') angles -= 2;
    if (angles < 0 || value === ')' || value === ']' || value === '}') return [-1, at];
    if (value === ',' && angles === 0) {
      if (begin === at) return [-1, at];
      ranges.push(begin, at);
      begin = at + 1;
    }
  }
  if (angles !== 0) return [-1, end];
  if (begin < end) ranges.push(begin, end);
  return ranges;
}

/** @param {string[]} kinds @param {string[]} values @param {number} start @param {number} end @returns {number[]} */
function callbackPositions(kinds, values, start, end) {
  const mutable = isIdentifier(kinds, values, start, 'mut');
  const binder = mutable ? start + 1 : start;
  if (tokenFact(kinds, binder) !== 'identifier' || !isPunctuation(kinds, values, binder + 1, ':')) return [];
  const typeStart = binder + 2;
  const exclusive = isPunctuation(kinds, values, typeStart, '&');
  if (exclusive && !isIdentifier(kinds, values, typeStart + 1, 'mut')) return [];
  const implementation = exclusive ? typeStart + 2 : typeStart;
  if (!isIdentifier(kinds, values, implementation, 'impl') || !isIdentifier(kinds, values, implementation + 1, 'FnMut')) return [];
  const argumentsStart = implementation + 2;
  const argumentsEnd = closingPosition(kinds, values, argumentsStart, '(', ')');
  if (argumentsEnd < 0 || argumentsEnd + 2 >= end || !isPunctuation(kinds, values, argumentsEnd + 1, '->')) return [-1, argumentsStart];
  const argumentsRanges = delimitedPositions(kinds, values, argumentsStart + 1, argumentsEnd);
  if (argumentsRanges.length > 0 && readRequiredNumberFact(argumentsRanges, 0) < 0) return argumentsRanges;
  return [binder, exclusive ? 1 : 0, mutable ? 1 : 0, typeStart, end, argumentsEnd + 2, end, ...argumentsRanges];
}

/** Empty for ordinary functions. An error contains -1, a reason, and the
 * selected token. Successful rows contain body bounds (inclusive), optional
 * result bounds, formal count, actual callback index, generic count and starts,
 * then the callback positions. All type-range ends are exclusive.
 * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number[]}
 */
export function readRustFnMutSourceTokenPlan(kinds, values, start) {
  if (kinds.length !== values.length || !isIdentifier(kinds, values, start, 'fn') ||
      tokenFact(kinds, start + 1) !== 'identifier') return [-1, 1, start];
  let headerEnd = start + 2;
  let selected = false;
  while (headerEnd < kinds.length && !isPunctuation(kinds, values, headerEnd, '{') &&
         !isPunctuation(kinds, values, headerEnd, ';') && tokenFact(kinds, headerEnd) !== 'eof') {
    if (headerEnd - start >= 1000000) return [-1, 2, start];
    if (isIdentifier(kinds, values, headerEnd, 'FnMut')) selected = true;
    headerEnd += 1;
  }
  if (!selected) return [];

  const generics = [];
  const genericNames = [];
  let formalStart = start + 2;
  if (isPunctuation(kinds, values, formalStart, '<')) {
    const end = closingPosition(kinds, values, formalStart, '<', '>');
    if (end < 0 || end >= headerEnd) return [-1, 3, formalStart];
    const ranges = delimitedPositions(kinds, values, formalStart + 1, end);
    if (ranges.length === 0 || readRequiredNumberFact(ranges, 0) < 0) return [-1, 3, formalStart];
    for (let at = 0; at < ranges.length; at += 2) {
      const binder = readRequiredNumberFact(ranges, at);
      const name = tokenFact(values, binder);
      if (readRequiredNumberFact(ranges, at + 1) !== binder + 1 ||
          tokenFact(kinds, binder) !== 'identifier' || genericNames.includes(name)) return [-1, 3, binder];
      generics.push(binder);
      genericNames.push(name);
    }
    formalStart = end + 1;
  }

  const formalEnd = closingPosition(kinds, values, formalStart, '(', ')');
  if (formalEnd < 0 || formalEnd >= headerEnd) return [-1, 4, formalStart];
  const formals = delimitedPositions(kinds, values, formalStart + 1, formalEnd);
  if (formals.length > 0 && readRequiredNumberFact(formals, 0) < 0) {
    return [-1, 4, readRequiredNumberFact(formals, 1)];
  }
  let callback = [];
  let callbackIndex = -1;
  let callbacks = 0;
  for (let at = 0; at < formals.length; at += 2) {
    const binder = readRequiredNumberFact(formals, at);
    const positions = callbackPositions(kinds, values, binder, readRequiredNumberFact(formals, at + 1));
    if (positions.length === 0) continue;
    if (readRequiredNumberFact(positions, 0) < 0) return [-1, 5, readRequiredNumberFact(positions, 1)];
    callback = positions;
    callbackIndex = at / 2;
    callbacks += 1;
  }

  for (let at = formalEnd + 1; at < headerEnd; at += 1) {
    if (isIdentifier(kinds, values, at, 'where')) return [-1, 6, at];
  }
  const bodyEnd = closingPosition(kinds, values, headerEnd, '{', '}');
  if (bodyEnd < 0) return [-1, 6, headerEnd];
  const formalCount = formals.length / 2;
  if (readRustFnMutSourceDeclarationBudgetError(bodyEnd - start + 1, formalCount, generics.length, callbacks) !== '') {
    return [-1, 7, start];
  }
  const hasResult = isPunctuation(kinds, values, formalEnd + 1, '->') && formalEnd + 2 < headerEnd;
  return [headerEnd, bodyEnd, hasResult ? formalEnd + 2 : -1, hasResult ? headerEnd : -1,
    formalCount, callbackIndex, generics.length, ...generics, ...callback];
}

/** @param {number} reason @returns {string} */
export function readRustFnMutSourceTokenError(reason) {
  if (reason === 1) return 'the selected source function needs its actual lexical token and owner path';
  if (reason === 2) return 'the original function header exceeds its actual source token budget';
  if (reason === 3) return 'this closed specialization requires distinct unconstrained actual type binders';
  if (reason === 4) return 'an actual source formal has an inconsistent complete delimiter inventory';
  if (reason === 5) return 'the actual FnMut bound needs its complete explicit result source position';
  if (reason === 6) return 'where predicates and absent generic bodies need their actual separate source interpretation';
  return 'a complete FnMut source declaration needs one actual callback and bounded tokens, parameters and concrete generic arguments';
}
