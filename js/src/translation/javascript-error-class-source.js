// Source-class plans contain actual lexical binding coordinates. Runtime
// identities/prototypes come from the supplied realm registry, never numbers
// or class names invented by this compiler. No class body executes here.
import {compileJavaScriptCallSourceWithCaptures,compileJavaScriptMutableCallSourceWithCaptures} from './javascript-call-source.js';
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
/** @typedef {{$:'errorConstant',value:JavaScriptSemanticValue} |
 * {$:'errorClosure',owner:number,ordinal:number,captures:number[],receiverMode?:string} |
 * {$:'errorArgumentSpread',value:JavaScriptErrorExpression,iteratorKey:JavaScriptErrorExpression} |
 * {$:'errorBinding',identity:number} | {$:'errorModuleBinding',identity:number} | {$:'errorThis'} |
 * {$:'errorConditional',condition:JavaScriptErrorExpression,yes:JavaScriptErrorExpression,no:JavaScriptErrorExpression} |
 * {$:'errorGet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression} |
 * {$:'errorComputedGet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression} |
 * {$:'errorComputedSet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression,value:JavaScriptErrorExpression} |
 * {$:'errorComputedMethodCall',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression,arguments:JavaScriptErrorExpression[]} |
 * {$:'errorOptionalGet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression} |
 * {$:'errorSet',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression,value:JavaScriptErrorExpression} |
 * {$:'errorModuleCell',identity:number} | {$:'errorModuleCellSet',identity:number,value:JavaScriptErrorExpression} |
 * {$:'errorUnary',operator:string,value:JavaScriptErrorExpression} |
 * {$:'errorUpdate',operator:string,target:JavaScriptErrorExpression,prefix:boolean} |
 * {$:'errorCompoundAssignment',operator:string,target:JavaScriptErrorExpression,value:JavaScriptErrorExpression} |
 * {$:'errorBinary',operator:string,left:JavaScriptErrorExpression,right:JavaScriptErrorExpression,port:string} |
 * {$:'errorArray',factory:string,items:JavaScriptErrorExpression[]} |
 * {$:'errorArrayAccumulation',parts:JavaScriptErrorArrayPart[],iteratorKey:JavaScriptErrorExpression} |
 * {$:'errorArrayLiteral',length:number,elements:JavaScriptErrorArrayElement[]} |
 * {$:'errorObject',properties:JavaScriptErrorObjectProperty[]} |
 * {$:'errorTemplate',parts:JavaScriptErrorTemplatePart[]} |
 * {$:'errorCall',callee:JavaScriptErrorExpression,receiver:JavaScriptErrorExpression,arguments:JavaScriptErrorExpression[],construct:boolean} |
 * {$:'errorMethodCall',object:JavaScriptErrorExpression,key:JavaScriptErrorExpression,arguments:JavaScriptErrorExpression[]} |
 * {$:'errorFunctionReference',identity:number} | {$:'errorNativeReference',source:string} |
 * {$:'errorReceiver'} | {$:'errorOrdinaryConstruction',identity:number,arguments:JavaScriptErrorExpression[]} |
 * {$:'errorCollectionConstruction',source:string,arguments:JavaScriptErrorExpression[]} |
 * {$:'errorBindingSet',identity:number,value:JavaScriptErrorExpression} |
 * {$:'errorSpecialNumber',form:string} |
 * {$:'errorStrictEqual',left:JavaScriptErrorExpression,right:JavaScriptErrorExpression,negate:boolean}} JavaScriptErrorExpression */
/** @typedef {{$:'errorObjectProperty',key:number[],value:JavaScriptErrorExpression} | {$:'errorObjectAccessor',key:number[],value:JavaScriptErrorExpression,kind:string} | {$:'errorLiteralPrototype',value:JavaScriptErrorExpression} | {$:'errorComputedObjectProperty',key:JavaScriptErrorExpression,value:JavaScriptErrorExpression,inferName?:boolean,kind?:string} | {$:'errorObjectSpread',input:JavaScriptErrorExpression}} JavaScriptErrorObjectProperty */
/** @typedef {{$:'errorArrayValue',value:JavaScriptErrorExpression} | {$:'errorArraySpread',value:JavaScriptErrorExpression} | {$:'errorArrayHole'}} JavaScriptErrorArrayPart */
/** @typedef {{$:'errorArrayElement',index:number,value:JavaScriptErrorExpression}} JavaScriptErrorArrayElement */
/** @typedef {{$:'errorTemplateText',units:number[]} | {$:'errorTemplateValue',value:JavaScriptErrorExpression}} JavaScriptErrorTemplatePart */
/** @typedef {{$:'errorParameterField',identity:number,key:number[],hasDefault:boolean,value:JavaScriptErrorExpression,keyExpression?:JavaScriptErrorExpression,pattern?:JavaScriptErrorParameter}} JavaScriptErrorParameterField */
/** @typedef {{$:'errorRestParameter',identity:number,pattern?:JavaScriptErrorParameter,shared?:boolean} | {$:'errorRequiredParameter',identity:number,shared?:boolean} | {$:'errorDefaultParameter',identity:number,value:JavaScriptErrorExpression,shared?:boolean} | {$:'errorObjectParameter',identity:number,hasDefault:boolean,value:JavaScriptErrorExpression,fields:JavaScriptErrorParameterField[],rest?:number} | {$:'errorArrayParameter',identity:number,hasDefault:boolean,value:JavaScriptErrorExpression,iteratorKey:JavaScriptErrorExpression,elements:JavaScriptErrorArrayBindingPart[],rest?:JavaScriptErrorArrayBindingElement}} JavaScriptErrorParameter */
/** @typedef {{$:'errorArrayBindingElement',identity:number,hasDefault:boolean,value:JavaScriptErrorExpression,pattern?:JavaScriptErrorParameter}} JavaScriptErrorArrayBindingElement */
/** @typedef {JavaScriptErrorArrayBindingElement | {$:'errorArrayElision'}} JavaScriptErrorArrayBindingPart */
/** @typedef {{$:'sourceForOfStatement',target:number,binding:number,mutable:boolean,input:JavaScriptErrorExpression,iteratorKey:JavaScriptErrorExpression,body:JavaScriptSourceStatement[],pattern?:JavaScriptErrorParameter} | {$:'sourceFinallyStatement',body:JavaScriptSourceStatement[],finalizer:JavaScriptSourceStatement[]} | {$:'sourceTryStatement',identity:number,body:JavaScriptSourceStatement[],handler:JavaScriptSourceStatement[],pattern?:JavaScriptErrorParameter} | {$:'sourceArrayBindingStatement',pattern:JavaScriptErrorParameter,value:JavaScriptErrorExpression,mutable:boolean} | {$:'sourceObjectBindingStatement',pattern:JavaScriptErrorParameter,value:JavaScriptErrorExpression,mutable:boolean} | {$:'sourceConstStatement',identity:number,value:JavaScriptErrorExpression} | {$:'sourceMutableStatement',identity:number,value:JavaScriptErrorExpression,shared?:boolean} | {$:'sourceExpressionStatement',value:JavaScriptErrorExpression} | {$:'sourceReturnStatement',value:JavaScriptErrorExpression} | {$:'sourceThrowStatement',value:JavaScriptErrorExpression} | {$:'sourceIfStatement',condition:JavaScriptErrorExpression,yes:JavaScriptSourceStatement[],no:JavaScriptSourceStatement[]} | {$:'sourceBlockStatement',statements:JavaScriptSourceStatement[]} | {$:'sourceDeclarationGroupStatement',statements:JavaScriptSourceStatement[]} | {$:'sourceWhileStatement',condition:JavaScriptErrorExpression,body:JavaScriptSourceStatement[],target?:number,update?:JavaScriptErrorExpression,postTest?:boolean} | {$:'sourceControlBlockStatement',target:number,body:JavaScriptSourceStatement[]} | {$:'sourceControlJumpStatement',target:number,continuing:boolean} | {$:'sourceSwitchStatement',target:number,value:JavaScriptErrorExpression,clauses:JavaScriptSourceSwitchClause[]}} JavaScriptSourceStatement */
/** @typedef {{$:'sourceCaseStatement',value:JavaScriptErrorExpression,body:JavaScriptSourceStatement[]} | {$:'sourceDefaultStatement',body:JavaScriptSourceStatement[]}} JavaScriptSourceSwitchClause */
/** @typedef {{$:'errorFieldAssignment',name:number[],value:JavaScriptErrorExpression}} JavaScriptErrorFieldAssignment */
/** @typedef {{$:'errorConstructorPlan',identity:number,name:string,baseClass:string,parameters:JavaScriptErrorParameter[],message:JavaScriptErrorExpression,fields:JavaScriptErrorFieldAssignment[]} |
 * {$:'errorFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],body:JavaScriptErrorExpression,abrupt:boolean} |
 * {$:'errorOrdinaryConstructorPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorMethodFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorMutableStatementFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorStatementFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]}} JavaScriptErrorFunctionPlan */
/** @typedef {{$:'errorArrowTemplatePlan',owner:number,ordinal:number,name:string,length:number,parameters:JavaScriptErrorParameter[],captures:number[],statements:JavaScriptSourceStatement[],receiverMode?:string}} JavaScriptErrorArrowTemplatePlan */
/** @typedef {{$:'errorModulePlan',functions:JavaScriptErrorFunctionPlan[],exports:number[],nextBinding:number,templates?:JavaScriptErrorArrowTemplatePlan[],sharedBindings?:number[]}} JavaScriptErrorModulePlan */
/** @typedef {{$:'errorConstructorOwner',source:number,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue,baseConstructor:JavaScriptSemanticValue,baseRealm:bigint} |
 * {$:'ordinaryConstructorOwner',source:number,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue} |
 * {$:'errorFunctionOwner',source:number,identity:bigint,realm:bigint}} JavaScriptErrorFunctionOwner */
