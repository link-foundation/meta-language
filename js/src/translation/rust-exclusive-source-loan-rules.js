// Decisions over actual source inventories; these do not grant parser authority.
export const rustExclusiveSourceLoanProfile='Rust exclusive source loans 1';
export function readRustExclusiveSourceLoanVersionError(major,minor){return major===1&&Number.isInteger(minor)&&minor>=90&&minor<100?'':'exclusive source loans cover Rust >=1.90.0 <1.100.0';}
export function readRustExclusiveSourceLoanBudgetError(bindings,loans,depth,work){return [bindings,loans,depth,work].every(value=>Number.isSafeInteger(value)&&value>=0)&&bindings<=4096&&loans<=4096&&depth<=256&&work<=100000?'':'exclusive source loan work exceeds its finite source profile';}
export function testRustExclusiveSourcePlacePrefix(left,right){return left.length<=right.length&&left.every((value,index)=>value===right[index]);}
export function testRustExclusiveSourcePlaceOverlap(leftRoot,left,rightRoot,right){return leftRoot===rightRoot&&(testRustExclusiveSourcePlacePrefix(left,right)||testRustExclusiveSourcePlacePrefix(right,left));}
export function readRustExclusiveSourceLoanConflict(left,right){return left==='exclusive'||right==='exclusive';}
export function readRustExclusiveSourceStorageRole(kind,owner,path,arguments_,wrappers){
 if(wrappers!==0)return '';
 if(owner==='unit'&&kind==='unit'&&path===''&&arguments_===0&&wrappers===0)return 'leaf';
 if(owner==='primitive'&&arguments_===0&&['unit','bool','fixed','int','nat','float','string'].includes(kind))return 'leaf';
 if(kind==='string'&&owner==='library'&&path==='std::string::String'&&arguments_===0)return 'leaf';
 if(kind==='array'&&owner==='library'&&path==='std::vec::Vec'&&arguments_===1)return 'vector';
 return kind==='named'&&owner==='declaration'&&arguments_===0?'record':'';
}
export function planRustExclusiveSourceCallPacket(modes){if(modes.length>256||modes.some(mode=>!['value','shared','exclusive'].includes(mode)))return [];let index=0;return modes.map(mode=>mode==='exclusive'?++index:0);}
