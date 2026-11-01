// Source-class plans contain actual lexical binding coordinates. Runtime
// identities/prototypes come from the supplied realm registry, never numbers
// or class names invented by this compiler. No class body executes here.
import {compileJavaScriptCallSourceWithCaptures} from './javascript-call-source.js';
import {readJavaScriptSemanticValueError} from './javascript-semantic-values.js';
/** @typedef {{$:'errorConstant',value:JavaScriptSemanticValue} |
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
 * {$:'errorBinary',operator:string,left:JavaScriptErrorExpression,right:JavaScriptErrorExpression,port:string} |
 * {$:'errorArray',factory:string,items:JavaScriptErrorExpression[]} |
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
/** @typedef {{$:'errorObjectProperty',key:number[],value:JavaScriptErrorExpression}} JavaScriptErrorObjectProperty */
/** @typedef {{$:'errorArrayElement',index:number,value:JavaScriptErrorExpression}} JavaScriptErrorArrayElement */
/** @typedef {{$:'errorTemplateText',units:number[]} | {$:'errorTemplateValue',value:JavaScriptErrorExpression}} JavaScriptErrorTemplatePart */
/** @typedef {{$:'errorParameterField',identity:number,key:number[],hasDefault:boolean,value:JavaScriptErrorExpression}} JavaScriptErrorParameterField */
/** @typedef {{$:'errorRequiredParameter',identity:number} | {$:'errorDefaultParameter',identity:number,value:JavaScriptErrorExpression} | {$:'errorObjectParameter',identity:number,hasDefault:boolean,value:JavaScriptErrorExpression,fields:JavaScriptErrorParameterField[]}} JavaScriptErrorParameter */
/** @typedef {{$:'sourceConstStatement',identity:number,value:JavaScriptErrorExpression} | {$:'sourceMutableStatement',identity:number,value:JavaScriptErrorExpression} | {$:'sourceExpressionStatement',value:JavaScriptErrorExpression} | {$:'sourceReturnStatement',value:JavaScriptErrorExpression} | {$:'sourceThrowStatement',value:JavaScriptErrorExpression} | {$:'sourceIfStatement',condition:JavaScriptErrorExpression,yes:JavaScriptSourceStatement[],no:JavaScriptSourceStatement[]} | {$:'sourceBlockStatement',statements:JavaScriptSourceStatement[]} | {$:'sourceWhileStatement',condition:JavaScriptErrorExpression,body:JavaScriptSourceStatement[]}} JavaScriptSourceStatement */
/** @typedef {{$:'errorFieldAssignment',name:number[],value:JavaScriptErrorExpression}} JavaScriptErrorFieldAssignment */
/** @typedef {{$:'errorConstructorPlan',identity:number,name:string,baseClass:string,parameters:JavaScriptErrorParameter[],message:JavaScriptErrorExpression,fields:JavaScriptErrorFieldAssignment[]} |
 * {$:'errorFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],body:JavaScriptErrorExpression,abrupt:boolean} |
 * {$:'errorOrdinaryConstructorPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorMethodFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorMutableStatementFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]} |
 * {$:'errorStatementFunctionPlan',identity:number,name:string,parameters:JavaScriptErrorParameter[],statements:JavaScriptSourceStatement[]}} JavaScriptErrorFunctionPlan */
/** @typedef {{$:'errorModulePlan',functions:JavaScriptErrorFunctionPlan[],exports:number[],nextBinding:number}} JavaScriptErrorModulePlan */
/** @typedef {{$:'errorConstructorOwner',source:number,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue,baseConstructor:JavaScriptSemanticValue,baseRealm:bigint} |
 * {$:'ordinaryConstructorOwner',source:number,identity:bigint,realm:bigint,prototype:JavaScriptSemanticValue} |
 * {$:'errorFunctionOwner',source:number,identity:bigint,realm:bigint}} JavaScriptErrorFunctionOwner */
