// Additive supplied-module class profile. The original function-only policy
// remains its own export; actual CST/binding owners are host-supplied facts.
import {readSourceLibraryDeclarationBindingKind} from './source-library-binding-rules.js';
import {acceptJavaScriptSourceBindingName} from './source-identifier-rules.js';
import {acceptJavaScriptReservedIdentifier} from './javascript-identifier-rules.js';
import {readRequiredNumberFact as number,readRequiredTextFact as text} from './fact-column-rules.js';
/** @param {string} profile @param {boolean} dependencies @param {boolean} aliases @param {boolean} shared @returns {string} */
export function readSourceLibraryClassProfileError(profile,dependencies,aliases,shared){if(profile==='')return '';return profile==='closed local class declarations 1'&&dependencies&&aliases&&shared?'':'closed class composition needs its exact supplied dependency, resolved alias and shared reference profiles';}
/** @param {string} profile @param {boolean} dependencies @param {boolean} aliases @returns {string} */
export function readSourceLibraryReExportProfileError(profile,dependencies,aliases){return profile===''||profile==='resolved named module providers 1'&&dependencies&&aliases?'':'named re-export composition needs actual supplied dependencies and resolved aliases';}
/** Only actual named provider identifiers are selected in this first profile.
 * Default/namespace forms retain their separate obligations.
 * @param {string} kind @param {string} name @returns {boolean} */
export function acceptSourceLibraryReExportName(kind,name){return acceptJavaScriptSourceBindingName(kind,name,false,[])&&!acceptJavaScriptReservedIdentifier(name)&&name!=='eval'&&name!=='arguments';}
/** A public native entry name cannot silently become a private provider name.
 * Alias wrappers require their own actual callable/constructor transport.
 * @param {string} exported @param {string} compiled @returns {boolean} */
export function acceptSourceLibraryEntryExportOwner(exported,compiled){return exported!==''&&exported===compiled;}
/** @param {string} keyword @param {string} kind @param {string} name @param {boolean} callable @returns {string} */
export function readSourceLibraryClassBindingKind(keyword,kind,name,callable){return keyword==='class'&&acceptJavaScriptSourceBindingName(kind,name,false,[])&&!acceptJavaScriptReservedIdentifier(name)&&name!=='eval'&&name!=='arguments'?'class':readSourceLibraryDeclarationBindingKind(keyword,kind,name,callable);}
/** @param {string} keyword @param {string} kind @param {string} name @param {boolean} callable @returns {boolean} */
export function acceptSourceLibraryClassExport(keyword,kind,name,callable){return readSourceLibraryClassBindingKind(keyword,kind,name,callable)!=='';}
/** @param {string} term @param {string} kind @returns {boolean} */
export function acceptSourceLibraryClassDeclarationTerm(term,kind){return kind==='class'?term==='class_declaration':kind==='function'?term==='function_declaration'||term==='generator_function_declaration':kind==='const'&&term==='lexical_declaration';}
/** @param {string[]} names @param {number[]} starts @param {number[]} ends @param {string[]} witnessed @param {number} first @param {number} last @returns {string} */
export function readSourceLibraryClassDeclarationOwner(names,starts,ends,witnessed,first,last){let owner='';if(names.length!==starts.length||names.length!==ends.length||first<0||last<first)return '';for(let at=0;at<names.length;at+=1)if(first>=number(starts,at)&&last<=number(ends,at)&&witnessed.includes(text(names,at))){if(owner!=='')return '#ambiguous';owner=text(names,at);}return owner;}
/** @param {string[]} importedOwners @param {string} constructor @param {string[]} helpers @param {string} helper @param {string} receiverOwner @returns {boolean} */
export function acceptSourceLibraryClassHelperCall(importedOwners,constructor,helpers,helper,receiverOwner){return constructor!==''&&constructor===receiverOwner&&importedOwners.includes(constructor)&&helpers.includes(helper);}
/** Static dispatch has an actual source constructor/table owner, without an
 * instance receiver. It cannot borrow a fabricated receiver proof.
 * @param {string[]} importedOwners @param {string} constructor
 * @param {string[]} helpers @param {string} helper @returns {boolean} */
export function acceptSourceLibraryClassStaticCall(importedOwners,constructor,helpers,helper){return constructor!==''&&importedOwners.includes(constructor)&&helpers.includes(helper);}
/** Reverse one actual edit phase. Unchanged coordinates remain exact; a
 * replacement retains the complete original occurrence, never an invented
 * point. Deletions at a touched boundary conservatively retain their input.
 * @param {number[]} starts @param {number[]} ends @param {number[]} lengths
 * @param {number} first @param {number} last @returns {number[]} */
export function projectSourceLibraryClassEditInterval(starts,ends,lengths,first,last){
 if(starts.length!==ends.length||starts.length!==lengths.length||starts.length>32768||!Number.isSafeInteger(first)||!Number.isSafeInteger(last)||first<0||last<first)return [];
 let delta=0,previous=0,low=-1,high=-1;
 for(let at=0;at<starts.length;at+=1){const start=number(starts,at),end=number(ends,at),length=number(lengths,at);if(!Number.isSafeInteger(start)||!Number.isSafeInteger(end)||!Number.isSafeInteger(length)||start<previous||end<start||length<0)return [];
  const changedStart=start+delta,changedEnd=changedStart+length;
  if(low<0&&first<changedStart)low=first-delta;if(high<0&&last<changedStart)high=last-delta;
  if(first>=changedStart&&first<=changedEnd&&low<0)low=start;
  if(last>=changedStart&&last<=changedEnd&&high<0)high=end;
  delta+=length-(end-start);previous=end;
 }
 if(low<0)low=first-delta;if(high<0)high=last-delta;return low<0||high<low?[]:[low,high];
}
