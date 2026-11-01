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
 * {$:'sourceVariadicCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[],mode:string,receiverAfterArguments:boolean} |
 * {$:'sourceArgumentSpread',value:JavaScriptCallSource,iteratorKey:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceConstructErrorValue',constructor:JavaScriptCallSource,newTarget:JavaScriptCallSource,prototype:JavaScriptCallSource,message:JavaScriptCallSource,present:JavaScriptCallSource,realm:bigint,errorClass:string,bindReceiver:boolean} |
 * {$:'sourceThrow',value:JavaScriptCallSource} |
 * {$:'sourceReturn',value:JavaScriptCallSource} |
 * {$:'sourceAssignBinding',identity:number,value:JavaScriptCallSource} |
 * {$:'sourceImmutableBindingWrite',value:JavaScriptCallSource} |
 * {$:'sourceWhile',condition:JavaScriptCallSource,body:JavaScriptCallSource,after:JavaScriptCallSource,target?:number,update?:JavaScriptCallSource,postTest?:boolean,iteration?:JavaScriptCallSource} |
 * {$:'sourceCloneBindingCell',identity:number,realm:bigint} |
 * {$:'sourceControlJump',target:number,continuing:boolean} |
 * {$:'sourceControlBlock',target:number,body:JavaScriptCallSource,after:JavaScriptCallSource} |
 * {$:'sourceSwitch',target:number,value:JavaScriptCallSource,clauses:JavaScriptCallSourceSwitchClause[],after:JavaScriptCallSource} |
 * {$:'sourceBindingScope',declarations:JavaScriptCallBindingDeclaration[],initializer:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceInitializeBinding',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceIteratorBinding',slot:number,input:JavaScriptCallSource,iteratorKey:JavaScriptCallSource,body:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceIteratorStep',slot:number,realm:bigint,readValue:boolean} |
 * {$:'sourceIteratorRemainder',slot:number,prototype:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceForOf',slot:number,target:number,binding:number,mutable:boolean,input:JavaScriptCallSource,iteratorKey:JavaScriptCallSource,body:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceIteratorLoop',slot:number,target:number,binding:number,mutable:boolean,body:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceIteratorClose',slot:number,realm:bigint} |
 * {$:'sourceFinally',body:JavaScriptCallSource,finalizer:JavaScriptCallSource} |
 * {$:'sourceTry',identity:number,body:JavaScriptCallSource,handler:JavaScriptCallSource} |
 * {$:'sourceBinding',identity:number} |
 * {$:'sourceLet',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceLocalLet',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource,mutable:boolean} |
 * {$:'sourceParameterRest',identity:number,from:number,prototype:JavaScriptCallSource,realm:bigint,body:JavaScriptCallSource} |
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
 * {$:'sourceArrayAccumulation',prototype:JavaScriptCallSource,parts:JavaScriptCallArrayPart[],iteratorKey:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceArray',prototype:JavaScriptCallSource,length:number,elements:JavaScriptCallSourceArrayElement[],realm:bigint} |
 * {$:'sourcePropertyKey',input:JavaScriptCallSource,realm:bigint} |
 * {$:'sourcePropertyDefinition',target:JavaScriptCallSource,key:JavaScriptCallSource,attributes:JavaScriptCallSource,realm:bigint} |
 * {$:'sourceSet',object:JavaScriptCallSource,key:JavaScriptCallSource,value:JavaScriptCallSource} |
 * {$:'sourceSnapshotSet',object:JavaScriptCallSource,key:JavaScriptCallSource,value:JavaScriptCallSource} |
 * {$:'sourceGet',object:JavaScriptCallSource,key:JavaScriptCallSource} |
 * {$:'sourceCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]} |
 * {$:'sourceSnapshotCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]}} JavaScriptCallSource
 */
/** @typedef {{$:'sourceObjectProperty',key:JavaScriptCallSource,value:JavaScriptCallSource,inferName?:boolean} | {$:'sourceObjectAccessor',key:JavaScriptCallSource,value:JavaScriptCallSource,kind:string,inferName?:boolean} | {$:'sourceLiteralPrototype',value:JavaScriptCallSource} | {$:'sourceObjectSpread',input:JavaScriptCallSource,excluded?:JavaScriptCallSource[]}} JavaScriptCallSourceProperty */
/** @typedef {{$:'sourceArrayValue',value:JavaScriptCallSource} | {$:'sourceArraySpread',value:JavaScriptCallSource} | {$:'sourceArrayHole'}} JavaScriptCallArrayPart */
/** @typedef {{$:'sourceArrayElement',index:number,value:JavaScriptCallSource}} JavaScriptCallSourceArrayElement */
/** @typedef {{$:'sourceSwitchCase',value:JavaScriptCallSource,body:JavaScriptCallSource} | {$:'sourceSwitchDefault',body:JavaScriptCallSource}} JavaScriptCallSourceSwitchClause */
/** @typedef {{$:'sourceControlScope',identity:number,kind:string,handlers:number}} JavaScriptCallSourceControlScope */
/** @typedef {{$:'sourceControlTransfer',position:number,target:number,continuing:boolean}} JavaScriptCallSourceControlTransfer */
/** @typedef {{$:'sourceBindingDeclaration',identity:number,mutable:boolean}} JavaScriptCallBindingDeclaration */
/** @typedef {{$:'sourceBindingRegister',identity:number,register:number,mutable:boolean,initializing?:boolean}} JavaScriptCallSourceBinding */
/** @typedef {{$:'sourceBuilder',instructions:JavaScriptCallInstruction[],next:number,visits:number,parameters:number,handlers:number,error:string,snapshotMutableBindings?:boolean,initializedBindings?:number[],controlScopes?:JavaScriptCallSourceControlScope[],controlTransfers?:JavaScriptCallSourceControlTransfer[]}} JavaScriptCallSourceBuilder */
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
   if(source.arguments.some(argument=>argument.$==='sourceArgumentSpread'))return lowerJavaScriptCallSourceArgumentList({$:'sourceVariadicCall',callee:source.callee,receiver:{$:'sourceConstant',value:{$:'undefined'}},arguments:source.arguments,mode:source.$==='sourceConstructCollection'?'collection':'error class',receiverAfterArguments:false},bindings,state,depth+1);
   if(source.arguments.length>4096)return refuseJavaScriptCallSource(state,'derived Error arguments exceed their explicit operand budget');
   let callee=lowerJavaScriptCallSourceExpression(source.callee,bindings,state,depth+1);if(source.$==='sourceConstructCollection'){const saved=allocateJavaScriptCallSourceRegister(callee.builder);callee={...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:callee.register})};}let current=callee.builder,arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=value.builder;if(source.$==='sourceConstructCollection'){const saved=allocateJavaScriptCallSourceRegister(current);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:value.register});arguments_.push(saved.register);}else arguments_.push(value.register);}
   const result=allocateJavaScriptCallSourceRegister(current);return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:source.$==='sourceConstructCollection'?'constructCollection':'constructErrorClass',destination:result.register,callee:callee.register,arguments:arguments_})};
  }
  case 'sourceClassConstructorCall':{
   if(source.arguments.some(argument=>argument.$==='sourceArgumentSpread'))return lowerJavaScriptCallSourceArgumentList({$:'sourceVariadicCall',callee:source.callee,receiver:source.receiver,arguments:source.arguments,mode:'ordinary class',receiverAfterArguments:false},bindings,state,depth+1);
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
  case 'sourceBindingScope':return lowerJavaScriptCallSourceBindingScope(source,bindings,state,depth);
  case 'sourceInitializeBinding':{
   const binding=bindings.find(row=>row.identity===source.identity);if(binding===undefined||binding.initializing!==true||(state.initializedBindings??[]).includes(source.identity))return refuseJavaScriptCallSource(state,'binding initialization retains its actual uninitialized owned register');
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1),copied=appendJavaScriptCallSourceInstruction(value.builder,{$:'copy',destination:binding.register,source:value.register});return lowerJavaScriptCallSourceExpression(source.body,bindings,{...copied,initializedBindings:(copied.initializedBindings??[]).concat([source.identity])},depth+1);
  }
  case 'sourceAssignBinding':{
   if(state.snapshotMutableBindings!==true)return refuseJavaScriptCallSource(state,'mutable source bindings require their snapshot compilation pass');
   const binding=bindings.find(row=>row.identity===source.identity);if(binding===undefined||binding.register===0||!binding.mutable||binding.initializing===true&&!(state.initializedBindings??[]).includes(source.identity))return refuseJavaScriptCallSource(state,'a mutable assignment needs its actual mutable local lexical register');
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1);return {...value,builder:appendJavaScriptCallSourceInstruction(value.builder,{$:'copy',destination:binding.register,source:value.register})};
  }
  case 'sourceCloneBindingCell':{
   const binding=bindings.find(row=>row.identity===source.identity);if(state.snapshotMutableBindings!==true||binding===undefined||binding.register===0||binding.initializing===true||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'iteration cloning requires its actual initialized lexical cell and source realm');
   const cell={$:'sourceObject',prototype:{$:'sourceConstant',value:{$:'null'}},realm:source.realm,properties:[{$:'sourceObjectProperty',key:{$:'sourceConstant',value:{$:'text',units:[118,97,108,117,101]}},value:{$:'sourceGet',object:{$:'sourceBinding',identity:source.identity},key:{$:'sourceConstant',value:{$:'text',units:[118,97,108,117,101]}}}}]},copied=lowerJavaScriptCallSourceExpression(cell,bindings,state,depth+1);
   return {...copied,builder:appendJavaScriptCallSourceInstruction(copied.builder,{$:'copy',destination:binding.register,source:copied.register})};
  }
  case 'sourceImmutableBindingWrite':{
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1),message=lowerJavaScriptCallSourceExpression({$:'sourceConstant',value:{$:'text',units:[65,115,115,105,103,110,109,101,110,116,32,116,111,32,99,111,110,115,116,97,110,116,32,118,97,114,105,97,98,108,101,46]}},bindings,value.builder,depth+1),present=lowerJavaScriptCallSourceExpression({$:'sourceConstant',value:{$:'boolean',value:true}},bindings,message.builder,depth+1),result=allocateJavaScriptCallSourceRegister(present.builder);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'primitive',destination:result.register,operation:'raiseJavaScriptTypeError',arguments:[message.register,present.register]})};
  }
  case 'sourceWhile':return lowerJavaScriptCallSourceLoop(source,bindings,state,depth);
  case 'sourceControlBlock':{const scoped=pushJavaScriptCallSourceControlScope(state,source.target,'block'),body=lowerJavaScriptCallSourceExpression(source.body,bindings,scoped,depth+1),finished=finishJavaScriptCallSourceControlScope(body.builder,state,source.target,body.builder.instructions.length,-1);return lowerJavaScriptCallSourceExpression(source.after,bindings,finished,depth+1);}
  case 'sourceSwitch':return lowerJavaScriptCallSourceSwitch(source,bindings,state,depth);
  case 'sourceControlJump':return lowerJavaScriptCallSourceControlJump(source,state);
  case 'sourceIteratorBinding':case 'sourceForOf':return lowerJavaScriptCallSourceForOf(source,bindings,state,depth);
  case 'sourceIteratorLoop':return lowerJavaScriptCallSourceIteratorLoop(source,bindings,state,depth);
  case 'sourceIteratorStep':{
   if(!testJavaScriptCallCodeCoordinate(source.slot,65536)||source.realm<=0n||source.realm>18446744073709551615n||typeof source.readValue!=='boolean')return refuseJavaScriptCallSource(state,'binding steps retain their actual iterator region, realm and value choice');
   const value=allocateJavaScriptCallSourceRegister(state),done=allocateJavaScriptCallSourceRegister(value.builder);return {$:'sourceResult',register:value.register,builder:appendJavaScriptCallSourceInstruction(done.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:source.slot,operation:'step',realm:source.realm,...(source.readValue?{}:{readValue:false})})};
  }
  case 'sourceIteratorRemainder':return lowerJavaScriptCallSourceIteratorRemainder(source,bindings,state,depth);
  case 'sourceIteratorClose':{
   if(!testJavaScriptCallCodeCoordinate(source.slot,65536)||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'iterator closing retains its actual source slot and realm');
   const value=allocateJavaScriptCallSourceRegister(state),done=allocateJavaScriptCallSourceRegister(value.builder);return {$:'sourceResult',register:value.register,builder:appendJavaScriptCallSourceInstruction(done.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:source.slot,operation:'close',realm:source.realm})};
  }
  case 'sourceFinally':return lowerJavaScriptCallSourceFinally(source,bindings,state,depth);
  case 'sourceTry':return lowerJavaScriptCallSourceException(source,bindings,state,depth);
  case 'sourceConstant':{
   const error=readJavaScriptCallConstantError(source.value);if(error!=='')return refuseJavaScriptCallSource(state,error);
   const result=allocateJavaScriptCallSourceRegister(state);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'constant',destination:result.register,value:source.value})};
  }
  case 'sourceBinding':{
   if(!testJavaScriptCallCodeCoordinate(source.identity,65536))return refuseJavaScriptCallSource(state,'source binding identities require exact nonnegative coordinates');
   for(let index=bindings.length-1;index>=0;index-=1){const entry=bindings[index];if(entry!==undefined&&entry.identity===source.identity){if(entry.initializing===true&&!(state.initializedBindings??[]).includes(source.identity))return refuseJavaScriptCallSource(state,'uninitialized binding reads require their actual temporal-dead-zone environment protocol');if(state.snapshotMutableBindings===true&&entry.mutable){const saved=allocateJavaScriptCallSourceRegister(state);return {...saved,builder:appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:entry.register})};}return {$:'sourceResult',builder:state,register:entry.register};}}
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
  case 'sourceParameterRest':{
   const binding=bindings.find(row=>row.identity===source.identity);if(state.snapshotMutableBindings!==true||binding===undefined||binding.register!==source.from+1||binding.register>state.parameters||!testJavaScriptCallCodeCoordinate(source.from,1048577)||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'rest parameters retain their actual formal coordinate and realm');
   const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,state,depth+1),copied=appendJavaScriptCallSourceInstruction(prototype.builder,{$:'argumentRest',destination:binding.register,prototype:prototype.register,from:source.from,realm:source.realm});return lowerJavaScriptCallSourceExpression(source.body,bindings,copied,depth+1);
  }
  case 'sourceParameterDefault':{
   if(state.snapshotMutableBindings!==true)return refuseJavaScriptCallSource(state,'mutable source bindings require their snapshot compilation pass');
   const binding=bindings.find(entry=>entry.identity===source.identity);if(binding===undefined||binding.register<1||binding.register>state.parameters)return refuseJavaScriptCallSource(state,'a parameter default needs its actual initialized parameter coordinate');
   const condition=lowerJavaScriptCallSourceExpression({$:'sourceStrictEqual',left:{$:'sourceBinding',identity:source.identity},right:{$:'sourceConstant',value:{$:'undefined'}}},bindings,state,depth+1),branch=condition.builder.instructions.length;
   const start=appendJavaScriptCallSourceInstruction(condition.builder,{$:'branch',condition:condition.register,yes:0,no:0}),value=lowerJavaScriptCallSourceExpression(source.value,bindings,start,depth+1),copied=appendJavaScriptCallSourceInstruction(value.builder,{$:'copy',destination:binding.register,source:value.register}),end=copied.instructions.length;
   const updated={...copied,instructions:copied.instructions.map((instruction,index)=>index===branch?{$:'branch',condition:condition.register,yes:branch+1,no:end}:instruction),...(copied.initializedBindings!==undefined?{initializedBindings:condition.builder.initializedBindings??[]}:{})};
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
  case 'sourceArrayAccumulation':{
   if(source.parts.length>4096||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(state,'Array accumulation retains its bounded parts and actual realm');
   const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,state,depth+1),object=allocateJavaScriptCallSourceRegister(prototype.builder);let current=appendJavaScriptCallSourceInstruction(object.builder,{$:'arrayAllocate',destination:object.register,prototype:prototype.register,length:0});
   const iteratorKey=lowerJavaScriptCallSourceExpression(source.iteratorKey,bindings,current,depth+1),savedKey=allocateJavaScriptCallSourceRegister(iteratorKey.builder),keyed=appendJavaScriptCallSourceInstruction(savedKey.builder,{$:'copy',destination:savedKey.register,source:iteratorKey.register}),index=allocateJavaScriptCallSourceRegister(keyed);current=appendJavaScriptCallSourceInstruction(index.builder,{$:'constant',destination:index.register,value:{$:'number',value:0}});
   for(const part of source.parts){
    if(!['sourceArraySpread','sourceArrayValue','sourceArrayHole'].includes(part.$))return refuseJavaScriptCallSource(current,'Array accumulation retains every actual part kind');
    if(part.$==='sourceArraySpread'){
     const input=lowerJavaScriptCallSourceExpression(part.value,bindings,current,depth+1),saved=allocateJavaScriptCallSourceRegister(input.builder),held=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:input.register});current=appendJavaScriptCallSourceInstruction(held,{$:'arraySpread',destination:index.register,target:object.register,input:saved.register,index:index.register,iteratorKey:savedKey.register,realm:source.realm});continue;
    }
    if(part.$==='sourceArrayValue'){
     const key=allocateJavaScriptCallSourceRegister(current),keyed=appendJavaScriptCallSourceInstruction(key.builder,{$:'propertyKey',destination:key.register,input:index.register,realm:source.realm}),value=lowerJavaScriptCallSourceExpression(part.value,bindings,keyed,depth+1),written=allocateJavaScriptCallSourceRegister(value.builder);
     current=appendJavaScriptCallSourceInstruction(written.builder,{$:'ordinaryDataProperty',destination:written.register,object:object.register,key:key.register,value:value.register,realm:source.realm});
    }
    current=appendJavaScriptCallSourceInstruction(current,{$:'primitive',destination:index.register,operation:'increment numeric value',arguments:[index.register]});
    if(part.$==='sourceArrayHole'){
     const key=lowerJavaScriptCallSourceExpression({$:'sourceConstant',value:{$:'text',units:[108,101,110,103,116,104]}},bindings,current,depth+1),written=allocateJavaScriptCallSourceRegister(key.builder);
     current=appendJavaScriptCallSourceInstruction(written.builder,{$:'ordinarySet',destination:written.register,object:object.register,key:key.register,value:index.register,receiver:object.register});current=appendJavaScriptCallSourceInstruction(current,{$:'requireWrite',success:written.register});
    }
   }
   return {$:'sourceResult',builder:current,register:object.register};
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
    if(property.$==='sourceLiteralPrototype'){
     const value=lowerJavaScriptCallSourceExpression(property.value,bindings,current,depth+1),result=allocateJavaScriptCallSourceRegister(value.builder);current=appendJavaScriptCallSourceInstruction(result.builder,{$:'literalPrototype',destination:result.register,object:object.register,value:value.register});continue;
    }
    if(property.$==='sourceObjectSpread'){
     const input=lowerJavaScriptCallSourceExpression(property.input,bindings,current,depth+1);current=input.builder;
     if((property.excluded??[]).length>4096)return refuseJavaScriptCallSource(current,'object copy exclusions exceed their actual operand budget');
     const excluded=[];
     for(const expression of property.excluded??[]){const key=lowerJavaScriptCallSourceExpression({$:'sourcePropertyKey',input:expression,realm:source.realm},bindings,current,depth+1),saved=allocateJavaScriptCallSourceRegister(key.builder);current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:key.register});excluded.push(saved.register);}
     const result=allocateJavaScriptCallSourceRegister(current);current=appendJavaScriptCallSourceInstruction(result.builder,{$:'objectSpread',destination:result.register,target:object.register,input:input.register,realm:source.realm,...(property.excluded!==undefined?{excluded}:{})});continue;
    }
    const key=lowerJavaScriptCallSourceExpression({$:'sourcePropertyKey',input:property.key,realm:source.realm},bindings,current,depth+1);
    const value=lowerJavaScriptCallSourceExpression(property.value,bindings,key.builder,depth+1),result=allocateJavaScriptCallSourceRegister(value.builder);
    let prepared=result.builder;if(property.inferName){const named=allocateJavaScriptCallSourceRegister(prepared);prepared=appendJavaScriptCallSourceInstruction(named.builder,{$:'functionName',destination:named.register,object:value.register,key:key.register,prefix:property.$==='sourceObjectAccessor'?property.kind:'',realm:source.realm});}
    current=appendJavaScriptCallSourceInstruction(prepared,property.$==='sourceObjectAccessor'?{$:'literalAccessor',destination:result.register,object:object.register,key:key.register,value:value.register,kind:property.kind,realm:source.realm}:{$:'ordinaryDataProperty',destination:result.register,object:object.register,key:key.register,value:value.register,realm:source.realm});
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
   if(source.arguments.some(argument=>argument.$==='sourceArgumentSpread'))return lowerJavaScriptCallSourceArgumentList({$:'sourceVariadicCall',callee:source.callee,receiver:source.receiver,arguments:source.arguments,mode:'call',receiverAfterArguments:false},bindings,state,depth+1);
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
  case 'sourceVariadicCall':return lowerJavaScriptCallSourceArgumentList(source,bindings,state,depth+1);
  case 'sourceArgumentSpread':return refuseJavaScriptCallSource(state,'argument spread requires its actual surrounding argument List');
 }
}

