// Shared raw surface metadata decisions. The native rules are generated from
// this complete module; field presence remains a host serialization concern.

/** @param {string} context @param {string} field @returns {boolean} */
export function acceptSurfaceRawMetadataField(context, field) {
  return context === 'expression' && field === 'parenthesized'
    || context === 'program' && field === 'referenceSemantics'
    || context === 'index' && (field === 'optional' || field === 'provenInBounds');
}

/** @param {string} form @returns {boolean} */
export function acceptSurfaceRawMetadataForm(form) {
  return form === 'boolean' || form === 'null';
}

/** @param {string} kind @returns {boolean} */
export function acceptSharedSurfaceReferenceKind(kind) {
  return kind === 'array' || kind === 'record' || kind === 'map' || kind === 'set';
}

/** @param {boolean} required @param {boolean} structural @param {boolean} enabled @returns {boolean} */
export function selectSurfaceReferenceSemantics(required, structural, enabled) {
  return required || enabled && structural;
}
