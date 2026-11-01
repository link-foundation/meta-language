// Actual source/cursor/catalog transport into shared standard sum decisions.
// Concrete sums reuse ordinary named datatype, constructor and match nodes.
import {unsupported,TranslationError} from './diagnostics.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {retainRustDataTypeFieldSignatures} from './rust-data-type-field-signatures.js';
import {readRustTypeAliasLookupKeys} from './rust-type-alias-rules.js';
import {readRustSourceNamespacePathRole} from './rust-source-namespace-rules.js';
import {readRustFunctionPointerTypeError} from './rust-function-pointer-rules.js';
import {measureRustStandardSumSourceMetadata,measureRustStandardSumSourceIdentityMetadata} from './rust-standard-sum-source-metadata.js';
import {readRustStandardSumSourceMetadataReservation} from './rust-standard-sum-source-metadata-rules.js';
import {readRustStandardSumSourceOwner,readRustStandardSumSourcePathOwner,readRustStandardSumSourcePathConstructor,readRustStandardSumSourceArgumentCount,acceptRustStandardSumSourceArguments,readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity,acceptRustStandardSumSourceInferredConstructor,readRustStandardSumSourceInstantiationLimit,readRustStandardSumSourceDepthLimit,readRustStandardSumSourceSpecializationName,readRustStandardSumSourcePhase,readRustStandardSumSourceDepth} from './rust-standard-sum-source-rules.js';

const fail=(reason,span)=>{throw unsupported('Rust standard sum source',reason,span);};
const copy=value=>structuredClone(value);
const where=(parser,start)=>({start:start.start,end:parser.cursor.tokens[Math.max(0,parser.cursor.index-1)].end});

export function initializeRustStandardSumSource(parser){
 parser.standardSumExpectedOrigin=null;
 parser.standardSumReturnOrigin=null;
 parser.standardSumSourceDepth=0;
 parser.standardSumSourceMaterializedNodes=0;
 parser.standardSumSourceMaterializedUnits=0;
 parser.standardSumSourceRows=[];
 for(const [name,entry] of parser.origins.catalog){
  if(entry.k==='data'&&entry.standardSumSourceOrigin){reserveSourceCopies(parser,[entry.standardSumSourceOrigin]);parser.standardSumSourceRows.push({name,origin:copy(entry.standardSumSourceOrigin)});parser.reservedNames.add(name);}
 }
}

export function withRustStandardSumSourceContext(parser,origin,read){
 const previous=parser.standardSumExpectedOrigin;
 parser.standardSumExpectedOrigin=origin;
 try{return read();}finally{parser.standardSumExpectedOrigin=previous;}
}

// The complete source catalog has already collected user declarations before
// final source admission. Standard prelude spellings are accepted only when
// no local declaration or import redirects them. A real local std/core module
// is never interpreted as a standard library definition.
function resolvedOwner(parser,segments,path,valueNamespace=false){
 const resolved=parser.resolve(segments,path),name=resolved.join('::');
 const owner=readRustStandardSumSourcePathOwner(name);
 if(!owner)return null;
 for(const names of [segments,resolved]){
  const role=readRustSourceNamespacePathRole(names.length,valueNamespace);
  if(parser.origins.lookupNamespace(names,path,role)||parser.origins.lookupNamespace([names[0]],path,role))return null;
  if(role==='type'&&readRustTypeAliasLookupKeys([names[0]],path).some(key=>parser.sourceTypeAliases?.has(key)))return null;
  if(role==='value'&&parser.origins.scope.has(names[0]))return null;
 }
 const canonical=readRustStandardSumSourceOwner(owner,true,0);
 return canonical?{owner:canonical,resolved,constructor:readRustStandardSumSourcePathConstructor(name)}:null;
}

export function resolveRustStandardSumSourceOwner(parser,segments,path){
 return resolvedOwner(parser,segments,path)?.owner??'';
}

export function readRustStandardSumSourceImport(parser,segments,path){
 if(!parser.ordinaryCompletionContext()||!['std','core'].includes(segments[0]))return false;
 return Boolean(resolvedOwner(parser,segments,path));
}