/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceArgumentList(source,bindings,builder,depth){
 if(source.$!=='sourceVariadicCall'||!['call','collection','error class','ordinary class'].includes(source.mode)||typeof source.receiverAfterArguments!=='boolean'||source.receiverAfterArguments&&source.mode!=='ordinary class')return refuseJavaScriptCallSource(builder,'variadic source retains its actual invocation and receiver allocation policy');
 if(source.arguments.length>4096)return refuseJavaScriptCallSource(builder,'variadic source exceeds its authored argument expression budget');
 const selected=lowerJavaScriptCallSourceExpression(source.callee,bindings,builder,depth+1),callee=allocateJavaScriptCallSourceRegister(selected.builder);let current=appendJavaScriptCallSourceInstruction(callee.builder,{$:'copy',destination:callee.register,source:selected.register}),receiverRegister=0;
 if(!source.receiverAfterArguments){const raw=lowerJavaScriptCallSourceExpression(source.receiver,bindings,current,depth+1),held=allocateJavaScriptCallSourceRegister(raw.builder);current=appendJavaScriptCallSourceInstruction(held.builder,{$:'copy',destination:held.register,source:raw.register});receiverRegister=held.register;}
 const slot=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'argumentListOpen',slot});
 for(const argument of source.arguments){
  if(argument.$!=='sourceArgumentSpread'){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=appendJavaScriptCallSourceInstruction(value.builder,{$:'argumentListAppend',slot,value:value.register});continue;}
  const input=lowerJavaScriptCallSourceExpression(argument.value,bindings,current,depth+1),held=allocateJavaScriptCallSourceRegister(input.builder),saved=appendJavaScriptCallSourceInstruction(held.builder,{$:'copy',destination:held.register,source:input.register}),key=lowerJavaScriptCallSourceExpression(argument.iteratorKey,bindings,saved,depth+1),heldKey=allocateJavaScriptCallSourceRegister(key.builder),keyed=appendJavaScriptCallSourceInstruction(heldKey.builder,{$:'copy',destination:heldKey.register,source:key.register}),value=allocateJavaScriptCallSourceRegister(keyed),done=allocateJavaScriptCallSourceRegister(value.builder);
  let iterator=0;while(iterator<65536&&done.builder.instructions.some(row=>row.$==='iteratorProtocol'&&row.slot===iterator))iterator++;
  if(iterator>=65536||current.handlers>=1024)return refuseJavaScriptCallSource(current,'argument spread exceeds its actual iterator slot or handler budget');
  current=appendJavaScriptCallSourceInstruction(done.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:held.register,iteratorKey:heldKey.register,slot:iterator,operation:'acquire',realm:argument.realm});
  const protect=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'pushFinally',target:0});const start=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:iterator,operation:'step',realm:argument.realm});
  const branch=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'branch',condition:done.register,yes:0,no:branch+1});current=appendJavaScriptCallSourceInstruction(current,{$:'argumentListAppend',slot,value:value.register});current=appendJavaScriptCallSourceInstruction(current,{$:'jump',target:start});
  const leave=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'completeJump',target:0,depth:builder.handlers});const cleanup=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:iterator,operation:'close',realm:argument.realm});current=appendJavaScriptCallSourceInstruction(current,{$:'finishFinally'});const end=current.instructions.length;
  current={...current,instructions:current.instructions.map((row,index)=>index===protect?{$:'pushFinally',target:cleanup}:index===branch?{$:'branch',condition:done.register,yes:leave,no:branch+1}:index===leave?{$:'completeJump',target:end,depth:builder.handlers}:row)};
 }
 if(source.receiverAfterArguments){const raw=lowerJavaScriptCallSourceExpression(source.receiver,bindings,current,depth+1),held=allocateJavaScriptCallSourceRegister(raw.builder);current=appendJavaScriptCallSourceInstruction(held.builder,{$:'copy',destination:held.register,source:raw.register});receiverRegister=held.register;}
 const result=allocateJavaScriptCallSourceRegister(current);return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'argumentListCall',destination:result.register,slot,callee:callee.register,receiver:receiverRegister,mode:source.mode})};
}

