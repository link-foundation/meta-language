// Token/span ownership stays in the frontend. The translated shared source
// owns admission, transformation, resource limits and interval projection.
import {lowerJavaScriptClosedClassSource,projectJavaScriptClassSourceInterval,acceptJavaScriptClosedClassSourceSize,readJavaScriptClosedClassRefusal,readJavaScriptClosedClassContextError,readJavaScriptClosedClassOwnFieldUseError,acceptJavaScriptClosedClassOwnField} from './javascript-class-source-rules.js';
import {readSourceParserIdentifierSupply} from './source-identifiers.js';
import {unsupported,TranslationError} from './diagnostics.js';
import {checkProgram} from './check.js';
import {createSurfaceReferenceFacts} from './surface-reference-facts.js';
import {readCollectionConstructionField} from './collection-construction-rules.js';
import {readJavaScriptClosedClassObservationOperationOperands,containsJavaScriptClosedClassBrand} from './javascript-class-observation-rules.js';
import {decodeJavaScriptClassOwnerRows} from './javascript-class-owners.js';

export function readJavaScriptClosedClassSource(source,tokens,context) {
  const hasClass=tokens.some((token,index)=>token.kind==='identifier'&&token.value==='class'
    && !['.','?.'].includes(tokens[index-1]?.value)
    && (tokens[index+1]?.kind==='identifier'||tokens[index+1]?.value==='{'));
  if(!hasClass)return null;
  if(!acceptJavaScriptClosedClassSourceSize(source.length,tokens.length))throw unsupported('JavaScript closed class source',readJavaScriptClosedClassRefusal(-8),tokens[0]);
  const plan=lowerJavaScriptClosedClassSource(source,tokens.map(token=>token.value),tokens.map(token=>token.kind),
    tokens.map(token=>token.start),tokens.map(token=>token.end),readSourceParserIdentifierSupply(source,tokens));
  if(plan.error){const token=tokens[plan.at]??tokens[0];throw unsupported('JavaScript closed class source',plan.error,token);}
  if(!plan.changed)return null;
  const contextError=readJavaScriptClosedClassContextError(context.referenceSemantics!==false,context.externals?.length??0,context.literalBindings?.size??0,
    context.moduleBindingNames?.length??0,context.moduleScopeKnown!==false,context.moduleDirectory!==undefined&&context.moduleDirectory!==null);
  if(contextError)throw unsupported('JavaScript closed class context',contextError,{start:0,end:source.length});
  return {...plan,owners:decodeJavaScriptClassOwnerRows(plan.ownerRows)};
}

export function validateJavaScriptClosedClassUses(surface,plan) {
  const program=checkProgram(surface),brands=new Set(plan.brands);
  const facts=createSurfaceReferenceFacts({declarations:[...program.declarations.values()],main:program.main},{retainValues:true}),identities=new WeakMap();
  facts.values.forEach((value,index)=>{if(value&&typeof value==='object')identities.set(value,index);});
  const declarations=readCollectionConstructionField(facts.starts,facts.counts,facts.labels,facts.targets,facts.root,'declarations');
  const pending=[{node:program,direct:false}];
  while(pending.length){
    const {node,direct}=pending.pop();if(!node||typeof node!=='object')continue;
    const object=node.object,instance=object?.type?.kind==='record'&&object.type.fields.some(field=>brands.has(field.name));
    const ownField=acceptJavaScriptClosedClassOwnField(object?.type?.kind==='record'?object.type.fields.map(field=>field.name):[],plan.publicFields,node.field??'');
    const error=readJavaScriptClosedClassOwnFieldUseError(node.k??'',node.op??'',Boolean(instance),brands.has(node.field),node.type?.kind==='function',direct,ownField);
    if(error)throw unsupported('JavaScript closed class observation',error,node.span??{start:0,end:plan.text.length});
    const identity=node.k==='strictEqual'||node.k==='binary'&&['eq','ne'].includes(node.op)||['eq','ne'].includes(node.p)&&node.reference===true;
    const observedBrands=identity?plan.ownerRows.filter(row=>row[2]==='prototype helpers').map(row=>row[1]):plan.brands;
    for(const key of readJavaScriptClosedClassObservationOperationOperands(node.k??'',node.p??'',node.op??'').split(' '))if(key){
      for(const operand of Array.isArray(node[key])?node[key]:[node[key]])if(operand?.type&&containsJavaScriptClosedClassBrand(facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,identities.get(operand.type)??-1,declarations,observedBrands))throw unsupported('JavaScript closed class observation','class instances and nested class values require an explicit observation model; hidden source brands are not printable or structural equality data',node.span??{start:0,end:plan.text.length});
    }
    if(node instanceof Map){for(const value of node.values())pending.push({node:value,direct:false});}
    else for(const [key,value]of Object.entries(node))if(!['type','domain','span','modulePath'].includes(key)&&value&&typeof value==='object')pending.push({node:value,direct:node.k==='apply'&&key==='fn'});
  }
  return surface;
}

export function projectJavaScriptClosedClassOrigins(value,plan) {
  const pending=[value],seen=new WeakSet();
  while(pending.length){
    const node=pending.pop();
    if(!node||typeof node!=='object'||seen.has(node))continue;
    seen.add(node);
    if(node.span&&Number.isSafeInteger(node.span.start)&&Number.isSafeInteger(node.span.end)){
      const interval=projectJavaScriptClassSourceInterval(plan.intervals,node.span.start,node.span.end);
      if(interval.length===2)node.span={start:interval[0],end:interval[1]};
    }
    for(const [key,child]of Object.entries(node))if(key!=='span'&&child&&typeof child==='object')pending.push(child);
  }
  return value;
}

export function projectJavaScriptClosedClassError(error,plan){
  if(!(error instanceof TranslationError)||!error.span)throw error;
  const interval=projectJavaScriptClassSourceInterval(plan.intervals,error.span.start,error.span.end);
  if(interval.length!==2)throw error;
  throw new TranslationError(error.kind,error.reason,{start:interval[0],end:interval[1]},error.details);
}
