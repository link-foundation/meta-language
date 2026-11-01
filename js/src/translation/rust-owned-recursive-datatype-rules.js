// Destructor-free recursive value ownership over the actual declaration graph.
// No Copy implementation or runtime producer follows from this type profile.
import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
import {planRustNominalOwnershipOwners,readRustNominalOwnershipGraphError} from './rust-nominal-ownership-rules.js';
/** @param {string[]} owners @param {boolean[]} copies @param {boolean[]} clones
 * @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} targets @param {number[]} roots @param {boolean[]} leaves
 * @returns {string[]} */
export function planRustOwnedRecursiveNominalOwners(owners,copies,clones,kinds,ends,edges,targets,roots,leaves){
 const checked=planRustNominalOwnershipOwners(owners,copies,clones,kinds,ends,edges,targets,roots);
 if(readRequiredTextFact(checked,0)!=='profiles'||leaves.length!==kinds.length||readRustNominalOwnershipGraphError(kinds.length,edges.length,0)!=='')return ['error','incomplete actual recursive datatype graph'];
 const count=kinds.length,size=owners.length,eligible=owners.map(()=>true),copyNodes=[];let work=0;
 // A tuple retains its old direct Copy interpretation in this first profile.
 for(let at=0;at<count;at+=1){const kind=readRequiredTextFact(kinds,at),target=readRequiredNumberFact(targets,at),start=at===0?0:readRequiredNumberFact(ends,at-1),end=readRequiredNumberFact(ends,at);let copy=target>=0?readRequiredBooleanFact(copies,target):kind==='copy';if(kind==='tuple'){copy=true;for(let edge=start;edge<end;edge+=1)if(!readRequiredBooleanFact(copyNodes,readRequiredNumberFact(edges,edge)))copy=false;}copyNodes.push(copy);}
 // Source Rust requires every infinite-size cycle to cross actual Box/Vec.
 // The emitter's independent indirection is never evidence for this property.
 for(let owner=0;owner<size;owner+=1){work+=count;if(work>1048576)return ['error','recursive datatype reasoning exceeds its finite source profile'];const pending=[readRequiredNumberFact(roots,owner)],seen=[];for(let at=0;at<count;at+=1)seen.push(false);
  while(pending.length){if(++work>1048576)return ['error','recursive datatype reasoning exceeds its finite source profile'];const at=readRequiredNumberFact(pending,pending.length-1);pending.pop();if(readRequiredBooleanFact(seen,at))continue;seen[at]=true;const kind=readRequiredTextFact(kinds,at),target=readRequiredNumberFact(targets,at);if(target>=0){if(target===owner){eligible[owner]=false;break;}pending.push(readRequiredNumberFact(roots,target));continue;}if(kind==='box'||kind==='vector'||kind==='pending vector')continue;const start=at===0?0:readRequiredNumberFact(ends,at-1);for(let edge=start;edge<readRequiredNumberFact(ends,at);edge+=1)pending.push(readRequiredNumberFact(edges,edge));}
 }
 let changed=true;
 while(changed){changed=false;const safe=[];
  for(let at=0;at<count;at+=1){if(++work>1048576)return ['error','recursive datatype reasoning exceeds its finite source profile'];const kind=readRequiredTextFact(kinds,at),target=readRequiredNumberFact(targets,at),start=at===0?0:readRequiredNumberFact(ends,at-1),end=readRequiredNumberFact(ends,at);let value=target>=0?readRequiredBooleanFact(eligible,target):kind==='copy'||kind==='string'?readRequiredBooleanFact(leaves,at):['box','vector','pending vector','option','result','constructor','nominal'].includes(kind)||kind==='tuple'&&readRequiredBooleanFact(copyNodes,at);if((kind==='box'||kind==='vector'||kind==='pending vector'||kind==='option')&&end-start!==1||kind==='result'&&end-start!==2||kind==='nominal'&&start===end)value=false;for(let edge=start;edge<end;edge+=1)if(!readRequiredBooleanFact(safe,readRequiredNumberFact(edges,edge)))value=false;safe.push(value);}
  for(let owner=0;owner<size;owner+=1)if(readRequiredBooleanFact(eligible,owner)&&!readRequiredBooleanFact(safe,readRequiredNumberFact(roots,owner))){eligible[owner]=false;changed=true;}
 }
 // Actual finite constructor witnesses, including empty Vec and None, prevent
 // uninhabited cycles from masquerading as constructive backend defaults.
 const inhabited=owners.map(()=>false),ranks=owners.map(()=>-1),witnesses=owners.map(()=>-1);let rank=0;changed=true;
 while(changed){changed=false;rank+=1;const values=[];
  for(let at=0;at<count;at+=1){if(++work>1048576)return ['error','recursive datatype reasoning exceeds its finite source profile'];const kind=readRequiredTextFact(kinds,at),target=readRequiredNumberFact(targets,at),start=at===0?0:readRequiredNumberFact(ends,at-1),end=readRequiredNumberFact(ends,at);let value=target>=0?readRequiredBooleanFact(inhabited,target):kind==='copy'||kind==='string'?readRequiredBooleanFact(leaves,at):['vector','pending vector','option'].includes(kind);if(kind==='constructor'||kind==='tuple'||kind==='box'){value=true;for(let edge=start;edge<end;edge+=1)if(!readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge)))value=false;}if(kind==='nominal'||kind==='result'){value=false;for(let edge=start;edge<end;edge+=1)if(readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge)))value=true;}values.push(value);}
  for(let owner=0;owner<size;owner+=1)if(readRequiredBooleanFact(eligible,owner)&&!readRequiredBooleanFact(inhabited,owner)){const root=readRequiredNumberFact(roots,owner),start=root===0?0:readRequiredNumberFact(ends,root-1);for(let edge=start;edge<readRequiredNumberFact(ends,root);edge+=1)if(readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge))){inhabited[owner]=true;ranks[owner]=rank;witnesses[owner]=edge-start;changed=true;break;}}
 }
 const implemented=owners.map((_,owner)=>readRequiredBooleanFact(eligible,owner)&&readRequiredBooleanFact(inhabited,owner)&&(readRequiredBooleanFact(copies,owner)||readRequiredBooleanFact(clones,owner)));changed=true;
 while(changed){changed=false;const values=[];
  for(let at=0;at<count;at+=1){if(++work>1048576)return ['error','recursive datatype reasoning exceeds its finite source profile'];const kind=readRequiredTextFact(kinds,at),target=readRequiredNumberFact(targets,at),start=at===0?0:readRequiredNumberFact(ends,at-1);let value=target>=0?readRequiredBooleanFact(implemented,target):kind==='copy'||kind==='string'?readRequiredBooleanFact(leaves,at):['box','vector','pending vector','option','result','constructor','nominal','tuple'].includes(kind);for(let edge=start;edge<readRequiredNumberFact(ends,at);edge+=1)if(!readRequiredBooleanFact(values,readRequiredNumberFact(edges,edge)))value=false;values.push(value);}
  for(let owner=0;owner<size;owner+=1)if(readRequiredBooleanFact(implemented,owner)&&!readRequiredBooleanFact(values,readRequiredNumberFact(roots,owner))){implemented[owner]=false;changed=true;}
 }
 const result=['profiles'];for(let owner=0;owner<size;owner+=1){const admitted=readRequiredBooleanFact(eligible,owner)&&readRequiredBooleanFact(inhabited,owner);result.push(admitted?readRequiredBooleanFact(copies,owner)?'copy':'owned':'');result.push(readRequiredBooleanFact(implemented,owner)?'Clone':'');result.push(admitted?'closed':'');result.push(String(readRequiredNumberFact(ranks,owner)));result.push(String(readRequiredNumberFact(witnesses,owner)));}return result;
}
/** @param {string[]} wrappers @param {string} owner @param {string} path
 * @param {number} arguments_ @returns {boolean} */
