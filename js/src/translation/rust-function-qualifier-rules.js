/** Actual token roles select ordinary runtime translation or the independently
 * checked closed const-predicate interpretation. Spelling inside a raw name,
 * string, attribute or constant initializer never selects a function header.
 * @param {boolean} ordinary @param {boolean} predicate
 * @param {boolean} constant @param {boolean} functionKeyword @returns {boolean} */
export function acceptRustConstFunctionPrefix(ordinary,predicate,constant,functionKeyword){
 return (ordinary||predicate)&&constant&&functionKeyword;
}
/** Runtime admission supplies no const-evaluation certificate.
 * @param {boolean} constant @param {boolean} predicate @returns {boolean} */
export function requireRustConstPredicateProof(constant,predicate){return constant&&predicate;}
/** Executable entry lowering has its own contract. A qualified source main
 * remains explicit until that entry interpretation is implemented.
 * @param {number} moduleDepth @param {boolean} inherent
 * @param {string} name @returns {boolean} */
export function acceptRustConstFunctionLocation(moduleDepth,inherent,name){return moduleDepth>0||inherent||name!=='main';}
/** Associated values use the same source constant construction as module
 * constants, qualified by their actual implementing datatype.
 * @param {boolean} ordinary @param {boolean} constant
 * @param {boolean} nextFunction @returns {boolean} */
export function acceptRustAssociatedConstantItem(ordinary,constant,nextFunction){return ordinary&&constant&&!nextFunction;}
