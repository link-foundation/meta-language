// Shared admission and selection of an explicit program string representation.
/** @param {string} kind @param {boolean} present @returns {boolean} */
export function acceptProgramStringRepresentationFlag(kind, present) {
  return !present || kind === 'boolean';
}

/** @param {boolean} program @param {boolean} explicit @param {boolean} required @returns {boolean} */
export function selectProgramStringRepresentation(program, explicit, required) {
  return program || explicit || required;
}
