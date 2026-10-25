// Actual array-pattern formal syntax, independent of named record admission.
import {acceptJavaScriptSourceBindingName} from './source-identifier-rules.js';
import {readRequiredTextFact,readRequiredNumberFact} from './fact-column-rules.js';
import {readArrayIterableForm} from './array-value-rules.js';
/** @param {string[]} kinds @param {string[]} values @param {number} start @returns {number} */
export function findJavaScriptArrayFormalPattern(kinds,values,start){
 if(kinds.length!==values.length||start<0||start%1!==0||start>=kinds.length)return -1;
 let depth=0,binding=true;
 for(let index=start;index<kinds.length;index+=1){
  const kind=readRequiredTextFact(kinds,index);
  if(kind==='eof')return -1;
  if(kind!=='punct'){if(depth===0)binding=false;continue;}
  const value=readRequiredTextFact(values,index);
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
/** Actual nested closures retain closure captures; named/flattened callables
 * retain their existing function boundary. @param {string} context @returns {number} */
export function readJavaScriptArrayFormalDefaultScopeBoundary(context){return context==='ordinary arrow function closure'||context==='ordinary function expression closure'||context==='ordinary object method closure'?1:2;}
/** [endExclusive,count,...instructions]; ordinary [0,slot,token]; array
 * [2,slot,open,close,bindingCount,...restFlag,elementIndex,bindingToken].
 * Reasons:1 malformed,2 nested/mixed object patterns,3 default,4 invalid or
 * duplicate BoundName,5 rest position,6 unsupported formal form.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptArrayFormalParameters(kinds,values,start,reserved){
 return planJavaScriptArrayFormalParameterBindings(kinds,values,start,reserved,false);
}
/** Separate source profile retains two actual initializer interval columns on
 * every array binding; -1,-1 denotes no initializer. The original operation
 * above retains its exact required-only syntax and failure contract.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptArrayFormalParametersWithDefaults(kinds,values,start,reserved){
 return planJavaScriptArrayFormalParameterBindings(kinds,values,start,reserved,true);
}
/** @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {boolean} defaults @returns {number[]} */
function planJavaScriptArrayFormalParameterBindings(kinds,values,start,reserved,defaults){
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
    names=names.concat([name]);index+=1;position+=1;let defaultStart=-1,defaultEnd=-1;
    if(index>=kinds.length)return [-1,index];
    if(readRequiredTextFact(kinds,index)==='punct'&&readRequiredTextFact(values,index)==='='){
     if(!defaults)return [-3,index];if(rest)return [-5,index];defaultStart=index+1;
     defaultEnd=findJavaScriptFormalDefaultExpressionEnd(kinds,values,defaultStart);if(defaultEnd<0)return [-1,defaultStart];index=defaultEnd;
    }
    bindings=bindings.concat(defaults?[rest?1:0,position-1,token,defaultStart,defaultEnd]:[rest?1:0,position-1,token]);
    const nextKind=readRequiredTextFact(kinds,index),next=readRequiredTextFact(values,index);
    if(nextKind==='punct'&&next==='=')return [-3,index];
    if(rest&&!(nextKind==='punct'&&next===']'))return [-5,index];
    if(nextKind==='punct'&&next===','){index+=1;continue;}
    if(!(nextKind==='punct'&&next===']'))return [-1,index];
   }
   if(index>=kinds.length)return [-1,index];
   instructions=instructions.concat([2,slot,open,index,bindings.length/(defaults?5:3)],bindings);index+=1;
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
/** Complete opaque tokens retain their contents; punctuation nesting determines
 * the AssignmentExpression boundary only. The ordinary expression parser must
 * subsequently consume exactly this original interval.
 * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number} */
export function findJavaScriptFormalDefaultExpressionEnd(kinds,values,start){
 if(kinds.length!==values.length||start<0||start%1!==0||start>=kinds.length)return -1;
 let stack=[];
 for(let index=start;index<kinds.length;index+=1){
  const kind=readRequiredTextFact(kinds,index);if(kind==='eof')return -1;
  if(kind!=='punct')continue;
  const value=readRequiredTextFact(values,index);
  if(stack.length===0&&(value===','||value===']'))return index===start?-1:index;
  if(value==='('||value==='['||value==='{')stack=stack.concat([value==='('?')':value==='['?']':'}']);
  else if(value===')'||value===']'||value==='}'){
   if(stack.length===0||readRequiredTextFact(stack,stack.length-1)!==value)return -1;
   stack=stack.slice(0,-1);
  }
 }
 return -1;
}
/** BoundName positions in the separate five-column default profile.
 * @param {number[]} plan @returns {number[]} */
export function readJavaScriptArrayFormalDefaultBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)throw new RangeError('an array default binding census needs a complete admitted plan');
 let cursor=2,names=[];
 while(cursor<plan.length){
  const form=readRequiredNumberFact(plan,cursor);cursor+=2;
  if(form===0){names=names.concat([readRequiredNumberFact(plan,cursor)]);cursor+=1;}
  else if(form===2){const count=readRequiredNumberFact(plan,cursor+2);cursor+=3;for(let index=0;index<count;index+=1){names=names.concat([readRequiredNumberFact(plan,cursor+2)]);cursor+=5;}}
  else throw new RangeError('an array default binding census needs actual formal instructions');
 }
 return names;
}
/** The original admitted plan supplies ordered BoundNames. An initializer can
 * read earlier initialized parameters; its own and later names remain in the
 * parameter environment's temporal dead zone. Result [availableCount,...earlier,
 * ...uninitialized] contains original token positions, never invented names.
 * @param {number[]} plan @param {number} binding @returns {number[]} */
export function partitionJavaScriptArrayFormalInitializerBindings(plan,binding){
 const positions=readJavaScriptArrayFormalDefaultBindingPositions(plan);
 let available=[],uninitialized=[],found=false;
 for(let index=0;index<positions.length;index+=1){
  const position=readRequiredNumberFact(positions,index);
  if(position===binding)found=true;
  if(position<binding)available=available.concat([position]);
  else uninitialized=uninitialized.concat([position]);
 }
 if(!found)throw new RangeError('an initializer needs its actual admitted BoundName position');
 return [available.length].concat(available,uninitialized);
}
/** This source profile supports earlier reads, not writes to the parameter
 * environment during initialization. The host supplies actual assignment
 * observations, including nested source bodies, without guessing from tokens.
 * @param {string[]} assignments @param {string[]} parameters @returns {number} */
export function findJavaScriptArrayFormalInitializerAssignment(assignments,parameters){
 for(let index=0;index<assignments.length;index+=1)
  for(let parameter=0;parameter<parameters.length;parameter+=1)
   if(readRequiredTextFact(assignments,index)===readRequiredTextFact(parameters,parameter))return index;
 return -1;
}
/** Selected source wrapper, lowered to established core operations before
 * typed archive encoding. It does not redefine ordinary for-of iteration.
 * @returns {string} */
export function readJavaScriptArrayFormalIterableOperation(){return 'readJavaScriptArrayFormalIterable';}
/** @param {string} kind @param {boolean} reference @returns {string} */
export function readJavaScriptArrayFormalIterableForm(kind,reference){return kind==='tuple'?'tuple':readArrayIterableForm(kind,reference);}
/** @param {string} kind @returns {string} */
export function readJavaScriptArrayFormalInferenceForm(kind){return kind==='var'||kind==='literal'?'wait':kind==='array'||kind==='tuple'?'identity':kind==='string'||kind==='set'||kind==='map'?'iterate':'refuse';}
