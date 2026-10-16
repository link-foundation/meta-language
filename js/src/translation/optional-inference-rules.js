// Optional constraints use the same primitive fact arrays and original term
// identities as callable constraints. The ordinary unifier owns recursion,
// variable binding, occurs checks, and the order of their effects.
import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';

/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {number} */
function optionalFactField(starts, counts, labels, targets, node, field) {
  if (node < 0 || node >= starts.length) return -1;
  let at = readRequiredNumberFact(starts, node);
  const end = at + readRequiredNumberFact(counts, node);
  while (at < end) {
    if (readRequiredTextFact(labels, at) === field) return readRequiredNumberFact(targets, at);
    at += 1;
  }
  return -1;
}

/** @param {string[]} texts @param {number} node @returns {string} */
function optionalFactText(texts, node) {
  return node < 0 || node >= texts.length ? '' : readRequiredTextFact(texts, node);
}

/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} field @returns {string} */
function optionalFactTag(texts, starts, counts, labels, targets, node, field) {
  return optionalFactText(texts, optionalFactField(starts, counts, labels, targets, node, field));
}

/** @param {number[]} starts @param {number[]} counts @param {number[]} targets @param {number} node @param {number} index @returns {number} */
function optionalFactElement(starts, counts, targets, node, index) {
  if (node < 0 || index < 0 || index >= readRequiredNumberFact(counts, node)) return -1;
  return readRequiredNumberFact(targets, readRequiredNumberFact(starts, node) + index);
}

/**
 * Plans one element pair followed by the two absence owners, or -1 owners
 * when only one side is optional. Absences are read after recursively solving
 * the pair: child aliases may update an owner's policy during that solve.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} left
 * @param {number} right @returns {number[]}
 */
export function planOptionalTypeConstraints(texts, starts, counts, labels, targets, left, right) {
  const leftOptional = optionalFactTag(texts, starts, counts, labels, targets, left, 'kind') === 'option';
  const rightOptional = optionalFactTag(texts, starts, counts, labels, targets, right, 'kind') === 'option';
  if (!leftOptional && !rightOptional) return [];
  if (!leftOptional) return [optionalFactField(starts, counts, labels, targets, right, 'element'), left, -1, -1];
  const first = optionalFactField(starts, counts, labels, targets, left, 'element');
  const second = rightOptional ? optionalFactField(starts, counts, labels, targets, right, 'element') : right;
  return [first, second, leftOptional && rightOptional ? left : -1, leftOptional && rightOptional ? right : -1];
}

/**
 * Empty means defer. Opcode 0 unifies two actual terms; opcode 1 wraps the
 * right term in Option/both before unifying it with the original left term.
 * Original and resolved identities stay separate, including on forced steps.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} left
 * @param {number} resolved @param {number} right @param {boolean} force
 * @returns {number[]}
 */
export function planOptionalCoalescingConstraints(texts, starts, counts, labels, targets, left, resolved, right, force) {
  if (force) return [1, left, right];
  const kind = optionalFactTag(texts, starts, counts, labels, targets, resolved, 'kind');
  if (kind === 'var') return [];
  return [0, kind === 'option' ? optionalFactField(starts, counts, labels, targets, resolved, 'element') : left, right];
}

/**
 * The first identity is the field's receiver element. -1 requests one fresh
 * element and the second identity receives Option/both of that fresh element.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} object
 * @param {number} resolved @returns {number[]}
 */
export function planOptionalFieldInference(texts, starts, counts, labels, targets, object, resolved) {
  const kind = optionalFactTag(texts, starts, counts, labels, targets, resolved, 'kind');
  if (kind === 'var') return [-1, object];
  return [kind === 'option' ? optionalFactField(starts, counts, labels, targets, resolved, 'element') : object];
}

/**
 * Opcode 0 recursively unifies [left,right] and then resolves the selected
 * value. Opcode 1 reuses the current optional value; opcode 2 constructs one
 * Option/undefined of the selected original branch. Reuse preserves identity
 * at a fixed point, rather than manufacturing a fresh equivalent record.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} left
 * @param {number} right @param {number} resolvedLeft @param {number} resolvedRight
 * @param {number} current @returns {number[]}
 */
export function planOptionalJoinConstraints(texts, starts, counts, labels, targets, left, right, resolvedLeft, resolvedRight, current) {
  const leftKind = optionalFactTag(texts, starts, counts, labels, targets, resolvedLeft, 'kind');
  const rightKind = optionalFactTag(texts, starts, counts, labels, targets, resolvedRight, 'kind');
  if ((leftKind === 'unit') !== (rightKind === 'unit')) {
    const element = leftKind === 'unit' ? right : left;
    if (optionalFactTag(texts, starts, counts, labels, targets, current, 'kind') === 'option'
      && optionalFactField(starts, counts, labels, targets, current, 'element') === element
      && optionalFactTag(texts, starts, counts, labels, targets, current, 'absence') === 'undefined') return [1, current];
    return [2, element];
  }
  return [0, rightKind === 'option' ? right : left, left, right];
}

