// Already converted internal descriptors travel as twelve frame operands, not
// a new source Object whose inherited fields could change ToPropertyDescriptor.
import {createJavaScriptEmptyDescriptor} from './javascript-descriptor-conversion.js';
import {readJavaScriptPropertyDescriptorError} from './javascript-property-descriptors.js';
/** @param {JavaScriptPropertyDescriptor} descriptor @returns {JavaScriptSemanticValue[]} */
export function projectJavaScriptDescriptorOperands(descriptor){return [
 {$:'boolean',value:descriptor.configurablePresent},{$:'boolean',value:descriptor.configurable},
 {$:'boolean',value:descriptor.enumerablePresent},{$:'boolean',value:descriptor.enumerable},
 {$:'boolean',value:descriptor.valuePresent},descriptor.value,
 {$:'boolean',value:descriptor.writablePresent},{$:'boolean',value:descriptor.writable},
 {$:'boolean',value:descriptor.getterPresent},descriptor.getter,
 {$:'boolean',value:descriptor.setterPresent},descriptor.setter];}
/** @param {JavaScriptSemanticValue[]} operands @returns {{descriptor:JavaScriptPropertyDescriptor,error:string}} */
export function readJavaScriptDescriptorOperands(operands){
 const descriptor=createJavaScriptEmptyDescriptor();if(operands.length!==12)return {descriptor,error:'an internal property descriptor retains six actual presence and payload pairs'};
 const fields=['configurable','enumerable','value','writable','getter','setter'];
 for(let index=0;index<fields.length;index++){
  const field=fields[index],presence=operands[index*2],value=operands[index*2+1];if(field===undefined||presence?.$!=='boolean'||value===undefined)return {descriptor,error:'descriptor operand presence retains its actual Boolean and complete semantic payload'};
  descriptor[field+'Present']=presence.value;if(!presence.value)continue;
  if(index===0||index===1||index===3){if(value.$!=='boolean')return {descriptor,error:'present descriptor attribute operands are actual Booleans'};descriptor[field]=value.value;}else descriptor[field]=value;
 }
 return {descriptor,error:readJavaScriptPropertyDescriptorError(descriptor)};
}
