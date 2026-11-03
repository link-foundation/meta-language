// Single-constructor match attempts share actual irrefutable field bindings.
// Fresh lexical names keep a failed attempt from capturing its continuation.
import {unsupported} from './diagnostics.js';
import {hasRustSourceRegionComposition,retainRustSourceRegionState} from './rust-source-region-composition.js';
import {readRustNominalOwnershipFacts} from './rust-nominal-ownership.js';
import {readRustOwnedRecordStateStorageError} from './rust-owned-record-state-rules.js';
import {renameRustSourceMatchBindings} from './rust-owned-record-effects.js';
const fail=(reason,span)=>{throw unsupported('Rust source place match',reason,span);};
export function readRustSourcePlaceMatchAttemptPlan(parser){
 const c=parser.cursor,stack=[],guards=[];let pattern=true,guarded=false;
 for(let offset=0;offset<100000;offset++){const token=c.peek(offset),value=token.value;if(token.kind==='eof')return null;
  if(pattern&&!guarded&&(['string','char','byteChar','byteString'].includes(token.kind)||token.kind==='number'&&c.peek(offset+1).value!==':'||['@','|','-','..='].includes(value)||['true','false'].includes(value)&&!token.raw?.startsWith('r#')))return null;
  if(['(','[','{'].includes(value)){stack.push(value==='('?')':value==='['?']':'}');continue;}
  if([')',']','}'].includes(value)){if(!stack.length)return value==='}'&&guards.length&&(!pattern||!guarded)&&!guards.at(-1)&&guards.slice(0,-1).every(Boolean)?guards:null;if(stack.pop()!==value)return null;continue;}
  if(stack.length)continue;if(value==='=>'){if(!pattern)return null;guards.push(guarded);pattern=false;continue;}if(value===','){if(pattern)return null;pattern=true;guarded=false;continue;}if(pattern&&value==='if'&&!token.raw?.startsWith('r#'))guarded=true;
 }
 fail('the complete source match attempt plan exceeds its token inspection budget',c.peek());
}
export function acceptRustSourcePlaceMatch(parser,scrutinee,path){
 if(!hasRustSourceRegionComposition(parser)||!retainRustSourceRegionState(parser))return false;const actual=parser.origins.origin(scrutinee,path),owner=actual.type?.kind==='named'?parser.origins.lookupTypeOwner(actual.type.path,actual.path):null,row=owner&&parser.origins.nominalOwnershipDeclarations.get(owner.fullName);
 return Boolean(owner?.k==='data'&&row?.constructors?.length===1&&['owned','copy'].includes(readRustNominalOwnershipFacts(parser.origins,{...actual,wrappers:[]}).profile)&&actual.wrappers.every(layer=>layer==='reference'));
}
export function acceptRustSourcePlaceMatchPattern(parser,pattern){
 const pending=[{pattern,depth:0}];let visits=0;while(pending.length){const {pattern:current,depth}=pending.pop(),budget=readRustOwnedRecordStateStorageError(++visits,depth);if(budget)fail(budget,current.span);if(!['ctor','bind','bindOrCtor','wild'].includes(current.k))return false;pending.push(...(current.args??[]).map(pattern=>({pattern,depth:depth+1})));}return true;
}
export function beginRustSourcePlaceMatch(parser,scrutinee,path,place){
 const scope=parser.origins.scope,before=parser.origins.snapshotOwnedBindings(),prefix=[];parser.origins.scope=new Map(scope);let object=scrutinee;
 if(!place){const name=parser.freshProductBinding();parser.origins.bind(name,scrutinee,path);prefix.push({name,value:scrutinee,span:scrutinee.span});object=parser.origins.mark({k:'name',path:[name],span:scrutinee.span},parser.origins.scope.get(name));}
 return {scope,before,prefix,object};
}
export function lowerRustSourcePlaceMatch(parser,context,arms,path,span,origin){
 if(!arms.length||arms.at(-1).guard||arms.slice(0,-1).some(arm=>!arm.guard))fail('a place-match attempt chain retains guarded attempts followed by its actual final exhaustive arm',span);
 const wrap=(bindings,body)=>bindings.reduceRight((body,binding)=>parser.origins.mark({k:'let',...binding,body,span:binding.span??span},origin),body);
 let result=renameRustSourceMatchBindings(parser,wrap(arms.at(-1).bindings,arms.at(-1).body),path);
 for(const arm of arms.slice(0,-1).toReversed()){
  const hole=parser.freshProductBinding(),success=parser.freshProductBinding(),body=renameRustSourceMatchBindings(parser,wrap(arm.bindings,arm.body),path),attempt=parser.origins.mark({k:'if',cond:arm.guard,then:{k:'name',path:[success],span:arm.span},else:{k:'name',path:[hole],span:arm.span},span:arm.span},origin),renamed=renameRustSourceMatchBindings(parser,wrap(arm.guardBindings??[],attempt),path);let visits=0;
  const plug=node=>{if(++visits>65536)fail('a match continuation exceeds its bounded actual binding spine',span);if(node.k==='let')return parser.origins.reuseExpressionOrigin(node,{...node,body:plug(node.body)});if(node.k!=='if'||node.then?.k!=='name'||node.then.path.length!==1||node.then.path[0]!==success||node.else?.k!=='name'||node.else.path.length!==1||node.else.path[0]!==hole)fail('a match attempt retains its actual distinct success and outer failure continuations',span);return parser.origins.reuseExpressionOrigin(node,{...node,then:body,else:result});};
  result=plug(renamed);
 }
 result=wrap(context.prefix,result);parser.origins.mark(result,origin);parser.origins.expireOwnedBindings(context.before,origin);parser.origins.scope=context.scope;return result;
}
