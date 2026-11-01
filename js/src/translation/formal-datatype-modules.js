import {datatypeGroups} from './emit-common.js';
import {planFormalDatatypeModulePrefixes,planFormalDatatypeModuleClosure} from './formal-datatype-module-rules.js';
import {unsupported} from './diagnostics.js';

/** A target placement plan preserves nominal source identity and the original
 * checked graph. Public aliases are obligations, never silently discarded.
 * Callers must emit each alias after the corresponding complete mutual block.
 * This function alone does not admit or emit any recursive declaration. */
export function planFormalDatatypeModules(program){
 const data=[...program.declarations.values()].filter(entry=>entry.k==='data');
 const positions=new Map(data.map((entry,index)=>[entry.fullName,index]));
 if(positions.size!==data.length)throw unsupported('formal datatype modules','nominal declarations need distinct actual identities');
 const groups=data.map((_,index)=>index);
 for(const [name,members] of datatypeGroups(program)){
  if(name!==members[0]?.fullName)continue;
  const indices=members.map(entry=>positions.get(entry.fullName));
  if(indices.some(index=>index===undefined))throw unsupported('formal datatype modules','a mutual group has an unknown nominal declaration');
  const representative=Math.min(...indices);
  for(const index of indices)groups[index]=representative;
 }
 const paths=data.map(entry=>entry.modulePath);
 const depths=planFormalDatatypeModuleClosure(paths,planFormalDatatypeModulePrefixes(paths,groups));
 if(depths.length!==data.length)throw unsupported('formal datatype modules','actual module paths exceed the bounded placement profile');
 const aliases=[],declarations=new Map(program.declarations);
 for(let index=0;index<data.length;index+=1){
  const entry=data[index],depth=depths[index];
  if(depth===entry.modulePath.length)continue;
  const targetModulePath=entry.modulePath.slice(0,depth);
  declarations.set(entry.fullName,{...entry,modulePath:targetModulePath});
  aliases.push({sourceFullName:entry.fullName,targetFullName:entry.fullName,sourceName:entry.name,sourceModulePath:[...entry.modulePath],targetModulePath,constructors:entry.ctors.map(ctor=>ctor.name)});
 }
 return {sourceProgram:program,program:{...program,declarations},aliases};
}

const scopeKey=path=>path.map(part=>String(part.length)+':'+part).join('');
/** Ordering obligations are separate from source type dependencies. A module
 * containing public aliases cannot open before all their private blocks exist. */
export function formalDatatypeModuleDependencies(plan){
 const scopes=new Map();
 for(const alias of plan.aliases){
  const boundary=alias.sourceModulePath.slice(0,alias.targetModulePath.length+1),key=scopeKey(boundary);
  const owners=scopes.get(key)??new Set();owners.add(alias.targetFullName);scopes.set(key,owners);
 }
 const result=new Map();let edges=0;
 for(const entry of plan.program.declarations.values()){
  const required=new Set();
  for(let depth=1;depth<=entry.modulePath.length;depth+=1){
   const owners=scopes.get(scopeKey(entry.modulePath.slice(0,depth)));
   if(owners)for(const owner of owners){if(!plan.program.declarations.has(owner))throw unsupported('target declaration dependencies','extra ordering dependencies require actual declared owners');if(!required.has(owner)){if(++edges>131072)throw unsupported('target declaration dependencies','extra ordering dependencies require bounded actual declaration edges');required.add(owner);}}
  }
  if(required.size)result.set(entry.fullName,required);
 }
 return result;
}