function qualifiedType(parser,type,path){
 if(!type)return null;
 if(type.kind==='named'){
  const entry=parser.origins.lookupTypeOwner(type.path,path);
  if(!entry||entry.k!=='data')return null;
  return {kind:'named',path:['crate',...entry.fullName.split('.')]};
 }
 if(type.kind==='tuple'){
  const elements=type.elements.map(element=>qualifiedType(parser,element,path));
  return elements.every(Boolean)?{kind:'tuple',elements}:null;
 }
 if(type.kind==='function'){
  const params=type.params.map(parameter=>qualifiedType(parser,parameter,path)),ret=qualifiedType(parser,type.ret,path);
  return params.every(Boolean)&&ret?{kind:'function',params,ret}:null;
 }
 return copy(type);
}

function qualifiedOrigin(parser,origin){
 if(!origin)return null;
 const result=copy(origin);
 if(result.fields){result.fields=result.fields.map(field=>qualifiedOrigin(parser,field));if(result.fields.some(field=>!field))return null;}
 // Product expression origins already retain every actual field. Recover a
 // tuple only when all of those fields have complete independent source types;
 // this does not default unsuffixed children or infer a nominal owner.
 if(!result.type&&result.fields&&result.fields.every(field=>field.expandedSourceOwner??field.sourceOwner)){
  result.type={kind:'tuple',elements:result.fields.map(field=>copy(field.type))};
  result.sourceOwner={kind:'tuple',path:[],arguments:result.fields.map(field=>copy(field.expandedSourceOwner??field.sourceOwner))};
 }
 if(!result.type)return null;
 const type=qualifiedType(parser,result.type,result.path);
 if(!type)return null;
 result.type=type;
 return result;
}

// Identity comes from complete checked source types/owners. Source coordinates,
// lexical aliases and incidental binding identities are not type meanings.
// Canonical named owners are resolved against their actual catalog definitions.
export function readRustStandardSumSourceIdentity(parser,origin){
 let visits=0;
 const project=(value,depth)=>{
  if(++visits>1048576||depth>256)fail('the standard sum identity projection exceeds its finite budget');
  if(!value?.proven||value.diverging||!value.type)return null;
  const entry=value.type.kind==='named'?parser.origins.lookupTypeOwner(value.type.path,value.path):null;
  const stored=entry?.k==='data'?entry.standardSumSourceOrigin:null;
  if(stored?.proven&&stored.copyEvidence&&stored.wrappers.length===0){
   const children=stored.standardSumSource.arguments.map(child=>project(child,depth+1));
   return children.some(child=>child===null)?null:{owner:stored.standardSumSource.owner,arguments:children,wrappers:value.wrappers};
  }
  const type=qualifiedType(parser,value.type,value.path);if(!type)return null;
  const fields=value.fields?.map(child=>project(child,depth+1))??null;
  if(fields?.some(child=>child===null))return null;
  return {type,owner:value.expandedSourceOwner??value.sourceOwner??null,wrappers:value.wrappers,fields,float:value.floatSourceModel??null,text:value.textSourceModel??null};
 };
 // Build child structures rather than repeatedly encoding identity strings.
 // One final encoding avoids recursive JSON escaping growth.
 metadataCost(parser,[origin],true,false);
 const result=project(origin,0);if(result===null)return '';
 return JSON.stringify(result);
}

function metadataCost(parser,origins,identities=false,header=false){
 try{
  const raw=measureRustStandardSumSourceMetadata(origins);
  if(!identities)return raw;
  const identity=measureRustStandardSumSourceIdentityMetadata(origins,(names,path)=>parser.origins.lookupTypeOwner(names,path),header);
  return [raw[0]+identity[0],raw[1]+identity[1]];
 }catch(error){if(error instanceof RangeError)fail(error.message);throw error;}
}

function reserveSourceCopies(parser,origins,identities=false,header=false){
 const cost=metadataCost(parser,origins,identities,header),next=readRustStandardSumSourceMetadataReservation(parser.standardSumSourceMaterializedNodes??0,parser.standardSumSourceMaterializedUnits??0,cost[0],cost[1]);
 if(next.length!==2)fail('the complete standard sum source metadata exceeds its materialization budget');
 parser.standardSumSourceMaterializedNodes=next[0];parser.standardSumSourceMaterializedUnits=next[1];
}

