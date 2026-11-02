import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {bigint} identity @param {bigint} realm
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @param {JavaScriptArraySpeciesDirectory|undefined} species
 * @param {JavaScriptSemanticValue|undefined} spreadableKey
 * @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArrayConcatenationFunction(identity,realm,roots,species,spreadableKey){
 if(species===undefined||species.realm!==realm||spreadableKey?.$!=='symbol')return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];if(!appendJavaScriptObjectCoercion(code,0,1,2,3,4,5,roots))return undefined;
 code.push({$:'constant',destination:6,value:spreadableKey},{$:'arrayConcatenation',destination:7,input:1,spreadableKey:6,species},{$:'return',source:7});return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:8,captures:[],instructions:code};
}
