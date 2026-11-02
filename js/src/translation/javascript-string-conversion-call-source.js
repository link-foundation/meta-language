// String Call and Symbol methods use actual description metadata, independent
// from SetFunctionName. Symbol inputs inside object coercion stay implicit.
import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,constructJavaScriptSemanticTextUnits} from './javascript-semantic-values.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStringConversionOperation(operation){return ['Symbol','String','Symbol.toString','Symbol.valueOf','Symbol.toPrimitive','Symbol.description'].includes(operation);}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptStringConversionFunction(identity,realm,operation){
 if(!testJavaScriptStringConversionOperation(operation))return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];
 if(operation==='Symbol'){
  code.push({$:'argument',destination:1,index:0},{$:'constant',destination:2,value:{$:'undefined'}},{$:'strictEqual',destination:3,left:1,right:2},{$:'branch',condition:3,yes:5,no:4},{$:'primitive',destination:1,operation:'convert primitive text',arguments:[1]},{$:'symbolAllocate',destination:2,input:1},{$:'return',source:2});
 }else if(operation==='String'){
  code.push({$:'argumentCount',destination:1},{$:'constant',destination:2,value:{$:'number',value:0}},{$:'numberBinary',destination:3,left:1,right:2,operator:'equal'},{$:'branch',condition:3,yes:14,no:4},{$:'argument',destination:1,index:0},{$:'primitive',destination:2,operation:'read semantic type',arguments:[1]},{$:'constant',destination:3,value:{$:'text',units:[115,121,109,98,111,108]}},{$:'strictEqual',destination:4,left:2,right:3},{$:'branch',condition:4,yes:11,no:9},{$:'primitive',destination:1,operation:'convert primitive text',arguments:[1]},{$:'return',source:1},{$:'symbolDescription',destination:2,input:1},{$:'primitive',destination:1,operation:'construct Symbol descriptive text',arguments:[2]},{$:'return',source:1},{$:'constant',destination:1,value:{$:'text',units:[]}},{$:'return',source:1});
 }else{
  code.push({$:'objectObservation',destination:1,operation:'Symbol.valueOf',input:0,other:0});
  if(operation==='Symbol.valueOf'||operation==='Symbol.toPrimitive')code.push({$:'return',source:1});
  else{code.push({$:'symbolDescription',destination:2,input:1});if(operation==='Symbol.toString')code.push({$:'primitive',destination:2,operation:'construct Symbol descriptive text',arguments:[2]});code.push({$:'return',source:2});}
 }
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:5,captures:[],instructions:code};
}
/** Description is actual optional text, never a Symbol-to-string fallback.
 * @param {JavaScriptSemanticValue} description @returns {JavaScriptCallPrimitiveResult} */
export function constructJavaScriptSymbolDescriptiveText(description){
 const error=readJavaScriptSemanticValueError(description);if(error!==''||description.$!=='undefined'&&description.$!=='text'&&description.$!=='sourceText')return {$:'primitiveInvalid',reason:error||'Symbol descriptive text requires actual optional text metadata'};
 const length=description.$==='undefined'?0:readJavaScriptSemanticTextLength(description);
 if(length>1048568)return {$:'primitiveResourceLimit',reason:'Symbol descriptive text exceeds its selected 1048576-unit output profile'};
 const units=[83,121,109,98,111,108,40];if(description.$!=='undefined')for(const unit of constructJavaScriptSemanticTextUnits(description,1048576))units.push(unit);units.push(41);
 return {$:'primitiveValue',value:{$:'text',units}};
}