// Never trust an expression's sum marker alone: resolve its actual named data
// declaration, and return the declaration's complete original child origins.
export function readRustStandardSumSourceOrigin(parser,origin){
 if(!origin?.proven||origin.type?.kind!=='named')return null;
 const entry=parser.origins.lookupTypeOwner(origin.type.path,origin.path);
 if(entry?.k!=='data'||!entry.standardSumSourceOrigin)return null;
 const stored=entry.standardSumSourceOrigin;
 if(!stored.proven||!stored.copyEvidence||stored.wrappers.length!==0)return null;
 metadataCost(parser,[stored]);
 return {...copy(stored),wrappers:[...origin.wrappers]};
}

function instantiate(parser,owner,arguments_,span){
 if(arguments_.length!==readRustStandardSumSourceArgumentCount(owner))fail('the source sum needs every declared type argument',span);
 const provisional=readRustStandardSumSourcePhase(parser.headerDiscovery===true,false,false)==='provisional';
 reserveSourceCopies(parser,arguments_,true,provisional);
 if(readRustStandardSumSourceDepth(arguments_.map(argument=>actualOriginSumDepth(parser,argument,new Set(),0)),true)<0)fail('the complete concrete sum type exceeds its shared nested depth bound',span);
 arguments_=provisional?cloneHeaderArguments(arguments_):arguments_.map(argument=>qualifiedOrigin(parser,argument));
 if(arguments_.some(argument=>!argument))fail('a concrete child needs its actual resolved datatype owner',span);
 const identities=provisional?arguments_.map(argument=>JSON.stringify([headerType(parser,argument.type,argument.path),argument.path,argument.expandedSourceOwner??argument.sourceOwner,argument.wrappers])):arguments_.map(argument=>readRustStandardSumSourceIdentity(parser,argument));
 if(!provisional&&!acceptRustStandardSumSourceArguments(owner,identities,arguments_.map(argument=>parser.origins.immutableProduct(argument))))fail('every explicit child needs a complete direct immutable Copy source type',span);
 const key=JSON.stringify([provisional?'private header':'admitted',owner,identities]);
 for(const row of parser.standardSumSourceRows)if(row.origin.standardSumSource.identity===key)return copy(row.origin);
 if(!parser.standardSumSourceUpgradeRow&&parser.standardSumSourceRows.length>=readRustStandardSumSourceInstantiationLimit())fail('the concrete standard sum inventory exceeds its shared bound',span);
 const upgrading=parser.standardSumSourceUpgradeRow??null;
 let name=upgrading?.name??'',collision=0;
 if(!upgrading)for(;;){name=readRustStandardSumSourceSpecializationName(parser.standardSumSourceRows.length,collision);if(!name)fail('standard sum specialization name collisions exceed their shared bound',span);if(!parser.reservedNames.has(name)&&!parser.origins.catalog.has(name))break;collision+=1;}
 parser.reservedNames.add(name);
 const typePath=['crate',name],constructors=readRustStandardSumSourceConstructors(owner),constructorPaths={};
 for(const constructor of constructors)constructorPaths[constructor]=[...typePath,constructor];
 const sourceOwner={kind:'library',path:owner.split('::'),arguments:arguments_.map(argument=>{
  const actual=argument.expandedSourceOwner??argument.sourceOwner;
  if(!actual)fail('each concrete child must retain its actual source owner',span);
  return copy(actual);
 })};
 const origin={...parser.origins.typeOrigin({kind:'named',path:typePath},[]),proven:!provisional,uncertain:false,diverging:false,copyEvidence:!provisional,wrappers:[],fields:null,sourceOwner,expandedSourceOwner:copy(sourceOwner),standardSumSource:{owner,arguments:copy(arguments_),typePath,constructorPaths,identity:key,provisional}};
 parser.origins.registerCatalogEntry(name,{k:'data',standardSumSourceOrigin:copy(origin)});
 for(const constructor of constructors){
  const argument=readRustStandardSumSourceFieldArgument(owner,constructor),fields=argument<0?[]:[copy(arguments_[argument])];
  parser.origins.registerCatalogEntry(name+'.'+constructor,{k:'ctor',owner:name,fields,result:copy(origin)});
 }
 if(upgrading)upgrading.origin=copy(origin);else parser.standardSumSourceRows.push({name,origin:copy(origin)});
 return origin;
}