/** @typedef {{$:'errorRealmOperations',realm:bigint,objectPrototype:JavaScriptSemanticValue,arrayPrototype?:JavaScriptSemanticValue,functionPrototype?:JavaScriptSemanticValue,get:JavaScriptSemanticValue,assign:JavaScriptSemanticValue}} JavaScriptErrorRealmOperations */
/** @typedef {{$:'errorIntrinsicOwner',identity:bigint,realm:bigint}} JavaScriptErrorIntrinsicOwner */
/** @typedef {{$:'errorModuleRegistry',owners:JavaScriptErrorFunctionOwner[],operations:JavaScriptErrorRealmOperations[],intrinsics:JavaScriptErrorIntrinsicOwner[],realms:bigint[],exoticKey:JavaScriptSemanticValue}} JavaScriptErrorModuleRegistry */
/** @typedef {{$:'sourceCapabilityOwner',source:string,identity:bigint,realm:bigint,operation:string,owner:string,callable:boolean,executable:boolean}} JavaScriptSourceCapabilityOwner */
/** @typedef {{$:'errorSourceContext',registry:JavaScriptErrorModuleRegistry,realm:JavaScriptErrorRealmOperations,capabilities:JavaScriptSourceCapabilityOwner[],next:number,visits:number,error:string,mutableBindings:boolean,globalBindings?:number[],sharedBindings?:number[]}} JavaScriptErrorSourceContext */
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
/** Evaluate the key expression before RequireObjectCoercible, but defer its
 * ToPropertyKey conversion until after the nullish receiver check. The actual
 * primitive-get intrinsic supplies that real TypeError; it never invokes a
 * represented key conversion on this branch.
 * @param {JavaScriptCallSource} receiver @param {JavaScriptCallSource} key
 * @param {JavaScriptErrorSourceContext} context @returns {JavaScriptCallSource} */
export function constructJavaScriptSourceComputedRead(receiver,key,context){
 const failure={$:'sourceCall',callee:constructJavaScriptErrorConstant(context.realm.get),receiver,arguments:[key]},read=constructJavaScriptErrorPropertyRead(receiver,{$:'sourcePropertyKey',input:key,realm:context.realm.realm},context);
 return {$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constructJavaScriptErrorConstant({$:'null'})},yes:failure,no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constructJavaScriptErrorConstant({$:'undefined'})},yes:failure,no:read}};
}
/** @param {string} key @returns {JavaScriptCallSource} */
export function constructJavaScriptSourceFieldKey(key){const units=[];for(let at=0;at<key.length;at++)units.push(key.charCodeAt(at));return constructJavaScriptErrorConstant({$:'text',units});}
/** @param {JavaScriptErrorSourceContext} context @param {number} identity
 * @param {JavaScriptCallSource} value @param {JavaScriptCallSource} body
 * @param {boolean} mutable @returns {JavaScriptCallSource} */
