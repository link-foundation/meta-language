import {unsupported} from './diagnostics.js';
import {readRustSourceTypeHead,readRustSourceTypeOwner} from './rust-source-signatures.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {resolveRustStandardCollectionSourceOwner} from './rust-standard-collection-source.js';
import {readRustStandardCollectionKind,readRustStandardCollectionSourceOriginError,readRustStandardCollectionIntegerWidth} from './rust-standard-collection-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source type',reason,span);};
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
 context.types.set(identity,row);
 const type=structuredClone(context.carrierType),result={...parser.origins.typeOrigin(type,path),proven:true,copyEvidence:false,sourceOwner,expandedSourceOwner:structuredClone(sourceOwner),sourceCollectionTypeIdentity:identity,sourceStandardCollection:origin,sourceOwnedResultProof:false};parser.origins.type=result;return type;
}
export function readRustStandardCollectionSourceOrigin(parser,origin){
 const context=parser.standardCollectionSource,row=context?.types.get(origin?.sourceCollectionTypeIdentity);if(!row||origin.type?.kind!=='named'||!origin.proven)return null;
 if(origin.wrappers.some(layer=>layer!=='reference'))return null;
 if(origin.sourceStandardCollection?.typeIdentity!==row.origin.typeIdentity)return null;
 return row;
}
