// Decisions for the additive direct declaration-value interpretation. The old
// selected-wrapper interpretation and the strict const proof are unchanged.
/** @param {string} language @param {boolean} originalConstProof @returns {boolean} */
export function acceptBooleanDeclarationValueContext(language, originalConstProof) {
  return language === 'JavaScript' && !originalConstProof || language === 'Rust' && originalConstProof;
}
/** @param {string} kind @param {boolean} nested @param {boolean} generated @returns {boolean} */
export function acceptBooleanDeclarationValueEntry(kind, nested, generated) {
  return kind === 'fn' && !nested && !generated;
}
/** @param {boolean} singleSelector @param {boolean} ownedDeclaration @returns {boolean} */
export function acceptBooleanDeclarationValueSelector(singleSelector, ownedDeclaration) {
  return singleSelector && ownedDeclaration;
}
