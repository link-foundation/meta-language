import {readRustReferenceTokenParts} from './rust-type-token-rules.js';
// Tokens and source origins are transported into ordinary module/function terms.
import{unsupported}from'./diagnostics.js';
import{prepareRustRecordMethodNamespace}from'./rust-records.js';
import{acceptRustConstFunctionPrefix,acceptRustAssociatedConstantItem}from'./rust-function-qualifier-rules.js';
import{resolveRustTypeAliasPath}from'./rust-type-aliases.js';
import{acceptRustSourceLifetimeReference}from'./rust-indirection-rules.js';
import{acceptRustInherentSourceProfiles,readRustInherentReceiverLayerMode,planRustInherentReceiverLayers,acceptRustInherentOwnerModule,readRustInherentParentPath}from'./rust-inherent-rules.js';
function span(from,to){return{start:from.start,end:Math.max(from.end,to&&to.kind!=='eof'?to.start:from.end)};}

export function splitRustReferenceToken(parser){
 const c=parser.cursor,token=c.peek(),parts=readRustReferenceTokenParts(token.value);if(parts.length===0)return;
 let offset=token.start;const tokens=parts.map(value=>{const start=offset;offset+=value.length;return {...token,value,raw:value,start,end:offset};});c.tokens.splice(c.index,1,...tokens);
}

export function readRustInherentItem(parser,path){
 const c=parser.cursor,start=c.next();
 const models=[parser.arrayResultModel??parser.origins.arrayResultModel,parser.completionModel,parser.sourceSignatureModel,parser.scalarLoopModel,
  parser.primitiveOwnerModel,parser.emptyOptionSourceModel,parser.typeNamespaceModel,parser.pureUnitCallModel,parser.localValueStateModel,
  parser.constFunctionModel,parser.sharedArrayHandleSourceModel];
 if(!acceptRustInherentSourceProfiles(models))throw unsupported('Rust inherent profile composition','closed source models need an explicit method composition proof',span(start,c.peek()));
 if(c.is('<'))throw unsupported('generic Rust implementation','type parameters require their complete trait and ownership interpretation',span(start,c.peek()));
 const segments=resolveRustTypeAliasPath(parser,parser.pathSegments('implementation owner'),path,'implementation owner');
 if(c.is('<')||c.is('for')||c.is('where'))throw unsupported('generic or trait Rust implementation','traits and predicates need their complete dispatch interpretation',span(start,c.peek()));
 const found=parser.origins.lookup(parser.resolve(segments,path),path,['data']);
 const resolved=parser.resolve(segments,path);
 const owner=found?found.fullName.split('.'):resolved[0]==='crate'?resolved.slice(1):resolved[0]==='self'?[...path,...resolved.slice(1)]:[...path,...resolved];
 if(!found&&!parser.inherentDiscovery)throw unsupported('Rust implementation owner','an inherent implementation needs its actual source datatype',span(start,c.peek()));
 if(!acceptRustInherentOwnerModule(path,owner))throw unsupported('Rust implementation location','cross-module implementation placement requires its explicit namespace mapping',span(start,c.peek()));
 c.expect('{','implementation');
 const previous=parser.inherentOwner,previousModule=parser.modulePath;
 parser.inherentOwner={path:owner,modulePath:[...path]};
 const items=[];
 try{
  parser.innerAttributes();
  while(!c.is('}')){
   parser.modulePath=[...path];parser.attributes();parser.visibility();
   if(acceptRustAssociatedConstantItem(parser.ordinaryCompletionContext(),c.is('const'),c.is('fn',1))){
    prepareRustRecordMethodNamespace(parser,owner,c.peek(1).value,span(c.peek(),c.peek(1)));
    const constant=parser.constItem(owner);items.push(constant);
    // Mode zero reserves the associated value namespace; it never supplies a
    // receiver method interpretation. Callable values still use const origins.
    parser.inherentMethods.set([...owner,constant.name].join('.'),{owner:owner.join('.'),mode:0});
    continue;
   }
   const constant=acceptRustConstFunctionPrefix(parser.ordinaryCompletionContext(),false,c.is('const'),c.is('fn',1))?c.next():null;
   if(!c.is('fn'))throw unsupported('Rust associated item','associated constants, types, macros and modified functions need their own source interpretation',span(c.peek(),c.peek()));
   prepareRustRecordMethodNamespace(parser,owner,c.peek(1).value,span(c.peek(),c.peek(1)));
   const fn=parser.fn(owner,constant);items.push(fn);
   const mode=fn.params[0]?.name==='self'?parser.inherentMethods.get([...owner,fn.name].join('.'))?.mode??0:0;
   if(!mode)parser.inherentMethods.set([...owner,fn.name].join('.'),{owner:owner.join('.'),mode:0});
  }
  c.expect('}','implementation');
 }finally{parser.inherentOwner=previous;parser.modulePath=previousModule;}
 return{k:'module',name:owner.at(-1),items,span:span(start,c.peek())};
}