/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceFinally(source,bindings,builder,depth){
 if(source.$!=='sourceFinally')return refuseJavaScriptCallSource(builder,'finally lowering retains its actual protected body and finalizer');
 if(builder.handlers>=1024)return refuseJavaScriptCallSource(builder,'finally exceeds its actual shared handler budget');
 const result=allocateJavaScriptCallSourceRegister(builder),position=result.builder.instructions.length,protectedBuilder=appendJavaScriptCallSourceInstruction({...result.builder,handlers:builder.handlers+1},{$:'pushFinally',target:0}),body=lowerJavaScriptCallSourceExpression(source.body,bindings,protectedBuilder,depth+1),copied=appendJavaScriptCallSourceInstruction(body.builder,{$:'copy',destination:result.register,source:body.register}),jump=copied.instructions.length,normal=appendJavaScriptCallSourceInstruction(copied,{$:'completeJump',target:0,depth:builder.handlers}),entry=normal.instructions.length,finalizer=lowerJavaScriptCallSourceExpression(source.finalizer,bindings,normal,depth+1),finished=appendJavaScriptCallSourceInstruction(finalizer.builder,{$:'finishFinally'}),end=finished.instructions.length;
 const instructions=finished.instructions.map((instruction,index)=>index===position?{$:'pushFinally',target:entry}:index===jump?{$:'completeJump',target:end,depth:builder.handlers}:instruction);
 return {$:'sourceResult',builder:{...finished,instructions,handlers:builder.handlers},register:result.register};
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
 const handler=lowerJavaScriptCallSourceExpression(source.handler,bindings.concat([{$:'sourceBindingRegister',identity:source.identity,register:caught.register,mutable:true}]),alternative.initializedBindings===undefined?alternative:{...alternative,initializedBindings:builder.initializedBindings??[]},depth+1);
 const finished=appendJavaScriptCallSourceInstruction(handler.builder,{$:'copy',destination:result.register,source:handler.register}),end=finished.instructions.length;
 if(finished.error!=='')return {$:'sourceResult',builder:finished,register:0};
 const instructions=finished.instructions.map((entry,index)=>index===position?{$:'pushHandler',destination:caught.register,target}:index===jump?{$:'jump',target:end}:entry);
 return {$:'sourceResult',builder:{...finished,instructions,...(body.builder.initializedBindings!==undefined||handler.builder.initializedBindings!==undefined?{initializedBindings:(body.builder.initializedBindings??[]).filter(identity=>(handler.builder.initializedBindings??[]).includes(identity))}:{})},register:result.register};
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
 const no=lowerJavaScriptCallSourceExpression(source.no,bindings,alternative.initializedBindings===undefined?alternative:{...alternative,initializedBindings:condition.builder.initializedBindings??[]},depth+1),copiedNo=appendJavaScriptCallSourceInstruction(no.builder,{$:'copy',destination:result.register,source:no.register});
 const end=copiedNo.instructions.length;
 if(copiedNo.error!=='')return {$:'sourceResult',builder:copiedNo,register:0};
 const instructions=copiedNo.instructions.map((entry,index)=>index===branch?{$:'branch',condition:condition.register,yes:yesPosition,no:noPosition}:index===jump?{$:'jump',target:end}:entry);
 return {$:'sourceResult',builder:{...copiedNo,instructions,...(yes.builder.initializedBindings!==undefined||no.builder.initializedBindings!==undefined?{initializedBindings:(yes.builder.initializedBindings??[]).filter(identity=>(no.builder.initializedBindings??[]).includes(identity))}:{})},register:result.register};
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
 let builder=appendJavaScriptCallSourceInstruction(result.builder,{$:'return',source:result.register});
 if((builder.controlTransfers??[]).length)builder={...builder,error:'completed source compilation cannot retain unresolved control transfers'};
 return {$:'sourceCompiled',instructions:builder.error===''?builder.instructions:[],registers:builder.error===''?builder.next:0,error:builder.error};
}


