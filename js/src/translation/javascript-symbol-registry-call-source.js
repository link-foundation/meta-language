/** @param {string} operation @returns {boolean} */
export function testJavaScriptSymbolRegistryOperation(operation){return operation==='Symbol.for'||operation==='Symbol.keyFor';}
/** Coercion is the ordinary implicit text protocol, not String(Symbol).
 * @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptSymbolRegistryFunction(identity,realm,operation){
 if(!testJavaScriptSymbolRegistryOperation(operation))return undefined;
 const instructions=[{$:'argument',destination:1,index:0}];if(operation==='Symbol.for')instructions.push({$:'primitive',destination:1,operation:'convert primitive text',arguments:[1]});instructions.push({$:'symbolRegistry',destination:2,input:1,operation:operation==='Symbol.for'?'for':'keyFor'},{$:'return',source:2});
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:3,captures:[],instructions};
}
