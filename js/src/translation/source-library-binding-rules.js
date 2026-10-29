// Complete source-authoring binding decisions. Actual token structure and
// module ownership are supplied by the source parser, never executed values.
import {acceptJavaScriptSourceBindingName as admittedName} from './source-identifier-rules.js';
/** @param {string} keyword @param {string} kind @param {string} name
 * @param {boolean} callable @returns {string} */
export const readSourceLibraryDeclarationBindingKind=function classifySourceLibraryDeclaration(keyword,kind,name,callable){
 if(!admittedName(kind,name,false,[]))return '';
 return keyword==='function'?'function':keyword==='const'&&callable?'const':'';
};
/** @param {string} kind @returns {boolean} */
export function acceptJavaScriptSourceFunctionHeadName(kind){return kind==='identifier';}
/** @param {string} async @param {string} second @param {boolean} lineBreak
 * @returns {number} */
export function readSourceLibraryFunctionDeclarationOffset(async,second,lineBreak){
 return async==='function'?0:async==='async'&&second==='function'&&!lineBreak?1:-1;
}
/** @param {string} term @returns {boolean} */
export function testJavaScriptSourceFunctionExpressionTerm(term){return term==='function_expression'||term==='generator_function';}
/** The actual CST role takes precedence over a context-free word inventory.
 * @param {boolean} identifier @param {boolean} keyword @returns {boolean} */
export function testJavaScriptSourceKeywordRole(identifier,keyword){return !identifier&&keyword;}
