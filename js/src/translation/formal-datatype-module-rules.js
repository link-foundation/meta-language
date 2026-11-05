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

/** Protect each original module that would otherwise open before its lifted
 * mutual block is ready. All datatype declarations under that boundary move
 * together; public aliases keep their actual original scopes.
 * @param {string[][]} paths @param {number[]} depths @returns {number[]} */
export function planFormalDatatypeModuleClosure(paths,depths){
 if(paths.length!==depths.length||paths.length>8192)return [];
 /** @type {Map<string,number>} */
 const boundaries=new Map();
 let budget=0;
 for(let index=0;index<paths.length;index+=1){
  const path=paths[index],depth=depths[index];
  if(path===undefined||path.length>256||depth===undefined||!Number.isSafeInteger(depth)||depth<0||depth>path.length||depth===0&&1/depth<0)return [];
  let key='';
  for(let part=0;part<path.length;part+=1){
   const value=path[part];if(value===undefined||value.length===0||value.length>4096)return [];
   budget+=value.length+1;if(budget>8388608)return [];
   key+=String(value.length)+':'+value;
   if(part===depth){const previous=boundaries.get(key);boundaries.set(key,previous===undefined?depth:Math.min(previous,depth));}
  }
 }
 const result=[];
 for(let index=0;index<paths.length;index+=1){
  let selected=depths[index]??-1,key='';
  for(const value of paths[index]??[]){key+=String(value.length)+':'+value;const protectedDepth=boundaries.get(key);if(protectedDepth!==undefined)selected=Math.min(selected,protectedDepth);}
  result.push(selected);
 }
 return result;
}
