// Module placement only. Ownership, strict positivity and constructor/default
// witnesses remain independent requirements of the actual datatype emitter.
/** @param {string[][]} paths @param {number[]} groups @returns {number[]} */
export function planFormalDatatypeModulePrefixes(paths,groups){
 if(paths.length!==groups.length||paths.length>8192)return [];
 const depths=[];
 let total=0;
 for(let index=0;index<paths.length;index+=1){
  const path=paths[index],group=groups[index];
  if(path===undefined||path.length>256||group===undefined||!Number.isSafeInteger(group)||group<0||group>=paths.length||group===0&&1/group<0)return [];
  if(groups[group]!==group)return [];
  for(const part of path){if(part.length===0||part.length>4096)return [];total+=part.length;if(total>8388608)return [];}
  depths.push(path.length);
 }
 // A component's source-earliest representative makes placement deterministic.
 // Every member is compared with that one path, preserving an exact prefix.
 const common=paths.map(path=>path.length);
 for(let index=0;index<paths.length;index+=1){
  const group=groups[index],path=paths[index];
  if(group===undefined||path===undefined||group>index)return [];
  const representative=paths[group];if(representative===undefined)return [];
  let depth=0;
  while(depth<path.length&&depth<representative.length&&path[depth]===representative[depth])depth+=1;
  common[group]=Math.min(common[group]??0,depth);
 }
 for(let index=0;index<paths.length;index+=1)depths[index]=common[groups[index]??-1]??-1;
 return depths;
}
