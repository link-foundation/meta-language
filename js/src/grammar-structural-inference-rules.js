import { readRequiredNumberFact } from './translation/fact-column-rules.js';

/**
 * Split a token sequence into observed productions. Group zero owns the
 * whole sequence. Within a production, nonnegative entries are terminal
 * token indexes and negative entries reference their delimiter group.
 * Open and close tokens each belong to at most one positive group.
 * @param {number[]} tokens @param {number[]} openingGroups
 * @param {number[]} closingGroups @param {number} depthLimit
 * @returns {number[][]}
 */
export function readGrammarStructuralProductions(tokens, openingGroups, closingGroups, depthLimit) {
  /** @type {number[][]} */
  const rows = [];
  const groups = [0];
  const starts = [0];
  let symbols = [0];
  for (const token of tokens) {
    const depth = groups.length - 1;
    if (token < 0 || token >= openingGroups.length || token >= closingGroups.length) return [[-1]];
    const opening = readRequiredNumberFact(openingGroups, token);
    const closing = readRequiredNumberFact(closingGroups, token);
    if (closing > 0) {
      if (closing !== readRequiredNumberFact(groups, depth)) return [[-1]];
      const start = readRequiredNumberFact(starts, depth);
      rows.push(symbols.slice(start));
      symbols = symbols.slice(0, start);
      groups.pop();
      starts.pop();
    } else if (opening > 0) {
      if (depth >= depthLimit) return [[-2]];
      symbols.push(-opening);
      starts.push(symbols.length);
      groups.push(opening);
      symbols.push(opening);
    } else {
      symbols.push(token);
    }
  }
  if (groups.length !== 1) return [[-1]];
  rows.push(symbols);
  return rows;
}

/**
 * Compare production keys without coercing tokens to text or losing order.
 * @param {number[]} left @param {number[]} right @returns {boolean}
 */
export function testSameGrammarStructuralProduction(left, right) {
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index += 1) {
    if (readRequiredNumberFact(left, index) !== readRequiredNumberFact(right, index)) return false;
  }
  return true;
}

/**
 * Infer mutually recursive alternatives by retaining the first completed observation
 * of each delimiter production. Reject the candidate if it accepts a supplied
 * negative. A depth refusal on either label remains a resource refusal.
 * @param {number[][]} positives @param {number[][]} negatives
 * @param {number[]} openingGroups @param {number[]} closingGroups
 * @param {number} depthLimit @param {number} productionLimit
 * @returns {number[][]}
 */
export function inferGrammarStructuralProductions(positives, negatives, openingGroups, closingGroups, depthLimit, productionLimit) {
  /** @type {number[][]} */
  let productions = [];
  for (const tokens of positives) {
    const rows = readGrammarStructuralProductions(tokens, openingGroups, closingGroups, depthLimit);
    for (const row of rows) {
      const group = readRequiredNumberFact(row, 0);
      if (group < 0) return [row];
      if (!productions.some(previous => testSameGrammarStructuralProduction(previous, row))) {
        if (productions.length >= productionLimit) return [[-3]];
        productions = productions.concat([row]);
      }
    }
  }
  for (const tokens of negatives) {
    const rows = readGrammarStructuralProductions(tokens, openingGroups, closingGroups, depthLimit);
    let accepted = true;
    for (const row of rows) {
      const group = readRequiredNumberFact(row, 0);
      if (group === -2) return [row];
      if (group < 0 || !productions.some(previous => testSameGrammarStructuralProduction(previous, row))) accepted = false;
    }
    if (accepted) return [[-4]];
  }
  return productions;
}
