/** A source constant is read as its value, independently of expected Fn types.
 * The existing checked zero-input declaration supplies that value exactly once.
 * @param {string} kind @param {string} owner @param {string} temporary
 * @returns {string[]} */
export function planRustSourceConstantValue(kind,owner,temporary){return kind==='const'?['let',temporary,'name',owner,'name',temporary]:[];}
/** @param {string} kind @param {string} role @returns {string} */
export function readRustSourceCallableValueError(kind,role){return kind==='function'?'':'the source '+role+' is not callable';}
/** Constants in patterns cannot silently become variable bindings.
 * @param {string} declaration @returns {string} */
export function readRustSourceConstantPatternError(declaration){return declaration==='const'?'source constant patterns require their represented value and type':'';}
/** Unqualified module values belong to that module; parent items need a path.
 * @param {string[]} modulePath @param {string[]} resolved @returns {string[]} */
export function readRustSourceValuePath(modulePath,resolved){return resolved.length===1?['crate',...modulePath,...resolved]:resolved;}
/** Source calls remain distinct from function references at zero arguments.
 * @param {boolean} ordinary @param {number} count @returns {string} */
export function readRustSourceCallForm(ordinary,count){return ordinary||count!==0?'app':'name';}
