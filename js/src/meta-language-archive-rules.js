// Shared archive vocabulary and admission. Native adapters own storage only.

/** @returns {string} */
export function readMetaLanguageArchiveLanguage() {
  return 'meta-language complete graph archive';
}

/** @param {string} role @returns {string} */
export function readMetaLanguageArchiveTerm(role) {
  if (role === 'root') return 'complete meta-language archive';
  if (role === 'group') return 'complete source network';
  return '';
}

/** Ordered schema fields are shared data, including explicit cache state.
 * @param {string} role @returns {string}
 */
export function readMetaLanguageArchiveRecordFields(role) {
  if (role === 'descriptor') return 'schemaVersion|entries|groups';
  if (role === 'group') return 'identities|nextIdentity|terms|syntax';
  if (role === 'entry') return 'identifier|kind|group|root';
  if (role === 'term') return 'term|identity';
  if (role === 'syntax') return 'concept|language|value';
  return '';
}

/** @param {string} kind @returns {boolean} */
export function acceptMetaLanguageArchiveKind(kind) {
  return kind === 'concrete syntax' || kind === 'abstract syntax' || kind === 'grammar'
    || kind === 'language configuration'
    || kind === 'global concept space' || kind === 'formal concept definition'
    || kind === 'ontology' || kind === 'taxonomy' || kind === 'resource description graph'
    || kind === 'resource description dataset' || kind === 'typed program'
    || kind === 'discovered concepts' || kind === 'decorators' || kind === 'grammar inference';
}

/** @param {number} version @returns {boolean} */
export function acceptMetaLanguageArchiveVersion(version) {
  return version === 1;
}

/** @param {number} next @param {number} maximum @returns {boolean} */
export function acceptMetaLanguageArchiveNextIdentity(next, maximum) {
  return Number.isSafeInteger(next) && next >= 1 && next > maximum;
}

/** @param {number} group @param {number} count @returns {boolean} */
export function acceptMetaLanguageArchiveEntryGroup(group, count) {
  return Number.isSafeInteger(group) && group >= 0 && group < count;
}

/** @param {boolean} error @param {boolean} containing @param {boolean} missing @returns {boolean} */
export function acceptMetaLanguageArchiveFlags(error, containing, missing) {
  return !(error || missing) || containing;
}
