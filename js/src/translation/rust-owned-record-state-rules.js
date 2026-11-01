import {readRustOwnedRecordFieldIdentity,testRustOwnedRecordFieldDescendant} from './rust-owned-record-update-rules.js';
import {readRustOwnedRecordPlaceTokenCount} from './rust-owned-record-place-rules.js';
/** @param {string[]} forms @returns {number} */
export function readRustOwnedRecordStateAssignmentTokenCount(forms){const count=readRustOwnedRecordPlaceTokenCount(forms.map(form=>form==='='?'}':form));return count>=0&&forms[count]==='='&&forms.slice(0,count).includes('.')?count:-1;}
/** @param {string} kind @param {number} layers @param {string} role
 * @returns {string} */
export function readRustOwnedRecordStateStorageDisposition(kind,layers,role){if(layers!==0||role!=='copy'&&role!=='owned'||kind==='function')return 'refuse';if(kind==='named'||kind==='array'||kind==='tuple')return 'children';return ['string','unit','bool','fixed','int','nat','float'].includes(kind)?'leaf':'refuse';}
/** Box can erase its payload's core type, so source owner is checked too.
 * @param {string} kind @param {string} path @returns {boolean} */
export function acceptRustOwnedRecordStateStorageOwner(kind,path){return kind!=='library'||path!=='std::boxed::Box'&&path!=='alloc::boxed::Box';}
/** @param {number} nodes @param {number} depth @returns {string} */
export function readRustOwnedRecordStateStorageError(nodes,depth){return Number.isInteger(nodes)&&nodes>=0&&nodes<=65536&&Number.isInteger(depth)&&depth>=0&&depth<=256?'':'the complete record state storage exceeds its finite source budget';}
/** @param {boolean} ordinary @param {boolean} written @param {string} profile
 * @param {boolean} binding @param {boolean} lexical @param {number} layers
 * @returns {string} */
export function readRustOwnedRecordMutableBindingError(ordinary,written,profile,binding,lexical,layers){return ordinary&&written&&profile==='owned'&&binding&&lexical&&layers===0?'':'a mutable record needs its actual mutable declaration and complete destructor-free owned lexical root';}
/** @param {boolean} mutable @param {boolean} callable @param {boolean} scope
 * @param {string} root @param {string} field @param {boolean} type
 * @param {boolean} loan @returns {string} */
export function readRustOwnedRecordStateWriteError(mutable,callable,scope,root,field,type,loan){
 if(!mutable)return 'record field assignment needs its actual mutable root binding';
 if(!callable)return 'a captured mutable record needs its separate callable environment state interpretation';
 if(!scope)return 'this expression needs an explicit record state result continuation before an outer place can be changed';
 if(root!=='available'||field!=='available'&&field!=='moved')return 'field reinitialization needs its still initialized record root and exact available or moved field';
 if(!type)return 'field assignment must preserve its actual complete source type and owner';
 return loan?'field assignment overlaps an actual surviving whole or field loan':'';
}
/** Scope-held field identities describe disjoint loans more precisely than
 * the containing root's common lifetime. Unclassified contained loans remain
 * conservative rather than disappear into a TYPE contract.
 * @param {string} root @param {string} target @param {string[]} loans
 * @param {boolean} lexical @returns {boolean} */
export function testRustOwnedRecordStateLoanOverlap(root,target,loans,lexical){
 let precise=false;for(const loan of loans)if(testRustOwnedRecordFieldDescendant(root,loan)){precise=true;if(loan===target||testRustOwnedRecordFieldDescendant(target,loan)||testRustOwnedRecordFieldDescendant(loan,target))return true;}
 if(precise)return false;
 return lexical||loans.includes(root);
}
/** Ordered packet positions are ordinary tuple terms, not source tuples and
 * not opaque state. Position zero is the actual expression result.
 * @param {number} roots @returns {number[]} */
export function planRustOwnedRecordStatePacket(roots){if(!Number.isInteger(roots)||roots<0||roots>256)return [];const result=[0];for(let index=0;index<roots;index+=1)result.push(index+1);return result;}
/** Assignment consumes RHS first; declaration-order reconstruction replaces
 * only the actual selected child. Others retain their existing place state.
 * @param {number} fields @param {number} selected @returns {number[]} */
export function planRustOwnedRecordStateFields(fields,selected){if(!Number.isInteger(fields)||fields<1||fields>1024||!Number.isInteger(selected)||selected<0||selected>=fields)return [];const result=[];for(let index=0;index<fields;index+=1)result.push(index===selected?-1:index);return result;}
/** @param {string} root @param {number[]} fields @returns {string} */
export function readRustOwnedRecordStateFieldIdentity(root,fields){let result=root;for(const index of fields){result=readRustOwnedRecordFieldIdentity(result,index);if(result==='')return '';}return result;}
