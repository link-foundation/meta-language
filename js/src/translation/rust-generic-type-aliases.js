// Actual source-token/origin transport around the shared generic alias policy.
// Templates are private construction trees; only checked concrete types and
// their complete original source origins enter the ordinary semantic program.
import {array,tuple,UNIT} from './types.js';
import {unsupported} from './diagnostics.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {readRustTypeAliasLookupKeys,readRustTypeAliasCloneError} from './rust-type-alias-rules.js';
import {acceptRustProductOrigin} from './rust-product-rules.js';
import {retainRustNominalCopyOrigin} from './rust-nominal-copies.js';
import {resolveRustStandardSumSourceOwner,createRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumSourcePathConstructor} from './rust-standard-sum-source-rules.js';
import {readRustContextualCallArguments} from './rust-copy-closures.js';
import {captureRustStandardSumConstructor,readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {acceptRustGenericTypeAliasOwnedStandardSumLeaf,readRustGenericTypeAliasOwnedConstructorPlan} from './rust-generic-type-alias-rules.js';
import {readRustGenericTypeAliasHeaderSubstitutionPlan,acceptRustGenericTypeAliasHeaderStandardSumOrigin,readRustGenericTypeAliasHeaderConstructorPlan} from './rust-generic-type-alias-rules.js';
import {acceptRustGenericTypeAliasConcreteCallableTemplate,acceptRustGenericTypeAliasHeaderCallableTemplate,acceptRustGenericTypeAliasOwnedCallableTemplate} from './rust-generic-type-alias-rules.js';
import {acceptRustGenericTypeAliasFunctionTemplateHead,acceptRustGenericTypeAliasFunctionParameterName,readRustGenericTypeAliasFunctionTemplateError} from './rust-generic-type-alias-rules.js';
import {readRustFunctionPointerDomainError,readRustFunctionPointerHeaderDomainError} from './rust-function-pointer-rules.js';
import {cloneRustSourceTypeOrigin as cloneRustCallableTypeOrigin} from './rust-source-type-origins.js';
import {readRustGenericTypeAliasDefaultSubstitutionPlan,readRustGenericTypeAliasDefaultArgumentPlan,acceptRustGenericTypeAliasConcreteIdentifier,readRustGenericTypeAliasDefaultAccessHeadLimit,readRustGenericTypeAliasDefaultOwnerPhase} from './rust-generic-type-alias-rules.js';
import {readRustGenericTypeAliasBindingPlan,readRustGenericTypeAliasSubstitutionPlan,readRustGenericTypeAliasExpansionError,readRustGenericTypeAliasTemplateDepthError,readRustGenericTypeAliasUnusedParameterError,acceptRustGenericTypeAliasConcreteTemplate,acceptRustGenericTypeAliasNamedTemplate,readRustGenericTypeAliasStandardSumOwner,readRustGenericTypeAliasStandardSumDepthError,acceptRustGenericTypeAliasStandardSumLeaf,readRustGenericTypeAliasVectorOwner,acceptRustGenericTypeAliasVectorArguments,readRustGenericTypeAliasVectorElementError,acceptRustGenericTypeAliasOwnedStringTemplate,readRustGenericTypeAliasConstructorPlan} from './rust-generic-type-alias-rules.js';

function charge(parser,context,row,count=1){
 const error=readRustTypeAliasCloneError(parser.sourceTypeAliasNodes,count);if(error)context.fail(parser,error,row);
 parser.sourceTypeAliasNodes+=count;
}
function bindings(parser,node,row,context){
 const result=[],pending=[{node,depth:0}];let count=0;
 while(pending.length){
  const entry=pending.pop(),current=entry.node;count+=1;
  const depthError=readRustGenericTypeAliasTemplateDepthError(entry.depth);if(depthError)context.fail(parser,depthError,row);
  const error=readRustTypeAliasCloneError(parser.sourceTypeAliasNodes,count);if(error)context.fail(parser,error,row);
  if(current.kind==='parameter')result.push(current.identity);
  else if(current.kind==='tuple'||current.kind==='standard sum'){const children=current.kind==='tuple'?current.fields:current.arguments;for(let at=children.length-1;at>=0;at-=1)pending.push({node:children[at],depth:entry.depth+1});}
  else if(current.kind==='vector')pending.push({node:current.element,depth:entry.depth+1});
  else if(current.kind==='function'){pending.push({node:current.result,depth:entry.depth+1});for(let at=current.parameters.length-1;at>=0;at-=1)pending.push({node:current.parameters[at],depth:entry.depth+1});}
 }
 parser.sourceTypeAliasNodes+=count;return result;
}
function aliasRow(parser,segments,path=parser.modulePath){
 const resolved=parser.resolve(segments,path);
 for(const key of readRustTypeAliasLookupKeys(resolved,path)){
  const row=parser.sourceTypeAliases.get(key);if(row)return row;
  if(['data','module','externalCrate'].includes(parser.origins.namespaceEntry(key,'type')?.k??''))return null;
 }
 return null;
}
function vectorOwner(parser,segments){
 const path=parser.modulePath,resolved=parser.resolve(segments,path);
 const roots=[segments.slice(0,1),resolved.slice(0,1)];
 const shadowed=roots.some(root=>Boolean(parser.origins.lookupTypeOwner(root,path))||readRustTypeAliasLookupKeys(root,path).some(key=>parser.sourceTypeAliases.has(key)));
 return readRustGenericTypeAliasVectorOwner(resolved.join('::'),shadowed);
}
function closeArguments(parser,context,row){
 splitRustTypeClosingToken(parser);
 if(!parser.cursor.eat('>'))context.fail(parser,'generic alias arguments need a complete closing delimiter',row);
}
function headerOrigin(parser,origin){
 if(headerCallable(parser,origin))return true;
 const data=origin.type?.kind==='named'?parser.origins.lookupTypeOwner(origin.type.path,origin.path):null;
 const stored=data?.standardSumSourceOrigin,sum=stored?.standardSumSource,owner=origin.expandedSourceOwner??origin.sourceOwner;
 return acceptRustGenericTypeAliasHeaderStandardSumOrigin(parser.headerDiscovery===true,origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,sum?.owner??'',sum?.arguments.length??-1,stored?.wrappers.length??-1,sum?.provisional===true);
}
function callableFacts(parser,origin){
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 return [parser.ordinaryCompletionContext(),origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.length??-1,owner?.arguments.length??-1,origin.type?.kind==='function'?origin.type.params.length:-1,(origin.callableParameters??[]).map(parameter=>parser.origins.immutableProduct(parameter)),parser.origins.immutableProduct(origin.callableResult),Boolean(origin.callableResult?.type)];
}
function concreteCallable(parser,origin){return acceptRustGenericTypeAliasConcreteCallableTemplate(...callableFacts(parser,origin),origin.proven===true,origin.copyEvidence===true,origin.copyClosureSpan!=null||origin.copyClosureIdentity!=null||origin.copyClosureCaptureCount!=null);}
function headerCallable(parser,origin){return acceptRustGenericTypeAliasHeaderCallableTemplate(...callableFacts(parser,origin),parser.headerDiscovery===true,origin.sourceFunctionPointerHeader===true,origin.copyClosureSpan!=null||origin.copyClosureIdentity!=null||origin.copyClosureCaptureCount!=null);}
function ownedCallable(parser,origin){
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 return acceptRustGenericTypeAliasOwnedCallableTemplate(parser.ordinaryCompletionContext(),origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.length??-1,owner?.arguments.length??-1,origin.type?.kind==='function'?origin.type.params.length:-1,(origin.callableParameters??[]).map(parameter=>readRustStandardSumOwnershipFacts(parser.origins,parameter).profile),readRustStandardSumOwnershipFacts(parser.origins,origin.callableResult).profile,Boolean(origin.callableResult?.type),origin.proven===true,origin.copyEvidence===true,origin.copyClosureSpan!=null||origin.copyClosureIdentity!=null||origin.copyClosureCaptureCount!=null,parser.sourceVersionMinor);
}
function symbolicApplication(parser,row,arguments_,context){
 const template=prepareRustGenericTypeAlias(parser,row,context),defaults=parser.sourceTypeAliasDefaults.get(row.key),completed=[...arguments_];
 const missing=readRustGenericTypeAliasDefaultArgumentPlan(defaults.map(Boolean),completed.length,true);if(missing[0]!=='arguments')context.fail(parser,missing[1],row);
 for(const position of missing.slice(1)){
  const index=Number(position),node=defaults[index],occurrences=bindings(parser,node,row,context),plan=readRustGenericTypeAliasDefaultSubstitutionPlan(row.parameterBindings,index,completed.length,occurrences);
  if(plan[0]!=='substitution')context.fail(parser,plan[1],row);
  completed.push(substituteTemplate(parser,row,node,completed,plan,context));
 }
 const occurrences=bindings(parser,template,row,context);
 const plan=readRustGenericTypeAliasBindingPlan(row.parameterBindings,completed.length,occurrences);
 if(plan[0]!=='substitution')context.fail(parser,plan[1],row);
 return substituteTemplate(parser,row,template,completed,plan,context);
}
function substituteTemplate(parser,row,template,arguments_,plan,context){
 let occurrence=1;
 const replace=(node,depth)=>{
  const error=readRustGenericTypeAliasTemplateDepthError(depth);if(error)context.fail(parser,error,row);charge(parser,context,row);
  if(node.kind==='parameter'){const index=Number(plan[occurrence++]);if(index<0)context.fail(parser,'generic alias template contains an unresolved parameter binding',row);return arguments_[index];}
  if(node.kind==='tuple')return {kind:'tuple',fields:node.fields.map(child=>replace(child,depth+1))};
  if(node.kind==='standard sum')return {kind:'standard sum',owner:node.owner,arguments:node.arguments.map(child=>replace(child,depth+1))};
  if(node.kind==='vector')return {kind:'vector',owner:node.owner,element:replace(node.element,depth+1)};
  if(node.kind==='function')return {kind:'function',parameters:node.parameters.map(child=>replace(child,depth+1)),result:replace(node.result,depth+1),span:node.span};
  return node;
 };
 return replace(template,0);
}
function templateValue(parser,row,context,depth){
 const error=readRustGenericTypeAliasTemplateDepthError(depth);if(error)context.fail(parser,error,row);
 charge(parser,context,row);
 const c=parser.cursor,start=c.index,startToken=c.peek();
 if(acceptRustGenericTypeAliasFunctionTemplateHead(startToken.value,startToken.raw)){
  c.next();c.expect('(','generic alias function template');const parameters=[];
  while(!c.is(')')){
   if(c.is(':',1)){
    if(!acceptRustGenericTypeAliasFunctionParameterName(c.peek().value,c.peek().raw))context.fail(parser,'function pointer parameter names require actual source identifiers',row);
    c.next();c.next();
   }
   const error=readRustGenericTypeAliasFunctionTemplateError(parameters.length+1);if(error)context.fail(parser,error,row);
   parameters.push(templateValue(parser,row,context,depth+1));if(!c.eat(','))break;
  }
  c.expect(')','generic alias function template');let result;
  if(c.eat('->'))result=templateValue(parser,row,context,depth+1);
  else{charge(parser,context,row);parser.origins.typed(UNIT,parser.modulePath);parser.origins.type.sourceOwner={kind:'unit',path:[],arguments:[]};result={kind:'origin',origin:context.copyOrigin(parser,parser.origins.type,row)};}
  return {kind:'function',parameters,result,span:{start:startToken.start,end:startToken.end}};
 }
 if(c.eat('(')){
  if(c.eat(')')){parser.origins.typed(UNIT,parser.modulePath);parser.origins.type.sourceOwner={kind:'unit',path:[],arguments:[]};return {kind:'origin',origin:context.copyOrigin(parser,parser.origins.type,row)};}
  const first=templateValue(parser,row,context,depth+1);
  if(!c.eat(',')){c.expect(')','generic alias parenthesized template');return first;}
  const fields=[first];
  while(!c.is(')')){fields.push(templateValue(parser,row,context,depth+1));if(!c.eat(','))break;}
  c.expect(')','generic alias tuple template');return {kind:'tuple',fields};
 }
 if(c.peek().kind==='identifier'){
  const parameter=row.parameters.indexOf(c.peek().value);
  if(parameter>=0){
   c.next();if(c.is('::')||c.is('<'))context.fail(parser,'generic alias parameter projections need their complete interpretation',row);
   return {kind:'parameter',identity:row.parameterBindings[parameter]};
  }
  const segments=parser.pathSegments('generic alias template type'),target=aliasRow(parser,segments);
  if(target?.parameters.length){
   // Establish declaration recursion before descending into arguments; an
   // expanding A<Box<T>> must not evade the A declaration-owner cycle.
   prepareRustGenericTypeAlias(parser,target,context);
   const explicit=Boolean(c.eat('<'));
   const arguments_=[];
   if(explicit){splitRustTypeClosingToken(parser);
   while(!c.is('>')){
    arguments_.push(templateValue(parser,row,context,depth+1));
    splitRustTypeClosingToken(parser);if(!c.eat(','))break;splitRustTypeClosingToken(parser);
   }
   closeArguments(parser,context,row);}
   const missing=readRustGenericTypeAliasDefaultArgumentPlan(parser.sourceTypeAliasDefaults.get(target.key).map(Boolean),arguments_.length,explicit);if(missing[0]!=='arguments')context.fail(parser,missing[1],target);
   return symbolicApplication(parser,target,arguments_,context);
  }
  const sumOwner=!target?resolveRustStandardSumSourceOwner(parser,segments,parser.modulePath):'';
  if(sumOwner&&c.eat('<')){
   const constructor=readRustStandardSumSourcePathConstructor(parser.resolve(segments,parser.modulePath).join('::')),arguments_=[];
   splitRustTypeClosingToken(parser);
   while(!c.is('>')){arguments_.push(templateValue(parser,row,context,depth+1));splitRustTypeClosingToken(parser);if(!c.eat(','))break;splitRustTypeClosingToken(parser);}
   closeArguments(parser,context,row);
   const owner=readRustGenericTypeAliasStandardSumOwner(sumOwner,constructor,arguments_.length);
   if(!owner)context.fail(parser,'generic alias standard sum needs its actual type owner and complete explicit arguments',row);
   return {kind:'standard sum',owner,arguments:arguments_};
  }
  const vector=!target?vectorOwner(parser,segments):'';
  if(vector&&c.eat('<')){
   const arguments_=[];splitRustTypeClosingToken(parser);
   while(!c.is('>')){arguments_.push(templateValue(parser,row,context,depth+1));splitRustTypeClosingToken(parser);if(!c.eat(','))break;splitRustTypeClosingToken(parser);}
   closeArguments(parser,context,row);
   if(!acceptRustGenericTypeAliasVectorArguments(vector,arguments_.length))context.fail(parser,'generic vector alias templates need the actual standard owner and one explicit element',row);
   return {kind:'vector',owner:vector,element:arguments_[0]};
  }
  c.index=start;
 }
 // Concrete leaves reuse the existing parser, but cannot hide an unresolved
 // parameter inside an unmodeled wrapper, callable or library application.
 parser.type();
 const origin=parser.origins.type,owner=origin.expandedSourceOwner??origin.sourceOwner;
 const data=origin.type?.kind==='named'?parser.origins.lookup(origin.type.path,origin.path,['data']):null;
 const stored=data?.standardSumSourceOrigin,sum=stored?.standardSumSource;
 const owned=stored?readRustStandardSumOwnershipFacts(parser.origins,stored).profile:'';
 if(!acceptRustGenericTypeAliasConcreteTemplate(origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',String(origin.type?.bits??''),origin.type?.signed===true,owner?.arguments.length??-1,origin.proven===true)&&!acceptRustGenericTypeAliasNamedTemplate(origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.proven===true,data?.fullName??'')&&!acceptRustGenericTypeAliasStandardSumLeaf(origin.type?.kind??'',origin.wrappers.length,sum?.owner??'',sum?.arguments.length??-1,origin.proven===true,stored?.proven===true,stored?.copyEvidence===true,stored?.wrappers.length??-1)&&!acceptRustGenericTypeAliasOwnedStandardSumLeaf(origin.type?.kind??'',origin.wrappers.length,sum?.owner??'',sum?.arguments.length??-1,origin.proven===true,stored?.proven===true,stored?.wrappers.length??-1,owned)&&!concreteCallable(parser,origin)&&!ownedCallable(parser,origin)&&!headerOrigin(parser,origin)&&!acceptRustGenericTypeAliasOwnedStringTemplate(origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.proven===true,origin.textSourceModel??''))context.fail(parser,'generic alias concrete leaves require a direct admitted scalar, Unit, owned String or actual source datatype',row);
 for(let at=c.tokens.indexOf(startToken);at<c.index;at+=1){if(c.tokens[at].kind==='identifier'&&!acceptRustGenericTypeAliasConcreteIdentifier(row.parameters,c.tokens[at].value,c.tokens[at-1]?.value==='::'))context.fail(parser,'parameter-dependent wrapper and library alias templates need their complete interpretation',row);}
 return {kind:'origin',origin:context.copyOrigin(parser,origin,row)};
}

export function prepareRustGenericTypeAlias(parser,row,context){
 const cached=parser.sourceTypeAliasTemplates.get(row.key);if(cached)return cached;
 const error=readRustGenericTypeAliasExpansionError(row.key,[...parser.sourceTypeAliasStack],parser.sourceTypeAliasExpansions);if(error)context.fail(parser,error,row);
 const previous={token:parser.cursor.peek(),path:parser.modulePath,lifetimes:parser.lifetimeBindings,header:parser.lifetimeHeader,inherent:parser.inherentOwner};
 parser.sourceTypeAliasExpansions+=1;parser.sourceTypeAliasStack.add(row.key);
 parser.cursor.index=parser.cursor.tokens.findIndex(token=>token.start===row.first);parser.modulePath=row.path;parser.lifetimeBindings=new Set();parser.lifetimeHeader=true;parser.inherentOwner=null;
 try{
  const defaults=[];
  for(let index=0;index<row.parameters.length;index+=1){
   const span=row.defaultSpans[index];if(!span){defaults.push(null);continue;}
   parser.cursor.index=parser.cursor.tokens.findIndex(token=>token.start===span.start);
   const node=templateValue(parser,row,context,0);
   if(parser.cursor.peek().start!==span.end)context.fail(parser,'the complete generic alias default needs one admitted source type',row);
   const plan=readRustGenericTypeAliasDefaultSubstitutionPlan(row.parameterBindings,index,index,bindings(parser,node,row,context));
   if(plan[0]!=='substitution')context.fail(parser,plan[1],row);defaults.push(node);
  }
  parser.cursor.index=parser.cursor.tokens.findIndex(token=>token.start===row.first);
  const template=templateValue(parser,row,context,0);
  if(parser.cursor.peek().start!==row.end||!parser.cursor.is(';'))context.fail(parser,'the complete alias definition needs one admitted source type',row);
  const used=bindings(parser,template,row,context),unused=readRustGenericTypeAliasUnusedParameterError(row.parameterBindings,used);if(unused)context.fail(parser,unused,row);
  parser.sourceTypeAliasDefaults.set(row.key,defaults);parser.sourceTypeAliasTemplates.set(row.key,template);return template;
 }finally{
  parser.cursor.index=parser.cursor.tokens.indexOf(previous.token);parser.modulePath=previous.path;parser.lifetimeBindings=previous.lifetimes;parser.lifetimeHeader=previous.header;parser.inherentOwner=previous.inherent;parser.sourceTypeAliasStack.delete(row.key);
 }
}

export function readRustGenericTypeAliasValue(parser,row,context,allowArgumentTokens=true){
 const template=prepareRustGenericTypeAlias(parser,row,context),c=parser.cursor;
 const explicit=allowArgumentTokens&&Boolean(c.eat('<'));
 const arguments_=[];
 if(explicit){splitRustTypeClosingToken(parser);
 while(!c.is('>')){
  parser.type();arguments_.push(context.copyOrigin(parser,parser.origins.type,row));
  splitRustTypeClosingToken(parser);if(!c.eat(','))break;splitRustTypeClosingToken(parser);
 }
 closeArguments(parser,context,row);}
 const defaults=parser.sourceTypeAliasDefaults.get(row.key),missing=readRustGenericTypeAliasDefaultArgumentPlan(defaults.map(Boolean),arguments_.length,explicit);
 if(missing[0]!=='arguments')context.fail(parser,missing[1],row);
 let occurrence=1,plan=[];
 const materialize=(node,depth)=>{
  const error=readRustGenericTypeAliasTemplateDepthError(depth);if(error)context.fail(parser,error,row);charge(parser,context,row);
  if(node.kind==='parameter'){const index=Number(plan[occurrence++]);if(index<0)context.fail(parser,'generic alias template contains an unresolved parameter binding',row);return context.copyOrigin(parser,arguments_[index],row);}
  if(node.kind==='origin'){const origin=context.copyOrigin(parser,node.origin,row);if(origin.type?.kind==='named'&&!origin.standardSumSource)retainRustNominalCopyOrigin(parser.origins,origin);return origin;}
  if(node.kind==='function'){
   const parameters=node.parameters.map(child=>cloneRustCallableTypeOrigin(materialize(child,depth+1))),result=cloneRustCallableTypeOrigin(materialize(node.result,depth+1)),copies=parameters.map(parameter=>parser.origins.immutableProduct(parameter)),resultCopy=parser.origins.immutableProduct(result),roles=parameters.map(parameter=>readRustStandardSumOwnershipFacts(parser.origins,parameter).profile),resultRole=readRustStandardSumOwnershipFacts(parser.origins,result).profile;
   const actual=readRustFunctionPointerDomainError(parser.ordinaryCompletionContext(),copies,resultCopy,roles,resultRole,parser.sourceVersionMinor),error=readRustFunctionPointerHeaderDomainError(parser.ordinaryCompletionContext(),copies,resultCopy,roles,resultRole,parser.sourceVersionMinor,parser.headerDiscovery===true);
   if(error)throw unsupported('Rust function pointer',error,node.span);
   const children=[...parameters,result],owners=children.map(child=>child.expandedSourceOwner??child.sourceOwner);
   if(owners.some(owner=>!owner)||children.some(child=>!child.type))context.fail(parser,'generic function alias children need complete actual source types and owners',row);
   const type={kind:'function',params:parameters.map(parameter=>parameter.type),ret:result.type};parser.origins.typed(type,row.path);
   const origin=parser.origins.type;origin.proven=actual==='';origin.copyEvidence=actual==='';origin.sourceFunctionPointerHeader=actual!=='';
   origin.sourceOwner={kind:'function',path:[],arguments:owners.map(owner=>structuredClone(owner))};origin.callableParameters=parameters;origin.callableResult=result;
   if(!concreteCallable(parser,origin)&&!ownedCallable(parser,origin)&&!headerCallable(parser,origin))context.fail(parser,'generic function alias needs its complete existing pointer source contract',row);
   return context.copyOrigin(parser,origin,row);
  }
  if(node.kind==='standard sum'){
   parser.standardSumSourceDepth+=1;
   try{
    const error=readRustGenericTypeAliasStandardSumDepthError(parser.standardSumSourceDepth);if(error)context.fail(parser,error,row);
    const arguments_=node.arguments.map(child=>materialize(child,depth+1));
    return context.copyOrigin(parser,createRustStandardSumSourceOrigin(parser,node.owner,arguments_,row.keywordSpan),row);
   }finally{parser.standardSumSourceDepth-=1;}
  }
  if(node.kind==='vector'){
   const element=materialize(node.element,depth+1),owner=element.expandedSourceOwner??element.sourceOwner;
   const error=readRustGenericTypeAliasVectorElementError(element.type?.kind??'',element.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',String(element.type?.bits??''),element.type?.signed===true,owner?.arguments.length??-1,element.proven===true,element.textSourceModel??'');if(error)context.fail(parser,error,row);
   parser.origins.typed(array(element.type),row.path);
   const origin=parser.origins.type;
   origin.sourceOwner={kind:'library',path:node.owner.split('::'),arguments:[structuredClone(owner)]};
   return context.copyOrigin(parser,origin,row);
  }
  const fields=node.fields.map(child=>materialize(child,depth+1));
  parser.origins.typed(tuple(fields.map(field=>field.type)),row.path,[],fields);
  const origin=parser.origins.type;
  if(!parser.headerDiscovery&&!acceptRustProductOrigin(parser.origins.immutableProduct(origin),parser.origins.ownedLocalProduct(origin)))context.fail(parser,'generic alias tuple fields need the existing immutable or owned product interpretation',row);
  origin.sourceOwner={kind:'tuple',path:[],arguments:fields.map(field=>structuredClone(field.sourceOwner))};
  return context.copyOrigin(parser,origin,row);
 };
 // Every use constructs its own complete origin. Declaration-only caches must
 // never conflate distinct concrete arguments or their actual source owners.
 for(const position of missing.slice(1)){
  const index=Number(position),node=defaults[index];
  plan=readRustGenericTypeAliasDefaultSubstitutionPlan(row.parameterBindings,index,arguments_.length,bindings(parser,node,row,context));
  if(plan[0]!=='substitution')context.fail(parser,plan[1],row);
  occurrence=1;arguments_.push(materialize(node,0));
 }
 const concreteOccurrences=bindings(parser,template,row,context);
 plan=readRustGenericTypeAliasSubstitutionPlan(row.parameterBindings,arguments_.map(origin=>origin.proven===true&&Boolean(origin.type)),concreteOccurrences);
 if(plan[0]!=='substitution')plan=readRustGenericTypeAliasHeaderSubstitutionPlan(row.parameterBindings,parser.headerDiscovery===true,arguments_.map(origin=>Boolean(origin.type&&(origin.expandedSourceOwner??origin.sourceOwner))),arguments_.map(origin=>origin.proven===true),arguments_.map(origin=>headerOrigin(parser,origin)),concreteOccurrences);
 if(plan[0]!=='substitution')context.fail(parser,plan[1],row);
 occurrence=1;return materialize(template,0);
}

// This cache represents the declaration's complete all-default application
// only. Explicit specializations continue to construct independent origins.
export function readRustGenericTypeAliasDefaultOrigin(parser,row,context){
 const plan=readRustGenericTypeAliasDefaultArgumentPlan(row.defaultSpans.map(Boolean),0,false);if(plan[0]!=='arguments')return null;
 const cached=parser.sourceTypeAliasCache.get(row.key);if(cached)return context.copyOrigin(parser,cached,row);
 const origin=readRustGenericTypeAliasValue(parser,row,context,false);
 origin.expandedSourceOwner=structuredClone(origin.sourceOwner);
 parser.sourceTypeAliasCache.set(row.key,context.copyOrigin(parser,origin,row));return origin;
}

export function prepareRustGenericTypeAliasDefaultAccess(parser,segments,path,context){
 if(!parser.ordinaryCompletionContext()||segments.length<2)return;
 prepareRustGenericTypeAliasDefaultOwner(parser,segments.slice(0,-1),path,context);
}
export function prepareRustGenericTypeAliasDefaultOwner(parser,segments,path,context,discovery=false){
 if(!parser.ordinaryCompletionContext()||readRustGenericTypeAliasDefaultOwnerPhase(parser.origins.collecting,discovery)==='defer')return;
 const row=aliasRow(parser,segments,path);
 if(!row?.parameters.length||parser.sourceTypeAliasCache.has(row.key))return;
 readRustGenericTypeAliasDefaultOrigin(parser,row,context);
}
export function prepareRustGenericTypeAliasDefaultAccessHead(parser,path,context){
 if(!parser.ordinaryCompletionContext()||parser.cursor.is('::'))return;
 const c=parser.cursor,segments=[];let offset=0;
 while(c.peek(offset).kind==='identifier'&&segments.length<readRustGenericTypeAliasDefaultAccessHeadLimit()){
  segments.push(c.peek(offset).value);offset+=1;
  if(!c.is('::',offset)){prepareRustGenericTypeAliasDefaultAccess(parser,segments,path,context);return;}
  if(c.peek(offset+1).kind!=='identifier')return;
  offset+=1;
 }
}

/** Actual alias-qualified turbofish construction. Nothing is selected from
 * a type spelling alone: instantiate first, then consult its complete owner.
 */
export function readRustGenericTypeAliasConstructor(parser,segments,path,start,context){
 const c=parser.cursor;
 if(!parser.ordinaryCompletionContext())return null;
 const explicit=c.is('::')&&c.is('<',1);
 if(!explicit&&segments.length<2)return null;
 const row=aliasRow(parser,explicit?segments:segments.slice(0,-1),path);if(!row?.parameters.length)return null;
 if(!explicit&&readRustGenericTypeAliasDefaultArgumentPlan(row.defaultSpans.map(Boolean),0,false)[0]!=='arguments')return null;
 const previousMember=explicit?null:c.tokens[c.index-1];
 if(explicit)c.next();const origin=explicit?readRustGenericTypeAliasValue(parser,row,context):readRustGenericTypeAliasDefaultOrigin(parser,row,context);
 origin.expandedSourceOwner=structuredClone(origin.sourceOwner);
 if(explicit)c.expect('::','generic alias associated constructor');
 const member=explicit?c.identifier('generic alias constructor member'):previousMember,owner=origin.expandedSourceOwner??origin.sourceOwner;
 const data=origin.type?.kind==='named'?parser.origins.lookup(origin.type.path,origin.path,['data']):null,stored=data?.standardSumSourceOrigin,sum=stored?.standardSumSource;
 let plan=readRustGenericTypeAliasConstructorPlan(origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.type?.element?.kind??'',origin.arraySourceModel??'',origin.proven===true,sum?.owner??'',sum?.arguments.length??-1,stored?.proven===true,stored?.copyEvidence===true,member.value);
 if(plan.length===0)plan=readRustGenericTypeAliasOwnedConstructorPlan(origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.proven===true,sum?.owner??'',sum?.arguments.length??-1,stored?.proven===true,stored?readRustStandardSumOwnershipFacts(parser.origins,stored).profile:'',member.value);
 if(plan.length===0)plan=readRustGenericTypeAliasHeaderConstructorPlan(parser.headerDiscovery===true,sum?.provisional===true,origin.type?.kind??'',origin.wrappers.length,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,sum?.owner??'',sum?.arguments.length??-1,stored?.wrappers.length??-1,member.value);
 if(plan.length===0){if(!explicit)return null;context.fail(parser,'generic alias associated construction needs its actual admitted vector or standard sum constructor',row);}
 const arity=Number(plan[1]);let end=member.end,args=[],constructorPath=null;
 if(plan[0]==='vector constructor'){
  c.expect('(','generic alias Vec constructor');end=c.expect(')','generic alias Vec constructor').end;
  return parser.origins.markVectorConstructor({k:'array',items:[],element:origin.type.element,span:{start:start.start,end}},{...origin,path:[...path]});
 }
 constructorPath=sum.constructorPaths[member.value];
 if(!constructorPath)context.fail(parser,'the generic alias constructor needs its actual registered variant path',row);
 if(arity===0){if(c.is('('))context.fail(parser,'a nullary generic alias variant is a value without call arguments',row);}
 else{
  c.expect('(','generic alias standard sum constructor');
  args=readRustContextualCallArguments(parser,{k:'name',path:constructorPath,span:{start:start.start,end:member.end}},path);
  end=c.expect(')','generic alias standard sum constructor').end;
  if(args.length!==arity)context.fail(parser,'generic alias constructor arguments differ from their actual source fields',row);
 }
 // Contextual arguments may already have transferred real owners. The
 // constructor-origin consumer validates fields and records any remaining
 // transfer exactly once; a second generic call pass would consume them again.
 const span={start:start.start,end};
 const name={k:'name',path:[...constructorPath],span};
 const node=args.length?{k:'app',fn:name,args,span}:name;
 captureRustStandardSumConstructor(parser,node,origin,args,path,span);return node;
}
