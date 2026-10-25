// Journal vocabulary and bounds are shared source, not checkpoint success claims.
import {readRequiredNumberFact,readRequiredTextFact} from './translation/fact-column-rules.js';
/** @param {number} count @returns {boolean} */
export function acceptRegularGrammarRefinementCount(count){return count>=0&&count<=8&&count%1===0;}
/** @param {number} version @param {string} model @returns {boolean} */
export function acceptRegularGrammarRefinementJournal(version,model){return version===1&&model==='labelled regular grammar refinement 1';}
/** @param {number} ordinal @param {string} role @returns {string} */
export function readRegularGrammarRefinementEntryName(ordinal,role){return 'regular-discovery-history-'+String(ordinal)+'-'+role;}
/** @param {string} previous @param {number} ordinal @returns {string} */
export function readRegularGrammarRefinementIdentifier(previous,ordinal){return previous+'@refinement-'+String(ordinal);}
/** @param {string} role @returns {number} */
export function readRegularGrammarRefinementResourceLimit(role){return role==='nodes'?1000000:role==='references'?4000000:role==='bytes'?134217728:0;}
/** @param {string[]} labels @param {number[]} starts @param {number[]} counts
 * @param {number[]} targets @param {number} node @param {string} name @returns {number} */
function refinementField(labels,starts,counts,targets,node,name){
 if(node<0||node>=starts.length||node>=counts.length)return -1;
 const start=readRequiredNumberFact(starts,node),count=readRequiredNumberFact(counts,node);
 for(let at=start;at<start+count;at+=1)if(readRequiredTextFact(labels,at)===name)return readRequiredNumberFact(targets,at);
 return -1;
}
/** Exact ordinary data equality ignores object insertion order but preserves
 * array positions, all scalar forms and every field. Host callbacks are absent.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} first @param {number} second @returns {boolean} */
function sameRefinementValues(forms,texts,starts,counts,labels,targets,first,second){
 let left=[first],right=[second];
 for(let index=0;index<left.length;index+=1){
  if(index>=100000||left.length>100000)return false;
  const one=readRequiredNumberFact(left,index),other=readRequiredNumberFact(right,index);
  if(one<0||other<0||one>=forms.length||other>=forms.length)return false;
  const form=readRequiredTextFact(forms,one);
  if(form!==readRequiredTextFact(forms,other)||readRequiredTextFact(texts,one)!==readRequiredTextFact(texts,other)||readRequiredNumberFact(counts,one)!==readRequiredNumberFact(counts,other))return false;
  const start=readRequiredNumberFact(starts,one),count=readRequiredNumberFact(counts,one);
  for(let at=start;at<start+count;at+=1){
   const label=readRequiredTextFact(labels,at),target=refinementField(labels,starts,counts,targets,other,label);
   if(target<0)return false;
   left=left.concat([readRequiredNumberFact(targets,at)]);right=right.concat([target]);
  }
 }
 return true;
}
/** @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number} before @param {number} added @param {number} after @returns {boolean} */
function appendedRefinementValues(forms,texts,starts,counts,labels,targets,before,added,after){
 if(before<0||added<0||after<0)return false;
 if(readRequiredTextFact(forms,before)!=='array'||readRequiredTextFact(forms,added)!=='array'||readRequiredTextFact(forms,after)!=='array')return false;
 const previous=readRequiredNumberFact(counts,before),extra=readRequiredNumberFact(counts,added);
 if(previous+extra!==readRequiredNumberFact(counts,after))return false;
 for(let index=0;index<previous+extra;index+=1){
  const source=index<previous?before:added,position=index<previous?index:index-previous;
  const first=refinementField(labels,starts,counts,targets,source,String(position)),second=refinementField(labels,starts,counts,targets,after,String(index));
  if(!sameRefinementValues(forms,texts,starts,counts,labels,targets,first,second))return false;
 }
 return true;
}
/** Complete journal continuity decisions over actual model/step data facts.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {number[]} models @param {number[]} steps @returns {number} */
export function validateRegularGrammarRefinementHistory(forms,texts,starts,counts,labels,targets,models,steps){
 if(!acceptRegularGrammarRefinementCount(steps.length)||models.length!==steps.length+1)return -1;
 for(let index=0;index<models.length;index+=1){
  const current=readRequiredNumberFact(models,index);
  if(current<0||readRequiredTextFact(forms,current)!=='object'||readRequiredNumberFact(counts,current)!==10)return -6;
  for(const field of ['schemaVersion','algorithm','identifier','language','edition','options','positives','negatives','alphabet','table'])if(refinementField(labels,starts,counts,targets,current,field)<0)return -6;
  const options=refinementField(labels,starts,counts,targets,current,'options');
  if(readRequiredTextFact(forms,options)!=='object'||readRequiredNumberFact(counts,options)!==7)return -6;
  for(const field of ['strategy','alpha','stateLimit','pairLimit','attemptLimit','symbolLimit','byteLimit'])if(refinementField(labels,starts,counts,targets,options,field)<0)return -6;
  const identity=refinementField(labels,starts,counts,targets,current,'identifier');
  if(identity<0||readRequiredTextFact(forms,identity)!=='string')return -2;
  for(let previous=0;previous<index;previous+=1){const other=refinementField(labels,starts,counts,targets,readRequiredNumberFact(models,previous),'identifier');if(readRequiredTextFact(texts,other)===readRequiredTextFact(texts,identity))return -2;}
  if(index===0)continue;
  const before=readRequiredNumberFact(models,index-1),step=readRequiredNumberFact(steps,index-1);
  if(step<0||readRequiredTextFact(forms,step)!=='object'||readRequiredNumberFact(counts,step)!==4)return -5;
  for(const field of ['identifier','nextIdentifier','positives','negatives'])if(refinementField(labels,starts,counts,targets,step,field)<0)return -5;
  for(const field of ['language','edition','options','algorithm','schemaVersion'])if(!sameRefinementValues(forms,texts,starts,counts,labels,targets,refinementField(labels,starts,counts,targets,before,field),refinementField(labels,starts,counts,targets,current,field)))return -3;
  if(!sameRefinementValues(forms,texts,starts,counts,labels,targets,refinementField(labels,starts,counts,targets,before,'identifier'),refinementField(labels,starts,counts,targets,step,'identifier'))||!sameRefinementValues(forms,texts,starts,counts,labels,targets,identity,refinementField(labels,starts,counts,targets,step,'nextIdentifier')))return -2;
  for(const field of ['positives','negatives'])if(!appendedRefinementValues(forms,texts,starts,counts,labels,targets,refinementField(labels,starts,counts,targets,before,field),refinementField(labels,starts,counts,targets,step,field),refinementField(labels,starts,counts,targets,current,field)))return -4;
 }
 return 1;
}
/** @param {string} identifier @param {string[]} existing @returns {boolean} */
export function acceptRegularGrammarRefinementIdentifier(identifier,existing){
 if(identifier==='')return false;
 for(let index=0;index<existing.length;index+=1)if(readRequiredTextFact(existing,index)===identifier)return false;
 return true;
}
