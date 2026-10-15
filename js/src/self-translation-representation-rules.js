// Generated from the shared self-translation representation policy. Do not edit.
/** @param {string} name @returns {boolean} */
export function acceptSelfTranslationUtf16Member(name) {
  return name === "split" || name === "slice" || name === "indexOf" || name === "lastIndexOf" || name === "replace" || name === "replaceAll" || name === "codePointAt" || name === "repeat" || name === "padStart" || name === "padEnd";
}

/** @param {boolean} explicit @param {string} family @param {boolean} member @returns {boolean} */
export function requireSelfTranslationUtf16Representation(explicit, family, member) {
  return explicit || (family === 'JavaScript' && member);
}

/** @param {string} kind @returns {boolean} */
export function requireSelfTranslationUtf16Text(kind) {
  return kind === 'string';
}
