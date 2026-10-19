// Array.from decisions select original type owners from primitive fact columns.
// The adapters evaluate expressions and apply constraints in source order.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';
import { readArrayIterableForm } from './array-value-rules.js';

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {number} */
export function readArrayFromField(starts, counts, labels, targets, node, field) {
  if (node < 0) return -1;
  const start = readRequiredNumberFact(starts, node);
  const count = readRequiredNumberFact(counts, node);
  for (let at = start; at < start + count; at += 1) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
  }
  return -1;
}

/** @param {number[]} starts @param {number[]} counts @param {number[]} targets @param {number} node @param {number} index @returns {number} */
export function readArrayFromElement(starts, counts, targets, node, index) {
  if (node < 0 || index < 0 || index >= readRequiredNumberFact(counts, node)) return -1;
  return readRequiredNumberFact(targets, readRequiredNumberFact(starts, node) + index);
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @returns {string} */
export function readArrayFromKind(texts, starts, counts, labels, targets, node) {
  const kind = readArrayFromField(starts, counts, labels, targets, node, 'kind');
  return kind < 0 ? '' : readRequiredTextFact(texts, kind);
}

/** @param {number} count @returns {string} */
export function readArrayFromArgumentError(count) {
  return count < 1 || count > 2 ? 'Array.from expects one or two arguments' : '';
}

/** @param {string} kind @returns {string} */
export function readArrayFromReceiverError(kind) {
  return readArrayIterableForm(kind, false) ? '' : 'this iterable needs an operational iterator encoding';
}

// Codes retain the original array/set element or ordered Map key/value owners.
// String's element is the shared String primitive, constructed by the adapter.
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} object @returns {number[]} */
export function planArrayFromIterableType(texts, starts, counts, labels, targets, object) {
  const form = readArrayIterableForm(readArrayFromKind(texts, starts, counts, labels, targets, object), false);
  if (form === 'array') return [0, readArrayFromField(starts, counts, labels, targets, object, 'element')];
  if (form === 'string') return [1];
  if (form === 'set') return [2, readArrayFromField(starts, counts, labels, targets, object, 'element')];
  if (form === 'map') return [3, readArrayFromField(starts, counts, labels, targets, object, 'key'), readArrayFromField(starts, counts, labels, targets, object, 'value')];
  return [];
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} expected @returns {number[]} */
export function planArrayFromExpectedElement(texts, starts, counts, labels, targets, expected) {
  if (readArrayFromKind(texts, starts, counts, labels, targets, expected) === 'array') {
    return [1, readArrayFromField(starts, counts, labels, targets, expected, 'element')];
  }
  return [0, -1];
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} args @param {number} expected @returns {number[]} */
export function planArrayFromInferenceInputs(texts, starts, counts, labels, targets, args, expected) {
  return planArrayFromExpectedElement(texts, starts, counts, labels, targets, expected).concat([
    readArrayFromElement(starts, counts, targets, args, 0),
    readArrayFromElement(starts, counts, targets, args, 1),
  ]);
}

/** @param {number} declared @param {number} provided @returns {string} */
export function readArrayFromCallbackError(declared, provided) {
  return declared > provided ? 'a callback declares more parameters than its operation supplies' : '';
}

// The leading instruction is -1 for the existing count refusal, 0 for ordered
// unification pairs, or 1 for binding the original unresolved term to a function.
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} term @param {number} declared @param {number} params @param {number} ret @returns {number[]} */
export function planArrayFromCallbackConstraints(texts, starts, counts, labels, targets, term, declared, params, ret) {
  if (readArrayFromKind(texts, starts, counts, labels, targets, declared) !== 'function') return [1, term, params, ret];
  const declaredParams = readArrayFromField(starts, counts, labels, targets, declared, 'params');
  const count = readRequiredNumberFact(counts, declaredParams);
  if (readArrayFromCallbackError(count, readRequiredNumberFact(counts, params))) return [-1];
  let pairs = [0];
  for (let index = 0; index < count; index += 1) {
    pairs = pairs.concat([readArrayFromElement(starts, counts, targets, declaredParams, index), readArrayFromElement(starts, counts, targets, params, index)]);
  }
  return pairs.concat([readArrayFromField(starts, counts, labels, targets, declared, 'ret'), ret]);
}

// Checked .map rederivation shares the actual parameter owners, with the
// original kind/count refusals preceding all structural type comparisons.
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} callback @param {number} params @returns {number[]} */
export function planArrayFromMapCallback(texts, starts, counts, labels, targets, callback, params) {
  if (readArrayFromKind(texts, starts, counts, labels, targets, callback) !== 'function') return [0, -1];
  const declared = readArrayFromField(starts, counts, labels, targets, callback, 'params');
  const count = readRequiredNumberFact(counts, declared);
  if (readArrayFromCallbackError(count, readRequiredNumberFact(counts, params))) return [1, -1];
  let plan = [2, readArrayFromField(starts, counts, labels, targets, callback, 'ret')];
  for (let index = 0; index < count; index += 1) {
    plan = plan.concat([readArrayFromElement(starts, counts, targets, declared, index), readArrayFromElement(starts, counts, targets, params, index)]);
  }
  return plan;
}

/** @param {number} status @param {boolean} parametersEqual @returns {string} */
export function readArrayFromMapCallbackError(status, parametersEqual) {
  if (status === 0) return '.map() requires a function';
  if (status === 1 || !parametersEqual) return '.map() callback parameters differ';
  return '';
}

// This is the existing map-only formatter. Child emission and target runtime
// selection remain in their original host positions, before this formatting.
/** @param {string} target @param {string} object @param {string} callback @param {number} parameterCount @param {string} suffix @returns {string} */
export function renderArrayFromMapping(target, object, callback, parameterCount, suffix) {
  if (target === 'JavaScript') return '(' + object + ').map(' + callback + ')';
  if (target === 'Rust') {
    const values = 'ml_values' + suffix;
    const fn = 'ml_callback' + suffix;
    const index = 'ml_index' + suffix;
    const value = 'ml_value' + suffix;
    const available = [value, '(' + index + ' as f64)', values + '.clone()'];
    const call = fn + '(' + available.slice(0, parameterCount).join(', ') + ')';
    const head = 'let ' + values + ' = ' + object + '; let ' + fn + ' = ' + callback + ';';
    return '{ ' + head + ' ' + values + '.iter().cloned().enumerate().map(|(' + index + ', ' + value + ')| ' + call + ').collect::<Vec<_>>() }';
  }
  if (target === 'Lean') {
    const available = ['ml_pair.1', '(Float.ofNat ml_pair.2)', 'ml_values'];
    const call = parameterCount ? '(ml_callback ' + available.slice(0, parameterCount).join(' ') + ')' : 'ml_callback';
    const head = '(let ml_values := ' + object + '; let ml_callback := ' + callback + '; ';
    return head + ' (ml_values.toList.zipIdx.map (fun ml_pair => ' + call + ')).toArray)';
  }
  if (target === 'Rocq') {
    const available = ['(fst ml_pair)', '(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (snd ml_pair))))', 'ml_values'];
    const call = parameterCount ? '(ml_callback ' + available.slice(0, parameterCount).join(' ') + ')' : 'ml_callback';
    const head = '(let ml_values := ' + object + ' in let ml_callback := ' + callback + ' in ';
    return head + ' List.map (fun ml_pair => ' + call + ') (List.combine ml_values (List.seq 0 (List.length ml_values))))';
  }
  return '';
}
