// Opt-in source linkage. The supplied complete library is regenerated and
// checked before any import/type/method can obtain standard authority.
import {unsupported} from './diagnostics.js';
import {readRustStandardCollectionProjectedLibraryError,testRustStandardCollectionProjectedGraphEquality} from './rust-standard-collection-call-adapter.js';
import {resolveRustStandardCollectionOwner,readRustStandardCollectionSourceOriginError,readRustStandardCollectionSourceVersionError} from './rust-standard-collection-rules.js';
import {readRustStandardCollectionLibraryOrigin,projectRustStandardCollectionLibraryType} from './rust-standard-collection-signatures.js';
export const rustStandardCollectionSourceModel='Rust closed standard collections 1';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source',reason,span);};
export function initializeRustStandardCollectionSource(parser,model='',library=null,pointerWidth=64){
 parser.standardCollectionSourceModel=model;
 if(model===''){parser.standardCollectionSource=null;parser.standardCollectionSourceStorage=null;parser.standardCollectionSourcePrivateNameGuard=null;parser.standardCollectionSourceObservationGuard=null;parser.origins.standardCollectionSourceModel='';parser.origins.standardCollectionSourceStorage=null;return;}
 if(model!==rustStandardCollectionSourceModel)fail('select the exact closed standard collection source profile');
 if(pointerWidth!==64)fail('the current Rust source frontend primitive usize/isize interface covers only explicit 64-bit targets; 32-bit kernel domains remain independently available');
 const version=readRustStandardCollectionSourceVersionError(parser.sourceVersionMinor);if(version)fail(version);
 const error=readRustStandardCollectionProjectedLibraryError(library);if(error)fail(error);
 if(typeof library.sourceHandlers?.invalid!=='function'||typeof library.sourceHandlers?.resource!=='function')fail('actual projected source interfaces require distinct protocol and resource continuations');
 // Function continuations are retained separately; every supplied graph is a
 // private per-pass clone before ordinary parser/checker code can annotate it.
 const sourceHandlers=library.sourceHandlers,local={...library,closure:structuredClone(library.closure),program:structuredClone(library.program),sourceProgram:structuredClone(library.sourceProgram),functions:structuredClone(library.functions),packetType:structuredClone(library.packetType),sourceHandlers},context={model,library:local,pointerWidth,types:new Map(),wrappers:[],privateNames:new Set(local.program.declarations.keys())};library=local;parser.standardCollectionSource=context;
 const register=(name,entry)=>{
  const before=parser.origins.catalog.get(name);
  if(before&&(!before.sourceCollectionLibraryDeclaration||!testRustStandardCollectionProjectedGraphEquality(before.sourceCollectionLibraryDeclaration,entry.sourceCollectionLibraryDeclaration)))fail('a projected collection library declaration collides with an actual user declaration');
  parser.origins.registerCatalogEntry(name,entry);context.privateNames.add(name);for(const part of name.split('.'))parser.reservedNames.add(part);
 };
 for(const [name,entry] of library.program.declarations){if(entry.k!=='data')continue;register(name,{k:'data',sourceCollectionLibraryDeclaration:entry,unitFields:entry.ctors.every(row=>row.fields.length===0)});}
 for(const [name,entry] of library.program.declarations){
  if(entry.k==='data')for(const constructor of entry.ctors){const result=readRustStandardCollectionLibraryOrigin(parser,{kind:'data',name},[]);register(name+'.'+constructor.name,{k:'ctor',owner:name,fields:constructor.fields.map(field=>readRustStandardCollectionLibraryOrigin(parser,field.type,[])),result,sourceCollectionLibraryDeclaration:entry});}
  else if(entry.k==='fn')register(name,{k:'fn',parameters:entry.params.map(parameter=>readRustStandardCollectionLibraryOrigin(parser,parameter.type,[])),result:readRustStandardCollectionLibraryOrigin(parser,entry.ret,[]),sourceCollectionLibraryDeclaration:entry});
  else fail('a collection library needs actual complete function and datatype declarations only');
 }
 const packet=library.functions.find(row=>row.role==='packet'),state=library.program.declarations.get(packet.checkedName).params[0].type;
 if(state.kind!=='data'||!library.program.declarations.has(state.name))fail('the actual projected kernel receiver must retain its complete named state carrier');
 context.carrierType=projectRustStandardCollectionLibraryType(state);context.carrierCheckedType=state;
 parser.standardCollectionSourceStorage=origin=>readRustStandardCollectionSourceStorage(parser,origin);
 parser.standardCollectionSourcePrivateNameGuard=(segments,path,span)=>refuseRustStandardCollectionSourcePrivateName(parser,segments,path,span);
 parser.standardCollectionSourceObservationGuard=(node,path,operation,span)=>refuseRustStandardCollectionSourceObservation(parser,node,path,operation,span);
 parser.origins.standardCollectionSourceModel=model;parser.origins.standardCollectionSourceStorage=parser.standardCollectionSourceStorage;
}
export function resolveRustStandardCollectionSourceOwner(parser,segments,path){
 if(!parser.standardCollectionSource||!parser.ordinaryCompletionContext())return '';
 const resolved=parser.resolve(segments,path),owner=resolveRustStandardCollectionOwner(resolved.join('::'),true,false);if(!owner)return '';
 for(const names of [segments,resolved])if(parser.origins.lookupTypeOwner(names,path)||parser.origins.lookupTypeOwner([names[0]],path)||parser.origins.lookupValueOwner([names[0]],path)||parser.origins.scope.has(names[0])||parser.sourceTypeAliases?.has([...path,names[0]].join('.')))return '';
 return owner;
}
export function readRustStandardCollectionSourceImport(parser,segments,path){return Boolean(parser.standardCollectionSource&&segments[0]==='std'&&resolveRustStandardCollectionSourceOwner(parser,segments,path));}
export function refuseRustStandardCollectionSourcePrivateName(parser,segments,path,span){
 const context=parser.standardCollectionSource;if(!context)return;const resolved=parser.resolve(segments,path),actual=resolved[0]==='crate'?resolved.slice(1):resolved;
 if(actual.some((_,index)=>context.privateNames.has(actual.slice(0,index+1).join('.')))||segments.some((_,index)=>context.privateNames.has(segments.slice(0,index+1).join('.'))))fail('projected collection implementation names are private and cannot become original source declarations or calls',span);
}
export function refuseRustStandardCollectionSourceObservation(parser,node,path,operation,span){
 if(!parser.standardCollectionSource)return;const origin=parser.origins.origin(node,path);
 if(origin&&readRustStandardCollectionSourceStorage(parser,{...origin,wrappers:[]})===true)fail('collection '+operation+' requires its actual separate source interpretation; the internal carrier is not an observable Rust representation',span);
}
export function readRustStandardCollectionSourceStorage(parser,origin){
 const context=parser.standardCollectionSource,row=context?.types.get(origin?.sourceCollectionTypeIdentity);if(!row||!origin.proven||origin.wrappers.length!==0||origin.type?.kind!=='named')return false;
 if(!testRustStandardCollectionProjectedGraphEquality(origin.type,context.carrierType)||!testRustStandardCollectionProjectedGraphEquality(origin.sourceOwner,row.origin.sourceOwner)||!testRustStandardCollectionProjectedGraphEquality(origin.sourceStandardCollection,row.origin))return false;
 const owner=parser.origins.lookupTypeOwner(origin.type.path,origin.path),actual=context.library.program.declarations.get(context.carrierCheckedType.name);
 return owner?.k==='data'&&testRustStandardCollectionProjectedGraphEquality(owner.sourceCollectionLibraryDeclaration,actual)&&readRustStandardCollectionSourceOriginError(row.origin,row.keyDomain,row.valueDomain,row.pointerWidth)==='';
}
export function finishRustStandardCollectionSource(parser,program){
 const context=parser.standardCollectionSource;if(!context)return program;
 const libraryNames=context.privateNames,collision=[];function inspect(items,path=[]){for(const item of items){if(item.k==='module')inspect(item.items,[...path,item.name]);else if(libraryNames.has([...path,item.name].join('.')))collision.push(item.name);}}inspect(program.items);
 if(collision.length)fail('user declarations cannot replace projected collection library bodies: '+collision.join(', '));
 return {...program,items:[...structuredClone(context.library.sourceProgram.items),...program.items,...context.wrappers]};
}
