import {acceptRustScalarPredicate} from './rust-scalar-callable-rules.js';
import {testRustOptionPanicConsumer} from './rust-panic-value-rules.js';
import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustCheckedIntegerArity(method,ordinary){
 if(!ordinary)return -1;
 return ['checked_add','checked_sub','checked_mul','checked_div','checked_rem','checked_div_euclid','checked_rem_euclid','checked_shl','checked_shr'].includes(method)?1:['checked_neg','checked_abs'].includes(method)?0:-1;
}
/** @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustCheckedIntegerReceiver(method,kind,bits,signed,owner,path,children,proven,layers){return readRustCheckedIntegerArity(method,true)>=0&&kind==='fixed'&&(method!=='checked_abs'||signed)&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** @param {string} consumer @returns {number} */
export function readRustCheckedIntegerConsumerArity(consumer){return consumer==='is_some'||consumer==='is_none'||consumer==='unwrap_or_default'||consumer==='unwrap'?0:consumer==='unwrap_or'||consumer==='expect'||testRustCheckedIntegerLazyFallback(consumer)||testRustCheckedIntegerPredicateConsumer(consumer)?1:consumer==='map_or'?2:-1;}
/** Actual standard checked shift counts are u32, independently of Self.
 * @param {string} method @param {string} receiver @returns {string} */
export function readRustCheckedIntegerArgumentOwner(method,receiver){return method==='checked_shl'||method==='checked_shr'?'u32':receiver;}
/** @param {string} method @returns {string} */
export function readRustCheckedIntegerArgumentReason(method){return method==='checked_shl'||method==='checked_shr'?'the actual shift count must retain standard u32 ownership':'the actual argument must retain the receiver type and source owner';}
/** @param {string} term @returns {string} */
export function readRustCheckedIntegerShiftOperation(term){return term==='shift left'?'shiftRustPrimitiveBitsLeft':term==='shift right'?'shiftRustPrimitiveBitsRight':'';}
/** Guard ASTs retain lazy conditional boundaries around each threshold division
 * and every potentially overflowing threshold. No larger integer is required.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustCheckedIntegerCondition(method,signed){
 if(method==='checked_shl'||method==='checked_shr')return ['right','width','lt'];
 if(method==='checked_neg')return signed?['operand','minimum','ne']:['operand','zero','eq'];
 if(method==='checked_abs')return signed?['operand','minimum','ne']:[];
 if(method==='checked_add')return signed?[
  'right','zero','gt','operand','maximum','right','sub','le',
  'right','zero','lt','operand','minimum','right','sub','ge','true','if','if',
 ]:['operand','maximum','right','sub','le'];
 if(method==='checked_sub')return signed?[
  'right','zero','lt','operand','maximum','right','add','le',
  'right','zero','gt','operand','minimum','right','add','ge','true','if','if',
 ]:['operand','right','ge'];
 if(method==='checked_mul'){
  const positivePositive=['operand','maximum','right','div','le'];
  if(!signed)return ['operand','zero','eq','true','right','zero','eq','true'].concat(positivePositive,['if','if']);
  const positiveNegative=['right','minimum','operand','div','ge'];
  const negativePositive=['operand','minimum','right','div','ge'];
  const negativeNegative=['operand','maximum','right','div','ge'];
  const signs=['operand','zero','gt','right','zero','gt'].concat(positivePositive,positiveNegative,['if','right','zero','gt'],negativePositive,negativeNegative,['if','if']);
  return ['operand','zero','eq','true','right','zero','eq','true'].concat(signs,['if','if']);
 }
 if(method==='checked_div'||method==='checked_rem'||method==='checked_div_euclid'||method==='checked_rem_euclid')return signed?['right','zero','eq','false','operand','minimum','eq','right','negative unit','ne','true','if','if']:['right','zero','ne'];
 return [];
}
/** Successful arithmetic appears only within its complete condition's branch.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustCheckedIntegerSuccess(method,signed){
 if(method==='checked_div_euclid'||method==='checked_rem_euclid'){
  const quotient=['operand','right','div'],remainder=['operand','right','rem'];
  if(!signed)return method==='checked_div_euclid'?quotient:remainder;
  // Avoid abs(MIN), which cannot be represented in the signed source domain.
  const positive=method==='checked_div_euclid'?quotient.concat(['negative unit','add']):remainder.concat(['right','add']);
  const negative=method==='checked_div_euclid'?quotient.concat(['negative unit','sub']):remainder.concat(['right','sub']);
  return remainder.concat(['zero','lt','right','zero','gt'],positive,negative,['if'],method==='checked_div_euclid'?quotient:remainder,['if']);
 }
 if(method==='checked_shl'||method==='checked_shr')return ['operand','right',method==='checked_shl'?'shift left':'shift right'];
 if(method==='checked_neg')return signed?['operand','neg']:['zero'];
 if(method==='checked_abs')return signed?['operand','zero','lt','operand','neg','operand','if']:[];
 const operation=method==='checked_add'?'add':method==='checked_sub'?'sub':method==='checked_mul'?'mul':method==='checked_div'?'div':method==='checked_rem'?'rem':'';
 return operation?['operand','right',operation]:[];
}
/** @param {string} consumer @returns {string[]} */
export function planRustCheckedIntegerConsumer(consumer){return testRustCheckedIntegerPredicateConsumer(consumer)?['condition','fallback','success','call one',consumer==='is_none_or'?'true':'false','if']:consumer==='is_some'?['condition']:consumer==='is_none'?['condition','not']:consumer==='unwrap'?['condition','success','unwrap message','raise','if']:consumer==='expect'?['condition','success','fallback','raise','if']:consumer==='unwrap_or_default'?['condition','success','default','if']:testRustCheckedIntegerLazyFallback(consumer)?['condition','success','fallback','call zero','if']:consumer==='unwrap_or'||consumer==='map_or'?['condition','success','fallback','if']:[];}
/** @param {string} consumer @param {string} receiver @param {string} mapper @returns {string} */
export function readRustCheckedIntegerResultOwner(consumer,receiver,mapper){return consumer==='is_some'||consumer==='is_none'||testRustCheckedIntegerPredicateConsumer(consumer)?'bool':consumer==='unwrap_or'||consumer==='unwrap_or_default'||testRustOptionPanicConsumer(consumer)||testRustCheckedIntegerLazyFallback(consumer)?receiver:consumer==='map_or'?mapper:'';}

