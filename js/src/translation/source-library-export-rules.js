// Import aliases retain their actual constant-function provider identity.
import {readSourceLibraryDeclarationBindingKind as bindingKind} from './source-library-binding-rules.js';
/** @param {string} keyword @param {string} kind @param {string} name
 * @param {boolean} callable @returns {boolean} */
export function acceptSourceLibraryFunctionExport(keyword,kind,name,callable){
 return bindingKind(keyword,kind,name,callable)!=='';
}
