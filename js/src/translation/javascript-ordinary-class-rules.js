// Base ordinary classes retain their actual call/new and completion behavior.
// Derived Error constructors retain their separate existing protocol.
import {findJavaScriptOrdinaryObject,findJavaScriptOrdinaryProperty} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextString,testJavaScriptSemanticSameValue} from './javascript-semantic-values.js';
/** @param {string} mode @returns {boolean} */
export function testJavaScriptOrdinaryClassConstructorMode(mode){return mode==='ordinary class';}
/** @param {string} mode @param {string} result @returns {boolean} */
export function testJavaScriptOrdinaryClassReceiverResult(mode,result){return testJavaScriptOrdinaryClassConstructorMode(mode)&&result!=='object';}
/** The closed source profile retains actual named declarations and prohibits
 * descriptor replacement. This is the pinned Node 22/24 class-call message,
 * selected from the actual callable own data name rather than a source guess.
 * @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @returns {string} */
export function readJavaScriptOrdinaryClassCallReason(heap,identity){
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,identity)];if(object===undefined||object.$!=='ordinaryObject')return '';
 const field=object.properties[findJavaScriptOrdinaryProperty(object.properties,{$:'text',units:[110,97,109,101]})];
 if(field===undefined||!field.descriptor.valuePresent||field.descriptor.getterPresent||field.descriptor.setterPresent)return '';
 const value=field.descriptor.value;if(value.$!=='text'&&value.$!=='sourceText')return '';
 return 'Class constructor '+readJavaScriptSemanticTextString(value)+" cannot be invoked without 'new'";
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} callee
 * @param {JavaScriptSemanticValue} receiver @returns {string} */
export function readJavaScriptOrdinaryClassReceiverError(heap,callee,receiver){
 if(callee.$!=='object'||!callee.callable||receiver.$!=='object'||receiver.callable)return 'ordinary class construction needs its actual callable and noncallable allocated receiver';
 const constructor=heap.objects[findJavaScriptOrdinaryObject(heap.objects,callee.identity)],instance=heap.objects[findJavaScriptOrdinaryObject(heap.objects,receiver.identity)];
 if(constructor===undefined||constructor.$!=='ordinaryObject'||instance===undefined||instance.$!=='ordinaryObject')return 'ordinary class construction cannot substitute opaque constructor or receiver kinds';
 const field=constructor.properties[findJavaScriptOrdinaryProperty(constructor.properties,{$:'text',units:[112,114,111,116,111,116,121,112,101]})];
 return field!==undefined&&field.descriptor.valuePresent&&!field.descriptor.getterPresent&&!field.descriptor.setterPresent&&field.descriptor.value.$==='object'&&testJavaScriptSemanticSameValue(field.descriptor.value,instance.prototype)?'':'the allocated class receiver retains its actual constructor own data prototype';
}
