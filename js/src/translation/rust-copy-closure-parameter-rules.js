// An immutable scalar reference can be passed to a nonescaping callable body.
/** @param {boolean} known @param {boolean} baseCopy @param {number} layers
 * @param {boolean} reference @param {boolean} primitiveOwner @returns {boolean} */
export function acceptRustCopyClosureParameterSource(known,baseCopy,layers,reference,primitiveOwner){
 return known&&baseCopy&&(layers===0||Number.isInteger(layers)&&layers>=1&&layers<=256&&reference&&primitiveOwner);
}

/** @param {boolean} written @param {boolean} parametersOnly @returns {string} */
export function readRustCopyClosureResultContext(written,parametersOnly){return written?'written':parametersOnly?'inferred':'expected';}

/** @param {string} layer @returns {boolean} */
export function testRustSourceReferenceLifetimeLayer(layer){return layer==='reference';}
