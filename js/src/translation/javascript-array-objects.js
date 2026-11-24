// Array exotic descriptor semantics. Numeric conversion is a separate resumable
// operation: callers must run BOTH conversions before reading the current length.
// https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arraysetlength
import {createJavaScriptPropertyDescriptor,planJavaScriptPropertyDefinition} from './javascript-property-descriptors.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticTextLength,constructJavaScriptSemanticTextUnits} from './javascript-semantic-values.js';
import {readJavaScriptLiteralPropertyIndex} from './javascript-literal-property-rules.js';
import {findJavaScriptHeapDirectoryProperty} from './javascript-ordinary-heap-directory.js';
/** @typedef {{$:'arrayPropertyPlan',object:JavaScriptOrdinaryObject,success:boolean,error:string,errorClass:string,writes:string[]}} JavaScriptArrayPropertyPlan */
/** @returns {JavaScriptSemanticValue} */
export function readJavaScriptArrayLengthKey(){return {$:'text',units:[108,101,110,103,116,104]};}
/** @param {JavaScriptSemanticValue} key @returns {boolean} */
export function testJavaScriptArrayLengthKey(key){return testJavaScriptSemanticSameValue(key,readJavaScriptArrayLengthKey());}
/** @param {JavaScriptSemanticValue} key @returns {number} */
export function readJavaScriptArrayIndex(key){const length=readJavaScriptSemanticTextLength(key);return length<0||length>10?-1:readJavaScriptLiteralPropertyIndex(constructJavaScriptSemanticTextUnits(key,10));}
/** @param {JavaScriptOrdinaryProperty[]} properties @param {JavaScriptSemanticValue} key @returns {number} */
export function findJavaScriptArrayProperty(properties,key){return findJavaScriptHeapDirectoryProperty(properties,key);}
/** @param {JavaScriptOrdinaryObject} object @returns {number} */
export function readJavaScriptArrayLength(object){
 const property=object.properties[findJavaScriptArrayProperty(object.properties,readJavaScriptArrayLengthKey())];
 return property===undefined?-1:readJavaScriptArrayLengthValue(property.descriptor.value);
}
/** @param {JavaScriptSemanticValue} value @returns {number} */
function readJavaScriptArrayLengthValue(value){
 switch(value.$){
  case 'number':return value.value;
  default:return -1;
 }
}
/** @param {number} length @returns {boolean} */
export function testJavaScriptArrayLength(length){return Number.isInteger(length)&&length>=0&&length<=4294967295&&!Object.is(length,-0);}
/** @param {JavaScriptOrdinaryObject} object @returns {string} */
export function readJavaScriptArrayObjectError(object){
 if(object.callable)return 'an Array exotic object is not callable';
 const property=object.properties[findJavaScriptArrayProperty(object.properties,readJavaScriptArrayLengthKey())];
 const invalid='an Array retains its actual nonenumerable nonconfigurable integral length data property';
 if(property===undefined||!property.descriptor.valuePresent||property.descriptor.getterPresent||property.descriptor.setterPresent||property.descriptor.enumerable||property.descriptor.configurable)return invalid;
 const length=readJavaScriptArrayLengthValue(property.descriptor.value);
 if(!testJavaScriptArrayLength(length))return invalid;
 for(const entry of object.properties){const index=readJavaScriptArrayIndex(entry.key);if(index>=length)return 'an Array index must remain below its actual length';}
 return '';
}
/** @param {bigint} identity @param {JavaScriptSemanticValue} prototype @param {number} length @returns {JavaScriptOrdinaryObject} */
export function constructJavaScriptArrayObject(identity,prototype,length){
 const descriptor=createJavaScriptPropertyDescriptor('data',true,false);descriptor.configurable=false;descriptor.writable=true;descriptor.value={$:'number',value:length};
 return {$:'arrayObject',identity,prototype,extensible:true,callable:false,properties:[{$:'ordinaryProperty',key:readJavaScriptArrayLengthKey(),descriptor}]};
}
/** @param {JavaScriptOrdinaryObject} object @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} proposed @returns {JavaScriptArrayPropertyPlan} */
export function planJavaScriptArrayOrdinaryDefinition(object,key,proposed){
 const at=findJavaScriptArrayProperty(object.properties,key),property=object.properties[at],current=property===undefined?createJavaScriptPropertyDescriptor('data',false,false):property.descriptor;
 const plan=planJavaScriptPropertyDefinition(property!==undefined,object.extensible,current,proposed);
 if(plan.disposition==='reject')return {$:'arrayPropertyPlan',object,success:false,error:plan.error,errorClass:'',writes:[]};
 if(plan.disposition==='keep')return {$:'arrayPropertyPlan',object,success:true,error:'',errorClass:'',writes:[]};
 if(at<0&&object.properties.length>=4096)return {$:'arrayPropertyPlan',object,success:false,error:'Array properties exceed their explicit ordered storage budget',errorClass:'resource',writes:[]};
 /** @type {JavaScriptOrdinaryProperty} */
 const replacement={$:'ordinaryProperty',key,descriptor:plan.descriptor};
 /** @type {JavaScriptOrdinaryProperty[]} */
 const properties=[];
 let index=0;
 for(const entry of object.properties){properties.push(index===at?replacement:entry);index+=1;}
 if(at<0)properties.push(replacement);
 return {$:'arrayPropertyPlan',object:{...object,properties},success:true,error:'',errorClass:'',writes:plan.writes};
}
/** Define an already converted numeric length. This runs against the ACTUAL
 * object AFTER both ToNumber effects. No source valueOf or getter executes here.
 * @param {JavaScriptOrdinaryObject} object @param {JavaScriptPropertyDescriptor} proposed
 * @param {number} unsignedLength @param {number} numberLength @returns {JavaScriptArrayPropertyPlan} */
