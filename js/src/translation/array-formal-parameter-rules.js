// Actual array-pattern formal syntax, independent of named record admission.
import {acceptJavaScriptSourceBindingName} from './source-identifier-rules.js';
import {readRequiredTextFact} from './fact-column-rules.js';
import {readArrayIterableForm} from './array-value-rules.js';
/** @param {string[]} kinds @param {string[]} values @param {number} start @returns {number} */
export function findJavaScriptArrayFormalPattern(kinds,values,start){
 if(kinds.length!==values.length||start<0||start%1!==0||start>=kinds.length)return -1;
 let depth=0,binding=true;
 for(let index=start;index<kinds.length;index+=1){
  const kind=readRequiredTextFact(kinds,index),value=readRequiredTextFact(values,index);
  if(kind==='eof')return -1;
  if(kind!=='punct'){if(depth===0)binding=false;continue;}
  if(depth===0&&value===')')return -1;
  if(depth===0&&binding&&value==='[')return index;
  if(value==='('||value==='['||value==='{'){depth+=1;binding=false;}
  else if(value===')'||value===']'||value==='}'){depth-=1;if(depth<0)return -1;}
  else if(depth===0)binding=value===',';
 }
 return -1;
}
/** @param {string} context @param {boolean} asynchronous @param {number} documentationParameters @returns {boolean} */
export function acceptJavaScriptArrayFormalContext(context,asynchronous,documentationParameters){
 return !asynchronous&&documentationParameters===0&&(context==='ordinary named function declaration'||context==='ordinary arrow function closure'||context==='ordinary function expression closure'||context==='ordinary object method closure'||context==='ordinary constant arrow function'||context==='ordinary constant function expression'||context==='ordinary namespace object method');
}
/** [endExclusive,count,...instructions]; ordinary [0,slot,token]; array
 * [2,slot,open,close,bindingCount,...restFlag,elementIndex,bindingToken].
 * Reasons:1 malformed,2 nested/mixed object patterns,3 default,4 invalid or
 * duplicate BoundName,5 rest position,6 unsupported formal form.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptArrayFormalParameters(kinds,values,start,reserved){
 if(kinds.length!==values.length||start<0||start%1!==0||start>=kinds.length)return [-1,start];
 let index=start,slot=0;let names=[],instructions=[];
 while(index<kinds.length&&!(readRequiredTextFact(kinds,index)==='punct'&&readRequiredTextFact(values,index)===')')){
  const kind=readRequiredTextFact(kinds,index),value=readRequiredTextFact(values,index);
  if(kind==='punct'&&value==='['){
   const open=index;index+=1;let position=0,bindings=[];
   while(index<kinds.length&&!(readRequiredTextFact(kinds,index)==='punct'&&readRequiredTextFact(values,index)===']')){
    if(readRequiredTextFact(kinds,index)==='punct'&&readRequiredTextFact(values,index)===','){position+=1;index+=1;continue;}
    const rest=readRequiredTextFact(kinds,index)==='punct'&&readRequiredTextFact(values,index)==='...';
    if(rest)index+=1;
    if(index>=kinds.length)return [-1,index];
    const token=index,name=readRequiredTextFact(values,index),tokenKind=readRequiredTextFact(kinds,index);
    if(tokenKind==='punct'&&(name==='['||name==='{'))return [-2,index];
    let duplicate=false;for(let previous=0;previous<names.length;previous+=1)if(readRequiredTextFact(names,previous)===name)duplicate=true;
    if(!acceptJavaScriptSourceBindingName(tokenKind,name,duplicate,reserved))return [-4,index];
    names=names.concat([name]);bindings=bindings.concat([rest?1:0,position,token]);index+=1;position+=1;
    if(index>=kinds.length)return [-1,index];
    const nextKind=readRequiredTextFact(kinds,index),next=readRequiredTextFact(values,index);
    if(nextKind==='punct'&&next==='=')return [-3,index];
    if(rest&&!(nextKind==='punct'&&next===']'))return [-5,index];
    if(nextKind==='punct'&&next===','){index+=1;continue;}
    if(!(nextKind==='punct'&&next===']'))return [-1,index];
   }
   if(index>=kinds.length)return [-1,index];
   instructions=instructions.concat([2,slot,open,index,bindings.length/3],bindings);index+=1;
  }else{
   if(kind==='punct'&&value==='{')return [-2,index];
   let duplicate=false;for(let previous=0;previous<names.length;previous+=1)if(readRequiredTextFact(names,previous)===value)duplicate=true;
   if(!acceptJavaScriptSourceBindingName(kind,value,duplicate,reserved))return [-6,index];
   names=names.concat([value]);instructions=instructions.concat([0,slot,index]);index+=1;
  }
  slot+=1;if(index>=kinds.length)return [-1,index];
  const delimiterKind=readRequiredTextFact(kinds,index),delimiterValue=readRequiredTextFact(values,index);
  if(delimiterKind==='punct'&&delimiterValue==='=')return [-3,index];
  if(delimiterKind==='punct'&&delimiterValue===','){index+=1;continue;}
  if(!(delimiterKind==='punct'&&delimiterValue===')'))return [-1,index];
 }
 return index<kinds.length?[index+1,slot].concat(instructions):[-1,index];
}
/** Selected source wrapper, lowered to established core operations before
 * typed archive encoding. It does not redefine ordinary for-of iteration.
 * @returns {string} */
export function readJavaScriptArrayFormalIterableOperation(){return 'readJavaScriptArrayFormalIterable';}
/** @param {string} kind @param {boolean} reference @returns {string} */
export function readJavaScriptArrayFormalIterableForm(kind,reference){return kind==='tuple'?'tuple':readArrayIterableForm(kind,reference);}
/** @param {string} kind @returns {string} */
export function readJavaScriptArrayFormalInferenceForm(kind){return kind==='var'||kind==='literal'?'wait':kind==='array'||kind==='tuple'?'identity':kind==='string'||kind==='set'||kind==='map'?'iterate':'refuse';}
