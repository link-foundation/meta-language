// The complete portable admission module. The existing checker supplies the
// numeric conversion semantics after projecting the actual optional child.
/** @param {string} kind @param {string} operation @param {string} field @returns {boolean} */
export function acceptProgramDerivedNumberOperand(kind, operation, field) {
  return kind === 'binary' && (field === 'left' || field === 'right')
    && (operation === 'add' || operation === 'sub' || operation === 'mul' || operation === 'div' || operation === 'rem');
}