/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBuilder} builder
 * @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceControlJump(source,builder){
 if(source.$!=='sourceControlJump'||typeof source.continuing!=='boolean')return refuseJavaScriptCallSource(builder,'a control jump requires its actual source target');
 const scope=(builder.controlScopes??[]).find(row=>row.identity===source.target);
 if(scope===undefined||source.continuing&&scope.kind!=='loop'||scope.handlers>builder.handlers)return refuseJavaScriptCallSource(builder,'break and continue retain an active actual target and handler depth');
 let current=builder;const position=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,builder.handlers>scope.handlers?{$:'completeJump',target:0,depth:scope.handlers}:{$:'jump',target:0});
 const transfers=current.controlTransfers??[];if(transfers.length>=65536)return refuseJavaScriptCallSource(current,'control transfers exceed their explicit transfer budget');
 return {$:'sourceResult',builder:{...current,controlTransfers:transfers.concat([{$:'sourceControlTransfer',position,target:source.target,continuing:source.continuing}])},register:0};
}
/** @param {JavaScriptCallSourceBuilder} builder @param {number} target @param {string} kind
 * @returns {JavaScriptCallSourceBuilder} */
export function pushJavaScriptCallSourceControlScope(builder,target,kind){
 const scopes=builder.controlScopes??[];
 if(!['loop','switch','block'].includes(kind)||!testJavaScriptCallCodeCoordinate(target,65536)||scopes.length>=128||scopes.some(scope=>scope.identity===target))return {...builder,error:'control scopes require unique actual source targets within their lexical budget'};
 return {...builder,controlScopes:scopes.concat([{$:'sourceControlScope',identity:target,kind,handlers:builder.handlers}])};
}
/** @param {JavaScriptCallSourceBuilder} builder @param {JavaScriptCallSourceBuilder} outer
 * @param {number} target @param {number} end @param {number} update
 * @returns {JavaScriptCallSourceBuilder} */
