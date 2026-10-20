// Standard borrowed-str ASCII query decisions. Native implementations are
// generated from this complete module and its complete recorded dependency.
import { acceptRustConstFunctionName, acceptRustConstFunctionNameCharacter } from './rust-const-function-rules.js';

/** @param {string} method @param {number} scalar @returns {boolean} */
export function acceptAsciiCharacterMethod(method, scalar) {
  if (!Number.isInteger(scalar) || scalar < 0 || scalar > 127) return false;
  const upper = scalar >= 65 && scalar <= 90;
  const lower = scalar >= 97 && scalar <= 122;
  const digit = scalar >= 48 && scalar <= 57;
  if (method === 'is_ascii_alphabetic') return upper || lower;
  if (method === 'is_ascii_alphanumeric') return upper || lower || digit;
  if (method === 'is_ascii_uppercase') return upper;
  if (method === 'is_ascii_lowercase') return lower;
  if (method === 'is_ascii_digit') return digit;
  return false;
}

/** The proven standard str query supplies actual primitive char when no
 * annotation exists. An explicit annotation retains its actual type owner.
 * @param {boolean} annotated @param {string} resolved @param {boolean} shadowed
 * @returns {boolean} */
export function acceptRustAsciiCharacterOwner(annotated, resolved, shadowed) {
  return !annotated || (resolved === 'char' && !shadowed);
}

/** [first=1/all=2, actual callback opening index], or an unrelated syntax [].
 * @param {string} method @param {string[]} kinds @param {string[]} values
 * @param {number} start @returns {number[]} */
export function planRustAsciiTextQuery(method, kinds, values, start) {
  if (kinds.length !== values.length || !Number.isInteger(start) || start < 0 || start >= kinds.length) return [];
  if (method === 'starts_with' && kinds[start] === 'punct' && values[start] === '('
    && kinds[start + 1] === 'punct' && values[start + 1] === '|') return [1, start + 1];
  if (method === 'chars' && kinds[start] === 'punct' && values[start] === '('
    && kinds[start + 1] === 'punct' && values[start + 1] === ')'
    && kinds[start + 2] === 'punct' && values[start + 2] === '.'
    && kinds[start + 3] === 'identifier' && values[start + 3] === 'all'
    && kinds[start + 4] === 'punct' && values[start + 4] === '(') return [2, start + 5];
  return [];
}

/** Closed pure callback |c[:char]| c.METHOD() || c == 'ASCII'. The standard
 * query owns the inferred char; no general character/iterator type is added.
 * [actual closing-parenthesis index, annotation index or -1, ...ASCII set].
 * @param {string[]} kinds @param {string[]} values @param {number[]} characters
 * @param {number} start @returns {number[]} */
export function planRustAsciiCharacterCallback(kinds, values, characters, start) {
  if (kinds.length !== values.length || kinds.length !== characters.length
    || !Number.isInteger(start) || start < 0 || start >= kinds.length) return [];
  if (kinds[start] !== 'punct' || values[start] !== '|') return [];
  let at = start + 1;
  if (kinds[at] !== 'identifier') return [];
  const parameter = values[at] ?? '';
  if (!acceptRustConstFunctionName(parameter)) return [];
  for (let index = 0; index < parameter.length; index += 1) {
    if (!acceptRustConstFunctionNameCharacter(parameter.charCodeAt(index) ?? -1, index === 0)) return [];
  }
  at += 1;
  let annotation = -1;
  if (kinds[at] === 'punct' && values[at] === ':') {
    at += 1;
    annotation = at;
    if (kinds[at] !== 'identifier' || values[at] !== 'char') return [];
    at += 1;
  }
  if (kinds[at] !== 'punct' || values[at] !== '|') return [];
  at += 1;
  /** @type {string[]} */
  const methods = [];
  /** @type {number[]} */
  const literals = [];
  while (at < kinds.length) {
    if (kinds[at] !== 'identifier' || values[at] !== parameter) return [];
    at += 1;
    if (kinds[at] === 'punct' && values[at] === '.') {
      at += 1;
      const method = values[at] ?? '';
      if (kinds[at] !== 'identifier' || (method !== 'is_ascii_alphabetic'
        && method !== 'is_ascii_alphanumeric' && method !== 'is_ascii_uppercase'
        && method !== 'is_ascii_lowercase' && method !== 'is_ascii_digit')) return [];
      at += 1;
      if (kinds[at] !== 'punct' || values[at] !== '(') return [];
      at += 1;
      if (kinds[at] !== 'punct' || values[at] !== ')') return [];
      at += 1;
      methods.push(method);
      literals.push(-1);
    } else {
      if (kinds[at] !== 'punct' || values[at] !== '==') return [];
      at += 1;
      const scalar = characters[at] ?? -1;
      if (kinds[at] !== 'char' || !Number.isInteger(scalar) || scalar < 0 || scalar > 127) return [];
      methods.push('literal');
      literals.push(scalar);
      at += 1;
    }
    if (kinds[at] === 'punct' && values[at] === '||') {
      at += 1;
      continue;
    }
    if (kinds[at] !== 'punct' || values[at] !== ')') return [];
    /** @type {number[]} */
    const result = [at, annotation];
    for (let scalar = 0; scalar < 128; scalar += 1) {
      let accepted = false;
      for (let index = 0; index < methods.length; index += 1) {
        const method = methods[index] ?? '';
        if (method === 'literal' ? scalar === (literals[index] ?? -1)
          : acceptAsciiCharacterMethod(method, scalar)) accepted = true;
      }
      if (accepted) result.push(scalar);
    }
    return result;
  }
  return [];
}
