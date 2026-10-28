import {shareSourceLexicalAliases} from './source-lexical-aliases.mjs';
// Source construction only. The shared finite-inventory rule supplies the
// proof; this adapter transports actual checked owners and occurrence paths.
import {planBooleanConstructorMatchSharing,planClosedConstructorMatchSharing,planUnusedConstructorBindings,testUnusedConstructorAlias} from '../src/translation/boolean-constructor-match-rules.js';
import {sameType,typeKey} from '../src/translation/types.js';
import {readProjectData} from './project-data.mjs';
import {shareSourceConstructorProjections,shareSourceScopedConstructorProjections} from './source-constructor-projections.mjs';
const root=new URL('../../',import.meta.url);
export function shareSourceBooleanConstructorMatches(program){
 return shareSourceConstructorMatches(program,false);
}
export function shareSourceClosedConstructorMatches(program){return shareSourceConstructorMatches(program,true);}
function shareSourceConstructorMatches(program,closed){
 if(closed&&program.sourceLanguage!=='JavaScript')throw new TypeError('closed constructor sharing requires the checked JavaScript value model; ownership-sensitive source models need their own proof');
 const rule=readProjectData(new URL('parity/self-translation/source-'+(closed?'closed':'boolean')+'-match-sharing.lino',root));
 if(rule.schemaVersion!==1||rule.model!==(closed?'closed constructor match sharing 1':'literal Boolean constructor match arms 1')||rule.preserveScrutinee!==true||rule.requireCompleteOwner!==true||closed&&(rule.pruneUnusedBindings!==true||rule.pruneUnusedAliases!==true||rule.shareSingleConstructorProjections!==true||rule.shareScopedConstructorProjections!==true||rule.shareImmutableLexicalAliases!==true||rule.shareLiteralText!==true||rule.shareLiteralScalar!==true))throw new TypeError('actual qualified constructor match sharing rules are required');
 const memo=new WeakMap(),active=new WeakSet(),referenceCache=new WeakMap(),proofs=[];let visits=0,referenceVisits=0;
 function references(body){
  if(body&&typeof body==='object'&&referenceCache.has(body))return referenceCache.get(body);
  const names=new Set(),seen=new WeakSet(),pending=[body];
  while(pending.length){
   const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);
   if(++referenceVisits>1000000)throw new TypeError('source binding references exceed their structural budget');
   if(value.k==='var'&&typeof value.name==='string')names.add(value.name);
   for(const [key,child]of Object.entries(value))if(!['type','domain','from','to','span','pattern'].includes(key))pending.push(child);
  }
  const result=[...names];if(body&&typeof body==='object')referenceCache.set(body,result);return result;
 }
 function aliases(body,available,owner,path,depth=0){
  if(depth>256)throw new TypeError('constructor alias prefixes exceed their structural budget');
  if(body?.k==='if'){
   const yes=aliases(body.then,available,owner,[...path,'then'],depth+1),no=aliases(body.else,available,owner,[...path,'else'],depth+1);
   return yes===body.then&&no===body.else?body:{...body,then:yes,else:no};
  }
  if(body?.k!=='let'||typeof body.name!=='string'||body.name==='')return body;
  const scope=new Set(available);scope.add(body.name);
  const next=aliases(body.body,scope,owner,[...path,'body'],depth+1);
  if(body.value?.k==='var'&&testUnusedConstructorAlias(body.name,body.value.name,[...available],references(next))){
   proofs.push({operation:'remove unused constructor alias',owner,path,name:body.name,value:body.value.name,preserveBody:true,preserveScrutinee:true});
   return next;
  }
  return next===body.body?body:{...body,body:next};
 }
 function rewrite(value,path,owner,depth=0){
  if(!value||typeof value!=='object')return value;
  if(active.has(value))throw new TypeError('source match sharing refuses structural cycles');
  if(memo.has(value))return memo.get(value);
  if(depth>256||++visits>100000)throw new TypeError('source match sharing exceeds its structural budget');
  active.add(value);let changed=false;
  const result=Array.isArray(value)?value.map((child,index)=>{const next=rewrite(child,[...path,index],owner,depth+1);changed||=next!==child;return next;}):Object.fromEntries(Object.entries(value).map(([key,child])=>{
   const next=['type','domain','from','to','span','pattern'].includes(key)?child:rewrite(child,[...path,key],owner,depth+1);changed||=next!==child;return [key,next];
  }));
  let output=changed?result:value;
  if(closed&&program.sourceLanguage==='JavaScript'&&output.k==='match'&&output.scrutinee?.type?.kind==='data'&&Array.isArray(output.cases)){
   const datatype=program.declarations.get(output.scrutinee.type.name);
   if(datatype?.k==='data'){
    let pruned=false;
    const cases=output.cases.map((arm,index)=>{
     const pattern=arm.pattern,constructor=datatype.ctors.find(ctor=>ctor.name===pattern?.ctor);
     if(pattern?.k!=='ctor'||pattern.data!==datatype.fullName||!constructor||!Array.isArray(pattern.binds)||pattern.binds.length!==constructor.fields.length||pattern.binds.some(name=>name!==null&&(typeof name!=='string'||name==='')))return arm;
     const bindings=pattern.binds.filter(name=>name!==null);if(new Set(bindings).size!==bindings.length)return arm;
     const available=new Set([...(program.declarations.get(owner)?.params??[]).map(parameter=>parameter.name),...bindings]);
     if(output.scrutinee.k==='var')available.add(output.scrutinee.name);
     const body=aliases(arm.body,available,owner,[...path,'cases',index,'body']);
     const removed=planUnusedConstructorBindings(pattern.binds.map(name=>name??''),references(body));
     if(!removed.length){if(body===arm.body)return arm;pruned=true;return {...arm,body};}
     const positions=new Set(removed);pruned=true;
     proofs.push({operation:'remove unused constructor field bindings',owner,path:[...path,'cases',index],datatype:datatype.fullName,constructor:constructor.name,removed:removed.map(position=>({position,name:pattern.binds[position]})),preserveScrutinee:true,preserveBody:true});
     return {...arm,body,pattern:{...pattern,binds:pattern.binds.map((name,position)=>positions.has(position)?null:name)}};
    });
    if(pruned)output={...output,cases};
   }
  }
  if(output.k==='match'&&(closed||output.type?.kind==='bool')&&output.scrutinee?.type?.kind==='data'&&Array.isArray(output.cases)){
   const datatype=program.declarations.get(output.scrutinee.type.name);
   const cases=output.cases;
   if(datatype?.k==='data'&&cases.every(arm=>arm.pattern?.k==='ctor'&&Array.isArray(arm.pattern.binds)&&Object.keys(arm).every(key=>['pattern','body','span'].includes(key))&&Object.keys(arm.pattern).every(key=>['k','data','ctor','binds','span'].includes(key))&&arm.body?.type&&sameType(arm.body.type,output.type))){
    const shapes=[],outcomes=cases.map(arm=>{
     const body=arm.body;
     if(body.k==='lit'&&body.type.kind==='bool'&&typeof body.value==='boolean'&&Object.keys(body).every(key=>['k','type','value','span'].includes(key)))return {kind:'literal Boolean',value:body.value};
     if(closed&&body.k==='lit'&&body.type.kind==='string'&&typeof body.value==='string'&&Object.keys(body).every(key=>['k','type','value','span'].includes(key)))return {kind:'literal text',value:body.value};
     if(closed&&body.k==='lit'&&['float','int','nat'].includes(body.type.kind)&&typeof body.value==='string'&&Object.keys(body).every(key=>['k','type','value','span'].includes(key)))return {kind:'literal scalar',type:typeKey(body.type),value:body.value};
     if(closed&&body.k==='abort'&&typeof body.message==='string'&&Object.keys(body).every(key=>['k','type','message','span'].includes(key)))return {kind:'abort',message:body.message,type:typeKey(body.type)};
     return null;
    });
    const classes=outcomes.map(outcome=>{if(outcome===null)return -1;const shape=JSON.stringify(outcome);let index=shapes.indexOf(shape);if(index<0){index=shapes.length;shapes.push(shape);}return index;});
    const declared=datatype.ctors.map(ctor=>ctor.name),owners=cases.map(arm=>arm.pattern.data),constructors=cases.map(arm=>arm.pattern.ctor);
    const plan=closed?planClosedConstructorMatchSharing(datatype.fullName,declared,owners,constructors,classes):outcomes.every(outcome=>outcome?.kind==='literal Boolean')?planBooleanConstructorMatchSharing(datatype.fullName,declared,owners,constructors,cases.map(arm=>arm.body.value?1:0)):[];
    if(plan.length){
     const fallback=closed?outcomes[classes.indexOf(plan[0])]:plan[0]===1,indices=plan.slice(1),body=closed?cases[classes.indexOf(plan[0])].body:cases.find(arm=>arm.body.value===fallback).body;
     output={...output,cases:[...indices.map(index=>cases[index]),{pattern:{k:'wild'},body}]};
     proofs.push({owner,path,datatype:datatype.fullName,declared,original:cases.map((arm,index)=>({pattern:arm.pattern,...(closed?{outcome:outcomes[index]}:{literal:arm.body.value})})),retainedIndices:indices,fallback,preserveScrutinee:true});
    }
   }
  }
  active.delete(value);memo.set(value,output);return output;
 }
 const declarations=new Map();
 for(const [name,entry]of program.declarations){
  let body=entry.k==='fn'?rewrite(entry.body,['body'],name):entry.body;
  if(closed&&entry.k==='fn'){body=shareSourceLexicalAliases(program,entry,body,proofs);body=shareSourceScopedConstructorProjections(program,entry,body,proofs);body=shareSourceConstructorProjections(program,entry,body,proofs);}
  declarations.set(name,body===entry.body?entry:{...entry,body});
 }
 function items(values){return values.map(item=>{
  if(item.k==='module')return {...item,items:items(item.items)};
  if(program.declarations.get(item.fullName)!==item)throw new TypeError('source match sharing needs actual declaration owners');
  return declarations.get(item.fullName);
 });}
 return {program:{...program,declarations,items:items(program.items)},proof:{model:rule.model,proofs}};
}
