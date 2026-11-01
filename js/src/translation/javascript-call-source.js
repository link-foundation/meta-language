import {testJavaScriptCallCodeCoordinate,readJavaScriptCallConstantError} from './javascript-call-code.js';
import {readJavaScriptCallPrimitiveArityError} from './javascript-call-primitives.js';

/** Binding identities are supplied from actual checked lexical occurrences.
 * The compiler never derives binding identity from equal source spellings.
 * @typedef {{$:'sourceConstant',value:JavaScriptSemanticValue} |
 * {$:'sourceReceiver'} | {$:'sourceArgument',index:number} |
 * {$:'sourceDerivedReceiver'} | {$:'sourceErrorIntrinsic'} |
 * {$:'sourceConstructErrorClass',callee:JavaScriptCallSource,arguments:JavaScriptCallSource[]} |
 * {$:'sourceClassConstructorCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]} |
 * {$:'sourceConstructCollection',callee:JavaScriptCallSource,arguments:JavaScriptCallSource[]} |
 * {$:'sourceConstructErrorValue',constructor:JavaScriptCallSource,newTarget:JavaScriptCallSource,prototype:JavaScriptCallSource,message:JavaScriptCallSource,present:JavaScriptCallSource,realm:bigint,errorClass:string,bindReceiver:boolean} |
 * {$:'sourceThrow',value:JavaScriptCallSource} |
 * {$:'sourceReturn',value:JavaScriptCallSource} |
 * {$:'sourceAssignBinding',identity:number,value:JavaScriptCallSource} |
 * {$:'sourceWhile',condition:JavaScriptCallSource,body:JavaScriptCallSource,after:JavaScriptCallSource} |
 * {$:'sourceTry',identity:number,body:JavaScriptCallSource,handler:JavaScriptCallSource} |
 * {$:'sourceBinding',identity:number} |
 * {$:'sourceLet',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceLocalLet',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource,mutable:boolean} |
 * {$:'sourceParameterDefault',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceIf',condition:JavaScriptCallSource,yes:JavaScriptCallSource,no:JavaScriptCallSource} |
 * {$:'sourceNumberBinary',operator:string,left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceSameValue',left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceStrictEqual',left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceSnapshotStrictEqual',left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourcePrimitive',operation:string,arguments:JavaScriptCallSource[]} |
 * {$:'sourceRecur',arguments:JavaScriptCallSource[]} |
 * {$:'sourceAllocate',prototype:JavaScriptCallSource} |
 * {$:'sourceClosure',owner:bigint,ordinal:number,captures:JavaScriptCallSource[],receiver:JavaScriptCallSource} |
 * {$:'sourceObject',prototype:JavaScriptCallSource,properties:JavaScriptCallSourceProperty[],realm:bigint} |
 * {$:'sourceArray',prototype:JavaScriptCallSource,length:number,elements:JavaScriptCallSourceArrayElement[],realm:bigint} |
 * {$:'sourcePropertyKey',input:JavaScriptCallSource,realm:bigint} |
 * {$:'sourcePropertyDefinition',target:JavaScriptCallSource,key:JavaScriptCallSource,attributes:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceSet',object:JavaScriptCallSource,key:JavaScriptCallSource,value:JavaScriptCallSource} |
 * {$:'sourceSnapshotSet',object:JavaScriptCallSource,key:JavaScriptCallSource,value:JavaScriptCallSource} |
 * {$:'sourceGet',object:JavaScriptCallSource,key:JavaScriptCallSource} |
 * {$:'sourceCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]} |
 * {$:'sourceSnapshotCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]}} JavaScriptCallSource
 */
