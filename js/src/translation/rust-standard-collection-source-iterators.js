// The physical array transports ordered items within the consuming loop.
// The source origin remains the actual standard collection iterator.
import {testRustStandardCollectionProjectedGraphEquality} from './rust-standard-collection-call-adapter.js';
import {unsupported} from './diagnostics.js';
import {readRustStandardCollectionSourceOrigin} from './rust-standard-collection-types.js';
import {readRustStandardCollectionKind,testRustStandardCollectionOrdered} from './rust-standard-collection-rules.js';
import {readRustStandardCollectionSourceStorage} from './rust-standard-collection-source.js';
import {projectRustStandardCollectionSourceCarrier,encodeRustStandardCollectionSourceConstant,matchRustStandardCollectionSourceVariant,projectRustStandardCollectionLibraryType} from './rust-standard-collection-signatures.js';
import {decodeRustStandardCollectionSourceScalar,replaceRustStandardCollectionSourceConstantField} from './rust-standard-collection-calls.js';
import {readRustBorrowedIteratorArgumentRegions} from './rust-borrowed-iterators.js';
import {selectRustExclusiveSourcePlace} from './rust-exclusive-source-calls.js';
import {registerRustMutableRecordLoanRoot,captureRustMutableRecordValueLoans,promoteRustMutableRecordValueLoans} from './rust-mutable-record-loans.js';
import {readRustOwnedRecordUltimateRoot} from './rust-owned-record-update-rules.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
const fail=(reason,span)=>{throw unsupported('Rust standard ordered iterator',reason,span);};
const views=new WeakMap();
export function readRustStandardCollectionSourceIterator(parser,node){const view=views.get(node);if(!view)return null;const actual=parser.origins.origin(node,view.path);if(actual.wrappers.length||actual.type?.kind!=='array'||actual.arraySourceModel!==null||!testRustStandardCollectionProjectedGraphEquality(actual.type,view.origin.type)||!testRustStandardCollectionProjectedGraphEquality(actual.sourceOwner,view.origin.sourceOwner))return null;if(parser.cursor.tokens.slice(view.headEnd,parser.cursor.index).some(token=>token.value!==')'))return null;return view;}
function entryFields(parser,library,type,expression,next,span){
 if(type.kind==='record')return next(new Map(type.fields.map(field=>[field.name,{type:field.type,expression:{k:'field',object:expression,field:field.name,span}}])));
 const entry=type.kind==='data'?library.program.declarations.get(type.name):null;if(entry?.ctors.length!==1)fail('the actual projected iterator entry retains its complete key/value inventory',span);return matchRustStandardCollectionSourceVariant(parser,library,type,expression,entry.ctors[0].name,next,()=>library.sourceHandlers.invalid(),span);
}
function borrowReceiver(parser,receiver,actual,path,span){
 checkRustOwnedOrigin(parser.origins,actual,span);const place=selectRustExclusiveSourcePlace(parser,receiver,path),identity=actual.sourcePlaceBindingIdentity??actual.sourceBindingIdentity??'',root=readRustOwnedRecordUltimateRoot(identity)||identity,owning=place?{...place.slot.origin,sourceBindingIdentity:place.slot.root,sourceLexicalBindingIdentity:place.slot.lexical,wrappers:[]}:Array.from(parser.origins.scope.values()).find(origin=>origin.sourceBindingIdentity===root&&origin.wrappers.length===0);
 if(!parser.origins.collecting&&owning)registerRustMutableRecordLoanRoot(parser.origins,owning,span);
 const regions=readRustBorrowedIteratorArgumentRegions(actual);if(!parser.origins.collecting&&regions.length===0)fail('a shared iterator needs actual live receiver-derived source regions',span);
 const loan=captureRustMutableRecordValueLoans(parser.origins,{...structuredClone(actual),wrappers:['reference'],sourceBorrowedPlace:true,copyEvidence:true,sourceValueBorrowBindings:regions},span);promoteRustMutableRecordValueLoans(parser.origins,loan);
 return loan;
}
function moveReceiver(parser,receiver,actual,path,span){
 const table=parser.origins;
 if(actual.wrappers.length||actual.sourceBorrowedPlace===true)fail('ordered IntoIterator requires a direct owned BTree receiver',span);
 checkRustOwnedOrigin(table,actual,span);
 const transferred=table.transferOwnedResult(receiver,path);
 if(!table.collecting&&(!table.availableOwnedBinding(transferred)||transferred.sourceOwnedResultProof!==true||!table.consumeOwnedBinding(transferred,span)))fail('ordered IntoIterator requires one actual available owned collection transfer',span);
}
export function createRustStandardCollectionSourceIterator(parser,receiver,method,args,path,span){
 const context=parser.standardCollectionSource,actual=parser.origins.origin(receiver,path),row=readRustStandardCollectionSourceOrigin(parser,actual);if(!row||!testRustStandardCollectionOrdered(row.origin.resolvedOwner)||!readRustStandardCollectionSourceStorage(parser,{...actual,wrappers:[]}))fail('an ordered iterator retains its actual registered BTree receiver and complete concrete domains',span);
 const kind=readRustStandardCollectionKind(row.origin.resolvedOwner),operation=method.value,owned=operation==='into_iter',pairs=kind==='map'&&(owned||operation==='iter');
 if(args.length!==0||!(owned||kind==='set'&&operation==='iter'||kind==='map'&&['iter','keys','values'].includes(operation)))fail('an ordered BTree iterator retains its actual standard zero-argument method',span);
 let loan=null;
 if(owned)moveReceiver(parser,receiver,actual,path,span);
 else loan=borrowReceiver(parser,receiver,actual,path,span);
 const regions=loan?.sourceValueBorrowBindings??[];
 const itemOrigin=origin=>({...structuredClone(origin),wrappers:owned?[]:['reference'],sourceLifetimes:owned?[]:[actual.sourceLifetimes?.[0]??null],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceBorrowedPlace:!owned,copyEvidence:!owned||origin.type.kind!=='string',sourceOwnedResultProof:owned&&origin.type.kind==='string',sourceValueBorrowBindings:[...regions],sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[]});
 const key=itemOrigin(row.arguments[0]),value=kind==='map'?itemOrigin(row.arguments[1]):null;
 const item=pairs?{...parser.origins.typeOrigin({kind:'tuple',elements:[key.type,value.type]},path),fields:[key,value],sourceOwner:{kind:'tuple',path:[],arguments:[key.sourceOwner,value.sourceOwner]},copyEvidence:key.copyEvidence&&value.copyEvidence,sourceValueBorrowBindings:[...regions]}:operation==='values'?value:key;
 const iterator=owned?'IntoIter':operation==='keys'?'Keys':operation==='values'?'Values':'Iter';
 const resultType={kind:'array',element:item.type},origin={...parser.origins.typeOrigin(resultType,path),arraySourceModel:null,arrayElementCopyEvidence:false,sourceOwner:{kind:'library',path:['std','collections',kind==='map'?'btree_map':'btree_set',iterator],arguments:row.origin.sourceOwner.arguments},sourceBorrowedPlace:!owned,copyEvidence:false,sourceValueBorrowBindings:[...regions]};
 const library=context.library,binding=library.functions.find(entry=>entry.role==='packet'),packet=library.program.declarations.get(binding.checkedName),helper=parser.freshProductBinding(),parameter=parser.freshProductBinding(),sourceParameter={...structuredClone(actual),wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null};
 const call={$:'rustStandardCollectionCallOrigin',owner:row.origin.resolvedOwner,typeIdentity:row.origin.typeIdentity,receiverRoot:'actual iterator receiver root',method:operation,dispatchOwner:owned?'core::iter::IntoIterator':row.origin.resolvedOwner,proven:true,shadowed:false,mutable:false,available:true,overlappingLoan:false,arguments:[],...(owned?{ownedReceiverTransfer:true}:{})};
 const body=projectRustStandardCollectionSourceCarrier(parser,row,{k:'name',path:[parameter]},state=>matchRustStandardCollectionSourceVariant(parser,library,context.carrierCheckedType,state,'rustStandardCollectionState',stateFields=>{
  const root=stateFields.get('root'),metadata=encodeRustStandardCollectionSourceConstant(library,call,packet.params[3].type,span);if(!root)fail('a shared iterator retains its actual runtime receiver root',span);replaceRustStandardCollectionSourceConstantField(library,metadata,packet.params[3].type,'receiverRoot',root.expression,span);
  const invocation={k:'app',fn:{k:'name',path:binding.sourcePath},args:[state,{k:'str',value:operation},{k:'array',element:projectRustStandardCollectionLibraryType(packet.params[2].type.element),items:[]},metadata,{k:'str',value:owned?'owned':'borrowed'}],span};
  return matchRustStandardCollectionSourceVariant(parser,library,packet.ret,invocation,'rustCollectionPacketNormal',normal=>{
   const values=normal.get('values'),writeback=normal.get('writeback');if(values?.type.kind!=='tuple'||!writeback)fail('the actual shared iterator packet retains its result and readonly state positions',span);
   const decoded=matchRustStandardCollectionSourceVariant(parser,library,values.type.elements[0],{k:'tupleIndex',object:values.expression,index:0},'rustCollectionOrderedEntries',fields=>{
    const entries=fields.get('entries');if(entries?.type.kind!=='array')fail('the shared BTree iterator retains its actual ordered entry storage',span);const accumulator=parser.freshProductBinding(),entry=parser.freshProductBinding(),decode=(field,domain,origin)=>{if(!field)fail('the shared iterator retains its selected actual entry field',span);const optional=field.type.kind==='option';return decodeRustStandardCollectionSourceScalar(parser,optional?{k:'coalesce',left:field.expression,right:library.sourceHandlers.invalid()}:field.expression,optional?field.type.element:field.type,domain,origin,span);},selected=entryFields(parser,library,entries.type.element,{k:'name',path:[entry]},entryFields=>pairs?{k:'tuple',items:[decode(entryFields.get('key'),row.keyDomain,key),decode(entryFields.get('value'),row.valueDomain,value)]}:operation==='values'?decode(entryFields.get('value'),row.valueDomain,value):decode(entryFields.get('key'),row.keyDomain,key),span),append={k:'array',element:item.type,items:[{value:{k:'name',path:[accumulator]},spread:true},{value:selected,spread:false}]};
    return {k:'method',object:entries.expression,op:'reduce',args:[{k:'lambda',params:[{name:accumulator,type:resultType},{name:entry,type:projectRustStandardCollectionLibraryType(entries.type.element)}],ret:resultType,captures:[],body:append},{k:'array',element:item.type,items:[]}],span};
   },()=>library.sourceHandlers.invalid(),span);
   return {k:'if',cond:{k:'unary',op:'not',arg:writeback.expression},then:decoded,else:library.sourceHandlers.invalid(),span};
  },(tag,fields)=>tag==='rustCollectionResourceLimit'?library.sourceHandlers.resource(fields.get('state')?.expression,fields.get('reason')?.expression):library.sourceHandlers.invalid(fields.get('reason')?.expression),span);
 },()=>library.sourceHandlers.invalid(),span),span);
 parser.origins.registerCatalogEntry(helper,{k:'fn',parameters:[sourceParameter],result:origin});context.privateNames.add(helper);context.wrappers.push({k:'fn',name:helper,params:[{name:parameter,type:sourceParameter.type}],ret:resultType,body,span});
 const node=parser.origins.mark({k:'app',fn:{k:'name',path:['crate',helper]},args:[receiver],span},origin),view={node,item,row,actual,loan,owned,origin,path:[...path],operation,headEnd:parser.cursor.index};views.set(node,view);return node;
}
