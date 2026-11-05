/** @param {number} minor @returns {string} */
export function readRustOwnedRecordUpdateVersionError(minor){return Number.isInteger(minor)&&minor>=90&&minor<100?'':'owned record update source semantics are uncovered outside Rust >=1.90.0 <1.100.0';}
/** Shared references are Copy independently of their referent. This does not
 * upgrade the payload's Copy evidence or erase its intrinsic wrappers.
 * @param {boolean} proven @param {boolean} copy @param {string[]} wrappers
 * @returns {boolean} */
export function testRustOwnedRecordFieldCopy(proven,copy,wrappers){return proven&&(wrappers[0]==='reference'||wrappers.length===0&&copy);}
/** FIELD access has no match-ergonomic automatic reference layer.
 * @param {boolean} copy @param {boolean} borrowed @returns {string} */
export function readRustOwnedRecordFieldMode(copy,borrowed){return copy?'copied value':borrowed?'borrowed place':'owned place';}
/** Runtime field identities occupy a separate source-state namespace.
 * @param {string} root @param {number} index @returns {string} */
export function readRustOwnedRecordFieldIdentity(root,index){return root!==''&&Number.isInteger(index)&&index>=0&&index<1024?'record field '+root+' '+String(index):'';}
/** @param {string} identity @returns {string} */
export function readRustOwnedRecordFieldRoot(identity){
 if(!identity.startsWith('record field '))return '';
 const boundary=identity.lastIndexOf(' '),root=identity.slice(13,boundary),index=Number(identity.slice(boundary+1));
 return readRustOwnedRecordFieldIdentity(root,index)===identity?root:'';
}
/** Missing branch facts describe an untouched field only when its actual
 * root is still available in that same branch snapshot.
 * @param {string} root @returns {string} */
export function readRustOwnedRecordAbsentFieldState(root){return root==='available'?'available':'absent';}
/** @param {string} identity @returns {string} */
export function readRustOwnedRecordUltimateRoot(identity){let root=readRustOwnedRecordFieldRoot(identity);if(root==='')return '';for(let depth=0;depth<1024;depth+=1){const parent=readRustOwnedRecordFieldRoot(root);if(parent==='')return root;root=parent;}return '';}
/** @param {string} root @param {string} identity @returns {boolean} */
export function testRustOwnedRecordFieldDescendant(root,identity){let parent=readRustOwnedRecordFieldRoot(identity);for(let depth=0;depth<1024&&parent!=='';depth+=1){if(parent===root)return true;parent=readRustOwnedRecordFieldRoot(parent);}return false;}
/** @param {string} root @param {string[]} identities @param {string[]} states
 * @returns {string} */
export function readRustOwnedRecordWholeError(root,identities,states){
 if(identities.length!==states.length)return 'record field availability needs every actual source state';
 if(root==='')return '';
 for(let index=0;index<identities.length;index+=1)if(testRustOwnedRecordFieldDescendant(root,identities[index])&&states[index]!=='available')return 'a partially moved record cannot be used as a whole value';
 return '';
}
/** @param {string} root @param {string} field @returns {string} */
export function readRustOwnedRecordFieldError(root,field){return root==='available'&&field==='available'?'':'a record field requires its actual available root and unmoved field';}
/** @param {string} profile @param {boolean} owner @param {number} layers
 * @param {number} fields @param {number} positions @returns {string} */
export function readRustOwnedRecordUpdateError(profile,owner,layers,fields,positions){return (profile==='owned'||profile==='copy')&&owner&&layers===0&&Number.isInteger(fields)&&fields>=0&&fields<=1024&&positions===fields?'':'an owned record update needs its actual destructor-free struct and complete field layout';}
