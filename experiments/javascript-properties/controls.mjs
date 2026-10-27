// Deferred controls only; never import during implementation source authoring.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {planJavaScriptPropertyDefinition} from '../../js/src/translation/javascript-property-descriptors.js';
import {testJavaScriptSemanticSameValue,readJavaScriptSemanticValueError} from '../../js/src/translation/javascript-semantic-values.js';
export function assertVersions(){const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');}
export const fixture=readProjectData(new URL('../../parity/fixtures/javascript-property-descriptors.lino',import.meta.url));
export function numberFromBits(bits){const bytes=new ArrayBuffer(8),view=new DataView(bytes);view.setBigUint64(0,BigInt('0x'+bits));return view.getFloat64(0);}
function numberBits(value){const bytes=new ArrayBuffer(8),view=new DataView(bytes);view.setFloat64(0,value);return view.getBigUint64(0).toString(16).padStart(16,'0');}
export function decodeValue(value){switch(value.$){case 'number':return {$:'number',value:numberFromBits(value.bits)};case 'integer':return {$:'integer',value:BigInt(value.value)};case 'symbol':return {...value,identity:BigInt(value.identity)};case 'object':return {...value,identity:BigInt(value.identity)};default:return value;}}
export function encodeValue(value){switch(value.$){case 'number':return {$:'number',bits:numberBits(value.value)};case 'integer':return {$:'integer',value:String(value.value)};case 'symbol':return {...value,identity:String(value.identity)};case 'object':return {...value,identity:String(value.identity)};default:return value;}}
export function decodeDescriptor(value){return {...value,value:decodeValue(value.value),getter:decodeValue(value.getter),setter:decodeValue(value.setter)};}
export function encodeDescriptor(value){return {...value,value:encodeValue(value.value),getter:encodeValue(value.getter),setter:encodeValue(value.setter)};}
// A single identity map per control models the required global context. Its
// callability facts cannot disagree for the same object identity.
export function createHostContext(){
 const identities=new Map(),facts=new Map();let calls=0;
 const materialize=value=>{switch(value.$){
  case 'undefined':return undefined;case 'null':return null;case 'boolean':case 'number':case 'integer':return value.value;
  case 'text':{let output='';for(let at=0;at<value.units.length;at+=4096)output+=String.fromCharCode(...value.units.slice(at,at+4096));return output;}
  case 'object':case 'symbol':{const key=value.$+':'+value.identity;if(value.$==='object'){if(facts.has(key))assert.equal(facts.get(key),value.callable);facts.set(key,value.callable);}
   if(!identities.has(key))identities.set(key,value.$==='symbol'?Symbol(key):value.callable?function(){calls+=1;}:{});return identities.get(key);}
  default:throw new TypeError('uncovered semantic value '+value.$);
 }};
 return {materialize,calls:()=>calls};
}
function hostDescriptor(descriptor,context){const output={};for(const [field,host]of [['configurable','configurable'],['enumerable','enumerable'],['value','value'],['writable','writable'],['getter','get'],['setter','set']])if(descriptor[field+'Present'])output[host]=['value','getter','setter'].includes(field)?context.materialize(descriptor[field]):descriptor[field];return output;}
function assertHostDescriptor(actual,expected,context){const planned=hostDescriptor(expected,context);assert.deepEqual(Reflect.ownKeys(actual).sort(),Reflect.ownKeys(planned).sort());for(const key of Reflect.ownKeys(planned))assert.ok(Object.is(actual[key],planned[key]),'complete descriptor field '+key);}
export function observePropertyCase(row){
 const current=decodeDescriptor(row.current),proposed=decodeDescriptor(row.proposed),result=planJavaScriptPropertyDefinition(row.exists,row.extensible,current,proposed);
 assert.equal(result.disposition,row.disposition,row.name);assert.equal(result.error,row.error,row.name);assert.deepEqual(result.writes,row.writes,row.name);
 if(row.oracle==='ordinary'){
  const context=createHostContext(),receiver={};if(row.exists)Object.defineProperty(receiver,'property',hostDescriptor(current,context));if(!row.extensible)Object.preventExtensions(receiver);
  let accepted,error='';try{accepted=Reflect.defineProperty(receiver,'property',hostDescriptor(proposed,context));}catch(observed){error=observed.name;accepted=false;}
  assert.equal(error,result.error,row.name);assert.equal(accepted,result.disposition!=='reject',row.name);
  const actual=Object.getOwnPropertyDescriptor(receiver,'property');
  if(accepted)assertHostDescriptor(actual,result.descriptor,context);else if(row.exists)assertHostDescriptor(actual,current,context);else assert.equal(actual,undefined);
  assert.equal(context.calls(),0,'defining a property never executes its getter or setter');
 }
 if(result.disposition==='keep'||result.disposition==='reject')assert.deepEqual(encodeDescriptor(result.descriptor),encodeDescriptor(current),'no writes retain every original stored bit and absent payload');
 return {name:row.name,exists:row.exists,extensible:row.extensible,current:row.current,proposed:row.proposed,result:{...result,descriptor:encodeDescriptor(result.descriptor)}};
}
export function observeSameValueCase(row){const left=decodeValue(row.left),right=decodeValue(row.right),context=createHostContext(),actual=testJavaScriptSemanticSameValue(left,right);assert.equal(actual,row.expected,row.name);assert.equal(actual,Object.is(context.materialize(left),context.materialize(right)),row.name);return {...row,actual};}
export function observeInvalidValue(row){const value=decodeValue(row.value),actual=readJavaScriptSemanticValueError(value);assert.equal(actual,row.message,row.name);assert.throws(()=>testJavaScriptSemanticSameValue(value,{$:'undefined'}),{name:'TypeError'});return {...row,actual};}
export function observeTextBoundary(row){const value={$:'text',units:Array(row.units).fill(65)},actual=readJavaScriptSemanticValueError(value);assert.equal(actual,row.error);if(!actual)assert.equal(testJavaScriptSemanticSameValue(value,{$:'text',units:[...value.units]}),true);return {...row,actual};}
