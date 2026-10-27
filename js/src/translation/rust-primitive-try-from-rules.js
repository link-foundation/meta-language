import {readRustPrimitiveIntegerConstantOwner} from './rust-primitive-integer-constant-rules.js';
import {acceptRustPrimitiveFromSource,acceptRustPrimitiveFromMethod} from './rust-primitive-from-rules.js';
/** @param {string[]} segments @param {boolean} ordinary @param {boolean} shadowed
 * @param {boolean} absolute @returns {string} */
export function readRustPrimitiveTryFromOwner(segments,ordinary,shadowed,absolute){return readRustPrimitiveIntegerConstantOwner(segments,ordinary,shadowed,absolute);}
/** @param {string} path @param {boolean} ordinary @param {boolean} shadowed @returns {boolean} */
export function acceptRustPrimitiveTryFromImport(path,ordinary,shadowed){return ordinary&&!shadowed&&(path==='std::convert::TryFrom'||path==='core::convert::TryFrom');}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustPrimitiveTryFromSource(kind,bits,signed,owner,path,children,proven,layers){return kind==='fixed'&&acceptRustPrimitiveFromMethod(path,'from')&&acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,children,proven,layers);}
/** @param {string} method @returns {number} */
export function readRustPrimitiveTryFromConsumerArity(method){return method==='is_ok'||method==='is_err'?0:method==='unwrap_or'?1:method==='map_or'?2:-1;}
/** Exact range tests; construct only bounds representable in the input domain.
 * @param {number} sourceBits @param {boolean} sourceSigned
 * @param {number} targetBits @param {boolean} targetSigned @returns {number} */
export function readRustPrimitiveTryFromGuards(sourceBits,sourceSigned,targetBits,targetSigned){
 if(![8,16,32,64,128].includes(sourceBits)||![8,16,32,64,128].includes(targetBits))return -1;
 const lower=sourceSigned&&(!targetSigned||sourceBits>targetBits);
 const upper=targetSigned?(sourceSigned?sourceBits>targetBits:sourceBits>=targetBits):sourceBits>targetBits;
 return (lower?1:0)+(upper?2:0);
}
/** Immediate standard Result elimination only; error payloads cannot escape.
 * @param {string} method @param {number} guards @returns {string[]} */
export function planRustPrimitiveTryFromConsumer(method,guards){
 if(readRustPrimitiveTryFromConsumerArity(method)<0||![0,1,2,3].includes(guards))return [];
 const lower=['input','minimum','ge'],upper=['input','maximum','le'];
 const condition=guards===0?['true']:guards===1?lower:guards===2?upper:lower.concat(upper,['and']);
 return method==='is_ok'?condition:method==='is_err'?condition.concat(['not']):condition.concat(['success','fallback','if']);
}
/** @param {string} method @param {string} target @param {string} mapper @returns {string} */
export function readRustPrimitiveTryFromResultOwner(method,target,mapper){return method==='is_ok'||method==='is_err'?'bool':method==='unwrap_or'?target:method==='map_or'?mapper:'';}
