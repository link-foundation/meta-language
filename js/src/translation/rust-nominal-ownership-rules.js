import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
import {acceptRustSourceArrayElement} from './rust-array-rules.js';
import {acceptRustRetainedFunctionOwner} from './rust-source-owner-retention-rules.js';
/** Source syntax/capability projection only. Header pointer Copy is a trait
 * fact about an actual fn pointer, never a parameter-domain Copy assumption.
 * @param {string} kind @param {string} bits @param {boolean} signed
 * @param {string[]} wrappers @param {boolean} proven @param {boolean} copy
 * @param {string} owner @param {string} path @param {number} ownerPaths @param {number} arguments_
 * @param {string} model @param {boolean} pointerHeader @param {number} parameters
 * @param {boolean} scope @param {boolean} closed @param {string} element
 * @returns {string} */
export function readRustNominalOwnershipLeaf(kind,bits,signed,wrappers,proven,copy,owner,path,ownerPaths,arguments_,model,pointerHeader,parameters,scope,closed,element){
 if(!scope||!closed)return 'unknown';
 if(wrappers.length)return wrappers.every(layer=>layer==='reference')?'copy':'unknown';
 if(proven&&copy&&acceptRustPrimitiveFromSource(kind,bits,signed,owner,path,arguments_,true,0))return 'copy';
 if(kind==='unit'&&proven&&copy&&owner==='unit'&&path===''&&arguments_===0)return 'copy';
 if(kind==='function'&&(proven&&copy||pointerHeader)&&acceptRustRetainedFunctionOwner('Rust source function signature 2',owner,kind,ownerPaths,arguments_,parameters,false))return 'copy';
 if(kind==='string'&&proven&&owner==='library'&&['std::string::String','alloc::string::String'].includes(path)&&arguments_===0&&model==='owned string')return 'string';
 if(kind==='array'&&proven&&owner==='library'&&['std::vec::Vec','alloc::vec::Vec'].includes(path)&&arguments_===1&&model==='owned vector'&&acceptRustSourceArrayElement(element,false))return 'vector';
 if(kind==='tuple'&&owner==='tuple')return 'tuple';
 return kind==='named'&&owner==='declaration'?'named':'unknown';
}
/** Actual canonical stored sum structure may contribute type capabilities
 * before the finalizer upgrades its private header. No runtime flags change.
 * @param {string} owner @param {number} arguments_ @param {boolean} provisional
 * @param {boolean} proven @param {boolean} scope @returns {string} */
export function readRustNominalOwnershipSumNode(owner,arguments_,provisional,proven,scope){return scope&&(proven||provisional)?owner==='std::option::Option'&&arguments_===1?'option':owner==='std::result::Result'&&arguments_===2?'result':'unknown':'unknown';}
/** @param {string} kind @param {string} path @param {number} arguments_ @param {string} declared @returns {boolean} */
export function acceptRustNominalOwnershipDeclarationOwner(kind,path,arguments_,declared){return kind==='declaration'&&declared!==''&&path===declared&&arguments_===0;}
/** @param {number} nodes @param {number} edges @param {number} depth @returns {string} */
export function readRustNominalOwnershipGraphError(nodes,edges,depth){return !Number.isInteger(nodes)||!Number.isInteger(edges)||!Number.isInteger(depth)||nodes<0||edges<0||depth<0||nodes>65536||edges>131072||depth>256?'the actual nominal ownership field graph exceeds its finite source profile':'';}
/** @param {string} kind @param {string[]} roles @returns {string} */
export function readRustNominalOwnershipNodeRole(kind,roles){
 if(kind==='copy')return 'copy';
 if(kind==='string')return 'owned';
 if(kind==='vector')return roles.length===1&&(roles[0]==='copy'||roles[0]==='owned')?'owned':'';
 if(kind==='option'&&roles.length!==1||kind==='result'&&roles.length!==2)return '';
 if(!['constructor','tuple','option','result'].includes(kind)||roles.some(role=>role!=='copy'&&role!=='owned'))return '';
 const owned=roles.includes('owned');return kind==='tuple'&&owned?'':owned?'owned':'copy';
}
/** Actual source derives remain independent of representation cloning.
 * @param {string[]} owners @param {boolean[]} copies @param {boolean[]} clones
 * @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} targets @param {number[]} roots @returns {string[]} */
