// Inspectable internal slots in the SAME global heap. No host Map/Set bodies,
// snapshots, hidden JS collections or source-name guesses implement operations.
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,replaceJavaScriptOrdinaryObject,planJavaScriptOrdinaryGet} from './javascript-ordinary-heap.js';
import {testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
import {testJavaScriptCollectionKey,normalizeJavaScriptCollectionKey,readJavaScriptCollectionOperationKind} from './javascript-collection-rules.js';
/** @typedef {{$:'collectionNormal',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'collectionInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionTypeError',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'collectionNeedIterable',heap:JavaScriptOrdinaryHeap,target:JavaScriptSemanticValue,input:JavaScriptSemanticValue}} JavaScriptCollectionCompletion */
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {string} kind @returns {JavaScriptCollectionCompletion} */
export function allocateJavaScriptCollection(heap,identity,prototype,kind){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 if(kind!=='Map'&&kind!=='Set')return {$:'collectionInvalid',heap,reason:'collection allocation needs its actual registered Map or Set kind'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||heap.symbols.includes(identity))return {$:'collectionInvalid',heap,reason:'collection allocation needs a fresh external global identity'};
 if(heap.objects.length>=4096)return {$:'collectionResourceLimit',heap,reason:'collection allocation exceeds the global heap directory budget'};
 if(prototype.$!=='null'&&prototype.$!=='object')return {$:'collectionInvalid',heap,reason:'collection allocation retains its actual null or Object prototype'};
 const invalid=readJavaScriptOrdinaryValueError(heap,prototype);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};
 const object=kind==='Map'?{$:'mapObject',identity,prototype,extensible:true,callable:false,properties:[],entries:[]}:{$:'setObject',identity,prototype,extensible:true,callable:false,properties:[],entries:[]};
 let index=0;while(index<heap.objects.length&&(heap.objects[index]?.identity??identity)<identity)index++;
 const updated={...heap,objects:heap.objects.slice(0,index).concat([object],heap.objects.slice(index))},invalidPrototype=readJavaScriptOrdinaryHeapError(updated);
 if(invalidPrototype!=='')return {$:'collectionInvalid',heap,reason:invalidPrototype};
 return {$:'collectionNormal',heap:updated,value:{$:'object',identity,callable:false}};
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} receiver @param {string} operation @param {JavaScriptSemanticValue[]} args @returns {JavaScriptCollectionCompletion} */
export function executeJavaScriptCollectionOperation(heap,receiver,operation,args){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 const kind=readJavaScriptCollectionOperationKind(operation);if(kind==='')return {$:'collectionInvalid',heap,reason:'collection operations require their actual registered operation'};
 if(args.length>4096)return {$:'collectionResourceLimit',heap,reason:'collection arguments exceed their execution budget'};
 for(const value of [receiver,...args]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};}
 const object=receiver.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)]:undefined;
 if(object===undefined||!(kind==='Map'&&object.$==='mapObject'||kind==='Set'&&object.$==='setObject'))return {$:'collectionTypeError',heap,reason:operation+' requires its actual collection internal slot receiver'};
 const key=normalizeJavaScriptCollectionKey(args[0]??{$:'undefined'}),at=object.entries.findIndex(entry=>entry.present&&testJavaScriptCollectionKey(entry.key,key)),entry=object.entries[at];
 if(operation==='Map.get')return {$:'collectionNormal',heap,value:entry===undefined?{$:'undefined'}:entry.value};
 if(operation==='Map.has'||operation==='Set.has')return {$:'collectionNormal',heap,value:{$:'boolean',value:at>=0}};
 if(operation==='Map.size'||operation==='Set.size')return {$:'collectionNormal',heap,value:{$:'number',value:object.entries.filter(entry=>entry.present).length}};
 if(operation==='Map.clear'||operation==='Set.clear')return {$:'collectionNormal',heap:replaceJavaScriptOrdinaryObject(heap,{...object,entries:object.entries.map(entry=>({...entry,key:{$:'undefined'},value:{$:'undefined'},present:false}))}),value:{$:'undefined'}};
 if(operation==='Map.delete'||operation==='Set.delete')return {$:'collectionNormal',heap:at<0?heap:replaceJavaScriptOrdinaryObject(heap,{...object,entries:object.entries.map((entry,index)=>index===at?{...entry,key:{$:'undefined'},value:{$:'undefined'},present:false}:entry)}),value:{$:'boolean',value:at>=0}};
 if(at<0&&object.entries.length>=65536)return {$:'collectionResourceLimit',heap,reason:'collection insertion history exceeds its explicit ordinal budget'};
 const value=kind==='Set'?key:args[1]??{$:'undefined'},next={$:'collectionEntry',key,value,present:true};
 const entries=at<0?object.entries.concat([next]):object.entries.map((current,index)=>index===at?{...current,value}:current);
 const updated=replaceJavaScriptOrdinaryObject(heap,{...object,entries}),limit=readJavaScriptOrdinaryHeapError(updated);
 if(limit!=='')return {$:'collectionResourceLimit',heap,reason:limit};
 return {$:'collectionNormal',heap:updated,value:receiver};
}
/** The caller supplies independently registered STANDARD operation owners.
 * Pure data-property checks establish the fast-copy preconditions. Getters,
 * overridden adders/iterators/next methods and other iterables retain the real
 * iterable-protocol obligation; they never become invented TypeErrors.
 * @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} target @param {JavaScriptSemanticValue} input
 * @param {JavaScriptSemanticValue} adder @param {JavaScriptSemanticValue} iteratorKey @param {JavaScriptSemanticValue} iteratorMethod
 * @param {JavaScriptSemanticValue} iteratorPrototype @param {JavaScriptSemanticValue} nextMethod @returns {JavaScriptCollectionCompletion} */