export function finishJavaScriptCallSourceControlScope(builder,outer,target,end,update){
 const transfers=builder.controlTransfers??[],selected=transfers.filter(row=>row.target===target);
 if(selected.some(row=>row.continuing&&update<0))return {...builder,error:'continue requires its actual loop update or condition target'};
 const positions=selected.map(row=>row.position);
 let previous=-1;for(const position of positions){if(!testJavaScriptCallCodeCoordinate(position,builder.instructions.length)||position<=previous||!['jump','completeJump'].includes(builder.instructions[position]?.$??''))return {...builder,error:'source control transfers retain ordered unique instruction positions'};previous=position;}
 const instructions=builder.instructions.map((instruction,index)=>{const position=findJavaScriptCallSourceInstructionPosition(positions,index);if(position<0)return instruction;const transfer=selected[position];return transfer===undefined?instruction:{...instruction,target:transfer.continuing?update:end};});
 const remaining=transfers.filter(row=>row.target!==target);
 return {...builder,instructions,...(outer.controlScopes!==undefined?{controlScopes:outer.controlScopes}:{controlScopes:[]}),...(remaining.length||outer.controlTransfers!==undefined?{controlTransfers:remaining}:{controlTransfers:[]})};
}
/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceLoop(source,bindings,builder,depth){
 if(source.$!=='sourceWhile')return refuseJavaScriptCallSource(builder,'loop lowering requires its actual condition and body');
 const scoped=source.target===undefined?builder:pushJavaScriptCallSourceControlScope(builder,source.target,'loop');
 if(source.postTest===true){
  if(source.update!==undefined||source.iteration!==undefined)return refuseJavaScriptCallSource(scoped,'a post-test loop retains its actual separate condition without an invented update or lexical head clone');
  const start=scoped.instructions.length,body=lowerJavaScriptCallSourceExpression(source.body,bindings,scoped,depth+1),test=body.builder.instructions.length,condition=lowerJavaScriptCallSourceExpression(source.condition,bindings,body.builder,depth+1),end=condition.builder.instructions.length+1;
  let finished=appendJavaScriptCallSourceInstruction(condition.builder,{$:'branch',condition:condition.register,yes:start,no:end});
  if(source.target!==undefined)finished=finishJavaScriptCallSourceControlScope(finished,builder,source.target,end,test);
  return lowerJavaScriptCallSourceExpression(source.after,bindings,finished,depth+1);
 }
 const initial=source.iteration===undefined?scoped:lowerJavaScriptCallSourceExpression(source.iteration,bindings,scoped,depth+1).builder,start=initial.instructions.length,condition=lowerJavaScriptCallSourceExpression(source.condition,bindings,initial,depth+1),branch=condition.builder.instructions.length;
 const test=appendJavaScriptCallSourceInstruction(condition.builder,{$:'branch',condition:condition.register,yes:branch+1,no:0}),body=lowerJavaScriptCallSourceExpression(source.body,bindings,test,depth+1),update=body.builder.instructions.length;
 const cloned=source.iteration===undefined?body.builder:lowerJavaScriptCallSourceExpression(source.iteration,bindings,body.builder,depth+1).builder,updated=source.update===undefined?cloned:lowerJavaScriptCallSourceExpression(source.update,bindings,cloned,depth+1).builder;
 const loop=appendJavaScriptCallSourceInstruction(updated,{$:'jump',target:start}),end=loop.instructions.length;
 let finished={...loop,instructions:loop.instructions.map((instruction,index)=>index===branch?{$:'branch',condition:condition.register,yes:branch+1,no:end}:instruction)};
 if(source.target!==undefined)finished=finishJavaScriptCallSourceControlScope(finished,builder,source.target,end,source.update===undefined&&source.iteration===undefined?start:update);
 return lowerJavaScriptCallSourceExpression(source.after,bindings,finished,depth+1);
}
/** Switch statement sequencing: compare once-captured input to case selectors
 * until a match, then execute adjacent clause blocks without re-testing labels.
 * @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceSwitch(source,bindings,builder,depth){
 if(source.$!=='sourceSwitch'||source.clauses.length>4096)return refuseJavaScriptCallSource(builder,'switch requires its actual bounded case directory');
 const input=lowerJavaScriptCallSourceExpression(source.value,bindings,builder,depth+1),saved=allocateJavaScriptCallSourceRegister(input.builder);
 let current=appendJavaScriptCallSourceInstruction(saved.builder,{$:'copy',destination:saved.register,source:input.register});current=pushJavaScriptCallSourceControlScope(current,source.target,'switch');
 /** @type {number[]} */const branches=[];let fallback=-1;
 for(let index=0;index<source.clauses.length;index++){
  const clause=source.clauses[index];if(clause===undefined)return refuseJavaScriptCallSource(current,'switch clauses must be dense');
  if(clause.$==='sourceSwitchDefault'){if(fallback>=0)return refuseJavaScriptCallSource(current,'switch cannot contain multiple default clauses');fallback=index;branches.push(-1);continue;}
  const value=lowerJavaScriptCallSourceExpression(clause.value,bindings,current,depth+1),comparison=allocateJavaScriptCallSourceRegister(value.builder),compared=appendJavaScriptCallSourceInstruction(comparison.builder,{$:'strictEqual',destination:comparison.register,left:saved.register,right:value.register}),position=compared.instructions.length;
  branches.push(position);current=appendJavaScriptCallSourceInstruction(compared,{$:'branch',condition:comparison.register,yes:0,no:position+1});
 }
 const dispatch=current.instructions.length;current=appendJavaScriptCallSourceInstruction(current,{$:'jump',target:0});
 /** @type {number[]} */const bodies=[];
 for(const clause of source.clauses){bodies.push(current.instructions.length);current=lowerJavaScriptCallSourceExpression(clause.body,bindings,current,depth+1).builder;}
 const end=current.instructions.length,positions=branches.filter(position=>position>=0),indices=branches.map((_,index)=>index).filter(index=>(branches[index]??-1)>=0);
 current={...current,instructions:current.instructions.map((instruction,index)=>{
  if(index===dispatch)return {$:'jump',target:fallback<0?end:bodies[fallback]??end};
  const selected=findJavaScriptCallSourceInstructionPosition(positions,index);if(selected<0)return instruction;const clause=indices[selected];if(clause===undefined||instruction.$!=='branch')return instruction;
  return {$:'branch',condition:instruction.condition,yes:bodies[clause]??end,no:instruction.no};
 })};
 current=finishJavaScriptCallSourceControlScope(current,builder,source.target,end,-1);
 return lowerJavaScriptCallSourceExpression(source.after,bindings,current,depth+1);
}

