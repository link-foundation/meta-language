// Closed capture and latent-effect decisions over actual checked node identities.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {number} */
function callableEmissionField(starts, counts, labels, targets, node, field) {
  if (node < 0) return -1;
  let at = readRequiredNumberFact(starts, node);
  const end = at + readRequiredNumberFact(counts, node);
  while (at < end) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
    at += 1;
  }
  return -1;
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {string} */
function callableEmissionText(texts, starts, counts, labels, targets, node, field) {
  const child = callableEmissionField(starts, counts, labels, targets, node, field);
  return child < 0 ? '' : readRequiredTextFact(texts, child);
}

/** Stable first-name insertion order, selecting the actual last reference/type.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number[]} references @returns {number[]}
 */
export function planCallableCaptureBindings(texts, starts, counts, labels, targets, references) {
  let selected = [];
  for (const reference of references) {
    const name = callableEmissionText(texts, starts, counts, labels, targets, reference, 'name');
    let position = -1;
    let at = 0;
    while (at < selected.length) {
      const previous = readRequiredNumberFact(selected, at);
      if (callableEmissionText(texts, starts, counts, labels, targets, previous, 'name') === name) position = at;
      at += 1;
    }
    if (position < 0) selected = selected.concat([reference]);
    else selected = selected.slice(0, position).concat([reference]).concat(selected.slice(position + 1));
  }
  return selected;
}

/** A closure body runs at invocation, separately from creating its value.
 * @param {string} kind @returns {boolean}
 */
export function acceptCallableCreationEffectTraversal(kind) {
  return kind !== 'lambda';
}

/** After the existing total-operation settling, these operations can abort.
 * @param {string} kind @param {string} semantics @param {string} flavor @param {string} byZero @returns {boolean}
 */
export function requireCallableOutputEffect(kind, semantics, flavor, byZero) {
  return kind === 'print' || kind === 'abort'
    || kind === 'cast' && flavor === 'checked'
    || kind === 'unary' && semantics === 'checked'
    || kind === 'binary' && (semantics === 'checked' || byZero === 'abort');
}

/** @param {string} field @returns {boolean} */
function acceptCallableEffectField(field) {
  return field !== 'type' && field !== 'domain' && field !== 'from' && field !== 'to' && field !== 'span';
}

/** @param {string[]} names @param {string} name @returns {boolean} */
function callableEmissionHasName(names, name) {
  for (const previous of names) if (previous === name) return true;
  return false;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string[]} names @returns {boolean} */
export function readCallableEagerEffects(forms, texts, starts, counts, labels, targets, node, names) {
  if (node < 0) return false;
  const form = readRequiredTextFact(forms, node);
  if (form !== 'object' && form !== 'array') return false;
  const kind = callableEmissionText(texts, starts, counts, labels, targets, node, 'k');
  if (!acceptCallableCreationEffectTraversal(kind)) return false;
  if (requireCallableOutputEffect(kind,
    callableEmissionText(texts, starts, counts, labels, targets, node, 'semantics'),
    callableEmissionText(texts, starts, counts, labels, targets, node, 'flavor'),
    callableEmissionText(texts, starts, counts, labels, targets, node, 'byZero'))) return true;
  if (kind === 'call' && callableEmissionHasName(names, callableEmissionText(texts, starts, counts, labels, targets, node, 'fn'))) return true;
  if (kind === 'apply') {
    const fn = callableEmissionField(starts, counts, labels, targets, node, 'fn');
    const target = callableEmissionText(texts, starts, counts, labels, targets, fn, 'k');
    if (target === 'functionRef' && callableEmissionHasName(names, callableEmissionText(texts, starts, counts, labels, targets, fn, 'fn'))) return true;
    if (target === 'lambda' && readCallableEagerEffects(forms, texts, starts, counts, labels, targets, callableEmissionField(starts, counts, labels, targets, fn, 'body'), names)) return true;
  }
  let at = readRequiredNumberFact(starts, node);
  const end = at + readRequiredNumberFact(counts, node);
  while (at < end) {
    if (acceptCallableEffectField(readRequiredTextFact(labels, at)) && readCallableEagerEffects(forms, texts, starts, counts, labels, targets, readRequiredNumberFact(targets, at), names)) return true;
    at += 1;
  }
  return false;
}

