import {testJavaScriptValidatedHeapSnapshot} from '#ordinary-heap-validation-authority';
import {readJavaScriptWeakKeyEligibility} from './javascript-weak-collection-rules.js';
import {readJavaScriptArrayObjectError,planJavaScriptArrayPropertyDefinition,testJavaScriptArrayLengthKey} from './javascript-array-objects.js';
import {readJavaScriptSemanticValueError,testJavaScriptSemanticSameValue,constructJavaScriptSemanticTextUnits,readJavaScriptSemanticTextLength} from './javascript-semantic-values.js';
import {createJavaScriptPropertyDescriptor,testCompleteJavaScriptPropertyDescriptor,planJavaScriptPropertyDefinition} from './javascript-property-descriptors.js';
import {testJavaScriptCollectionKey,orderJavaScriptCollectionKeys,compareJavaScriptCollectionKeys} from './javascript-collection-rules.js';
import {readJavaScriptLiteralPropertyIndex} from './javascript-literal-property-rules.js';

import {findJavaScriptHeapDirectoryObject,testJavaScriptHeapDirectorySymbol,readJavaScriptHeapDirectoryValueError,readJavaScriptHeapDirectoryKeyError,findJavaScriptHeapDirectoryProperty} from './javascript-ordinary-heap-directory.js';

/** @param {JavaScriptOrdinaryObject[]} objects @param {bigint} identity @returns {number} */
export function findJavaScriptOrdinaryObject(objects, identity) {
  return findJavaScriptHeapDirectoryObject(objects, identity);
}

/** @param {bigint[]} symbols @param {bigint} identity @returns {boolean} */
export function testJavaScriptOrdinarySymbol(symbols, identity) {
  return testJavaScriptHeapDirectorySymbol(symbols, identity);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} value @returns {string} */
export function readJavaScriptOrdinaryValueError(heap, value) {
  return readJavaScriptHeapDirectoryValueError(heap, value);
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} key @returns {string} */
export function readJavaScriptOrdinaryKeyError(heap, key) {
  return readJavaScriptHeapDirectoryKeyError(heap, key);
}

/** @param {JavaScriptOrdinaryProperty[]} properties @param {JavaScriptSemanticValue} key @returns {number} */
export function findJavaScriptOrdinaryProperty(properties, key) {
  return findJavaScriptHeapDirectoryProperty(properties, key);
}

