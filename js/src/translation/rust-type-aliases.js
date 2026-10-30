import {readRustConstantPrimitiveAliasOwner} from './rust-constant-integer-rules.js';
// Token/type/source-origin transport. Alias lookup and bounds are shared rules.
import {unsupported} from './diagnostics.js';
import {readRustTypeAliasDeclarationPlan,readRustTypeAliasLookupKeys,readRustTypeAliasExpansionError,readRustTypeAliasCloneError,readRustTypeAliasAccessPrefixLengths,acceptRustTypeAliasNominalAccess,readRustTypeAliasNamespaceDisposition,readRustTypeAliasValueConstructorError} from './rust-type-alias-rules.js';
function fail(parser,message,row){const token=row?.keywordSpan??parser.cursor.peek();throw unsupported('Rust type alias',message,{start:token.start,end:token.end});}
export function initializeRustTypeAliases(parser){
 parser.sourceTypeAliases=new Map();parser.sourceTypeAliasCache=new Map();parser.sourceTypeAliasStack=new Set();parser.sourceTypeAliasExpansions=0;parser.sourceTypeAliasNodes=0;parser.sourceTypeAliasesChecked=false;
 if(!parser.ordinaryCompletionContext())return;
 if(!parser.cursor.tokens.some(token=>token.kind==='identifier'&&token.value==='type'&&token.raw==='type'))return;
 const tokens=parser.cursor.tokens,plan=readRustTypeAliasDeclarationPlan(tokens.map(token=>token.kind==='identifier'&&['mod','type'].includes(token.value)&&token.raw===token.value?'keyword':token.kind),tokens.map(token=>token.value));
 if(plan[0]!=='aliases')fail(parser,plan[1]);
 for(let at=1;at<plan.length;at+=5){const key=plan[at],keyword=tokens[Number(plan[at+1])],first=tokens[Number(plan[at+2])],last=tokens[Number(plan[at+3])];parser.sourceTypeAliases.set(key,{key,keywordSpan:{start:keyword.start,end:keyword.end},first:first.start,end:last.start,path:key.split('.').slice(0,-1)});}
}
function qualifyType(parser,type,path){
 if(type.kind==='named'){
  const resolved=parser.resolve(type.path,path),entry=sourceDataOwner(parser,resolved,path);
  return {...type,path:entry?['crate',...entry.split('.')]:resolved};
 }
 if(type.kind==='array')return {...type,element:qualifyType(parser,type.element,path)};
 if(type.kind==='tuple')return {...type,elements:type.elements.map(value=>qualifyType(parser,value,path))};
 if(type.kind==='function')return {...type,params:type.params.map(value=>qualifyType(parser,value,path)),ret:qualifyType(parser,type.ret,path)};
 return structuredClone(type);
}
function sourceDataOwner(parser,segments,path){for(const key of readRustTypeAliasLookupKeys(segments,path)){const owner=parser.origins.catalog.get(key)?.k??'';if(owner==='data')return key;if(readRustTypeAliasNamespaceDisposition(false,owner)==='block')return null;}return null;}
function typeChildren(type){return type.kind==='array'?[type.element]:type.kind==='tuple'?type.elements:type.kind==='function'?[...type.params,type.ret]:[];}
function copyOrigin(parser,origin,row){
 const origins=[origin],types=[];let nodes=0;
 for(let at=0;at<origins.length;at+=1){const value=origins[at];nodes+=1;if(value.type)types.push(value.type);origins.push(...(value.fields??[]),...(value.callableParameters??[]));if(value.callableResult)origins.push(value.callableResult);const message=readRustTypeAliasCloneError(parser.sourceTypeAliasNodes,nodes);if(message)fail(parser,message,row);}
 for(let at=0;at<types.length;at+=1){nodes+=1;types.push(...typeChildren(types[at]));const message=readRustTypeAliasCloneError(parser.sourceTypeAliasNodes,nodes);if(message)fail(parser,message,row);}
 parser.sourceTypeAliasNodes+=nodes;
 const copy=value=>{const {type,fields,callableParameters,callableResult,...rest}=value;return {...structuredClone(rest),type:type?qualifyType(parser,type,value.path):null,fields:fields?.map(copy)??null,callableParameters:callableParameters?.map(copy)??null,callableResult:callableResult?copy(callableResult):null};};
 return copy(origin);
}
function expandAlias(parser,row){
 const cached=parser.sourceTypeAliasCache.get(row.key);if(cached)return copyOrigin(parser,cached,row);
 const message=readRustTypeAliasExpansionError(parser.sourceTypeAliasStack.size,parser.sourceTypeAliasExpansions,parser.sourceTypeAliasStack.has(row.key));if(message)fail(parser,message,row);
 const previous={index:parser.cursor.index,path:parser.modulePath,lifetimes:parser.lifetimeBindings,header:parser.lifetimeHeader,inherent:parser.inherentOwner};
 parser.sourceTypeAliasExpansions+=1;parser.sourceTypeAliasStack.add(row.key);
 parser.cursor.index=parser.cursor.tokens.findIndex(token=>token.start===row.first);parser.modulePath=row.path;parser.lifetimeBindings=new Set();parser.lifetimeHeader=true;parser.inherentOwner=null;
 try{
  parser.type();if(parser.cursor.peek().start!==row.end||!parser.cursor.is(';'))fail(parser,'the complete alias definition needs one admitted source type',row);
  if(parser.origins.type.sourceLifetimes?.some(value=>value===null))fail(parser,'reference aliases require explicit source lifetimes',row);
  const origin=copyOrigin(parser,parser.origins.type,row);parser.sourceTypeAliasCache.set(row.key,origin);return copyOrigin(parser,origin,row);
 }finally{parser.cursor.index=previous.index;parser.modulePath=previous.path;parser.lifetimeBindings=previous.lifetimes;parser.lifetimeHeader=previous.header;parser.inherentOwner=previous.inherent;parser.sourceTypeAliasStack.delete(row.key);}
}
export function readRustTypeAliasValue(parser,segments){
 if(!parser.ordinaryCompletionContext())return null;
 const resolved=parser.resolve(segments,parser.modulePath);
 for(const key of readRustTypeAliasLookupKeys(resolved,parser.modulePath)){
  const row=parser.sourceTypeAliases.get(key);if(!row){if(readRustTypeAliasNamespaceDisposition(false,parser.origins.catalog.get(key)?.k??'')==='block')return null;continue;}
  const origin=expandAlias(parser,row);origin.expandedSourceOwner=structuredClone(origin.sourceOwner);parser.origins.type=origin;return origin.type;
 }
 return null;
}
export function readRustTypeAliasItem(parser,path){
 const row=[...parser.sourceTypeAliases.values()].find(row=>row.keywordSpan.start===parser.cursor.peek().start);
 if(!row||row.path.join('.')!==path.join('.'))fail(parser,'the alias needs its actual module declaration');
 parser.cursor.index=parser.cursor.tokens.findIndex(token=>token.start===row.end)+1;
}
export function finishRustTypeAliases(parser){
 if(!parser.ordinaryCompletionContext()||parser.sourceTypeAliasesChecked)return;
 // The collecting pass may have read a header before a later use item. Rebuild
 // from the complete actual namespace, retaining the charged allocation budget.
 parser.sourceTypeAliasCache.clear();
 for(const row of parser.sourceTypeAliases.values()){
  if(['data','module'].includes(parser.origins.catalog.get(row.key)?.k))fail(parser,'a type alias cannot replace another source type or module',row);
  const origin=expandAlias(parser,row),types=origin.type?[origin.type]:[];
  for(let at=0;at<types.length;at+=1){const type=types[at];types.push(...typeChildren(type));if(type.kind==='named'&&!sourceDataOwner(parser,parser.resolve(type.path,origin.path),origin.path))fail(parser,'the complete alias definition needs an actual source datatype owner',row);}
 }
 parser.sourceTypeAliasesChecked=true;
}
/** Read the complete already checked type namespace without changing values,
 * argument evaluation, source ownership or the original constructor rules. */
