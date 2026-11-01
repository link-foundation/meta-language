// Standard mutation bodies are checked shared bytecode, never host Array calls.
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayMutationOperation(operation){return operation==='Array.push'||operation==='Array.pop';}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptArrayMutationFunction(identity,realm,operation){
 if(!testJavaScriptArrayMutationOperation(operation))return undefined;
 /** @type {JavaScriptCallInstruction[]} */const instructions=[
  {$:'primitive',destination:11,operation:'require array method object',arguments:[0]},
  {$:'constant',destination:1,value:{$:'text',units:[108,101,110,103,116,104]}},{$:'ordinaryGet',destination:2,object:0,key:1,receiver:0},
  {$:'primitive',destination:2,operation:'coerce primitive number',arguments:[2]},{$:'primitive',destination:2,operation:'read array like length',arguments:[2]}
 ];
 if(operation==='Array.push'){
  instructions.push({$:'argumentCount',destination:3},{$:'primitive',destination:9,operation:'read checked array append length',arguments:[2,3]},{$:'constant',destination:4,value:{$:'number',value:0}},{$:'constant',destination:8,value:{$:'number',value:1}});
  const start=instructions.length;instructions.push({$:'numberBinary',destination:10,left:4,right:3,operator:'lessThan'});const branch=instructions.length;instructions.push({$:'branch',condition:10,yes:branch+1,no:0});
  instructions.push({$:'argumentAt',destination:5,index:4},{$:'propertyKey',destination:6,input:2,realm},{$:'ordinarySet',destination:7,object:0,key:6,value:5,receiver:0},{$:'requireWrite',success:7},{$:'numberBinary',destination:2,left:2,right:8,operator:'add'},{$:'numberBinary',destination:4,left:4,right:8,operator:'add'},{$:'jump',target:start});
  const end=instructions.length;instructions[branch]={$:'branch',condition:10,yes:branch+1,no:end};instructions.push({$:'ordinarySet',destination:7,object:0,key:1,value:9,receiver:0},{$:'requireWrite',success:7},{$:'return',source:9});
 }else{
  instructions.push({$:'constant',destination:5,value:{$:'undefined'}},{$:'constant',destination:8,value:{$:'number',value:1}},{$:'constant',destination:3,value:{$:'number',value:0}},{$:'numberBinary',destination:10,left:2,right:3,operator:'equal'});
  const branch=instructions.length;instructions.push({$:'branch',condition:10,yes:0,no:branch+1},{$:'numberBinary',destination:2,left:2,right:8,operator:'subtract'},{$:'propertyKey',destination:6,input:2,realm},{$:'ordinaryGet',destination:5,object:0,key:6,receiver:0},{$:'ordinaryDelete',destination:7,object:0,key:6},{$:'primitive',destination:11,operation:'require delete success',arguments:[7]});
  const finish=instructions.length;instructions[branch]={$:'branch',condition:10,yes:finish,no:branch+1};instructions.push({$:'ordinarySet',destination:7,object:0,key:1,value:2,receiver:0},{$:'requireWrite',success:7},{$:'return',source:5});
 }
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:12,captures:[],instructions};
}
