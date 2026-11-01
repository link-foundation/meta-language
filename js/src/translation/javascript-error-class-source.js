// Source-class plans contain actual lexical binding coordinates. Runtime
// identities/prototypes come from the supplied realm registry, never numbers
// or class names invented by this compiler. No class body executes here.
import {compileJavaScriptCallSourceWithCaptures} from './javascript-call-source.js';
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
/** @typedef {{$:'errorConstant',value:JavaScriptSemanticValue} |
 * {$:'errorBinding',identity:number} | {$:'errorModuleBinding',identity:number} | {$:'errorThis'} |
 * {$:'errorConditional',condition:JavaScriptErrorExpression,yes:JavaScriptErrorExpression,no:JavaScriptErrorExpression} |
 * {$:'errorGet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression} |
 * {$:'errorObject',properties:JavaScriptErrorObjectProperty[]} |
 * {$:'errorTemplate',parts:JavaScriptErrorTemplatePart[]} |
 * {$:'errorCall',callee:JavaScriptErrorExpression,receiver:JavaScriptErrorExpression,arguments:JavaScriptErrorExpression[],construct:boolean} |
 * {$:'errorMethodCall',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression,arguments:JavaScriptErrorExpression[]} |
 * {$:'errorFunctionReference',identity:number} |
 * {$:'errorStrictEqual',left:JavaScriptErrorExpression,right:JavaScriptErrorExpression,negate:boolean}} JavaScriptErrorExpression */
/** @typedef {{$:'errorObjectProperty',key:number[],value:JavaScriptErrorExpression}} JavaScriptErrorObjectProperty */
/** @typedef {{$:'errorTemplateText',units:number[]} | {$:'errorTemplateValue',value:JavaScriptErrorExpression}} JavaScriptErrorTemplatePart */
/** @typedef {{$:'errorRequiredParameter',identity:number} | {$:'errorDefaultParameter',identity:number,value:JavaScriptErrorExpression}} JavaScriptErrorParameter */
/** @typedef {{$:'errorFieldAssignment',name:number[],value:JavaScriptErrorExpression}} JavaScriptErrorFieldAssignment */
/** @typedef {{$:'errorConstructorPlan',identity:number,name:string,baseClass:string,parameters:JavaScriptErrorParameter[],message:JavaScriptErrorExpression,fields:JavaScriptErrorFieldAssignment[]} |
 * {$:'errorFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],body:JavaScriptErrorExpression,abrupt:boolean}} JavaScriptErrorFunctionPlan */
/** @typedef {{$:'errorModulePlan',functions:JavaScriptErrorFunctionPlan[],exports:number[],nextBinding:number}} JavaScriptErrorModulePlan */
/** @typedef {{$:'errorConstructorOwner',source:number,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue,baseConstructor:JavaScriptSemanticValue,baseRealm:bigint} |
 * {$:'errorFunctionOwner',source:number,identity:bigint,realm:bigint}} JavaScriptErrorFunctionOwner */
/** @typedef {{$:'errorRealmOperations',realm:bigint,objectPrototype:JavaScriptSemanticValue,get:JavaScriptSemanticValue,assign:JavaScriptSemanticValue}} JavaScriptErrorRealmOperations */
/** @typedef {{$:'errorIntrinsicOwner',identity:bigint,realm:bigint}} JavaScriptErrorIntrinsicOwner */
/** @typedef {{$:'errorModuleRegistry',owners:JavaScriptErrorFunctionOwner[],operations:JavaScriptErrorRealmOperations[],intrinsics:JavaScriptErrorIntrinsicOwner[],realms:bigint[],exoticKey:JavaScriptSemanticValue}} JavaScriptErrorModuleRegistry */
/** @typedef {{$:'errorSourceContext',registry:JavaScriptErrorModuleRegistry,realm:JavaScriptErrorRealmOperations,next:number,visits:number,error:string}} JavaScriptErrorSourceContext */
/** @typedef {{$:'errorSourceExpression',source:JavaScriptCallSource,context:JavaScriptErrorSourceContext}} JavaScriptErrorSourceExpression */
/** @typedef {{$:'errorModuleCompilation',program:JavaScriptCallProgram,error:string}} JavaScriptErrorModuleCompilation */
/** @typedef {{$:'errorModuleBindingValue',identity:number,value:JavaScriptSemanticValue}} JavaScriptErrorModuleBinding */
/** @param {JavaScriptSemanticValue} value @returns {JavaScriptCallSource} */
export function constructJavaScriptErrorConstant(value){return {$:'sourceConstant',value};}
/** @param {JavaScriptErrorSourceContext} context @param {string} error @returns {JavaScriptErrorSourceExpression} */
export function refuseJavaScriptErrorSource(context,error){return {$:'errorSourceExpression',source:constructJavaScriptErrorConstant({$:'undefined'}),context:{...context,error}};}
/** Ordinary objects retain machine getter dispatch. Primitive property access
 * needs the real realm's boxing/null boundary; it cannot invoke a represented
 * object accessor through an untranslated host function.
 * @param {JavaScriptCallSource} receiver @param {JavaScriptCallSource} key
 * @param {JavaScriptErrorSourceContext} context @returns {JavaScriptCallSource} */
