/** @param {string} receiverOperator @param {string} member @param {string} callOperator @returns {boolean} */
export function acceptSelfTranslationRelativeArrayCall(receiverOperator, member, callOperator) {
  return receiverOperator === '.' && member === 'at' && callOperator === '(';
}