/**
 * One recursive absence-widening step: [additional flag, actual,declared,...].
 * The common solver resolves each returned pair before its next step, retaining
 * its existing variable aliases. Function arguments do not widen a return
 * annotation; only the actual function result participates here.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} actual
 * @param {number} declared @returns {number[]}
 */
export function planOptionalAdditionalAbsence(texts, starts, counts, labels, targets, actual, declared) {
  const actualKind = optionalFactTag(texts, starts, counts, labels, targets, actual, 'kind');
  const declaredKind = optionalFactTag(texts, starts, counts, labels, targets, declared, 'kind');
  if (actualKind === 'option') {
    const policy = optionalFactTag(texts, starts, counts, labels, targets, declared, 'absence');
    if (declaredKind !== 'option' || policy !== '' && policy !== 'both'
      && optionalFactTag(texts, starts, counts, labels, targets, actual, 'absence') !== policy) return [1];
    return [0, optionalFactField(starts, counts, labels, targets, actual, 'element'), optionalFactField(starts, counts, labels, targets, declared, 'element')];
  }
  if (actualKind !== declaredKind) return [0];
  if (actualKind === 'array' || actualKind === 'set' || actualKind === 'cell') return [0, optionalFactField(starts, counts, labels, targets, actual, 'element'), optionalFactField(starts, counts, labels, targets, declared, 'element')];
  if (actualKind === 'map') return [0,
    optionalFactField(starts, counts, labels, targets, actual, 'key'), optionalFactField(starts, counts, labels, targets, declared, 'key'),
    optionalFactField(starts, counts, labels, targets, actual, 'value'), optionalFactField(starts, counts, labels, targets, declared, 'value')];
  if (actualKind === 'function') return [0, optionalFactField(starts, counts, labels, targets, actual, 'ret'), optionalFactField(starts, counts, labels, targets, declared, 'ret')];
  let pairs = [0];
  if (actualKind === 'tuple') {
    const values = optionalFactField(starts, counts, labels, targets, actual, 'elements');
    const expected = optionalFactField(starts, counts, labels, targets, declared, 'elements');
    let at = 0;
    while (values >= 0 && at < readRequiredNumberFact(counts, values)) {
      const other = optionalFactElement(starts, counts, targets, expected, at);
      if (other >= 0 && optionalFactTag(texts, starts, counts, labels, targets, other, 'kind') !== '') pairs = pairs.concat([optionalFactElement(starts, counts, targets, values, at), other]);
      at += 1;
    }
  }
  if (actualKind === 'record') {
    const fields = optionalFactField(starts, counts, labels, targets, actual, 'fields');
    const expected = optionalFactField(starts, counts, labels, targets, declared, 'fields');
    let at = 0;
    while (fields >= 0 && at < readRequiredNumberFact(counts, fields)) {
      const field = optionalFactElement(starts, counts, targets, fields, at);
      const name = optionalFactTag(texts, starts, counts, labels, targets, field, 'name');
      let candidate = 0;
      let matched = false;
      while (!matched && expected >= 0 && candidate < readRequiredNumberFact(counts, expected)) {
        const other = optionalFactElement(starts, counts, targets, expected, candidate);
        if (optionalFactTag(texts, starts, counts, labels, targets, other, 'name') === name) {
          pairs = pairs.concat([optionalFactField(starts, counts, labels, targets, field, 'type'), optionalFactField(starts, counts, labels, targets, other, 'type')]);
          matched = true;
        }
        candidate += 1;
      }
      at += 1;
    }
  }
  return pairs;
}


/** One logical type-identity step. Empty refuses; [1,...pairs] asks the common
 * host traversal to compare resolved children. Field names are paired by their
 * stable duplicate ordinal, exactly as canonical field-name sorting does.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} left
 * @param {number} right @returns {number[]} */