export function constructJavaScriptErrorPropertyRead(receiver,key,context){
 return {$:'sourceIf',condition:{$:'sourcePrimitive',operation:'test object value',arguments:[receiver]},yes:{$:'sourceGet',object:receiver,key},no:{$:'sourceCall',callee:constructJavaScriptErrorConstant(context.realm.get),receiver,arguments:[key]}};
}
/** @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptErrorExpression(expression,context,depth){
 const next={...context,visits:context.visits+1};if(next.error!=='')return refuseJavaScriptErrorSource(next,next.error);if(depth>128||next.visits>65536)return refuseJavaScriptErrorSource(next,'Error source expressions exceed their explicit structural budget');
 const constant=constructJavaScriptErrorConstant;
 switch(expression.$){
  case 'errorConstant':return {$:'errorSourceExpression',source:constant(expression.value),context:next};
  case 'errorBinding':case 'errorModuleBinding':return {$:'errorSourceExpression',source:{$:'sourceBinding',identity:expression.identity},context:next};
  case 'errorThis':return {$:'errorSourceExpression',source:{$:'sourceDerivedReceiver'},context:next};
  case 'errorFunctionReference':{const owner=next.registry.owners.find(owner=>owner.source===expression.identity);return owner===undefined?refuseJavaScriptErrorSource(next,'a source function reference needs its actual supplied callable identity'):{$:'errorSourceExpression',source:constant({$:'object',identity:owner.identity,callable:true}),context:next};}
  case 'errorGet':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next;return {$:'errorSourceExpression',source:{$:'sourceLet',identity,value:object.source,body:constructJavaScriptErrorPropertyRead({$:'sourceBinding',identity},key.source,key.context)},context:{...key.context,next:identity+1}};}
  case 'errorConditional':{const condition=lowerJavaScriptErrorExpression(expression.condition,next,depth+1),yes=lowerJavaScriptErrorExpression(expression.yes,condition.context,depth+1),no=lowerJavaScriptErrorExpression(expression.no,yes.context,depth+1);return {$:'errorSourceExpression',source:{$:'sourceIf',condition:condition.source,yes:yes.source,no:no.source},context:no.context};}
  case 'errorStrictEqual':{const left=lowerJavaScriptErrorExpression(expression.left,next,depth+1),right=lowerJavaScriptErrorExpression(expression.right,left.context,depth+1),source={$:'sourceStrictEqual',left:left.source,right:right.source};return {$:'errorSourceExpression',source:expression.negate?{$:'sourceIf',condition:source,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:source,context:right.context};}
  case 'errorObject':{let current=next;const properties=[];for(const property of expression.properties){const value=lowerJavaScriptErrorExpression(property.value,current,depth+1);current=value.context;properties.push({$:'sourceObjectProperty',key:constant({$:'text',units:property.key}),value:value.source});}return {$:'errorSourceExpression',source:{$:'sourceObject',prototype:constant(next.realm.objectPrototype),properties,realm:next.realm.realm},context:current};}
  case 'errorTemplate':{let current=next,source=constant({$:'text',units:[]});for(const part of expression.parts){if(part.$==='errorTemplateText')source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,constant({$:'text',units:part.units})]};else{const value=lowerJavaScriptErrorExpression(part.value,current,depth+1);current=value.context;source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,{$:'sourcePrimitive',operation:'convert primitive text',arguments:[value.source]}]};}}return {$:'errorSourceExpression',source,context:current};}
  case 'errorCall':{const callee=lowerJavaScriptErrorExpression(expression.callee,next,depth+1),receiver=lowerJavaScriptErrorExpression(expression.receiver,callee.context,depth+1);let current=receiver.context,arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}return {$:'errorSourceExpression',source:expression.construct?{$:'sourceConstructErrorClass',callee:callee.source,arguments:arguments_}:{$:'sourceCall',callee:callee.source,receiver:receiver.source,arguments:arguments_},context:current};}
  case 'errorMethodCall':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next;let current={...key.context,next:identity+1},arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}const receiver={$:'sourceBinding',identity};return {$:'errorSourceExpression',source:{$:'sourceLet',identity,value:object.source,body:{$:'sourceCall',callee:constructJavaScriptErrorPropertyRead(receiver,key.source,next),receiver,arguments:arguments_}},context:current};}
 }
}
/** @param {JavaScriptErrorModulePlan} plan @param {JavaScriptErrorModuleRegistry} registry @returns {JavaScriptErrorModuleCompilation} */
export function compileJavaScriptErrorModulePlan(plan,registry){
 return compileJavaScriptErrorModulePlanWithBindings(plan,registry,[]);
}
/** Actual initialized module captures are values/handles, not literal policy
 * snapshots. All functions receive the same values in existing capture slots.
 * @param {JavaScriptErrorModulePlan} plan @param {JavaScriptErrorModuleRegistry} registry
 * @param {JavaScriptErrorModuleBinding[]} bindings @returns {JavaScriptErrorModuleCompilation} */
