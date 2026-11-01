// Consuming operations reuse the actual source ownership state machine. Roles
// and transfer identities are projections of admitted origins and transitions.
/** @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumMethodArity(owner,method){
 if(owner==='std::option::Option')return method==='unwrap_or'||method==='ok_or'?1:-1;
 return owner==='std::result::Result'?method==='unwrap_or'?1:method==='ok'||method==='err'?0:-1:-1;
}
/** Option ok_or can first introduce ownership in its eager error argument.
 * @param {string} owner @param {string} method @param {string} role @returns {string} */
export function readRustOwnedStandardSumMethodReceiverMode(owner,method,role){
 if(readRustOwnedStandardSumMethodArity(owner,method)<0)return '';
 return role==='owned'?'owned':owner==='std::option::Option'&&method==='ok_or'&&role==='copy'?'copy':'';
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
 return method==='unwrap_or'?'payload':method==='ok_or'?'std::result::Result':'std::option::Option';
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
  if(method==='ok_or')return variant==='Some'?['Ok','payload']:variant==='None'?['Err','argument']:[];
 }
 if(owner==='std::result::Result'){
  if(method==='unwrap_or')return variant==='Ok'?['','payload']:variant==='Err'?['','argument']:[];
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