/** Actual named declarations whose invocation prints or may abort.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} declarations @returns {number[]}
 */
export function readCallableEffectfulDeclarations(forms, texts, starts, counts, labels, targets, declarations) {
  let selected = [];
  let names = [];
  let grew = true;
  while (grew) {
    grew = false;
    let at = readRequiredNumberFact(starts, declarations);
    const end = at + readRequiredNumberFact(counts, declarations);
    while (at < end) {
      const entry = readRequiredNumberFact(targets, at);
      const kind = callableEmissionText(texts, starts, counts, labels, targets, entry, 'k');
      const name = callableEmissionText(texts, starts, counts, labels, targets, entry, 'fullName');
      if (kind === 'fn' && !callableEmissionHasName(names, name) && readCallableEagerEffects(forms, texts, starts, counts, labels, targets, callableEmissionField(starts, counts, labels, targets, entry, 'body'), names)) {
        selected = selected.concat([entry]);
        names = names.concat([name]);
        grew = true;
      }
      at += 1;
    }
  }
  return selected;
}

/** @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string[]} names @returns {number[]} */
function callableEmissionObligations(forms, texts, starts, counts, labels, targets, node, names) {
  if (node < 0) return [];
  const form = readRequiredTextFact(forms, node);
  if (form !== 'object' && form !== 'array') return [];
  const kind = callableEmissionText(texts, starts, counts, labels, targets, node, 'k');
  let selected = [];
  if (kind === 'lambda' && readCallableEagerEffects(forms, texts, starts, counts, labels, targets, callableEmissionField(starts, counts, labels, targets, node, 'body'), names)) selected = [node];
  if (kind === 'functionRef' && callableEmissionHasName(names, callableEmissionText(texts, starts, counts, labels, targets, node, 'fn'))) selected = [node];
  if (kind === 'apply') {
    const fn = callableEmissionField(starts, counts, labels, targets, node, 'fn');
    const target = callableEmissionText(texts, starts, counts, labels, targets, fn, 'k');
    if (target === 'functionRef' && callableEmissionHasName(names, callableEmissionText(texts, starts, counts, labels, targets, fn, 'fn'))) selected = [node];
    if (target === 'lambda' && readCallableEagerEffects(forms, texts, starts, counts, labels, targets, callableEmissionField(starts, counts, labels, targets, fn, 'body'), names)) selected = [node];
  }
  let at = readRequiredNumberFact(starts, node);
  const end = at + readRequiredNumberFact(counts, node);
  while (at < end) {
    if (acceptCallableEffectField(readRequiredTextFact(labels, at))) selected = selected.concat(callableEmissionObligations(forms, texts, starts, counts, labels, targets, readRequiredNumberFact(targets, at), names));
    at += 1;
  }
  return selected;
}

/** Returned IDs retain the actual Lambda, FunctionRef and Apply source spans.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {number[]} declarations @returns {number[]}
 */
export function planCallableOutputObligations(forms, texts, starts, counts, labels, targets, node, declarations) {
  let names = [];
  for (const entry of declarations) names = names.concat([callableEmissionText(texts, starts, counts, labels, targets, entry, 'fullName')]);
  return callableEmissionObligations(forms, texts, starts, counts, labels, targets, node, names);
}

/** @param {string} target @param {number} count @returns {string} */
export function readCallableOutputTargetRefusal(target, count) {
  if ((target === 'Lean' || target === 'Rocq') && count > 0) return 'function values that print or may abort require an output-threaded function type';
  return '';
}
