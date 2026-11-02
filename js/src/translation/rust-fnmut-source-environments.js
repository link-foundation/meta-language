// Mutable closure captures become actual source state formals and ordinary
// result/state packets. The existing state-body interpreter owns all effects.
import {unsupported} from './diagnostics.js';
import {beginRustOwnedRecordStateBlock,endRustOwnedRecordStateBlock,readRustOwnedRecordStateSlots,readRustOwnedRecordStateDeclarations} from './rust-owned-record-state.js';
import {lowerRustOwnedRecordEffectBody} from './rust-owned-record-effects.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readCallableStatementCaptures} from './callable-source.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {testRustFnMutSourcePosition} from './rust-fnmut-source-positions.js';
import {beginRustSourceRegionCallback,endRustSourceRegionBody} from './rust-source-region-composition.js';
const active=new WeakMap(),records=new WeakMap(),frames=new WeakSet();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut environment',reason,span);};
export function withRustFnMutSourceClosure(parser,expected,readActual){
 if(active.has(parser)||parser.fnMutSourceModel!=='Rust closed source FnMut specializations 1'||typeof readActual!=='function')fail('a source FnMut closure needs its actual nonnested selected callback reader');
 const context={expected,caller:parser.ownedRecordState?.currentCallable??0,roots:readRustOwnedRecordStateSlots(parser),entered:false};if(context.roots.length>256)fail('the actual mutable callback environment exceeds its source root budget');const regions=beginRustSourceRegionCallback(parser);active.set(parser,context);
 try{return readActual();}finally{active.delete(parser);endRustSourceRegionBody(parser,regions);}
}
// A shared reference is Copy independently of its closed referent. This
// header role does not upgrade the referent or release a returned reference.
export function readRustFnMutSourceClosureParameterRole(parser,origin,index){
 const context=active.get(parser),expected=context?.expected.callableParameters?.[index];return context&&origin?.proven&&origin.wrappers?.length===1&&origin.wrappers[0]==='reference'&&testRustFnMutSourcePosition(parser,origin,expected)?'copy':'';
}
export function beginRustFnMutSourceEnvironment(parser){
 const context=active.get(parser);if(!context)return null;if(context.entered)fail('nested mutable callbacks need their own selected specialization context');context.entered=true;
 const state=parser.ownedRecordState,frame=beginRustOwnedRecordStateBlock(parser,false),aliases=[],declarations=readRustOwnedRecordStateDeclarations(parser),reads=parser.ownedRecordEffects?.reads.length??0,writes=parser.ownedRecordEffects?.writes.length??0,calls=parser.exclusiveSourceCalls?.calls.length??0,exclusiveWrites=parser.exclusiveSourceCalls?.writes.length??0;
 for(const root of context.roots){if(root.callable!==context.caller)fail('each actual mutable capture retains its original callable and lexical root');const parameter=parser.freshProductBinding(),slot={...root,origin:{...structuredClone(root.origin),sourceBorrowedPlace:true,sourceExclusiveParameter:true},currentName:parameter,frame:frame.id,callable:state.currentCallable,exclusiveMode:'exclusive'};state.slots.set(slot.lexical,slot);declarations.set(parameter,slot);aliases.push({outer:root,slot,parameter});}
 const result={parser,context,frame,aliases,reads,writes,calls,exclusiveWrites};frames.add(result);return result;
}
export function finishRustFnMutSourceEnvironment(parser,environment,parameters,result,body,path,span,outer){
 if(!environment)return null;if(!frames.has(environment)||environment.parser!==parser||active.get(parser)!==environment.context||!result?.type||!result.proven)fail('a callback environment needs its actual complete parsed body and result origin',span);
 const parameterOrigins=parameters.map(parameter=>parser.origins.scope.get(parameter.name)),expected=environment.context.expected.callableParameters;if(!Array.isArray(expected)||expected.length!==parameters.length||parameterOrigins.some((origin,index)=>!testRustFnMutSourcePosition(parser,origin,expected[index])))fail('each callback parameter retains its actual complete source annotation and selected bound, independently of erased core type equality',span);
 const used=new Set(),consider=slot=>{if(slot&&slot.callable===parser.ownedRecordState.currentCallable&&environment.aliases.some(row=>row.slot.lexical===slot.lexical&&row.slot.root===slot.root&&row.slot.callable===slot.callable))used.add(slot.lexical);};
 for(const slot of parser.ownedRecordEffects.reads.slice(environment.reads))consider(slot);
 for(const write of parser.ownedRecordEffects.writes.slice(environment.writes))consider(write.slot);
 for(const call of (parser.exclusiveSourceCalls?.calls??[]).slice(environment.calls))for(const borrow of call.borrows)consider(borrow.slot);
 for(const write of (parser.exclusiveSourceCalls?.writes??[]).slice(environment.exclusiveWrites))consider(write.slot);
 const captures=environment.aliases.filter(row=>used.has(row.slot.lexical)),initial=new Map(captures.map(row=>[row.slot.lexical,{slot:row.slot,name:row.parameter}])),declarations=new Map(readRustOwnedRecordStateDeclarations(parser)),ret={kind:'tuple',elements:[readRustRecordSourceType(parser,result.type,result.path),...captures.map(row=>readRustRecordSourceType(parser,row.slot.origin.type,row.slot.origin.path))]},lowered=lowerRustOwnedRecordEffectBody(parser,body,path,declarations,initial,ret.elements[0],(value,after)=>({k:'tuple',items:[value,...captures.map(row=>{const state=after.get(row.slot.lexical);if(!state||state.slot.root!==row.slot.root)fail('every callback completion publishes its actual still-live capture root',span);return {k:'name',path:[state.name],span};})],span}));
 const expanded=[...parameters,...captures.map(row=>({name:row.parameter,type:readRustRecordSourceType(parser,row.slot.origin.type,row.slot.origin.path),span}))],observed=readCallableStatementCaptures([{s:'return',expr:lowered}],expanded.map(parameter=>parameter.name),outer.keys());
 if(observed.assignedCaptures.length)fail('every actual mutable callback capture must be represented by its source state slot',span);
 for(const name of observed.captures){const origin=outer.get(name);checkRustOwnedOrigin(parser.origins,origin,span);const source=parser.fnMutSourceSpecialization,callback=origin?.sourceFnMutParameter&&origin.sourceLexicalBindingIdentity===source?.runtime?.callbackLexical&&name===source.declaration.callback.name;if(!callback&&(!parser.origins.immutableProduct(origin)||origin.wrappers.length||origin.sourceBorrowedPlace))fail('a nonstate callback capture needs its actual immutable Copy value or selected callback formal with explicit state; borrowed or owned captured environments need their separate live carrier interpretation',span);}
 const node={k:'lambda',params:expanded,ret,captures:observed.captures,body:lowered,span},origin={...parser.origins.typeOrigin({kind:'function',params:expanded.map(parameter=>parameter.type),ret},path),copyEvidence:false,callableParameters:parameterOrigins.map(origin=>structuredClone(origin)),callableResult:parser.origins.typeOrigin(ret,path),sourceFnMutClosure:true};parser.origins.mark(node,origin);records.set(node,{parser,node,parameters,parameterOrigins:parameterOrigins.map(origin=>structuredClone(origin)),result,captures,immutableCaptures:observed.captures,path:[...path],range:{...span}});return node;
}
export function endRustFnMutSourceEnvironment(parser,environment){if(!environment)return;if(!frames.has(environment)||environment.parser!==parser)fail('callback state frames must leave their actual source parser');endRustOwnedRecordStateBlock(parser,environment.frame);frames.delete(environment);}
export function readRustFnMutSourceClosureRecord(parser,node){const record=records.get(node);if(!record||record.parser!==parser)fail('a mutable callback body must originate in this actual source state reader',node?.span);return record;}
