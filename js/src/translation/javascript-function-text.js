// Source text is compiler-supplied CST metadata, never a reconstructed body or
// an execution certificate. Native representations require an independent port.
import {findJavaScriptOrdinaryObject,readJavaScriptOrdinaryValueError} from './javascript-ordinary-heap.js';
import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,readJavaScriptSemanticTextUnit,constructJavaScriptSourceText} from './javascript-semantic-values.js';
import {findJavaScriptCallCodeFunction} from './javascript-call-code.js';
import {findJavaScriptCallTemplate} from './javascript-call-templates.js';
/** @typedef {{$:'callSourceUnit',identifier:string,text:JavaScriptSemanticValue}} JavaScriptCallSourceUnit */
/** @typedef {{$:'callSourceSpan',unit:string,start:number,end:number}} JavaScriptCallSourceSpan */
/** @typedef {{$:'functionTextValue',value:JavaScriptSemanticValue} | {$:'functionTextNeedAuthority',kind:string} | {$:'functionTextError',errorClass:string,reason:string} | {$:'functionTextInvalid',reason:string}} JavaScriptFunctionTextResult */
/** @param {JavaScriptCallSourceUnit[]} sources @param {JavaScriptCallSourceSpan} span @returns {string} */
export function readJavaScriptFunctionSourceSpanError(sources,span){
 if(span===null||typeof span!=='object')return 'function source spans retain actual nonempty UTF16 intervals in a supplied source unit';
 const unit=sources.find(row=>row.identifier===span.unit);return span.$!=='callSourceSpan'||unit===undefined||!Number.isSafeInteger(span.start)||Object.is(span.start,-0)||span.start<0||!Number.isSafeInteger(span.end)||span.end<=span.start||span.end>readJavaScriptSemanticTextLength(unit.text)?'function source spans retain actual nonempty UTF16 intervals in a supplied source unit':'';
}
/** @param {JavaScriptCallProgram} program @returns {string} */
export function readJavaScriptFunctionSourceDirectoryError(program){
 const sources=program.sources===undefined?[]:program.sources;if(!Array.isArray(sources)||sources.length>4096)return 'function source units exceed their shared static directory budget';let total=0;
 for(let index=0;index<sources.length;index++){const row=sources[index];if(row===null||typeof row!=='object'||row.$!=='callSourceUnit'||typeof row.identifier!=='string'||row.identifier.length===0||row.identifier.length>8192||sources.slice(0,index).some(prior=>prior.identifier===row.identifier)||row.text===null||typeof row.text!=='object'||row.text.$!=='text'&&row.text.$!=='sourceText'||readJavaScriptSemanticValueError(row.text)!=='')return 'function source units retain unique complete supplied source identifiers and text';total+=readJavaScriptSemanticTextLength(row.text);if(total>16777216)return 'function source units exceed their shared UTF16 storage profile';}
 for(const fn of program.functions)if(fn.source!==undefined){const error=readJavaScriptFunctionSourceSpanError(sources,fn.source);if(error!=='')return error;if(!['strict','ordinary class','derived Error'].includes(fn.receiverMode))return 'portable intrinsic bodies do not establish original function source text';}
 for(const template of program.templates??[])if(template.source!==undefined){const error=readJavaScriptFunctionSourceSpanError(sources,template.source);if(error!=='')return error;const owner=program.functions[findJavaScriptCallCodeFunction(program,template.owner)];if(owner?.source===undefined||owner.source.unit!==template.source.unit||template.source.start<owner.source.start||template.source.end>owner.source.end)return 'lexical source sites remain inside their actual original source owner';}
 return '';
}
/** @param {JavaScriptCallSourceUnit[]} sources @param {JavaScriptCallSourceSpan} span @returns {JavaScriptSemanticValue} */
export function projectJavaScriptFunctionSourceText(sources,span){
 const unit=sources.find(row=>row.identifier===span.unit);if(unit===undefined)throw new TypeError('source projection requires its actual retained source unit');
 if(unit.text.$==='sourceText')return constructJavaScriptSourceText(unit.text.text.slice(span.start,span.end));
 const units=[];for(let index=span.start;index<span.end;index++)units.push(readJavaScriptSemanticTextUnit(unit.text,index));return {$:'text',units};
}
/** @param {JavaScriptCallProgram} program @param {JavaScriptOrdinaryHeap} heap @param {JavaScriptSemanticValue} input @returns {JavaScriptFunctionTextResult} */
export function readJavaScriptFunctionText(program,heap,input){
 const error=readJavaScriptOrdinaryValueError(heap,input)||readJavaScriptFunctionSourceDirectoryError(program);if(error!=='')return {$:'functionTextInvalid',reason:error};
 if(input.$!=='object'||!input.callable)return {$:'functionTextError',errorClass:'TypeError',reason:'Function.prototype.toString requires a callable receiver'};
 const object=heap.objects[findJavaScriptOrdinaryObject(heap.objects,input.identity)];if(object?.$==='boundFunctionObject')return {$:'functionTextNeedAuthority',kind:'bound'};
 if(object?.$==='closureObject'){const template=(program.templates??[])[findJavaScriptCallTemplate(program,object.owner,object.ordinal)];return template?.source===undefined?{$:'functionTextNeedAuthority',kind:'source'}:{$:'functionTextValue',value:projectJavaScriptFunctionSourceText(program.sources??[],template.source)};}
 const fn=program.functions[findJavaScriptCallCodeFunction(program,input.identity)];return fn?.source!==undefined?{$:'functionTextValue',value:projectJavaScriptFunctionSourceText(program.sources??[],fn.source)}:{$:'functionTextNeedAuthority',kind:fn!==undefined&&['strict','ordinary class','derived Error'].includes(fn.receiverMode)?'source':'external'};
}
