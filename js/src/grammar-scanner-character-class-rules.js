import { readRequiredNumberFact, readRequiredTextFact } from './translation/fact-column-rules.js';

/**
 * Coalesce observed single-scalar spellings with identical scanner behavior.
 * Unobserved scalars are excluded, including gaps between observed endpoints.
 * @param {string[]} modes @param {string[]} spellings @param {string[]} tokens
 * @param {string[]} nextModes @param {number[]} codePoints @param {number[]} plan
 * @returns {string[]}
 */
export function planGrammarScannerCharacterClasses(modes, spellings, tokens, nextModes, codePoints, plan) {
  /** @type {number[]} */
  const grouped = [];
  /** @type {string[]} */
  const matchers = [];
  for (let position = 0; position < plan.length; position += 1) {
    const observation = readRequiredNumberFact(plan, position);
    if (grouped.includes(position)) {
      matchers.push('');
      continue;
    }
    const spelling = readRequiredTextFact(spellings, observation);
    if (readRequiredNumberFact(codePoints, observation) < 0) {
      matchers.push('(literal ' + spelling + ')');
      continue;
    }
    const mode = readRequiredTextFact(modes, observation);
    const token = readRequiredTextFact(tokens, observation);
    const nextMode = readRequiredTextFact(nextModes, observation);
    /** @type {number[]} */
    const members = [];
    for (let candidate = position; candidate < plan.length; candidate += 1) {
      const index = readRequiredNumberFact(plan, candidate);
      if (readRequiredNumberFact(codePoints, index) >= 0
        && mode === readRequiredTextFact(modes, index)
        && token === readRequiredTextFact(tokens, index)
        && nextMode === readRequiredTextFact(nextModes, index)) {
        members.push(index);
        if (candidate !== position) grouped.push(candidate);
      }
    }
    matchers.push(members.length === 1 ? '(literal ' + spelling + ')'
      : '(class plain' + renderGrammarScannerCharacterClassMembers(members, spellings, codePoints) + ')');
  }
  return matchers;
}

/** @param {number[]} members @param {string[]} spellings @param {number[]} codePoints @returns {string} */
function renderGrammarScannerCharacterClassMembers(members, spellings, codePoints) {
  /** @type {number[]} */
  const ordered = [];
  let previous = -1;
  for (let position = 0; position < members.length; position += 1) {
    let selected = -1;
    let minimum = 1114112;
    for (const member of members) {
      const scalar = readRequiredNumberFact(codePoints, member);
      if (previous < scalar && scalar < minimum) {
        selected = member;
        minimum = scalar;
      }
    }
    ordered.push(selected);
    previous = minimum;
  }
  let text = '';
  let first = 0;
  while (first < ordered.length) {
    let last = first;
    while (last + 1 < ordered.length
      && readRequiredNumberFact(codePoints, readRequiredNumberFact(ordered, last + 1))
        === readRequiredNumberFact(codePoints, readRequiredNumberFact(ordered, last)) + 1) {
      last += 1;
    }
    const lower = readRequiredTextFact(spellings, readRequiredNumberFact(ordered, first));
    const upper = readRequiredTextFact(spellings, readRequiredNumberFact(ordered, last));
    text += first === last ? ' (char ' + lower + ')' : ' (range ' + lower + ' ' + upper + ')';
    first = last + 1;
  }
  return text;
}

/**
 * Place exact multi-scalar prefixes ahead of every single-scalar predicate,
 * independently of encoded Unicode byte lengths.
 * @param {string[]} modes @param {string[]} tokens @param {string[]} nextModes
 * @param {number[]} codePoints @param {number[]} plan @param {string[]} matchers
 * @param {string} scanner @returns {string}
 */
export function renderGrammarScannerCharacterClassDefinition(modes, tokens, nextModes, codePoints, plan, matchers, scanner) {
  /** @type {string[]} */
  const declaredModes = ['default'];
  /** @type {string[]} */
  const declaredTokens = [];
  let declarations = '';
  let literals = '';
  let characters = '';
  for (let position = 0; position < plan.length; position += 1) {
    const matcher = readRequiredTextFact(matchers, position);
    if (matcher === '') continue;
    const index = readRequiredNumberFact(plan, position);
    const mode = readRequiredTextFact(modes, index);
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
    const operation = '(if (all (valid ' + token + ') (equal mode (text ' + mode + ')) (next '
      + matcher + ')) (then (consume ' + matcher + ') (setMode ' + next + ') (emit ' + token + '))) ';
    if (readRequiredNumberFact(codePoints, index) < 0) literals += operation;
    else characters += operation;
  }
  return declarations + '(scanner ' + scanner + ' (tokens ' + declaredTokens.join(' ')
    + ') (operations ' + literals + characters + 'fail))\n';
}