export function constructJavaScriptErrorLocalBinding(context,identity,value,body,mutable){return context.mutableBindings?{$:'sourceLocalLet',identity,value,body,mutable}:{$:'sourceLet',identity,value,body};}
/** @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptErrorExpression(expression,context,depth){
 const next={...context,visits:context.visits+1};if(next.error!=='')return refuseJavaScriptErrorSource(next,next.error);if(depth>128||next.visits>65536)return refuseJavaScriptErrorSource(next,'Error source expressions exceed their explicit structural budget');
 const constant=constructJavaScriptErrorConstant;
 switch(expression.$){
  case 'errorConstant':return {$:'errorSourceExpression',source:constant(expression.value),context:next};
  case 'errorSpecialNumber':return expression.form==='NaN'||expression.form==='Infinity'?{$:'errorSourceExpression',source:constant({$:'number',value:expression.form==='NaN'?NaN:Infinity}),context:next}:refuseJavaScriptErrorSource(next,'a special Number requires its actual unshadowed source global');
  case 'errorArgumentSpread':{const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1),key=lowerJavaScriptErrorExpression(expression.iteratorKey,value.context,depth+1);return {$:'errorSourceExpression',source:{$:'sourceArgumentSpread',value:value.source,iteratorKey:key.source,realm:next.realm.realm},context:key.context};}
  case 'errorClosure':{
   const owner=next.registry.owners.find(row=>row.source===expression.owner);
   if(owner===undefined||owner.realm!==next.realm.realm)return refuseJavaScriptErrorSource(next,'arrow creation retains its actual source function owner and realm');
   const identities=(next.globalBindings??[]).concat(expression.captures);
   if(identities.length>4096||identities.some((identity,index)=>identities.indexOf(identity)!==index))return refuseJavaScriptErrorSource(next,'arrow captures require distinct actual lexical coordinates within their budget');
   if(expression.receiverMode!==undefined&&expression.receiverMode!=='strict'&&expression.receiverMode!=='lexical')return refuseJavaScriptErrorSource(next,'runtime function creation retains its actual strict or lexical receiver policy');
   return {$:'errorSourceExpression',source:{$:'sourceClosure',owner:owner.identity,ordinal:expression.ordinal,captures:identities.map(identity=>({$:'sourceBinding',identity})),receiver:expression.receiverMode==='strict'?constant({$:'undefined'}):{$:'sourceReceiver'}},context:next};
  }
  case 'errorBinding':case 'errorModuleBinding':return {$:'errorSourceExpression',source:{$:'sourceBinding',identity:expression.identity},context:next};
  case 'errorThis':return {$:'errorSourceExpression',source:{$:'sourceDerivedReceiver'},context:next};
  case 'errorReceiver':return {$:'errorSourceExpression',source:{$:'sourceReceiver'},context:next};
  case 'errorBindingSet':{if(!next.mutableBindings)return refuseJavaScriptErrorSource(next,'mutable lexical assignment requires its explicit source profile');const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1);return {...value,source:{$:'sourceAssignBinding',identity:expression.identity,value:value.source}};}
  case 'errorOrdinaryConstruction':{
   if(!next.mutableBindings)return refuseJavaScriptErrorSource(next,'ordinary class construction requires its explicit source profile');
   const owner=next.registry.owners.find(row=>row.source===expression.identity);if(owner===undefined||owner.$!=='ordinaryConstructorOwner')return refuseJavaScriptErrorSource(next,'ordinary construction requires its actual supplied class and prototype owner');
   if(expression.arguments.some(argument=>argument.$==='errorArgumentSpread')){
    let current=next;const arguments_=[];for(const argument of expression.arguments){const lowered=lowerJavaScriptErrorExpression(argument,current,depth+1);current=lowered.context;arguments_.push(lowered.source);}
    return {$:'errorSourceExpression',source:{$:'sourceVariadicCall',callee:constant({$:'object',identity:owner.identity,callable:true}),receiver:{$:'sourceAllocate',prototype:constant(owner.prototype)},arguments:arguments_,mode:'ordinary class',receiverAfterArguments:true},context:current};
   }
   let current=next;/** @type {{identity:number,value:JavaScriptCallSource}[]} */ const captures=[];
   for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1),identity=value.context.next;current={...value.context,next:identity+1};captures.push({identity,value:value.source});}
   const identity=current.next;current={...current,next:identity+1};const receiver={$:'sourceBinding',identity};
   let source=constructJavaScriptErrorLocalBinding(current,identity,{$:'sourceAllocate',prototype:constant(owner.prototype)},{$:'sourceClassConstructorCall',callee:constant({$:'object',identity:owner.identity,callable:true}),receiver,arguments:captures.map(row=>({$:'sourceBinding',identity:row.identity}))},false);
   for(let at=captures.length-1;at>=0;at--){const capture=captures[at];if(capture!==undefined)source=constructJavaScriptErrorLocalBinding(current,capture.identity,capture.value,source,false);}
   return {$:'errorSourceExpression',source,context:current};
  }
  case 'errorCollectionConstruction':{
   if(!next.mutableBindings)return refuseJavaScriptErrorSource(next,'standard collection construction requires its explicit source profile');
   const owner=next.capabilities.find(row=>row.source===expression.source);if(owner===undefined||!owner.callable||owner.executable)return refuseJavaScriptErrorSource(next,'collection construction requires its actual portable standard constructor reference');
   let current=next;const arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}
   return {$:'errorSourceExpression',source:{$:'sourceConstructCollection',callee:constant({$:'object',identity:owner.identity,callable:true}),arguments:arguments_},context:current};
  }
  case 'errorFunctionReference':{const owner=next.registry.owners.find(owner=>owner.source===expression.identity);return owner===undefined?refuseJavaScriptErrorSource(next,'a source function reference needs its actual supplied callable identity'):{$:'errorSourceExpression',source:constant({$:'object',identity:owner.identity,callable:true}),context:next};}
  case 'errorNativeReference':{const owners=next.capabilities.filter(owner=>owner.source===expression.source);const owner=owners[0];return owners.length!==1||owner===undefined?refuseJavaScriptErrorSource(next,'a native reference requires one actual source-qualified capability owner'):{$:'errorSourceExpression',source:constant({$:'object',identity:owner.identity,callable:owner.callable}),context:next};}
  case 'errorGet':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next;return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(key.context,identity,object.source,constructJavaScriptErrorPropertyRead({$:'sourceBinding',identity},key.source,key.context),false),context:{...key.context,next:identity+1}};}
  case 'errorComputedGet':case 'errorComputedMethodCall':{
   const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),receiverID=key.context.next,keyID=receiverID+1;
   let current={...key.context,next:keyID+1},read=constructJavaScriptSourceComputedRead({$:'sourceBinding',identity:receiverID},{$:'sourceBinding',identity:keyID},current);
   if(expression.$==='errorComputedMethodCall'){const arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}read={$:'sourceSnapshotCall',callee:read,receiver:{$:'sourceBinding',identity:receiverID},arguments:arguments_};}
   return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(current,receiverID,object.source,constructJavaScriptErrorLocalBinding(current,keyID,key.source,read,false),false),context:current};
  }
  case 'errorComputedSet':{
   // A computed Reference retains the raw key. Simple assignment evaluates and
   // captures its RHS BEFORE PutValue performs ToObject and ToPropertyKey.
   // https://tc39.es/ecma262/2026/multipage/ecmascript-language-expressions.html#sec-evaluate-property-access-with-expression-key
   const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),receiverID=key.context.next,keyID=receiverID+1,valueID=keyID+1,value=lowerJavaScriptErrorExpression(expression.value,{...key.context,next:valueID+1},depth+1),receiver={$:'sourceBinding',identity:receiverID},rawKey={$:'sourceBinding',identity:keyID},assigned={$:'sourceBinding',identity:valueID};
   const failure={$:'sourceCall',callee:constant(next.realm.assign),receiver,arguments:[rawKey,assigned]},converted={$:'sourcePropertyKey',input:rawKey,realm:next.realm.realm},checkedKey={$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constant({$:'null'})},yes:failure,no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constant({$:'undefined'})},yes:failure,no:converted}},write={$:'sourceSnapshotSet',object:receiver,key:checkedKey,value:assigned};
   return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(value.context,receiverID,object.source,constructJavaScriptErrorLocalBinding(value.context,keyID,key.source,constructJavaScriptErrorLocalBinding(value.context,valueID,value.source,write,false),false),false),context:value.context};
  }
  case 'errorOptionalGet':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next,value={$:'sourceBinding',identity},absent=constant({$:'undefined'}),read=constructJavaScriptErrorPropertyRead(value,key.source,key.context);return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(key.context,identity,object.source,{$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:absent},yes:absent,no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:'null'})},yes:absent,no:read}},false),context:{...key.context,next:identity+1}};}
  case 'errorModuleCell':return {$:'errorSourceExpression',source:{$:'sourceGet',object:{$:'sourceBinding',identity:expression.identity},key:constructJavaScriptSourceFieldKey('value')},context:next};
  case 'errorModuleCellSet':{const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1);return {$:'errorSourceExpression',source:{$:'sourceSet',object:{$:'sourceBinding',identity:expression.identity},key:constructJavaScriptSourceFieldKey('value'),value:value.source},context:value.context};}
  case 'errorSet':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),value=lowerJavaScriptErrorExpression(expression.value,key.context,depth+1);return {$:'errorSourceExpression',source:{$:next.mutableBindings?'sourceSnapshotSet':'sourceSet',object:object.source,key:key.source,value:value.source},context:value.context};}
  case 'errorUnary':{
   const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1);
   if(expression.operator==='!')return {$:'errorSourceExpression',source:{$:'sourceIf',condition:value.source,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})},context:value.context};
   if(expression.operator==='typeof')return {$:'errorSourceExpression',source:{$:'sourcePrimitive',operation:'read semantic type',arguments:[value.source]},context:value.context};
   if(expression.operator==='+')return {$:'errorSourceExpression',source:{$:'sourcePrimitive',operation:'coerce primitive number',arguments:[value.source]},context:value.context};
   if(expression.operator==='-')return {$:'errorSourceExpression',source:{$:'sourcePrimitive',operation:'negate numeric value',arguments:[{$:'sourcePrimitive',operation:'coerce primitive numeric',arguments:[value.source]}]},context:value.context};
   return refuseJavaScriptErrorSource(value.context,'this unary source operator requires its own semantic profile');
  }
  case 'errorArray':{const owner=next.capabilities.find(owner=>owner.source===expression.factory);if(owner===undefined||!owner.executable)return refuseJavaScriptErrorSource(next,'array literals require their actual protected realm constructor capability');let current=next;const arguments_=[];for(const item of expression.items){const value=lowerJavaScriptErrorExpression(item,current,depth+1);current=value.context;arguments_.push(value.source);}return {$:'errorSourceExpression',source:{$:'sourceCall',callee:constant({$:'object',identity:owner.identity,callable:owner.callable}),receiver:constant({$:'undefined'}),arguments:arguments_},context:current};}
  case 'errorCompoundAssignment':return lowerJavaScriptSourceCompoundAssignment(expression,next,depth+1);
  case 'errorUpdate':return lowerJavaScriptSourceUpdate(expression,next,depth+1);
  case 'errorBinary':return lowerJavaScriptSourceBinary(expression,next,depth+1);
  case 'errorConditional':{const condition=lowerJavaScriptErrorExpression(expression.condition,next,depth+1),yes=lowerJavaScriptErrorExpression(expression.yes,condition.context,depth+1),no=lowerJavaScriptErrorExpression(expression.no,yes.context,depth+1);return {$:'errorSourceExpression',source:{$:'sourceIf',condition:condition.source,yes:yes.source,no:no.source},context:no.context};}
  case 'errorStrictEqual':{const left=lowerJavaScriptErrorExpression(expression.left,next,depth+1),right=lowerJavaScriptErrorExpression(expression.right,left.context,depth+1),source={$:next.mutableBindings?'sourceSnapshotStrictEqual':'sourceStrictEqual',left:left.source,right:right.source};return {$:'errorSourceExpression',source:expression.negate?{$:'sourceIf',condition:source,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:source,context:right.context};}
  case 'errorArrayAccumulation':{
   if(!next.mutableBindings||next.realm.arrayPrototype===undefined)return refuseJavaScriptErrorSource(next,'Array accumulation requires its actual ordinary-source realm');
   const iteratorKey=lowerJavaScriptErrorExpression(expression.iteratorKey,next,depth+1);let current=iteratorKey.context;const parts=[];
   for(const part of expression.parts){if(part.$==='errorArrayHole'){parts.push({$:'sourceArrayHole'});continue;}const value=lowerJavaScriptErrorExpression(part.value,current,depth+1);current=value.context;parts.push({$:part.$==='errorArraySpread'?'sourceArraySpread':'sourceArrayValue',value:value.source});}
   return {$:'errorSourceExpression',source:{$:'sourceArrayAccumulation',prototype:constant(next.realm.arrayPrototype),parts,iteratorKey:iteratorKey.source,realm:next.realm.realm},context:current};
  }
  case 'errorArrayLiteral':{
   if(!next.mutableBindings||next.realm.arrayPrototype===undefined)return refuseJavaScriptErrorSource(next,'Array literals require the actual ordinary-source realm Array prototype');
   let current=next;const elements=[];for(const element of expression.elements){const value=lowerJavaScriptErrorExpression(element.value,current,depth+1);current=value.context;elements.push({$:'sourceArrayElement',index:element.index,value:value.source});}
   return {$:'errorSourceExpression',source:{$:'sourceArray',prototype:constant(next.realm.arrayPrototype),length:expression.length,elements,realm:next.realm.realm},context:current};
  }
  case 'errorObject':{
   let current=next;const properties=[];
   for(const property of expression.properties){
    if(property.$==='errorLiteralPrototype'){const value=lowerJavaScriptErrorExpression(property.value,current,depth+1);current=value.context;properties.push({$:'sourceLiteralPrototype',value:value.source});continue;}
    if(property.$==='errorObjectSpread'){const input=lowerJavaScriptErrorExpression(property.input,current,depth+1);current=input.context;properties.push({$:'sourceObjectSpread',input:input.source});continue;}
    let key=constant({$:'undefined'});if(property.$==='errorComputedObjectProperty'){const lowered=lowerJavaScriptErrorExpression(property.key,current,depth+1);current=lowered.context;key=lowered.source;}else key=constant({$:'text',units:property.key});
    const value=lowerJavaScriptErrorExpression(property.value,current,depth+1);current=value.context;const inferName=property.$==='errorComputedObjectProperty'&&property.inferName?{inferName:true}:{};
    properties.push(property.$==='errorObjectAccessor'||property.$==='errorComputedObjectProperty'&&property.kind!==undefined&&property.kind!==''?{$:'sourceObjectAccessor',key,value:value.source,kind:property.kind??'',...inferName}:{$:'sourceObjectProperty',key,value:value.source,...inferName});
   }
   return {$:'errorSourceExpression',source:{$:'sourceObject',prototype:constant(next.realm.objectPrototype),properties,realm:next.realm.realm},context:current};
  }
  case 'errorTemplate':{let current=next,source=constant({$:'text',units:[]});for(const part of expression.parts){if(part.$==='errorTemplateText')source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,constant({$:'text',units:part.units})]};else{const value=lowerJavaScriptErrorExpression(part.value,current,depth+1);current=value.context;source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,{$:'sourcePrimitive',operation:'convert primitive text',arguments:[value.source]}]};}}return {$:'errorSourceExpression',source,context:current};}
  case 'errorCall':{const callee=lowerJavaScriptErrorExpression(expression.callee,next,depth+1),receiver=lowerJavaScriptErrorExpression(expression.receiver,callee.context,depth+1);let current=receiver.context,arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}return {$:'errorSourceExpression',source:expression.construct?{$:'sourceConstructErrorClass',callee:callee.source,arguments:arguments_}:{$:next.mutableBindings?'sourceSnapshotCall':'sourceCall',callee:callee.source,receiver:receiver.source,arguments:arguments_},context:current};}
  case 'errorMethodCall':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next;let current={...key.context,next:identity+1},arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}const receiver={$:'sourceBinding',identity};return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(current,identity,object.source,{$:next.mutableBindings?'sourceSnapshotCall':'sourceCall',callee:constructJavaScriptErrorPropertyRead(receiver,key.source,next),receiver,arguments:arguments_},false),context:current};}
 }
}
/** @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceBinary(expression,context,depth){
 if(expression.$!=='errorBinary')return refuseJavaScriptErrorSource(context,'binary source lowering needs its actual operator node');
 const left=lowerJavaScriptErrorExpression(expression.left,context,depth+1),first=left.context.next,right=lowerJavaScriptErrorExpression(expression.right,{...left.context,next:first+1},depth+1),second=right.context.next,next={...right.context,next:second+1},a={$:'sourceBinding',identity:first},b={$:'sourceBinding',identity:second},constant=constructJavaScriptErrorConstant;
 if(expression.operator===',')return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,first,left.source,right.source,false),context:next};
 if(expression.operator==='??'){
  const nullish={$:'sourceIf',condition:{$:'sourceStrictEqual',left:a,right:constant({$:'null'})},yes:constant({$:'boolean',value:true}),no:{$:'sourceStrictEqual',left:a,right:constant({$:'undefined'})}};
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,first,left.source,{$:'sourceIf',condition:nullish,yes:right.source,no:a},false),context:next};
 }
 if(expression.operator==='&&'||expression.operator==='||')return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,first,left.source,{$:'sourceIf',condition:a,yes:expression.operator==='&&'?right.source:a,no:expression.operator==='&&'?a:right.source},false),context:next};
 if(context.mutableBindings&&['+','-','*','/','%','**'].includes(expression.operator)){
  const firstConverted=next.next,secondConverted=firstConverted+1,complete={...next,next:secondConverted+1},convert=expression.operator==='+'?'convert primitive value':'coerce primitive numeric',operation=expression.operator==='+'?'add primitive values':expression.operator==='-'?'subtract numeric values':expression.operator==='*'?'multiply numeric values':expression.operator==='/'?'divide numeric values':expression.operator==='%'?'read numeric remainder':'raise numeric power';
  const convertedA={$:'sourceBinding',identity:firstConverted},convertedB={$:'sourceBinding',identity:secondConverted};
  const body=constructJavaScriptErrorLocalBinding(complete,firstConverted,{$:'sourcePrimitive',operation:convert,arguments:[a]},constructJavaScriptErrorLocalBinding(complete,secondConverted,{$:'sourcePrimitive',operation:convert,arguments:[b]},{$:'sourcePrimitive',operation,arguments:[convertedA,convertedB]},false),false);
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(complete,first,left.source,constructJavaScriptErrorLocalBinding(complete,second,right.source,body,false),false),context:complete};
 }
 if(context.mutableBindings&&['<','<=','>','>='].includes(expression.operator)){
  const firstConverted=next.next,secondConverted=firstConverted+1,comparison=secondConverted+1,complete={...next,next:comparison+1},ordering={$:'sourceBinding',identity:comparison};
  const chosen=expression.operator==='<'?constructJavaScriptSourceOrderingComparison(ordering,-1):expression.operator==='>'?constructJavaScriptSourceOrderingComparison(ordering,1):{$:'sourceIf',condition:constructJavaScriptSourceOrderingComparison(ordering,0),yes:constant({$:'boolean',value:true}),no:constructJavaScriptSourceOrderingComparison(ordering,expression.operator==='<='?-1:1)};
  const compared=constructJavaScriptErrorLocalBinding(complete,comparison,{$:'sourcePrimitive',operation:'compare relational values',arguments:[{$:'sourceBinding',identity:firstConverted},{$:'sourceBinding',identity:secondConverted}]},chosen,false);
  const converted=constructJavaScriptErrorLocalBinding(complete,firstConverted,{$:'sourcePrimitive',operation:'convert number primitive value',arguments:[a]},constructJavaScriptErrorLocalBinding(complete,secondConverted,{$:'sourcePrimitive',operation:'convert number primitive value',arguments:[b]},compared,false),false);
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(complete,first,left.source,constructJavaScriptErrorLocalBinding(complete,second,right.source,converted,false),false),context:complete};
 }
 let body=constant({$:'undefined'});
 if(expression.operator==='instanceof'){const owner=context.capabilities.find(owner=>owner.source===expression.port);if(owner===undefined||!owner.executable)return refuseJavaScriptErrorSource(next,'instanceof needs its actual qualified constructor/operator owner');body={$:'sourceCall',callee:constant({$:'object',identity:owner.identity,callable:true}),receiver:constant({$:'undefined'}),arguments:[a,b]};}
 else if(expression.operator==='+'&&context.mutableBindings)body={$:'sourcePrimitive',operation:'add primitive values',arguments:[a,b]};
 else if(['*','/','-','%','+'].includes(expression.operator)){const operator=expression.operator==='*'?'multiply':expression.operator==='/'?'divide':expression.operator==='-'?'subtract':expression.operator==='%'?'remainder':'add';body={$:'sourceNumberBinary',operator,left:a,right:b};}
 else if(['<','<=','>','>='].includes(expression.operator)){const swapped=expression.operator==='>'||expression.operator==='>=',less={$:'sourceNumberBinary',operator:'lessThan',left:swapped?b:a,right:swapped?a:b};body=expression.operator==='<'||expression.operator==='>'?less:{$:'sourceIf',condition:less,yes:constant({$:'boolean',value:true}),no:{$:'sourceNumberBinary',operator:'equal',left:a,right:b}};}
 else return refuseJavaScriptErrorSource(next,'this source binary operator requires its actual semantic profile');
 return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,first,left.source,constructJavaScriptErrorLocalBinding(next,second,right.source,body,false),false),context:next};
}
/** Control-flow lowering uses actual statements. Early returns bypass later
 * effects; branch fallthrough retains the source continuation and local scope.
 * @param {JavaScriptSourceStatement[]} statements @param {number} first
 * @param {JavaScriptErrorSourceContext} context @param {JavaScriptCallSource} continuation
 * @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceStatements(statements,first,context,continuation,depth){
 if(depth>128||first>65536||statements.length>65536)return refuseJavaScriptErrorSource(context,'source statements exceed their explicit structural budget');
 const statement=statements[first];if(statement===undefined)return {$:'errorSourceExpression',source:continuation,context};
 if(statement.$==='sourceControlJumpStatement')return {$:'errorSourceExpression',source:{$:'sourceControlJump',target:statement.target,continuing:statement.continuing},context};
 if(statement.$==='sourceReturnStatement'||statement.$==='sourceThrowStatement'){const value=lowerJavaScriptErrorExpression(statement.value,context,depth+1);return {...value,source:{$:statement.$==='sourceThrowStatement'?'sourceThrow':'sourceReturn',value:value.source}};}
 const rest=lowerJavaScriptSourceStatements(statements,first+1,context,continuation,depth+1);
 if(statement.$==='sourceForOfStatement'){
  const input=lowerJavaScriptErrorExpression(statement.input,rest.context,depth+1),key=lowerJavaScriptErrorExpression(statement.iteratorKey,input.context,depth+1),body=lowerJavaScriptSourceStatements(statement.body,0,key.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1),initialized=statement.pattern===undefined?body:lowerJavaScriptSourceBindingPattern(statement.pattern,body.context,body.source,statement.mutable),identity=initialized.context.next,next={...initialized.context,next:identity+1};
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,identity,{$:'sourceForOf',slot:statement.target,target:statement.target,binding:statement.binding,mutable:statement.mutable,input:input.source,iteratorKey:key.source,body:initialized.source,realm:next.realm.realm},rest.source,false),context:next};
 }
 if(statement.$==='sourceFinallyStatement'){
  const body=lowerJavaScriptSourceStatements(statement.body,0,rest.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1),finalizer=lowerJavaScriptSourceStatements(statement.finalizer,0,body.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1),identity=finalizer.context.next,next={...finalizer.context,next:identity+1};
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,identity,{$:'sourceFinally',body:body.source,finalizer:finalizer.source},rest.source,false),context:next};
 }
 if(statement.$==='sourceTryStatement'){
  const body=lowerJavaScriptSourceStatements(statement.body,0,rest.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1),handler=lowerJavaScriptSourceStatements(statement.handler,0,body.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1);let initialized=statement.pattern===undefined?handler:lowerJavaScriptSourceBindingPattern(statement.pattern,handler.context,handler.source,true);
  if(statement.pattern===undefined&&(handler.context.sharedBindings??[]).includes(statement.identity)){
   const temporary=initialized.context.next,next={...initialized.context,next:temporary+1},cell={$:'sourceObject',prototype:constructJavaScriptErrorConstant({$:'null'}),realm:next.realm.realm,properties:[{$:'sourceObjectProperty',key:constructJavaScriptSourceFieldKey('value'),value:{$:'sourceBinding',identity:statement.identity}}]};
   initialized={$:'errorSourceExpression',context:next,source:constructJavaScriptErrorLocalBinding(next,temporary,{$:'sourceAssignBinding',identity:statement.identity,value:cell},initialized.source,false)};
  }
  const identity=initialized.context.next,next={...initialized.context,next:identity+1};
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,identity,{$:'sourceTry',identity:statement.identity,body:body.source,handler:initialized.source},rest.source,false),context:next};
 }
 if(statement.$==='sourceObjectBindingStatement'||statement.$==='sourceArrayBindingStatement'){
  const value=lowerJavaScriptErrorExpression(statement.value,rest.context,depth+1),initialized=lowerJavaScriptSourceBindingPattern(statement.pattern,value.context,rest.source,statement.mutable);
  return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(initialized.context,statement.pattern.identity,value.source,initialized.source,false),context:initialized.context};
 }
 if(statement.$==='sourceBlockStatement'||statement.$==='sourceDeclarationGroupStatement')return lowerJavaScriptSourceStatements(statement.statements,0,rest.context,rest.source,depth+1);
 if(statement.$==='sourceControlBlockStatement'){
  const body=lowerJavaScriptSourceStatements(statement.body,0,rest.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1);
  return {$:'errorSourceExpression',source:{$:'sourceControlBlock',target:statement.target,body:body.source,after:rest.source},context:body.context};
 }
 if(statement.$==='sourceWhileStatement'){
  const condition=lowerJavaScriptErrorExpression(statement.condition,rest.context,depth+1),body=lowerJavaScriptSourceStatements(statement.body,0,condition.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1),update=statement.update===undefined?null:lowerJavaScriptErrorExpression(statement.update,body.context,depth+1);
  return {$:'errorSourceExpression',source:{$:'sourceWhile',condition:condition.source,body:body.source,after:rest.source,...(statement.target!==undefined?{target:statement.target}:{}),...(update!==null?{update:update.source}:{}),...(statement.postTest===true?{postTest:true}:{})},context:update===null?body.context:update.context};
 }
 if(statement.$==='sourceSwitchStatement'){
  const value=lowerJavaScriptErrorExpression(statement.value,rest.context,depth+1);let current=value.context;/** @type {JavaScriptCallSourceSwitchClause[]} */const clauses=[];
  for(const clause of statement.clauses){
   if(clause.$==='sourceCaseStatement'){const test=lowerJavaScriptErrorExpression(clause.value,current,depth+1),body=lowerJavaScriptSourceStatements(clause.body,0,test.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1);current=body.context;clauses.push({$:'sourceSwitchCase',value:test.source,body:body.source});}
   else{const body=lowerJavaScriptSourceStatements(clause.body,0,current,constructJavaScriptErrorConstant({$:'undefined'}),depth+1);current=body.context;clauses.push({$:'sourceSwitchDefault',body:body.source});}
  }
  return {$:'errorSourceExpression',source:{$:'sourceSwitch',target:statement.target,value:value.source,clauses,after:rest.source},context:current};
 }
 if(statement.$==='sourceIfStatement'){const condition=lowerJavaScriptErrorExpression(statement.condition,rest.context,depth+1),yes=lowerJavaScriptSourceStatements(statement.yes,0,condition.context,rest.source,depth+1),no=lowerJavaScriptSourceStatements(statement.no,0,yes.context,rest.source,depth+1);return {$:'errorSourceExpression',source:{$:'sourceIf',condition:condition.source,yes:yes.source,no:no.source},context:no.context};}
 const local=statement.$==='sourceConstStatement'||statement.$==='sourceMutableStatement',value=lowerJavaScriptErrorExpression(statement.value,rest.context,depth+1),identity=local?statement.identity:value.context.next,next=local?value.context:{...value.context,next:value.context.next+1},shared=statement.$==='sourceMutableStatement'&&statement.shared===true;
 const stored=shared?{$:'sourceObject',prototype:constructJavaScriptErrorConstant({$:'null'}),realm:next.realm.realm,properties:[{$:'sourceObjectProperty',key:constructJavaScriptSourceFieldKey('value'),value:value.source}]}:value.source;
 return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,identity,stored,rest.source,statement.$==='sourceMutableStatement'&&!shared),context:next};
}
/** Shared ordered ObjectBindingInitialization, preserving each field Get,
 * default and rest copy. The source value has already been evaluated once.
 * @param {JavaScriptErrorParameter} parameter @param {JavaScriptErrorSourceContext} first
 * @param {JavaScriptCallSource} continuation @param {boolean} mutable @param {number} [depth] @param {boolean} [initializeExisting]
 * @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceObjectBinding(parameter,first,continuation,mutable,depth=0,initializeExisting=false){
 if(depth>128)return refuseJavaScriptErrorSource(first,'object binding lowering exceeds its actual nesting budget');
 if(parameter.$!=='errorObjectParameter')return refuseJavaScriptErrorSource(first,'object binding retains its actual object pattern');
 let context=first,source=continuation;
 const keys=[];
 for(const field of parameter.fields){
  if(field.keyExpression===undefined)keys.push({source:constructJavaScriptErrorConstant({$:'text',units:field.key}),identity:-1});
  else{const identity=context.next;context={...context,next:identity+1};keys.push({source:{$:'sourceBinding',identity},identity});}
 }
 if(parameter.rest!==undefined){
  const input={$:'sourceBinding',identity:parameter.identity},excluded=keys.map(key=>key.source),value={$:'sourceObject',prototype:constructJavaScriptErrorConstant(context.realm.objectPrototype),realm:context.realm.realm,properties:[{$:'sourceObjectSpread',input,excluded}]};
  source=constructJavaScriptSourcePatternBinding(context,parameter.rest,value,source,mutable,initializeExisting);
 }
 for(let fieldAt=parameter.fields.length-1;fieldAt>=0;fieldAt--){
  const field=parameter.fields[fieldAt],key=keys[fieldAt];if(field===undefined||key===undefined)continue;
  if(field.pattern!==undefined){const initialized=lowerJavaScriptSourceBindingPattern(field.pattern,context,source,mutable,depth+1,initializeExisting);context=initialized.context;source=initialized.source;}
  const raw=constructJavaScriptErrorPropertyRead({$:'sourceBinding',identity:parameter.identity},key.source,context);
  if(field.hasDefault){
   const value=lowerJavaScriptErrorExpression(field.value,context,0),identity=value.context.next;context={...value.context,next:identity+1};const saved={$:'sourceBinding',identity};
   source=constructJavaScriptErrorLocalBinding(context,identity,raw,constructJavaScriptSourcePatternBinding(context,field.identity,{$:'sourceIf',condition:{$:'sourceStrictEqual',left:saved,right:constructJavaScriptErrorConstant({$:'undefined'})},yes:value.source,no:saved},source,mutable,initializeExisting),false);
  }else source=constructJavaScriptSourcePatternBinding(context,field.identity,raw,source,mutable,initializeExisting);
  if(field.keyExpression!==undefined){const value=lowerJavaScriptErrorExpression(field.keyExpression,context,0);context=value.context;source=constructJavaScriptErrorLocalBinding(context,key.identity,{$:'sourcePropertyKey',input:value.source,realm:context.realm.realm},source,false);}
 }
 {const identity=context.next;context={...context,next:identity+1};source=constructJavaScriptErrorLocalBinding(context,identity,{$:'sourcePrimitive',operation:'require object coercible',arguments:[{$:'sourceBinding',identity:parameter.identity}]},source,false);}
 return {$:'errorSourceExpression',source,context};
}
/** Shared ordered initialization for ordinary functions, methods and arrows.
 * @param {JavaScriptErrorParameter[]} parameters @param {JavaScriptErrorSourceContext} first
 * @param {JavaScriptCallSource} continuation @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceParameters(parameters,first,continuation){
 let context=first,source=continuation;
  for(let at=parameters.length-1;at>=0;at-=1){const parameter=parameters[at];if(parameter===undefined)continue;
   if((parameter.$==='errorRequiredParameter'||parameter.$==='errorDefaultParameter'||parameter.$==='errorRestParameter')&&parameter.shared){
    if(parameter.$==='errorRestParameter'&&parameter.pattern!==undefined)return refuseJavaScriptErrorSource(context,'shared destructuring parameter cells require their own binding initialization');
    const temporary=context.next;context={...context,next:temporary+1};
    const cell={$:'sourceObject',prototype:constructJavaScriptErrorConstant({$:'null'}),realm:context.realm.realm,properties:[{$:'sourceObjectProperty',key:constructJavaScriptSourceFieldKey('value'),value:{$:'sourceBinding',identity:parameter.identity}}]};
    source=constructJavaScriptErrorLocalBinding(context,temporary,{$:'sourceAssignBinding',identity:parameter.identity,value:cell},source,false);
   }
   if(parameter.$==='errorRestParameter'){
    if(at!==parameters.length-1||context.realm.arrayPrototype===undefined)return refuseJavaScriptErrorSource(context,'rest parameters retain one actual final formal and its realm Array prototype');
    if(parameter.pattern!==undefined){const initialized=lowerJavaScriptSourceBindingPattern(parameter.pattern,context,source,true);context=initialized.context;source=initialized.source;}
    source={$:'sourceParameterRest',identity:parameter.identity,from:at,prototype:constructJavaScriptErrorConstant(context.realm.arrayPrototype),realm:context.realm.realm,body:source};
   }else if(parameter.$==='errorObjectParameter'||parameter.$==='errorArrayParameter'){
    const initialized=lowerJavaScriptSourceBindingPattern(parameter,context,source,true);context=initialized.context;source=initialized.source;
    if(parameter.hasDefault){const value=lowerJavaScriptErrorExpression(parameter.value,context,0);context=value.context;source={$:'sourceParameterDefault',identity:parameter.identity,value:value.source,body:source};}
   }else if(parameter.$==='errorDefaultParameter'){const value=lowerJavaScriptErrorExpression(parameter.value,context,0);context=value.context;source={$:'sourceParameterDefault',identity:parameter.identity,value:value.source,body:source};}
  }
 return {$:'errorSourceExpression',source,context};
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
 return compileJavaScriptErrorModulePlanWithCapabilities(plan,registry,bindings,[]);
}
/** Each capability is an actual qualified external value/operation. Source
 * aliases can share one genuine identity without inventing a function body;
 * nonexecuting namespace/source references never become host callbacks.
 * @param {JavaScriptErrorModulePlan} plan @param {JavaScriptErrorModuleRegistry} registry
 * @param {JavaScriptErrorModuleBinding[]} bindings @param {JavaScriptSourceCapabilityOwner[]} capabilities
 * @returns {JavaScriptErrorModuleCompilation} */