export function planJavaScriptArrayLengthDefinition(object,proposed,unsignedLength,numberLength){
 if(!testJavaScriptArrayLength(unsignedLength))return {$:'arrayPropertyPlan',object,success:false,error:'array length continuation requires its actual unsigned32 conversion',errorClass:'invalid',writes:[]};
 if(unsignedLength!==numberLength)return {$:'arrayPropertyPlan',object,success:false,error:'Invalid array length',errorClass:'RangeError',writes:[]};
 const key=readJavaScriptArrayLengthKey(),at=findJavaScriptArrayProperty(object.properties,key),old=object.properties[at];
 if(old===undefined)return {$:'arrayPropertyPlan',object,success:false,error:'the actual Array length descriptor must remain present',errorClass:'invalid',writes:[]};
 const length=readJavaScriptArrayLength(object),descriptor={...proposed,valuePresent:true,value:{$:'number',value:unsignedLength}};
 if(unsignedLength>=length)return planJavaScriptArrayOrdinaryDefinition(object,key,descriptor);
 if(!old.descriptor.writable)return {$:'arrayPropertyPlan',object,success:false,error:'',errorClass:'',writes:[]};
 const newWritable=!descriptor.writablePresent||descriptor.writable,temporary=newWritable?descriptor:{...descriptor,writable:true};
 const initial=planJavaScriptArrayOrdinaryDefinition(object,key,temporary);if(!initial.success)return initial;
 // Array [[Delete]] has no user trap: the highest nonconfigurable index is
 // exactly the first descending deletion failure. One scan preserves the same
 // partial deletion and rollback without iterating billions of sparse holes.
 let blocked=-1;
 for(const property of object.properties){const index=readJavaScriptArrayIndex(property.key);if(index>=unsignedLength&&!property.descriptor.configurable&&index>blocked)blocked=index;}
 const finalLength=blocked<0?unsignedLength:blocked+1;
 /** @type {JavaScriptOrdinaryProperty[]} */
 const properties=[];
 for(const property of initial.object.properties){
  const index=readJavaScriptArrayIndex(property.key);
  if(index<0||index<finalLength){
   properties.push(testJavaScriptArrayLengthKey(property.key)?{...property,descriptor:{...property.descriptor,value:{$:'number',value:finalLength},writable:newWritable}}:property);
  }
 }
 return {$:'arrayPropertyPlan',object:{...initial.object,properties},success:blocked<0,error:'',errorClass:'',writes:initial.writes.concat(['array indices','array length'],!newWritable&&!initial.writes.includes('writable')?['writable']:[])};
}
/** @param {JavaScriptOrdinaryObject} object @param {JavaScriptSemanticValue} key @param {JavaScriptPropertyDescriptor} proposed @returns {JavaScriptArrayPropertyPlan} */
export function planJavaScriptArrayPropertyDefinition(object,key,proposed){
 if(testJavaScriptArrayLengthKey(key)){
  if(!proposed.valuePresent)return planJavaScriptArrayOrdinaryDefinition(object,key,proposed);
  const value=proposed.value;
  switch(value.$){
   case 'number':return planJavaScriptArrayLengthDefinition(object,proposed,convertJavaScriptArrayUnsignedLength(value.value),value.value);
   default:return {$:'arrayPropertyPlan',object,success:false,error:'array length requires its actual two numeric conversion continuations',errorClass:'conversion',writes:[]};
  }
 }
 const index=readJavaScriptArrayIndex(key),length=readJavaScriptArrayLength(object),lengthProperty=object.properties[findJavaScriptArrayProperty(object.properties,readJavaScriptArrayLengthKey())];
 if(index>=length&&(lengthProperty===undefined||!lengthProperty.descriptor.writable))return {$:'arrayPropertyPlan',object,success:false,error:'',errorClass:'',writes:[]};
 const plan=planJavaScriptArrayOrdinaryDefinition(object,key,proposed);if(!plan.success||index<length||index<0)return plan;
 /** @type {JavaScriptOrdinaryProperty[]} */
 const properties=[];
 for(const property of plan.object.properties){
  properties.push(testJavaScriptArrayLengthKey(property.key)?{...property,descriptor:{...property.descriptor,value:{$:'number',value:index+1}}}:property);
 }
 return {...plan,object:{...plan.object,properties},writes:plan.writes.concat(['array length'])};
}
/** @param {number} value @returns {number} */
export function convertJavaScriptArrayUnsignedLength(value){if(!Number.isFinite(value)||value===0)return 0;const integer=Math.trunc(value),remainder=integer%4294967296;return remainder<0?remainder+4294967296:remainder===0?0:remainder;}
