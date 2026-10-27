import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustCheckedIntegerArity(method,ordinary){
 if(!ordinary)return -1;
 return ['checked_add','checked_sub','checked_mul','checked_div','checked_rem'].includes(method)?1:['checked_neg','checked_abs'].includes(method)?0:-1;
}
/** @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustCheckedIntegerReceiver(method,kind,bits,signed,owner,path,children,proven,layers){return readRustCheckedIntegerArity(method,true)>=0&&kind==='fixed'&&(method!=='checked_abs'||signed)&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** @param {string} consumer @returns {number} */
export function readRustCheckedIntegerConsumerArity(consumer){return consumer==='is_some'||consumer==='is_none'||consumer==='unwrap_or_default'?0:consumer==='unwrap_or'?1:consumer==='map_or'?2:-1;}
/** Guard ASTs retain lazy conditional boundaries around each threshold division
 * and every potentially overflowing threshold. No larger integer is required.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustCheckedIntegerCondition(method,signed){
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
 if(method==='checked_div'||method==='checked_rem')return signed?['right','zero','eq','false','operand','minimum','eq','right','negative unit','ne','true','if','if']:['right','zero','ne'];
 return [];
}
/** Successful arithmetic appears only within its complete condition's branch.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustCheckedIntegerSuccess(method,signed){
 if(method==='checked_neg')return signed?['operand','neg']:['zero'];
 if(method==='checked_abs')return signed?['operand','zero','lt','operand','neg','operand','if']:[];
 const operation=method==='checked_add'?'add':method==='checked_sub'?'sub':method==='checked_mul'?'mul':method==='checked_div'?'div':method==='checked_rem'?'rem':'';
 return operation?['operand','right',operation]:[];
}
/** @param {string} consumer @returns {string[]} */
export function planRustCheckedIntegerConsumer(consumer){return consumer==='is_some'?['condition']:consumer==='is_none'?['condition','not']:consumer==='unwrap_or_default'?['condition','success','default','if']:consumer==='unwrap_or'||consumer==='map_or'?['condition','success','fallback','if']:[];}
/** @param {string} consumer @param {string} receiver @param {string} mapper @returns {string} */
export function readRustCheckedIntegerResultOwner(consumer,receiver,mapper){return consumer==='is_some'||consumer==='is_none'?'bool':consumer==='unwrap_or'||consumer==='unwrap_or_default'?receiver:consumer==='map_or'?mapper:'';}
