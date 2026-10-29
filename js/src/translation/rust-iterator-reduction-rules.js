// Standard dense immutable iterator reductions reuse the common array algebra.
/** @param {string} method @param {number} count @param {boolean} ordinary @returns {boolean} */
export function acceptRustIteratorReductionStart(method,count,ordinary){return ordinary&&method==='iter'&&count===0;}
/** @param {string} method @returns {boolean} */
export function acceptRustIteratorCopyAdapter(method){return method==='copied'||method==='cloned';}
/** @param {string} method @returns {string} */
export function readRustIteratorReductionOperation(method){return method==='all'?'every':method==='any'?'some':(method==='fold'||method==='rfold')?'reduce':'';}
/** @param {string} method @returns {number} */
export function readRustIteratorReductionCallbackArity(method){return (method==='fold'||method==='rfold')?2:method==='all'||method==='any'?1:-1;}
/** @param {string} method @returns {string[]} */
export function readRustIteratorReductionCaptureRoles(method){return (method==='fold'||method==='rfold')?['receiver','initial','callback']:method==='all'||method==='any'?['receiver','callback']:[];}
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

/** @param {string} method @returns {boolean} */
export function acceptRustIteratorReductionAdapter(method){return method==='rev'||acceptRustIteratorCopyAdapter(method);}
/** State transitions retain actual source item indirection; rev never copies.
 * @param {string} method @param {string} state @param {string} direction @returns {string[]} */
export function readRustIteratorReductionAdapterState(method,state,direction){
 if(state!=='borrowed'&&state!=='copied'||direction!=='forward'&&direction!=='reverse')return [];
 if(method==='rev')return [state,direction==='forward'?'reverse':'forward'];
 if(acceptRustIteratorCopyAdapter(method)&&state==='borrowed')return ['copied',direction];
 return [];
}
/** Reversal applies to a fresh scalar copy, never the actual source receiver.
 * @param {string} method @param {string} direction @returns {string[]} */
export function planRustIteratorReductionReceiver(method,direction){
 return (method==='rfold')!==(direction==='reverse')?['map reversed indices']:[];
}

/** Complete postfix common-term plan; indices originate from the dense map.
 * @returns {string[]} */
export function planRustIteratorReductionReverseIndex(){return ['receiver','receiver','length','index','sub','one','sub','natural','lookup'];}
