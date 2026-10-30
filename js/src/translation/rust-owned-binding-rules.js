// Ownership states are source facts, not runtime test/completion statuses.
/** @param {boolean} collecting @param {boolean} excluded @returns {boolean} */
export function acceptRustOwnedBindingTracking(collecting,excluded){return !collecting&&!excluded;}
/** @param {string} kind @param {string} storage @param {number} layers @param {boolean} known @returns {boolean} */
export function acceptRustOwnedBindingValue(kind,storage,layers,known){return kind==='array'&&storage==='owned vector'&&layers===0&&known;}
/** @param {string} state @returns {string} */
export function readRustOwnedBindingReadError(state){return state==='available'?'':state==='moved'?'a moved source binding cannot be read or consumed again':state==='dropped'?'an owned source binding does not outlive this use':state==='possibly moved'?'a source binding moved on a returning branch cannot be reused':'a source binding needs its actual ownership state';}
/** @param {string} state @returns {string} */
export function consumeRustOwnedBindingState(state){return state==='available'?'moved':state;}
/** Only real returning paths reach a later use; absent arm-local identities
 * cannot name an outer binding. @param {string} left @param {string} right
 * @param {boolean} leftReturns @param {boolean} rightReturns @returns {string} */
export function mergeRustOwnedBindingStates(left,right,leftReturns,rightReturns){
 if(!leftReturns&&!rightReturns)return 'unreachable';
 if(!leftReturns||left==='absent')return right;
 if(!rightReturns||right==='absent')return left;
 if(left===right)return left;
 return 'possibly moved';
}
/** @param {string[]} before @param {string[]} after @returns {boolean} */
export function acceptRustOwnedBindingGuard(before,after){if(before.length!==after.length)return false;for(let index=0;index<before.length;index+=1)if(before[index]!==after[index])return false;return true;}

/** @param {string} state @returns {string} */
export function expireRustOwnedBindingState(state){return state==='available'||state==='possibly moved'?'dropped':state;}
