// An immutable scalar reference can be passed to a nonescaping callable body.
/** @param {boolean} known @param {boolean} baseCopy @param {number} layers
 * @param {boolean} reference @param {boolean} primitiveOwner @returns {boolean} */
export function acceptRustCopyClosureParameterSource(known,baseCopy,layers,reference,primitiveOwner){
 return known&&baseCopy&&(layers===0||layers===1&&reference&&primitiveOwner);
}
