// Shared project worklist decisions. Native hosts supply actual source and
// compiler results; no source stub or behavioral observation is manufactured.

/** @param {string} status @param {boolean} sameFamily @returns {boolean} */
export function acceptProjectTranslationItem(status, sameFamily) {
  return status === 'translated' || status === 'comment' || status === 'provenance'
    || sameFamily && status === 'kept';
}

/** Names are kept in source-inventory order, with one next visit per dependent.
 * @param {string[]} identifiers @param {string[]} changed
 * @param {string[]} importers @param {string[]} providers @returns {string[]}
 */
export function readProjectTranslationFrontier(identifiers, changed, importers, providers) {
  const active = new Set(changed), selected = [];
  for (let edge = 0; edge < importers.length; edge += 1) {
    const provider = providers[edge], importer = importers[edge];
    if (provider !== undefined && importer !== undefined && active.has(provider ?? '')) selected.push(importer ?? '');
  }
  const affected = new Set(selected);
  const next = [];
  for (const identifier of identifiers) if (affected.has(identifier)) next.push(identifier);
  return next;
}

/** @param {string} source @param {string} target @returns {boolean} */
export function acceptProjectTranslationSourceFamily(source, target) {
  return source === target || source === 'JavaScript' && target === 'TypeScript';
}

/** Source spelling order, including UTF-16 surrogate ordering, is retained by
 * the ordinary shared string-order operation in the native translation.
 * @param {string} first @param {string} second @returns {number}
 */
export function compareProjectTranslationIdentifiers(first, second) {
  return first < second ? -1 : first > second ? 1 : 0;
}
