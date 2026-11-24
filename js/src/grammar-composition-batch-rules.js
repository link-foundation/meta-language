import {readRequiredTextFact} from './translation/fact-column-rules.js';
/** @param {string[]} values @param {string} key @returns {boolean} */
function containsSorted(values,key){
 let first=0,last=values.length;
 while(first<last){const middle=Math.floor((first+last)/2),value=readRequiredTextFact(values,middle);if(value===key)return true;if(value<key)first=middle+1;else last=middle;}
 return false;
}
/** Exact source/rule pairs have an unambiguous delimiter after admission.
 * @param {string} value @returns {boolean} */
function validName(value){return value!==''&&!value.includes('\0');}
/** @param {number} sources @param {number} rules @param {number} policies @returns {string} */
export function readGrammarCompositionBatchCountIssue(sources,rules,policies){return !Number.isInteger(sources)||!Number.isInteger(rules)||!Number.isInteger(policies)||sources<1||sources>4096||rules<0||rules>65536||policies<0||policies>4096?'source, rule or policy count exceeds the batch limit':'';}
/** @param {number} units @returns {string} */
export function readGrammarCompositionBatchTextIssue(units){return !Number.isInteger(units)||units<0||units>8388608?'batch text exceeds the source unit limit':'';}
/** Policy binding is shared; actual grammar composition remains with the
 * existing decorator engine. Sorted directories avoid per-policy full scans.
 * @param {string[]} sources @param {string[]} owners @param {string[]} names
 * @param {string[]} kinds @param {string[]} policySources
 * @param {string[]} policyRules @param {string[]} compositions @returns {string} */
export function readGrammarCompositionBatchIssue(sources,owners,names,kinds,policySources,policyRules,compositions){
 const countIssue=readGrammarCompositionBatchCountIssue(sources.length,names.length,policySources.length);if(countIssue)return countIssue;
 if(owners.length!==names.length||kinds.length!==names.length||policySources.length!==policyRules.length||policySources.length!==compositions.length)return 'unaligned batch columns';
 let units=0;
 for(let index=0;index<sources.length;index+=1){units+=readRequiredTextFact(sources,index).length;const issue=readGrammarCompositionBatchTextIssue(units);if(issue)return issue;}
 const sortedSources=sources.slice().toSorted();
 for(let index=0;index<sortedSources.length;index+=1){const name=readRequiredTextFact(sortedSources,index);if(!validName(name)||index>0&&name===readRequiredTextFact(sortedSources,index-1))return 'invalid or duplicate source identity';}
 const keys=[],normal=[];
 for(let index=0;index<names.length;index+=1){
  const owner=readRequiredTextFact(owners,index),name=readRequiredTextFact(names,index),kind=readRequiredTextFact(kinds,index);
  if(!validName(name)||!containsSorted(sortedSources,owner))return 'a rule needs its exact existing source owner and name';
  const key=owner+'\0'+name;keys.push(key);if(kind==='normal')normal.push(key);units+=key.length;
  if(units>8388608)return 'batch text exceeds the source unit limit';
 }
 const sortedKeys=keys.slice().toSorted(),normalKeys=normal.slice().toSorted();
 for(let index=1;index<sortedKeys.length;index+=1)if(readRequiredTextFact(sortedKeys,index)===readRequiredTextFact(sortedKeys,index-1))return 'duplicate source-qualified rule';
 const targets=[];
 for(let index=0;index<policySources.length;index+=1){
  const source=readRequiredTextFact(policySources,index),rule=readRequiredTextFact(policyRules,index),composition=readRequiredTextFact(compositions,index);
  if(!validName(source)||!validName(rule)||composition==='')return 'policies require exact source and rule names and nonempty Links composition';
  const key=source+'\0'+rule;
  if(!containsSorted(sortedKeys,key))return 'a policy target needs an existing source-qualified rule';
  if(!containsSorted(normalKeys,key))return 'a policy target needs a normal syntactic rule';
  targets.push(key);units+=key.length+composition.length;if(units>8388608)return 'batch text exceeds the source unit limit';
 }
 const sortedTargets=targets.slice().toSorted();
 for(let index=1;index<sortedTargets.length;index+=1)if(readRequiredTextFact(sortedTargets,index)===readRequiredTextFact(sortedTargets,index-1))return 'multiple policies target the same source-qualified rule';
 return '';
}
/** @param {string} role @returns {string} */
export function readGrammarCompositionBatchFieldOrder(role){
 if(role==='policy')return 'source|rule|composition';
 if(role==='configuration')return 'schemaVersion|model|policies';
 return '';
}
/** @param {string} role @returns {string} */
export function readGrammarCompositionBatchEntryName(role){
 if(role==='original')return 'grammar-composition-original-definitions';
 if(role==='current')return 'grammar-composition-current-definitions';
 if(role==='configuration')return 'grammar-composition-batch-configuration';
 if(role==='algorithm')return 'grammar-composition-batch-algorithm';
 return '';
}
/** @param {number} version @param {string} model @returns {boolean} */
export function acceptGrammarCompositionBatchConfiguration(version,model){return version===1&&model==='source-qualified grammar composition batch 1';}