export function readRustInherentReceiver(parser,name,path,index){
 if(!parser.inherentOwner)return null;
 const c=parser.cursor,start=c.peek();
 if(!(c.is('self')||c.is('&')||c.is('mut')&&c.is('self',1)))return null;
 if(index!==0)throw unsupported('Rust receiver position','a receiver must be the first associated function parameter',span(start,start));
 const borrowed=Boolean(c.eat('&'));
 let lifetime=null;
 if(borrowed&&c.eat("'")){
  const token=c.identifier('lifetime');
  if(!acceptRustSourceLifetimeReference(token.value,parser.lifetimeBindings.has(token.value)))throw unsupported('lifetime parameter','a reference needs a declared function lifetime, an elided placeholder, or \'static',span(start,token));
  lifetime=token.value;
 }
 if(c.is('mut'))throw unsupported('Rust mutable or explicit lifetime receiver','receiver mutation and named regions need an explicit ownership interpretation',span(start,c.peek()));
 c.expect('self','method receiver');
 let type;
 if(c.eat(':')){
  if(borrowed)throw unsupported('Rust receiver syntax','an explicitly typed receiver uses an unprefixed self binding',span(start,c.peek()));
  type=parser.type();
  const origin=parser.origins.type;
  const owner=type.kind==='named'?parser.origins.lookup(type.path,origin.path,['data']):null;
  if((!owner||owner.fullName!==parser.inherentOwner.path.join('.'))&&!parser.inherentDiscovery)throw unsupported('Rust explicit receiver owner','an explicit receiver must resolve to its actual implementing datatype',span(start,c.peek()));
 }else{
  type={kind:'named',path:['crate',...parser.inherentOwner.path]};
  parser.origins.typed(type,parser.inherentOwner.modulePath,borrowed?['reference']:[]);
  parser.origins.type.sourceOwner={kind:'declaration',path:[...parser.inherentOwner.path],arguments:[]};
  if(borrowed)parser.origins.type.sourceLifetimes=[lifetime];
 }
 const origin=parser.origins.type;
 const mode=readRustInherentReceiverLayerMode(origin.wrappers);
 if(!mode)throw unsupported('Rust explicit receiver layers','the receiver needs represented owned, immutable reference or standard Box layers',span(start,c.peek()));
 parser.inherentMethods.set([...path,name].join('.'),{owner:parser.inherentOwner.path.join('.'),mode,receiver:[...origin.wrappers]});
 return{parameter:{name:'self',type,span:span(start,c.peek())},origin};
}

export function readRustInherentSelfType(parser){
 if(!parser.inherentOwner||!parser.cursor.is('Self'))return null;
 parser.cursor.next();
 return parser.origins.typed({kind:'named',path:['crate',...parser.inherentOwner.path]},parser.inherentOwner.modulePath);
}

export function resolveRustInherentPath(parser,segments,path){
 const context=parser.inherentOwner;
 if(!context)return null;
 if(segments[0]==='Self')return['crate',...context.path,...segments.slice(1)];
 const normalized=path.join('.')===context.path.join('.');
 if(!normalized&&path.join('.')!==context.modulePath.join('.'))return null;
 if(normalized&&segments.length===1&&parser.origins.scope.has(segments[0]))return segments;
 const actual=context.modulePath;
 if(segments[0]==='self')return['crate',...actual,...segments.slice(1)];
 if(segments[0]==='super'){
  let count=0;while(segments[count]==='super')count+=1;
  return readRustInherentParentPath(actual,path.length,count,segments.slice(count));
 }
 const aliases=parser.aliases.get(actual.join('.'));
 const resolved=aliases?.has(segments[0])?[...aliases.get(segments[0]),...segments.slice(1)]:segments;
 const found=parser.origins.lookup(resolved,actual);
 return found?['crate',...found.fullName.split('.')]:resolved;
}

export function readRustInherentMethod(parser,receiver,method,args,where,path){
 const origin=parser.origins.origin(receiver,path);
 const owner=origin.type?.kind==='named'?parser.origins.lookup(origin.type.path,origin.path,['data']):null;
 const methodPath=owner?[...owner.fullName.split('.'),method.value]:null;
 const selected=methodPath?parser.inherentMethods.get(methodPath.join('.')):null;
 if(selected?.mode){
  const plan=planRustInherentReceiverLayers(origin.wrappers,selected.receiver);
  if(owner.fullName!==selected.owner||plan[0]!==1)throw unsupported('Rust inherent receiver','the method needs its actual nominal receiver and supported immutable dereference or borrow layers',where);
  const argument={...receiver};
  parser.origins.mark(argument,{...origin,wrappers:plan.slice(1).map(layer=>layer===1?'reference':'box')});
  const values=[argument,...args],rooted=['crate',...methodPath];
  parser.origins.call(rooted,values,path,where);
  return{k:'app',fn:{k:'name',path:rooted,span:where},args:values,span:where};
 }
 if(parser.inherentDiscovery){
  return{k:'app',fn:{k:'name',path:methodPath??['unknown_inherent_owner',method.value],span:where},args:[receiver,...args],span:where};
 }
 return null;
}