export function copyJavaScriptCollection(heap,target,input,adder,iteratorKey,iteratorMethod,iteratorPrototype,nextMethod){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'collectionInvalid',heap,reason:error};
 for(const value of [target,input,adder,iteratorKey,iteratorMethod,iteratorPrototype,nextMethod]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'collectionInvalid',heap,reason:invalid};}
 const object=target.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,target.identity)]:undefined;
 if(object===undefined||object.$!=='mapObject'&&object.$!=='setObject'||object.entries.length!==0)return {$:'collectionInvalid',heap,reason:'constructor copying needs its actual fresh empty collection'};
 if(input.$==='undefined'||input.$==='null')return {$:'collectionNormal',heap,value:target};
 const source=input.$==='object'?heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)]:undefined;
 if(source===undefined||source.$!==object.$||iteratorKey.$!=='symbol'||iteratorPrototype.$!=='object'||adder.$!=='object'||!adder.callable||iteratorMethod.$!=='object'||!iteratorMethod.callable||nextMethod.$!=='object'||!nextMethod.callable)return {$:'collectionNeedIterable',heap,target,input};
 const addKey=object.$==='setObject'?{$:'text',units:[97,100,100]}:{$:'text',units:[115,101,116]},nextKey={$:'text',units:[110,101,120,116]};
 const actualAdder=planJavaScriptOrdinaryGet(heap,object.identity,addKey,target),actualIterator=planJavaScriptOrdinaryGet(heap,source.identity,iteratorKey,input),actualNext=planJavaScriptOrdinaryGet(heap,iteratorPrototype.identity,nextKey,iteratorPrototype);
 if(actualAdder.action!=='value'||actualIterator.action!=='value'||actualNext.action!=='value'||!testJavaScriptSemanticSameValue(actualAdder.value,adder)||!testJavaScriptSemanticSameValue(actualIterator.value,iteratorMethod)||!testJavaScriptSemanticSameValue(actualNext.value,nextMethod))return {$:'collectionNeedIterable',heap,target,input};
 const entries=source.entries.filter(entry=>entry.present).map(entry=>({...entry}));
 const updated=replaceJavaScriptOrdinaryObject(heap,{...object,entries}),limit=readJavaScriptOrdinaryHeapError(updated);
 if(limit!=='')return {$:'collectionResourceLimit',heap,reason:limit};
 return {$:'collectionNormal',heap:updated,value:target};
}