export function planInferenceTermIdentity(texts, starts, counts, labels, targets, left, right) {
  const kind = optionalFactTag(texts, starts, counts, labels, targets, left, 'kind');
  if (kind !== optionalFactTag(texts, starts, counts, labels, targets, right, 'kind')) return [];
  if (kind === 'var' && optionalFactTag(texts, starts, counts, labels, targets, left, 'id') !== optionalFactTag(texts, starts, counts, labels, targets, right, 'id')) return [];
  if (kind === 'fixed' && (optionalFactTag(texts, starts, counts, labels, targets, left, 'bits') !== optionalFactTag(texts, starts, counts, labels, targets, right, 'bits')
    || optionalFactTag(texts, starts, counts, labels, targets, left, 'signed') !== optionalFactTag(texts, starts, counts, labels, targets, right, 'signed'))) return [];
  if (kind === 'data' && optionalFactTag(texts, starts, counts, labels, targets, left, 'name') !== optionalFactTag(texts, starts, counts, labels, targets, right, 'name')) return [];
  if ((kind === 'array' || kind === 'record' || kind === 'map' || kind === 'set')
    && (optionalFactTag(texts, starts, counts, labels, targets, left, 'reference') === 'true') !== (optionalFactTag(texts, starts, counts, labels, targets, right, 'reference') === 'true')) return [];
  if (kind === 'option') {
    let first = left;
    let second = right;
    let firstAbsence = optionalFactTag(texts, starts, counts, labels, targets, first, 'absence');
    let secondAbsence = optionalFactTag(texts, starts, counts, labels, targets, second, 'absence');
    if (firstAbsence === '') firstAbsence = 'both';
    if (secondAbsence === '') secondAbsence = 'both';
    first = optionalFactField(starts, counts, labels, targets, first, 'element');
    second = optionalFactField(starts, counts, labels, targets, second, 'element');
    while (optionalFactTag(texts, starts, counts, labels, targets, first, 'kind') === 'option') {
      const absence = optionalFactTag(texts, starts, counts, labels, targets, first, 'absence');
      if (absence !== firstAbsence) firstAbsence = 'both';
      first = optionalFactField(starts, counts, labels, targets, first, 'element');
    }
    while (optionalFactTag(texts, starts, counts, labels, targets, second, 'kind') === 'option') {
      const absence = optionalFactTag(texts, starts, counts, labels, targets, second, 'absence');
      if (absence !== secondAbsence) secondAbsence = 'both';
      second = optionalFactField(starts, counts, labels, targets, second, 'element');
    }
    return firstAbsence === secondAbsence ? [1, first, second] : [];
  }
  if (kind === 'array' || kind === 'set' || kind === 'cell') return [1, optionalFactField(starts, counts, labels, targets, left, 'element'), optionalFactField(starts, counts, labels, targets, right, 'element')];
  if (kind === 'map') return [1,
    optionalFactField(starts, counts, labels, targets, left, 'key'), optionalFactField(starts, counts, labels, targets, right, 'key'),
    optionalFactField(starts, counts, labels, targets, left, 'value'), optionalFactField(starts, counts, labels, targets, right, 'value')];
  let pairs = [1];
  if (kind === 'tuple' || kind === 'union' || kind === 'function') {
    const field = kind === 'tuple' ? 'elements' : kind === 'union' ? 'alternatives' : 'params';
    const first = optionalFactField(starts, counts, labels, targets, left, field);
    const second = optionalFactField(starts, counts, labels, targets, right, field);
    const length = readRequiredNumberFact(counts, first);
    if (length !== readRequiredNumberFact(counts, second)) return [];
    let at = 0;
    while (at < length) {
      pairs = pairs.concat([optionalFactElement(starts, counts, targets, first, at), optionalFactElement(starts, counts, targets, second, at)]);
      at += 1;
    }
    if (kind === 'function') pairs = pairs.concat([optionalFactField(starts, counts, labels, targets, left, 'ret'), optionalFactField(starts, counts, labels, targets, right, 'ret')]);
  }
  if (kind === 'record') {
    if ((optionalFactTag(texts, starts, counts, labels, targets, left, 'open') === 'true') !== (optionalFactTag(texts, starts, counts, labels, targets, right, 'open') === 'true')) return [];
    const first = optionalFactField(starts, counts, labels, targets, left, 'fields');
    const second = optionalFactField(starts, counts, labels, targets, right, 'fields');
    const length = readRequiredNumberFact(counts, first);
    if (length !== readRequiredNumberFact(counts, second)) return [];
    let at = 0;
    while (at < length) {
      const field = optionalFactElement(starts, counts, targets, first, at);
      const name = optionalFactTag(texts, starts, counts, labels, targets, field, 'name');
      let ordinal = 0;
      let prior = 0;
      while (prior < at) {
        if (optionalFactTag(texts, starts, counts, labels, targets, optionalFactElement(starts, counts, targets, first, prior), 'name') === name) ordinal += 1;
        prior += 1;
      }
      let candidate = 0;
      let matching = -1;
      while (candidate < length && matching < 0) {
        const other = optionalFactElement(starts, counts, targets, second, candidate);
        if (optionalFactTag(texts, starts, counts, labels, targets, other, 'name') === name) {
          if (ordinal < 1) matching = other;
          else ordinal -= 1;
        }
        candidate += 1;
      }
      if (matching < 0) return [];
      pairs = pairs.concat([optionalFactField(starts, counts, labels, targets, field, 'type'), optionalFactField(starts, counts, labels, targets, matching, 'type')]);
      at += 1;
    }
  }
  return pairs;
}


/** An inferred semantic result may widen absence while retaining the declared
 * value types. This uses the same optional pair order and logical identities;
 * an unrelated retained annotation is never accepted as descriptive metadata.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} actual
 * @param {number} declared @returns {number[]} */
export function planOptionalAnnotationConstraints(texts, starts, counts, labels, targets, actual, declared) {
  const optional = planOptionalTypeConstraints(texts, starts, counts, labels, targets, actual, declared);
  if (optional.length > 0) return [1, readRequiredNumberFact(optional, 0), readRequiredNumberFact(optional, 1)];
  const first = optionalFactTag(texts, starts, counts, labels, targets, actual, 'kind');
  const second = optionalFactTag(texts, starts, counts, labels, targets, declared, 'kind');
  if ((first === 'int' || first === 'nat' || first === 'fixed') && (second === 'int' || second === 'nat' || second === 'fixed')) return [1];
  return planInferenceTermIdentity(texts, starts, counts, labels, targets, actual, declared);
}