export function planRustNominalOwnershipOwners(owners,copies,clones,kinds,ends,edges,targets,roots){
 const count=kinds.length,size=owners.length;
 if(size>4096||copies.length!==size||clones.length!==size||roots.length!==size||ends.length!==count||targets.length!==count||readRustNominalOwnershipGraphError(count,edges.length,0)!=='')return ['error','incomplete actual nominal ownership graph'];
 let previous=0;
 for(let at=0;at<count;at+=1){const end=readRequiredNumberFact(ends,at),target=readRequiredNumberFact(targets,at);if(!Number.isInteger(end)||end<previous||end>edges.length||!Number.isInteger(target)||target< -1||target>=size)return ['error','incomplete actual nominal ownership graph'];for(let edge=previous;edge<end;edge+=1){const child=readRequiredNumberFact(edges,edge);if(!Number.isInteger(child)||child<0||child>=at)return ['error','nominal field edges need actual postorder identities'];}previous=end;}
 if(previous!==edges.length)return ['error','incomplete actual nominal ownership edges'];
 for(let owner=0;owner<size;owner+=1){const name=readRequiredTextFact(owners,owner),root=readRequiredNumberFact(roots,owner);if(name===''||owners.indexOf(name)!==owner||!Number.isInteger(root)||root<0||root>=count||readRequiredTextFact(kinds,root)!=='nominal')return ['error','incomplete actual nominal declaration roots'];if(readRequiredBooleanFact(copies,owner)&&!readRequiredBooleanFact(clones,owner))return ['error','actual nominal Copy must retain its complete Clone derivation'];}
 const known=owners.map((_,index)=>readRequiredBooleanFact(copies,index)?'copy':''),implemented=owners.map((_,index)=>readRequiredBooleanFact(copies,index));let changed=true,work=0;
 while(changed){changed=false;const roles=[],cloneFields=[];
  for(let at=0;at<count;at+=1){if(++work>1048576)return ['error','actual nominal ownership graph reasoning exceeds its finite source profile'];const childRoles=[],childClones=[],start=at===0?0:readRequiredNumberFact(ends,at-1);for(let edge=start;edge<readRequiredNumberFact(ends,at);edge+=1){const child=readRequiredNumberFact(edges,edge);childRoles.push(readRequiredTextFact(roles,child));childClones.push(readRequiredBooleanFact(cloneFields,child));}const target=readRequiredNumberFact(targets,at),kind=readRequiredTextFact(kinds,at);roles.push(target>=0?readRequiredTextFact(known,target):readRustNominalOwnershipNodeRole(kind,childRoles));cloneFields.push(target>=0?readRequiredBooleanFact(implemented,target):kind==='copy'||kind==='string'||['vector','tuple','constructor','option','result'].includes(kind)&&childClones.every(value=>value)&&readRustNominalOwnershipNodeRole(kind,childRoles)!=='');}
  for(let owner=0;owner<size;owner+=1){if(readRequiredBooleanFact(copies,owner))continue;const root=readRequiredNumberFact(roots,owner),start=root===0?0:readRequiredNumberFact(ends,root-1),end=readRequiredNumberFact(ends,root);let ready=end>start,clone=true;for(let edge=start;edge<end;edge+=1){const child=readRequiredNumberFact(edges,edge);if(readRequiredTextFact(roles,child)==='')ready=false;if(!readRequiredBooleanFact(cloneFields,child))clone=false;}if(ready&&readRequiredTextFact(known,owner)===''){known[owner]='owned';changed=true;}if(ready&&clone&&readRequiredBooleanFact(clones,owner)&&!readRequiredBooleanFact(implemented,owner)){implemented[owner]=true;changed=true;}}
 }
 const result=['profiles'];for(let owner=0;owner<size;owner+=1){result.push(readRequiredTextFact(known,owner));result.push(readRequiredBooleanFact(implemented,owner)?'Clone':'');}return result;
}
/** @param {boolean} ordinary @param {string} role @param {boolean} actualClone @param {number} wrappers @param {boolean} borrowed @param {boolean} tracking @param {boolean} available @param {boolean} produced @returns {string} */
export function readRustNominalCloneError(ordinary,role,actualClone,wrappers,borrowed,tracking,available,produced){return !ordinary||!actualClone?'nominal Clone needs its actual complete derived implementation':wrappers!==0||borrowed?'nominal Clone needs an actual direct source value':role==='copy'||role==='owned'&&tracking&&(available||produced)?'':'nominal Clone needs an available value or actual producer independently of its type';}
/** @param {string} role @returns {string} */
export function readRustNominalRecordUpdateError(role){return role==='owned'?'owned nominal record updates need their actual partial-field move interpretation':'';}
/** @param {boolean} implemented @returns {string} */
export function readRustNominalCloneImplementationError(implemented){return implemented?'':'Clone needs each actual nominal Clone implementation independently of ownership';}
/** A nominal match transfers one actual parent, then creates independent
 * selected field values. Whole-parent aliases and mixed alternative moves
 * need another partial-move interpretation.
 * @param {string[]} identities @param {boolean[]} fields @param {boolean[]} owned
 * @param {boolean[]} produced @param {boolean} deferred @returns {string[]} */
