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
/** @param {string} term @returns {string} */
export function readJavaScriptSourceFunctionScopeForm(term){
 return testJavaScriptSourceFunctionExpressionTerm(term)?'function expression':term==='function_declaration'||term==='generator_function_declaration'?'function declaration':term==='arrow_function'?'arrow function':term==='method_definition'?'method definition':'';
}
/** @param {string[]} terms @returns {boolean} */
export function acceptJavaScriptSourceSimpleParameterList(terms){
 for(const term of terms)if(['assignment_pattern','object_assignment_pattern','rest_pattern','object_pattern','array_pattern'].includes(term))return false;
 return true;
}
/** @param {string} first @param {string} second @param {boolean} bodyParent
 * @returns {boolean} */
export function testJavaScriptSourceParameterVariableDependency(first,second,bodyParent){return bodyParent&&(first==='parameter'&&second==='var'||first==='var'&&second==='parameter');}
/** @param {string} kind @param {boolean} existing @returns {boolean} */
export function testJavaScriptSourceVariableRedeclaration(kind,existing){return kind==='var'&&existing;}
