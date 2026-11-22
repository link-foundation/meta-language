// Decisions over actual source inventories; these do not grant parser authority.
/** @returns {string} */
export function readRustExclusiveSourceLoanProfile(){return 'Rust exclusive source loans 1';}
/** @param {number} major @param {number} minor @returns {string} */
export function readRustExclusiveSourceLoanVersionError(major,minor){return major===1&&Number.isInteger(minor)&&minor>=90&&minor<100?'':'exclusive source loans cover Rust >=1.90.0 <1.100.0';}
/** @param {number} bindings @param {number} loans @param {number} depth @param {number} work @returns {string} */
export function readRustExclusiveSourceLoanBudgetError(bindings,loans,depth,work){return [bindings,loans,depth,work].every(value=>Number.isSafeInteger(value)&&value>=0)&&bindings<=4096&&loans<=4096&&depth<=256&&work<=100000?'':'exclusive source loan work exceeds its finite source profile';}
/** @param {string[]} left @param {string[]} right @returns {boolean} */
export function testRustExclusiveSourcePlacePrefix(left,right){return left.length<=right.length&&left.every((value,index)=>value===right[index]);}
/** @param {string} leftRoot @param {string[]} left @param {string} rightRoot @param {string[]} right @returns {boolean} */
export function testRustExclusiveSourcePlaceOverlap(leftRoot,left,rightRoot,right){return leftRoot===rightRoot&&(testRustExclusiveSourcePlacePrefix(left,right)||testRustExclusiveSourcePlacePrefix(right,left));}
/** @param {string} left @param {string} right @returns {boolean} */
export function readRustExclusiveSourceLoanConflict(left,right){return left==='exclusive'||right==='exclusive';}
/** @param {string} kind @param {string} owner @param {string} path @param {number} arguments_ @param {number} wrappers @returns {string} */
export function readRustExclusiveSourceStorageRole(kind,owner,path,arguments_,wrappers){
 if(wrappers!==0)return '';
 if(owner==='unit'&&kind==='unit'&&path===''&&arguments_===0&&wrappers===0)return 'leaf';
 if(owner==='primitive'&&arguments_===0&&['unit','bool','fixed','int','nat','float','string'].includes(kind))return 'leaf';
 if(kind==='string'&&owner==='library'&&path==='std::string::String'&&arguments_===0)return 'leaf';
 if(kind==='array'&&owner==='library'&&path==='std::vec::Vec'&&arguments_===1)return 'vector';
 return kind==='named'&&owner==='declaration'&&arguments_===0?'record':'';
}
/** @param {string[]} modes @returns {number[]} */
export function planRustExclusiveSourceCallPacket(modes){
 if(modes.length>256||modes.some(mode=>!['value','shared','exclusive'].includes(mode)))return [];
 const positions=[];
 let next=0;
 for(const mode of modes){
  if(mode==='exclusive'){next+=1;positions.push(next);}
  else positions.push(0);
 }
 return modes.map((mode,index)=>positions[index]??0);
}