export function acceptRustOwnedBoxTypeNode(wrappers,owner,path,arguments_){return wrappers.length>0&&readRequiredTextFact(wrappers,0)==='box'&&owner==='library'&&['std::boxed::Box','alloc::boxed::Box'].includes(path)&&arguments_===1;}
/** @param {boolean} data @param {boolean} complete @param {string} role @returns {boolean} */
export function acceptRustOwnedNominalHeaderType(data,complete,role){return data&&complete&&(role==='copy'||role==='owned');}
/** @param {string} kind @param {string[]} wrappers @param {boolean} proven
 * @param {string} owner @param {string} path @param {number} arguments_
 * @param {string} model @param {string} leaf @param {boolean} copy
 * @param {string} nominal @param {boolean} scope @returns {string} */
export function readRustOwnedContainerTypeNode(kind,wrappers,proven,owner,path,arguments_,model,leaf,copy,nominal,scope){
 if(!scope||!proven)return 'unknown';
 if(acceptRustOwnedBoxTypeNode(wrappers,owner,path,arguments_))return 'box';
 if(wrappers.length)return wrappers.every(value=>value==='reference')?'reference':'unknown';
 if(kind==='named'&&(nominal==='copy'||nominal==='owned'))return 'named';
 if(kind==='array'&&owner==='library'&&['std::vec::Vec','alloc::vec::Vec'].includes(path)&&arguments_===1&&model==='owned vector')return 'vector';
 if(leaf==='copy'&&copy&&['bool','fixed','float','unit'].includes(kind))return 'copy';
 if(leaf==='string')return 'string';
 return kind==='tuple'&&leaf==='tuple'&&copy?'tuple':'unknown';
}
/** @param {string} owner @param {number} arguments_ @param {boolean} proven
 * @param {boolean} provisional @param {number} wrappers @returns {string} */
