import {datatypeGroups} from './emit-common.js';
import {planFormalDatatypeModulePrefixes} from './formal-datatype-module-rules.js';
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
 const depths=planFormalDatatypeModulePrefixes(data.map(entry=>entry.modulePath),groups);
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
