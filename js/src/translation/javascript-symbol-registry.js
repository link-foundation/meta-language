// The represented agent registry lives once in the shared heap, independently
// of any callable's realm. Native Symbol.for is never a contents fallback.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,registerJavaScriptOrdinarySymbol} from './javascript-ordinary-heap.js';
import {compareJavaScriptCollectionKeys} from './javascript-collection-rules.js';
import {readJavaScriptSemanticTextLength,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
const model='represented global Symbol registry 1';
function initializeJavaScriptSymbolRegistryDirectory(heap){return heap.symbolRegistryModel===undefined?{...heap,symbolRegistryModel:model,symbolRegistry:(heap.symbolRegistrations??[]).filter(fact=>fact.key!==undefined).map(fact=>({$:'symbolRegistryEntry',identity:fact.identity,key:fact.key})).toSorted((first,second)=>compareJavaScriptCollectionKeys(first.key,second.key))}:heap;}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} key @returns {number} */
export function findJavaScriptSymbolRegistryKey(heap,key){
 const entries=heap.symbolRegistry??[];let low=0,high=entries.length;
 while(low<high){const middle=Math.floor((low+high)/2),entry=entries[middle];if(entry===undefined)return -1;const order=compareJavaScriptCollectionKeys(entry.key,key);if(order===0)return middle;if(order<0)low=middle+1;else high=middle;}return -low-1;
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @returns {JavaScriptSymbolRegistrationFact|undefined} */
export function readJavaScriptSymbolRegistrationFact(heap,identity){const facts=heap.symbolRegistrations??[];let low=0,high=facts.length;while(low<high){const middle=Math.floor((low+high)/2),fact=facts[middle];if(fact!==undefined&&fact.identity<identity)low=middle+1;else high=middle;}return facts[low]?.identity===identity?facts[low]:undefined;}
/** Input for `for` is already its actual implicit ToString result. `keyFor`
 * accepts the original value and never invokes an object coercion hook.
 * @param {JavaScriptOrdinaryHeap} heap @param {string} operation @param {JavaScriptSemanticValue} input @returns {object} */
export function executeJavaScriptSymbolRegistryOperation(heap,operation,input){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'symbolRegistryInvalid',heap,reason:error};const invalid=readJavaScriptOrdinaryValueError(heap,input);if(invalid!=='')return {$:'symbolRegistryInvalid',heap,reason:invalid};
 if(operation==='keyFor'){
  if(input.$!=='symbol')return {$:'symbolRegistryError',heap,errorClass:'TypeError',reason:'Symbol.keyFor requires an actual primitive Symbol without coercion'};
  const fact=readJavaScriptSymbolRegistrationFact(heap,input.identity);if(fact===undefined)return {$:'symbolRegistryInvalid',heap,reason:'Symbol.keyFor requires actual registration authority for this Symbol'};
  if(!fact.registered)return {$:'symbolRegistryValue',heap,value:{$:'undefined'}};
  if(fact.key===undefined)return {$:'symbolRegistryInvalid',heap,reason:'registered Symbol keyFor requires its exact actual registry key witness'};
  return {$:'symbolRegistryValue',heap,value:fact.key};
 }
 if(operation!=='for'||input.$!=='text'&&input.$!=='sourceText')return {$:'symbolRegistryInvalid',heap,reason:'Symbol.for retains its actual completed implicit text conversion'};
 if(readJavaScriptSemanticTextLength(input)>1048576)return {$:'symbolRegistryResourceLimit',heap,reason:'Symbol registry keys exceed the selected 1048576 UTF16 unit profile'};
 heap=initializeJavaScriptSymbolRegistryDirectory(heap);
 const at=findJavaScriptSymbolRegistryKey(heap,input),entry=heap.symbolRegistry?.[at];if(entry!==undefined)return {$:'symbolRegistryValue',heap,value:{$:'symbol',identity:entry.identity}};
 for(const identity of heap.symbols){const fact=readJavaScriptSymbolRegistrationFact(heap,identity);if(fact===undefined||fact.registered&&fact.key===undefined)return {$:'symbolRegistryInvalid',heap,reason:'new registry keys require complete actual imported Symbol registration/key authority'};}
 if(heap.symbolRegistryModel!==undefined&&heap.symbolRegistryModel!==model)return {$:'symbolRegistryInvalid',heap,reason:'Symbol registry operations retain their actual shared agent directory model'};
 if(heap.symbols.length>=16384)return {$:'symbolRegistryResourceLimit',heap,reason:'Symbol registry allocation exceeds the shared 16384 global identity budget'};
 return {$:'symbolRegistryAllocate',heap:heap.symbolRegistryModel===undefined?{...heap,symbolRegistryModel:model,symbolRegistry:[]}:heap,key:input};
}
/** Actual boundary witnesses import only keys/classification for a supplied
 * native Symbol identity. All represented allocation/membership stays here.
 * Undefined key means the supplied actual Symbol is nonregistered.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key @returns {object} */