export function compileJavaScriptErrorModulePlanWithCapabilities(plan,registry,bindings,capabilities){
 const empty={$:'callProgram',functions:[],realms:registry.realms,exoticKey:registry.exoticKey};
 const shared=plan.sharedBindings??[];if(shared.length>4096||shared.some((identity,index)=>!Number.isSafeInteger(identity)||Object.is(identity,-0)||identity<0||identity>=plan.nextBinding||index>0&&identity<=(shared[index-1]??-1)))return {$:'errorModuleCompilation',program:empty,error:'shared lexical cells retain ordered unique actual bounded source binding identities'};
 if(capabilities.length>4096)return {$:'errorModuleCompilation',program:empty,error:'qualified source capabilities exceed their actual directory budget'};
 for(let at=0;at<capabilities.length;at++){const capability=capabilities[at];if(capability===undefined||capability.source===''||capability.owner===''||capability.operation===''||capability.identity<=0n||capability.identity>18446744073709551615n||!registry.realms.includes(capability.realm)||capability.executable&&!capability.callable||capabilities.findIndex(row=>row.source===capability.source)!==at)return {$:'errorModuleCompilation',program:empty,error:'source capabilities require distinct actual references, identities, realms and operation owners'};const previous=capabilities.findIndex(row=>row.identity===capability.identity);if(previous!==at){const owner=capabilities[previous];if(owner===undefined||owner.realm!==capability.realm||owner.owner!==capability.owner||owner.operation!==capability.operation||owner.callable!==capability.callable||owner.executable!==capability.executable)return {$:'errorModuleCompilation',program:empty,error:'one actual capability identity cannot acquire conflicting operation or namespace roles'};}}
 if(bindings.length>4096)return {$:'errorModuleCompilation',program:empty,error:'actual module captures exceed the source frame budget'};
 for(let at=0;at<bindings.length;at++){const binding=bindings[at];if(binding===undefined||!Number.isSafeInteger(binding.identity)||binding.identity<0||binding.identity>=plan.nextBinding||bindings.findIndex(row=>row.identity===binding.identity)!==at||readJavaScriptSemanticValueError(binding.value)!=='')return {$:'errorModuleCompilation',program:empty,error:'module captures require distinct actual lexical owners and semantic values'};}
 if(plan.functions.length>4096||registry.owners.length!==plan.functions.length||registry.intrinsics.length>4096||plan.nextBinding<0||!Number.isSafeInteger(plan.nextBinding))return {$:'errorModuleCompilation',program:empty,error:'Error module plans need complete bounded source and callable registries'};
 let functions=[];const identities=[];
 for(const definition of plan.functions){
  const matches=registry.owners.filter(owner=>owner.source===definition.identity);if(matches.length!==1)return {$:'errorModuleCompilation',program:empty,error:'every actual source definition needs exactly one supplied callable owner'};const owner=matches[0];if(owner===undefined)return {$:'errorModuleCompilation',program:empty,error:'source callable owners must be dense'};
  if(identities.includes(owner.identity)||!registry.realms.includes(owner.realm))return {$:'errorModuleCompilation',program:empty,error:'source callable owners need distinct identities and actual realms'};identities.push(owner.identity);
  const operations=registry.operations.filter(row=>row.realm===owner.realm);if(operations.length!==1)return {$:'errorModuleCompilation',program:empty,error:'Error source operations require their actual complete source realm'};const realm=operations[0];if(realm===undefined)return {$:'errorModuleCompilation',program:empty,error:'Error source realm rows must be dense'};
  let context={$:'errorSourceContext',registry,realm,capabilities,sharedBindings:plan.sharedBindings??[],next:plan.nextBinding,visits:0,error:'',globalBindings:bindings.map(binding=>binding.identity),mutableBindings:definition.$==='errorOrdinaryConstructorPlan'||definition.$==='errorMethodFunctionPlan'||definition.$==='errorMutableStatementFunctionPlan'},source=constructJavaScriptErrorConstant({$:'undefined'});
  if(definition.$==='errorConstructorPlan'){
   if(owner.$!=='errorConstructorOwner'||!registry.realms.includes(owner.baseRealm))return {$:'errorModuleCompilation',program:empty,error:'an Error subclass requires its actual base constructor and realm authority'};
   const message=lowerJavaScriptErrorExpression(definition.message,context,0);context=message.context;
   const messageIdentity=context.next;context={...context,next:context.next+1};const raw={$:'sourceBinding',identity:messageIdentity};
   const absent={$:'sourceStrictEqual',left:raw,right:constructJavaScriptErrorConstant({$:'undefined'})};
   const construct={$:'sourceConstructErrorValue',constructor:constructJavaScriptErrorConstant(owner.baseConstructor),newTarget:constructJavaScriptErrorConstant({$:'object',identity:owner.identity,callable:true}),prototype:constructJavaScriptErrorConstant(owner.prototype),message:{$:'sourceIf',condition:absent,yes:constructJavaScriptErrorConstant({$:'text',units:[]}),no:{$:'sourcePrimitive',operation:'convert primitive text',arguments:[raw]}},present:{$:'sourceIf',condition:absent,yes:constructJavaScriptErrorConstant({$:'boolean',value:false}),no:constructJavaScriptErrorConstant({$:'boolean',value:true})},realm:owner.baseRealm,errorClass:definition.baseClass,bindReceiver:true};
   const writes=[];for(const field of definition.fields){const value=lowerJavaScriptErrorExpression(field.value,context,0);context=value.context;writes.push({$:'sourceCall',callee:constructJavaScriptErrorConstant(realm.assign),receiver:{$:'sourceDerivedReceiver'},arguments:[constructJavaScriptErrorConstant({$:'text',units:field.name}),value.source]});}
   source={$:'sourceDerivedReceiver'};for(let at=writes.length-1;at>=0;at-=1){const identity=context.next;context={...context,next:identity+1};source={$:'sourceLet',identity,value:writes[at]??constructJavaScriptErrorConstant({$:'undefined'}),body:source};}
   const superIdentity=context.next;context={...context,next:superIdentity+1};source={$:'sourceLet',identity:messageIdentity,value:message.source,body:{$:'sourceLet',identity:superIdentity,value:construct,body:source}};
  }else{
   if(definition.$==='errorOrdinaryConstructorPlan'?owner.$!=='ordinaryConstructorOwner':owner.$!=='errorFunctionOwner')return {$:'errorModuleCompilation',program:empty,error:'ordinary source functions require their actual function or class constructor ownership'};
   if(definition.$==='errorStatementFunctionPlan'||definition.$==='errorOrdinaryConstructorPlan'||definition.$==='errorMethodFunctionPlan'||definition.$==='errorMutableStatementFunctionPlan'){const body=lowerJavaScriptSourceStatements(definition.statements,0,context,constructJavaScriptErrorConstant({$:'undefined'}),0);context=body.context;source=body.source;}
   else{const body=lowerJavaScriptErrorExpression(definition.body,context,0);context=body.context;source=definition.abrupt?{$:'sourceThrow',value:body.source}:body.source;}
  }
  const initialized=lowerJavaScriptSourceParameters(definition.parameters,context,source);context=initialized.context;source=initialized.source;
  if(context.error!=='')return {$:'errorModuleCompilation',program:empty,error:context.error};
  const code=context.mutableBindings?compileJavaScriptMutableCallSourceWithCaptures(source,definition.parameters.map(parameter=>parameter.identity),bindings.map(binding=>binding.identity)):compileJavaScriptCallSourceWithCaptures(source,definition.parameters.map(parameter=>parameter.identity),bindings.map(binding=>binding.identity));if(code.error!=='')return {$:'errorModuleCompilation',program:empty,error:code.error};
  functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:definition.$==='errorConstructorPlan'?'derived Error':definition.$==='errorOrdinaryConstructorPlan'?'ordinary class':'strict',lexicalReceiver:{$:'undefined'},parameters:definition.parameters.length,registers:code.registers,captures:bindings.map(binding=>binding.value),instructions:code.instructions});
 }
 /** @type {JavaScriptCallTemplate[]} */ const templates=[];
 if((plan.templates??[]).length>4096)return {$:'errorModuleCompilation',program:empty,error:'source arrow template directory exceeds its budget'};
 for(const definition of plan.templates??[]){
  const owner=registry.owners.find(row=>row.source===definition.owner),sourceOwner=plan.functions.find(row=>row.identity===definition.owner);
  if(owner===undefined||sourceOwner===undefined||sourceOwner.$==='errorConstructorPlan'||!Number.isSafeInteger(definition.ordinal)||definition.ordinal<0||definition.ordinal>=4096||Object.is(definition.ordinal,-0)||templates.some(row=>row.owner===owner.identity&&row.ordinal===definition.ordinal))return {$:'errorModuleCompilation',program:empty,error:'arrow templates retain unique actual source owners and lexical sites; derived-this cells are separate'};
  const realm=registry.operations.find(row=>row.realm===owner.realm);
  if(realm===undefined||realm.functionPrototype===undefined)return {$:'errorModuleCompilation',program:empty,error:'arrow source needs its actual realm Function prototype'};
  const firstDefault=definition.parameters.findIndex(parameter=>parameter.$==='errorDefaultParameter'||parameter.$==='errorRestParameter'||(parameter.$==='errorObjectParameter'||parameter.$==='errorArrayParameter')&&parameter.hasDefault),length=firstDefault<0?definition.parameters.length:firstDefault;
  if(definition.length!==length)return {$:'errorModuleCompilation',program:empty,error:'arrow Function length retains its actual parameter initializer boundary'};
  const identities=bindings.map(binding=>binding.identity).concat(definition.captures),parameters=definition.parameters.map(parameter=>parameter.identity);
  if(identities.length>4096||identities.some((identity,index)=>identities.indexOf(identity)!==index||parameters.includes(identity)))return {$:'errorModuleCompilation',program:empty,error:'arrow parameters and captures retain disjoint actual lexical bindings'};
  const context={$:'errorSourceContext',registry,realm,capabilities,sharedBindings:plan.sharedBindings??[],globalBindings:bindings.map(binding=>binding.identity),next:plan.nextBinding,visits:0,error:'',mutableBindings:true};
  const lowered=lowerJavaScriptSourceStatements(definition.statements,0,context,constructJavaScriptErrorConstant({$:'undefined'}),0);
  if(lowered.context.error!=='')return {$:'errorModuleCompilation',program:empty,error:lowered.context.error};
  const initialized=lowerJavaScriptSourceParameters(definition.parameters,lowered.context,lowered.source);if(initialized.context.error!=='')return {$:'errorModuleCompilation',program:empty,error:initialized.context.error};
  const code=compileJavaScriptMutableCallSourceWithCaptures(initialized.source,parameters,identities);if(code.error!=='')return {$:'errorModuleCompilation',program:empty,error:code.error};
  if(definition.receiverMode!==undefined&&definition.receiverMode!=='strict'&&definition.receiverMode!=='lexical')return {$:'errorModuleCompilation',program:empty,error:'runtime source functions require explicit strict method or lexical arrow receiver mode'};
  templates.push({$:'callTemplate',owner:owner.identity,ordinal:definition.ordinal,realm:owner.realm,parameters:parameters.length,registers:code.registers,captureCount:identities.length,instructions:code.instructions,prototype:realm.functionPrototype,name:{$:'text',units:Array.from({length:definition.name.length},(_,index)=>definition.name.charCodeAt(index))},length:definition.length,...(definition.receiverMode!==undefined?{receiverMode:definition.receiverMode}:{})});
 }
 const orderedTemplates=templates.toSorted((left,right)=>left.owner<right.owner?-1:left.owner>right.owner?1:left.ordinal-right.ordinal);
 for(const intrinsic of registry.intrinsics){if(identities.includes(intrinsic.identity)||!registry.realms.includes(intrinsic.realm))return {$:'errorModuleCompilation',program:empty,error:'external Error callbacks require distinct actual callable/realm identities'};identities.push(intrinsic.identity);functions.push({$:'callFunction',identity:intrinsic.identity,realm:intrinsic.realm,receiverMode:'Error intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'invokeErrorIntrinsic',destination:1},{$:'return',source:1}]});}
 for(let at=0;at<capabilities.length;at++){const capability=capabilities[at];if(capability===undefined||!capability.executable||capabilities.findIndex(row=>row.identity===capability.identity)!==at)continue;if(identities.includes(capability.identity))return {$:'errorModuleCompilation',program:empty,error:'a host capability cannot replace an actual source or Error callable body'};identities.push(capability.identity);functions.push({$:'callFunction',identity:capability.identity,realm:capability.realm,receiverMode:'qualified host capability',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'invokeHostCapability',destination:1},{$:'return',source:1}]});}
 functions=functions.toSorted((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0);
 return {$:'errorModuleCompilation',program:{$:'callProgram',functions,realms:registry.realms,exoticKey:registry.exoticKey,...(plan.templates!==undefined?{templates:orderedTemplates}:{})},error:''};
}

/** @typedef {{$:'sourceReferenceCapture',identity:number,value:JavaScriptCallSource}} JavaScriptSourceReferenceCapture */
/** @typedef {{$:'sourceReference',kind:string,identity:number,read:JavaScriptCallSource,receiver:JavaScriptCallSource,key:JavaScriptCallSource,captures:JavaScriptSourceReferenceCapture[],context:JavaScriptErrorSourceContext}} JavaScriptSourceReference */
/** Capture reference evaluation independently of later GetValue/RHS/PutValue.
 * @param {JavaScriptErrorExpression} target @param {JavaScriptErrorSourceContext} context
 * @param {number} depth @returns {JavaScriptSourceReference} */
export function lowerJavaScriptSourceReference(target,context,depth){
 const constant=constructJavaScriptErrorConstant;let current=context,read=constant({$:'undefined'}),receiver=constant({$:'undefined'}),key=constant({$:'undefined'}),kind='',identity=0;
 const captures=[];
 if(target.$==='errorBinding'){kind='binding';identity=target.identity;read={$:'sourceBinding',identity:target.identity};}
 else if(target.$==='errorModuleCell'){kind='property';receiver={$:'sourceBinding',identity:target.identity};key=constructJavaScriptSourceFieldKey('value');read={$:'sourceGet',object:receiver,key};}
 else if(target.$==='errorGet'||target.$==='errorComputedGet'){
  kind='property';const object=lowerJavaScriptErrorExpression(target.object,current,depth+1),property=lowerJavaScriptErrorExpression(target.key,object.context,depth+1),receiverID=property.context.next,keyID=receiverID+1,convertedID=keyID+1;current={...property.context,next:convertedID+1};receiver={$:'sourceBinding',identity:receiverID};const rawKey={$:'sourceBinding',identity:keyID};key={$:'sourceBinding',identity:convertedID};
  const failure={$:'sourceCall',callee:constant(context.realm.get),receiver,arguments:[rawKey]},conversion={$:'sourcePropertyKey',input:rawKey,realm:context.realm.realm};
  const checked={$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constant({$:'null'})},yes:failure,no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:receiver,right:constant({$:'undefined'})},yes:failure,no:conversion}};
  captures.push({$:'sourceReferenceCapture',identity:receiverID,value:object.source},{$:'sourceReferenceCapture',identity:keyID,value:property.source},{$:'sourceReferenceCapture',identity:convertedID,value:checked});read=constructJavaScriptErrorPropertyRead(receiver,key,current);
 }else current={...current,error:'source read/modify/write retains an actual lexical, module-cell or property reference'};
 return {$:'sourceReference',kind,identity,read,receiver,key,captures,context:current};
}
/** @param {JavaScriptSourceReference} reference @param {JavaScriptCallSource} value @returns {JavaScriptCallSource} */
export function constructJavaScriptSourceReferenceWrite(reference,value){
 return reference.kind==='binding'?{$:'sourceAssignBinding',identity:reference.identity,value}:{$:'sourceIf',condition:{$:'sourcePrimitive',operation:'test object value',arguments:[reference.receiver]},yes:{$:'sourceSnapshotSet',object:reference.receiver,key:reference.key,value},no:{$:'sourceCall',callee:constructJavaScriptErrorConstant(reference.context.realm.assign),receiver:reference.receiver,arguments:[reference.key,value]}};
}
/** @param {JavaScriptSourceReference} reference @param {JavaScriptErrorSourceContext} context
 * @param {JavaScriptCallSource} body @returns {JavaScriptCallSource} */