export function planRustNominalPatternTransfers(identities,fields,owned,produced,deferred){
 const size=identities.length;if(size>65536||fields.length!==size||owned.length!==size||produced.length!==size)return ['error','incomplete nominal pattern transfer facts'];
 const plan=['transfers'],seen=[];
 for(let index=0;index<size;index+=1){if(!readRequiredBooleanFact(fields,index)||!readRequiredBooleanFact(owned,index))continue;
  if(deferred)return ['error','owned nominal guarded fields need their deferred partial-move interpretation'];
  const identity=readRequiredTextFact(identities,index);if(identity===''&&!readRequiredBooleanFact(produced,index))return ['error','a nominal field move needs its actual available parent or actual producer'];
  for(let peer=0;peer<size;peer+=1)if(peer!==index&&readRequiredTextFact(identities,peer)===identity&&readRequiredBooleanFact(owned,peer)&&!readRequiredBooleanFact(fields,peer))return ['error','a whole nominal alias cannot also move its owned fields'];
  if(identity!==''&&!seen.includes(identity)){seen.push(identity);plan.push(String(index));}
 }
 return plan;
}
/** @param {boolean} same @returns {string} */
export function readRustNominalPatternAlternativeError(same){return same?'':'every alternative needs the same actual nominal field moves, owners and parent lineage';}
/** Source graph already authenticates the leaf owner. This extra vector
 * interpretation excludes indirection and callable storage independently of
 * their Copy/Clone traits.
 * @param {string} kind @param {string[]} wrappers @returns {boolean} */
export function acceptRustNominalVectorElementLeaf(kind,wrappers){return wrappers.length===0&&acceptRustSourceArrayElement(kind,false);}
/** @param {string[]} owners @param {boolean[]} copies @param {boolean[]} clones
 * @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} targets @param {number[]} roots @param {boolean[]} leaves
 * @returns {boolean[]} */
export function planRustNominalVectorElementOwners(owners,copies,clones,kinds,ends,edges,targets,roots,leaves){
 const checked=planRustNominalOwnershipOwners(owners,copies,clones,kinds,ends,edges,targets,roots);
 if(readRequiredTextFact(checked,0)!=='profiles'||leaves.length!==kinds.length)return [];
 const known=owners.map(()=>false);let changed=true,work=0;
 while(changed){changed=false;const safe=[];
  for(let at=0;at<kinds.length;at+=1){if(++work>1048576)return [];const target=readRequiredNumberFact(targets,at),kind=readRequiredTextFact(kinds,at);let actual=target>=0?readRequiredBooleanFact(known,target):['copy','string'].includes(kind)?readRequiredBooleanFact(leaves,at):['constructor','tuple','option','result','nominal'].includes(kind);const start=at===0?0:readRequiredNumberFact(ends,at-1),end=readRequiredNumberFact(ends,at);if(kind==='nominal'&&start===end)actual=false;for(let edge=start;edge<end;edge+=1)if(!readRequiredBooleanFact(safe,readRequiredNumberFact(edges,edge)))actual=false;safe.push(actual);}
  for(let owner=0;owner<owners.length;owner+=1)if(!readRequiredBooleanFact(known,owner)&&readRequiredBooleanFact(copies,owner)&&readRequiredBooleanFact(clones,owner)&&readRequiredBooleanFact(safe,readRequiredNumberFact(roots,owner))){known[owner]=true;changed=true;}
 }
 return known;
}
/** @param {string} profile @param {boolean} clone @param {boolean} fields @returns {boolean} */
export function acceptRustNominalVectorElement(profile,clone,fields){return profile==='copy'&&clone&&fields;}
/** Header deferral retains syntax only; its caller clears private admission
 * flags and the complete syntax pass must select an actual registry proof.
 * @param {boolean} legacy @param {boolean} nominal @param {boolean} header
 * @param {boolean} collecting @param {boolean} complete @param {string} kind
 * @param {number} wrappers @returns {string} */
