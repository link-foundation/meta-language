// Closed source monomorphization reparses real declarations. Original generic
// and callback bodies are never replaced by a name-indexed native dispatcher.
import {unsupported,typeError} from './diagnostics.js';
import {readRustSourceVersion} from './rust-source-versions.js';
import {readRustFnMutSourceModelError} from './rust-fnmut-source-declaration-rules.js';
import {captureRustFnMutSourceDeclaration,readRustFnMutSourceDeclaration,specializeRustFnMutSourceDeclaration,previewRustFnMutSourceDeclaration} from './rust-fnmut-source-declarations.js';
import {readRustFnMutSourceCallback,readRustFnMutSourceCallbackBinding} from './rust-fnmut-source-callbacks.js';
import {createRustSourceSignature} from './rust-source-signatures.js';
import {beginRustOwnedRecordStateBlock,endRustOwnedRecordStateBlock,registerRustExclusiveSourceStateRoot,readRustOwnedRecordStateDeclarations} from './rust-owned-record-state.js';
import {initializeRustOwnedRecordEffects,lowerRustOwnedRecordEffectBody} from './rust-owned-record-effects.js';
import {readRustRecordSourceType} from './rust-records.js';
import {UNIT} from './types.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustOwnedRecordStatePlace,readRustOwnedRecordStateSlots} from './rust-owned-record-state.js';
import {acquireRustExclusiveSourceStateArguments,recordRustExclusiveSourcePacketEffect,recordRustExclusiveSourceLocalPacketEffect} from './rust-exclusive-source-calls.js';
import {captureRustInherentReceiverValueLoan} from './rust-inherent.js';
import {prepareRustBorrowedInherentReceiverPosition} from './rust-borrowed-expression-positions.js';
import {planRustInherentReceiverLayers} from './rust-inherent-rules.js';
import {testRustFnMutSourcePosition} from './rust-fnmut-source-positions.js';
import {rustFnMutSourceOpaqueOwnership,rustFnMutSourceArgumentError,guardRustFnMutSourceObservation,eraseRustFnMutSourceOpacity,rustFnMutSourceConstructorField} from './rust-fnmut-source-opacity.js';
const registries=new WeakMap(),interfaces=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut function',reason,span);};
function registry(parser){const value=registries.get(parser);if(!value)fail('the actual source FnMut parser inventory is not initialized');return value;}
export function initializeRustFnMutSource(parser,model='',previous=null){
 parser.fnMutSourceModel=model;if(!model)return;
 const version=readRustSourceVersion(parser.exclusiveSourceVersion??''),error=readRustFnMutSourceModelError(model,version.minor);if(error||version.major!==1||parser.exclusiveSourceModel!=='Rust exclusive source loans 1')fail(error||'source FnMut specialization requires the actual exclusive source state profile and covered version');
 const value={templates:new Map(),instances:[],headers:new Map()};registries.set(parser,value);
 parser.origins.fnMutSourceOpaqueOwnership=rustFnMutSourceOpaqueOwnership;
 parser.origins.fnMutSourceArgumentError=(actual,expected)=>rustFnMutSourceArgumentError(parser,actual,expected);
 parser.origins.fnMutSourceConstructorField=rustFnMutSourceConstructorField;
 parser.fnMutSourceObservationGuard=(node,path,operation,span)=>guardRustFnMutSourceObservation(parser,node,path,operation,span);
 for(const row of previous??[]){const handle=captureRustFnMutSourceDeclaration(parser,row.path,{start:row.start}),facts=readRustFnMutSourceDeclaration(parser,handle);value.templates.set([...facts.path,facts.name].join('.'),{handle,facts});}
}
export function readRustFnMutSourceTemplateFacts(parser){const value=registries.get(parser);return value?[...value.templates.values()].map(row=>({path:[...row.facts.path],start:row.facts.range.start})):[];}
function receiver(parser,facts){
 const source=parser.source.slice(facts.range.start,facts.bodyRange.start),round=source.indexOf('('),formal=source.slice(round+1).split(',')[0];return parser.inherentOwner&&/^\s*&\s*self\s*$/u.test(formal)?{name:'self',type:{kind:'named',path:['crate',...parser.inherentOwner.path]}}:null;
}
export function readRustFnMutSourceFunctionTemplate(parser,path,start,name,constKeyword=null){
 if(!parser.fnMutSourceModel||parser.fnMutSourceSpecialization)return null;
 const arena=registry(parser),key=[...path,name].join('.'),tokens=parser.sourceConstantTokens,from=tokens.findIndex(token=>token.start===start.start);let callback=false;
 for(let at=from+1;at<tokens.length&&!(tokens[at].kind==='punct'&&tokens[at].value==='{');at++){if(at-from>1000000)fail('the original function header exceeds its actual source token budget',start);if(tokens[at].value==='FnMut')callback=true;}
 if(!callback){if(arena.templates.has(key)&&arena.templates.get(key).facts.range.start!==start.start)throw typeError('duplicate source generic function '+key,start);return null;}
 if(constKeyword)fail('const generic FnMut calls need their independent const trait/effect interpretation',start);
 const handle=captureRustFnMutSourceDeclaration(parser,path,start),facts=readRustFnMutSourceDeclaration(parser,handle),before=arena.templates.get(key);if(before&&before.facts.range.start!==facts.range.start)throw typeError('duplicate source generic function '+key,start);arena.templates.set(key,{handle,facts});
 const self=receiver(parser,facts),params=self?[self]:[],origin=parser.origins.typeOrigin(UNIT,path);origin.sourceOwner={kind:'unit',path:[],arguments:[]};
 parser.origins.declaration(name,path,{k:'fn',parameters:[],result:origin,sourceFnMutTemplate:true});if(self)parser.inherentMethods.set(key,{owner:path.join('.'),mode:1,receiver:['reference'],sourceFnMutTemplate:true});
 const end=parser.cursor.tokens.findIndex(token=>token.kind==='punct'&&token.value==='}'&&token.end===facts.range.end);if(end<0)fail('the original generic body must remain in the actual token stream',facts.range);parser.cursor.index=end+1;
 return {k:'fn',name,params,ret:UNIT,body:{k:'unit',span:facts.bodyRange},sourceFnMutTemplate:true,span:facts.range};
}
export function readRustFnMutSourceFunctionGenerics(parser,start,name,path){
 const context=parser.fnMutSourceSpecialization;if(!context)return false;const declaration=context.declaration;
 if(start.start!==declaration.range.start||name!==declaration.name||path.join('.')!==declaration.path.join('.'))fail('specialization reparses its actual original generic function owner',start);
 const c=parser.cursor;if(declaration.generics.length){c.expect('<','actual source generic binders');for(let at=0;at<declaration.generics.length;at++){if(at)c.expect(',','source generic binder');const token=c.identifier('source generic binder'),expected=declaration.generics[at];if(token.value!==expected.name||token.start!==expected.start||token.end!==expected.end)fail('generic binders retain their actual original source identity',token);}c.eat(',');c.expect('>','source generic binders');}
 context.headerConsumed=true;return true;
}
function qualified(parser,origin){return {...structuredClone(origin),type:readRustRecordSourceType(parser,origin.type,origin.path),path:[]};}
function functionOrigin(parser,parameters,result,path){parameters=parameters.map(value=>qualified(parser,value));result=qualified(parser,result);return {...parser.origins.typeOrigin({kind:'function',params:parameters.map(value=>value.type),ret:result.type},path),proven:true,copyEvidence:false,wrappers:[],sourceOwner:{kind:'function',path:[],arguments:[...parameters.map(value=>structuredClone(value.sourceOwner)),structuredClone(result.sourceOwner)]},callableParameters:parameters,callableResult:result,sourceFnMutParameter:true};}
export function readRustFnMutSourceParameterType(parser,name,path){
 const context=parser.fnMutSourceSpecialization,original=context?.declaration.callback,c=parser.cursor;if(!original||name!==original.name||c.peek().start!==original.typeRange.start)return null;
 if(original.mode==='exclusive'){c.expect('&','actual exclusive FnMut bound');c.expect('mut','actual exclusive FnMut bound');}c.expect('impl','actual source FnMut bound');c.expect('FnMut','actual source FnMut bound');c.expect('(','source FnMut arguments');const parameters=[];
 while(!c.is(')')){parser.type();parameters.push(structuredClone(parser.origins.type));if(!c.eat(','))break;}c.expect(')','source FnMut arguments');c.expect('->','source FnMut result');parser.type();const result=structuredClone(parser.origins.type);if(c.peek(-1).end!==original.typeRange.end||parameters.length!==original.parameterRanges.length)fail('the complete original FnMut header must finish actual source type admission',original.typeRange);
 context.callbackHeader={parameters,result};context.callbackConsumed=true;
 if(parameters.some(value=>value.type.kind==='function'||value.sourceExclusiveMode==='exclusive'||value.wrappers.length>1||value.wrappers.some(layer=>layer!=='reference')))fail('this source FnMut profile uses direct noncallable values and nonescaping immutable whole-reference inputs',original.typeRange);
 let origin;if(context.phase==='preview')origin=functionOrigin(parser,parameters,result,path);else{
  const binding=readRustFnMutSourceCallbackBinding(parser,context.handle,context.callbackBinding);for(let at=0;at<parameters.length;at++){parser.origins.argument(binding.parameters[at],parameters[at],original.parameterRanges[at]);if(!testRustFnMutSourcePosition(parser,binding.parameters[at],parameters[at]))fail('the actual callback preserves its complete source argument owner, lifetime and type',original.parameterRanges[at]);}parser.origins.argument(binding.result,result,original.resultRange);if(!testRustFnMutSourcePosition(parser,binding.result,result))fail('the actual callback preserves its complete source result owner and type',original.resultRange);
  const captured=binding.captures.map(row=>({...structuredClone(row.outer.origin),wrappers:[],sourceLifetimes:[]})),packet={...parser.origins.typeOrigin(binding.node.ret,path),sourceOwner:{kind:'tuple',path:[],arguments:[result.sourceOwner,...captured.map(value=>value.expandedSourceOwner??value.sourceOwner)]}};origin=functionOrigin(parser,[...parameters,...captured],packet,path);
 }
 parser.origins.type=origin;return origin.type;
}
export function appendRustFnMutSourceStateParameters(parser,params,sourceParams){
 const context=parser.fnMutSourceSpecialization;if(!context||context.phase==='preview')return;
 if(params.length!==context.declaration.formalCount||!context.callbackConsumed)fail('the complete actual source formal inventory precedes hidden callback state');
 const binding=readRustFnMutSourceCallbackBinding(parser,context.handle,context.callbackBinding);context.stateParameters=[];
 for(const capture of binding.captures){if(!capture.outer.sourceBinder)fail('an actual mutable callback capture retains its original source binder token',binding.range);const name=parser.freshProductBinding(),origin={...structuredClone(capture.outer.origin),wrappers:['reference'],sourceLifetimes:[null],sourceExclusiveMode:'exclusive',sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[]};context.stateParameters.push({parameter:params.length,name,capture});params.push({name,type:readRustRecordSourceType(parser,origin.type,origin.path),span:binding.range});sourceParams.push(origin);}
}
export function rustFnMutSourceDeclarationTarget(parser,name,path){const context=parser.fnMutSourceSpecialization;return context&&context.phase!=='preview'?{name:context.generatedName,path:context.declaration.path}:{name,path};}
export function recordRustFnMutSourceFunctionHeader(parser,params,sourceParams,sourceResult){const context=parser.fnMutSourceSpecialization;if(context)context.resolvedHeader={params,sourceParams,sourceResult};}
export function beginRustFnMutSourceFunction(parser,name,path,params,sourceParams,result,mutableParams){
 const context=parser.fnMutSourceSpecialization;if(!context||context.phase==='preview')return null;initializeRustOwnedRecordEffects(parser);const frame=beginRustOwnedRecordStateBlock(parser,true),slots=[];
 for(let index=0;index<params.length;index++)if(sourceParams[index].sourceExclusiveMode==='exclusive'||mutableParams.some(row=>row.name===params[index].name))slots.push({parameter:index,slot:registerRustExclusiveSourceStateRoot(parser,params[index].name,sourceParams[index].sourceExclusiveMode==='exclusive'?'exclusive':'owned',params[index].span)});
 context.runtime={params,sourceParams,result,slots,callbackLexical:parser.origins.scope.get(context.declaration.callback.name)?.sourceLexicalBindingIdentity};return {frame,context,slots,path:[...path],name};
}
function positions(origins,result){return createRustSourceSignature('Rust source function signature 2',origins,result,new Set());}
export function finishRustFnMutSourceFunction(parser,frame,declaration){
 if(!frame)return declaration;const context=frame.context,runtime=context.runtime,binding=readRustFnMutSourceCallbackBinding(parser,context.handle,context.callbackBinding),exclusive=frame.slots.filter(row=>runtime.sourceParams[row.parameter].sourceExclusiveMode==='exclusive'),initial=new Map(frame.slots.map(row=>[row.slot.lexical,{slot:row.slot,name:declaration.params[row.parameter].name}])),body=lowerRustOwnedRecordEffectBody(parser,declaration.body,frame.path,new Map(readRustOwnedRecordStateDeclarations(parser)),initial,declaration.ret,(value,after)=>({k:'tuple',items:[value,...exclusive.map(row=>{const state=after.get(row.slot.lexical);if(!state||state.slot.root!==row.slot.root)fail('each source generic completion publishes its actual mutable environment',declaration.span);return {k:'name',path:[state.name]};})],span:value.span}));
 const mode=origin=>origin.sourceFnMutParameter?'callback':origin.sourceExclusiveMode==='exclusive'?'exclusive':origin.wrappers.length?'shared':'value',referent=origin=>qualified(parser,origin.sourceExclusiveMode==='exclusive'?{...origin,wrappers:[],sourceLifetimes:[]}:origin),expanded=positions(runtime.sourceParams.map(referent),qualified(parser,runtime.result)),callback=positions(context.callbackHeader.parameters.map(value=>qualified(parser,value)),qualified(parser,context.callbackHeader.result)),typeArguments=context.declaration.generics.map(row=>{const origin=qualified(parser,context.typeBindings.get(row.name)),position=positions([origin],origin).parameters[0];return {name:row.name,position};}),original=readRustFnMutSourceDeclaration(parser,context.handle),sourceCallback=original.callback;
 const facet={model:parser.fnMutSourceModel,language:'Rust',sourceVersion:parser.exclusiveSourceVersion,declaration:{path:[...original.path,original.name],range:original.range,bodyRange:original.bodyRange,generics:original.generics,formalCount:original.formalCount,callback:{index:sourceCallback.index,name:sourceCallback.name,mode:sourceCallback.mode,mutable:sourceCallback.mutable,binder:sourceCallback.binder,typeRange:sourceCallback.typeRange,parameterRanges:sourceCallback.parameterRanges,resultRange:sourceCallback.resultRange}},typeArguments,callback:{parameter:sourceCallback.index,parameters:callback.parameters,result:callback.result,captures:context.stateParameters.map(row=>({parameter:row.parameter,binder:{...row.capture.outer.sourceBinder},position:expanded.parameters[row.parameter]}))},parameters:expanded.parameters.map((position,index)=>({mode:mode(runtime.sourceParams[index]),position})),result:expanded.result,packet:exclusive.map(row=>row.parameter)};
 const output={...declaration,name:context.generatedName,generated:true,ret:{kind:'tuple',elements:[declaration.ret,...exclusive.map(row=>readRustRecordSourceType(parser,row.slot.origin.type,row.slot.origin.path))]},body,sourceFnMutSpecialization:facet};interfaces.set(output,{parameters:runtime.sourceParams.map(referent),result:qualified(parser,runtime.result)});context.bodyConsumed=true;return output;
}
export function endRustFnMutSourceFunction(parser,frame){if(frame)endRustOwnedRecordStateBlock(parser,frame.frame);}
export function finishRustFnMutSource(parser,program){
 const arena=registries.get(parser);if(!arena)return program;
 function prune(items,path=[]){const result=[];for(const item of items){if(item.k==='module')result.push({...item,items:prune(item.items,[...path,item.name])});else if(!item.sourceFnMutTemplate)result.push(item);}for(const row of arena.instances)if(row.path.join('.')===path.join('.'))result.push(row.declaration);return result;}
 if(!parser.origins.collecting&&!parser.headerDiscovery)for(const [name,row]of arena.templates)if(!arena.instances.some(instance=>instance.handle===row.handle))fail('an uninstantiated generic source declaration still needs complete actual body admission: '+name,row.facts.range);
 return {...program,items:[...prune(program.items),...(parser.fnMutSourceIteratorHelpers??[])]};
}
function selectedTemplate(parser,segments,path){
 const arena=registries.get(parser);if(!arena||segments.length===1&&parser.origins.scope.has(segments[0]))return null;const entry=parser.origins.lookupValueOwner(segments,path);return entry?.sourceFnMutTemplate?arena.templates.get(entry.fullName):null;
}
function withTemplateOwner(parser,row,read){
 const previous=parser.inherentOwner,owner=row.facts.path.length?parser.origins.lookupTypeOwner(['crate',...row.facts.path],[]):null;
 if(owner?.fullName===row.facts.path.join('.'))parser.inherentOwner={path:[...row.facts.path],modulePath:row.facts.path.slice(0,-1)};
 try{return read();}finally{parser.inherentOwner=previous;}
}
function concreteBindings(parser,row){
 const c=parser.cursor,bindings=new Map();if(c.eat('::')){c.expect('<','actual source type arguments');for(let at=0;at<row.facts.generics.length;at++){if(at)c.expect(',','source type argument');parser.type();bindings.set(row.facts.generics[at].name,structuredClone(parser.origins.type));}c.eat(',');c.expect('>','source type arguments');return bindings;}
 if(!row.facts.generics.length)return bindings;
 const expected=parser.standardSumExpectedOrigin,sum=expected?.standardSumSource,range=row.facts.resultRange,tokens=range?parser.sourceConstantTokens.filter(token=>token.start>=range.start&&token.end<=range.end):[];
 if(row.facts.generics.length===1&&sum?.owner==='std::result::Result'&&tokens.some(token=>token.value==='Result')&&tokens.at(-1)?.value==='>'&&tokens.at(-2)?.value===row.facts.generics[0].name){bindings.set(row.facts.generics[0].name,structuredClone(sum.arguments[1]));return bindings;}
 fail('concrete source type arguments need actual turbofish positions or the selected standard Result error context',row.facts.range);
}
function preview(parser,row,typeBindings){
 const key=[...row.facts.path,row.facts.name].join('.'),before=parser.origins.valueCatalog.get(key);
 try{return withTemplateOwner(parser,row,()=>previewRustFnMutSourceDeclaration(parser,row.handle,typeBindings,()=>parser.fn(row.facts.path)));}finally{if(before)parser.origins.registerCatalogEntry(key,before);}
}
function specialize(parser,row,typeBindings,binding){
 return withTemplateOwner(parser,row,()=>specializeRustFnMutSourceDeclaration(parser,row.handle,typeBindings,binding,(_parser,context)=>{context.phase='body';context.generatedName=parser.freshProductBinding();return parser.fn(row.facts.path);}));
}
function invocation(parser,row,typeBindings,header,sourceArgs,binding,path,span){
 const arena=registry(parser),compiled=specialize(parser,row,typeBindings,binding),actual=readRustFnMutSourceCallbackBinding(parser,row.handle,binding),interface_=interfaces.get(compiled),parameters=interface_.parameters,result={...eraseRustFnMutSourceOpacity(parser,interface_.result),sourceOwnedResultProof:true},callee=['crate',...row.facts.path,compiled.name],packetOrigin={...parser.origins.typeOrigin(compiled.ret,row.facts.path),fields:[result,...actual.captures.map(row=>row.outer.origin)],copyEvidence:false};
 parser.origins.registerCatalogEntry([...row.facts.path,compiled.name].join('.'),{k:'fn',parameters,result:packetOrigin});arena.instances.push({handle:row.handle,path:row.facts.path,declaration:compiled});
 const places=actual.captures.map(capture=>{const slot=readRustOwnedRecordStateSlots(parser).find(slot=>slot.lexical===capture.outer.lexical&&slot.root===capture.outer.root);if(!slot||slot.sourceBinder?.start!==capture.outer.sourceBinder?.start)fail('an actual FnMut capture must still be this caller lexical source root',span);return {slot,layouts:[],fields:[],field:slot.origin};}),held=acquireRustExclusiveSourceStateArguments(parser,parameters,places,sourceArgs.length,span);
 return recordRustExclusiveSourcePacketEffect(parser,callee,[...sourceArgs,...held.args],parameters,result,held.borrows,span);
}
function sourceArguments(parser,row,header,path,receiver=null){
 const c=parser.cursor,args=receiver?[receiver]:[],offset=receiver?1:0;if(row.facts.callback.index!==row.facts.formalCount-1)fail('this closed callback profile requires the actual callback as the final original source operand',row.facts.range);let binding=null;
 while(!c.is(')')){
  const index=args.length;if(index>=header.sourceParams.length)throw typeError('generic source call has too many actual operands',c.peek());const expected=header.sourceParams[index];
  if(index===row.facts.callback.index){if(row.facts.callback.mode==='exclusive'){c.expect('&','immediate mutable source callback');c.expect('mut','immediate mutable source callback');}binding=readRustFnMutSourceCallback(parser,row.handle,expected.callableParameters,expected.callableResult,path);args.push(readRustFnMutSourceCallbackBinding(parser,row.handle,binding).node);}
  else{const value=withRustStandardSumSourceContext(parser,expected,()=>readRustExpectedCopyClosure(parser,path,expected));parser.origins.argument(parser.origins.origin(value,path),expected,value.span);parser.origins.prepareOwnedArgument(value,expected,path);args.push(value);}
  if(!c.eat(','))break;
 }
 if(args.length!==header.sourceParams.length||!binding||offset>row.facts.callback.index)throw typeError('generic source call needs its actual complete original operands and callback',c.peek());return {args,binding};
}
export function readRustFnMutSourceCall(parser,segments,path,start){
 const row=selectedTemplate(parser,segments,path);if(!row)return null;const typeBindings=concreteBindings(parser,row),header=preview(parser,row,typeBindings);parser.cursor.expect('(','generic source call');const operands=sourceArguments(parser,row,header,path),close=parser.cursor.expect(')','generic source call');return invocation(parser,row,typeBindings,header,operands.args,operands.binding,path,{start:start.start,end:close.end});
}
export function readRustFnMutSourceMethod(parser,receiver,method,path,start){
 if(!parser.fnMutSourceModel)return null;const origin=parser.origins.origin(receiver,path),owner=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null;if(!owner)return null;
 const row=selectedTemplate(parser,['crate',...owner.fullName.split('.'),method.value],path);if(!row)return null;const typeBindings=concreteBindings(parser,row),header=preview(parser,row,typeBindings);if(header.params[0]?.name!=='self'||header.sourceParams[0].wrappers.join(',')!=='reference')fail('generic FnMut methods require their actual immutable whole receiver profile',method);
 captureRustInherentReceiverValueLoan(parser,receiver,method,path);const actual=parser.origins.origin(receiver,path),selected=parser.inherentMethods.get([...owner.fullName.split('.'),method.value].join('.')),plan=planRustInherentReceiverLayers(actual.wrappers,selected?.receiver??[]);if(plan[0]!==1)fail('the source generic method retains its actual immutable receiver layers',method);const argument={...receiver};parser.origins.mark(argument,prepareRustBorrowedInherentReceiverPosition(parser.origins,actual,plan.slice(1).map(layer=>layer===1?'reference':'box'),method));parser.origins.argument(parser.origins.origin(argument,path),header.sourceParams[0],method);
 parser.cursor.expect('(','generic source method');const operands=sourceArguments(parser,row,header,path,argument),close=parser.cursor.expect(')','generic source method');return invocation(parser,row,typeBindings,header,operands.args,operands.binding,path,{start:start.start,end:close.end});
}
export function readRustFnMutSourceLocalCall(parser,fn,path,start){
 const context=parser.fnMutSourceSpecialization,runtime=context?.runtime,origin=fn.path?.length===1?parser.origins.scope.get(fn.path[0]):null;if(!origin?.sourceFnMutParameter)return null;if(!runtime||origin.sourceLexicalBindingIdentity!==runtime.callbackLexical||!parser.cursor.is('('))fail('a source FnMut formal cannot escape or expose its internal ABI through an alias or bare value',start);
 const c=parser.cursor,args=[],parameters=context.callbackHeader.parameters;c.expect('(','source callback invocation');while(!c.is(')')){const expected=parameters[args.length];if(!expected)throw typeError('source callback has too many actual operands',c.peek());const value=withRustStandardSumSourceContext(parser,expected,()=>readRustExpectedCopyClosure(parser,path,expected));parser.origins.argument(parser.origins.origin(value,path),expected,value.span);parser.origins.prepareOwnedArgument(value,expected,path);args.push(value);if(!c.eat(','))break;}const close=c.expect(')','source callback invocation');if(args.length!==parameters.length)throw typeError('source callback requires its actual complete operands',close);
 const places=context.stateParameters.map(row=>{const place=readRustOwnedRecordStatePlace(parser,row.name,[],start);if(!place)fail('the current source callback state parameter must retain its actual lexical slot',start);return place;}),expanded=[...parameters,...places.map(place=>place.field)],held=acquireRustExclusiveSourceStateArguments(parser,expanded,places,args.length,start),result={...structuredClone(context.callbackHeader.result),sourceOwnedResultProof:true};return recordRustExclusiveSourceLocalPacketEffect(parser,fn.path[0],runtime.callbackLexical,[...args,...held.args],expanded,result,held.borrows,{start:start.start,end:close.end});
}
