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
 for(let index=0;index<6;index++){
  const presence=operands[index*2],value=operands[index*2+1];
  if(presence===undefined||presence.$!=='boolean'||value===undefined)return {descriptor,error:'descriptor operand presence retains its actual Boolean and complete semantic payload'};
  const present=presence.value;
  switch(index){
   case 0:
    descriptor.configurablePresent=present;
    if(present&&value.$==='boolean')descriptor.configurable=value.value;
    break;
   case 1:
    descriptor.enumerablePresent=present;
    if(present&&value.$==='boolean')descriptor.enumerable=value.value;
    break;
   case 2:
    descriptor.valuePresent=present;
    if(present)descriptor.value=value;
    break;
   case 3:
    descriptor.writablePresent=present;
    if(present&&value.$==='boolean')descriptor.writable=value.value;
    break;
   case 4:
    descriptor.getterPresent=present;
    if(present)descriptor.getter=value;
    break;
   case 5:
    descriptor.setterPresent=present;
    if(present)descriptor.setter=value;
    break;
  }
  if(present&&(index===0||index===1||index===3)&&value.$!=='boolean')return {descriptor,error:'present descriptor attribute operands are actual Booleans'};
 }
 return {descriptor,error:readJavaScriptPropertyDescriptorError(descriptor)};
}