/** @typedef {{$:'sourceObjectProperty',key:JavaScriptCallSource,value:JavaScriptCallSource}} JavaScriptCallSourceProperty */
/** @typedef {{$:'sourceArrayElement',index:number,value:JavaScriptCallSource}} JavaScriptCallSourceArrayElement */
/** @typedef {{$:'sourceBindingRegister',identity:number,register:number,mutable:boolean}} JavaScriptCallSourceBinding */
/** @typedef {{$:'sourceBuilder',instructions:JavaScriptCallInstruction[],next:number,visits:number,parameters:number,handlers:number,error:string,snapshotMutableBindings?:boolean}} JavaScriptCallSourceBuilder */
/** @typedef {{$:'sourceResult',builder:JavaScriptCallSourceBuilder,register:number}} JavaScriptCallSourceResult */
/** @typedef {{$:'sourceCompiled',instructions:JavaScriptCallInstruction[],registers:number,error:string}} JavaScriptCallSourceCompilation */

/** @param {JavaScriptCallSourceBuilder} builder @param {string} error @returns {JavaScriptCallSourceResult} */
export function refuseJavaScriptCallSource(builder,error){return {$:'sourceResult',builder:{...builder,error},register:0};}
/** @param {JavaScriptCallSourceBuilder} builder @param {JavaScriptCallInstruction} instruction @returns {JavaScriptCallSourceBuilder} */
export function appendJavaScriptCallSourceInstruction(builder,instruction){
 if(builder.error!=='')return builder;
 if(builder.instructions.length>=65536)return {...builder,error:'source lowering exceeds its instruction budget'};
 return {...builder,instructions:builder.instructions.concat([instruction])};
}
/** @param {JavaScriptCallSourceBuilder} builder @returns {JavaScriptCallSourceResult} */
export function allocateJavaScriptCallSourceRegister(builder){
 if(builder.error!=='')return {$:'sourceResult',builder,register:0};
 if(builder.next>=4096)return refuseJavaScriptCallSource(builder,'source lowering exceeds its register budget');
 return {$:'sourceResult',builder:{...builder,next:builder.next+1},register:builder.next};
}

