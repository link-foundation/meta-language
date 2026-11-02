// Protected Number/Math bodies compose existing scalar and IEEE754 primitives.
/** @returns {string[]} */
export function readJavaScriptNumberPredicateNames(){return ['isNaN','isFinite','isInteger','isSafeInteger'];}
/** @returns {string[]} */
export function readJavaScriptMathMethodNames(){return ['clz32','imul','abs','floor','ceil','trunc','round','sign','sqrt','pow','min','max'];}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptNumericIntrinsicOperation(operation){return ['Number','Boolean','BigInt','BigInt.asIntN','BigInt.asUintN','BigInt.toString','Number.parseInt','Number.parseFloat','isNaN','isFinite'].includes(operation)||readJavaScriptNumberPredicateNames().some(name=>operation==='Number.'+name)||readJavaScriptMathMethodNames().some(name=>operation==='Math.'+name);}
/** @param {bigint} identity @param {bigint} realm @param {string} operation @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptNumericIntrinsicFunction(identity,realm,operation){
 if(!testJavaScriptNumericIntrinsicOperation(operation))return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[];
 if(operation==='BigInt.toString'){
  code.push({$:'objectObservation',destination:1,operation:'BigInt.valueOf',input:0,other:0},{$:'argument',destination:2,index:0},{$:'constant',destination:3,value:{$:'undefined'}},{$:'strictEqual',destination:4,left:2,right:3},{$:'branch',condition:4,yes:5,no:7},{$:'constant',destination:2,value:{$:'number',value:10}},{$:'jump',target:8},{$:'primitive',destination:2,operation:'coerce primitive number',arguments:[2]},{$:'primitive',destination:5,operation:'convert integer radix text',arguments:[1,2]},{$:'return',source:5});
 }else if(operation==='Number.parseInt'||operation==='Number.parseFloat'){
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'convert primitive text',arguments:[1]});
  if(operation==='Number.parseInt')code.push({$:'argument',destination:2,index:1},{$:'primitive',destination:2,operation:'coerce primitive number',arguments:[2]},{$:'primitive',destination:1,operation:'parse integer text prefix',arguments:[1,2]});
  else code.push({$:'primitive',destination:1,operation:'parse float text prefix',arguments:[1]});
  code.push({$:'return',source:1});
 }else if(operation==='isNaN'||operation==='isFinite'){
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'coerce primitive number',arguments:[1]},{$:'primitive',destination:1,operation:operation==='isNaN'?'test NaN number':'test finite number',arguments:[1]},{$:'return',source:1});
 }else if(operation==='Boolean'){
  code.push({$:'argument',destination:1,index:0},{$:'branch',condition:1,yes:2,no:4},{$:'constant',destination:2,value:{$:'boolean',value:true}},{$:'return',source:2},{$:'constant',destination:2,value:{$:'boolean',value:false}},{$:'return',source:2});
 }else if(operation==='BigInt'){
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'convert explicit integer',arguments:[1]},{$:'return',source:1});
 }else if(operation==='BigInt.asIntN'||operation==='BigInt.asUintN'){
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'coerce primitive number',arguments:[1]},{$:'primitive',destination:1,operation:'read integer index',arguments:[1]},{$:'argument',destination:2,index:1},{$:'primitive',destination:2,operation:'coerce primitive integer',arguments:[2]},{$:'primitive',destination:3,operation:operation==='BigInt.asIntN'?'wrap signed integer':'wrap unsigned integer',arguments:[1,2]},{$:'return',source:3});
 }else if(operation==='Number'){
  code.push({$:'argumentCount',destination:1},{$:'constant',destination:2,value:{$:'number',value:0}},{$:'numberBinary',destination:3,left:1,right:2,operator:'equal'},{$:'branch',condition:3,yes:7,no:4},{$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'convert explicit number',arguments:[1]},{$:'return',source:1},{$:'return',source:2});
 }else if(operation.startsWith('Number.')){
  const primitive=operation==='Number.isNaN'?'test NaN number':operation==='Number.isFinite'?'test finite number':operation==='Number.isInteger'?'test integer number':'test safe integer number';
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:2,operation:'read semantic type',arguments:[1]},{$:'constant',destination:3,value:{$:'text',units:[110,117,109,98,101,114]}},{$:'strictEqual',destination:4,left:2,right:3},{$:'branch',condition:4,yes:5,no:7},{$:'primitive',destination:5,operation:primitive,arguments:[1]},{$:'return',source:5},{$:'constant',destination:5,value:{$:'boolean',value:false}},{$:'return',source:5});
 }else if(operation==='Math.min'||operation==='Math.max'){
  code.push({$:'constant',destination:1,value:{$:'number',value:operation==='Math.min'?Infinity:-Infinity}},{$:'argumentCount',destination:5},{$:'constant',destination:4,value:{$:'number',value:0}},{$:'constant',destination:8,value:{$:'number',value:1}});const loop=code.length;code.push({$:'numberBinary',destination:6,left:4,right:5,operator:'lessThan'});const branch=code.length;code.push({$:'branch',condition:6,yes:branch+1,no:0},{$:'argumentAt',destination:7,index:4},{$:'primitive',destination:7,operation:'coerce primitive number',arguments:[7]},{$:'primitive',destination:1,operation:operation==='Math.min'?'read minimum number':'read maximum number',arguments:[1,7]},{$:'numberBinary',destination:4,left:4,right:8,operator:'add'},{$:'jump',target:loop});const end=code.length;code[branch]={$:'branch',condition:6,yes:branch+1,no:end};code.push({$:'return',source:1});
 }else{
  code.push({$:'argument',destination:1,index:0},{$:'primitive',destination:1,operation:'coerce primitive number',arguments:[1]});
  if(operation==='Math.pow'||operation==='Math.imul')code.push({$:'argument',destination:2,index:1},{$:'primitive',destination:2,operation:'coerce primitive number',arguments:[2]},{$:'primitive',destination:1,operation:operation==='Math.pow'?'raise numeric power':'multiply signed integer bits',arguments:[1,2]});
  else {const primitive=operation==='Math.clz32'?'count leading zero bits':operation==='Math.abs'?'read absolute number':operation==='Math.floor'?'floor number':operation==='Math.ceil'?'ceil number':operation==='Math.trunc'?'truncate number':operation==='Math.round'?'round number':operation==='Math.sign'?'read number sign':'read square root';code.push({$:'primitive',destination:1,operation:primitive,arguments:[1]});}
  code.push({$:'return',source:1});
 }
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:10,captures:[],instructions:code};
}