/** @param {string} consumer @returns {boolean} */
export function testRustCheckedIntegerLazyFallback(consumer){return consumer==='unwrap_or_else';}
/** The callback result must keep the receiver's actual standard scalar owner.
 * @param {string} kind @param {number} parameters @param {number} layers
 * @param {string} result @param {string} bits @param {boolean} signed
 * @param {string} ownerKind @param {string} owner @param {number} children
 * @param {boolean} proven @param {number} resultLayers
 * @param {string} expectedBits @param {boolean} expectedSigned
 * @param {string} expectedOwner @returns {string} */
export function readRustCheckedIntegerFallbackError(kind,parameters,layers,result,bits,signed,ownerKind,owner,children,proven,resultLayers,expectedBits,expectedSigned,expectedOwner){return kind==='function'&&parameters===0&&layers===0&&result==='fixed'&&bits===expectedBits&&signed===expectedSigned&&owner===expectedOwner&&acceptRustPrimitiveSelectionReceiver(result,bits,signed,ownerKind,owner,children,proven,resultLayers)?'':'a checked integer fallback needs a direct zero-argument callable with the actual receiver result owner';}

/** @param {string} consumer @returns {boolean} */
export function testRustCheckedIntegerPredicateConsumer(consumer){return consumer==='is_some_and'||consumer==='is_none_or';}
/** Standard Option and Result predicates have the same actual scalar contract.
 * @param {string} kind @param {number} parameters @param {number} layers
 * @param {boolean} parameter @param {boolean} result @returns {string} */
export function readRustCheckedIntegerPredicateError(kind,parameters,layers,parameter,result){return acceptRustScalarPredicate(kind,parameters,layers,parameter,result)?'':'a checked integer predicate needs a direct one-argument callable with its actual integer parameter and Boolean result';}
