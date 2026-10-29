// Complete source-authoring binding decisions. Actual token structure and
// module ownership are supplied by the source parser, never executed values.
import {acceptJavaScriptSourceBindingName as admittedName} from './source-identifier-rules.js';
/** @param {string} keyword @param {string} kind @param {string} name
 * @param {boolean} callable @returns {string} */
export const readSourceLibraryDeclarationBindingKind=function(keyword,kind,name,callable){
 if(!admittedName(kind,name,false,[]))return '';
 return keyword==='function'?'function':keyword==='const'&&callable?'const':'';
};
/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptSourceFunctionHeadName(kind){return kind==='identifier';}
/** @param {string} first @param {string} second @param {boolean} lineBreak
 * @returns {number} */
export function readSourceLibraryFunctionDeclarationOffset(first,second,lineBreak){
 return first==='function'?0:first==='async'&&second==='function'&&!lineBreak?1:-1;
}
