// Complete source-authoring binding decisions. Actual token structure and
// module ownership are supplied by the source parser, never executed values.
import {acceptJavaScriptSourceBindingName as admittedName} from './source-identifier-rules.js';
/** @param {string} keyword @param {string} kind @param {string} name
 * @param {boolean} callable @returns {string} */
export const readSourceLibraryDeclarationBindingKind=(keyword,kind,name,callable)=>{
 if(!admittedName(kind,name,false,[]))return '';
 return keyword==='function'?'function':keyword==='const'&&callable?'const':'';
};