function actualOriginSumDepth(parser,origin,active,depth){
 if(depth>256)return -1;
 const entry=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null;
 const sum=entry?.standardSumSourceOrigin?.standardSumSource;
 if(sum){
  if(active.has(entry.fullName))return -1;active.add(entry.fullName);
  const result=readRustStandardSumSourceDepth(sum.arguments.map(argument=>actualOriginSumDepth(parser,argument,active,depth+1)),true);active.delete(entry.fullName);return result;
 }
 const children=[...(origin.fields??[]),...(origin.callableParameters??[]),...(origin.callableResult?[origin.callableResult]:[])];
 return readRustStandardSumSourceDepth(children.map(child=>actualOriginSumDepth(parser,child,active,depth+1)),false);
}

function cloneHeaderArguments(arguments_){
 return arguments_.map(argument=>argument?.type&&argument.sourceOwner?copy(argument):null);
}

function headerType(parser,type,path){
 if(type.kind==='named')return qualifiedType(parser,type,path)??{kind:'named',path:[...type.path]};
 if(type.kind==='tuple')return {kind:'tuple',elements:type.elements.map(element=>headerType(parser,element,path))};
 if(type.kind==='function')return {kind:'function',params:type.params.map(parameter=>headerType(parser,parameter,path)),ret:headerType(parser,type.ret,path)};
 return copy(type);
}

