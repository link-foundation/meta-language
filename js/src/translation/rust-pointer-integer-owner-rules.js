/** Equal representation does not supply a Rust source type coercion. Other
 * domains stay with existing type/ownership checks and their diagnostics.
 * @param {string} actual @param {string} expected @returns {string} */
export function readRustPointerIntegerOwnerError(actual,expected){
 return actual==='usize'&&expected==='u64'||actual==='u64'&&expected==='usize'||actual==='isize'&&expected==='i64'||actual==='i64'&&expected==='isize'
  ?'distinct pointer-sized and fixed-width integer source types need an explicit cast':'';
}
/** @param {string[]} owners @returns {string} */
export function readRustPointerIntegerOwnerJoinError(owners){
 let unsigned='',signed='';
 for(const owner of owners){
  if(owner==='usize'||owner==='u64'){const error=readRustPointerIntegerOwnerError(owner,unsigned);if(error)return error;unsigned=owner;}
  if(owner==='isize'||owner==='i64'){const error=readRustPointerIntegerOwnerError(owner,signed);if(error)return error;signed=owner;}
 }
 return '';
}
/** Context-free arithmetic retains an actual agreed pointer-width source
 * atom. The existing checker still checks every other operand and width.
 * @param {string[]} owners @returns {string} */
export function selectRustPointerIntegerResultOwner(owners){
 if(readRustPointerIntegerOwnerJoinError(owners))return '';
 let selected='';
 for(const owner of owners)if(owner==='usize'||owner==='u64'||owner==='isize'||owner==='i64'){if(selected!==''&&selected!==owner)return '';selected=owner;}
 return selected;
}
/** Source owner projection crosses only actual represented standard containers.
 * @param {string} kind @param {string} path @param {number} arguments_ @returns {boolean} */
export function acceptRustPointerIntegerOwnerChild(kind,path,arguments_){return arguments_===1&&(kind==='slice'||kind==='library'&&(path==='std::vec::Vec'||path==='std::boxed::Box'));}
