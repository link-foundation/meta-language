// Native output DATA observer only. Slot readers inspect already materialized
// carriers; represented source, getters and methods are never fallback bodies.
import {types as nativeTypes} from 'node:util';
const descriptors=Reflect.getOwnPropertyDescriptor,keys=Reflect.ownKeys,getPrototype=Reflect.getPrototypeOf,isExtensible=Reflect.isExtensible,apply=Reflect.apply;
const slotReaders=[{kind:'number wrapper',test:nativeTypes.isNumberObject,reader:descriptors(Number.prototype,'valueOf').value},{kind:'boolean wrapper',test:nativeTypes.isBooleanObject,reader:descriptors(Boolean.prototype,'valueOf').value},{kind:'integer wrapper',test:nativeTypes.isBigIntObject,reader:descriptors(BigInt.prototype,'valueOf').value},{kind:'symbol wrapper',test:nativeTypes.isSymbolObject,reader:descriptors(Symbol.prototype,'valueOf').value},{kind:'string wrapper',test:nativeTypes.isStringObject,reader:descriptors(String.prototype,'valueOf').value}];
const prototypeRoots=[{name:'Object.prototype',value:Object.prototype},{name:'Array.prototype',value:Array.prototype},{name:'Number.prototype',value:Number.prototype},{name:'Boolean.prototype',value:Boolean.prototype},{name:'BigInt.prototype',value:BigInt.prototype},{name:'Symbol.prototype',value:Symbol.prototype},{name:'String.prototype',value:String.prototype}];
const symbolConstructor=Symbol,symbolDescription=descriptors(Symbol.prototype,'description').get,symbolRegistry=Symbol.keyFor;
const wellKnownNames=['asyncDispose','asyncIterator','dispose','hasInstance','isConcatSpreadable','iterator','match','matchAll','replace','search','species','split','toPrimitive','toStringTag','unscopables'];
/** A JSON-safe signature retains data slots, exact UTF16, per-graph object and
 * Symbol identities, own descriptors, extensibility and supplied prototype roots.
 * NaN and signed-zero observations are explicit declarations, not lossy JSON.
 * @param {unknown} value @returns {object} */
export function observeWrapperMaterialization(value){
 const identities=new Map(),objects=[],symbols=[];let next=1,propertyCount=0,textUnits=0;
 function text(value){if(value.length>16777216||textUnits+value.length>67108864)throw new RangeError('wrapper observation exceeds its explicit UTF16 storage profile');textUnits+=value.length;const units=[];for(let at=0;at<value.length;at+=1)units.push(value.charCodeAt(at));return {text:units};}
 function encode(value){
  if(value===undefined)return {undefined:true};if(value===null)return {null:true};
  if(typeof value==='boolean')return {boolean:value};
  if(typeof value==='number')return {number:value!==value?'NaN':value===0&&1/value===-Infinity?'negative zero':value===Infinity?'positive infinity':value===-Infinity?'negative infinity':value};
  if(typeof value==='bigint')return {integer:String(value)};
  if(typeof value==='string')return text(value);
  if(typeof value==='symbol'){
   if(identities.has(value))return {symbol:identities.get(value)};if(symbols.length>=16384)throw new RangeError('wrapper observation exceeds its explicit Symbol profile');const identity=next++;identities.set(value,identity);
   const description=apply(symbolDescription,value,[]),record={identity,description:description===undefined?{undefined:true}:text(description)},known=wellKnownNames.find(name=>descriptors(symbolConstructor,name)?.value===value),registry=apply(symbolRegistry,symbolConstructor,[value]);
   if(known!==undefined)record.wellKnown=known;if(registry!==undefined)record.registry=text(registry);symbols.push(record);return {symbol:identity};
  }
  if(typeof value==='function')throw new TypeError('wrapper observation cannot execute or transport represented callable bodies');
  if(typeof value!=='object')throw new TypeError('wrapper observation requires an actual supported data value');
  if(identities.has(value))return {reference:identities.get(value)};if(objects.length>=4096)throw new RangeError('wrapper observation exceeds its explicit Object profile');
  const identity=next++;identities.set(value,identity);const selected=slotReaders.find(slot=>slot.test(value)),prototype=getPrototype(value),root=prototype===null?'null':prototypeRoots.find(row=>row.value===prototype)?.name;
  if(root===undefined)throw new TypeError('wrapper observation refuses unsupported custom prototypes');
  const record={identity,kind:selected?.kind??(Array.isArray(value)?'array':'object'),prototype:root,extensible:isExtensible(value),properties:[]};objects.push(record);
  if(selected!==undefined)record.slot=encode(apply(selected.reader,value,[]));
  const own=keys(value);if(own.length>4096||propertyCount+own.length>16384)throw new RangeError('wrapper observation exceeds its explicit own-property profile');propertyCount+=own.length;
  for(const key of own){const descriptor=descriptors(value,key);if(!descriptor||!('value'in descriptor))throw new TypeError('wrapper observation cannot execute or bypass an accessor continuation');record.properties.push({key:encode(key),value:encode(descriptor.value),writable:descriptor.writable,enumerable:descriptor.enumerable,configurable:descriptor.configurable});}
  return {reference:identity};
 }
 const root=encode(value);return {root,objects,symbols};
}
