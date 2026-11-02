import {readRustStandardCollectionSourceEqualityMethod} from './rust-standard-collection-equality-source.js';
// Real source bodies call the supplied checked kernel and transport result/state
// packets. No represented collection operation runs while parsing source.
import {unsupported,typeError} from './diagnostics.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {sameType,BOOL,UNIT,INT,rustFixedType} from './types.js';
import {readRustStandardCollectionSourceOrigin,readRustStandardCollectionSourceScalarDomain} from './rust-standard-collection-types.js';
import {resolveRustStandardCollectionSourceOwner,readRustStandardCollectionSourceStorage} from './rust-standard-collection-source.js';
import {readRustStandardCollectionMethodArity,readRustStandardCollectionMethodResult,readRustStandardCollectionCallOriginError,readRustStandardCollectionArgumentOriginError,testRustStandardCollectionMutation,readRustStandardCollectionKind} from './rust-standard-collection-rules.js';
import {projectRustStandardCollectionLibraryType,readRustStandardCollectionLibraryOrigin,encodeRustStandardCollectionSourceConstant,constructRustStandardCollectionSourceVariant,matchRustStandardCollectionSourceVariant,constructRustStandardCollectionSourceCarrier as wrapRustStandardCollectionSourceState,projectRustStandardCollectionSourceCarrier as withRustStandardCollectionSourceState} from './rust-standard-collection-signatures.js';
import {createRustStandardSumSourceOrigin,withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {testRustOwnedRecordStatePlaceLoans} from './rust-owned-record-state.js';
import {recordRustExclusiveSourcePacketEffect,readRustExclusiveSourceNode,selectRustExclusiveSourcePlace} from './rust-exclusive-source-calls.js';
import {testRustExclusiveSourcePlaceOverlap} from './rust-exclusive-source-loan-rules.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source call',reason,span);};
const copy=value=>structuredClone(value),name=value=>({k:'name',path:[value]});
function sourceFunction(parser,expected,arity,span){const library=parser.standardCollectionSource.library,entry=library.program.declarations.get(expected);if(entry?.k!=='fn'||!entry.body||entry.params.length!==arity)fail('the complete actual projected library callable is unavailable: '+expected,span);return {entry,path:['crate',...expected.split('.')]};}
function sourceHandlers(parser,span){const supplied=parser.standardCollectionSource.library.sourceHandlers;return {invalid:(message='collection source protocol mismatch')=>supplied.invalid({k:'str',value:message,span}),resource:(state,reason)=>supplied.resource(state,reason)};}
function exactArgumentOrigin(parser,node,expectedDomain,borrowed,path,span,expected){
 let actual=parser.origins.origin(node,path);
 // Rust infers unsuffixed integer literals from this actual protected method
 // position. No unresolved variable or independently typed operand is recast.
 const literal=node.k==='num'&&!node.type?node:node.k==='unary'&&node.op==='neg'&&node.arg?.k==='num'&&!node.arg.type?node.arg:null;
 if(literal&&expected?.type.kind==='fixed'&&expectedDomain!=='String'&&expectedDomain!=='bool'){
  literal.type=copy(expected.type);const inferred={...actual,type:copy(expected.type),sourceOwner:copy(expected.sourceOwner),proven:true,copyEvidence:true};parser.origins.mark(node,inferred);actual=inferred;
 }
 const transferred=parser.origins.ownedArgumentTransfers.has(node);if(!transferred)checkRustOwnedOrigin(parser.origins,actual,span);
 let owner=actual.expandedSourceOwner??actual.sourceOwner;
 if(!owner&&actual.type?.kind==='string')owner=readRustSourceTypeOwner(parser,[actual.textSourceModel==='string slice'?'str':'String'],[],actual.type);
 let domain=readRustStandardCollectionSourceScalarDomain({...actual,sourceOwner:owner,wrappers:[]},parser.standardCollectionSource.pointerWidth);
 if(expectedDomain==='String'&&borrowed&&actual.type?.kind==='string'&&actual.textSourceModel==='string slice'&&actual.wrappers.every(layer=>layer==='reference')){domain='str';owner={kind:'primitive',path:['str'],arguments:[]};}
 if(!owner&&node.k==='str'&&expectedDomain==='String'&&borrowed){domain='str';owner={kind:'primitive',path:['str'],arguments:[]};}
 const argument={domain,proven:actual.proven===true,available:true,borrowed:borrowed&&actual.wrappers.length>0,ownedTransfer:!borrowed&&domain==='String'&&(transferred||parser.origins.availableOwnedBinding(actual)||actual.sourceOwnedResultProof===true),sourceOwner:owner};
 const invalid=readRustStandardCollectionArgumentOriginError(argument,expectedDomain,borrowed);if(invalid)fail(invalid,span);return {actual,argument};
}
function encodeArgument(parser,node,domain,type,span){
 const library=parser.standardCollectionSource.library;
 if(domain==='String'){const helper=sourceFunction(parser,'encodeRustStandardCollectionText',1,span);return {k:'app',fn:{k:'name',path:helper.path},args:[node],span};}
 return constructRustStandardCollectionSourceVariant(library,type,domain==='bool'?'rustCollectionBoolean':'rustCollectionInteger',new Map([['value',domain==='bool'?node:{k:'cast',arg:node,to:INT,flavor:'exact',span}]]),span);
}
function decodeScalar(parser,expression,type,domain,resultOrigin,span){
 const library=parser.standardCollectionSource.library,handlers=sourceHandlers(parser,span),tag=domain==='String'?'rustCollectionString':domain==='bool'?'rustCollectionBoolean':'rustCollectionInteger';
 return matchRustStandardCollectionSourceVariant(parser,library,type,expression,tag,fields=>{
  if(domain==='String'){const helper=sourceFunction(parser,'decodeRustStandardCollectionText',1,span),units=fields.get('units');if(!units)fail('String decoding retains its actual UTF16 field',span);return {k:'app',fn:{k:'name',path:helper.path},args:[units.expression],span};}
  const value=fields.get('value');if(!value)fail('scalar decoding retains its actual field',span);return domain==='bool'?value.expression:{k:'cast',arg:value.expression,to:resultOrigin.type,flavor:'exact',span};
 },()=>handlers.invalid(),span);
}
function resultOrigin(parser,row,method,path,span,snapshotOrigin=null){
 const kind=readRustStandardCollectionMethodResult(row.origin.resolvedOwner,method);
 if(kind==='owned ordered values'){if(!snapshotOrigin)fail('ordered iterator collection retains its actual complete annotated Vec target',span);return snapshotOrigin;}
 if(kind==='owned collection')return {...parser.origins.typeOrigin(row.carrierType,path),copyEvidence:false,sourceOwner:copy(row.origin.sourceOwner),expandedSourceOwner:copy(row.origin.sourceOwner),sourceCollectionTypeIdentity:row.origin.typeIdentity,sourceStandardCollection:copy(row.origin),sourceOwnedResultProof:true};
 if(kind==='bool')return {...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 if(kind==='unit')return parser.origins.typeOrigin(UNIT,path);
 if(kind==='usize')return {...parser.origins.typeOrigin(rustFixedType('usize'),path),sourceOwner:{kind:'primitive',path:['usize'],arguments:[]}};
 const payload=copy(row.arguments[readRustStandardCollectionKind(row.origin.resolvedOwner)==='map'?1:0]);return {...createRustStandardSumSourceOrigin(parser,'std::option::Option',[payload],span),sourceOwnedResultProof:true};
}
function decodeResult(parser,row,method,expression,type,origin,span){
 const library=parser.standardCollectionSource.library,handlers=sourceHandlers(parser,span),kind=readRustStandardCollectionMethodResult(row.origin.resolvedOwner,method);
 if(kind==='owned collection')return matchRustStandardCollectionSourceVariant(parser,library,type,expression,'rustStandardCollectionState',fields=>wrapRustStandardCollectionSourceState(row,constructRustStandardCollectionSourceVariant(library,parser.standardCollectionSource.carrierCheckedType,'rustStandardCollectionState',new Map([...fields].map(([key,value])=>[key,value.expression])),span),span),()=>handlers.invalid(),span);
 if(kind==='owned ordered values')return matchRustStandardCollectionSourceVariant(parser,library,type,expression,'rustCollectionOrderedValues',fields=>{
  const values=fields.get('values');if(values?.type.kind!=='array')fail('ordered collection retains its actual complete scalar sequence',span);
  const accumulator=parser.freshProductBinding(),item=parser.freshProductBinding(),domain=method==='values'?row.valueDomain:row.keyDomain,payload=row.arguments[method==='values'?1:0],element=origin.type.element,decoded=decodeScalar(parser,{k:'name',path:[item]},values.type.element,domain,payload,span),body={k:'array',element,items:[{value:{k:'name',path:[accumulator]},spread:true},{value:decoded,spread:false}],span};
  return {k:'method',object:values.expression,op:'reduce',args:[{k:'lambda',params:[{name:accumulator,type:origin.type},{name:item,type:projectRustStandardCollectionLibraryType(values.type.element)}],ret:origin.type,captures:[],body,span},{k:'array',element,items:[],span}],span};
 },()=>handlers.invalid(),span);
 if(kind==='unit')return matchRustStandardCollectionSourceVariant(parser,library,type,expression,'rustCollectionUnit',()=>({k:'unit',span}),()=>handlers.invalid(),span);
 if(kind==='bool'||kind==='usize')return decodeScalar(parser,expression,type,kind==='bool'?'bool':'usize',origin,span);
 const sum=origin.standardSumSource,domain=readRustStandardCollectionKind(row.origin.resolvedOwner)==='map'?row.valueDomain:row.keyDomain,payload=row.arguments[readRustStandardCollectionKind(row.origin.resolvedOwner)==='map'?1:0];
 if(!sum?.constructorPaths?.Some||!sum.constructorPaths.None)fail('the returned Option needs its actual complete standard constructor inventory',span);
 return matchRustStandardCollectionSourceVariant(parser,library,type,expression,'rustCollectionOption',fields=>{
  const present=fields.get('present'),value=fields.get('value');if(!present||value?.type.kind!=='option')fail('Option decoding retains actual presence and optional payload fields',span);
  const selected={k:'coalesce',left:value.expression,right:handlers.invalid('present collection Option lost its actual payload'),span},decoded=decodeScalar(parser,selected,value.type.element,domain,payload,span);
  return {k:'if',cond:present.expression,then:{k:'app',fn:{k:'name',path:sum.constructorPaths.Some},args:[decoded],span},else:{k:'name',path:sum.constructorPaths.None,span},span};
 },()=>handlers.invalid(),span);
}
function rejectOverlappingArgumentEffects(parser,args,place,span){
 const pending=[...args];let count=0;while(pending.length){const node=pending.pop();if(!node||typeof node!=='object')continue;if(++count>100000)fail('collection argument effects exceed their source inspection bound',span);const effect=readRustExclusiveSourceNode(parser,node),overlap=other=>other?.slot&&testRustExclusiveSourcePlaceOverlap(place.slot.lexical,place.fields,other.slot.lexical,other.fields??other.layouts?.map(row=>row.index)??[]);if(effect?.kind==='call'&&effect.info.borrows.some(overlap)||effect?.kind==='write'&&overlap(effect.info))fail('collection receiver reservation overlaps a later argument mutation of the same source place',span);for(const child of Object.values(node))if(child&&typeof child==='object')pending.push(child);}
}
function registerWrapper(parser,parameters,result,body,span){
 const context=parser.standardCollectionSource,helper=parser.freshProductBinding(),parameterNames=parameters.map(()=>parser.freshProductBinding()),packetType={kind:'tuple',elements:[result.type,parameters[0].type]},packetOrigin={...parser.origins.typeOrigin(packetType,[]),fields:[result,parameters[0]],copyEvidence:false},entry={k:'fn',parameters:parameters.map(copy),result:packetOrigin};
 parser.origins.registerCatalogEntry(helper,entry);context.privateNames.add(helper);context.wrappers.push({k:'fn',name:helper,params:parameterNames.map((name,index)=>({name,type:parameters[index].type})),ret:packetType,body:body(parameterNames.map(name)),span});return {path:['crate',helper],parameters:entry.parameters};
}
function replaceConstantField(library,node,type,field,value,span){
 if(type.kind==='record'){const entry=node.entries.find(row=>row.name===field);if(!entry)fail('actual call metadata field is unavailable',span);entry.value=value;return;}
 const entry=type.kind==='data'?library.program.declarations.get(type.name):null,constructor=entry?.ctors.find(row=>row.name===node.fn?.path.at(-1)),index=constructor?.fields.findIndex(row=>row.name===field)??-1;if(index<0)fail('actual call metadata constructor field is unavailable',span);node.args[index]=value;
}
export function readRustStandardCollectionSourceArguments(parser,receiver,method,path){
 const actual=parser.origins.origin(receiver,path),row=readRustStandardCollectionSourceOrigin(parser,actual);if(!row)return null;
 const equality=['eq','ne'].includes(method.value),arity=equality?1:readRustStandardCollectionMethodArity(row.origin.resolvedOwner,method.value);if(arity<0)fail('the actual standard collection method has no admitted source argument contract',method);
 const c=parser.cursor,args=[];c.expect('(','standard collection method');
 while(!c.is(')')){
  const index=args.length;if(index>=arity)throw typeError('collection method has too many actual arguments',method);
  const expected=row.arguments[index],argument=equality?parser.expr(path):withRustStandardSumSourceContext(parser,expected,()=>parser.expr(path));
  if(!equality)exactArgumentOrigin(parser,argument,index===0?row.keyDomain:row.valueDomain,method.value!=='insert',path,argument.span??method,expected);
  if(method.value==='insert')parser.origins.prepareOwnedArgument(argument,expected,path);
  args.push(argument);if(!c.eat(','))break;
 }
 c.expect(')','standard collection method');return args;
}
export function readRustStandardCollectionSourceConstructor(parser,segments,args,path,span){
 if(!parser.standardCollectionSource)return null;const method=segments.at(-1),receiver=segments.slice(0,-1),owner=resolveRustStandardCollectionSourceOwner(parser,receiver,path),trait=parser.resolve(receiver,path).join('::'),expected=parser.standardSumExpectedOrigin,row=readRustStandardCollectionSourceOrigin(parser,expected);
 if(!owner&&!['std::default::Default','core::default::Default'].includes(trait))return null;
 if(!owner&&(parser.origins.lookupTypeOwner(receiver,path)||parser.origins.lookupTypeOwner([receiver[0]],path)))fail('Default construction retains its actual unshadowed standard trait owner',span);
 if(!['new','default'].includes(method)||!row||owner&&owner!==row.origin.resolvedOwner||args.length)fail('collection constructors require an explicit matching closed type annotation and their actual zero-argument new/Default owner',span);
 const context=parser.standardCollectionSource,library=context.library,helper=sourceFunction(parser,'createRustStandardCollectionState',6,span),root=parser.freshProductBinding(),call={$:'rustStandardCollectionCallOrigin',owner:row.origin.resolvedOwner,typeIdentity:row.origin.typeIdentity,receiverRoot:root,method,dispatchOwner:method==='default'?(owner?'core::default::Default':trait):owner,proven:true,shadowed:false,mutable:false,available:false,overlappingLoan:false,arguments:[]},constants=[row.origin,row.keyDomain,row.valueDomain,row.pointerWidth,root,call],application={k:'app',fn:{k:'name',path:helper.path},args:constants.map((value,index)=>encodeRustStandardCollectionSourceConstant(library,value,helper.entry.params[index].type,span)),span},handlers=sourceHandlers(parser,span),body=matchRustStandardCollectionSourceVariant(parser,library,helper.entry.ret,application,'rustCollectionCreated',fields=>fields.get('state')?wrapRustStandardCollectionSourceState(row,fields.get('state').expression,span):handlers.invalid(),(tag,fields)=>tag==='rustCollectionResourceLimit'?handlers.resource(fields.get('state')?.expression,fields.get('reason')?.expression):handlers.invalid(),span),origin=resultOrigin(parser,row,'new',path,span);
 return parser.origins.mark(body,origin);
}
export function readRustStandardCollectionSourceMethod(parser,receiver,method,args,span,path=parser.modulePath){
 const context=parser.standardCollectionSource;if(!context)return null;const actual=parser.origins.origin(receiver,path),row=readRustStandardCollectionSourceOrigin(parser,actual);if(!row)return null;
 const equality=readRustStandardCollectionSourceEqualityMethod(parser,receiver,method,args,path,span);if(equality)return equality;
 const operation=method.value,arity=readRustStandardCollectionMethodArity(row.origin.resolvedOwner,operation);if(arity<0)fail('collection iteration custom hashers and unimplemented methods retain their explicit refusal',span);if(['new','default'].includes(operation)||args.length!==arity)throw typeError('collection method requires its actual complete argument list',span);
 let projection='',snapshotOrigin=null;if(['iter','keys','values'].includes(operation)){const selected=readRustStandardCollectionSourceSnapshotTarget(parser,row,operation,path,span);projection=selected.projection;snapshotOrigin=selected.origin;}
 if(operation==='get'){const c=parser.cursor;c.expect('.','immediate collection get projection');const selected=c.identifier('collection Option projection');if(!['copied','cloned'].includes(selected.value))fail('collection get references require immediate actual copied/cloned projection',selected);projection=selected.value;c.expect('(','collection Option projection');c.expect(')','collection Option projection');if(projection==='copied'&&(readRustStandardCollectionKind(row.origin.resolvedOwner)==='map'?row.valueDomain:row.keyDomain)==='String')fail('String collection get cannot acquire Copy semantics',span);}
 const mutation=testRustStandardCollectionMutation(row.origin.resolvedOwner,operation),place=selectRustExclusiveSourcePlace(parser,receiver,path),slot=place?.slot,exclusive=parser.exclusiveSourceModel==='Rust exclusive source loans 1'&&(actual.sourceExclusiveMode==='exclusive'||slot?.origin.sourceExclusiveMode==='exclusive'||slot?.origin.sourceExclusiveParameter===true);
 if(mutation&&(actual.wrappers.length||actual.sourceBorrowedPlace===true)&&!exclusive)fail('mutation through shared collection receivers cannot acquire exclusive authority',span);
 if(operation==='clone'&&actual.wrappers.length>1)fail('nested reference Clone dispatch needs its separate actual reference result interpretation',span);
 if(mutation&&(!place||!readRustStandardCollectionSourceStorage(parser,{...place.field,wrappers:[]})))fail('collection mutation requires its actual available mutable root or field carrier slot',span);
 if(place)rejectOverlappingArgumentEffects(parser,args,place,span);checkRustOwnedOrigin(parser.origins,actual,span);
 const overlappingLoan=mutation&&testRustOwnedRecordStatePlaceLoans(parser,place.slot,place.fields),facts=args.map((node,index)=>exactArgumentOrigin(parser,node,index===0?row.keyDomain:row.valueDomain,operation!=='insert',path,span,row.arguments[index])),root=slot?.root??actual.sourceBindingIdentity??parser.freshProductBinding(),call={$:'rustStandardCollectionCallOrigin',owner:row.origin.resolvedOwner,typeIdentity:row.origin.typeIdentity,receiverRoot:root,method:operation,dispatchOwner:operation==='clone'?'core::clone::Clone':row.origin.resolvedOwner,proven:true,shadowed:false,mutable:Boolean(place),available:true,overlappingLoan,arguments:facts.map(row=>row.argument),...(operation==='clone'?{outputRoot:parser.freshProductBinding()}:{})},invalid=readRustStandardCollectionCallOriginError(row.origin,root,operation,args.length,call);if(invalid)fail(invalid,span);
 if(operation==='insert')for(let index=0;index<args.length;index++)if(!parser.origins.ownedArgumentTransfers.has(args[index]))parser.origins.prepareOwnedArgument(args[index],row.arguments[index],path);
 const library=context.library,packet=sourceFunction(parser,'executeRustStandardCollectionValuePacket',5,span),origin=resultOrigin(parser,row,operation,path,span,snapshotOrigin),parameters=[{...copy(actual),wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null},...facts.map(row=>({...copy(row.actual),wrappers:[],sourceLifetimes:[]}))],handlers=sourceHandlers(parser,span),wrapper=registerWrapper(parser,parameters,origin,references=>withRustStandardCollectionSourceState(parser,row,references[0],kernelState=>matchRustStandardCollectionSourceVariant(parser,library,context.carrierCheckedType,kernelState,'rustStandardCollectionState',fields=>{
  const heldRoot=fields.get('root');if(!heldRoot)fail('actual collection state carrier needs its retained root field',span);
  const metadata=encodeRustStandardCollectionSourceConstant(library,call,packet.entry.params[3].type,span);replaceConstantField(library,metadata,packet.entry.params[3].type,'receiverRoot',heldRoot.expression,span);if(operation==='clone')replaceConstantField(library,metadata,packet.entry.params[3].type,'outputRoot',{k:'binary',op:'concat',left:heldRoot.expression,right:{k:'str',value:'/'+call.outputRoot},span},span);
  const scalarType=packet.entry.params[2].type.element,argumentList={k:'array',element:projectRustStandardCollectionLibraryType(scalarType),items:args.map((_,index)=>({value:encodeArgument(parser,references[index+1],index===0?row.keyDomain:row.valueDomain,scalarType,span),spread:false})),span},application={k:'app',fn:{k:'name',path:packet.path},args:[kernelState,{k:'str',value:operation,span},argumentList,metadata,{k:'str',value:projection,span}],span};
  return matchRustStandardCollectionSourceVariant(parser,library,packet.entry.ret,application,'rustCollectionPacketNormal',normal=>{const values=normal.get('values'),writeback=normal.get('writeback');if(values?.type.kind!=='tuple'||values.type.elements.length!==2||!writeback)fail('actual kernel packet retains result zero and state one',span);const result=decodeResult(parser,row,operation,{k:'tupleIndex',object:values.expression,index:0},values.type.elements[0],origin,span),state=wrapRustStandardCollectionSourceState(row,{k:'tupleIndex',object:values.expression,index:1},span),tuple={k:'tuple',items:[result,state],span};return {k:'if',cond:mutation?writeback.expression:{k:'unary',op:'not',arg:writeback.expression},then:tuple,else:handlers.invalid('kernel packet writeback does not match the actual source operation'),span};},(tag,fields)=>tag==='rustCollectionResourceLimit'?handlers.resource(fields.get('state')?.expression,fields.get('reason')?.expression):handlers.invalid(),span);
 },()=>handlers.invalid(),span),span),span);
 const operands=[receiver,...args];if(mutation)return recordRustExclusiveSourcePacketEffect(parser,wrapper.path,operands,wrapper.parameters,origin,[{index:0,...place}],span);
 const result={k:'tupleIndex',object:{k:'app',fn:{k:'name',path:wrapper.path},args:operands,span},index:0,span};return snapshotOrigin?parser.origins.markVectorConstructor(result,origin):parser.origins.mark(result,origin);
}

function readRustStandardCollectionSourceSnapshotTarget(parser,row,method,path,span){
 const c=parser.cursor;c.expect('.','immediate ordered iterator projection');const projection=c.identifier('ordered iterator projection');if(!['copied','cloned'].includes(projection.value))fail('borrowed ordered iterators require immediate copied/cloned then concrete Vec collection',projection);c.expect('(','ordered iterator projection');c.expect(')','ordered iterator projection');
 const index=method==='values'?1:0,domain=index===1?row.valueDomain:row.keyDomain,expected=row.arguments[index];if(projection.value==='copied'&&domain==='String')fail('String ordered iterator elements retain Clone rather than Copy',projection);
 c.expect('.','immediate ordered iterator collection');c.expect('collect','ordered iterator collection');c.expect('::','explicit ordered Vec target');c.expect('<','explicit ordered Vec target');const type=parser.type(),origin=copy(parser.origins.type),owner=origin.expandedSourceOwner??origin.sourceOwner;splitRustTypeClosingToken(parser);c.expect('>','explicit ordered Vec target');c.expect('(','ordered iterator collection');c.expect(')','ordered iterator collection');
 if(type.kind!=='array'||origin.wrappers.length||owner?.kind!=='library'||!['std::vec::Vec','alloc::vec::Vec'].includes(owner.path.join('::'))||owner.arguments.length!==1||!sameType(type.element,expected.type)||readRustStandardCollectionSourceScalarDomain({...expected,type:type.element,sourceOwner:owner.arguments[0],wrappers:[]},row.pointerWidth)!==domain)fail('ordered iterator collection retains its actual same-domain standard owned Vec target',span);
 return {projection:projection.value,origin:{...origin,sourceOwnedResultProof:true,arraySourceModel:'owned vector',arrayElementCopyEvidence:domain!=='String',freshArrayValue:true}};
}
