// JavaScript constant widths specialize the existing complete modular plan.
// This does not admit new Rust primitive integer types.
import {planFixedIntegerCastExpression} from './fixed-integer-cast-rules.js';

/** ToIndex on the represented constant Number, with a separate construction
 * profile. Unsupported widths refuse rather than receiving a fake result.
 * @param {number} bits @param {boolean} signed @returns {string[]} */
export function planJavaScriptFixedIntegerCastExpression(bits,signed){
 const width=Number.isNaN(bits)?0:Math.trunc(bits);
 if(!Number.isFinite(width)||width<0||width>4096)return [];
 let modulus=1n;
 for(let index=0;index<width;index+=1)modulus*=2n;
 const modulusText=String(modulus),limitText=String(modulus/2n);
 const plan=planFixedIntegerCastExpression(8,signed&&width!==0);
 return plan.map((cell,index)=>index%5===1&&plan[index-1]==='integer'?(cell==='256'?modulusText:cell==='128'?limitText:cell):cell);
}

/** Capture the value and ignored operands before the modular operation.
 * Width syntax is separately restricted to a pure Number constant.
 * The preceding two-argument construction keeps its original surface shape.
 * @param {number} count @returns {number[]} */
export function planJavaScriptFixedIntegerArguments(count){
 if(!Number.isInteger(count)||count<3||count>4096)return [];
 /** @type {number[]} */ const coordinates=[];
 for(let index=1;index<count;index+=1)coordinates.push(index);
 return coordinates;
}
