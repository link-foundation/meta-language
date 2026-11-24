// Source coordinates remain available even when a pattern binds no names.
import {readRequiredNumberFact} from './fact-column-rules.js';
import {readJavaScriptRecursiveObjectBindingPositions} from './recursive-binding-rules.js';
import {partitionJavaScriptFormalInitializerBindingPositions} from './array-formal-parameter-rules.js';
/** Actual binder or pattern opening; never a fabricated BoundName.
 * @param {number[]} plan @param {number} row @returns {number} */
export function readJavaScriptPatternInitializerAnchor(plan,row){
 const at=2+row*8,binding=readRequiredNumberFact(plan,at+3);if(binding>=0)return binding;
 const open=readRequiredNumberFact(plan,at+6),close=readRequiredNumberFact(plan,at+7),positions=readJavaScriptRecursiveObjectBindingPositions(plan);
 for(let index=0;index<positions.length;index+=1){const position=readRequiredNumberFact(positions,index);if(position>open&&position<close)return position;}
 if(open>=0&&close>open)return open;
 throw new TypeError('an initializer needs its actual binder or complete pattern interval');
}
/** Preserve the original binder partition; an empty pattern partitions at
 * its actual opening, which has no name to initialize.
 * @param {number[]} positions @param {number} anchor @param {string} kind
 * @param {string} value @returns {number[]} */
export function partitionJavaScriptPatternInitializerPositions(positions,anchor,kind,value){
 if(positions.includes(anchor))return partitionJavaScriptFormalInitializerBindingPositions(positions,anchor);
 if(!Number.isSafeInteger(anchor)||anchor<0||kind!=='punct'||value!=='['&&value!=='{')throw new RangeError('a nameless initializer needs its actual pattern opening');
 let available=[],uninitialized=[],previous=-1;
 for(let index=0;index<positions.length;index+=1){const position=readRequiredNumberFact(positions,index);if(!Number.isSafeInteger(position)||position<=previous)throw new RangeError('actual binding positions must retain their strict source order');previous=position;if(position<anchor)available=available.concat([position]);else uninitialized=uninitialized.concat([position]);}
 return [available.length].concat(available,uninitialized);
}
