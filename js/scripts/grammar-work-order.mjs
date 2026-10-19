import {readProjectData} from './project-data.mjs';

/** The shared authoring policy changes work order, never grammar coverage. */
export function readGrammarWorkPriorities(){
 const policy=readProjectData(new URL('../../parity/grammar-work-priorities.lino',import.meta.url));
 if(policy.schemaVersion!==1)throw new TypeError('Unknown grammar work priority schema');
 return policy.priorityLanguages;
}

/** Return every original index once, preserving catalog order within a rank. */
export function planGrammarWorkOrder(languages,priorities=readGrammarWorkPriorities()){
 if(!Array.isArray(languages)||!Array.isArray(priorities))throw new TypeError('Grammar work languages and priorities must be arrays');
 const rank=new Map();
 for(const name of priorities){
  if(typeof name!=='string'||name.length===0||rank.has(name))throw new TypeError('Grammar work priorities must be distinct nonempty language names');
  rank.set(name,rank.size);
 }
 return languages.map((entry,index)=>({index,rank:rank.get(entry.name)??rank.size}))
  .sort((left,right)=>left.rank-right.rank||left.index-right.index).map(entry=>entry.index);
}
