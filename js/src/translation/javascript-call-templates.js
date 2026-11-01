// Templates are code-only data, never fake callable heap objects. Each actual
// arrow evaluation requests a fresh global identity for its captured values.
import {findJavaScriptCallCodeFunction,testJavaScriptCallCodeCoordinate,readJavaScriptCallCodeDestination} from './javascript-call-code.js';
import {readJavaScriptOrdinaryHeapError,readJavaScriptOrdinaryValueError,findJavaScriptOrdinaryObject,testJavaScriptOrdinarySymbol} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticTextLength} from './javascript-semantic-values.js';
import {createJavaScriptPropertyDescriptor} from './javascript-property-descriptors.js';
/** @typedef {{$:'closureAllocationNormal',heap:JavaScriptOrdinaryHeap,value:JavaScriptSemanticValue} |
 * {$:'closureAllocationInvalid',heap:JavaScriptOrdinaryHeap,reason:string} |
 * {$:'closureAllocationResourceLimit',heap:JavaScriptOrdinaryHeap,reason:string}} JavaScriptClosureAllocation */
/** @param {JavaScriptCallProgram} program @param {bigint} owner @param {number} ordinal @returns {number} */
export function findJavaScriptCallTemplate(program,owner,ordinal){
 const templates=program.templates??[];let low=0,high=templates.length;
 while(low<high){const middle=Math.floor((low+high)/2),entry=templates[middle];if(entry===undefined)return -1;if(entry.owner===owner&&entry.ordinal===ordinal)return middle;if(entry.owner<owner||entry.owner===owner&&entry.ordinal<ordinal)low=middle+1;else high=middle;}
 return -1;
}
/** Body projection shares instruction validation without publishing a template
 * as a static callable. The supplied identity is either the real source owner
 * for validation or the real allocated closure identity for an actual call.
 * @param {JavaScriptCallTemplate} template @param {bigint} identity
 * @param {JavaScriptSemanticValue[]} captures @param {JavaScriptSemanticValue} receiver
 * @returns {JavaScriptCallFunction} */
