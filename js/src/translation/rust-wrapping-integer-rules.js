import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustWrappingIntegerArity(method,ordinary){
 if(!ordinary)return -1;
 if(method==='wrapping_add'||method==='wrapping_sub')return 1;
 return method==='wrapping_neg'||method==='wrapping_abs'?0:-1;
}
/** @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustWrappingIntegerReceiver(method,kind,bits,signed,owner,path,children,proven,layers){return readRustWrappingIntegerArity(method,true)>=0&&kind==='fixed'&&(signed||method!=='wrapping_abs')&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** Exact modular source terms. Overflow branches reconstruct the wrapped
 * result entirely from representable operands; no wider integer is assumed.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustWrappingInteger(method,signed){
 if(method==='wrapping_neg')return signed?['operand','minimum','eq','minimum','operand','neg','if']:['operand','zero','eq','zero','maximum','operand','sub','positive unit','add','if'];
 if(method==='wrapping_abs'&&signed)return ['operand','minimum','eq','minimum','operand','zero','lt','operand','neg','operand','if','if'];
 if(method==='wrapping_add'){
  const normal=['operand','right','add'];
  const upper=signed?['minimum','operand','maximum','right','sub','sub','positive unit','sub','add']:['operand','maximum','right','sub','sub','positive unit','sub'];
  const positive=['operand','maximum','right','sub','gt'].concat(upper,normal,['if']);
  if(!signed)return positive;
  const lower=['maximum','operand','minimum','right','sub','sub','positive unit','add','add'];
  const negative=['operand','minimum','right','sub','lt'].concat(lower,normal,['if']);
  return ['right','zero','gt'].concat(positive,['right','zero','lt'],negative,['operand','if','if']);
 }
 if(method==='wrapping_sub'){
  const normal=['operand','right','sub'];
  if(!signed)return ['operand','right','lt','maximum','right','operand','sub','sub','positive unit','add'].concat(normal,['if']);
  const upper=['minimum','operand','positive unit','sub','maximum','right','add','sub','add'];
  const negative=['operand','maximum','right','add','gt'].concat(upper,normal,['if']);
  const lower=['maximum','operand','minimum','right','add','sub','positive unit','add','add'];
  const positive=['operand','minimum','right','add','lt'].concat(lower,normal,['if']);
  return ['right','zero','lt'].concat(negative,['right','zero','gt'],positive,['operand','if','if']);
 }
 return [];
}
