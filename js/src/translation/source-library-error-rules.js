// Explicit semantic Error module closure; ordinary record-class and typed-core
// profiles retain their old admission. All owner facts come from actual CST.
/** @param {string} profile @param {boolean} dependencies @param {boolean} aliases @param {string} ordinaryClasses @returns {string} */
export function readSourceLibraryErrorProfileError(profile,dependencies,aliases,ordinaryClasses){return profile===''?'':profile==='resolved supplied Error source modules 1'&&dependencies&&aliases&&ordinaryClasses===''?'':'Error source linkage requires its exact supplied modules, resolved aliases and separate semantic class profile';}
/** @param {string} sourceName @param {string} compiledName @param {number} count @returns {boolean} */
export function acceptSourceLibraryErrorDeclarationOwner(sourceName,compiledName,count){return sourceName!==''&&compiledName!==''&&count===1;}
/** @param {string[]} exports @param {string} entry @param {number} count @returns {boolean} */
export function acceptSourceLibraryErrorEntryOwner(exports,entry,count){return entry!==''&&exports.includes(entry)&&count===1;}
