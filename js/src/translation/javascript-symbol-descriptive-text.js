import {readJavaScriptSemanticValueError,readJavaScriptSemanticTextLength,constructJavaScriptSemanticTextUnits} from './javascript-semantic-values.js';

/** Description is actual optional text, never a Symbol-to-string fallback.
 * @param {JavaScriptSemanticValue} description @returns {JavaScriptCallPrimitiveResult} */
export function constructJavaScriptSymbolDescriptiveText(description){
 const error=readJavaScriptSemanticValueError(description);if(error!==''||description.$!=='undefined'&&description.$!=='text'&&description.$!=='sourceText')return {$:'primitiveInvalid',reason:error||'Symbol descriptive text requires actual optional text metadata'};
 const length=description.$==='undefined'?0:readJavaScriptSemanticTextLength(description);
 if(length>1048568)return {$:'primitiveResourceLimit',reason:'Symbol descriptive text exceeds its selected 1048576-unit output profile'};
 const units=[83,121,109,98,111,108,40];if(description.$!=='undefined')for(const unit of constructJavaScriptSemanticTextUnits(description,1048576))units.push(unit);units.push(41);
 return {$:'primitiveValue',value:{$:'text',units}};
}
