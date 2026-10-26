// Structured source-owner transport; permitted steps come from shared rules.
import {acceptRustPointerIntegerOwnerChild} from './rust-pointer-integer-owner-rules.js';
export function readRustPointerIntegerContractOwner(owner,constraint=''){
 if(!owner)return constraint;
 for(let depth=0;owner&&depth<128;depth+=1){
  if(owner.kind==='primitive')return owner.path.join('::');
  if(!acceptRustPointerIntegerOwnerChild(owner.kind,owner.path.join('::'),owner.arguments.length))return '';
  owner=owner.arguments[0];
 }
 return '';
}