/** @typedef {{$:'errorRealmOperations',realm:bigint,objectPrototype:JavaScriptSemanticValue,arrayPrototype?:JavaScriptSemanticValue,functionPrototype?:JavaScriptSemanticValue,get:JavaScriptSemanticValue,assign:JavaScriptSemanticValue}} JavaScriptErrorRealmOperations */
/** @typedef {{$:'errorIntrinsicOwner',identity:bigint,realm:bigint}} JavaScriptErrorIntrinsicOwner */
/** @typedef {{$:'errorModuleRegistry',owners:JavaScriptErrorFunctionOwner[],operations:JavaScriptErrorRealmOperations[],intrinsics:JavaScriptErrorIntrinsicOwner[],realms:bigint[],exoticKey:JavaScriptSemanticValue}} JavaScriptErrorModuleRegistry */
/** @typedef {{$:'sourceCapabilityOwner',source:string,identity:bigint,realm:bigint,operation:string,owner:string,callable:boolean,executable:boolean}} JavaScriptSourceCapabilityOwner */
/** @typedef {{$:'errorSourceContext',registry:JavaScriptErrorModuleRegistry,realm:JavaScriptErrorRealmOperations,capabilities:JavaScriptSourceCapabilityOwner[],next:number,visits:number,error:string,mutableBindings:boolean}} JavaScriptErrorSourceContext */
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
  case 'errorBinding':case 'errorModuleBinding':return {$:'errorSourceExpression',source:{$:'sourceBinding',identity:expression.identity},context:next};
  case 'errorThis':return {$:'errorSourceExpression',source:{$:'sourceDerivedReceiver'},context:next};
  case 'errorReceiver':return {$:'errorSourceExpression',source:{$:'sourceReceiver'},context:next};
  case 'errorBindingSet':{if(!next.mutableBindings)return refuseJavaScriptErrorSource(next,'mutable lexical assignment requires its explicit source profile');const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1);return {...value,source:{$:'sourceAssignBinding',identity:expression.identity,value:value.source}};}
  case 'errorOrdinaryConstruction':{
   if(!next.mutableBindings)return refuseJavaScriptErrorSource(next,'ordinary class construction requires its explicit source profile');
   const owner=next.registry.owners.find(row=>row.source===expression.identity);if(owner===undefined||owner.$!=='ordinaryConstructorOwner')return refuseJavaScriptErrorSource(next,'ordinary construction requires its actual supplied class and prototype owner');
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
  case 'errorUnary':{const value=lowerJavaScriptErrorExpression(expression.value,next,depth+1);if(expression.operator==='!')return {$:'errorSourceExpression',source:{$:'sourceIf',condition:value.source,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})},context:value.context};if(expression.operator==='typeof'||expression.operator==='-')return {$:'errorSourceExpression',source:{$:'sourcePrimitive',operation:expression.operator==='typeof'?'read semantic type':'negate number',arguments:[value.source]},context:value.context};return refuseJavaScriptErrorSource(value.context,'this unary operation requires its actual source semantic profile');}
  case 'errorArray':{const owner=next.capabilities.find(owner=>owner.source===expression.factory);if(owner===undefined||!owner.executable)return refuseJavaScriptErrorSource(next,'array literals require their actual protected realm constructor capability');let current=next;const arguments_=[];for(const item of expression.items){const value=lowerJavaScriptErrorExpression(item,current,depth+1);current=value.context;arguments_.push(value.source);}return {$:'errorSourceExpression',source:{$:'sourceCall',callee:constant({$:'object',identity:owner.identity,callable:owner.callable}),receiver:constant({$:'undefined'}),arguments:arguments_},context:current};}
  case 'errorBinary':return lowerJavaScriptSourceBinary(expression,next,depth+1);
  case 'errorConditional':{const condition=lowerJavaScriptErrorExpression(expression.condition,next,depth+1),yes=lowerJavaScriptErrorExpression(expression.yes,condition.context,depth+1),no=lowerJavaScriptErrorExpression(expression.no,yes.context,depth+1);return {$:'errorSourceExpression',source:{$:'sourceIf',condition:condition.source,yes:yes.source,no:no.source},context:no.context};}
  case 'errorStrictEqual':{const left=lowerJavaScriptErrorExpression(expression.left,next,depth+1),right=lowerJavaScriptErrorExpression(expression.right,left.context,depth+1),source={$:next.mutableBindings?'sourceSnapshotStrictEqual':'sourceStrictEqual',left:left.source,right:right.source};return {$:'errorSourceExpression',source:expression.negate?{$:'sourceIf',condition:source,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:source,context:right.context};}
  case 'errorArrayLiteral':{
   if(!next.mutableBindings||next.realm.arrayPrototype===undefined)return refuseJavaScriptErrorSource(next,'Array literals require the actual ordinary-source realm Array prototype');
   let current=next;const elements=[];for(const element of expression.elements){const value=lowerJavaScriptErrorExpression(element.value,current,depth+1);current=value.context;elements.push({$:'sourceArrayElement',index:element.index,value:value.source});}
   return {$:'errorSourceExpression',source:{$:'sourceArray',prototype:constant(next.realm.arrayPrototype),length:expression.length,elements,realm:next.realm.realm},context:current};
  }
  case 'errorObject':{let current=next;const properties=[];for(const property of expression.properties){const value=lowerJavaScriptErrorExpression(property.value,current,depth+1);current=value.context;properties.push({$:'sourceObjectProperty',key:constant({$:'text',units:property.key}),value:value.source});}return {$:'errorSourceExpression',source:{$:'sourceObject',prototype:constant(next.realm.objectPrototype),properties,realm:next.realm.realm},context:current};}
  case 'errorTemplate':{let current=next,source=constant({$:'text',units:[]});for(const part of expression.parts){if(part.$==='errorTemplateText')source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,constant({$:'text',units:part.units})]};else{const value=lowerJavaScriptErrorExpression(part.value,current,depth+1);current=value.context;source={$:'sourcePrimitive',operation:'concatenate text',arguments:[source,{$:'sourcePrimitive',operation:'convert primitive text',arguments:[value.source]}]};}}return {$:'errorSourceExpression',source,context:current};}
  case 'errorCall':{const callee=lowerJavaScriptErrorExpression(expression.callee,next,depth+1),receiver=lowerJavaScriptErrorExpression(expression.receiver,callee.context,depth+1);let current=receiver.context,arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}return {$:'errorSourceExpression',source:expression.construct?{$:'sourceConstructErrorClass',callee:callee.source,arguments:arguments_}:{$:next.mutableBindings?'sourceSnapshotCall':'sourceCall',callee:callee.source,receiver:receiver.source,arguments:arguments_},context:current};}
  case 'errorMethodCall':{const object=lowerJavaScriptErrorExpression(expression.object,next,depth+1),key=lowerJavaScriptErrorExpression(expression.key,object.context,depth+1),identity=key.context.next;let current={...key.context,next:identity+1},arguments_=[];for(const argument of expression.arguments){const value=lowerJavaScriptErrorExpression(argument,current,depth+1);current=value.context;arguments_.push(value.source);}const receiver={$:'sourceBinding',identity};return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(current,identity,object.source,{$:next.mutableBindings?'sourceSnapshotCall':'sourceCall',callee:constructJavaScriptErrorPropertyRead(receiver,key.source,next),receiver,arguments:arguments_},false),context:current};}
 }
}
/** @param {JavaScriptErrorExpression} expression @param {JavaScriptErrorSourceContext} context @param {number} depth @returns {JavaScriptErrorSourceExpression} */
export function lowerJavaScriptSourceBinary(expression,context,depth){
 if(expression.$!=='errorBinary')return refuseJavaScriptErrorSource(context,'binary source lowering needs its actual operator node');
 const left=lowerJavaScriptErrorExpression(expression.left,context,depth+1),first=left.context.next,right=lowerJavaScriptErrorExpression(expression.right,{...left.context,next:first+1},depth+1),second=right.context.next,next={...right.context,next:second+1},a={$:'sourceBinding',identity:first},b={$:'sourceBinding',identity:second},constant=constructJavaScriptErrorConstant;
 if(expression.operator==='&&'||expression.operator==='||')return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,first,left.source,{$:'sourceIf',condition:a,yes:expression.operator==='&&'?right.source:a,no:expression.operator==='&&'?a:right.source},false),context:next};
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
 if(statement.$==='sourceReturnStatement'||statement.$==='sourceThrowStatement'){const value=lowerJavaScriptErrorExpression(statement.value,context,depth+1);return {...value,source:{$:statement.$==='sourceThrowStatement'?'sourceThrow':'sourceReturn',value:value.source}};}
 const rest=lowerJavaScriptSourceStatements(statements,first+1,context,continuation,depth+1);
 if(statement.$==='sourceBlockStatement')return lowerJavaScriptSourceStatements(statement.statements,0,rest.context,rest.source,depth+1);
 if(statement.$==='sourceWhileStatement'){const condition=lowerJavaScriptErrorExpression(statement.condition,rest.context,depth+1),body=lowerJavaScriptSourceStatements(statement.body,0,condition.context,constructJavaScriptErrorConstant({$:'undefined'}),depth+1);return {$:'errorSourceExpression',source:{$:'sourceWhile',condition:condition.source,body:body.source,after:rest.source},context:body.context};}
 if(statement.$==='sourceIfStatement'){const condition=lowerJavaScriptErrorExpression(statement.condition,rest.context,depth+1),yes=lowerJavaScriptSourceStatements(statement.yes,0,condition.context,rest.source,depth+1),no=lowerJavaScriptSourceStatements(statement.no,0,yes.context,rest.source,depth+1);return {$:'errorSourceExpression',source:{$:'sourceIf',condition:condition.source,yes:yes.source,no:no.source},context:no.context};}
 const local=statement.$==='sourceConstStatement'||statement.$==='sourceMutableStatement',value=lowerJavaScriptErrorExpression(statement.value,rest.context,depth+1),identity=local?statement.identity:value.context.next,next=local?value.context:{...value.context,next:value.context.next+1};return {$:'errorSourceExpression',source:constructJavaScriptErrorLocalBinding(next,identity,value.source,rest.source,statement.$==='sourceMutableStatement'),context:next};
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
  let context={$:'errorSourceContext',registry,realm,capabilities,next:plan.nextBinding,visits:0,error:'',mutableBindings:definition.$==='errorOrdinaryConstructorPlan'||definition.$==='errorMethodFunctionPlan'||definition.$==='errorMutableStatementFunctionPlan'},source=constructJavaScriptErrorConstant({$:'undefined'});
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
  for(let at=definition.parameters.length-1;at>=0;at-=1){const parameter=definition.parameters[at];if(parameter===undefined)continue;
   if(parameter.$==='errorObjectParameter'){
    for(let fieldAt=parameter.fields.length-1;fieldAt>=0;fieldAt--){const field=parameter.fields[fieldAt];if(field===undefined)continue;const raw=constructJavaScriptErrorPropertyRead({$:'sourceBinding',identity:parameter.identity},constructJavaScriptErrorConstant({$:'text',units:field.key}),context);
     if(field.hasDefault){const value=lowerJavaScriptErrorExpression(field.value,context,0),identity=value.context.next;context={...value.context,next:identity+1};const saved={$:'sourceBinding',identity};source=constructJavaScriptErrorLocalBinding(context,identity,raw,constructJavaScriptErrorLocalBinding(context,field.identity,{$:'sourceIf',condition:{$:'sourceStrictEqual',left:saved,right:constructJavaScriptErrorConstant({$:'undefined'})},yes:value.source,no:saved},source,true),false);}
     else source=constructJavaScriptErrorLocalBinding(context,field.identity,raw,source,true);
    }
    if(parameter.hasDefault){const value=lowerJavaScriptErrorExpression(parameter.value,context,0);context=value.context;source={$:'sourceParameterDefault',identity:parameter.identity,value:value.source,body:source};}
   }else if(parameter.$==='errorDefaultParameter'){const value=lowerJavaScriptErrorExpression(parameter.value,context,0);context=value.context;source={$:'sourceParameterDefault',identity:parameter.identity,value:value.source,body:source};}
  }
  if(context.error!=='')return {$:'errorModuleCompilation',program:empty,error:context.error};
  const code=compileJavaScriptCallSourceWithCaptures(source,definition.parameters.map(parameter=>parameter.identity),bindings.map(binding=>binding.identity));if(code.error!=='')return {$:'errorModuleCompilation',program:empty,error:code.error};
  functions.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:definition.$==='errorConstructorPlan'?'derived Error':definition.$==='errorOrdinaryConstructorPlan'?'ordinary class':'strict',lexicalReceiver:{$:'undefined'},parameters:definition.parameters.length,registers:code.registers,captures:bindings.map(binding=>binding.value),instructions:code.instructions});
 }
 for(const intrinsic of registry.intrinsics){if(identities.includes(intrinsic.identity)||!registry.realms.includes(intrinsic.realm))return {$:'errorModuleCompilation',program:empty,error:'external Error callbacks require distinct actual callable/realm identities'};identities.push(intrinsic.identity);functions.push({$:'callFunction',identity:intrinsic.identity,realm:intrinsic.realm,receiverMode:'Error intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'invokeErrorIntrinsic',destination:1},{$:'return',source:1}]});}
 for(let at=0;at<capabilities.length;at++){const capability=capabilities[at];if(capability===undefined||!capability.executable||capabilities.findIndex(row=>row.identity===capability.identity)!==at)continue;if(identities.includes(capability.identity))return {$:'errorModuleCompilation',program:empty,error:'a host capability cannot replace an actual source or Error callable body'};identities.push(capability.identity);functions.push({$:'callFunction',identity:capability.identity,realm:capability.realm,receiverMode:'qualified host capability',lexicalReceiver:{$:'undefined'},parameters:0,registers:2,captures:[],instructions:[{$:'invokeHostCapability',destination:1},{$:'return',source:1}]});}
 functions=functions.toSorted((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0);
 return {$:'errorModuleCompilation',program:{$:'callProgram',functions,realms:registry.realms,exoticKey:registry.exoticKey},error:''};
}
