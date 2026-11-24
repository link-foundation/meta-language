/** A read-only slice may borrow a direct field of the sole constructor of an
 * immutable nominal value. All coordinates/owners come from the checked IR.
 * No effectful scrutinee, optional field, shared mutable array, computed body
 * or incomplete constructor directory is rewritten.
 * @param {string} result @param {boolean} reference @param {string} receiver
 * @param {number} constructors @param {number} cases @param {boolean} owner
 * @param {number} fields @param {number} bindings @param {number} slot
 * @param {number} matches @param {boolean} direct @returns {boolean} */
export function acceptNominalArrayProjectionLoan(result,reference,receiver,constructors,cases,owner,fields,bindings,slot,matches,direct){
  return result === 'array' && !reference && receiver === 'var' && constructors === 1 && cases === 1
    && owner && Number.isSafeInteger(fields) && fields > 0 && fields <= 65536 && bindings === fields
    && Number.isSafeInteger(slot) && slot >= 0 && slot < fields && matches === 1 && direct;
}

/** Names and constructor layout are emitted by the ordinary checked emitter.
 * Rust match ergonomics borrow the selected Vec; no value or field is moved.
 * @param {string} receiver @param {string} pattern @param {string} binding @returns {string} */
export function renderNominalArrayProjectionLoan(receiver,pattern,binding){
  return '(match &' + receiver + ' { ' + pattern + ' => ' + binding + '.as_slice() })';
}