/** Source evaluation order is instruction order. Conditional branches compile
 * into separate blocks, never eager argument evaluation of both alternatives.
 * @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth
 * @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceExpression(source,bindings,builder,depth){
 if(builder.error!=='')return {$:'sourceResult',builder,register:0};
 if(depth>128||builder.visits>=65536)return refuseJavaScriptCallSource(builder,'source lowering exceeds its structural budget');
 const state={...builder,visits:builder.visits+1};
 switch(source.$){
  case 'sourceReceiver':return {$:'sourceResult',builder:state,register:0};
  case 'sourceDerivedReceiver':case 'sourceErrorIntrinsic':{const result=allocateJavaScriptCallSourceRegister(state);return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,source.$==='sourceDerivedReceiver'?{$:'readDerivedReceiver',destination:result.register}:{$:'invokeErrorIntrinsic',destination:result.register})};}
  case 'sourceConstructErrorClass':case 'sourceConstructCollection':{
   if(source.arguments.length>4096)return refuseJavaScriptCallSource(state,'derived Error arguments exceed their explicit operand budget');
   let callee=lowerJavaScriptCallSourceExpression(source.callee,bindings,state,depth+1);if(source.$==='sourceConstructCollection'){const saved=allocateJavaScriptCallSourceRegister(callee.builder);callee={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:callee.register})};}let current=callee.builder,arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=value.builder;if(source.$==='sourceConstructCollection'){const saved=allocateJavaScriptCallSourceRegister(current);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:value.register});arguments_.push(saved.register);}else arguments_.push(value.register);}
   const result=allocateJavaScriptCallSourceRegister(current);return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:source.$==='sourceConstructCollection'?'constructCollection':'constructErrorClass',destination:result.register,callee:callee.register,arguments:arguments_})};
  }
  case 'sourceClassConstructorCall':{
   if(source.arguments.length>4096)return refuseJavaScriptCallSource(state,'ordinary class arguments exceed their explicit operand budget');
   const selected=lowerJavaScriptCallSourceExpression(source.callee,bindings,state,depth+1),callee=allocateJavaScriptCallSourceRegister(selected.builder),heldCallee=appendJavaScriptCallSourceInstruction(callee.builder,{$:'copy',destination:callee.register,source:selected.register}),rawReceiver=lowerJavaScriptCallSourceExpression(source.receiver,bindings,heldCallee,depth+1),receiver=allocateJavaScriptCallSourceRegister(rawReceiver.builder);let current=appendJavaScriptCallSourceInstruction(receiver.builder,{$:'copy',destination:receiver.register,source:rawReceiver.register});const arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1),saved=allocateJavaScriptCallSourceRegister(value.builder);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:value.register});arguments_.push(saved.register);}
   const result=allocateJavaScriptCallSourceRegister(current);return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'callClassConstructor',destination:result.register,callee:callee.register,receiver:receiver.register,arguments:arguments_})};
  }
  case 'sourceConstructErrorValue':{
   if(source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'Error construction needs its actual unsigned64 intrinsic realm');
   const constructor=lowerJavaScriptCallSourceExpression(source.constructor,bindings,state,depth+1),newTarget=lowerJavaScriptCallSourceExpression(source.newTarget,bindings,constructor.builder,depth+1),prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,newTarget.builder,depth+1),message=lowerJavaScriptCallSourceExpression(source.message,bindings,prototype.builder,depth+1),present=lowerJavaScriptCallSourceExpression(source.present,bindings,message.builder,depth+1),result=allocateJavaScriptCallSourceRegister(present.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'constructErrorValue',destination:result.register,constructor:constructor.register,newTarget:newTarget.register,prototype:prototype.register,message:message.register,present:present.register,realm:source.realm,errorClass:source.errorClass,bindReceiver:source.bindReceiver})};
  }
  case 'sourceArgument':{
   if(!testJavaScriptCallCodeCoordinate(source.index,1048576))return refuseJavaScriptCallSource(state,'source argument reads require an actual bounded nonnegative coordinate');
   const result=allocateJavaScriptCallSourceRegister(state);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'argument',destination:result.register,index:source.index})};
  }
  case 'sourceThrow':case 'sourceReturn':{
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1);
   return {...value,builder:appendJavaScriptCallSourceInstruction(value.builder,{$:source.$==='sourceThrow'?'throw':'return',source:value.register})};
  }
  case 'sourceAssignBinding':{
   if(state.snapshotMutableBindings!==true)return refuseJavaScriptCallSource(state,'mutable source bindings require their snapshot compilation pass');
   const binding=bindings.find(row=>row.identity===source.identity);if(binding===undefined||binding.register===0||!binding.mutable)return refuseJavaScriptCallSource(state,'a mutable assignment needs its actual mutable local lexical register');
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1);return {...value,builder:appendJavaScriptCallSourceInstruction(value.builder,{$:'copy',destination:binding.register,source:value.register})};
  }
  case 'sourceWhile':{
   const start=state.instructions.length,condition=lowerJavaScriptCallSourceExpression(source.condition,bindings,state,depth+1),branch=condition.builder.instructions.length;
   const test=appendJavaScriptCallSourceInstruction(condition.builder,{$:'branch',condition:condition.register,yes:branch+1,no:0});
   const body=lowerJavaScriptCallSourceExpression(source.body,bindings,test,depth+1),loop=appendJavaScriptCallSourceInstruction(body.builder,{$:'jump',target:start}),end=loop.instructions.length;
   const updated={...loop,instructions:loop.instructions.map((instruction,index)=>index===branch?{$:'branch',condition:condition.register,yes:branch+1,no:end}:instruction)};
   return lowerJavaScriptCallSourceExpression(source.after,bindings,updated,depth+1);
  }
  case 'sourceTry':return lowerJavaScriptCallSourceException(source,bindings,state,depth);
  case 'sourceConstant':{
   const error=readJavaScriptCallConstantError(source.value);if(error!=='')return refuseJavaScriptCallSource(state,error);
   const result=allocateJavaScriptCallSourceRegister(state);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'constant',destination:result.register,value:source.value})};
  }
  case 'sourceBinding':{
   if(!testJavaScriptCallCodeCoordinate(source.identity,65536))return refuseJavaScriptCallSource(state,'source binding identities require exact nonnegative coordinates');
   for(let index=bindings.length-1;index>=0;index-=1){const entry=bindings[index];if(entry!==undefined&&entry.identity===source.identity){if(state.snapshotMutableBindings===true&&entry.mutable){const saved=allocateJavaScriptCallSourceRegister(state);return {...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:entry.register})};}return {$:'sourceResult',builder:state,register:entry.register};}}
   return refuseJavaScriptCallSource(state,'a source binding reference has no actual lexical owner');
  }
  case 'sourceLet':case 'sourceLocalLet':{
   if(!testJavaScriptCallCodeCoordinate(source.identity,65536)||bindings.some(entry=>entry.identity===source.identity))return refuseJavaScriptCallSource(state,'a source binding owner must be unique in its lexical scope');
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1);
   if(source.$==='sourceLocalLet'){
    const local=allocateJavaScriptCallSourceRegister(value.builder),copied=appendJavaScriptCallSourceInstruction(local.builder,{$:'copy',destination:local.register,source:value.register});
    return lowerJavaScriptCallSourceExpression(source.body,bindings.concat([{$:'sourceBindingRegister',identity:source.identity,register:local.register,mutable:source.mutable}]),copied,depth+1);
   }
   return lowerJavaScriptCallSourceExpression(source.body,bindings.concat([{$:'sourceBindingRegister',identity:source.identity,register:value.register,mutable:false}]),value.builder,depth+1);
  }
  case 'sourceParameterDefault':{
   if(state.snapshotMutableBindings!==true)return refuseJavaScriptCallSource(state,'mutable source bindings require their snapshot compilation pass');
   const binding=bindings.find(entry=>entry.identity===source.identity);if(binding===undefined||binding.register<1||binding.register>state.parameters)return refuseJavaScriptCallSource(state,'a parameter default needs its actual initialized parameter coordinate');
   const condition=lowerJavaScriptCallSourceExpression({$:'sourceStrictEqual',left:{$:'sourceBinding',identity:source.identity},right:{$:'sourceConstant',value:{$:'undefined'}}},bindings,state,depth+1),branch=condition.builder.instructions.length;
   const start=appendJavaScriptCallSourceInstruction(condition.builder,{$:'branch',condition:condition.register,yes:0,no:0}),value=lowerJavaScriptCallSourceExpression(source.value,bindings,start,depth+1),copied=appendJavaScriptCallSourceInstruction(value.builder,{$:'copy',destination:binding.register,source:value.register}),end=copied.instructions.length;
   const updated={...copied,instructions:copied.instructions.map((instruction,index)=>index===branch?{$:'branch',condition:condition.register,yes:branch+1,no:end}:instruction)};
   return lowerJavaScriptCallSourceExpression(source.body,bindings,updated,depth+1);
  }
  case 'sourceIf':return lowerJavaScriptCallSourceBranch(source,bindings,state,depth);
  case 'sourceNumberBinary':case 'sourceSameValue':case 'sourceStrictEqual':case 'sourceSnapshotStrictEqual':{
   let left=lowerJavaScriptCallSourceExpression(source.left,bindings,state,depth+1);
   if(source.$==='sourceSnapshotStrictEqual'){const saved=allocateJavaScriptCallSourceRegister(left.builder);left={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:left.register})};}
   const right=lowerJavaScriptCallSourceExpression(source.right,bindings,left.builder,depth+1),result=allocateJavaScriptCallSourceRegister(right.builder);
   if(source.$==='sourceNumberBinary'){
    if(!['add','subtract','multiply','divide','remainder','power','lessThan','equal'].includes(source.operator))return refuseJavaScriptCallSource(result.builder,'source Number lowering requires its exact registered operator');
    return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'numberBinary',destination:result.register,left:left.register,right:right.register,operator:source.operator})};
   }
   if(source.$==='sourceStrictEqual'||source.$==='sourceSnapshotStrictEqual')return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'strictEqual',destination:result.register,left:left.register,right:right.register})};
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'sameValue',destination:result.register,left:left.register,right:right.register})};
  }
  case 'sourcePrimitive':{
   const error=readJavaScriptCallPrimitiveArityError(source.operation,source.arguments.length);if(error!=='')return refuseJavaScriptCallSource(state,error);
   let current=state,arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=value.builder;arguments_.push(value.register);}
   const result=allocateJavaScriptCallSourceRegister(current);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'primitive',destination:result.register,operation:source.operation,arguments:arguments_})};
  }
  case 'sourceRecur':{
   if(state.handlers!==0)return refuseJavaScriptCallSource(state,'source recurrence through an active exception handler requires explicit unwinding');
   if(source.arguments.length!==state.parameters)return refuseJavaScriptCallSource(state,'loop updates require every actual parameter operand');
   let current=state,arguments_=[];
   for(const argument of source.arguments){
    const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1),captured=allocateJavaScriptCallSourceRegister(value.builder);
    current=appendJavaScriptCallSourceInstruction(captured.builder,{$:'copy',destination:captured.register,source:value.register});arguments_.push(captured.register);
   }
   for(let index=0;index<arguments_.length;index+=1)current=appendJavaScriptCallSourceInstruction(current,{$:'copy',destination:index+1,source:arguments_[index]??0});
   return {$:'sourceResult',builder:appendJavaScriptCallSourceInstruction(current,{$:'jump',target:0}),register:0};
  }
  case 'sourceSet':case 'sourceSnapshotSet':{
   let object=lowerJavaScriptCallSourceExpression(source.object,bindings,state,depth+1);
   if(source.$==='sourceSnapshotSet'){const saved=allocateJavaScriptCallSourceRegister(object.builder);object={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:object.register})};}
   let key=lowerJavaScriptCallSourceExpression(source.key,bindings,object.builder,depth+1);
   if(source.$==='sourceSnapshotSet'){const saved=allocateJavaScriptCallSourceRegister(key.builder);key={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:key.register})};}
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,key.builder,depth+1),result=allocateJavaScriptCallSourceRegister(value.builder);
   const write=appendJavaScriptCallSourceInstruction(result.builder,{$:'ordinarySet',destination:result.register,object:object.register,key:key.register,value:value.register,receiver:object.register});
   return {$:'sourceResult',builder:appendJavaScriptCallSourceInstruction(write,{$:'requireWrite',success:result.register}),register:value.register};
  }
  case 'sourceClosure':{
   if(typeof source.owner!=='bigint'||source.owner<=0n||source.owner>18446744073709551615n||!testJavaScriptCallCodeCoordinate(source.ordinal,4096)||source.captures.length>4096)return refuseJavaScriptCallSource(state,'source closure requires its actual static owner, lexical site and bounded capture vector');
   const receiver=lowerJavaScriptCallSourceExpression(source.receiver,bindings,state,depth+1),savedReceiver=allocateJavaScriptCallSourceRegister(receiver.builder);
   let current=appendJavaScriptCallSourceInstruction(savedReceiver.builder,{$:'copy',destination:savedReceiver.register,source:receiver.register});const captures=[];
   for(const capture of source.captures){const value=lowerJavaScriptCallSourceExpression(capture,bindings,current,depth+1),saved=allocateJavaScriptCallSourceRegister(value.builder);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:value.register});captures.push(saved.register);}
   const result=allocateJavaScriptCallSourceRegister(current);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'closureAllocate',destination:result.register,owner:source.owner,ordinal:source.ordinal,captures,receiver:savedReceiver.register})};
  }
  case 'sourceArray':{
   if(!Number.isInteger(source.length)||source.length<0||source.length>4294967295||Object.is(source.length,-0)||source.elements.length>4096||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'source Array construction requires its actual sparse length, bounded elements and realm');
   const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,state,depth+1),object=allocateJavaScriptCallSourceRegister(prototype.builder);
   let current=appendJavaScriptCallSourceInstruction(object.builder,{$:'arrayAllocate',destination:object.register,prototype:prototype.register,length:source.length}),previous=-1;
   for(const element of source.elements){
    if(!Number.isSafeInteger(element.index)||Object.is(element.index,-0)||element.index<=previous||element.index>=source.length)return refuseJavaScriptCallSource(current,'source Array elements retain their actual strictly increasing literal coordinates');previous=element.index;
    const key=lowerJavaScriptCallSourceExpression({$:'sourcePropertyKey',input:{$:'sourceConstant',value:{$:'number',value:element.index}},realm:source.realm},bindings,current,depth+1),value=lowerJavaScriptCallSourceExpression(element.value,bindings,key.builder,depth+1),result=allocateJavaScriptCallSourceRegister(value.builder);
    current=appendJavaScriptCallSourceInstruction(result.builder,{$:'ordinaryDataProperty',destination:result.register,object:object.register,key:key.register,value:value.register,realm:source.realm});
   }
   return {$:'sourceResult',builder:current,register:object.register};
  }
  case 'sourceObject':{
   if(source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'source object construction requires its actual unsigned64 realm identity');
   if(source.properties.length>4096)return refuseJavaScriptCallSource(state,'source object properties exceed their explicit structural budget');
   const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,state,depth+1),object=allocateJavaScriptCallSourceRegister(prototype.builder);
   let current=appendJavaScriptCallSourceInstruction(object.builder,{$:'ordinaryAllocate',destination:object.register,prototype:prototype.register});
   for(const property of source.properties){
    const key=lowerJavaScriptCallSourceExpression({$:'sourcePropertyKey',input:property.key,realm:source.realm},bindings,current,depth+1);
    const value=lowerJavaScriptCallSourceExpression(property.value,bindings,key.builder,depth+1),result=allocateJavaScriptCallSourceRegister(value.builder);
    current=appendJavaScriptCallSourceInstruction(result.builder,{$:'ordinaryDataProperty',destination:result.register,object:object.register,key:key.register,value:value.register,realm:source.realm});
   }
   return {$:'sourceResult',builder:current,register:object.register};
  }
  case 'sourcePropertyKey':{
   if(source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'source property conversion requires its actual unsigned64 realm identity');
   const input=lowerJavaScriptCallSourceExpression(source.input,bindings,state,depth+1),result=allocateJavaScriptCallSourceRegister(input.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'propertyKey',destination:result.register,input:input.register,realm:source.realm})};
  }
  case 'sourcePropertyDefinition':{
   if(source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'source property definition requires its actual unsigned64 realm identity');
   const target=lowerJavaScriptCallSourceExpression(source.target,bindings,state,depth+1),key=lowerJavaScriptCallSourceExpression(source.key,bindings,target.builder,depth+1),attributes=lowerJavaScriptCallSourceExpression(source.attributes,bindings,key.builder,depth+1),result=allocateJavaScriptCallSourceRegister(attributes.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'propertyDefinition',destination:result.register,target:target.register,key:key.register,attributes:attributes.register,realm:source.realm})};
  }
  case 'sourceAllocate':{
   const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,state,depth+1),result=allocateJavaScriptCallSourceRegister(prototype.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'ordinaryAllocate',destination:result.register,prototype:prototype.register})};
  }
  case 'sourceGet':{
   const object=lowerJavaScriptCallSourceExpression(source.object,bindings,state,depth+1),key=lowerJavaScriptCallSourceExpression(source.key,bindings,object.builder,depth+1),result=allocateJavaScriptCallSourceRegister(key.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'ordinaryGet',destination:result.register,object:object.register,key:key.register,receiver:object.register})};
  }
  case 'sourceCall':case 'sourceSnapshotCall':{
   if(source.arguments.length>4096)return refuseJavaScriptCallSource(state,'source call operands exceed their explicit budget');
   let callee=lowerJavaScriptCallSourceExpression(source.callee,bindings,state,depth+1);
   if(source.$==='sourceSnapshotCall'){const saved=allocateJavaScriptCallSourceRegister(callee.builder);callee={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:callee.register})};}
   let receiver=lowerJavaScriptCallSourceExpression(source.receiver,bindings,callee.builder,depth+1);
   if(source.$==='sourceSnapshotCall'){const saved=allocateJavaScriptCallSourceRegister(receiver.builder);receiver={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:receiver.register})};}
   let current=receiver.builder,arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=value.builder;
    if(source.$==='sourceSnapshotCall'){const saved=allocateJavaScriptCallSourceRegister(current);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:value.register});arguments_.push(saved.register);}
    else arguments_.push(value.register);
   }
   const result=allocateJavaScriptCallSourceRegister(current);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'call',destination:result.register,callee:callee.register,receiver:receiver.register,arguments:arguments_})};
  }
 }
}

/** Catch binding identity is local to the handler, never the protected body.
 * Normal evaluation pops exactly its own handler. The machine removes that
 * handler before entering catch; exceptions in catch reach the outer handler.
 * @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth
 * @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceException(source,bindings,builder,depth){
 if(source.$!=='sourceTry')return refuseJavaScriptCallSource(builder,'an exception compiler requires its actual protected body and catch binding');
 if(!testJavaScriptCallCodeCoordinate(source.identity,65536)||bindings.some(entry=>entry.identity===source.identity))return refuseJavaScriptCallSource(builder,'a catch binding owner must be unique in its lexical scope');
 if(builder.handlers>=1024)return refuseJavaScriptCallSource(builder,'source exception handlers exceed their explicit active handler budget');
 const caught=allocateJavaScriptCallSourceRegister(builder),result=allocateJavaScriptCallSourceRegister(caught.builder),position=result.builder.instructions.length;
 const protectedBuilder=appendJavaScriptCallSourceInstruction({...result.builder,handlers:builder.handlers+1},{$:'pushHandler',destination:caught.register,target:0});
 const body=lowerJavaScriptCallSourceExpression(source.body,bindings,protectedBuilder,depth+1);
 const popped=appendJavaScriptCallSourceInstruction({...body.builder,handlers:builder.handlers},{$:'popHandler'});
 const copied=appendJavaScriptCallSourceInstruction(popped,{$:'copy',destination:result.register,source:body.register}),jump=copied.instructions.length;
 const alternative=appendJavaScriptCallSourceInstruction(copied,{$:'jump',target:0}),target=alternative.instructions.length;
 const handler=lowerJavaScriptCallSourceExpression(source.handler,bindings.concat([{$:'sourceBindingRegister',identity:source.identity,register:caught.register,mutable:true}]),alternative,depth+1);
 const finished=appendJavaScriptCallSourceInstruction(handler.builder,{$:'copy',destination:result.register,source:handler.register}),end=finished.instructions.length;
 if(finished.error!=='')return {$:'sourceResult',builder:finished,register:0};
 const instructions=finished.instructions.map((entry,index)=>index===position?{$:'pushHandler',destination:caught.register,target}:index===jump?{$:'jump',target:end}:entry);
 return {$:'sourceResult',builder:{...finished,instructions},register:result.register};
}

/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceBranch(source,bindings,builder,depth){
 if(source.$!=='sourceIf')return refuseJavaScriptCallSource(builder,'a source branch compiler requires its actual conditional');
 const condition=lowerJavaScriptCallSourceExpression(source.condition,bindings,builder,depth+1),result=allocateJavaScriptCallSourceRegister(condition.builder),branch=result.builder.instructions.length;
 const start=appendJavaScriptCallSourceInstruction(result.builder,{$:'branch',condition:condition.register,yes:0,no:0});
 const yesPosition=start.instructions.length,yes=lowerJavaScriptCallSourceExpression(source.yes,bindings,start,depth+1);
 const copiedYes=appendJavaScriptCallSourceInstruction(yes.builder,{$:'copy',destination:result.register,source:yes.register}),jump=copiedYes.instructions.length;
 const alternative=appendJavaScriptCallSourceInstruction(copiedYes,{$:'jump',target:0}),noPosition=alternative.instructions.length;
 const no=lowerJavaScriptCallSourceExpression(source.no,bindings,alternative,depth+1),copiedNo=appendJavaScriptCallSourceInstruction(no.builder,{$:'copy',destination:result.register,source:no.register});
 const end=copiedNo.instructions.length;
 if(copiedNo.error!=='')return {$:'sourceResult',builder:copiedNo,register:0};
 const instructions=copiedNo.instructions.map((entry,index)=>index===branch?{$:'branch',condition:condition.register,yes:yesPosition,no:noPosition}:index===jump?{$:'jump',target:end}:entry);
 return {$:'sourceResult',builder:{...copiedNo,instructions},register:result.register};
}

/** Compile a complete expression function. Receiver register zero is reserved;
 * parameter owners map to actual machine parameter coordinates one onward.
 * Captures, mutation and unsupported expression kinds require further source
 * lowering rather than invented register conventions.
 * @param {JavaScriptCallSource} source @param {number[]} parameters
 * @returns {JavaScriptCallSourceCompilation} */