export function finishRustStandardSumSourceDeclarations(parser){
 const header=parser.headerDiscovery;parser.headerDiscovery=false;
 const active=new Set(),resolved=new Map(),rows=[...parser.standardSumSourceRows];
 const priorProvisionalNames=new Set(rows.filter(row=>row.origin.standardSumSource.provisional).map(row=>row.name));
 const refresh=origin=>{
  const entry=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null;
  if(entry?.standardSumSourceOrigin){
   const row=rows.find(row=>row.name===entry.fullName);
   if(!row)fail('the private header sum has no actual inventory row');
   const actual=resolve(row);return {...copy(actual),wrappers:[...origin.wrappers]};
  }
  const result=qualifiedOrigin(parser,origin);if(!result)fail('the complete header child must resolve to its actual datatype owner');
  if(result.fields)result.fields=result.fields.map(refresh);
  if(result.callableParameters)result.callableParameters=result.callableParameters.map(refresh);
  if(result.callableResult)result.callableResult=refresh(result.callableResult);
  if(result.type.kind==='tuple'&&result.fields)result.type={kind:'tuple',elements:result.fields.map(field=>copy(field.type))};
  if(result.type.kind==='function'&&result.callableParameters&&result.callableResult)result.type={kind:'function',params:result.callableParameters.map(parameter=>copy(parameter.type)),ret:copy(result.callableResult.type)};
  if(result.sourceFunctionPointerHeader){
   const reason=readRustFunctionPointerTypeError(parser.ordinaryCompletionContext(),(result.callableParameters??[]).map(parameter=>parser.origins.immutableProduct(parameter)),Boolean(result.callableResult&&parser.origins.immutableProduct(result.callableResult)));
   if(reason)fail(reason);
   delete result.sourceFunctionPointerHeader;result.proven=true;result.copyEvidence=true;
  }
  if(result.type.kind==='named'){
   const actual=parser.origins.typeOrigin(result.type,result.path);result.copyEvidence=actual.copyEvidence;result.proven=actual.proven;
   const owner=parser.origins.lookupTypeOwner(result.type.path,result.path);
   if(owner)for(const key of ['sourceOwner','expandedSourceOwner'])if(result[key]?.kind==='declaration')result[key]={...result[key],path:owner.fullName.split('.')};
  }else if(result.type.kind==='tuple')result.copyEvidence=result.fields?.every(field=>parser.origins.immutableProduct(field))===true;
  const ownerChildren=result.type.kind==='tuple'?result.fields:result.type.kind==='function'&&result.callableParameters&&result.callableResult?[...result.callableParameters,result.callableResult]:null;
  if(ownerChildren)for(const key of ['sourceOwner','expandedSourceOwner'])if(result[key]?.kind===result.type.kind)result[key]={...result[key],arguments:ownerChildren.map(child=>copy(child.expandedSourceOwner??child.sourceOwner))};
  return result;
 };
 const resolve=row=>{
  if(resolved.has(row.name))return resolved.get(row.name);
  if(active.has(row.name))fail('a private header sum child inventory is cyclic');
  active.add(row.name);
  const sum=row.origin.standardSumSource,arguments_=sum.arguments.map(refresh),previous=parser.standardSumSourceUpgradeRow;
  parser.standardSumSourceUpgradeRow=row;
  let actual;try{actual=instantiate(parser,sum.owner,arguments_);}finally{parser.standardSumSourceUpgradeRow=previous;}
  active.delete(row.name);resolved.set(row.name,actual);return actual;
 };
 try{
  for(const row of rows)resolve(row);
  if(!header&&rows.some(row=>{const actual=resolved.get(row.name);return actual.type.path[1]!==row.name||priorProvisionalNames.has(row.name);} ))parser.origins.standardSumSourceNeedsFinalAdmission=true;
  const rewriteType=type=>{
   if(!type)return type;
   if(type.kind==='named'){
    const row=type.path[0]==='crate'&&type.path.length===2?resolved.get(type.path[1]):null;
    return row?copy(row.type):type;
   }
   const result=copy(type);
   if(result.elements)result.elements=result.elements.map(rewriteType);
   if(result.params)result.params=result.params.map(rewriteType);
   if(result.ret)result.ret=rewriteType(result.ret);
   if(result.element)result.element=rewriteType(result.element);
   return result;
  };
  const rewrite=origin=>{
   if(origin.sourceFunctionPointerHeader)origin=refresh(origin);
   const entry=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null;
   const sum=entry?resolved.get(entry.fullName):null;
   if(sum)return {...copy(sum),wrappers:[...origin.wrappers]};
   const value=copy(origin);value.type=rewriteType(value.type);
   if(value.fields)value.fields=value.fields.map(rewrite);
   if(value.callableParameters)value.callableParameters=value.callableParameters.map(rewrite);
   if(value.callableResult)value.callableResult=rewrite(value.callableResult);
   return value;
  };
  for(const [name,entry] of new Map([...parser.origins.valueCatalog,...parser.origins.catalog])){
   // A legacy primary module can hide a function in the independent value
   // namespace. Refresh that function as well as the visible primary entry.
   const hidden=parser.origins.valueCatalog.get(name);
   if(hidden&&hidden!==entry){
    if(hidden.parameters)hidden.parameters=hidden.parameters.map(rewrite);
    if(hidden.fields)hidden.fields=hidden.fields.map(rewrite);
    if(hidden.result)hidden.result=rewrite(hidden.result);
    parser.origins.registerCatalogEntry(name,hidden);
   }
   if(entry.parameters)entry.parameters=entry.parameters.map(rewrite);
   if(entry.fields)entry.fields=entry.fields.map(rewrite);
   if(entry.result)entry.result=rewrite(entry.result);
   parser.origins.registerCatalogEntry(name,entry);
  }
  const unique=new Map();for(const row of rows){const actual=resolved.get(row.name),name=actual.type.path[1];if(!unique.has(name))unique.set(name,{name,origin:copy(actual)});}
  for(const row of rows)if(!unique.has(row.name)){parser.origins.removeCatalogEntry(row.name);for(const variant of readRustStandardSumSourceConstructors(row.origin.standardSumSource.owner))parser.origins.removeCatalogEntry(row.name+'.'+variant);}
  parser.standardSumSourceRows=[...unique.values()];
 }finally{parser.headerDiscovery=header;}
}

export function createRustStandardSumSourceOrigin(parser,owner,arguments_,span){
 return instantiate(parser,owner,arguments_,span);
}

