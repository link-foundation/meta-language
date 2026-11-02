import {unsupported} from './diagnostics.js';
import {readRustSourceTypeHead,readRustSourceTypeOwner} from './rust-source-signatures.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {resolveRustStandardCollectionSourceOwner} from './rust-standard-collection-source.js';
import {readRustStandardCollectionKind,readRustStandardCollectionSourceOriginError,readRustStandardCollectionIntegerWidth} from './rust-standard-collection-rules.js';
import {readRustStandardCollectionLibraryOrigin} from './rust-standard-collection-signatures.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source type',reason,span);};
function retainRustStandardCollectionNominalCarrier(parser,row,span){
 const context=parser.standardCollectionSource,cached=context.types.get(row.origin.typeIdentity);let carrierName=cached?.carrierName;
 if(!carrierName){
  if(context.types.size>=256)fail('concrete collection source carriers exceed their finite specialization budget',span);
  const base='RustStandardCollection'+(row.origin.resolvedOwner.endsWith('HashMap')?'HashMap':'HashSet')+'_'+row.keyDomain+'_'+row.valueDomain+'_'+row.pointerWidth;
  for(let index=0;index<=100000;index++){const candidate=index===0?base:base+'_'+index,existing=parser.origins.catalog.get(candidate);if(existing?.sourceStandardCollectionCarrier?.origin?.typeIdentity===row.origin.typeIdentity||!existing&&!parser.reservedNames.has(candidate)){carrierName=candidate;break;}}
  if(!carrierName)fail('collection specialization exhausted its actual fresh source name supply',span);
 }
 const packet=context.library.functions.find(entry=>entry.role==='packet'),constructor='CollectionState',carrierType={kind:'named',path:['crate',carrierName]},facet={model:context.model,origin:structuredClone(row.origin),keyDomain:row.keyDomain,valueDomain:row.valueDomain,pointerWidth:row.pointerWidth,concreteIdentity:row.origin.typeIdentity,kernelType:structuredClone(context.carrierType),kernelLibrary:{entryPath:context.library.entryPath,sourceSha256:context.library.closure.sourceSha256,kernelDeclaration:context.carrierCheckedType.name,packetDeclaration:packet.checkedName}},declaration={k:'data',name:carrierName,ctors:[{name:constructor,fields:[{name:'state',type:structuredClone(context.carrierType)}]}],sourceStandardCollectionCarrier:facet,span};
 const before=parser.origins.catalog.get(carrierName);if(before&&!before.sourceStandardCollectionCarrier)fail('a collection carrier cannot replace an actual source declaration',span);
 parser.origins.registerCatalogEntry(carrierName,{k:'data',unitFields:false,sourceStandardCollectionCarrier:facet});
 const result={...parser.origins.typeOrigin(carrierType,[]),copyEvidence:false,proven:true};parser.origins.registerCatalogEntry(carrierName+'.'+constructor,{k:'ctor',owner:carrierName,fields:[readRustStandardCollectionLibraryOrigin(parser,context.carrierCheckedType,[])],result,sourceStandardCollectionCarrier:facet});
 parser.reservedNames.add(carrierName);context.privateNames.add(carrierName);context.privateNames.add(carrierName+'.'+constructor);
 return {...row,carrierName,carrierType,carrierConstructor:['crate',carrierName,constructor],carrierDeclaration:declaration,carrierFacet:facet};
}
export function readRustStandardCollectionSourceScalarDomain(origin,pointerWidth){
 if(!origin?.proven||origin.wrappers.length||!origin.sourceOwner||origin.sourceOwner.arguments.length)return '';
 const owner=origin.sourceOwner,path=owner.path.join('::');if(origin.type?.kind==='string'&&owner.kind==='library'&&['std::string::String','alloc::string::String'].includes(path)&&origin.textSourceModel==='owned string')return 'String';
 const domain=owner.kind==='primitive'?owner.path.at(-1):'';
 if(domain==='bool'&&origin.type?.kind==='bool')return domain;
 const width=readRustStandardCollectionIntegerWidth(domain,pointerWidth);return width&&origin.type?.kind==='fixed'&&origin.type.bits===width&&origin.type.signed===(domain[0]==='i')?domain:'';
}
export function readRustStandardCollectionSourceType(parser,segments,resolved,path,start){
 const owner=resolveRustStandardCollectionSourceOwner(parser,segments,path);if(!owner)return null;
 const context=parser.standardCollectionSource,c=parser.cursor,kind=readRustStandardCollectionKind(owner),arguments_=[],domains=[];
 c.expect('<','concrete standard collection type');for(let index=0;index<(kind==='map'?2:1);index++){
  if(index)c.expect(',','standard collection type argument');const head=readRustSourceTypeHead(c),type=parser.type(),argument=structuredClone(parser.origins.type);argument.sourceOwner??=readRustSourceTypeOwner(parser,head,[],type);
  const domain=readRustStandardCollectionSourceScalarDomain(argument,context.pointerWidth);if(!domain)fail('each key/value requires its actual closed direct String Boolean or integer source owner',start);
  arguments_.push(argument);domains.push(domain);
 }
 c.eat(',');splitRustTypeClosingToken(parser);c.expect('>','standard collection type');
 const scalarOwners=domains.map(domain=>({kind:domain==='String'?'library':'primitive',path:domain==='String'?['std','string','String']:[domain],arguments:[]})),scalarFacts=domains.map((domain,index)=>({proven:true,wrappers:[],sourceOwner:scalarOwners[index],...(domain==='String'?{textSourceModel:'owned string'}:{})})),sourceOwner={kind:'library',path:owner.split('::'),arguments:scalarOwners},identity=owner+'<'+domains.join(',')+'>@'+context.pointerWidth,origin={$:'rustStandardCollectionOrigin',ordinary:true,proven:true,shadowed:false,typeIdentity:identity,resolvedOwner:owner,sourceOwner,arguments:scalarFacts,sourceVersionMinor:parser.sourceVersionMinor,pointerWidth:context.pointerWidth,customHasher:false,customEquality:false},row={origin,keyDomain:domains[0],valueDomain:kind==='map'?domains[1]:'unit',pointerWidth:context.pointerWidth,arguments:arguments_};
 const invalid=readRustStandardCollectionSourceOriginError(origin,row.keyDomain,row.valueDomain,row.pointerWidth);if(invalid)fail(invalid,start);
 const retained=retainRustStandardCollectionNominalCarrier(parser,row,start);context.types.set(identity,retained);
 const type=structuredClone(retained.carrierType),result={...parser.origins.typeOrigin(type,path),proven:true,copyEvidence:false,sourceOwner,expandedSourceOwner:structuredClone(sourceOwner),sourceCollectionTypeIdentity:identity,sourceStandardCollection:origin,sourceOwnedResultProof:false};parser.origins.type=result;return type;
}
export function readRustStandardCollectionSourceOrigin(parser,origin){
 const context=parser.standardCollectionSource,row=context?.types.get(origin?.sourceCollectionTypeIdentity);if(!row||origin.type?.kind!=='named'||!origin.proven)return null;
 if(origin.wrappers.some(layer=>layer!=='reference'))return null;
 if(origin.sourceStandardCollection?.typeIdentity!==row.origin.typeIdentity)return null;
 return row;
}