export function compileJavaScriptCallSource(source,parameters){
 return compileJavaScriptCallSourceWithCaptures(source,parameters,[]);
}
/** Actual module const captures retain semantic handles in the existing frame
 * capture slots. They are not copied object literals or guessed Unit values.
 * @param {JavaScriptCallSource} source @param {number[]} parameters
 * @param {number[]} captures @returns {JavaScriptCallSourceCompilation} */
export function compileJavaScriptCallSourceWithCaptures(source,parameters,captures){
 const first=compileJavaScriptCallSourceBindingReads(source,parameters,captures,false);return first.error==='mutable source bindings require their snapshot compilation pass'?compileJavaScriptCallSourceBindingReads(source,parameters,captures,true):first;
}
/** Snapshot each read of a mutable lexical slot before later operand effects.
 * Immutable captures keep their genuine shared handles; object data is not copied.
 * @param {JavaScriptCallSource} source @param {number[]} parameters
 * @param {number[]} captures @returns {JavaScriptCallSourceCompilation} */
export function compileJavaScriptMutableCallSourceWithCaptures(source,parameters,captures){
 return compileJavaScriptCallSourceBindingReads(source,parameters,captures,true);
}
/** @param {JavaScriptCallSource} source @param {number[]} parameters
 * @param {number[]} captures @param {boolean} snapshotMutableBindings
 * @returns {JavaScriptCallSourceCompilation} */
