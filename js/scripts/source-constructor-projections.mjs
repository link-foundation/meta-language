// Algebraic value projections only. No ordinary property Get, reference
// object, shadowed parameter, deferred closure or unmatched owner is hoisted.
import {planSingleConstructorProjections} from '../src/translation/boolean-constructor-match-rules.js';
import {sameType} from '../src/translation/types.js';
export function shareSourceConstructorProjections(program,entry,body,proofs){
 if(program.sourceLanguage!=='JavaScript'||program.referenceSemantics===true)return body;
 const used=new Set([...program.declarations.keys(),...program.declarations.values()].flatMap(value=>typeof value==='string'?[value]:[value.name]));
 const shadowed=new Set(),seen=new WeakSet(),pending=[body];let visits=0;
 while(pending.length){
  const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);
  if(++visits>100000)throw new TypeError('constructor projection ownership exceeds its structural budget');
  if(typeof node.name==='string')used.add(node.name);
  if(node.k==='let')shadowed.add(node.name);
  if(node.k==='lambda')for(const parameter of node.params){used.add(parameter.name);shadowed.add(parameter.name);}
  if(node.pattern?.binds)for(const name of node.pattern.binds)if(typeof name==='string'){used.add(name);shadowed.add(name);}
  if(node.pattern?.k==='bind'||node.pattern?.k==='natSucc'){used.add(node.pattern.name);shadowed.add(node.pattern.name);}
  for(const [key,child]of Object.entries(node))if(!['type','domain','from','to','span','pattern'].includes(key))pending.push(child);
 }
 for(const parameter of entry.params)used.add(parameter.name);
 let ordinal=0;
 function fresh(){let name;do{name='sharedConstructorField'+(++ordinal);}while(used.has(name));used.add(name);return name;}
 const excluded=new Set(['type','domain','from','to','span','pattern']);
 for(const [parameterIndex,parameter]of entry.params.entries()){
  if(parameter.type.kind!=='data'||shadowed.has(parameter.name))continue;
  const datatype=program.declarations.get(parameter.type.name);if(datatype?.k!=='data'||datatype.ctors.length!==1)continue;
  const constructor=datatype.ctors[0],counts=constructor.fields.map(()=>0),occurrences=[];
  function position(node){
   if(node?.k!=='match'||node.scrutinee?.k!=='var'||node.scrutinee.name!==parameter.name||node.scrutinee.type?.name!==datatype.fullName||node.cases.length!==1)return -1;
   const arm=node.cases[0],pattern=arm.pattern;
   if(pattern?.k!=='ctor'||pattern.data!==datatype.fullName||pattern.ctor!==constructor.name||!Array.isArray(pattern.binds)||pattern.binds.length!==constructor.fields.length||arm.body?.k!=='var')return -1;
   const index=pattern.binds.indexOf(arm.body.name);
   if(index<0||pattern.binds.some((name,at)=>at!==index&&name!==null)||!sameType(node.type,constructor.fields[index].type)||!sameType(arm.body.type,node.type))return -1;
   return index;
  }
  const counted=new WeakSet();
  function count(node,path,depth=0){
   if(!node||typeof node!=='object'||counted.has(node)||node.k==='lambda')return;counted.add(node);
   if(depth>256||++visits>1000000)throw new TypeError('constructor projection analysis exceeds its work budget');
   const index=position(node);if(index>=0){counts[index]+=1;occurrences.push({path,field:index});return;}
   for(const [key,child]of Object.entries(node))if(!excluded.has(key))count(child,[...path,key],depth+1);
  }
  count(body,['body']);
  const fields=planSingleConstructorProjections(datatype.ctors.length,counts);if(!fields.length)continue;
  const names=new Map(fields.map(index=>[index,fresh()])),memo=new WeakMap();
  function rewrite(node,depth=0){
   if(!node||typeof node!=='object'||node.k==='lambda')return node;if(memo.has(node))return memo.get(node);
   if(depth>256||++visits>1000000)throw new TypeError('constructor projection rewriting exceeds its work budget');
   const index=position(node);if(index>=0)return {k:'var',name:names.get(index),type:node.type,...(node.span?{span:node.span}:{})};
   let changed=false;const result=Array.isArray(node)?node.map(child=>{const next=rewrite(child,depth+1);changed||=next!==child;return next;}):Object.fromEntries(Object.entries(node).map(([key,child])=>{const next=excluded.has(key)?child:rewrite(child,depth+1);changed||=next!==child;return[key,next];}));
   const output=changed?result:node;memo.set(node,output);return output;
  }
  const rewritten=rewrite(body);
  body={k:'match',scrutinee:{k:'var',name:parameter.name,type:parameter.type},cases:[{pattern:{k:'ctor',data:datatype.fullName,ctor:constructor.name,binds:constructor.fields.map((_,index)=>names.get(index)??null)},body:rewritten}],type:body.type};
  proofs.push({operation:'share immutable single-constructor parameter projections',owner:entry.fullName,parameter:parameterIndex,datatype:datatype.fullName,constructor:constructor.name,fields:fields.map(index=>({index,binding:names.get(index)})),occurrences,preserveParameter:true});
 }
 return body;
}
