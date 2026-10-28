import {testJavaScriptCallCodeCoordinate,readJavaScriptCallConstantError} from './javascript-call-code.js';

/** Binding identities are supplied from actual checked lexical occurrences.
 * The compiler never derives binding identity from equal source spellings.
 * @typedef {{$:'sourceConstant',value:JavaScriptSemanticValue} |
 * {$:'sourceBinding',identity:number} |
 * {$:'sourceLet',identity:number,value:JavaScriptCallSource,body:JavaScriptCallSource} |
 * {$:'sourceIf',condition:JavaScriptCallSource,yes:JavaScriptCallSource,no:JavaScriptCallSource} |
 * {$:'sourceNumberBinary',operator:string,left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceSameValue',left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceStrictEqual',left:JavaScriptCallSource,right:JavaScriptCallSource} |
 * {$:'sourceCall',callee:JavaScriptCallSource,receiver:JavaScriptCallSource,arguments:JavaScriptCallSource[]}} JavaScriptCallSource
 */
/** @typedef {{$:'sourceBindingRegister',identity:number,register:number}} JavaScriptCallSourceBinding */
/** @typedef {{$:'sourceBuilder',instructions:JavaScriptCallInstruction[],next:number,visits:number,error:string}} JavaScriptCallSourceBuilder */
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
  case 'sourceConstant':{
   const error=readJavaScriptCallConstantError(source.value);if(error!=='')return refuseJavaScriptCallSource(state,error);
   const result=allocateJavaScriptCallSourceRegister(state);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'constant',destination:result.register,value:source.value})};
  }
  case 'sourceBinding':{
   if(!testJavaScriptCallCodeCoordinate(source.identity,65536))return refuseJavaScriptCallSource(state,'source binding identities require exact nonnegative coordinates');
   for(let index=bindings.length-1;index>=0;index-=1){const entry=bindings[index];if(entry!==undefined&&entry.identity===source.identity)return {$:'sourceResult',builder:state,register:entry.register};}
   return refuseJavaScriptCallSource(state,'a source binding reference has no actual lexical owner');
  }
  case 'sourceLet':{
   if(!testJavaScriptCallCodeCoordinate(source.identity,65536)||bindings.some(entry=>entry.identity===source.identity))return refuseJavaScriptCallSource(state,'a source binding owner must be unique in its lexical scope');
   const value=lowerJavaScriptCallSourceExpression(source.value,bindings,state,depth+1);
   return lowerJavaScriptCallSourceExpression(source.body,bindings.concat([{$:'sourceBindingRegister',identity:source.identity,register:value.register}]),value.builder,depth+1);
  }
  case 'sourceIf':return lowerJavaScriptCallSourceBranch(source,bindings,state,depth);
  case 'sourceNumberBinary':case 'sourceSameValue':case 'sourceStrictEqual':{
   const left=lowerJavaScriptCallSourceExpression(source.left,bindings,state,depth+1),right=lowerJavaScriptCallSourceExpression(source.right,bindings,left.builder,depth+1),result=allocateJavaScriptCallSourceRegister(right.builder);
   if(source.$==='sourceNumberBinary'){
    if(!['add','subtract','multiply','divide','remainder','power','lessThan','equal'].includes(source.operator))return refuseJavaScriptCallSource(result.builder,'source Number lowering requires its exact registered operator');
    return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'numberBinary',destination:result.register,left:left.register,right:right.register,operator:source.operator})};
   }
   if(source.$==='sourceStrictEqual')return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'strictEqual',destination:result.register,left:left.register,right:right.register})};
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'sameValue',destination:result.register,left:left.register,right:right.register})};
  }
  case 'sourceCall':{
   if(source.arguments.length>4096)return refuseJavaScriptCallSource(state,'source call operands exceed their explicit budget');
   const callee=lowerJavaScriptCallSourceExpression(source.callee,bindings,state,depth+1),receiver=lowerJavaScriptCallSourceExpression(source.receiver,bindings,callee.builder,depth+1);
   let current=receiver.builder,arguments_=[];
   for(const argument of source.arguments){const value=lowerJavaScriptCallSourceExpression(argument,bindings,current,depth+1);current=value.builder;arguments_.push(value.register);}
   const result=allocateJavaScriptCallSourceRegister(current);
   return {...result,builder:appendJavaScriptCallSourceInstruction(result.builder,{$:'call',destination:result.register,callee:callee.register,receiver:receiver.register,arguments:arguments_})};
  }
 }
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
 if(parameters.length>=4096)return {$:'sourceCompiled',instructions:[],registers:0,error:'source parameters exceed their explicit register budget'};
 for(let index=0;index<parameters.length;index+=1){const owner=parameters[index]??-1;if(!testJavaScriptCallCodeCoordinate(owner,65536)||parameters.indexOf(owner)!==index)return {$:'sourceCompiled',instructions:[],registers:0,error:'source parameters require distinct actual lexical owners'};}
 const bindings=parameters.map((identity,index)=>({$:'sourceBindingRegister',identity,register:index+1}));
 const result=lowerJavaScriptCallSourceExpression(source,bindings,{$:'sourceBuilder',instructions:[],next:parameters.length+1,visits:0,error:''},0);
 const builder=appendJavaScriptCallSourceInstruction(result.builder,{$:'return',source:result.register});
 return {$:'sourceCompiled',instructions:builder.error===''?builder.instructions:[],registers:builder.error===''?builder.next:0,error:builder.error};
}
