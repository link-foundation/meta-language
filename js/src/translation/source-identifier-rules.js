// Source identifiers, generated binders and runtime helpers occupy disjoint
// names. The host transports actual names and a numerically sorted ordinal
// column; all namespace selection and target spelling decisions are shared.
/** @param {string} name @returns {boolean} */
export function acceptSourcePrivateIdentifier(name) { return name.startsWith('ml_'); }
/** @param {string} name @returns {number} */
export function readSourceIdentifierPrefixOrdinal(name) {
  if (!name.startsWith('ml_source')) return -1;
  let index = 9, ordinal = 0;
  if (name.charCodeAt(index) < 49 || name.charCodeAt(index) > 57 || index >= name.length) return -1;
  while (index < name.length) {
    const unit = name.charCodeAt(index);
    if (unit < 48 || unit > 57) break;
    ordinal = ordinal * 10 + unit - 48;
    if (!Number.isSafeInteger(ordinal)) return -1;
    index += 1;
  }
  return name.charCodeAt(index) === 95 ? ordinal : -1;
}
/** @param {boolean} privateNames @param {number[]} ordinals @returns {string} */
export function readSourceIdentifierPrefix(privateNames, ordinals) {
  if (!privateNames) return 'ml_';
  let next = 1, previous = 0;
  for (let index = 0; index < ordinals.length; index += 1) {
    const ordinal = ordinals[index] ?? -1;
    if (!Number.isSafeInteger(ordinal) || ordinal < 1 || ordinal < previous)
      throw new Error('source identifier ordinals need a sorted positive integer column');
    previous = ordinal;
    if (ordinal === next) next += 1;
  }
  return 'ml_source' + next + '_';
}
/** @param {string} name @returns {string} */
export function renderSourceTargetIdentifier(name) {
  return acceptSourcePrivateIdentifier(name) ? 'source_' + name : name;
}
/** @param {string} prefix @param {string} role @param {number} ordinal @returns {string} */
export function renderSourceGeneratedIdentifier(prefix, role, ordinal) {
  if (!Number.isSafeInteger(ordinal) || ordinal < 0) throw new Error('source identifier generation ordinal exceeds its exact range');
  return prefix + (role.startsWith('ml_') ? role.slice(3) : role) + ordinal;
}
/** @param {string} prefix @param {string} name @param {boolean} reference @returns {string} */
export function renderSourceCellParameter(prefix, name, reference) {
  return reference ? prefix + 'cell_' + name : name;
}

/** Ordinary BoundName admission after the actual token's lexical role check.
 * Closed historical profiles remain separate operations.
 * @param {string} kind @param {string} name @param {boolean} duplicate
 * @param {string[]} reserved @returns {boolean} */
export function acceptJavaScriptSourceBindingName(kind, name, duplicate, reserved) {
  if (kind !== 'identifier' || name === '' || duplicate) return false;
  for (let index = 0; index < reserved.length; index += 1) {
    const word = reserved[index];
    if (word === undefined) throw new Error('source binding reserved names need a complete text column');
    if (word === name) return false;
  }
  return true;
}