export function constructJavaScriptSourceReferenceCaptures(reference,context,body){
 let source=body;for(let at=reference.captures.length-1;at>=0;at--){const capture=reference.captures[at];if(capture!==undefined)source=constructJavaScriptErrorLocalBinding(context,capture.identity,capture.value,source,false);}return source;
}
/** Retain one actual Reference, one GetValue and one ToNumeric before PutValue.
 * @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context
 * @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceUpdate(expression,context,depth){
 if(expression.$!=='errorUpdate'||!context.mutableBindings||!['++','--'].includes(expression.operator)||typeof expression.prefix!=='boolean')return refuseJavaScriptErrorSource(context,'source updates require their actual mutable reference and prefix/postfix operator');
 const reference=lowerJavaScriptSourceReference(expression.target,context,depth+1);if(reference.context.error!=='')return refuseJavaScriptErrorSource(reference.context,reference.context.error);
 const prior=reference.context.next,updated=prior+1,effect=updated+1,current={...reference.context,next:effect+1},old={$:'sourceBinding',identity:prior},value={$:'sourceBinding',identity:updated};
 const write=constructJavaScriptSourceReferenceWrite(reference,value),body=constructJavaScriptErrorLocalBinding(current,prior,{$:'sourcePrimitive',operation:'coerce primitive numeric',arguments:[reference.read]},constructJavaScriptErrorLocalBinding(current,updated,{$:'sourcePrimitive',operation:expression.operator==='++'?'increment numeric value':'decrement numeric value',arguments:[old]},constructJavaScriptErrorLocalBinding(current,effect,write,expression.prefix?value:old,false),false),false);
 return {$:'errorSourceExpression',source:constructJavaScriptSourceReferenceCaptures(reference,current,body),context:current};
}
/** The source RHS runs after GetValue and before either operand conversion.
 * @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context
 * @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceCompoundAssignment(expression,context,depth){
 if(expression.$!=='errorCompoundAssignment'||!context.mutableBindings||!['+=','-=','*=','/=','%=','**=','&&=','||=','??='].includes(expression.operator))return refuseJavaScriptErrorSource(context,'compound assignment requires its actual source read/modify/write reference');
 const reference=lowerJavaScriptSourceReference(expression.target,context,depth+1);if(reference.context.error!=='')return refuseJavaScriptErrorSource(reference.context,reference.context.error);
 if(['&&=','||=','??='].includes(expression.operator)){
  const prior=reference.context.next,rhs=lowerJavaScriptErrorExpression(expression.value,{...reference.context,next:prior+1},depth+1),updated=rhs.context.next,effect=updated+1,current={...rhs.context,next:effect+1},old={$:'sourceBinding',identity:prior},value={$:'sourceBinding',identity:updated},constant=constructJavaScriptErrorConstant;
  const selected=constructJavaScriptErrorLocalBinding(current,updated,rhs.source,constructJavaScriptErrorLocalBinding(current,effect,constructJavaScriptSourceReferenceWrite(reference,value),value,false),false);
  const nullish={$:'sourceIf',condition:{$:'sourceStrictEqual',left:old,right:constant({$:'null'})},yes:constant({$:'boolean',value:true}),no:{$:'sourceStrictEqual',left:old,right:constant({$:'undefined'})}};
  const choice=expression.operator==='&&='?{$:'sourceIf',condition:old,yes:selected,no:old}:expression.operator==='||='?{$:'sourceIf',condition:old,yes:old,no:selected}:{$:'sourceIf',condition:nullish,yes:selected,no:old};
  const body=constructJavaScriptErrorLocalBinding(current,prior,reference.read,choice,false);return {$:'errorSourceExpression',source:constructJavaScriptSourceReferenceCaptures(reference,current,body),context:current};
 }
 const prior=reference.context.next,binary=lowerJavaScriptSourceBinary({$:'errorBinary',operator:expression.operator.slice(0,-1),left:{$:'errorBinding',identity:prior},right:expression.value,port:''},{...reference.context,next:prior+1},depth+1),updated=binary.context.next,effect=updated+1,current={...binary.context,next:effect+1},value={$:'sourceBinding',identity:updated};
 const write=constructJavaScriptSourceReferenceWrite(reference,value),body=constructJavaScriptErrorLocalBinding(current,prior,reference.read,constructJavaScriptErrorLocalBinding(current,updated,binary.source,constructJavaScriptErrorLocalBinding(current,effect,write,value,false),false),false);
 return {$:'errorSourceExpression',source:constructJavaScriptSourceReferenceCaptures(reference,current,body),context:current};
}

/** @param {JavaScriptCallSource} ordering @param {number} code @returns {JavaScriptCallSource} */
export function constructJavaScriptSourceOrderingComparison(ordering,code){return {$:'sourceNumberBinary',operator:'equal',left:ordering,right:constructJavaScriptErrorConstant({$:'number',value:code})};}


