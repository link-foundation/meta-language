import {readRustPrimitiveOwnerPath} from './rust-primitive-owner-path-rules.js';
/** @param {string[]} segments @param {boolean} ordinary @param {boolean} shadowed
 * @param {boolean} absolute @returns {string} */
export function readRustPrimitiveIntegerConstantOwner(segments,ordinary,shadowed,absolute){
 const owner=readRustPrimitiveOwnerPath(segments,ordinary,shadowed,absolute);
 return owner==='bool'?'':owner;
}
/** @param {string} name @returns {string} */
export function readRustPrimitiveIntegerConstantTerm(name){return name==='MIN'?'minimum':name==='MAX'?'maximum':name==='BITS'?'width':'';}
/** @param {string} owner @param {string} term @returns {string} */
export function readRustPrimitiveIntegerConstantResultOwner(owner,term){return term==='width'?'u32':term==='minimum'||term==='maximum'?owner:'';}
