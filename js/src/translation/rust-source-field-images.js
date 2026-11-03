// Actual declaration fields have independent ownership roots inside their
// containing lifetime. Images are private proof coordinates, never archived.
import {unsupported} from './diagnostics.js';
import {sameType} from './types.js';
import {readRustRecordPlaceOwner,readRustRecordPlaceBoxProjectionDepth} from './rust-record-place-owner-rules.js';
import {sameRustSourcePlaceOwner} from './rust-source-place-transforms.js';
import {checkRustSourceExpressionPosition} from './rust-source-expression-positions.js';
import {readRustOwnedRecordStateStorageError} from './rust-owned-record-state-rules.js';
import {readRustBorrowedSourceReadError,consumeRustBorrowedSourceRegion,expireRustBorrowedSourceRegion} from './rust-borrowed-source-region-rules.js';
const fail=(reason,node)=>{throw unsupported('Rust source record field image',reason,node?.span);};
export function sameRustSourceRecordImagePosition(left,right){return Boolean(left&&right)&&sameType(left.type,right.type)&&sameRustSourcePlaceOwner(left.owner,right.owner)&&left.wrappers.length===right.wrappers.length&&left.wrappers.every((value,index)=>value===right.wrappers[index])&&left.lifetimes.length===right.lifetimes.length&&left.lifetimes.every((value,index)=>value===right.lifetimes[index])&&!left.arrayElementPosition&&!right.arrayElementPosition;}
export function readRustSourceFieldLifetime(visitor,root,node){
 if(!Number.isSafeInteger(root)||root<0||root>=visitor.states.length)fail('a component lifetime belongs to an actual allocated source place',node);
 let lifetime=visitor.fieldLifetimes.get(root);if(lifetime===undefined){lifetime=visitor.root(visitor.lifetimes[root]??'');visitor.fieldLifetimes.set(root,lifetime);}return lifetime;
}
export function retireRustSourceFieldLifetimes(visitor,roots,mode,node){
 if(!['move','expire'].includes(mode))fail('a component lifetime ends by actual transfer or scope expiry',node);
 for(const root of roots){const lifetime=visitor.fieldLifetimes.get(root);if(lifetime!==undefined)visitor.states[lifetime]=mode==='move'?consumeRustBorrowedSourceRegion(visitor.states[lifetime]):expireRustBorrowedSourceRegion(visitor.states[lifetime]);}
}
export function prepareRustSourceRecordFieldImage(visitor,root,position,node){
 if(root===null||root===undefined||!position)return;readRustSourceFieldLifetime(visitor,root,node);if(position.type.kind!=='data')return;
 const pending=[{root,position,depth:0,active:[]}];let visits=0;
 while(pending.length){
  const current=pending.pop(),error=readRustOwnedRecordStateStorageError(++visits,current.depth);if(error)fail(error,node);
  readRustSourceFieldLifetime(visitor,current.root,node);
  const view=readRustRecordPlaceOwner(current.position,false);if(!view||view.position.type.kind!=='data')continue;
  const data=visitor.program.declarations.get(view.position.type.name),fields=data?.sourceFieldSignatures?.ctors?.[0]?.fields;
  if(data?.k!=='data'||data.ctors.length!==1||!Array.isArray(fields)||fields.length!==data.ctors[0].fields.length)continue;
  const prior=visitor.fieldImages.get(current.root);if(prior){if(!sameRustSourceRecordImagePosition(prior.position,view.position))fail('a field image retains its actual original nominal declaration inventory',node);continue;}
  if(current.active.includes(data.fullName))fail('recursive single-constructor field storage needs its finite constructive image',node);
  const roots=fields.map(()=>visitor.root(visitor.lifetimes[current.root]??''));visitor.fieldImages.set(current.root,{position:visitor.position(view.position),fields:roots});
  for(let index=0;index<fields.length;index++)pending.push({root:roots[index],position:fields[index],depth:current.depth+1,active:[...current.active,data.fullName]});
 }
}
export function collectRustSourceRecordFieldRoots(visitor,roots,node){
 const pending=[...roots],seen=new Set();let visits=0;
 while(pending.length){const root=pending.pop();if(root===null||root===undefined||seen.has(root))continue;if(!Number.isSafeInteger(root)||root<0||root>=visitor.states.length)fail('a field image refers to an actual allocated source region',node);seen.add(root);const budget=readRustOwnedRecordStateStorageError(++visits,0);if(budget)fail(budget,node);const lifetime=visitor.fieldLifetimes.get(root);pending.push(...(lifetime===undefined?[]:[lifetime]),...(visitor.fieldImages.get(root)?.fields??[]),...(visitor.borrowedFieldImages.get(root)?.values()??[]));}return [...seen];
}
export function checkRustSourceRecordFieldImage(visitor,roots,node){
 for(const root of collectRustSourceRecordFieldRoots(visitor,roots,node)){const error=readRustBorrowedSourceReadError(visitor.states[root]??'absent',[]);if(error)fail('a selected or whole record image contains an unavailable field: '+error,node);}
}
export function rebaseRustSourceRecordFieldImage(visitor,source,target,mapping,node){
 const pending=[[source,target]],seen=new Map();let visits=0;
 while(pending.length){
  const [prior,current]=pending.pop(),budget=readRustOwnedRecordStateStorageError(++visits,0);if(budget)fail(budget,node);
  if(seen.has(prior)){if(seen.get(prior)!==current)fail('a joined field image retains its actual unique corresponding storage component',node);continue;}seen.set(prior,current);mapping.set(prior,current);
  const lifetime=visitor.fieldLifetimes.get(prior);if(lifetime!==undefined)mapping.set(lifetime,readRustSourceFieldLifetime(visitor,current,node));
  const left=visitor.fieldImages.get(prior),right=visitor.fieldImages.get(current);if(Boolean(left)!==Boolean(right)||left&&(!sameRustSourceRecordImagePosition(left.position,right.position)||left.fields.length!==right.fields.length))fail('a joined field image retains the same actual declaration and complete field order',node);
  if(left)for(let index=0;index<left.fields.length;index++)pending.push([left.fields[index],right.fields[index]]);
  for(const [name,root]of visitor.borrowedFieldImages.get(prior)??[]){let views=visitor.borrowedFieldImages.get(current);if(!views){views=new Map();visitor.borrowedFieldImages.set(current,views);}let destination=views.get(name);if(destination===undefined){const image=visitor.fieldImages.get(root);if(!image)fail('a joined borrowed field view retains its checked declaration inventory',node);destination=visitor.root(visitor.lifetimes[current]??'');views.set(name,destination);prepareRustSourceRecordFieldImage(visitor,destination,image.position,node);}pending.push([root,destination]);}
 }
}
export function readRustSourceRecordProjectionOwner(visitor,base,env,bindings,position,depth){
 if(base.k==='var'&&env.has(base.name)&&!base.sourceStateTransition&&!base.sourceLoopFold&&!base.sourcePlaceTransform&&!base.sourceRecordPlace){
  const actual=env.get(base.name);let qualified=null;
  if(base.sourcePosition){qualified=checkRustSourceExpressionPosition(base.sourcePosition,base.type,visitor.program.sourceLanguage,value=>value,value=>visitor.program.declarations.get(value.name)).position;visitor.scope(qualified,bindings,base);}
  const exact=sameRustSourceRecordImagePosition(actual.position,position),payload=!exact&&Boolean(actual.position)&&sameType(actual.position.type,position.type)&&readRustRecordPlaceBoxProjectionDepth(actual.position,position)>0;
  if((exact||payload)&&(!qualified||sameRustSourceRecordImagePosition(qualified,position))){const value=visitor.readLifetime(actual,base);return visitor.compatible(payload?{...value,position:visitor.position(position),role:visitor.value(position).role}:value,position,base);}
 }
 const actual=visitor.expression(base,env,bindings,position,depth+1);
 // A nested checked projection selects a container image without promoting
 // it to a whole usable value. Other owner expressions retain ordinary reads.
 return visitor.compatible(base.sourceRecordPlace?visitor.readLifetime(actual,base):visitor.read(actual,'read',base),position,base);
}
export function projectRustSourceRecordFieldImage(visitor,owner,facet,node){
 const borrowed=owner.role==='reference'||owner.borrowedPlace===true,view=readRustRecordPlaceOwner(facet.root,true),parents=owner.role==='reference'?(owner.borrowRoots??owner.regions):(owner.placeRoots??[owner.storageRoot??owner.root]);
 if(!view||!parents.length||parents.some(root=>root===null||root===undefined))fail('a field image retains its actual containing storage or referent roots',node);
 const containers=parents.map(root=>{
  const actual=visitor.fieldImages.get(root);if(owner.role==='reference'&&(!actual||!sameRustSourceRecordImagePosition(actual.position,view.position))){let views=visitor.borrowedFieldImages.get(root);if(!views){views=new Map();visitor.borrowedFieldImages.set(root,views);}let selected=views.get(view.position.type.name);if(selected===undefined){selected=visitor.root(visitor.lifetimes[root]??'');views.set(view.position.type.name,selected);prepareRustSourceRecordFieldImage(visitor,selected,view.position,node);}return selected;}
  prepareRustSourceRecordFieldImage(visitor,root,view.position,node);return root;
 }),selected=containers.map(root=>{const field=visitor.fieldImages.get(root)?.fields[facet.field];if(field===undefined)fail('a field image selects its actual declaration-order storage component',node);return field;}),ancestors=visitor.union([owner.role==='reference'?owner.regions:owner.placeLifetimes??[],containers.map(root=>readRustSourceFieldLifetime(visitor,root,node))]),lifetimes=visitor.union([ancestors,selected,selected.map(root=>readRustSourceFieldLifetime(visitor,root,node))]),inherited=visitor.union([owner.inheritedBorrowRoots??[],owner.role==='reference'?(owner.borrowRoots??owner.regions):[]]);
 return {...visitor.value(visitor.position(facet.value),selected[0],[],owner.producer),storageRoot:selected[0],storageIdentity:undefined,storageOwner:false,borrowedPlace:borrowed,placeRoots:[...new Set(selected)],placeRegions:lifetimes,placeLifetimes:ancestors,inheritedBorrowRoots:inherited};
}
