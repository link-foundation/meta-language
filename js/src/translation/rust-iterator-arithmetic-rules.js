// Standard numeric sum/product semantics. The current pointer model is 64-bit.
/** @param {string} consumer @returns {boolean} */
export function acceptRustIteratorArithmeticConsumer(consumer){return consumer==='sum'||consumer==='product';}
/** @param {string} kind @param {boolean} known @param {boolean} owner
 * @param {number} layers @param {boolean} resultKnown @param {number} resultLayers
 * @param {boolean} sameType @param {boolean} sameOwner @returns {string} */
export function readRustIteratorArithmeticError(kind,known,owner,layers,resultKnown,resultLayers,sameType,sameOwner){return (kind==='fixed'||kind==='float')&&known&&owner&&(layers===0||layers===1)&&resultKnown&&resultLayers===0&&sameType&&sameOwner?'':'sum and product need one standard numeric item owner and the matching direct result type';}
/** A structured literal transports sign separately, including floating -0.
 * @param {string} consumer @param {string} kind @returns {string[]} */
export function planRustIteratorArithmeticSeed(consumer,kind){return consumer==='sum'?(kind==='float'?['0','negative']:['0','positive']):consumer==='product'?['1','positive']:[];}
/** @param {string} consumer @returns {string[]} */
export function planRustIteratorArithmeticTerminal(consumer){return consumer==='sum'?['accumulator','item','add']:consumer==='product'?['accumulator','item','multiply']:[];}
