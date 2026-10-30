// Bounded original source-owner tree transport. Only shared registered
// container edges can be crossed; actual terminal owner records stay intact.
import {acceptRustPointerIntegerOwnerChild} from './rust-pointer-integer-owner-rules.js';
export function projectRustArrayLookupElementOwner(owner){
 for(let depth=0;owner&&depth<128;depth+=1){if(!acceptRustPointerIntegerOwnerChild(owner.kind,owner.path.join('::'),owner.arguments.length))return owner;owner=owner.arguments[0];}
 return null;
}
