// Base ordinary classes retain their actual call/new and completion behavior.
// Derived Error constructors retain their separate existing protocol.
import {findJavaScriptHeapDirectoryObject,findJavaScriptHeapDirectoryProperty} from './javascript-ordinary-heap-directory.js';
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
 const object=heap.objects[findJavaScriptHeapDirectoryObject(heap.objects,identity)];
 if(object===undefined)return '';
 if(object.$!=='ordinaryObject')return '';
 const field=object.properties[findJavaScriptHeapDirectoryProperty(object.properties,{$:'text',units:[110,97,109,101]})];
 if(field===undefined||!field.descriptor.valuePresent||field.descriptor.getterPresent||field.descriptor.setterPresent)return '';
 const value=field.descriptor.value;if(value.$!=='text'&&value.$!=='sourceText')return '';
 return 'Class constructor '+readJavaScriptSemanticTextString(value)+" cannot be invoked without 'new'";
}
/** @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} callee
 * @param {JavaScriptSemanticValue} receiver @returns {string} */
export function readJavaScriptOrdinaryClassReceiverError(heap,callee,receiver){
 const invalidReceiver='ordinary class construction needs its actual callable and noncallable allocated receiver';
 if(callee.$!=='object')return invalidReceiver;
 if(!callee.callable)return invalidReceiver;
 if(receiver.$!=='object')return invalidReceiver;
 if(receiver.callable)return invalidReceiver;
 const constructor=heap.objects[findJavaScriptHeapDirectoryObject(heap.objects,callee.identity)],instance=heap.objects[findJavaScriptHeapDirectoryObject(heap.objects,receiver.identity)];
 const invalidKind='ordinary class construction cannot substitute opaque constructor or receiver kinds';
 if(constructor===undefined)return invalidKind;
 if(constructor.$!=='ordinaryObject')return invalidKind;
 if(instance===undefined)return invalidKind;
 if(instance.$!=='ordinaryObject')return invalidKind;
 const field=constructor.properties[findJavaScriptHeapDirectoryProperty(constructor.properties,{$:'text',units:[112,114,111,116,111,116,121,112,101]})];
 return field!==undefined&&field.descriptor.valuePresent&&!field.descriptor.getterPresent&&!field.descriptor.setterPresent&&field.descriptor.value.$==='object'&&testJavaScriptSemanticSameValue(field.descriptor.value,instance.prototype)?'':'the allocated class receiver retains its actual constructor own data prototype';
}
