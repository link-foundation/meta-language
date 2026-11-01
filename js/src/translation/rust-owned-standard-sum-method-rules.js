// Consuming operations reuse the actual source ownership state machine. Roles
// and transfer identities are projections of admitted origins and transitions.
import {readRustPrimitiveDefaultTerm} from './rust-primitive-default-rules.js';
/** @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumMethodArity(owner,method){
 if(owner==='std::option::Option')return method==='unwrap_or'||method==='ok_or'?1:method==='unwrap_or_default'?0:-1;
 return owner==='std::result::Result'?method==='unwrap_or'?1:method==='ok'||method==='err'||method==='unwrap_or_default'?0:-1:-1;
}
/** Option ok_or can first introduce ownership in its eager error argument.
 * @param {string} owner @param {string} method @param {string} role @returns {string} */
export function readRustOwnedStandardSumMethodReceiverMode(owner,method,role){
 if(readRustOwnedStandardSumMethodArity(owner,method)<0)return '';
 return role==='owned'?'owned':role==='copy'&&(method==='unwrap_or_default'||owner==='std::option::Option'&&method==='ok_or')?'copy':'';
}
/** A real owning function result contract supplies production, independently
 * of Copy evidence or any temporary/type marker.
 * @param {string} role @param {boolean} function_ @param {boolean} recorded
 * @param {boolean} identity @param {number} wrappers @param {boolean} borrowed @returns {boolean} */
export function acceptRustOwnedStandardSumMethodCallProduction(role,function_,recorded,identity,wrappers,borrowed){return role==='owned'&&function_&&recorded&&identity&&wrappers===0&&!borrowed;}
/** @param {string} owner @param {string} method @param {number} minor @returns {string} */
export function readRustOwnedStandardSumMethodVersionError(owner,method,minor){
 if(readRustOwnedStandardSumMethodArity(owner,method)<0)return 'the consuming owned standard sum method is outside this profile';
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'the consuming owned standard sum method source version is uncovered';
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustOwnedStandardSumMethodResult(owner,method){
 if(readRustOwnedStandardSumMethodArity(owner,method)<0)return '';
 return method==='unwrap_or'||method==='unwrap_or_default'?'payload':method==='ok_or'?'std::result::Result':'std::option::Option';
}
/** A -1 argument comes from the actual eager default, never a guessed type.
 * @param {string} owner @param {string} method @returns {number[]} */
export function planRustOwnedStandardSumMethodArguments(owner,method){
 if(readRustOwnedStandardSumMethodArity(owner,method)<0)return [];
 return method==='ok_or'?[0,-1]:method==='ok'?[0]:method==='err'?[1]:[];
}
/** @param {string} owner @param {string} variant @param {string} method @returns {string[]} */
export function planRustOwnedStandardSumMethodBranch(owner,variant,method){
 if(owner==='std::option::Option'){
  if(method==='unwrap_or')return variant==='Some'?['','payload']:variant==='None'?['','argument']:[];
  if(method==='unwrap_or_default')return variant==='Some'?['','payload']:variant==='None'?['','standard default']:[];
  if(method==='ok_or')return variant==='Some'?['Ok','payload']:variant==='None'?['Err','argument']:[];
 }
 if(owner==='std::result::Result'){
  if(method==='unwrap_or')return variant==='Ok'?['','payload']:variant==='Err'?['','argument']:[];
  if(method==='unwrap_or_default')return variant==='Ok'?['','payload']:variant==='Err'?['','standard default']:[];
  if(method==='ok')return variant==='Ok'?['Some','payload']:variant==='Err'?['None','empty']:[];
  if(method==='err')return variant==='Err'?['Some','payload']:variant==='Ok'?['None','empty']:[];
 }
 return [];
}
/** Only a completed move into a distinct tracked temporary grants production.
 * @param {string} role @param {number} wrappers @param {boolean} borrowed
 * @param {boolean} tracking @param {string} before @param {string} after
 * @param {boolean} available @param {boolean} produced @returns {string} */
export function readRustOwnedStandardSumMethodCaptureError(role,wrappers,borrowed,tracking,before,after,available,produced){
 return role==='owned'&&wrappers===0&&!borrowed&&tracking&&after!==''&&after!==before&&available&&produced?'':'the consuming value needs its actual move into a fresh tracked owned temporary';
}
/** @param {string} role @returns {boolean} */
export function acceptRustOwnedStandardSumMethodArgument(role){return role==='copy'||role==='owned';}
/** @param {string} role @param {boolean} receiver @param {boolean} argument @returns {boolean} */
export function acceptRustOwnedStandardSumMethodProduction(role,receiver,argument){return role==='owned'&&receiver&&argument;}
/** Standard defaults need actual type ownership, never a spelling or a guessed
 * user trait implementation. Option's empty constructor needs no T default.
 * @param {string} kind @param {string} ownerKind @param {string} owner
 * @param {string} storage @param {number} wrappers @param {boolean} proven @returns {string} */
export function readRustOwnedStandardSumDefaultTerm(kind,ownerKind,owner,storage,wrappers,proven){
 if(!proven||wrappers!==0)return '';
 if(kind==='string'&&ownerKind==='library'&&storage==='owned string'&&['alloc::string::String','std::string::String'].includes(owner))return 'empty owned string';
 if(kind==='array'&&ownerKind==='library'&&storage==='owned vector'&&['alloc::vec::Vec','std::vec::Vec'].includes(owner))return 'empty owned vector';
 if(kind==='named'&&owner==='std::option::Option')return 'empty actual Option';
 if(kind==='unit'&&ownerKind==='unit')return 'unit';
 const primitive=ownerKind==='primitive'?readRustPrimitiveDefaultTerm(owner):'';
 return kind==='bool'&&primitive==='false'||kind==='float'&&primitive==='positive floating zero'||kind==='fixed'&&primitive==='integer zero'?primitive:'';
}
/** Supported ordinary sum fields already have their actual standard Clone
 * implementations. A live root or actual producer supplies the owned value;
 * the complete type alone cannot supply it. Shared/mutable handle profiles are
 * outside this ordinary value interpretation.
 * @param {string} owner @param {string} role @param {number} wrappers
 * @param {boolean} borrowed @param {boolean} tracking @param {boolean} available
 * @param {boolean} produced @returns {string} */
export function readRustStandardSumCloneSourceError(owner,role,wrappers,borrowed,tracking,available,produced){
 if(!['std::option::Option','std::result::Result'].includes(owner)||wrappers!==0||borrowed)return 'standard sum clone needs its actual direct ordinary value source';
 return role==='copy'||role==='owned'&&tracking&&(available||produced)?'':'standard sum clone needs an available actual owner or an actual fresh producer, independently of its type';
}
