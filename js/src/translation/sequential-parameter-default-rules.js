// Actual default source references select a sequential parameter environment.
import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} node @param {string} name @returns {number} */
function field(starts,counts,labels,targets,node,name){
 if(node<0)return -1;const first=readRequiredNumberFact(starts,node),count=readRequiredNumberFact(counts,node);
 for(let at=first;at<first+count;at+=1)if(readRequiredTextFact(labels,at)===name)return readRequiredNumberFact(targets,at);return -1;
}
/** A conservative source reference census can select the stronger environment
 * for a shadowed inner name too; it never changes the original lexical binding.
 * @param {string[]} forms @param {string[]} texts @param {number[]} starts
 * @param {number[]} counts @param {string[]} labels @param {number[]} targets
 * @param {string[]} names @returns {boolean} */
export function containsParameterDefaultNameReference(forms,texts,starts,counts,labels,targets,names){
 for(let node=0;node<forms.length;node+=1){
  if(readRequiredTextFact(forms,node)!=='object')continue;
  const kind=field(starts,counts,labels,targets,node,'k');if(kind<0||readRequiredTextFact(texts,kind)!=='name')continue;
  const path=field(starts,counts,labels,targets,node,'path');if(path<0||readRequiredNumberFact(counts,path)!==1)continue;
  const name=readRequiredNumberFact(targets,readRequiredNumberFact(starts,path));
  if(names.includes(readRequiredTextFact(texts,name)))return true;
 }
 return false;
}
/** @param {boolean[]} defaults @param {boolean} references @returns {boolean} */
export function requireSequentialParameterInitialization(defaults,references){
 if(references)return true;let seen=false;
 for(let at=0;at<defaults.length;at+=1){if(readRequiredBooleanFact(defaults,at))seen=true;else if(seen)return true;}
 return false;
}
/** Every supplied argument precedes every parameter initializer. Opcodes zero
 * and one name actual argument and parameter slots respectively.
 * @param {number} parameters @param {number} provided @returns {number[]} */
export function planSequentialParameterInitialization(parameters,provided){
 if(!Number.isSafeInteger(parameters)||!Number.isSafeInteger(provided)||parameters<1||parameters>1024||provided<0||provided>parameters)return [];
 let result=[];for(let at=0;at<provided;at+=1)result=result.concat([0,at]);for(let at=0;at<parameters;at+=1)result=result.concat([1,at]);return result;
}
/** Actual ordinary formal binder positions; nested initializer tokens remain
 * opaque to the binder census. The ordinary parser still owns syntax admission.
 * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number[]} */
export function planOrdinaryParameterBindingPositions(kinds,values,start){
 if(kinds.length!==values.length||!Number.isSafeInteger(start)||start<0)return [];
 let positions=[],closes=[],binding=true;
 for(let at=start;at<kinds.length;at+=1){const kind=readRequiredTextFact(kinds,at),value=readRequiredTextFact(values,at);
  if(!closes.length&&kind==='punct'&&value===')')return [at+1].concat(positions);
  if(!closes.length&&binding&&kind==='identifier'){positions=positions.concat([at]);binding=false;}
  if(kind!=='punct')continue;
  if(value==='('||value==='['||value==='{')closes=closes.concat([value==='('?')':value==='['?']':'}']);
  else if(value===')'||value===']'||value==='}'){
   if(!closes.length||readRequiredTextFact(closes,closes.length-1)!==value)return [];closes=closes.slice(0,closes.length-1);
  }else if(!closes.length&&value===',')binding=true;
 }
 return [];
}
