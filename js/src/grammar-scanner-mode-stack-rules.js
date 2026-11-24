import { readRequiredNumberFact, readRequiredTextFact } from './translation/fact-column-rules.js';

/** @returns {number} */
export function readGrammarScannerModeStackLimit() {
  return 32;
}

/** Infer one native mode operation by comparing complete, ordered stacks.
 * The caller supplies validated, percent-encoded mode names.
 * @param {string[]} before @param {string[]} after @returns {string}
 */
export function readGrammarScannerModeStackOperation(before, after) {
  if (before.length === 0 || after.length === 0
    || before.length > readGrammarScannerModeStackLimit()
    || after.length > readGrammarScannerModeStackLimit()) return '';
  const shared = Math.min(before.length, after.length);
  if (after.length > before.length + 1 || before.length > after.length + 1) return '';
  const equalDepth = before.length === after.length;
  const prefix = equalDepth ? shared - 1 : shared;
  for (let index = 0; index < prefix; index += 1) {
    if (readRequiredTextFact(before, index) !== readRequiredTextFact(after, index)) return '';
  }
  if (after.length < before.length) return '(popMode)';
  const next = readRequiredTextFact(after, after.length - 1);
  if (after.length > before.length) return '(pushMode ' + next + ')';
  if (next === readRequiredTextFact(before, before.length - 1)) return 'keep';
  return '(setMode ' + next + ')';
}

/** Place exact multi-scalar prefixes before finite single-scalar matches.
 * @param {string[]} modes @param {string[]} spellings @param {string[]} tokens
 * @param {string[]} effects @param {string[]} observedModes Unique modes in observation order.
 * @param {number[]} codePoints
 * @param {number[]} plan @param {string[]} matchers @param {string} scanner @returns {string}
 */
export function renderGrammarScannerModeStackDefinition(modes, spellings, tokens, effects, observedModes, codePoints, plan, matchers, scanner) {
  /** @type {string[]} */
  const declaredTokens = [];
  let declarations = '';
  let literals = '';
  let characters = '';
  for (const mode of observedModes) {
    if (mode !== 'default') declarations += '(mode ' + mode + ')\n';
  }
  for (let position = 0; position < plan.length; position += 1) {
    const index = readRequiredNumberFact(plan, position);
    const matcher = matchers.length === 0 ? '(literal ' + readRequiredTextFact(spellings, index) + ')'
      : readRequiredTextFact(matchers, position);
    if (matcher === '') continue;
    const mode = readRequiredTextFact(modes, index);
    const token = readRequiredTextFact(tokens, index);
    const effect = readRequiredTextFact(effects, index);
    if (!declaredTokens.includes(token)) declaredTokens.push(token);
    const operation = '(if (all (valid ' + token + ') (equal mode (text ' + mode + ')) (next '
      + matcher + ')) (then (consume ' + matcher + ') '
      + (effect === 'keep' ? '' : effect + ' ') + '(emit ' + token + '))) ';
    if (readRequiredNumberFact(codePoints, index) < 0) literals += operation;
    else characters += operation;
  }
  return declarations + '(scanner ' + scanner + ' (tokens ' + declaredTokens.join(' ')
    + ') (operations ' + literals + characters + 'fail))\n';
}