export function compileJavaScriptErrorModulePlanWithBindings(plan,registry,bindings){
 const empty={$:'callProgram',functions:[],realms:registry.realms,exoticKey:registry.exoticKey};
 if(bindings.length>4096)return {$:'errorModuleCompilation',program:empty,error:'actual module captures exceed the source frame budget'};
 for(let at=0;at<bindings.length;at++){const binding=bindings[at];if(binding===undefined||!Number.isSafeInteger(binding.identity)||binding.identity<0||binding.identity>=plan.nextBinding||bindings.findIndex(row=>row.identity===binding.identity)!==at||readJavaScriptSemanticValueError(binding.value)!=='')return {$:'errorModuleCompilation',program:empty,error:'module captures require distinct actual lexical owners and semantic values'};}
 if(plan.functions.length>4096||registry.owners.length!==plan.functions.length||registry.intrinsics.length>4096||plan.nextBinding<0||!Number.isSafeInteger(plan.nextBinding))return {$:'errorModuleCompilation',program:empty,error:'Error module plans need complete bounded source and callable registries'};
 let functions=[];const identities=[];
 for(const definition of plan.functions){
  const matches=registry.owners.filter(owner=>owner.source===definition.identity);if(matches.length!==1)return {$:'errorModuleCompilation',program:empty,error:'every actual source definition needs exactly one supplied callable owner'};const owner=matches[0];if(owner===undefined)return {$:'errorModuleCompilation',program:empty,error:'source callable owners must be dense'};
  if(identities.includes(owner.identity)||!registry.realms.includes(owner.realm))return {$:'errorModuleCompilation',program:empty,error:'source callable owners need distinct identities and actual realms'};identities.push(owner.identity);
  const operations=registry.operations.filter(row=>row.realm===owner.realm);if(operations.length!==1)return {$:'errorModuleCompilation',program:empty,error:'Error source operations require their actual complete source realm'};const realm=operations[0];if(realm===undefined)return {$:'errorModuleCompilation',program:empty,error:'Error source realm rows must be dense'};
  let context={$:'errorSourceContext',registry,realm,next:plan.nextBinding,visits:0,error:''},source=constructJavaScriptErrorConstant({$:'undefined'});
  if(definition.$==='errorConstructorPlan'){
   if(owner.$!=='errorConstructorOwner'||!registry.realms.includes(owner.baseRealm))return {$:'errorModuleCompilation',program:empty,error:'an Error subclass requires its actual base constructor and realm authority'};
   const message=lowerJavaScriptErrorExpression(definition.message,context,0);context=message.context;
   const messageIdentity=context.next;context={...context,next:context.next+1};const raw={$:'sourceBinding',identity:messageIdentity};
   const absent={$:'sourceStrictEqual',left:raw,right:constructJavaScriptErrorConstant({$:'undefined'})};
   const construct={$:'sourceConstructErrorValue',constructor:constructJavaScriptErrorConstant(owner.baseConstructor),newTarget:constructJavaScriptErrorConstant({$:'object',identity:owner.identity,callable:true}),prototype:constructJavaScriptErrorConstant(owner.prototype),message:{$:'sourceIf',condition:absent,yes:constructJavaScriptErrorConstant({$:'text',units:[]}),no:{$:'sourcePrimitive',operation:'convert primitive text',arguments:[raw]}},present:{$:'sourceIf',condition:absent,yes:constructJavaScriptErrorConstant({$:'boolean',value:false}),no:constructJavaScriptErrorConstant({$:'boolean',value:true})},realm:owner.baseRealm,errorClass:definition.baseClass,bindReceiver:true};
   const writes=[];for(const field of definition.fields){const value=lowerJavaScriptErrorExpression(field.value,context,0);context=value.context;writes.push({$:'sourceCall',callee:constructJavaScriptErrorConstant(realm.assign),receiver:{$:'sourceDerivedReceiver'},arguments:[constructJavaScriptErrorConstant({$:'text',units:field.name}),value.source]});}
   source={$:'sourceDerivedReceiver'};for(let at=writes.length-1;at>=0;at-=1){const identity=context.next;context={...context,next:identity+1};source={$:'sourceLet',identity,value:writes[at]??constructJavaScriptErrorConstant({$:'undefined'}),body:source};}
   const superIdentity=context.next;context={...context,next:superIdentity+1};source={$:'sourceLet',identity:messageIdentity,value:message.source,body:{$:'sourceLet',identity:superIdentity,value:construct,body:source}};
  }else{if(owner.$!=='errorFunctionOwner')return {$:'errorModuleCompilation',program:empty,error:'ordinary source functions cannot borrow constructor privileges'};const body=lowerJavaScriptErrorExpression(definition.body,context,0);context=body.context;source=definition.abrupt?{$:'sourceThrow',value:body.source}:body.source;}
  for(let at=definition.parameters.length-1;at>=0;at-=1){const parameter=definition.parameters[at];if(parameter!==undefined&&parameter.$==='errorDefaultParameter'){const value=lowerJavaScriptErrorExpression(parameter.value,context,0);context=value.context;source={$:'sourceParameterDefault',identity:parameter.identity,value:value.source,body:source};}}
  if(context.error!=='')return {$:'errorModuleCompilation',program:empty,error:context.error};
  const code=compileJavaScriptCallSourceWithCaptures(source,definition.parameters.map(parameter=>parameter.identity),bindings.map(binding=>binding.identity));if(code.error!=='')return {$:'errorModuleCompilation',program:empty,error:code.error};
  functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:definition.$==='errorConstructorPlan'?'derived Error':'strict',lexicalReceiver:{$:'undefined'},parameters:definition.parameters.length,registers:code.registers,captures:bindings.map(binding=>binding.value),instructions:code.instructions});
 }
 for(const intrinsic of registry.intrinsics){if(identities.includes(intrinsic.identity)||!registry.realms.includes(intrinsic.realm))return {$:'errorModuleCompilation',program:empty,error:'external Error callbacks require distinct actual callable/realm identities'};identities.push(intrinsic.identity);functions.push({$:'callFunction',identity:intrinsic.identity,realm:intrinsic.realm,receiverMode:'Error intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'invokeErrorIntrinsic',destination:1},{$:'return',source:1}]});}
 functions=functions.toSorted((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0);
 return {$:'errorModuleCompilation',program:{$:'callProgram',functions,realms:registry.realms,exoticKey:registry.exoticKey},error:''};
}