export function readRustOwnedContainerSumNode(owner,arguments_,proven,provisional,wrappers){return proven&&!provisional&&wrappers===0?owner==='std::option::Option'&&arguments_===1?'option':owner==='std::result::Result'&&arguments_===2?'result':'unknown':'unknown';}
/** @param {string} kind @param {string[]} children @param {string} nominal @returns {string} */
export function readRustOwnedContainerNodeRole(kind,children,nominal){
 if(kind==='copy'||kind==='reference')return 'copy';if(kind==='string')return 'owned';if(kind==='named')return nominal==='copy'||nominal==='owned'?nominal:'';
 if(children.some(value=>value!=='copy'&&value!=='owned'))return '';
 if(kind==='box'||kind==='vector')return children.length===1?'owned':'';
 if(kind==='tuple')return children.every(value=>value==='copy')?'copy':'';
 if(kind==='option'&&children.length===1||kind==='result'&&children.length===2)return children.includes('owned')?'owned':'copy';
 return '';
}
/** @param {string} kind @param {boolean[]} children @param {boolean} nominal @returns {boolean} */
export function acceptRustOwnedContainerClone(kind,children,nominal){return kind==='copy'||kind==='reference'||kind==='string'||kind==='named'&&nominal||['box','vector','tuple','option','result'].includes(kind)&&children.every(value=>value);}
/** @param {string} owner @returns {string} */
export function readRustOwnedBoxCanonicalOwner(owner){return owner==='Box'||owner==='std::boxed::Box'?'std::boxed::Box':owner==='alloc::boxed::Box'?'alloc::boxed::Box':'';}
/** @param {string} owner @param {string} payload @param {number} arguments_ @returns {string} */
export function readRustOwnedBoxConstructorError(owner,payload,arguments_){return ['std::boxed::Box','alloc::boxed::Box'].includes(owner)&&(payload==='copy'||payload==='owned')&&arguments_===1?'':'Box construction needs its authentic standard owner and one actual admitted payload';}
/** @param {boolean} tracking @param {boolean} available @param {boolean} produced @returns {string} */
export function readRustOwnedBoxMoveError(tracking,available,produced){return tracking&&(available||produced)?'':'moving a Box needs its actual available binding or completed allocation producer';}
/** @param {string} role @param {boolean} closed @returns {boolean} */
export function acceptRustOwnedNominalVectorElement(role,closed){return role==='owned'&&closed;}
/** @param {boolean} ordinary @param {number} minor @returns {boolean} */
export function acceptRustOwnedContainerSource(ordinary,minor){return ordinary&&Number.isInteger(minor)&&minor>=90&&minor<100;}
