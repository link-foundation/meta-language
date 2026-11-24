import {readRustStandardSumMethodBranch} from './rust-standard-sum-method-rules.js';
/** The actual standard methods borrow &self; they never consume their payload.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumObserverArity(owner,method){return owner==='std::option::Option'&&(method==='is_some'||method==='is_none')||owner==='std::result::Result'&&(method==='is_ok'||method==='is_err')?0:-1;}
/** Direct Copy sums keep the existing dispatcher. Referenced Copy sums can
 * carry a transitive loan from an owned enclosing constructor.
 * @param {string} profile @param {number} layers @returns {boolean} */
export function acceptRustOwnedStandardSumObserverProfile(profile,layers){return profile==='owned'||profile==='copy'&&Number.isInteger(layers)&&layers>0;}
/** Facts come from the original expression, including its root/loan identities.
 * A checked reference parameter can have elided lifetimes and no local owner;
 * local references retain their existing root/loan state checks separately.
 * @param {boolean} proven @param {number} layers @param {boolean} references
 * @param {number} lifetimes @param {boolean} tracked @param {boolean} available
 * @param {boolean} produced @param {boolean} borrowed @param {boolean} loans
 * @returns {string} */
export function readRustOwnedStandardSumObserverSourceError(proven,layers,references,lifetimes,tracked,available,produced,borrowed,loans){
 if(!proven||!tracked||!Number.isInteger(layers)||layers<0)return 'an owned sum observer needs its actual checked ownership and source origin';
 if(layers>0)return references&&lifetimes===layers?'':'an owned sum observer needs retained immutable reference and lifetime provenance';
 if(borrowed)return loans||produced?'':'an owned sum observer cannot inspect an unproven borrowed source place';
 return available||produced?'':'an owned sum observer needs an available source root or an actual owned producer';
}
/** @param {string} owner @param {string} variant @param {string} method
 * @returns {string} */
export function readRustOwnedStandardSumObserverBranch(owner,variant,method){return readRustOwnedStandardSumObserverArity(owner,method)<0?'':readRustStandardSumMethodBranch(owner,variant,method);}
/** @param {string} owner @param {string} method @param {number} minor
 * @returns {string} */
export function readRustOwnedStandardSumObserverVersionError(owner,method,minor){return readRustOwnedStandardSumObserverArity(owner,method)<0||Number.isInteger(minor)&&minor>=90&&minor<100?'':'owned standard sum observer source semantics are uncovered outside Rust >=1.90.0 <1.100.0';}