export function readRustNominalVectorElementAdmission(legacy,nominal,header,collecting,complete,kind,wrappers){return legacy||nominal?'admit':header&&collecting&&!complete&&kind==='named'&&wrappers===0?'defer':'refuse';}
/** Nominal datatype fields retain their actual catalog arity; standard sum
 * constructor zero/one-field decisions remain a separate unchanged profile.
 * @param {boolean} actual @param {number} fields @param {number} arguments_
 * @returns {string} */
export function readRustNominalConstructorError(actual,fields,arguments_){return actual&&Number.isInteger(fields)&&Number.isInteger(arguments_)&&fields>=0&&fields<=65536&&fields===arguments_?'':'nominal construction needs its actual constructor owner and exact finite field arity';}
/** A pending TYPE graph constructor grants no value capability. Its actual
 * named element must receive a separate finite Copy/Clone field witness.
 * @param {string} kind @param {string[]} wrappers @param {boolean} proven
 * @param {string} owner @param {string} path @param {number} arguments_
 * @param {string} model @param {string} element @param {boolean} scope
 * @returns {string} */
export function readRustNominalOwnershipVectorTemplateNode(kind,wrappers,proven,owner,path,arguments_,model,element,scope){return scope&&kind==='array'&&wrappers.length===0&&proven&&owner==='library'&&['std::vec::Vec','alloc::vec::Vec'].includes(path)&&arguments_===1&&model==='owned vector'&&element==='named'?'pending vector':'unknown';}
/** @param {string[]} kinds @param {number[]} ends @param {number[]} edges
 * @param {number[]} targets @param {boolean[]} elements @returns {string[]} */
export function planRustNominalOwnershipVectorNodes(kinds,ends,edges,targets,elements){
 if(kinds.length!==ends.length||kinds.length!==targets.length||readRustNominalOwnershipGraphError(kinds.length,edges.length,0)!=='')return [];
 const result=[];let previous=0;
 for(let at=0;at<kinds.length;at+=1){const kind=readRequiredTextFact(kinds,at),end=readRequiredNumberFact(ends,at);if(!Number.isInteger(end)||end<previous||end>edges.length)return [];for(let edge=previous;edge<end;edge+=1){const child=readRequiredNumberFact(edges,edge);if(!Number.isInteger(child)||child<0||child>=at)return [];}
  let selected=kind;
  if(kind==='pending vector'){selected='unknown';if(end===previous+1){const child=readRequiredNumberFact(edges,previous),target=readRequiredNumberFact(targets,child);if(readRequiredTextFact(kinds,child)==='reference'&&Number.isInteger(target)&&target>=0&&target<elements.length&&readRequiredBooleanFact(elements,target))selected='vector';}}
  result.push(selected);previous=end;
 }
 return previous===edges.length?result:[];
}
/** @param {string} kind @param {string[]} wrappers @param {boolean} proven
 * @param {string} owner @param {string} path @param {number} arguments_
 * @param {string} model @param {boolean} element @returns {boolean} */
export function acceptRustNominalVerifiedVectorClone(kind,wrappers,proven,owner,path,arguments_,model,element){return kind==='array'&&wrappers.length===0&&proven&&owner==='library'&&['std::vec::Vec','alloc::vec::Vec'].includes(path)&&arguments_===1&&model==='owned vector'&&element;}