export function resolveRustTypeAliasPath(parser,segments,path,role){
 if(!parser.ordinaryCompletionContext()||parser.sourceTypeAliases.size===0)return segments;
 for(const length of readRustTypeAliasAccessPrefixLengths(segments.length,role!=='associated value')){
  const resolved=parser.resolve(segments.slice(0,length),path);
  for(const key of readRustTypeAliasLookupKeys(resolved,parser.modulePath)){
   const disposition=readRustTypeAliasNamespaceDisposition(parser.sourceTypeAliases.has(key),parser.origins.catalog.get(key)?.k??'');
   if(disposition==='block')return segments;
   if(disposition!=='alias')continue;
   const origin=parser.sourceTypeAliasCache.get(key);if(!origin)return segments;
   if(role==='associated integer constant'){
    const source=origin.expandedSourceOwner??origin.sourceOwner,primitive=readRustConstantPrimitiveAliasOwner(origin.type?.kind??'',source?.kind??'',source?.path.join('::')??'',source?.arguments.length??-1,origin.wrappers.length,String(origin.type?.bits??''),origin.type?.signed===true);
    if(primitive)return [primitive,...segments.slice(length)];
   }
   const owner=origin.type?.kind==='named'?sourceDataOwner(parser,parser.resolve(origin.type.path,origin.path),origin.path):null;
   if(!owner||!acceptRustTypeAliasNominalAccess(origin.type?.kind??'',origin.wrappers.length,segments.length-length,role,parser.namedRecords.get(owner)?.syntax??''))return segments;
   return ['crate',...owner.split('.'),...segments.slice(length)];
  }
 }
 return segments;
}
export function requireRustTypeAliasValueConstructor(parser,segments,path,where){
 if(!parser.ordinaryCompletionContext()||parser.sourceTypeAliases.size===0)return;
 const resolved=parser.resolve(segments,path),local=segments.length===1&&parser.origins.scope.has(segments[0]),keys=readRustTypeAliasLookupKeys(resolved,parser.modulePath);
 const value=keys.some(key=>['fn','const','ctor'].includes(parser.origins.catalog.get(key)?.k??'')),alias=keys.some(key=>parser.sourceTypeAliases.has(key));
 const message=readRustTypeAliasValueConstructorError(local,value,alias);
 if(message)throw unsupported('Rust type alias value constructor',message,where);
}
