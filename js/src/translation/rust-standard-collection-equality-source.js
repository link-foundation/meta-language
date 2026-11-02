// Closed standard PartialEq source lowering invokes the real projected shared
// equality body. No structural comparison of nominal/kernel carrier fields.
import {unsupported,typeError} from './diagnostics.js';
import {BOOL} from './types.js';
import {readRustStandardCollectionSourceOrigin} from './rust-standard-collection-types.js';
import {readRustStandardCollectionSourceStorage} from './rust-standard-collection-source.js';
import {encodeRustStandardCollectionSourceConstant,matchRustStandardCollectionSourceVariant,projectRustStandardCollectionSourceCarrier} from './rust-standard-collection-signatures.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {selectRustExclusiveSourcePlace,readRustExclusiveSourceNode} from './rust-exclusive-source-calls.js';
import {testRustExclusiveSourcePlaceOverlap} from './rust-exclusive-source-loan-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection equality source',reason,span);};
function rejectRustStandardCollectionEqualityEffects(parser,left,right,path,span){
 const selected=selectRustExclusiveSourcePlace(parser,left,path);if(!selected)return;const pending=[right],seen=new Set();let work=0;
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;if(++work>100000)fail('equality operand effects exceed their actual source inspection bound',span);seen.add(node);
  const effect=readRustExclusiveSourceNode(parser,node),overlap=place=>place?.slot&&testRustExclusiveSourcePlaceOverlap(selected.slot.lexical,selected.fields,place.slot.lexical,place.fields??place.layouts?.map(row=>row.index)??[]);
  if(effect?.kind==='call'&&effect.info.borrows.some(overlap)||effect?.kind==='write'&&overlap(effect.info))fail('a later equality operand cannot mutate the already borrowed left collection place',span);
  for(const value of Object.values(node))if(value&&typeof value==='object')pending.push(value);
 }
}
export function readRustStandardCollectionSourceComparison(parser,left,right,operator,path,span,borrowedCall=false){
 const context=parser.standardCollectionSource;if(!context||!['eq','ne','==','!='].includes(operator))return null;
 const leftOrigin=parser.origins.origin(left,path),rightOrigin=parser.origins.origin(right,path),first=readRustStandardCollectionSourceOrigin(parser,leftOrigin),second=readRustStandardCollectionSourceOrigin(parser,rightOrigin);if(!first&&!second)return null;
 if(!first||!second||first.origin.typeIdentity!==second.origin.typeIdentity||!readRustStandardCollectionSourceStorage(parser,{...leftOrigin,wrappers:[]})||!readRustStandardCollectionSourceStorage(parser,{...rightOrigin,wrappers:[]}))throw typeError('standard collection equality requires the same actual concrete owner key/value and target domains',span);
 if(!borrowedCall&&leftOrigin.wrappers.length!==rightOrigin.wrappers.length)throw typeError('standard collection operator equality preserves the same actual reference nesting on both operands',span);
 checkRustOwnedOrigin(parser.origins,leftOrigin,span);checkRustOwnedOrigin(parser.origins,rightOrigin,span);rejectRustStandardCollectionEqualityEffects(parser,left,right,path,span);
 const binding=context.library.functions.find(row=>row.role==='equality'),entry=binding?context.library.program.declarations.get(binding.checkedName):null;
 if(entry?.k!=='fn'||!entry.body||entry.params.length!==4)fail('the actual projected order-independent equality callable body is unavailable',span);
 const helper=parser.freshProductBinding(),leftName=parser.freshProductBinding(),rightName=parser.freshProductBinding(),parameters=[leftOrigin,rightOrigin].map(origin=>({...structuredClone(origin),wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[]})),result={...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}},origin={$:'rustStandardCollectionEqualityOrigin',owner:'core::cmp::PartialEq',typeIdentity:first.origin.typeIdentity,operation:operator==='ne'||operator==='!='?'not equal':'equal',sourceVersionMinor:parser.sourceVersionMinor,proven:true,leftAvailable:true,rightAvailable:true,customImplementation:false},metadata=encodeRustStandardCollectionSourceConstant(context.library,origin,entry.params[2].type,span),handlers=context.library.sourceHandlers;
 const body=projectRustStandardCollectionSourceCarrier(parser,first,{k:'name',path:[leftName]},leftState=>projectRustStandardCollectionSourceCarrier(parser,second,{k:'name',path:[rightName]},rightState=>{
  const call={k:'app',fn:{k:'name',path:[...binding.sourcePath]},args:[leftState,rightState,metadata,{k:'num',value:'16777216',type:entry.params[3].type,span}],span};
  return matchRustStandardCollectionSourceVariant(parser,context.library,entry.ret,call,'rustCollectionEqualityCompleted',fields=>{
   const value=fields.get('value');if(value?.type.kind!=='bool')fail('the actual equality completion must retain its Boolean value',span);return value.expression;
  },(tag,fields)=>tag==='rustCollectionEqualityResourceLimit'?handlers.resource(fields.get('state')?.expression,fields.get('reason')?.expression):handlers.invalid(fields.get('reason')?.expression??{k:'str',value:'unexpected private collection equality continuation',span}),span);
 },span),span);
 parser.origins.registerCatalogEntry(helper,{k:'fn',parameters,result});context.privateNames.add(helper);context.wrappers.push({k:'fn',name:helper,params:[{name:leftName,type:parameters[0].type},{name:rightName,type:parameters[1].type}],ret:BOOL,body,span});
 return parser.origins.mark({k:'app',fn:{k:'name',path:['crate',helper]},args:[left,right],span},result);
}
export function readRustStandardCollectionSourceEqualityMethod(parser,receiver,method,args,path,span){
 if(!parser.standardCollectionSource||!['eq','ne'].includes(method.value))return null;
 const origin=parser.origins.origin(receiver,path);if(!readRustStandardCollectionSourceOrigin(parser,origin))return null;
 if(args.length!==1)throw typeError('standard collection PartialEq method requires its actual single borrowed operand',span);
 const argument=parser.origins.origin(args[0],path);if(argument.wrappers.length===0||argument.wrappers.some(layer=>layer!=='reference'))throw typeError('standard collection PartialEq method requires its actual shared reference argument',span);
 return readRustStandardCollectionSourceComparison(parser,receiver,args[0],method.value,path,span,true);
}
export function readRustStandardCollectionSourceEqualityCall(parser,segments,args,path,span){
 if(!parser.standardCollectionSource||!['eq','ne'].includes(segments.at(-1)))return null;
 const resolved=parser.resolve(segments,path),owner=resolved.slice(0,-1).join('::');if(!['core::cmp::PartialEq','std::cmp::PartialEq'].includes(owner))return null;
 if(parser.origins.lookupTypeOwner(segments.slice(0,-1),path)||parser.origins.lookupTypeOwner([resolved[0]],path)||parser.origins.scope.has(segments[0]))fail('explicit collection PartialEq call retains its actual unshadowed standard trait owner',span);
 if(args.length!==2)throw typeError('standard collection PartialEq function requires both actual borrowed operands',span);
 const origins=args.map(node=>parser.origins.origin(node,path));if(origins.every(origin=>!readRustStandardCollectionSourceOrigin(parser,origin)))return null;
 if(origins.some(origin=>origin.wrappers.length===0||origin.wrappers.some(layer=>layer!=='reference')))throw typeError('standard collection PartialEq function requires shared reference operands',span);
 return readRustStandardCollectionSourceComparison(parser,args[0],args[1],segments.at(-1),path,span,true);
}
