// Private interpretation proof for genuine source storage. A public qualifier,
// mask, packet spelling or constructor cannot manufacture this authority.
import {unsupported} from './diagnostics.js';
import {prepareRustSourceRecordFieldImage,collectRustSourceRecordFieldRoots,rebaseRustSourceRecordFieldImage,checkRustSourceRecordFieldImage} from './rust-source-field-images.js';
import {readRustBorrowedSourceReadError} from './rust-borrowed-source-region-rules.js';
const fail=(reason,node)=>{throw unsupported('Rust source partial record state',reason,node?.span);};
export function sealRustSourceStateImage(visitor,value,node){
 const storage=visitor.storages.get(value.storageIdentity);if(!storage||storage.root!==value.storageRoot)fail('state image authority belongs to the actual initialized current source storage',node);
 let proof=visitor.stateImageProofs.get(storage.root);if(!proof){proof={root:storage.root,identity:value.storageIdentity};visitor.stateImageProofs.set(storage.root,proof);}
 if(proof.identity!==value.storageIdentity)fail('a state image retains one actual original source binder',node);
 return {...value,stateImageProof:proof};
}
export function testRustSourceStateImage(visitor,value){
 const proof=value?.stateImageProof,storage=value?.storageIdentity&&visitor.storages.get(value.storageIdentity);
 return Boolean(proof&&storage&&proof===visitor.stateImageProofs.get(value.storageRoot)&&proof.root===storage.root&&proof.root===value.storageRoot&&proof.identity===value.storageIdentity);
}
function selectedField(visitor,source,position,path,node){
 if(!Array.isArray(path)||path.length>128)fail('partial state publication retains a real bounded declaration field path',node);
 prepareRustSourceRecordFieldImage(visitor,source,position,node);let selected=source;
 for(const index of path){const error=readRustBorrowedSourceReadError(visitor.states[selected]??'absent',[]);if(error)fail('field assignment retains initialized actual containing ancestors: '+error,node);const image=visitor.fieldImages.get(selected);if(!image||!Number.isSafeInteger(index)||index<0||index>=image.fields.length)fail('field assignment retains its actual complete declaration-order component path',node);selected=image.fields[index];}
 return selected;
}
export function checkRustSourceSelectedFieldImage(visitor,source,position,path,node){checkRustSourceRecordFieldImage(visitor,[selectedField(visitor,source,position,path,node)],node);}
export function publishRustSourceFieldUpdates(visitor,source,position,paths,node){
 if(!Array.isArray(paths)||!paths.length||paths.length>256)fail('a source field update retains its actual bounded selected place inventory',node);
 const selected=paths.map(path=>selectedField(visitor,source,position,path,node));
 for(let index=0;index<paths.length;index++)for(const prior of paths.slice(0,index))if(prior.slice(0,Math.min(prior.length,paths[index].length)).every((field,at)=>field===paths[index][at]))fail('a source field update retains distinct disjoint selected places',node);
 const target=visitor.root(),mapping=new Map();prepareRustSourceRecordFieldImage(visitor,target,position,node);rebaseRustSourceRecordFieldImage(visitor,source,target,mapping,node);
 for(const [prior,current]of mapping){const state=visitor.states[prior];if(!['available','moved','possibly moved','dropped'].includes(state))fail('a carried component retains its actual known source availability',node);visitor.states[current]=state;}
 // A complete checked RHS initializes only this leaf/subtree. Other moved or
 // conditional components retain their real state, even if the packet type is
 // the same ordinary whole nominal type.
 for(const component of selected){const replaced=mapping.get(component);if(replaced===undefined)fail('an update replaces exactly its actual selected component image',node);for(const root of collectRustSourceRecordFieldRoots(visitor,[replaced],node))visitor.states[root]='available';}
 return target;
}
export function mergeRustSourceStateImages(visitor,rows,target,snapshots,mappings,node){
 const components=new Map();for(let index=0;index<rows.length;index++){
  const source=rows[index].root,states=snapshots?.[index]??visitor.states,local=new Map();rebaseRustSourceRecordFieldImage(visitor,source,target,local,node);
  for(const [prior,current]of local){mappings[index].set(prior,current);const state=states[prior];if(!['available','moved','possibly moved','dropped'].includes(state))fail('a branch image retains every actual component availability observation',node);let alternatives=components.get(current);if(!alternatives){alternatives=[];components.set(current,alternatives);}alternatives.push(state);}
 }
 return components;
}