export function projectJavaScriptCallTemplateFunction(template,identity,captures,receiver){
 return {$:'callFunction',identity,realm:template.realm,receiverMode:'lexical',lexicalReceiver:receiver,parameters:template.parameters,registers:template.registers,captures,instructions:template.instructions};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {bigint} identity @returns {JavaScriptCallFunction|undefined} */
export function readJavaScriptExecutableCallFunction(program,heap,identity){
 const staticIndex=findJavaScriptCallCodeFunction(program,identity);if(staticIndex>=0)return program.functions[staticIndex];
 const closure=heap.objects[findJavaScriptOrdinaryObject(heap.objects,identity)];if(closure===undefined||closure.$!=='closureObject')return undefined;
 const template=(program.templates??[])[findJavaScriptCallTemplate(program,closure.owner,closure.ordinal)];if(template===undefined||template.captureCount!==closure.captures.length)return undefined;
 return projectJavaScriptCallTemplateFunction(template,identity,closure.captures,closure.lexicalReceiver);
}
/** Validate static ownership and storage before any per-template projection.
 * Common instruction validation remains the machine's shared body validator.
 * @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @returns {string} */
export function readJavaScriptCallTemplateDirectoryError(program,heap){
 const templates=program.templates??[];if(templates.length>4096)return 'call templates exceed their explicit static directory budget';
 let owner=0n,ordinal=-1,instructions=0,captures=0;
 for(const template of templates){
  if(template.$!=='callTemplate'||typeof template.owner!=='bigint'||template.owner<=0n||template.owner>18446744073709551615n||!testJavaScriptCallCodeCoordinate(template.ordinal,4096)||template.owner<owner||template.owner===owner&&template.ordinal<=ordinal)return 'call templates retain ordered unique actual source owners and lexical site ordinals';
  owner=template.owner;ordinal=template.ordinal;
  const fn=program.functions[findJavaScriptCallCodeFunction(program,owner)];if(fn===undefined||fn.realm!==template.realm||!program.realms.includes(template.realm))return 'a template retains its actual registered source code owner and realm';
  if(!testJavaScriptCallCodeCoordinate(template.captureCount,4097)||!testJavaScriptCallCodeCoordinate(template.parameters,4097)||!testJavaScriptCallCodeCoordinate(template.registers,4097)||template.registers===0||template.parameters+template.captureCount+1>template.registers||!testJavaScriptCallCodeCoordinate(template.length,template.parameters+1))return 'call template registers, captures and actual Function length need bounded complete coordinates';
  captures+=template.captureCount;instructions+=template.instructions.length;if(captures>1048576||instructions>65536||template.instructions.length===0)return 'call template code and capture storage exceed the shared static budget';
  for(const instruction of template.instructions){const destination=readJavaScriptCallCodeDestination(instruction);if(destination===0||destination>template.parameters&&destination<=template.parameters+template.captureCount)return 'arrow code cannot overwrite its immutable lexical receiver or capture slots';}
  const prototypeError=readJavaScriptOrdinaryValueError(heap,template.prototype);if(prototypeError!=='')return prototypeError;
  if(template.prototype.$!=='object'||!template.prototype.callable)return 'arrow allocation retains its actual callable realm Function prototype';
  const nameError=readJavaScriptOrdinaryValueError(heap,template.name);if(nameError!=='')return nameError;
  if(template.name.$!=='text'&&template.name.$!=='sourceText'||readJavaScriptSemanticTextLength(template.name)>65536)return 'arrow templates retain their actual bounded semantic Function name';
 }
 for(const object of heap.objects)if(object.$==='closureObject'){
  if(findJavaScriptCallCodeFunction(program,object.identity)>=0)return 'a runtime closure identity cannot also name a static code directory entry';
  const template=templates[findJavaScriptCallTemplate(program,object.owner,object.ordinal)];if(template===undefined||template.captureCount!==object.captures.length)return 'every live closure retains its actual declared template and complete capture vector';
 }
 return '';
}
/** @param {JavaScriptSemanticValue} value @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptClosureMetadataDescriptor(value){const descriptor=createJavaScriptPropertyDescriptor('data',true,false);descriptor.value=value;descriptor.writable=false;return descriptor;}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap
 * @param {bigint} identity @param {bigint} owner @param {number} ordinal
 * @param {JavaScriptSemanticValue[]} captures @param {JavaScriptSemanticValue} receiver
 * @returns {JavaScriptClosureAllocation} */
export function allocateJavaScriptCallClosure(program,heap,identity,owner,ordinal,captures,receiver){
 const error=readJavaScriptOrdinaryHeapError(heap);if(error!=='')return {$:'closureAllocationInvalid',heap,reason:error};
 const directory=readJavaScriptCallTemplateDirectoryError(program,heap);if(directory!=='')return {$:'closureAllocationInvalid',heap,reason:directory};
 const template=(program.templates??[])[findJavaScriptCallTemplate(program,owner,ordinal)];if(template===undefined||captures.length!==template.captureCount)return {$:'closureAllocationInvalid',heap,reason:'closure allocation requires its actual template and exact capture vector'};
 if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||findJavaScriptOrdinaryObject(heap.objects,identity)>=0||testJavaScriptOrdinarySymbol(heap.symbols,identity))return {$:'closureAllocationInvalid',heap,reason:'closure allocation retains a genuinely fresh external global identity'};
 let captureCount=captures.length,properties=2;for(const object of heap.objects){properties+=object.properties.length;if(object.$==='closureObject')captureCount+=object.captures.length;}
 if(heap.objects.length>=4096||captureCount>1048576||properties>16384)return {$:'closureAllocationResourceLimit',heap,reason:'closure allocation exceeds the actual shared object, property or capture budget'};
 for(const value of [receiver,...captures]){const invalid=readJavaScriptOrdinaryValueError(heap,value);if(invalid!=='')return {$:'closureAllocationInvalid',heap,reason:invalid};}
/** @type {JavaScriptClosureObject} */ const closure={$:'closureObject',identity,prototype:template.prototype,extensible:true,callable:true,owner,ordinal,captures:[...captures],lexicalReceiver:receiver,properties:[{$:'ordinaryProperty',key:{$:'text',units:[108,101,110,103,116,104]},descriptor:createJavaScriptClosureMetadataDescriptor({$:'number',value:template.length})},{$:'ordinaryProperty',key:{$:'text',units:[110,97,109,101]},descriptor:createJavaScriptClosureMetadataDescriptor(template.name)}]};
 let at=0;while(at<heap.objects.length&&(heap.objects[at]?.identity??identity)<identity)at++;
 const next={...heap,objects:heap.objects.slice(0,at).concat([closure],heap.objects.slice(at))},invalid=readJavaScriptOrdinaryHeapError(next);
 return invalid!==''?{$:'closureAllocationInvalid',heap,reason:invalid}:{$:'closureAllocationNormal',heap:next,value:{$:'object',identity,callable:true}};
}
