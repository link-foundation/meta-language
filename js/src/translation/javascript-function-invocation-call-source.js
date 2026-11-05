// Protected call/apply bodies build specification Lists in frame-owned slots.
// Array-like apply inputs use Get/ToLength, never the iterable spread protocol.
/** @param {string} operation @returns {boolean} */
export function testJavaScriptFunctionInvocationOperation(operation){return ['Function.call','Function.apply','Function.bind','Function.toString','Function.restricted property','Function.prototype','Reflect.apply'].includes(operation);}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptFunctionInvocationFunction(identity,realm,operation){
 if(!testJavaScriptFunctionInvocationOperation(operation))return undefined;
 if(operation==='Function.prototype')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'constant',destination:1,value:{$:'undefined'}},{$:'return',source:1}]};
 if(operation==='Function.bind')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'functionBind',destination:1},{$:'return',source:1}]};
 if(operation==='Function.toString')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'functionToString',destination:1},{$:'return',source:1}]};
 if(operation==='Function.restricted property')return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'primitive',destination:1,operation:'throw restricted Function access',arguments:[]},{$:'return',source:1}]};
 /** @type {JavaScriptCallInstruction[]} */const instructions=[];
 // 1=callee, 2=thisArgument, 3=arrayLike/count, 4=index, 5=length,
 // 6=key/condition, 7=selected value, 8=one, 9=return value.
 if(operation==='Reflect.apply')instructions.push({$:'argument',destination:1,index:0},{$:'argument',destination:2,index:1},{$:'argument',destination:3,index:2});
 else {instructions.push({$:'copy',destination:1,source:0},{$:'argument',destination:2,index:0});if(operation==='Function.apply')instructions.push({$:'argument',destination:3,index:1});}
 instructions.push({$:'primitive',destination:1,operation:'require callable target',arguments:[1]},{$:'argumentListOpen',slot:0},{$:'constant',destination:8,value:{$:'number',value:1}});
 if(operation==='Function.call'){
  instructions.push({$:'argumentCount',destination:5},{$:'constant',destination:4,value:{$:'number',value:1}});
  const loop=instructions.length;instructions.push({$:'numberBinary',destination:6,left:4,right:5,operator:'lessThan'});const branch=instructions.length;instructions.push({$:'branch',condition:6,yes:branch+1,no:0},{$:'argumentAt',destination:7,index:4},{$:'argumentListAppend',slot:0,value:7},{$:'numberBinary',destination:4,left:4,right:8,operator:'add'},{$:'jump',target:loop});
  const call=instructions.length;instructions[branch]={$:'branch',condition:6,yes:branch+1,no:call};instructions.push({$:'argumentListCall',destination:9,slot:0,callee:1,receiver:2,mode:'call'},{$:'return',source:9});
 }else{
  let nullBranch=-1,undefinedBranch=-1;
  if(operation==='Function.apply'){
   instructions.push({$:'constant',destination:6,value:{$:'null'}},{$:'strictEqual',destination:6,left:3,right:6});nullBranch=instructions.length;instructions.push({$:'branch',condition:6,yes:0,no:nullBranch+1},{$:'constant',destination:6,value:{$:'undefined'}},{$:'strictEqual',destination:6,left:3,right:6});undefinedBranch=instructions.length;instructions.push({$:'branch',condition:6,yes:0,no:undefinedBranch+1});
  }
  instructions.push({$:'primitive',destination:3,operation:'require array like arguments Object',arguments:[3]},{$:'constant',destination:6,value:{$:'text',units:[108,101,110,103,116,104]}},{$:'ordinaryGet',destination:5,object:3,key:6,receiver:3},{$:'primitive',destination:5,operation:'coerce primitive number',arguments:[5]},{$:'primitive',destination:5,operation:'read array like length',arguments:[5]},{$:'constant',destination:4,value:{$:'number',value:0}});
  const loop=instructions.length;instructions.push({$:'numberBinary',destination:6,left:4,right:5,operator:'lessThan'});const branch=instructions.length;instructions.push({$:'branch',condition:6,yes:branch+1,no:0},{$:'propertyKey',destination:6,input:4,realm},{$:'ordinaryGet',destination:7,object:3,key:6,receiver:3},{$:'argumentListAppend',slot:0,value:7},{$:'numberBinary',destination:4,left:4,right:8,operator:'add'},{$:'jump',target:loop});
  const call=instructions.length;instructions[branch]={$:'branch',condition:6,yes:branch+1,no:call};if(nullBranch>=0)instructions[nullBranch]={$:'branch',condition:6,yes:call,no:nullBranch+1};if(undefinedBranch>=0)instructions[undefinedBranch]={$:'branch',condition:6,yes:call,no:undefinedBranch+1};instructions.push({$:'argumentListCall',destination:9,slot:0,callee:1,receiver:2,mode:'call'},{$:'return',source:9});
 }
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:10,captures:[],instructions};
}