export function readRustStandardSumSourceType(parser,segments,start){
 if(!parser.ordinaryCompletionContext())return null;
 const selected=resolvedOwner(parser,segments,parser.modulePath);
 if(!selected||selected.constructor||!parser.cursor.is('<'))return null;
 const c=parser.cursor,arguments_=[];
 c.expect('<','standard sum type');
 parser.standardSumSourceDepth+=1;
 try{
  if(parser.standardSumSourceDepth>readRustStandardSumSourceDepthLimit())fail('the nested standard sum source type exceeds its shared depth bound',where(parser,start));
  const count=readRustStandardSumSourceArgumentCount(selected.owner);
  for(let at=0;at<count;at+=1){
   if(at>0)c.expect(',','standard sum type arguments');
   parser.type();arguments_.push(copy(parser.origins.type));
  }
  c.eat(',');splitRustTypeClosingToken(parser);c.expect('>','standard sum type');
  const origin=instantiate(parser,selected.owner,arguments_,where(parser,start));
  parser.origins.type=origin;
  return origin.type;
 }finally{parser.standardSumSourceDepth-=1;}
}

function selectedConstructor(parser,segments,path){
 const selected=resolvedOwner(parser,segments,path,true);
 return selected?.constructor?selected:null;
}

function expectedSum(parser,owner,span){
 const expected=readRustStandardSumSourceOrigin(parser,parser.standardSumExpectedOrigin);
 // A postfix conversion can change families before the enclosing expression
 // is checked. Only a matching direct context supplies constructor arguments.
 return expected&&expected.wrappers.length===0&&expected.standardSumSource.owner===owner?expected:null;
}

function explicitConstructorType(parser,selected,start){
 const c=parser.cursor,arguments_=[];
 c.expect('::','standard sum constructor arguments');c.expect('<','standard sum constructor arguments');
 parser.standardSumSourceDepth+=1;
 try{
  if(parser.standardSumSourceDepth>readRustStandardSumSourceDepthLimit())fail('the nested standard sum constructor exceeds its shared depth bound',where(parser,start));
  const count=readRustStandardSumSourceArgumentCount(selected.owner);
  for(let at=0;at<count;at+=1){
   if(at>0)c.expect(',','standard sum constructor type arguments');
   parser.type();arguments_.push(copy(parser.origins.type));
  }
  c.eat(',');splitRustTypeClosingToken(parser);c.expect('>','standard sum constructor type arguments');
  const origin=instantiate(parser,selected.owner,arguments_,where(parser,start));
  if(!selected.constructor){
   c.expect('::','standard sum constructor variant');
   const token=c.identifier('standard sum constructor variant');
   const constructor=readRustStandardSumSourcePathConstructor(selected.owner+'::'+token.value);
   if(!constructor)fail('the selected variant must belong to the actual standard sum owner',where(parser,start));
   selected={...selected,constructor};
  }
  return {selected,origin};
 }finally{parser.standardSumSourceDepth-=1;}
}

export function readRustStandardSumSourceConstructor(parser,segments,path,start){
 if(!parser.ordinaryCompletionContext())return null;
 let selected=resolvedOwner(parser,segments,path,true);
 if(!selected)return null;
 const c=parser.cursor,span=where(parser,start);
 let explicit=null;
 if(c.is('::')&&c.is('<',1)){
  const result=explicitConstructorType(parser,selected,start);selected=result.selected;explicit=result.origin;
 }
 if(!selected.constructor)return null;
 const arity=readRustStandardSumSourceConstructorArity(selected.owner,selected.constructor);
 let expected=explicit??expectedSum(parser,selected.owner,span);
 const argument=readRustStandardSumSourceFieldArgument(selected.owner,selected.constructor),args=[];
 if(arity===1){
  c.expect('(','standard sum constructor');
  args.push(withRustStandardSumSourceContext(parser,expected?.standardSumSource.arguments[argument]??null,()=>parser.expr(path)));
  c.eat(',');c.expect(')','standard sum constructor');
 }else if(c.is('('))throw new TranslationError('type','None is a nullary standard constructor',span);
 // A complete actual Some payload determines all Option parameters. Its type
 // takes precedence over ambient context for a later postfix transformation.
 if(!explicit&&arity===1&&acceptRustStandardSumSourceInferredConstructor(selected.owner,selected.constructor)){
  const actual=parser.origins.origin(args[0],path);
  const identity=parser.origins.immutableProduct(actual)?readRustStandardSumSourceIdentity(parser,actual):'';
  if(identity&&identity!==readRustStandardSumSourceIdentity(parser,expected?.standardSumSource.arguments[0]))expected=instantiate(parser,selected.owner,[actual],where(parser,start));
 }
 if(!expected){
  if(!acceptRustStandardSumSourceInferredConstructor(selected.owner,selected.constructor))fail('this constructor needs an explicit complete standard sum type context',where(parser,start));
  expected=instantiate(parser,selected.owner,[parser.origins.origin(args[0],path)],where(parser,start));
 }
 const fields=expected.standardSumSource.arguments;
 if(arity===1)parser.origins.argument(parser.origins.origin(args[0],path),fields[argument],where(parser,start));
 const constructor=expected.standardSumSource.constructorPaths[selected.constructor],range=where(parser,start);
 const node=arity?{k:'app',fn:{k:'name',path:constructor,span:range},args,span:range}:{k:'name',path:constructor,span:range};
 return parser.origins.mark(node,expected);
}

