// Unconstrained source generics stay opaque during selected-call checking.
// Concrete u8/Clone choices are not evidence of universal Copy/Add/Clone bounds.
import {unsupported} from './diagnostics.js';
import {readRustStandardSumSourceFieldArgument} from './rust-standard-sum-source-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust source FnMut opaque generic',reason,span);};
function enabled(parser){return parser.fnMutSourceModel==='Rust closed source FnMut specializations 1'&&parser.fnMutSourceSpecialization?.declaration.generics.length>0;}
function tagKey(value){return value?String(value.declaration)+':'+value.name:'';}
export function withRustFnMutSourceErrorType(parser,owner,index,read){
 if(!enabled(parser)||owner!=='std::result::Result'||index!==1)return read();
 const context=parser.fnMutSourceSpecialization,scopes=context.opaqueErrorScopes??=[],scope={start:parser.cursor.peek().start,depth:parser.sourceOwnerFrames.length||1};scopes.push(scope);
 try{return read();}finally{scopes.pop();}
}
export function markRustFnMutSourceGenericType(parser,token,origin){
 const context=parser.fnMutSourceSpecialization,scopes=context?.opaqueErrorScopes,scope=scopes?.at(-1);if(!scope||scopes.length!==1||scope.depth!==1||scope.start!==token.start)fail('an unconstrained concrete type binder is admitted only as a bare standard Result error position',token);
 return {...origin,sourceFnMutGenericBinder:{declaration:context.declaration.range.start,name:token.value,concreteCopy:origin.copyEvidence===true}};
}
function tags(origin){
 const pending=[origin],seen=new Set(),result=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(seen.size>=65536)fail('the complete opaque source position exceeds its inspection budget');seen.add(value);if(value.sourceFnMutGenericBinder)result.add(tagKey(value.sourceFnMutGenericBinder));pending.push(...(value.fields??[]),...(value.standardSumSource?.arguments??[]));}return [...result].sort();
}
export function hasRustFnMutSourceOpacity(origin){return tags(origin).length!==0;}
export function eraseRustFnMutSourceOpacity(parser,origin){
 if(!hasRustFnMutSourceOpacity(origin))return origin;const result=structuredClone(origin),pending=[result],seen=new Set();while(pending.length){const value=pending.pop();if(!value||seen.has(value))continue;seen.add(value);if(value.sourceFnMutGenericBinder)value.copyEvidence=value.sourceFnMutGenericBinder.concreteCopy;delete value.sourceFnMutGenericBinder;const declared=value.type?.kind==='named'?parser.origins.lookupTypeOwner(value.type.path,value.path):null;if(declared?.standardSumSourceOrigin)value.copyEvidence=declared.standardSumSourceOrigin.copyEvidence;pending.push(...(value.fields??[]),...(value.standardSumSource?.arguments??[]));}return result;
}
export function carryRustFnMutSourceOpacity(source,target){
 if(!hasRustFnMutSourceOpacity(source))return target;const result=structuredClone(target),pending=[[source,result]];let count=0;
 while(pending.length){if(++count>65536)fail('opaque source argument transport exceeds its bounded complete graph');const[a,b]=pending.pop();if(a.sourceFnMutGenericBinder)b.sourceFnMutGenericBinder={...a.sourceFnMutGenericBinder};for(let at=0;at<(a.standardSumSource?.arguments.length??0);at++)if(b.standardSumSource?.arguments[at])pending.push([a.standardSumSource.arguments[at],b.standardSumSource.arguments[at]]);for(let at=0;at<(a.fields?.length??0);at++)if(b.fields?.[at])pending.push([a.fields[at],b.fields[at]]);if(tags(a).length&&b.wrappers?.length===0)b.copyEvidence=false;}return result;
}
export function carryRustFnMutSourceArguments(origin,arguments_){return carryRustFnMutSourceOpacity({standardSumSource:{arguments:arguments_}},origin);}
export function rustFnMutSourceConstructorField(origin,node,index,field){
 const sum=origin.standardSumSource;if(!sum||index!==0)return field;const segments=node.k==='app'?node.fn?.path:node.path,variant=segments?.at(-1),argument=readRustStandardSumSourceFieldArgument(sum.owner,variant);return argument>=0&&sum.arguments[argument]?carryRustFnMutSourceOpacity(sum.arguments[argument],field):field;
}
export function rustFnMutSourceOpaqueOwnership(origin){
 if(!origin?.proven||origin.wrappers?.[0]==='reference'||!hasRustFnMutSourceOpacity(origin))return null;const sum=origin.standardSumSource,owner=sum?.owner??'',roles=sum?.arguments.map(value=>hasRustFnMutSourceOpacity(value)?'owned':value.copyEvidence&&value.wrappers.length===0?'copy':'owned')??[];return {profile:'owned',owner,roles,definitionName:origin.type?.kind==='named'?origin.type.path.filter(part=>part!=='crate').join('.'):'',cloneImplemented:false};
}
export function rustFnMutSourceArgumentError(parser,actual,expected){
 if(!enabled(parser)||!parser.fnMutSourceSpecialization.runtime||actual.diverging)return '';const left=tags(actual),right=tags(expected);return left.length!==right.length||left.some((value,index)=>value!==right[index])?'a generic Result error stays opaque across source argument, binding and return positions':'';
}
export function guardRustFnMutSourceObservation(parser,node,path,operation,span){
 if(!enabled(parser)||!parser.fnMutSourceSpecialization.runtime)return;const origin=parser.origins.origin(node,path);if(hasRustFnMutSourceOpacity(origin)&&!['is_ok','is_err','is_some','is_none'].includes(operation))fail('this generic body admits opaque Result propagation and discriminant queries; '+operation+' needs its real universal trait/ownership interpretation',span);
}
export function guardRustFnMutSourceResidual(parser,operand,target,span){
 if(!enabled(parser))return;const a=tags(operand),b=tags(target);if(a.length!==b.length||a.some((value,index)=>value!==b[index]))fail('generic residual errors retain the same actual opaque binder without monomorphic From assumptions',span);
}
export function guardRustFnMutSourceBodyTokens(parser,declaration){
 if(!declaration.generics.length)return;for(const token of parser.sourceConstantTokens){if(token.start<declaration.bodyRange.start||token.end>declaration.bodyRange.end)continue;if(token.kind==='identifier'&&token.value==='Err'||token.kind==='macro'&&token.value==='vec')fail('explicit generic error extraction/construction and inferred vector aggregation need their separate universal source interpretation',token);}
}
