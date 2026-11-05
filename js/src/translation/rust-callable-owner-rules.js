import {readRustPointerIntegerOwnerError} from './rust-pointer-integer-owner-rules.js';
/** @param {string} name @returns {number} */
export function readRustCallableOwnerProjectionLimit(name){return name==='visits'?65536:name==='depth'?256:name==='cells'?32768:0;}
/** @returns {string} */
export function readRustCallableOwnerProjectionFailure(){return 'invalid callable source projection';}
/** Descriptor rows retain structural paths and actual source owner atoms.
 * @param {string[]} actual @param {string[]} expected @returns {string} */
export function readRustCallableOwnerError(actual,expected){
 if(actual.length%2!==0||expected.length%2!==0||actual.length>32768||expected.length>32768||actual.includes(readRustCallableOwnerProjectionFailure())||expected.includes(readRustCallableOwnerProjectionFailure()))return 'the complete callable source contract exceeds its source budget';
 if(expected.length===0)return '';
 if(actual.length!==expected.length)return 'the actual callable needs its complete structural source signature';
 for(let index=0;index<expected.length;index+=2){const path=expected[index]??'',owner=expected[index+1]??'';if(actual[index]!==path)return 'the actual callable needs its complete structural source signature';const other=actual[index+1]??'',error=readRustPointerIntegerOwnerError(other,owner);if(error)return error;if(owner!==''&&other==='')return 'the actual callable needs complete parameter and result source owners';}
 return '';
}
/** @param {string[]} left @param {string[]} right @returns {boolean} */
export function acceptRustCallableOwnerDescriptorsEqual(left,right){if(left.length!==right.length||left.length%2!==0||left.length>32768||left.includes(readRustCallableOwnerProjectionFailure())||right.includes(readRustCallableOwnerProjectionFailure()))return false;for(let at=0;at<left.length;at+=1)if(left[at]!==right[at])return false;return true;}
/** @param {string[]} descriptors @returns {string} */
export function readRustCallableOwnerJoinError(descriptors){
 if(descriptors.length%2!==0||descriptors.length>32768||descriptors.includes(readRustCallableOwnerProjectionFailure()))return 'the complete callable source contract exceeds its source budget';
 for(let index=0;index<descriptors.length;index+=2){const owner=descriptors[index+1]??'';if(!['usize','u64','isize','i64'].includes(owner))continue;for(let before=0;before<index;before+=2)if(descriptors[index]===descriptors[before]){const error=readRustPointerIntegerOwnerError(owner,descriptors[before+1]??'');if(error)return error;}}
 return '';
}
