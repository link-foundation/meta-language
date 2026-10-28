// Source construction only. The shared finite-inventory rule supplies the
// proof; this adapter transports actual checked owners and occurrence paths.
import {planBooleanConstructorMatchSharing} from '../src/translation/boolean-constructor-match-rules.js';
import {readProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
export function shareSourceBooleanConstructorMatches(program){
 const rule=readProjectData(new URL('parity/self-translation/source-boolean-match-sharing.lino',root));
 if(rule.schemaVersion!==1||rule.model!=='literal Boolean constructor match arms 1'||rule.preserveScrutinee!==true||rule.requireCompleteOwner!==true)throw new TypeError('actual qualified Boolean match sharing rules are required');
 const memo=new WeakMap(),active=new WeakSet(),proofs=[];let visits=0;
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
  if(output.k==='match'&&output.type?.kind==='bool'&&output.scrutinee?.type?.kind==='data'&&Array.isArray(output.cases)){
   const datatype=program.declarations.get(output.scrutinee.type.name);
   const cases=output.cases;
   if(datatype?.k==='data'&&cases.every(arm=>arm.pattern?.k==='ctor'&&Array.isArray(arm.pattern.binds)&&arm.body?.k==='lit'&&arm.body.type?.kind==='bool'&&typeof arm.body.value==='boolean')){
    const plan=planBooleanConstructorMatchSharing(datatype.fullName,datatype.ctors.map(ctor=>ctor.name),cases.map(arm=>arm.pattern.data),cases.map(arm=>arm.pattern.ctor),cases.map(arm=>arm.body.value?1:0));
    if(plan.length){
     const fallback=plan[0]===1,indices=plan.slice(1),body=cases.find(arm=>arm.body.value===fallback).body;
     output={...output,cases:[...indices.map(index=>cases[index]),{pattern:{k:'wild'},body}]};
     proofs.push({owner,path,datatype:datatype.fullName,declared:datatype.ctors.map(ctor=>ctor.name),original:cases.map(arm=>({pattern:arm.pattern,literal:arm.body.value})),retainedIndices:indices,fallback,preserveScrutinee:true});
    }
   }
  }
  active.delete(value);memo.set(value,output);return output;
 }
 const declarations=new Map();
 for(const [name,entry]of program.declarations){
  const body=entry.k==='fn'?rewrite(entry.body,['body'],name):entry.body;
  declarations.set(name,body===entry.body?entry:{...entry,body});
 }
 function items(values){return values.map(item=>{
  if(item.k==='module')return {...item,items:items(item.items)};
  if(program.declarations.get(item.fullName)!==item)throw new TypeError('source match sharing needs actual declaration owners');
  return declarations.get(item.fullName);
 });}
 return {program:{...program,declarations,items:items(program.items)},proof:{model:rule.model,proofs}};
}