/** @param {number[]} positions @param {number} position @returns {number} */
export function findJavaScriptCallSourceInstructionPosition(positions,position){
 let low=0,high=positions.length;while(low<high){const middle=Math.floor((low+high)/2),value=positions[middle];if(value===undefined)return -1;if(value===position)return middle;if(value<position)low=middle+1;else high=middle;}return -1;
}


/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceForOf(source,bindings,builder,depth){
 if(source.$!=='sourceForOf'&&source.$!=='sourceIteratorBinding'||!testJavaScriptCallCodeCoordinate(source.slot,65536)||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(builder,'for-of retains its actual bounded source region and realm');
 const input=lowerJavaScriptCallSourceExpression(source.input,bindings,builder,depth+1),held=allocateJavaScriptCallSourceRegister(input.builder),saved=appendJavaScriptCallSourceInstruction(held.builder,{$:'copy',destination:held.register,source:input.register}),key=lowerJavaScriptCallSourceExpression(source.iteratorKey,bindings,saved,depth+1),heldKey=allocateJavaScriptCallSourceRegister(key.builder),keyed=appendJavaScriptCallSourceInstruction(heldKey.builder,{$:'copy',destination:heldKey.register,source:key.register}),value=allocateJavaScriptCallSourceRegister(keyed),done=allocateJavaScriptCallSourceRegister(value.builder),opened=appendJavaScriptCallSourceInstruction(done.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:held.register,iteratorKey:heldKey.register,slot:source.slot,operation:'acquire',realm:source.realm});
 return lowerJavaScriptCallSourceExpression({$:'sourceFinally',body:source.$==='sourceForOf'?{$:'sourceIteratorLoop',slot:source.slot,target:source.target,binding:source.binding,mutable:source.mutable,body:source.body,realm:source.realm}:source.body,finalizer:{$:'sourceIteratorClose',slot:source.slot,realm:source.realm}},bindings,opened,depth+1);
}
/** @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceIteratorLoop(source,bindings,builder,depth){
 if(source.$!=='sourceIteratorLoop'||!testJavaScriptCallCodeCoordinate(source.binding,65536)||!testJavaScriptCallCodeCoordinate(source.slot,65536)||source.realm<=0n||source.realm>18446744073709551615n||bindings.some(row=>row.identity===source.binding))return refuseJavaScriptCallSource(builder,'iterator loop retains its actual independent source binding');
 const scoped=pushJavaScriptCallSourceControlScope(builder,source.target,'loop'),value=allocateJavaScriptCallSourceRegister(scoped),done=allocateJavaScriptCallSourceRegister(value.builder),start=done.builder.instructions.length,stepped=appendJavaScriptCallSourceInstruction(done.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:source.slot,operation:'step',realm:source.realm}),branch=stepped.instructions.length,test=appendJavaScriptCallSourceInstruction(stepped,{$:'branch',condition:done.register,yes:0,no:branch+1}),body=lowerJavaScriptCallSourceExpression(source.body,bindings.concat([{$:'sourceBindingRegister',identity:source.binding,register:value.register,mutable:source.mutable}]),test,depth+1),loop=appendJavaScriptCallSourceInstruction(body.builder,{$:'jump',target:start}),end=loop.instructions.length,patched={...loop,instructions:loop.instructions.map((instruction,index)=>index===branch?{$:'branch',condition:done.register,yes:end,no:branch+1}:instruction)},finished=finishJavaScriptCallSourceControlScope(patched,builder,source.target,end,start);
 return lowerJavaScriptCallSourceExpression({$:'sourceConstant',value:{$:'undefined'}},bindings,finished,depth+1);
}


/** Preallocate actual outputs so initializer finalizers finish before the body.
 * Static uninitialized markers refuse unsupported TDZ/cell reads, never default
 * them to undefined. Only internal initialization may write constant outputs.
 * @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceBindingScope(source,bindings,builder,depth){
 if(source.$!=='sourceBindingScope'||source.declarations.length>4096)return refuseJavaScriptCallSource(builder,'binding output scopes retain their actual bounded declarations');
 let current=builder;const entries=[];
 for(const declaration of source.declarations){if(!testJavaScriptCallCodeCoordinate(declaration.identity,65536)||bindings.some(row=>row.identity===declaration.identity)||entries.some(row=>row.identity===declaration.identity)||typeof declaration.mutable!=='boolean')return refuseJavaScriptCallSource(current,'binding output scopes retain distinct actual lexical identities and mutability');const register=allocateJavaScriptCallSourceRegister(current);current=register.builder;entries.push({$:'sourceBindingRegister',identity:declaration.identity,register:register.register,mutable:declaration.mutable,initializing:true});}
 const initialized=lowerJavaScriptCallSourceExpression(source.initializer,bindings.concat(entries),current,depth+1);
 if(entries.some(row=>!(initialized.builder.initializedBindings??[]).includes(row.identity)))return refuseJavaScriptCallSource(initialized.builder,'every binding output must complete initialization before its body');
 return lowerJavaScriptCallSourceExpression(source.body,bindings.concat(entries.map(row=>({$:'sourceBindingRegister',identity:row.identity,register:row.register,mutable:row.mutable}))),initialized.builder,depth+1);
}
/** Consume the already cached record into a fresh Array without reacquisition.
 * @param {JavaScriptCallSource} source @param {JavaScriptCallSourceBinding[]} bindings
 * @param {JavaScriptCallSourceBuilder} builder @param {number} depth @returns {JavaScriptCallSourceResult} */
