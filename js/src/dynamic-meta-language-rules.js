// Shared stable directory roles and concrete snapshot interpretation.
/** @param {string} role @returns {string} */
export function readDynamicMetaLanguageIdentifier(role) {
  if (role === 'definitions') return 'language definitions';
  if (role === 'configuration') return 'language configuration';
  if (role === 'definition snapshot') return 'language definition snapshot ';
  if (role === 'configuration snapshot') return 'language configuration snapshot ';
  return '';
}
/** @param {string} projection @returns {boolean} */
export function acceptDynamicSyntaxProjection(projection) {
  return projection === 'concrete syntax' || projection === 'abstract syntax';
}
/** @returns {number} */
export function readDynamicConcreteSyntaxVersion() { return 2; }

/** @returns {string} */
export function readDynamicParserOptionOrder() {
  return 'startRule|maxDepth|stepLimit|memoLimit|memoryLimit|ambiguity|recovery|errorRecovery|maxRepairs|decorators';
}
/** @param {string} name @returns {boolean} */
export function acceptDynamicParserOption(name) {
  return name === 'startRule' || name === 'maxDepth' || name === 'stepLimit'
    || name === 'memoLimit' || name === 'memoryLimit' || name === 'ambiguity'
    || name === 'recovery' || name === 'errorRecovery' || name === 'maxRepairs' || name === 'decorators';
}

/** @param {string[]} identifiers @param {string} prefix @returns {string} */
export function readDynamicSnapshotIdentifier(identifiers, prefix) {
  let ordinal = 1;
  while (identifiers.includes(prefix + ordinal)) ordinal += 1;
  return prefix + ordinal;
}
/** @param {string[]} identifiers @param {string} source @returns {string} */
export function readDynamicConcreteIdentifier(identifiers, source) {
  const prefix = 'concrete syntax:' + source;
  let selected = prefix;
  let ordinal = 2;
  while (identifiers.includes(selected)) {
    selected = prefix + ':' + ordinal;
    ordinal += 1;
  }
  return selected;
}

/** @param {string} concrete @returns {string} */
export function readDynamicAbstractIdentifier(concrete) {
  return 'abstract syntax:' + concrete;
}