export function compileJavaScriptCallSourceBindingReads(source,parameters,captures,snapshotMutableBindings){
 if(parameters.length+captures.length>=4096)return {$:'sourceCompiled',instructions:[],registers:0,error:'source parameters exceed their explicit register budget'};
 for(let index=0;index<parameters.length;index+=1){const owner=parameters[index]??-1;if(!testJavaScriptCallCodeCoordinate(owner,65536)||parameters.indexOf(owner)!==index)return {$:'sourceCompiled',instructions:[],registers:0,error:'source parameters require distinct actual lexical owners'};}
 for(let index=0;index<captures.length;index++){const owner=captures[index]??-1;if(!testJavaScriptCallCodeCoordinate(owner,65536)||parameters.includes(owner)||captures.indexOf(owner)!==index)return {$:'sourceCompiled',instructions:[],registers:0,error:'source captures require distinct actual lexical owners'};}
 const bindings=parameters.concat(captures).map((identity,index)=>({$:'sourceBindingRegister',identity,register:index+1,mutable:index<parameters.length}));
 const result=lowerJavaScriptCallSourceExpression(source,bindings,{$:'sourceBuilder',instructions:[],next:parameters.length+captures.length+1,visits:0,parameters:parameters.length,handlers:0,error:'',...(snapshotMutableBindings?{snapshotMutableBindings:true}:{})},0);
 const builder=appendJavaScriptCallSourceInstruction(result.builder,{$:'return',source:result.register});
 return {$:'sourceCompiled',instructions:builder.error===''?builder.instructions:[],registers:builder.error===''?builder.next:0,error:builder.error};
}
