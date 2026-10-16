// Small portable decisions; native output must be generated from this source.

/** @param {string} mode @returns {boolean} */
export function acceptGrammarSelectionMode(mode) {
  return mode === 'any-grammar' || mode === 'all-grammar';
}

/** @param {string} mapping @returns {boolean} */
export function acceptGrammarSelectionMapping(mapping) {
  return mapping === 'identity';
}

/** All explicitly means conjunction over the same delimited source region.
 * Any retains every admitted witness and uses the first as the primary CST.
 * @param {string} mode @param {number} accepted @param {number} total @returns {boolean}
 */
export function acceptGrammarSelectionCount(mode, accepted, total) {
  if (total <= 0 || accepted <= 0) return false;
  if (mode === 'all-grammar') return accepted === total;
  return mode === 'any-grammar';
}

/** @param {string} mode @param {number} accepted @returns {boolean} */
export function reportGrammarSelectionAmbiguity(mode, accepted) {
  return mode === 'any-grammar' && accepted > 1;
}

/** Observable header fields; structural comparisons keep their historic
 * whitespace ownership rule, while a concrete output compares every field.
 * @param {string} kind @param {boolean} output @returns {string}
 */
export function readGrammarTreeEqualityFields(kind, output) {
  if (output && kind === 'missing') return 'type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical|literal';
  if (output) return 'type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical';
  if (kind === 'node') return 'type|kind|projectionName|projectionAnonymous';
  if (kind === 'embed') return 'type|kind|projectionName|projectionAnonymous|start|end|language';
  return 'type|kind|projectionName|projectionAnonymous|start|end';
}

/** Embedding always compares the complete nested CST and every witness,
 * even inside a structural comparison that ignores enclosing node trivia.
 * @param {string} kind @param {boolean} output @returns {string}
 */
export function readGrammarTreeRecursionPlan(kind, output) {
  if (kind === 'embed') return 'full-embedding';
  if (kind === 'node') return output ? 'full-children' : 'meaningful-children';
  return 'leaf';
}

/** Roots use the recursive tree plan; program pointers are private transport,
 * and never stand in for concrete tree or source-provenance equality.
 * @param {boolean} alternative @returns {string}
 */
export function readGrammarSelectionEqualityFields(alternative) {
  return alternative ? 'language|provenance' : 'mode|mapping|start|end|candidates|provenance|ambiguous';
}
