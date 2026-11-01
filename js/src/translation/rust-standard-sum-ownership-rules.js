import {readRustStandardSumSourceArgumentCount} from './rust-standard-sum-source-rules.js';

/** @param {number} minor @returns {string} */
export function readRustStandardSumOwnershipVersionError(minor){
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'owned standard sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
}

/** Roles are projections of actual complete source origins, not certificates.
 * @param {string} owner @param {string[]} roles @returns {string} */
export function readRustStandardSumOwnershipProfile(owner,roles){
 const count=readRustStandardSumSourceArgumentCount(owner);if(count<0||roles.length!==count)return '';
 let owned=false;for(const role of roles){if(role!=='copy'&&role!=='owned')return '';if(role==='owned')owned=true;}
 return owned?'owned':'copy';
}
/** The old Copy-only gate stays independent and unchanged.
 * @param {string} owner @param {string[]} identities @param {string[]} roles
 * @returns {boolean} */
export function acceptRustStandardSumOwnedArguments(owner,identities,roles){
 if(identities.length!==roles.length||readRustStandardSumOwnershipProfile(owner,roles)!=='owned')return false;
 for(const identity of identities)if(identity==='')return false;return true;
}
/** @param {boolean} owner @param {number} fields @param {number} arguments_
 * @returns {string} */
export function readRustStandardSumOwnedConstructorError(owner,fields,arguments_){
 return owner&&(fields===0||fields===1)&&arguments_===fields?'':'an owned sum construction needs its actual constructor owner and exact source field arguments';
}
/** @param {boolean} owned @param {boolean} transferred @param {boolean} available
 * @param {boolean} producer @returns {string} */
export function readRustStandardSumOwnedFieldProductionError(owned,transferred,available,producer){return !owned||transferred||available||producer?'':'an owned constructor field needs its actual available binding or source producer contract';}
/** @param {boolean} owned @param {boolean} borrowed @param {boolean} field
 * @returns {string} */
export function readRustStandardSumOwnedFieldDisposition(owned,borrowed,field){return !owned?'plain':borrowed?'borrow':field?'move':'plain';}
/** Guard bindings need their original delayed move/fallthrough interpretation.
 * @param {boolean} owned @param {boolean} guarded @returns {string} */
export function readRustStandardSumOwnedGuardError(owned,guarded){return owned&&guarded?'owned sum guards need their delayed payload-move and fallthrough model':'';}
/** Only a recorded source call to a real declared function is a producer.
 * @param {boolean} owned @param {boolean} function_ @param {boolean} call
 * @returns {boolean} */
export function acceptRustStandardSumOwnedCallResult(owned,function_,call){return owned&&function_&&call;}
/** Applied only when an owned pattern binding actually transfers, not `_`.
 * @param {boolean} required @param {boolean} binding @param {boolean} producer
 * @returns {string} */
export function readRustStandardSumOwnedPatternTransferError(required,binding,producer){return required&&!binding&&!producer?'an unbound owned pattern field needs its actual fresh parent producer contract':'';}
/** @returns {number} */
export function readRustStandardSumOwnershipNodeLimit(){return 100000;}
/** @returns {number} */
export function readRustStandardSumOwnershipDepthLimit(){return 256;}