export function importJavaScriptSymbolRegistryWitness(heap,identity,key){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'symbolRegistryInvalid',heap,reason:error};
 if(key.$!=='undefined'&&key.$!=='text'&&key.$!=='sourceText')return {$:'symbolRegistryInvalid',heap,reason:'a Symbol registration witness retains its actual optional primitive text key'};
 const invalid=readJavaScriptOrdinaryValueError(heap,key);if(invalid!=='')return {$:'symbolRegistryInvalid',heap,reason:invalid};
 heap=initializeJavaScriptSymbolRegistryDirectory(heap);
 const registered=key.$!=='undefined',prior=readJavaScriptSymbolRegistrationFact(heap,identity);if(prior&&(prior.registered!==registered||prior.key!==undefined&&(!registered||!testJavaScriptSemanticSameValue(prior.key,key))))return {$:'symbolRegistryInvalid',heap,reason:'Symbol registry witnesses preserve actual prior classification and exact key'};
 const at=registered?findJavaScriptSymbolRegistryKey(heap,key):-1,entry=heap.symbolRegistry?.[at];if(entry&&entry.identity!==identity)return {$:'symbolRegistryInvalid',heap,reason:'one actual global registry key has one canonical Symbol identity'};
 if(prior&&prior.registered===registered&&(!registered||prior.key!==undefined&&testJavaScriptSemanticSameValue(prior.key,key)&&entry?.identity===identity))return {$:'symbolRegistryValue',heap,value:{$:'symbol',identity}};
 let next=heap;if(!heap.symbols.includes(identity)){const added=registerJavaScriptOrdinarySymbol(heap,identity);if(!added.success)return {$:'symbolRegistryInvalid',heap,reason:added.error};next=added.heap;}
 const fact={$:'symbolRegistrationFact',identity,registered,...(registered?{key}:{})},facts=(next.symbolRegistrations??[]).filter(row=>row.identity!==identity).concat([fact]).toSorted((first,second)=>first.identity<second.identity?-1:first.identity>second.identity?1:0),entries=next.symbolRegistry??[],insert=-at-1,registry=registered&&!entry?entries.slice(0,insert).concat([{$:'symbolRegistryEntry',identity,key}],entries.slice(insert)):entries;
 const updated={...next,symbolRegistrationModel:'actual global Symbol registry classification 1',symbolRegistrations:facts,symbolRegistryModel:model,symbolRegistry:registry},failure=readJavaScriptOrdinaryHeapError(updated);return failure===''?{$:'symbolRegistryValue',heap:updated,value:{$:'symbol',identity}}:{$:'symbolRegistryInvalid',heap,reason:failure};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key @returns {object} */
export function allocateJavaScriptRegisteredSymbol(heap,identity,key){
 const request=executeJavaScriptSymbolRegistryOperation(heap,'for',key);if(request.$!=='symbolRegistryAllocate')return request;return importJavaScriptSymbolRegistryWitness(request.heap,identity,key);
}
