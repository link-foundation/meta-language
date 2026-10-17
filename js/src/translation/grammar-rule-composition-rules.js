// Closed authoring decisions. Actual grammar nodes, source mappings and source
// witnesses remain owned by the existing grammar adapters and runtime.

/** @param {string} role @returns {string} */
export function readGrammarCompositionFieldOrder(role) {
  if (role === 'selection') return 'kind|item|items';
  if (role === 'slot') return 'kind';
  if (role === 'json-candidate') return 'kind|name|mapping';
  if (role === 'field-candidate') return 'kind|name|mapping|selector';
  return '';
}

/** The first authoring profile wraps syntactic normal rules only. The empty
 * item is an authoring slot, never an executable replacement of the source.
 * @param {string} ruleKind @param {string} mode @param {string} slotKind
 * @param {boolean} treeDecorators @returns {string}
 */
export function readGrammarCompositionContextIssue(ruleKind, mode, slotKind, treeDecorators) {
  if (ruleKind !== 'normal') return 'composition requires a normal syntactic rule';
  if (mode !== 'any-grammar' && mode !== 'all-grammar') return 'composition requires any-grammar or all-grammar';
  if (slotKind !== 'empty') return 'composition requires one empty authoring slot';
  if (treeDecorators) return 'composition with executor or recovery decorators requires a source-witness proof';
  return '';
}

/** @param {string} kind @param {string} mapping @returns {boolean} */
export function acceptGrammarCompositionCandidateProfile(kind, mapping) {
  return (kind === 'grammar-field-candidate' && mapping === 'field-ranges')
    || (kind === 'grammar-candidate' && mapping === 'json-string');
}

/** These columns come from exact existing candidate/selector forms. A selector
 * key is their canonical Links expression after that existing shape check.
 * @param {string[]} kinds @param {string[]} mappings @param {string[]} names
 * @param {string[]} selectors @returns {string}
 */
export function readGrammarCompositionCandidateIssue(kinds, mappings, names, selectors) {
  if (kinds.length === 0 || kinds.length !== mappings.length
      || kinds.length !== names.length || kinds.length !== selectors.length) {
    return 'composition requires a nonempty complete candidate list';
  }
  const mapping = mappings[0] ?? '';
  if (mapping !== 'field-ranges' && mapping !== 'json-string') return 'composition uses an unsupported authoring profile';
  const selector = selectors[0] ?? '';
  for (let index = 0; index < kinds.length; index += 1) {
    const kind = kinds[index] ?? '';
    const name = names[index] ?? '';
    const currentSelector = selectors[index] ?? '';
    if (name === '') return 'composition candidates require nonempty source names';
    if ((mappings[index] ?? '') !== mapping) return 'composition requires one common source mapping';
    if (!acceptGrammarCompositionCandidateProfile(kind, mapping)) return 'composition needs its exact candidate form';
    if (mapping === 'field-ranges') {
      if (selector === '' || currentSelector !== selector) {
        return 'composition field candidates require one common exact selector';
      }
    } else if (currentSelector !== '') {
      return 'composition JSON candidates require their exact candidate form';
    }
    for (let previous = 0; previous < index; previous += 1) {
      if ((names[previous] ?? '') === name) return 'composition candidate source names collide';
    }
  }
  return '';
}

/** A repeated request ensures the exact already-present outer wrapper. A
 * distinct existing wrapper remains the complete actual input expression.
 * Keys describe all validated ordered candidates, never just their names.
 * @param {string} actualMode @param {string} actualCandidates
 * @param {string} requestedMode @param {string} requestedCandidates
 * @returns {number}
 */
export function readGrammarCompositionDisposition(actualMode, actualCandidates, requestedMode, requestedCandidates) {
  if (actualMode === requestedMode && actualCandidates === requestedCandidates) return 1;
  return 2;
}

/** @param {boolean} mappedSources @param {boolean} treeDecorators @returns {string} */
export function readGrammarCompositionPublicationIssue(mappedSources, treeDecorators) {
  if (mappedSources && treeDecorators) return 'a mapped grammar with executor or recovery decorators requires a source-witness proof';
  return '';
}
