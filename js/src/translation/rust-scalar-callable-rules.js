// One scalar predicate signature contract, shared by Option and Result adapters.
/** @param {string} kind @param {number} parameters @param {number} layers
 * @param {boolean} parameter @param {boolean} result @returns {boolean} */
export function acceptRustScalarPredicate(kind,parameters,layers,parameter,result){return kind==='function'&&parameters===1&&layers===0&&parameter&&result;}