/** @typedef {{closureCaptureCount:number,propertyCount:number,collectionEntryCount:number,collectionKeyUnits:number,collectionStoredUnits:number}} JavaScriptHeapValidationBudget */

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @returns {string} */
function readJavaScriptOpaqueObjectError(heap, object) {
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject'){
    if(object.callable||object.properties.length!==0)return 'opaque Error objects keep host properties outside ordinary descriptor snapshots';
    for(const hook of [object.hostGet,object.hostSet]){
      if(hook===undefined)return 'opaque Error access requires its actual externally registered call hooks';
      const error=readJavaScriptOrdinaryValueError(heap,hook);
      if(error!=='')return error;
      if(hook.$!=='object'||!hook.callable)return 'opaque Error access requires its actual externally registered call hooks';
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptClosureObjectError(heap, object, budget) {
  if(object.$==='closureObject'){
    if(!object.callable||typeof object.owner!=='bigint'||object.owner<=0n||object.owner>18446744073709551615n||!Number.isSafeInteger(object.ordinal)||object.ordinal<0||object.ordinal>4095||Object.is(object.ordinal,-0)||object.captures.length>4096)return 'closures retain their actual source owner, lexical site, callable brand and bounded captures';
    budget.closureCaptureCount+=object.captures.length;if(budget.closureCaptureCount>1048576)return 'closure captures exceed the shared heap value budget';
    const owner=heap.objects[findJavaScriptOrdinaryObject(heap.objects,object.owner)];if(owner===undefined||!owner.callable||owner.$==='closureObject'||owner.$==='boundFunctionObject')return 'closure code owners must be their actual static source function, never another runtime closure identity';
    const newTarget=object.lexicalNewTarget;
    if(newTarget!==undefined&&(newTarget.$!=='object'||!newTarget.callable||readJavaScriptOrdinaryValueError(heap,newTarget)!==''))return 'lexical newTarget retains its actual optional constructor identity';
    for(const value of [object.lexicalReceiver,...object.captures]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptBoundObjectError(heap, object, budget) {
  if(object.$==='boundFunctionObject'){
    if(!object.callable||object.boundTarget.$!=='object'||!object.boundTarget.callable||!Array.isArray(object.boundArguments)||object.boundArguments.length>1048576)return 'bound Functions retain actual callable targets and bounded complete argument Lists';
    budget.closureCaptureCount+=object.boundArguments.length+2;if(budget.closureCaptureCount>1048576)return 'bound slots and closure captures share the global value storage budget';
    for(const value of [object.boundTarget,object.boundThis,...object.boundArguments]){const error=readJavaScriptOrdinaryValueError(heap,value);if(error!=='')return error;}
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptCollectionIteratorObjectError(heap, object, budget) {
  if(object.$==='collectionIteratorObject'){
    if(!['Map','Set','Array','String'].includes(object.collectionKind)||object.callable||!Number.isSafeInteger(object.cursor)||object.cursor<0||Object.is(object.cursor,-0)||object.cursor>(object.collectionKind==='Array'?9007199254740991:object.collectionKind==='String'?16777216:65536)||!['keys','values','entries'].includes(object.kind)||object.collectionKind==='Array'&&typeof object.executing!=='boolean'||object.collectionKind!=='Array'&&object.executing!==undefined)return 'collection iterators require their exact bounded cursor and iteration kind';
    if(object.collectionKind==='String'){
      const text=object.iteratedText;
      if(object.collection!==0n||object.kind!=='values'||text===undefined)return 'String iterators retain only their actual text value and code-unit cursor';
      const error=readJavaScriptOrdinaryValueError(heap,text);if(error!=='')return error;
      const length=readJavaScriptSemanticTextLength(text);
      if(text.$==='undefined'){if(object.cursor!==0)return 'an exhausted String iterator releases its text and cursor';}
      else if(length<0||object.cursor>length)return 'a live String iterator retains its actual text and code-unit position';
      budget.collectionStoredUnits+=Math.max(0,length);if(budget.collectionStoredUnits>67108864)return 'iterator texts and collection values exceed their shared UTF16 storage budget';
    }else{
      if(object.iteratedText!==undefined)return 'non-String iterator families cannot invent String internal slots';
      if(object.collection!==0n){
        const collection=heap.objects[findJavaScriptOrdinaryObject(heap.objects,object.collection)];
        if(collection===undefined||object.collectionKind!=='Array'&&(!(object.collectionKind==='Map'&&collection.$==='mapObject'||object.collectionKind==='Set'&&collection.$==='setObject')||object.cursor>collection.entries.length))return 'a live collection iterator retains its actual collection identity and insertion position';
      }else if(object.cursor!==0||object.executing===true)return 'an exhausted collection iterator releases its collection, cursor and executing state';
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptWeakCollectionObjectError(heap, object, budget) {
  if(object.$==='weakMapObject'||object.$==='weakSetObject'){
    const entries=object.weakEntries;
    if(object.callable||entries===undefined||!Array.isArray(entries)||entries.length>65536||object.entries!==undefined)return 'weak collections retain their distinct noncallable bounded weak internal slots';
    budget.collectionEntryCount+=entries.length;if(budget.collectionEntryCount>1048576)return 'weak and strong slots exceed the shared collection storage budget';
    let previousWeakKey=0n;for(const entry of entries){
      if(entry===undefined||entry===null||entry.$!=='weakCollectionEntry')return 'weak internal slots retain dense actual entry records';
      const keyError=readJavaScriptOrdinaryValueError(heap,entry.key),valueError=readJavaScriptOrdinaryValueError(heap,entry.value);if(keyError!==''||valueError!=='')return keyError||valueError;
      if(readJavaScriptWeakKeyEligibility(heap,entry.key)!=='eligible')return 'weak slots require actual eligible Object or nonregistered Symbol keys';
      if((entry.key.$!=='object'&&entry.key.$!=='symbol')||entry.key.identity<=previousWeakKey)return 'weak slots retain unique ordered actual global key identities';previousWeakKey=entry.key.identity;
      if(object.$==='weakSetObject'&&entry.value.$!=='undefined')return 'WeakSet weak slots do not invent Map value edges';
      budget.collectionStoredUnits+=Math.max(0,readJavaScriptSemanticTextLength(entry.value));if(budget.collectionStoredUnits>67108864)return 'weak and strong values exceed their shared UTF16 storage budget';
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptStrongCollectionObjectError(heap, object, budget) {
  if(object.$==='mapObject'||object.$==='setObject'){
    const entries=object.entries;
    if(object.callable||entries===undefined||entries.length>65536)return 'collection objects require noncallable inspectable bounded internal slots';
    budget.collectionEntryCount+=entries.length;
    if(budget.collectionEntryCount>1048576)return 'collection entry history exceeds the shared heap storage budget';
    const keys=[];
    for(const entry of entries){
      if(entry===undefined||entry.$!=='collectionEntry'||typeof entry.present!=='boolean')return 'collection insertion history requires dense explicit entry records';
      const keyError=readJavaScriptOrdinaryValueError(heap,entry.key),valueError=readJavaScriptOrdinaryValueError(heap,entry.value);
      if(keyError!==''||valueError!=='')return keyError!==''?keyError:valueError;
      budget.collectionStoredUnits+=Math.max(0,readJavaScriptSemanticTextLength(entry.key))+Math.max(0,readJavaScriptSemanticTextLength(entry.value));
      if(budget.collectionStoredUnits>67108864)return 'collection entries exceed their shared UTF16 storage budget';
      const key=entry.key;
      if(key.$==='number'){
        if(Object.is(key.value,-0))return 'collection stored keys normalize negative Number zero';
      }
      if(object.$==='setObject'&&!testJavaScriptSemanticSameValue(entry.key,entry.value))return 'Set entries retain the same normalized key and value';
      if(entry.present){
        budget.collectionKeyUnits+=Math.max(0,readJavaScriptSemanticTextLength(entry.key));
        if(budget.collectionKeyUnits>16777216)return 'active collection keys exceed their shared UTF16 comparison budget';
        keys.push(entry.key);
      }
    }
    const orderedKeys=orderJavaScriptCollectionKeys(keys);
    for(let at=1;at<orderedKeys.length;at++){
      const prior=orderedKeys[at-1],current=orderedKeys[at];
      if(prior!==undefined&&current!==undefined&&testJavaScriptCollectionKey(prior,current))return 'active collection keys must be unique under SameValueZero';
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptPropertyDescriptor} descriptor @returns {string} */
function readJavaScriptOrdinaryDescriptorValueError(heap, descriptor) {
  if (!testCompleteJavaScriptPropertyDescriptor(descriptor)) return 'ordinary storage requires complete valid property descriptors';
  if (descriptor.valuePresent) {
    const error = readJavaScriptOrdinaryValueError(heap, descriptor.value);
    if (error !== '') return error;
  }
  if (descriptor.getterPresent) {
    const error = readJavaScriptOrdinaryValueError(heap, descriptor.getter);
    if (error !== '') return error;
  }
  if (descriptor.setterPresent) {
    const error = readJavaScriptOrdinaryValueError(heap, descriptor.setter);
    if (error !== '') return error;
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} prototype @param {JavaScriptOrdinaryProperty[]} properties @param {JavaScriptHeapValidationBudget} budget @returns {string} */
function readJavaScriptOrdinaryObjectDescriptorError(heap, prototype, properties, budget) {
  if (prototype.$ !== 'null' && prototype.$ !== 'object') return 'an ordinary prototype must be null or a declared Object';
  const prototypeError = readJavaScriptOrdinaryValueError(heap, prototype);
  if (prototypeError !== '') return prototypeError;
  budget.propertyCount += properties.length;
  if (budget.propertyCount > 16384 || properties.length > 4096) return 'ordinary properties exceed their explicit ordered storage budget';
  for (let at = 0; at < properties.length; at += 1) {
    const property = properties[at];
    if (property === undefined) return 'ordinary property storage must be dense';
    const keyError = readJavaScriptOrdinaryKeyError(heap, property.key);
    if (keyError !== '') return keyError;
    if (findJavaScriptOrdinaryProperty(properties, property.key) !== at) return 'ordinary property keys must be unique without reordering their creation history';
    const descriptorError = readJavaScriptOrdinaryDescriptorValueError(heap, property.descriptor);
    if (descriptorError !== '') return descriptorError;
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} object @returns {string} */
function readJavaScriptOrdinaryPrototypeChainError(heap, object) {
  let prototype = object.prototype;
  let depth = 0;
  while (prototype.$ === 'object') {
    depth += 1;
    if (prototype.identity === object.identity || depth > heap.objects.length) return 'ordinary prototype chains must be acyclic';
    const index = findJavaScriptOrdinaryObject(heap.objects, prototype.identity);
    const parent = heap.objects[index];
    if (parent === undefined) return 'every ordinary prototype must remain in the global directory';
    if(parent.$!=='ordinaryObject'&&parent.$!=='closureObject'&&parent.$!=='boundFunctionObject'&&parent.$!=='mapObject'&&parent.$!=='setObject'&&parent.$!=='weakMapObject'&&parent.$!=='weakSetObject'&&parent.$!=='collectionIteratorObject'&&parent.$!=='arrayObject')return 'opaque host Errors need explicit prototype traversal rather than ordinary descriptor lookup';
    prototype = parent.prototype;
  }
  return '';
}

/** Validate the whole bounded snapshot before a public transition. No missing
 * object, Symbol, prototype, attribute or callable fact is reconstructed from
 * a spelling or substituted host object. Prototype cycles are invalid for this
 * ordinary-only object domain; exotic objects retain their own operations.
 * @param {JavaScriptOrdinaryHeap} heap @returns {string} */
export function readJavaScriptOrdinaryHeapError(heap) {
  if(testJavaScriptValidatedHeapSnapshot(heap))return '';
  if (heap.objects.length > 4096 || heap.symbols.length > 16384) return 'ordinary heap identity directory exceeds its explicit resource budget';
  let previous = 0n;
  for (const object of heap.objects) {
    if(object.$==='ordinaryObject'){
      if(object.regularExpression!==undefined&&typeof object.regularExpression!=='boolean')return 'regular expression metadata retains its actual internal-slot Boolean';
      const primitive=object.primitiveData, text=object.stringData;
      if(primitive!==undefined&&(object.callable||!['number','boolean','integer','symbol'].includes(primitive.$)||readJavaScriptOrdinaryValueError(heap,primitive)!==''||text!==undefined))return 'primitive wrapper objects retain exactly one actual Number Boolean BigInt or Symbol internal slot';
      if(text!==undefined&&(object.callable||text.$!=='text'&&text.$!=='sourceText'||readJavaScriptSemanticValueError(text)!==''))return 'String objects retain actual primitive UTF16 StringData';
      if(object.immutablePrototype!==undefined&&typeof object.immutablePrototype!=='boolean')return 'immutable prototype metadata retains its actual Boolean slot';
    }
    if(!['ordinaryObject','closureObject','boundFunctionObject','arrayObject','mapObject','setObject','weakMapObject','weakSetObject','collectionIteratorObject','hostErrorObject','hostURLObject','hostArrayObject','hostByteBufferObject'].includes(object.$))return 'ordinary heap objects require their actual registered object kind';
    if (typeof object.identity !== 'bigint' || object.identity <= previous || object.identity > 18446744073709551615n) return 'ordinary Objects need strictly ordered unique positive unsigned64 identities';
    previous = object.identity;
  }
  previous = 0n;
  for (const symbol of heap.symbols) {
    if (typeof symbol !== 'bigint' || symbol <= previous || symbol > 18446744073709551615n || findJavaScriptOrdinaryObject(heap.objects, symbol) >= 0) return 'Symbols and Objects need distinct strictly ordered positive unsigned64 identities';
    previous = symbol;
  }
  if(heap.symbolRegistrations!==undefined||heap.symbolRegistrationModel!==undefined){
    if(heap.symbolRegistrationModel!=='actual global Symbol registry classification 1'||!Array.isArray(heap.symbolRegistrations)||heap.symbolRegistrations.length>16384)return 'Symbol registration authority exceeds its explicit directory budget';
    // The array check above guarantees a directory for a heap snapshot.
    const registrations=heap.symbolRegistrations??[];
    let prior=0n;
    for(const fact of registrations){
      if(fact?.$!=='symbolRegistrationFact'||typeof fact.registered!=='boolean'||typeof fact.identity!=='bigint'||fact.identity<=prior||!testJavaScriptOrdinarySymbol(heap.symbols,fact.identity))return 'Symbol registration facts retain actual unique ordered global Symbol identities';
      prior=fact.identity;
      const key=fact.key;
      if(key!==undefined&&(!fact.registered||key.$!=='text'&&key.$!=='sourceText'||readJavaScriptOrdinaryValueError(heap,key)!==''||readJavaScriptSemanticTextLength(key)>1048576))return 'registered Symbol key witnesses retain exact bounded UTF16 text';
    }
  }
  /** @type {JavaScriptSemanticValue[]} */
  const registeredKeys=[];
  for(const fact of heap.symbolRegistrations??[]){
    const key=fact.key;
    if(key!==undefined)registeredKeys.push(key);
  }
  const witnessedKeys=orderJavaScriptCollectionKeys(registeredKeys);
  let witnessedUnits=0, previousWitness;
  for(const key of witnessedKeys){
    witnessedUnits+=readJavaScriptSemanticTextLength(key);
    if(witnessedUnits>16777216)return 'Symbol key witnesses exceed their explicit UTF16 storage profile';
    const prior=previousWitness;
    if(prior!==undefined&&testJavaScriptSemanticSameValue(prior,key))return 'actual registered Symbol witnesses have unique global keys';
    previousWitness=key;
  }
  if(heap.symbolRegistry!==undefined||heap.symbolRegistryModel!==undefined){
    if(heap.symbolRegistryModel!=='represented global Symbol registry 1'||!Array.isArray(heap.symbolRegistry)||heap.symbolRegistry.length>16384)return 'the represented Symbol registry retains its actual bounded shared agent model';
    const facts=heap.symbolRegistrations??[],registry=heap.symbolRegistry??[],identities=[];
    /** @type {JavaScriptSemanticValue} */
    let previousKey={$:'undefined'};
    let units=0;
    for(const entry of registry){
      if(entry?.$!=='symbolRegistryEntry'||!testJavaScriptOrdinarySymbol(heap.symbols,entry.identity)||entry.key?.$!=='text'&&entry.key?.$!=='sourceText'||readJavaScriptOrdinaryValueError(heap,entry.key)!=='')return 'Symbol registry entries retain actual global Symbol identities and primitive UTF16 keys';
      units+=readJavaScriptSemanticTextLength(entry.key);if(readJavaScriptSemanticTextLength(entry.key)>1048576||units>16777216)return 'Symbol registry keys exceed their explicit UTF16 storage profile';
      if(previousKey.$!=='undefined'&&compareJavaScriptCollectionKeys(previousKey,entry.key)>=0)return 'Symbol registry keys retain unique strict UTF16 order';previousKey=entry.key;
      let low=0,high=facts.length;
      while(low<high){const middle=Math.floor((low+high)/2);if((facts[middle]?.identity??entry.identity)<entry.identity)low=middle+1;else high=middle;}
      const fact=facts[low];
      if(fact===undefined||fact.identity!==entry.identity||!fact.registered)return 'registry key entries retain their matching exact actual registration witness';
      const key=fact.key;
      if(key===undefined||!testJavaScriptSemanticSameValue(key,entry.key))return 'registry key entries retain their matching exact actual registration witness';
      identities.push({$:'symbol',identity:entry.identity});
    }
    const ordered=orderJavaScriptCollectionKeys(identities);
    let previousIdentity;
    for(const current of ordered){
      const prior=previousIdentity;
      if(prior!==undefined&&testJavaScriptSemanticSameValue(prior,current))return 'one global Symbol identity retains exactly one registry key';
      previousIdentity=current;
    }
    for(const fact of facts){
      const key=fact.key;
      if(key!==undefined){
        let low=0,high=registry.length;
        while(low<high){const middle=Math.floor((low+high)/2),entry=registry[middle];if(entry!==undefined&&compareJavaScriptCollectionKeys(entry.key,key)<0)low=middle+1;else high=middle;}
        if(registry[low]?.identity!==fact.identity)return 'known registered key witnesses remain in the monotonic global registry';
      }
    }
  }
  const budget = {closureCaptureCount:0,propertyCount:0,collectionEntryCount:0,collectionKeyUnits:0,collectionStoredUnits:0};
  for (const object of heap.objects) {
    const opaqueObjectError = readJavaScriptOpaqueObjectError(heap, object);
    if (opaqueObjectError !== '') return opaqueObjectError;
    const closureObjectError = readJavaScriptClosureObjectError(heap, object, budget);
    if (closureObjectError !== '') return closureObjectError;
    const boundObjectError = readJavaScriptBoundObjectError(heap, object, budget);
    if (boundObjectError !== '') return boundObjectError;
    const collectionIteratorObjectError = readJavaScriptCollectionIteratorObjectError(heap, object, budget);
    if (collectionIteratorObjectError !== '') return collectionIteratorObjectError;
    const weakCollectionObjectError = readJavaScriptWeakCollectionObjectError(heap, object, budget);
    if (weakCollectionObjectError !== '') return weakCollectionObjectError;
    const strongCollectionObjectError = readJavaScriptStrongCollectionObjectError(heap, object, budget);
    if (strongCollectionObjectError !== '') return strongCollectionObjectError;
    const ordinaryObjectDescriptorError = readJavaScriptOrdinaryObjectDescriptorError(heap, object.prototype, object.properties, budget);
    if (ordinaryObjectDescriptorError !== '') return ordinaryObjectDescriptorError;
    if (object.$ === 'arrayObject') {
      const arrayError = readJavaScriptArrayObjectError(object);
      if (arrayError !== '') return arrayError;
    }
    const ordinaryPrototypeChainError = readJavaScriptOrdinaryPrototypeChainError(heap, object);
    if (ordinaryPrototypeChainError !== '') return ordinaryPrototypeChainError;
  }
  // One immutable outgoing target edge per bound Function. Share completed
  // paths so validation inspects the whole target graph in linear node work.
  /** @type {{state:number}[]} */
  const marks=[];
  for(let index=0;index<heap.objects.length;index++)marks.push({state:0});
  for(let first=0;first<heap.objects.length;first++){
    let index=first;
    const path=[];
    while(index>=0){
      const current=heap.objects[index], mark=marks[index];
      if(current===undefined||mark===undefined||mark.state===2)break;
      if(current.$!=='boundFunctionObject')break;
      if(mark.state===1)return 'bound Function target chains retain their actual acyclic creation order';
      mark.state=1;
      path.push(index);
      const target=current.boundTarget;
      if(target.$!=='object')return 'bound Function targets must retain actual callable Object identities';
      index=findJavaScriptOrdinaryObject(heap.objects,target.identity);
    }
    for(const selected of path){
      const mark=marks[selected];
      if(mark!==undefined)mark.state=2;
    }
  }
  return '';
}

/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptOrdinaryObject} replacement
 * @returns {JavaScriptOrdinaryHeap} */
export function replaceJavaScriptOrdinaryObject(heap, replacement) {
  const objects = heap.objects.map(object => object.identity === replacement.identity ? replacement : object);
  return {...heap,objects};
}

/** Internal helper: callers have validated the complete snapshot and key.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key
 * @returns {{found:boolean,owner:bigint,descriptor:JavaScriptPropertyDescriptor}} */
export function locateJavaScriptOrdinaryProperty(heap, identity, key) {
  let current = identity;
  for (let depth = 0; depth < heap.objects.length; depth += 1) {
    const index = findJavaScriptOrdinaryObject(heap.objects, current);
    const object = heap.objects[index];
    if (object === undefined) break;
    const at = findJavaScriptOrdinaryProperty(object.properties, key);
    const property = object.properties[at];
    if (property !== undefined) return {found:true,owner:current,descriptor:property.descriptor};
    const prototype = object.prototype;
    if (prototype.$ !== 'object') break;
    current = prototype.identity;
  }
  return {found:false,owner:0n,descriptor:createJavaScriptPropertyDescriptor('data',false,false)};
}

/** A Get operation returns a value or an explicit accessor call obligation.
 * The caller must execute a call with exactly this receiver and then resume
 * the original evaluation; returning undefined for an unexecuted call is wrong.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} receiver
 * @returns {{action:string,error:string,value:JavaScriptSemanticValue,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[]}} */
export function planJavaScriptOrdinaryGet(heap, identity, key, receiver) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  const keyError = readJavaScriptOrdinaryKeyError(heap, key);
  const receiverError = readJavaScriptOrdinaryValueError(heap, receiver);
  const error = heapError !== '' ? heapError : keyError !== '' ? keyError : receiverError !== '' ? receiverError
    : findJavaScriptOrdinaryObject(heap.objects, identity) < 0 ? 'an ordinary Get needs its actual Object identity' : '';
  if (error !== '') return {action:'error',error,value:{$:'undefined'},callee:{$:'undefined'},receiver,arguments:[]};
  const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,identity)];
  if(target!==undefined&&target.$!=='ordinaryObject'&&target.$!=='closureObject'&&target.$!=='boundFunctionObject'&&target.$!=='mapObject'&&target.$!=='setObject'&&target.$!=='weakMapObject'&&target.$!=='weakSetObject'&&target.$!=='collectionIteratorObject'&&target.$!=='arrayObject')return {action:'call',error:'',value:{$:'undefined'},callee:target.hostGet??{$:'undefined'},receiver:{$:'object',identity,callable:false},arguments:[key,receiver]};
  const property = locateJavaScriptOrdinaryProperty(heap, identity, key);
  if (!property.found) return {action:'value',error:'',value:{$:'undefined'},callee:{$:'undefined'},receiver,arguments:[]};
  if (property.descriptor.valuePresent) return {action:'value',error:'',value:property.descriptor.value,callee:{$:'undefined'},receiver,arguments:[]};
  const getter = property.descriptor.getter;
  if (getter.$ === 'undefined') return {action:'value',error:'',value:{$:'undefined'},callee:{$:'undefined'},receiver,arguments:[]};
  return {action:'call',error:'',value:{$:'undefined'},callee:getter,receiver,arguments:[]};
}

/** DefineOwnProperty commits the exact source-selected descriptor transition.
 * A keep/reject retains the whole original state. Physical field adapters must
 * still obey the source write mask when applying a snapshot to native storage.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} proposed
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string,writes:string[],resourceLimit?:boolean}} */
export function defineJavaScriptOrdinaryProperty(heap, identity, key, proposed) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  const keyError = readJavaScriptOrdinaryKeyError(heap, key);
  const error = heapError !== '' ? heapError : keyError;
  if (error !== '') return {heap,success:false,error,writes:[]};
  if (proposed.valuePresent && readJavaScriptOrdinaryValueError(heap, proposed.value) !== '') return {heap,success:false,error:'proposed property values must belong to the actual global context',writes:[]};
  if (proposed.getterPresent && readJavaScriptOrdinaryValueError(heap, proposed.getter) !== '') return {heap,success:false,error:'proposed getters must belong to the actual global context',writes:[]};
  if (proposed.setterPresent && readJavaScriptOrdinaryValueError(heap, proposed.setter) !== '') return {heap,success:false,error:'proposed setters must belong to the actual global context',writes:[]};
  const index = findJavaScriptOrdinaryObject(heap.objects, identity);
  const object = heap.objects[index];
  if (object === undefined) return {heap,success:false,error:'DefineOwnProperty needs its actual Object identity',writes:[]};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {heap,success:false,error:'opaque Error descriptor definition requires its actual host protocol',writes:[]};
  if(object.$==='arrayObject'){
    const plan=planJavaScriptArrayPropertyDefinition(object,key,proposed);
    if(plan.errorClass==='resource')return {heap,success:false,error:plan.error,writes:[],resourceLimit:true};
    if(plan.errorClass!=='')return {heap,success:false,error:plan.error,writes:[]};
    let count=0;for(const entry of heap.objects)count+=entry.properties.length;
    if(count-object.properties.length+plan.object.properties.length>16384)return {heap,success:false,error:'ordinary properties exceed their explicit ordered storage budget',writes:[],resourceLimit:true};
    return {heap:replaceJavaScriptOrdinaryObject(heap,plan.object),success:plan.success,error:plan.error,writes:plan.writes};
  }
  const at = findJavaScriptOrdinaryProperty(object.properties, key);
  const property = object.properties[at];
  const current = property === undefined ? createJavaScriptPropertyDescriptor('data',false,false) : property.descriptor;
  const transition = planJavaScriptPropertyDefinition(property !== undefined, object.extensible, current, proposed);
  if (transition.disposition === 'reject') return {heap,success:false,error:transition.error,writes:[]};
  if (transition.disposition === 'keep') return {heap,success:true,error:'',writes:[]};
  if (at < 0 && object.properties.length >= 4096) return {heap,success:false,error:'ordinary properties exceed their explicit ordered storage budget',writes:[],resourceLimit:true};
  let count = 0;
  for (const entry of heap.objects) count += entry.properties.length;
  if (at < 0 && count >= 16384) return {heap,success:false,error:'ordinary properties exceed their explicit ordered storage budget',writes:[],resourceLimit:true};
  const updated = {$:'ordinaryProperty',key,descriptor:transition.descriptor};
  const properties = at < 0 ? object.properties.concat([updated]) : object.properties.map((entry,position) => position === at ? updated : entry);
  const replacement = {...object,properties};
  return {heap:replaceJavaScriptOrdinaryObject(heap,replacement),success:true,error:'',writes:transition.writes};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} key
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string}} */
export function deleteJavaScriptOrdinaryProperty(heap, identity, key) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  const keyError = readJavaScriptOrdinaryKeyError(heap, key);
  const error = heapError !== '' ? heapError : keyError;
  if (error !== '') return {heap,success:false,error};
  const index = findJavaScriptOrdinaryObject(heap.objects, identity);
  const object = heap.objects[index];
  if (object === undefined) return {heap,success:false,error:'Delete needs its actual Object identity'};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {heap,success:false,error:'opaque Error deletion requires its actual host protocol'};
  const at = findJavaScriptOrdinaryProperty(object.properties, key);
  const property = object.properties[at];
  if (property === undefined) return {heap,success:true,error:''};
  if (!property.descriptor.configurable) return {heap,success:false,error:''};
  const properties = object.properties.filter((entry,position) => position !== at);
  const replacement = {...object,properties};
  return {heap:replaceJavaScriptOrdinaryObject(heap,replacement),success:true,error:''};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string}} */
export function preventJavaScriptOrdinaryExtensions(heap, identity) {
  const error = readJavaScriptOrdinaryHeapError(heap);
  if (error !== '') return {heap,success:false,error};
  const index = findJavaScriptOrdinaryObject(heap.objects, identity);
  const object = heap.objects[index];
  if (object === undefined) return {heap,success:false,error:'PreventExtensions needs its actual Object identity'};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {heap,success:false,error:'opaque Error extensibility requires its actual host protocol'};
  const replacement = {...object,extensible:false};
  return {heap:replaceJavaScriptOrdinaryObject(heap,replacement),success:true,error:''};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @param {JavaScriptSemanticValue} prototype
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string}} */
export function setJavaScriptOrdinaryPrototype(heap, identity, prototype) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  if (heapError !== '') return {heap,success:false,error:heapError};
  if (prototype.$ !== 'null' && prototype.$ !== 'object') return {heap,success:false,error:'an ordinary prototype must be null or a declared Object'};
  const error = readJavaScriptOrdinaryValueError(heap, prototype);
  if (error !== '') return {heap,success:false,error};
  const index = findJavaScriptOrdinaryObject(heap.objects, identity);
  const object = heap.objects[index];
  if (object === undefined) return {heap,success:false,error:'SetPrototypeOf needs its actual Object identity'};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {heap,success:false,error:'opaque Error prototypes require their actual host protocol'};
  if (testJavaScriptSemanticSameValue(object.prototype, prototype)) return {heap,success:true,error:''};
  if (!object.extensible||object.$==='ordinaryObject'&&object.immutablePrototype===true) return {heap,success:false,error:''};
  let next = prototype;
  for (let depth = 0; depth <= heap.objects.length; depth += 1) {
    if (next.$ !== 'object') break;
    if (next.identity === identity) return {heap,success:false,error:''};
    const at = findJavaScriptOrdinaryObject(heap.objects, next.identity);
    const parent = heap.objects[at];
    if (parent === undefined) return {heap,success:false,error:'every ordinary prototype must remain in the global directory'};
    if(parent.$!=='ordinaryObject'&&parent.$!=='closureObject'&&parent.$!=='boundFunctionObject'&&parent.$!=='mapObject'&&parent.$!=='setObject'&&parent.$!=='weakMapObject'&&parent.$!=='weakSetObject'&&parent.$!=='collectionIteratorObject'&&parent.$!=='arrayObject')return {heap,success:false,error:'ordinary prototype mutation cannot silently traverse an opaque Error'};
    next = parent.prototype;
  }
  const replacement = {...object,prototype};
  return {heap:replaceJavaScriptOrdinaryObject(heap,replacement),success:true,error:''};
}

/** A receiver value update has only [[Value]] when an own data property exists;
 * a new ordinary property has all three writable/enumerable/configurable flags.
 * @param {JavaScriptSemanticValue} value @param {boolean} creation
 * @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptOrdinarySetDescriptor(value, creation) {
  return {configurablePresent:creation,configurable:creation,enumerablePresent:creation,enumerable:creation,
    valuePresent:true,value,writablePresent:creation,writable:creation,
    getterPresent:false,getter:{$:'undefined'},setterPresent:false,setter:{$:'undefined'}};
}

/** OrdinarySetWithOwnDescriptor preserves the actual receiver, including a
 * primitive receiver for a callable inherited setter. A pending call is not a
 * completed success: the evaluator must execute it, propagate abrupt completion
 * and resume using its actual latest shared heap, never this earlier snapshot.
 * Strict assignment versus Reflect.set handles a completed false separately.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value
 * @param {JavaScriptSemanticValue} receiver
 * @returns {JavaScriptOrdinarySetPlan} */
export function planJavaScriptOrdinarySet(heap, identity, key, value, receiver) {
 return planJavaScriptOrdinarySetDefinition(heap,identity,key,value,receiver,false);
}
/** @typedef {{heap:JavaScriptOrdinaryHeap,action:string,error:string,success:boolean,callee:JavaScriptSemanticValue,receiver:JavaScriptSemanticValue,arguments:JavaScriptSemanticValue[],definition?:JavaScriptPropertyDescriptor}} JavaScriptOrdinarySetPlan */
/** Keep the legacy synchronous planner for its pure controls. The machine uses
 * this selector to execute genuine Array length conversion callbacks.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value
 * @param {JavaScriptSemanticValue} receiver @returns {JavaScriptOrdinarySetPlan} */
export function planJavaScriptOrdinarySetWithArrayDefinition(heap,identity,key,value,receiver){
 return planJavaScriptOrdinarySetDefinition(heap,identity,key,value,receiver,true);
}
/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} key @param {JavaScriptSemanticValue} value
 * @param {JavaScriptSemanticValue} receiver @param {boolean} deferArrayLength
 * @returns {JavaScriptOrdinarySetPlan} */
export function planJavaScriptOrdinarySetDefinition(heap, identity, key, value, receiver,deferArrayLength) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  const keyError = readJavaScriptOrdinaryKeyError(heap, key);
  const valueError = readJavaScriptOrdinaryValueError(heap, value);
  const receiverError = readJavaScriptOrdinaryValueError(heap, receiver);
  const error = heapError !== '' ? heapError : keyError !== '' ? keyError : valueError !== '' ? valueError : receiverError !== '' ? receiverError
    : findJavaScriptOrdinaryObject(heap.objects, identity) < 0 ? 'an ordinary Set needs its actual Object identity' : '';
  if (error !== '') return {heap,action:'error',error,success:false,callee:{$:'undefined'},receiver,arguments:[]};
  const target=heap.objects[findJavaScriptOrdinaryObject(heap.objects,identity)];
  if(target!==undefined&&target.$!=='ordinaryObject'&&target.$!=='closureObject'&&target.$!=='boundFunctionObject'&&target.$!=='mapObject'&&target.$!=='setObject'&&target.$!=='weakMapObject'&&target.$!=='weakSetObject'&&target.$!=='collectionIteratorObject'&&target.$!=='arrayObject')return {heap,action:'host call',error:'',success:false,callee:target.hostSet??{$:'undefined'},receiver:{$:'object',identity,callable:false},arguments:[key,value,receiver]};
  const located = locateJavaScriptOrdinaryProperty(heap, identity, key);
  const own = located.found ? located.descriptor : createJavaScriptOrdinarySetDescriptor({$:'undefined'},true);
  if (own.getterPresent) {
    const setter = own.setter;
    if (setter.$ === 'undefined') return {heap,action:'complete',error:'',success:false,callee:{$:'undefined'},receiver,arguments:[]};
    return {heap,action:'call',error:'',success:false,callee:setter,receiver,arguments:[value]};
  }
  if (!own.writable) return {heap,action:'complete',error:'',success:false,callee:{$:'undefined'},receiver,arguments:[]};
  if (receiver.$ !== 'object') return {heap,action:'complete',error:'',success:false,callee:{$:'undefined'},receiver,arguments:[]};
  const index = findJavaScriptOrdinaryObject(heap.objects, receiver.identity);
  const object = heap.objects[index];
  if (object === undefined) return {heap,action:'error',error:'Set needs its actual receiver Object',success:false,callee:{$:'undefined'},receiver,arguments:[]};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {heap,action:'error',error:'a borrowed ordinary descriptor cannot replace an opaque Error receiver protocol',success:false,callee:{$:'undefined'},receiver,arguments:[]};
  const at = findJavaScriptOrdinaryProperty(object.properties, key);
  const property = object.properties[at];
  if (property !== undefined && (!property.descriptor.valuePresent || !property.descriptor.writable))
    return {heap,action:'complete',error:'',success:false,callee:{$:'undefined'},receiver,arguments:[]};
  const proposed = createJavaScriptOrdinarySetDescriptor(value,property === undefined);
  if(deferArrayLength&&object.$==='arrayObject'&&testJavaScriptArrayLengthKey(key))return {heap,action:'array definition',error:'',success:false,callee:{$:'undefined'},receiver,arguments:[],definition:proposed};
  const transition = defineJavaScriptOrdinaryProperty(heap,receiver.identity,key,proposed);
  return {heap:transition.heap,action:transition.error === '' ? 'complete' : 'error',error:transition.error,success:transition.success,callee:{$:'undefined'},receiver,arguments:[]};
}

/** OrdinaryOwnPropertyKeys: numeric text first in numeric order, then other
 * text in creation order, then Symbols in creation order. The semantic keys
 * retain exact UTF16 units and global Symbol identities, without host coercion.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @returns {{keys:JavaScriptSemanticValue[],error:string}} */
export function readJavaScriptOrdinaryOwnKeys(heap, identity) {
  const error = readJavaScriptOrdinaryHeapError(heap);
  if (error !== '') return {keys:[],error};
  const index = findJavaScriptOrdinaryObject(heap.objects, identity);
  const object = heap.objects[index];
  if (object === undefined) return {keys:[],error:'OwnPropertyKeys needs its actual Object identity'};
  if(object.$!=='ordinaryObject'&&object.$!=='closureObject'&&object.$!=='boundFunctionObject'&&object.$!=='mapObject'&&object.$!=='setObject'&&object.$!=='weakMapObject'&&object.$!=='weakSetObject'&&object.$!=='collectionIteratorObject'&&object.$!=='arrayObject')return {keys:[],error:'opaque Error own keys require their actual host protocol'};
  let numeric = [];
  let positions = [];
  let text = [];
  let symbols = [];
  for (const property of object.properties) {
    const key = property.key;
    if (key.$ === 'text'||key.$==='sourceText') {
      const position = readJavaScriptSemanticTextLength(key)>10?-1:readJavaScriptLiteralPropertyIndex(constructJavaScriptSemanticTextUnits(key,10));
      if (position < 0) text = text.concat([key]);
      else {
        let at = 0;
        while (at < positions.length && (positions[at] ?? -1) < position) at += 1;
        numeric = numeric.slice(0,at).concat([key],numeric.slice(at));
        positions = positions.slice(0,at).concat([position],positions.slice(at));
      }
    } else symbols = symbols.concat([key]);
  }
  return {keys:numeric.concat(text,symbols),error:''};
}

/** Insert an externally allocated global link identity. This operation does
 * not run a separate allocator or invent callable-code/prototype facts.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @param {JavaScriptSemanticValue} prototype
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string}} */
export function insertJavaScriptOrdinaryObject(heap, identity, prototype) {
  const heapError = readJavaScriptOrdinaryHeapError(heap);
  if (heapError !== '') return {heap,success:false,error:heapError};
  if (heap.objects.length >= 4096) return {heap,success:false,error:'ordinary heap identity directory exceeds its explicit resource budget'};
  if (identity <= 0n || identity > 18446744073709551615n || findJavaScriptOrdinaryObject(heap.objects,identity) >= 0 || testJavaScriptOrdinarySymbol(heap.symbols,identity))
    return {heap,success:false,error:'a fresh ordinary Object requires its distinct externally allocated unsigned64 global identity'};
  if (prototype.$ !== 'null' && prototype.$ !== 'object') return {heap,success:false,error:'an ordinary prototype must be null or a declared Object'};
  const error = readJavaScriptOrdinaryValueError(heap,prototype);
  if (error !== '') return {heap,success:false,error};
  if(prototype.$==='object'){const parent=heap.objects[findJavaScriptOrdinaryObject(heap.objects,prototype.identity)];if(parent!==undefined&&parent.$!=='ordinaryObject'&&parent.$!=='closureObject'&&parent.$!=='boundFunctionObject'&&parent.$!=='mapObject'&&parent.$!=='setObject'&&parent.$!=='weakMapObject'&&parent.$!=='weakSetObject'&&parent.$!=='collectionIteratorObject'&&parent.$!=='arrayObject')return {heap,success:false,error:'ordinary allocation with an opaque Error prototype needs its explicit host traversal protocol'};}
  let at = 0;
  for (const object of heap.objects) if (object.identity < identity) at += 1;
  const object = {$:'ordinaryObject',identity,prototype,extensible:true,callable:false,properties:[]};
  const objects = heap.objects.slice(0,at).concat([object],heap.objects.slice(at));
  return {heap:{...heap,objects},success:true,error:''};
}

/** @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity
 * @returns {{heap:JavaScriptOrdinaryHeap,success:boolean,error:string}} */
export function registerJavaScriptOrdinarySymbol(heap, identity) {
  const error = readJavaScriptOrdinaryHeapError(heap);
  if (error !== '') return {heap,success:false,error};
  if (heap.symbols.length >= 16384) return {heap,success:false,error:'ordinary heap identity directory exceeds its explicit resource budget'};
  if (identity <= 0n || identity > 18446744073709551615n || findJavaScriptOrdinaryObject(heap.objects,identity) >= 0 || testJavaScriptOrdinarySymbol(heap.symbols,identity))
    return {heap,success:false,error:'a fresh Symbol requires its distinct externally allocated unsigned64 global identity'};
  let at = 0;
  for (const symbol of heap.symbols) if (symbol < identity) at += 1;
  const symbols = heap.symbols.slice(0,at).concat([identity],heap.symbols.slice(at));
  return {heap:{...heap,symbols},success:true,error:''};
}
