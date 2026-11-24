/** @param {string} method @param {boolean} ordinary @returns {boolean} */
export function acceptRustStringByteViewCandidate(method,ordinary){return ordinary&&(method==='as_bytes'||method==='into_bytes'||method==='bytes');}
/** @param {string} method @param {number} count @param {string} kind
 * @param {string} storage @param {number} layers @param {boolean} known
 * @param {boolean} references @returns {string} */
export function readRustStringByteViewError(method,count,kind,storage,layers,known,references){
 if(count!==0)return 'a standard string byte view needs no arguments';
 if(!known||kind!=='string'||!references||!(storage==='owned string'&&layers>=0||storage==='string slice'&&layers>=1))return 'a byte view needs the actual standard String or str source owner';
 return method==='into_bytes'&&!(storage==='owned string'&&layers===0)?'consuming bytes need an actual owned standard String':'';
}
/** @param {string} method @returns {string} */
export function readRustStringByteViewStorage(method){return method==='into_bytes'?'owned vector':'immutable slice';}
/** @param {string} method @returns {number} */
export function readRustStringByteViewLayers(method){return method==='into_bytes'?0:1;}
/** @param {string} method @param {boolean} proven @returns {string} */
export function readRustStringByteIteratorError(method,proven){return method==='bytes'&&!proven?'byte iteration needs its actual source string view':'';}

/** A scalar-source kernel can require operational units locally in a program.
 * @param {string} language @param {string} kind @param {string} operation
 * @param {string} receiver @returns {boolean} */
export function acceptRustStringByteKernelUnitRequirement(language,kind,operation,receiver){return language==='Rust'&&kind==='method'&&operation==='codePointAt'&&receiver==='string';}
/** Logical Rust scalar strings remain scalar when a kernel selects unit storage.
 * @param {string} target @param {string} value @param {boolean} units @returns {string} */
export function renderRustStringByteScalarBridge(target,value,units){return units&&target==='Rust'?'format!("{}", '+value+')':value;}

/** @param {string} target @param {boolean} units @returns {boolean} */
export function acceptRustStringByteScalarBridge(target,units){return !units||target==='JavaScript'||target==='Rust'||target==='Lean'||target==='Rocq';}
