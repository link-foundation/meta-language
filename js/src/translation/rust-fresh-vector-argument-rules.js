// Fresh source constructors, never evidence for moving an existing local.
/** @param {string} kind @param {boolean} diverging @param {string} model
 * @param {number} wrappers @param {boolean} constructor @returns {string} */
export function readRustFreshVectorArgumentForm(kind,diverging,model,wrappers,constructor){
 if(diverging)return 'diverging';
 if(kind==='array'&&model==='owned vector'&&wrappers===0&&constructor)return 'constructor';
 if(kind==='let'||kind==='print')return 'body';
 if(kind==='if')return 'alternatives';
 if(kind==='match')return 'rows';
 return '';
}
/** Actual parameter typing remains the existing checker's responsibility.
 * @param {boolean} ordinary @param {string} model @param {number} wrappers
 * @param {string} element @param {boolean} fresh @returns {boolean} */
export function acceptRustFreshOwnedVectorArgument(ordinary,model,wrappers,element,fresh){
 return ordinary&&model==='owned vector'&&wrappers===0&&fresh
  &&(element==='bool'||element==='float'||element==='fixed'||element==='unit');
}
