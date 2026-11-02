// Explicit source adapter over actual parser-owned slots and the established
// state/completion lowerer. This entry does not parse or admit &mut syntax.
import {lowerRustOwnedRecordEffectBody} from './rust-owned-record-effects.js';
import {readRustOwnedRecordStateDeclarations,readRustOwnedRecordStateSlots,readRustOwnedRecordStateCallable} from './rust-owned-record-state.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustExclusiveSourceLoanContextError,readRustExclusiveSourcePositionBudgetError,readRustExclusiveSourceLoanStateError,beginRustExclusiveSourceCall,finishRustExclusiveSourceCall,createRustExclusiveSourceReturnPacket,lowerRustExclusiveSourceCallWriteback} from './rust-exclusive-source-loans.js';
import {unsupported} from './diagnostics.js';
const fail=(reason,node)=>{throw unsupported('Rust exclusive source call adapter',reason,node?.span);};
function sameOwner(left,right){return Boolean(left&&right)&&left.kind===right.kind&&left.path.length===right.path.length&&left.path.every((value,index)=>value===right.path[index])&&left.arguments.length===right.arguments.length&&left.arguments.every((value,index)=>sameOwner(value,right.arguments[index]));}
function slotPosition(slot,expected){const origin=slot.origin,owner=origin?.expandedSourceOwner??origin?.sourceOwner;const budget=readRustExclusiveSourcePositionBudgetError({type:origin?.type,owner,wrappers:origin?.wrappers});if(budget)return budget;return origin?.proven===true&&origin.wrappers.length===0&&origin.type?.kind===expected.type.kind&&(origin.sourceCollectionTypeIdentity??'')===(expected.collectionTypeIdentity??'')&&sameOwner(owner,expected.owner)?'':'an exclusive parameter slot needs its actual proven referent type and complete source owner';}
// parameterSlots: [{parameter,binder,lexical}], projected from the actual
// source header's exclusive formals after admission has established referents.
// Every supplied lexical identity must already exist in the parser's slot map.
export function lowerRustExclusiveSourceCallable(parser,context,callee,declaration,modulePath,parameterSlots){
 const invalid=readRustExclusiveSourceLoanContextError(context);if(invalid)fail(invalid,declaration);
 const entry=context.callables.find(row=>row.identity===callee),expected=entry?.parameters.map((row,index)=>({...row,index})).filter(row=>row.mode==='exclusive');
 if(!entry||!expected?.length||declaration?.k!=='fn'||entry.path.join('::')!==['crate',...modulePath,declaration.name].join('::')||declaration.params.length!==entry.parameters.length||!Array.isArray(parameterSlots)||parameterSlots.length!==expected.length)fail('the adapter retains its actual callable declaration and every exclusive formal',declaration);
 if(Object.hasOwn(declaration,'sourceSignature'))fail('exclusive packet adaptation requires its separate source-signature profile; existing immutable signature facets cannot be relabeled',declaration);
 if(parser?.origins?.collecting!==false||!parser.ownedRecordEffects)fail('exclusive body adaptation requires the actual completed source origin inventory and state-effect arena',declaration);
 const inventory=readRustOwnedRecordStateDeclarations(parser),slots=readRustOwnedRecordStateSlots(parser),callable=readRustOwnedRecordStateCallable(parser),initial=new Map(),bound=[];
 for(let index=0;index<expected.length;index++){
  const formal=expected[index],binding=parameterSlots[index],actual=context.binders.find(row=>row.identity===binding.binder),slot=slots.find(row=>row.lexical===binding.lexical),parameter=declaration.params[formal.index];
  if(binding.parameter!==formal.index||actual?.mode!=='exclusive'||actual.name!==parameter.name||actual.lexical!==binding.lexical||!slot||slot.callable!==callable||slot.name!==parameter.name||!slot.root||slot.origin.sourceBindingIdentity!==slot.root||slot.origin.sourceLexicalBindingIdentity!==slot.lexical||!inventory.has(slot.currentName)||inventory.get(slot.currentName)!==slot)fail('exclusive formals retain actual parser binder, callable, root and lexical slot identities',declaration);
  const error=slotPosition(slot,formal.position);if(error)fail(error,declaration);
  initial.set(slot.lexical,{slot,name:parameter.name});bound.push({formal,binding,slot});
 }
 const ret=readRustRecordSourceType(parser,entry.result.type,modulePath),resultTypes=[ret,...bound.map(row=>readRustRecordSourceType(parser,row.formal.position.type,modulePath))];
 const body=lowerRustOwnedRecordEffectBody(parser,declaration.body,modulePath,inventory,initial,ret,(value,environment)=>{
  const referents=bound.map(row=>{const current=environment.get(row.slot.lexical);if(!current||current.slot.root!==row.slot.root||current.slot.lexical!==row.slot.lexical)fail('every normal or source completion retains its actual exclusive root state',value);return {parameter:row.formal.index,value:{k:'name',path:[current.name]}};});
  const packet=createRustExclusiveSourceReturnPacket(context,callee,value,referents);if(packet.$!=='rustExclusiveSourceLowered')fail(packet.reason,value);return packet.expression;
 });
 return {declaration:{...declaration,ret:{kind:'tuple',elements:resultTypes},body},interface:{callee,originalParameters:declaration.params,originalResult:declaration.ret,exclusiveParameters:parameterSlots.map(row=>({...row}))}};
}
// Argument expressions have already been held once by the ordinary source
// evaluator. Acquired loans persist across following argument evaluation;
// this adapter performs the actual call and writes back before continuation.
export function lowerRustExclusiveSourceInvocation(context,state,callee,evaluatedArguments,arguments_,roots,names,continuation){
 const invalid=readRustExclusiveSourceLoanStateError(context,state);if(invalid)return {$:'rustExclusiveSourceLoweringInvalid',reason:invalid};
 const selected=beginRustExclusiveSourceCall(context,state,callee,arguments_);if(selected.$!=='rustExclusiveSourceContinue')return {$:'rustExclusiveSourceLoweringInvalid',reason:selected.reason};
 const entry=context.callables.find(row=>row.identity===callee),packet={k:'app',fn:{k:'name',path:entry.path},args:evaluatedArguments},lowered=lowerRustExclusiveSourceCallWriteback(context,selected.state,packet,roots,names,continuation);
 if(lowered.$!=='rustExclusiveSourceLowered')return lowered;const finished=finishRustExclusiveSourceCall(context,selected.state,selected.state.call.identity);return finished.$==='rustExclusiveSourceContinue'?{...lowered,state:finished.state}:{$:'rustExclusiveSourceLoweringInvalid',reason:finished.reason};
}