/** @param {JavaScriptErrorSourceContext} context @param {number} identity
 * @param {JavaScriptCallSource} value @param {JavaScriptCallSource} body
 * @param {boolean} mutable @param {boolean} initializeExisting @returns {JavaScriptCallSource} */
export function constructJavaScriptSourcePatternBinding(context,identity,value,body,mutable,initializeExisting){
 const shared=(context.sharedBindings??[]).includes(identity),stored=shared?{$:'sourceObject',prototype:constructJavaScriptErrorConstant({$:'null'}),realm:context.realm.realm,properties:[{$:'sourceObjectProperty',key:constructJavaScriptSourceFieldKey('value'),value}]}:value;
 return initializeExisting?{$:'sourceInitializeBinding',identity,value:stored,body}:constructJavaScriptErrorLocalBinding(context,identity,stored,body,mutable&&!shared);
}
/** @param {JavaScriptErrorParameter} pattern @param {JavaScriptErrorSourceContext} context
 * @param {JavaScriptCallSource} continuation @param {boolean} mutable @param {number} [depth]
 * @param {boolean} [initializeExisting] @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceBindingPattern(pattern,context,continuation,mutable,depth=0,initializeExisting=false){
 return pattern.$==='errorArrayParameter'?lowerJavaScriptSourceArrayBinding(pattern,context,continuation,mutable,depth,initializeExisting):lowerJavaScriptSourceObjectBinding(pattern,context,continuation,mutable,depth,initializeExisting);
}
/** @typedef {{$:'bindingDeclarations',declarations:JavaScriptCallBindingDeclaration[],error:string}} JavaScriptPatternBindingDeclarations */
/** @param {JavaScriptErrorParameter} pattern @param {boolean} mutable @param {number} depth @param {number[]} [shared] @returns {JavaScriptPatternBindingDeclarations} */
export function collectJavaScriptPatternBindingDeclarations(pattern,mutable,depth,shared=[]){
 if(depth>128||pattern.$!=='errorObjectParameter'&&pattern.$!=='errorArrayParameter')return {$:'bindingDeclarations',declarations:[],error:'pattern output declarations retain their actual bounded recursive pattern'};
 const fields=pattern.$==='errorObjectParameter'?pattern.fields:pattern.elements.filter(field=>field.$==='errorArrayBindingElement'),declarations=[];
 for(const field of fields){
  if(field.$==='errorArrayElision')continue;declarations.push({$:'sourceBindingDeclaration',identity:field.identity,mutable:mutable&&!shared.includes(field.identity)});
  if(field.pattern!==undefined){const child=collectJavaScriptPatternBindingDeclarations(field.pattern,mutable,depth+1,shared);if(child.error!=='')return child;declarations.push(...child.declarations);}
  if(declarations.length>4096)return {$:'bindingDeclarations',declarations:[],error:'binding outputs exceed their actual structural budget'};
 }
 if(pattern.$==='errorObjectParameter'&&pattern.rest!==undefined)declarations.push({$:'sourceBindingDeclaration',identity:pattern.rest,mutable:mutable&&!shared.includes(pattern.rest)});
 if(pattern.$==='errorArrayParameter'&&pattern.rest!==undefined){declarations.push({$:'sourceBindingDeclaration',identity:pattern.rest.identity,mutable:mutable&&!shared.includes(pattern.rest.identity)});if(pattern.rest.pattern!==undefined){const child=collectJavaScriptPatternBindingDeclarations(pattern.rest.pattern,mutable,depth+1,shared);if(child.error!=='')return child;declarations.push(...child.declarations);}}
 return declarations.length>4096?{$:'bindingDeclarations',declarations:[],error:'binding outputs exceed their actual structural budget'}:{$:'bindingDeclarations',declarations,error:''};
}
/** @param {JavaScriptErrorParameter} pattern @param {JavaScriptErrorSourceContext} first
 * @param {JavaScriptCallSource} continuation @param {boolean} mutable @param {number} depth
 * @param {boolean} initializeExisting @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceArrayBinding(pattern,first,continuation,mutable,depth,initializeExisting){
 if(depth>128||pattern.$!=='errorArrayParameter'||first.realm.arrayPrototype===undefined)return refuseJavaScriptErrorSource(first,'Array binding initialization retains its actual recursive pattern and realm Array prototype');
 const key=lowerJavaScriptErrorExpression(pattern.iteratorKey,first,0);let context=key.context,source=constructJavaScriptErrorConstant({$:'undefined'});
 if(pattern.rest!==undefined){const rest=pattern.rest;if(rest.pattern!==undefined){const nested=lowerJavaScriptSourceBindingPattern(rest.pattern,context,source,mutable,depth+1,true);context=nested.context;source=nested.source;}source=constructJavaScriptSourcePatternBinding(context,rest.identity,{$:'sourceIteratorRemainder',slot:pattern.identity,prototype:constructJavaScriptErrorConstant(context.realm.arrayPrototype),realm:context.realm.realm},source,mutable,true);}
 for(let at=pattern.elements.length-1;at>=0;at--){const element=pattern.elements[at];if(element===undefined)continue;
  if(element.$==='errorArrayElision'){const identity=context.next;context={...context,next:identity+1};source=constructJavaScriptErrorLocalBinding(context,identity,{$:'sourceIteratorStep',slot:pattern.identity,realm:context.realm.realm,readValue:false},source,false);continue;}
  if(element.pattern!==undefined){const nested=lowerJavaScriptSourceBindingPattern(element.pattern,context,source,mutable,depth+1,true);context=nested.context;source=nested.source;}
  const identity=context.next;context={...context,next:identity+1};const saved={$:'sourceBinding',identity};let value=saved;
  if(element.hasDefault){const lowered=lowerJavaScriptErrorExpression(element.value,context,0);context=lowered.context;value={$:'sourceIf',condition:{$:'sourceStrictEqual',left:saved,right:constructJavaScriptErrorConstant({$:'undefined'})},yes:lowered.source,no:saved};}
  source=constructJavaScriptErrorLocalBinding(context,identity,{$:'sourceIteratorStep',slot:pattern.identity,realm:context.realm.realm,readValue:true},constructJavaScriptSourcePatternBinding(context,element.identity,value,source,mutable,true),false);
 }
 const initializer={$:'sourceIteratorBinding',slot:pattern.identity,input:{$:'sourceBinding',identity:pattern.identity},iteratorKey:key.source,body:source,realm:context.realm.realm};
 if(initializeExisting){const identity=context.next;context={...context,next:identity+1};return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(context,identity,initializer,continuation,false),context};}
 const outputs=collectJavaScriptPatternBindingDeclarations(pattern,mutable,depth,context.sharedBindings??[]);if(outputs.error!=='')return refuseJavaScriptErrorSource(context,outputs.error);
 return {$:'errorSourceExpression',source:{$:'sourceBindingScope',declarations:outputs.declarations,initializer,body:continuation},context};
}
