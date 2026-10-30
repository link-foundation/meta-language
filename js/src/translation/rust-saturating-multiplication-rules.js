import {acceptRustPrimitiveSelectionReceiver} from './rust-primitive-selection-rules.js';
/** @param {string} method @param {boolean} ordinary @returns {boolean} */
export function acceptRustSaturatingMultiplicationMethod(method,ordinary){return ordinary&&method==='saturating_mul';}
/** @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string} owner @param {string} path @param {number} children
 * @param {boolean} proven @param {number} layers @returns {boolean} */
export function acceptRustSaturatingMultiplicationReceiver(kind,bits,signed,owner,path,children,proven,layers){return kind==='fixed'&&acceptRustPrimitiveSelectionReceiver(kind,bits,signed,owner,path,children,proven,layers);}
/** Conditional AST terms guard every divisor and product. MIN is divided
 * only by a positive value; MAX may be divided by a negative nonzero value.
 * Rust's integer division truncates toward zero in each threshold.
 * @param {boolean} signed @returns {string[]} */
export function planRustSaturatingMultiplication(signed){
 const product=['operand','right','mul'];
 const positivePositive=['operand','maximum','right','div','gt','maximum'].concat(product,['if']);
 if(!signed)return ['operand','zero','eq','zero','right','zero','eq','zero'].concat(positivePositive,['if','if']);
 const positiveNegative=['right','minimum','operand','div','lt','minimum'].concat(product,['if']);
 const negativePositive=['operand','minimum','right','div','lt','minimum'].concat(product,['if']);
 const negativeNegative=['operand','maximum','right','div','lt','maximum'].concat(product,['if']);
 const signs=['operand','zero','gt','right','zero','gt'].concat(positivePositive,positiveNegative,['if','right','zero','gt'],negativePositive,negativeNegative,['if','if']);
 return ['operand','zero','eq','zero','right','zero','eq','zero'].concat(signs,['if','if']);
}
