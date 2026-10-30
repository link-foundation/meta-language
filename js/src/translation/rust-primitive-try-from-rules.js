import {readRustIntegerConversionErrorDebugModel} from './rust-source-version-rules.js';
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
export function readRustPrimitiveTryFromConsumerArity(method){return method==='is_ok'||method==='is_err'||method==='unwrap_or_default'||method==='unwrap'?0:method==='unwrap_or'||method==='expect'||method==='is_ok_and'?1:method==='map_or'?2:-1;}
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
 const failure=method==='unwrap'?['panic prefix'].concat(planRustPrimitiveTryFromFailureDebug(guards),['concat','raise']):method==='expect'?['message','panic prefix'].concat(planRustPrimitiveTryFromFailureDebug(guards),['concat','concat','raise']):[];
 return testRustPrimitiveTryFromPanicConsumer(method)?condition.concat(['success'],failure,['if']):method==='is_ok'?condition:method==='is_err'?condition.concat(['not']):method==='is_ok_and'?condition.concat(['predicate','false','if']):condition.concat(['success',method==='unwrap_or_default'?'default':'fallback','if']);
}
/** @param {string} method @param {string} target @param {string} mapper @returns {string} */
export function readRustPrimitiveTryFromResultOwner(method,target,mapper){return method==='is_ok'||method==='is_err'||method==='is_ok_and'?'bool':method==='unwrap_or'||method==='unwrap_or_default'||testRustPrimitiveTryFromPanicConsumer(method)?target:method==='map_or'?mapper:'';}

/** @param {string} consumer @returns {boolean} */
export function testRustPrimitiveTryFromPanicConsumer(consumer){return consumer==='unwrap'||consumer==='expect';}
/** Debug payloads changed in Rust1.97; this snapshot matches CI Rust1.99.
 * @param {string} direction @returns {string} */
export function readRustPrimitiveTryFromFailureDebug(direction){return readRustPrimitiveTryFromVersionedFailureDebug(direction,99);}
/** @param {number} guards @returns {string[]} */
export function planRustPrimitiveTryFromFailureDebug(guards){if(![0,1,2,3].includes(guards))return [];return guards===1||guards===3?['input','minimum','ge','positive error','negative error','if']:['positive error'];}
/** @param {string} consumer @returns {string} */
export function readRustPrimitiveTryFromPanicPrefix(consumer){return consumer==='unwrap'?'called `Result::unwrap()` on an `Err` value: ':consumer==='expect'?': ':'';}

/** @returns {string} */
export function readRustPrimitiveTryFromPanicVersionRange(){return ">=1.97.0 <1.100.0";}

/** The choice is resolved in common terms before target emission.
 * @param {string} direction @param {number} minor @returns {string} */
export function readRustPrimitiveTryFromVersionedFailureDebug(direction,minor){
 if(direction!=='negative error'&&direction!=='positive error')return '';
 const model=readRustIntegerConversionErrorDebugModel(minor);
 return model==='unit payload'?'TryFromIntError(())':model==='overflow kind'?(direction==='negative error'?'TryFromIntError(NegOverflow)':'TryFromIntError(PosOverflow)'):'';
}

/** Actual callback signature, not its spelling, determines admission.
 * @param {string} kind @param {number} parameters @param {number} layers
 * @param {boolean} parameter @param {boolean} result @returns {string} */
export function readRustPrimitiveTryFromPredicateError(kind,parameters,layers,parameter,result){return kind==='function'&&parameters===1&&layers===0&&parameter&&result?'':'a Result predicate needs a direct one-argument callable with its actual integer parameter and Boolean result';}

/** @param {string} kind @param {number} parameters @param {number} layers
 * @param {boolean} parameter @param {boolean} result @returns {string} */
export function readRustPrimitiveTryFromMapperError(kind,parameters,layers,parameter,result){return kind==='function'&&parameters===1&&layers===0&&parameter&&result?'':'a Result mapper needs a direct one-argument callable with its actual integer parameter and scalar result';}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} ownerKind @param {string} owner @param {number} children
 * @param {boolean} proven @param {number} layers @returns {string} */
export function readRustPrimitiveTryFromMapperOwner(kind,bits,signed,ownerKind,owner,children,proven,layers){return acceptRustPrimitiveFromSource(kind,bits,signed,ownerKind,owner,children,proven,layers)?owner:'';}
