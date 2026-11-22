import { readRequiredNumberFact, readRequiredTextFact } from './translation/fact-column-rules.js';

/** @returns {number} */
export function readGrammarScannerObservationLimit() {
  return 256;
}

/** @returns {number} */
export function readGrammarScannerObservationByteLimit() {
  return 262144;
}

/**
 * Infers a deterministic finite scanner from exact percent-encoded trace
 * columns. Prefer longer spellings and retain observation order for ties.
 * A nonempty return value is an ordered list of original observation indices.
 * @param {string[]} modes @param {string[]} spellings @param {string[]} tokens
 * @param {string[]} nextModes @returns {number[]}
 */
export function planGrammarScannerTransitions(modes, spellings, tokens, nextModes) {
  if (modes.length === 0 || modes.length > readGrammarScannerObservationLimit()
    || modes.length !== spellings.length || modes.length !== tokens.length || modes.length !== nextModes.length) return [-1];
  /** @type {number[]} */
  const unique = [];
  for (let index = 0; index < modes.length; index += 1) {
    const mode = readRequiredTextFact(modes, index);
    const spelling = readRequiredTextFact(spellings, index);
    const token = readRequiredTextFact(tokens, index);
    const next = readRequiredTextFact(nextModes, index);
    if (mode === '' || mode === '%' || spelling === '' || spelling === '%'
      || token === '' || token === '%' || next === '' || next === '%') return [-1];
    let previous = -1;
    for (let position = 0; position < unique.length && previous < 0; position += 1) {
      const candidate = readRequiredNumberFact(unique, position);
      if (mode === readRequiredTextFact(modes, candidate) && spelling === readRequiredTextFact(spellings, candidate)) previous = candidate;
    }
    if (previous < 0) unique.push(index);
    else if (token !== readRequiredTextFact(tokens, previous) || next !== readRequiredTextFact(nextModes, previous)) return [-2];
  }
  /** @type {number[]} */
  const plan = [];
  for (let position = 0; position < unique.length; position += 1) {
    let selected = -1;
    let length = 0;
    for (let candidate = 0; candidate < unique.length; candidate += 1) {
      const index = readRequiredNumberFact(unique, candidate);
      const size = readRequiredTextFact(spellings, index).length;
      if (size > length && !plan.includes(index)) {
        selected = index;
        length = size;
      }
    }
    plan.push(selected);
  }
  return plan;
}

/** @param {number} code @returns {string} */
export function readGrammarScannerInferenceError(code) {
  if (code === -1) return 'expected bounded nonempty scanner trace observations';
  if (code === -2) return 'conflicting scanner outputs for the same state and spelling require unsupported context sensitivity';
  return '';
}

/**
 * @param {string[]} modes @param {string[]} spellings @param {string[]} tokens
 * @param {string[]} nextModes @param {number[]} plan @param {string} scanner
 * @returns {string}
 */
export function renderGrammarScannerDefinition(modes, spellings, tokens, nextModes, plan, scanner) {
  /** @type {string[]} */
  const declaredModes = ['default'];
  /** @type {string[]} */
  const declaredTokens = [];
  let declarations = '';
  let operations = '';
  for (let position = 0; position < plan.length; position += 1) {
    const index = readRequiredNumberFact(plan, position);
    const mode = readRequiredTextFact(modes, index);
    const spelling = readRequiredTextFact(spellings, index);
    const token = readRequiredTextFact(tokens, index);
    const next = readRequiredTextFact(nextModes, index);
    if (!declaredModes.includes(mode)) {
      declaredModes.push(mode);
      declarations += '(mode ' + mode + ')\n';
    }
    if (!declaredModes.includes(next)) {
      declaredModes.push(next);
      declarations += '(mode ' + next + ')\n';
    }
    if (!declaredTokens.includes(token)) declaredTokens.push(token);
    operations += '(if (all (valid ' + token + ') (equal mode (text ' + mode + ')) (next (literal '
      + spelling + '))) (then (consume (literal ' + spelling + ')) (setMode ' + next + ') (emit ' + token + '))) ';
  }
  return declarations + '(scanner ' + scanner + ' (tokens ' + declaredTokens.join(' ')
    + ') (operations ' + operations + 'fail))\n';
}