export function lowerJavaScriptCallSourceIteratorRemainder(source,bindings,builder,depth){
 if(source.$!=='sourceIteratorRemainder'||!testJavaScriptCallCodeCoordinate(source.slot,65536)||source.realm<=0n||source.realm>18446744073709551615n)return refuseJavaScriptCallSource(builder,'iterator rest retains its actual opened region and realm');
 const prototype=lowerJavaScriptCallSourceExpression(source.prototype,bindings,builder,depth+1),object=allocateJavaScriptCallSourceRegister(prototype.builder),created=appendJavaScriptCallSourceInstruction(object.builder,{$:'arrayAllocate',destination:object.register,prototype:prototype.register,length:0}),index=allocateJavaScriptCallSourceRegister(created),initial=appendJavaScriptCallSourceInstruction(index.builder,{$:'constant',destination:index.register,value:{$:'number',value:0}}),value=allocateJavaScriptCallSourceRegister(initial),done=allocateJavaScriptCallSourceRegister(value.builder),key=allocateJavaScriptCallSourceRegister(done.builder),written=allocateJavaScriptCallSourceRegister(key.builder),start=written.builder.instructions.length,stepped=appendJavaScriptCallSourceInstruction(written.builder,{$:'iteratorProtocol',destination:value.register,done:done.register,input:0,iteratorKey:0,slot:source.slot,operation:'step',realm:source.realm}),branch=stepped.instructions.length,test=appendJavaScriptCallSourceInstruction(stepped,{$:'branch',condition:done.register,yes:0,no:branch+1}),keyed=appendJavaScriptCallSourceInstruction(test,{$:'propertyKey',destination:key.register,input:index.register,realm:source.realm}),defined=appendJavaScriptCallSourceInstruction(keyed,{$:'ordinaryDataProperty',destination:written.register,object:object.register,key:key.register,value:value.register,realm:source.realm}),advanced=appendJavaScriptCallSourceInstruction(defined,{$:'primitive',destination:index.register,operation:'increment numeric value',arguments:[index.register]}),loop=appendJavaScriptCallSourceInstruction(advanced,{$:'jump',target:start}),end=loop.instructions.length;
 return {$:'sourceResult',register:object.register,builder:{...loop,instructions:loop.instructions.map((instruction,at)=>at===branch?{$:'branch',condition:done.register,yes:end,no:branch+1}:instruction)}};
}