export function readRustStandardSumSourcePattern(parser,segments,path,start,aliases=[]){
 if(!parser.ordinaryCompletionContext()||parser.cursor.is('@'))return null;
 const selected=selectedConstructor(parser,segments,path);
 if(!selected)return null;
 const c=parser.cursor,arity=readRustStandardSumSourceConstructorArity(selected.owner,selected.constructor),args=[];
 if(arity===1){
  c.expect('(','standard sum pattern');
  if(c.eat('..'))args.push({k:'wild'});
  else args.push(withRustStandardSumSourceContext(parser,null,()=>parser.pattern(path,aliases)));
  c.eat(',');c.expect(')','standard sum pattern');
 }else if(c.is('('))throw new TranslationError('type','None pattern expects no fields',where(parser,start));
 // if-let/let-else read the pattern before its scrutinee. Keep only its actual
 // resolved source owner here; origin extraction resolves its concrete data
 // path after reading that scrutinee, before installing any arm bindings.
 return {k:'rustStandardSumSourcePattern',owner:selected.owner,constructor:selected.constructor,args,span:where(parser,start)};
}

export function resolveRustStandardSumSourcePattern(origins,pattern,origin){
 if(pattern.k!=='rustStandardSumSourcePattern')return true;
 const entry=origin.type?.kind==='named'?origins.lookupTypeOwner(origin.type.path,origin.path):null;
 const stored=entry?.k==='data'?entry.standardSumSourceOrigin:null;
 if(!stored)return false;
 const sum=stored.standardSumSource,arity=readRustStandardSumSourceConstructorArity(sum.owner,pattern.constructor);
 if(!origin.proven||origin.wrappers.length||sum.owner!==pattern.owner||arity<0||arity!==pattern.args.length)fail('the pattern needs its actual direct scrutinee sum owner and exact constructor fields',pattern.span);
 const resolved={k:'ctor',path:[...sum.constructorPaths[pattern.constructor]],args:pattern.args,span:pattern.span};
 for(const key of Object.keys(pattern))delete pattern[key];
 Object.assign(pattern,resolved);
 return true;
}

export function refuseRustStandardSumSourcePatterns(root){
 const pending=[root],seen=new Set();
 while(pending.length){
  const value=pending.pop();
  if(value===null||typeof value!=='object'||seen.has(value))continue;
  if(seen.size>=100000)fail('source pattern admission exceeds its finite node budget',value.span);
  seen.add(value);
  if(value.k==='rustStandardSumSourcePattern')fail('a standard constructor pattern has no resolved actual scrutinee owner',value.span);
  for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);
 }
}

export function finishRustStandardSumSource(parser,items){
 if(parser.headerDiscovery)return;
 for(const row of parser.standardSumSourceRows){
  const sum=row.origin.standardSumSource,groups=[];
  const ctors=readRustStandardSumSourceConstructors(sum.owner).map(constructor=>{
   const argument=readRustStandardSumSourceFieldArgument(sum.owner,constructor),origins=argument<0?[]:[sum.arguments[argument]];
   groups.push(origins);
   return {name:constructor,fields:origins.map(origin=>({type:copy(origin.type)}))};
  });
  // Existing field-signature graph columns retain complete child ownership for
  // source-free checking, even when ordinary function signature export is off.
  items.push(retainRustDataTypeFieldSignatures({...parser,retainSourceTypeOwners:true},{k:'data',name:row.name,generated:true,ctors},[],groups));
 }
}
