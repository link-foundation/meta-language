import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {bigint} identity @param {bigint} realm
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptArraySpeciesDirectory|undefined} species
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArraySliceFunction(identity,realm,roots,species){
 if(species===undefined||species.realm!==realm)return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 code.push({$:'arraySlice',destination:6,input:1,species},{$:'return',source:6});return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:7,captures:[],instructions:code};
}
