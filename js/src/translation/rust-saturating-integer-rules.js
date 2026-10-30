import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
import {readFixedIntegerCastModulus,readFixedIntegerCastSignedLimit} from './fixed-integer-cast-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {number} */
export function readRustSaturatingIntegerArity(method,ordinary){
 if(!ordinary)return -1;
 if(method==='saturating_add'||method==='saturating_sub')return 1;
 return method==='saturating_neg'||method==='saturating_abs'?0:-1;
}
/** @param {string} method @param {string} kind @param {string} bits
 * @param {boolean} signed @param {string} owner @param {string} path
 * @param {number} children @param {boolean} proven @param {number} layers
 * @returns {boolean} */
export function acceptRustSaturatingIntegerReceiver(method,kind,bits,signed,owner,path,children,proven,layers){
 return readRustSaturatingIntegerArity(method,true)>=0&&kind==='fixed'&&(signed||method==='saturating_add'||method==='saturating_sub')&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);
}
/** Exact decimal predecessor for an admitted power-of-two bound. No integer
 * bound passes through binary64; each step changes one actual ASCII digit.
 * @param {string} source @returns {string} */
function readRustIntegerBoundPredecessor(source){
 if(source==='')return '';
 let end=source.length;
 while(end>0&&source.charCodeAt(end-1)===48)end-=1;
 if(end===0)return '';
 let result=source.slice(0,end-1)+String.fromCharCode(source.charCodeAt(end-1)-1);
 for(let index=end;index<source.length;index+=1)result+='9';
 return result;
}
/** Actual positive literal magnitude and its separate sign enter source ASTs.
 * @param {string} bound @param {number} bits @param {boolean} signed
 * @returns {string} */
export function readRustSaturatingIntegerBoundMagnitude(bound,bits,signed){
 const limit=signed?readFixedIntegerCastSignedLimit(bits):readFixedIntegerCastModulus(bits);
 if(limit==='')return '';
 if(bound==='minimum')return signed?limit:'0';
 return bound==='maximum'?readRustIntegerBoundPredecessor(limit):'';
}
/** @param {string} bound @param {boolean} signed @returns {boolean} */
export function acceptRustSaturatingIntegerBoundNegative(bound,signed){return bound==='minimum'&&signed;}
/** Postfix source AST plan; conditionals dominate every potentially overflowing
 * arithmetic operation. Both actual operands are captured before this plan.
 * @param {string} method @param {boolean} signed @returns {string[]} */
export function planRustSaturatingInteger(method,signed){
 if(method==='saturating_neg'&&signed)return ['operand','minimum','eq','maximum','operand','neg','if'];
 if(method==='saturating_abs'&&signed)return ['operand','minimum','eq','maximum','operand','zero','lt','operand','neg','operand','if','if'];
 if(method==='saturating_add'&&!signed)return ['operand','maximum','right','sub','gt','maximum','operand','right','add','if'];
 if(method==='saturating_sub'&&!signed)return ['operand','right','lt','zero','operand','right','sub','if'];
 if(method==='saturating_add'&&signed)return [
  'right','zero','gt',
  'operand','maximum','right','sub','gt','maximum','operand','right','add','if',
  'right','zero','lt',
  'operand','minimum','right','sub','lt','minimum','operand','right','add','if',
  'operand','if','if',
 ];
 if(method==='saturating_sub'&&signed)return [
  'right','zero','lt',
  'operand','maximum','right','add','gt','maximum','operand','right','sub','if',
  'right','zero','gt',
  'operand','minimum','right','add','lt','minimum','operand','right','sub','if',
  'operand','if','if',
 ];
 return [];
}
