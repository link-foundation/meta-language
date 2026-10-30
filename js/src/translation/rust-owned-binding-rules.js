// Ownership states are source facts, not runtime test/completion statuses.
/** @param {boolean} collecting @param {boolean} excluded @returns {boolean} */
export function acceptRustOwnedBindingTracking(collecting,excluded){return !collecting&&!excluded;}
/** @param {string} kind @param {string} storage @param {number} layers @param {boolean} known @returns {boolean} */
export function acceptRustOwnedBindingValue(kind,storage,layers,known){return (kind==='array'&&storage==='owned vector'||kind==='string'&&storage==='owned string')&&layers===0&&known;}
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

/** A result must be a tracked available owner or a constructed/previously
 * transferred temporary. @param {boolean} binding @param {boolean} available
 * @param {boolean} transferred @param {boolean} fresh @returns {boolean} */
export function acceptRustOwnedResultTransfer(binding,available,transferred,fresh){return binding?available:transferred||fresh;}

/** Legacy boundary when no explicit loan-union provenance is supplied.
 * @param {boolean} tracked @param {boolean} borrowed @param {boolean} named
 * @param {boolean} same @returns {string} */
export function readRustOwnedBorrowJoinError(tracked,borrowed,named,same){return tracked&&borrowed&&named&&!same?'alternative borrowed owners need their complete loan union proof':'';}

/** Canonical loan identities are actual source bindings, never type names.
 * @param {string[]} left @param {string[]} right @returns {string[]} */
export function mergeRustOwnedLoanBindings(left,right){
 let result=left.slice(0,0);
 for(const identity of left)if(!result.includes(identity))result=[...result,identity];
 for(const identity of right)if(!result.includes(identity))result=[...result,identity];
 return result;
}
/** @param {string} identity @param {string[]} loans @returns {string[]} */
export function projectRustOwnedLoanBindings(identity,loans){return mergeRustOwnedLoanBindings(identity===''?[]:[identity],loans);}
/** Error precedence is independent of host map/parameter enumeration order.
 * @param {string[]} states @returns {string} */
export function readRustOwnedLoanReadError(states){
 if(states.includes('dropped'))return readRustOwnedBindingReadError('dropped');
 if(states.includes('moved'))return readRustOwnedBindingReadError('moved');
 if(states.includes('possibly moved'))return readRustOwnedBindingReadError('possibly moved');
 for(const state of states)if(state!=='available')return readRustOwnedBindingReadError('');
 return '';
}

/** The represented standard String From cases preserve actual source storage.
 * @param {string} kind @param {string} storage @param {number} layers
 * @param {boolean} known @param {boolean} references @returns {string} */
export function readRustOwnedStringConstructionError(kind,storage,layers,known,references){return known&&kind==='string'&&(storage==='owned string'&&layers===0||(storage==='owned string'||storage==='string slice')&&layers===1&&references)?'':'standard String construction needs an actual owned String or one immutable String/str reference';}

/** Container elements move before the next source macro argument is evaluated.
 * @param {string} macro @returns {boolean} */
export function acceptRustOwnedMacroArgumentTransfer(macro){return macro==='vec';}

/** str itself is unsized; cloning its single shared reference keeps that loan.
 * @param {string} method @param {string} kind @param {string} storage
 * @param {number} layers @returns {boolean} */
export function acceptRustStringSliceCloneReference(method,kind,storage,layers){return method==='clone'&&kind==='string'&&storage==='string slice'&&layers===1;}
