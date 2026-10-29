// Standard dense immutable iterator reductions reuse the common array algebra.
/** @param {string} method @param {number} count @param {boolean} ordinary @returns {boolean} */
export function acceptRustIteratorReductionStart(method,count,ordinary){return ordinary&&method==='iter'&&count===0;}
/** @param {string} method @returns {boolean} */
export function acceptRustIteratorCopyAdapter(method){return method==='copied'||method==='cloned';}
/** @param {string} method @returns {string} */
export function readRustIteratorReductionOperation(method){return method==='all'?'every':method==='any'?'some':method==='fold'?'reduce':'';}
/** @param {string} method @returns {number} */
export function readRustIteratorReductionCallbackArity(method){return method==='fold'?2:method==='all'||method==='any'?1:-1;}
/** @param {string} method @returns {string[]} */
export function readRustIteratorReductionCaptureRoles(method){return method==='fold'?['receiver','initial','callback']:method==='all'||method==='any'?['receiver','callback']:[];}
/** @param {boolean} receiver @param {boolean} owner @returns {string} */
export function readRustIteratorReductionReceiverError(receiver,owner){return receiver&&owner?'':'a reduction needs an actual standard immutable scalar array and its complete primitive element owner';}
/** @param {boolean} known @param {boolean} copy @param {number} layers @returns {string} */
export function readRustIteratorReductionAccumulatorError(known,copy,layers){return known&&copy&&layers===0?'':'fold needs the actual direct immutable Copy accumulator type';}
/** @param {string} consumer @param {string} kind @param {number} count
 * @param {number} layers @param {boolean[]} parameters @param {boolean} result
 * @returns {string} */
export function readRustIteratorReductionCallbackError(consumer,kind,count,layers,parameters,result){
 const expected=readRustIteratorReductionCallbackArity(consumer);
 if(expected<0||kind!=='function'||count!==expected||layers!==0||parameters.length!==expected||!result)return 'a reduction needs its complete direct scalar callable source signature';
 for(const parameter of parameters)if(!parameter)return 'a reduction needs its complete direct scalar callable source signature';
 return '';
}
