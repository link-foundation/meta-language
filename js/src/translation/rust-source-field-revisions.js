// Current declaration images share untouched actual components. Rebuilding a
// containing value changes its whole-read revision, not every sibling lifetime.
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordStateStorageError} from './rust-owned-record-state-rules.js';
import {readRustBorrowedSourceReadError,consumeRustBorrowedSourceRegion} from './rust-borrowed-source-region-rules.js';
import {prepareRustSourceRecordFieldImage,readRustSourceFieldLifetime,retireRustSourceFieldLifetimes,sameRustSourceRecordImagePosition} from './rust-source-field-images.js';
const fail=(reason,node)=>{throw unsupported('Rust source field revisions',reason,node?.span);};
function fields(visitor,image,node){const data=visitor.program.declarations.get(image.position.type.name),positions=data?.sourceFieldSignatures?.ctors?.[0]?.fields;if(data?.k!=='data'||data.ctors.length!==1||!Array.isArray(positions)||positions.length!==image.fields.length)fail('a component revision retains its actual complete single-constructor declaration fields',node);return positions;}
export function reviseRustSourceFieldImage(visitor,source,position,paths,node){
 if(!Array.isArray(paths)||!paths.length||paths.length>256||paths.some(path=>!Array.isArray(path)||path.length>128||path.some(index=>!Number.isSafeInteger(index)||index<0||Object.is(index,-0))))fail('component revisions retain bounded actual declaration-order paths',node);
 const tree={leaf:false,children:new Map()};for(const path of paths){let current=tree;for(const index of path){if(current.leaf)fail('selected component revisions have disjoint original paths',node);let child=current.children.get(index);if(!child){child={leaf:false,children:new Map()};current.children.set(index,child);}current=child;}if(current.leaf||current.children.size)fail('selected component revisions have distinct nonoverlapping original paths',node);current.leaf=true;}
 let visits=0;const retired=[];
 const build=(prior,actual,selection,depth)=>{
  const budget=readRustOwnedRecordStateStorageError(++visits,depth);if(budget)fail(budget,node);
  const root=visitor.root(visitor.lifetimes[prior]??'');
  if(selection.leaf){prepareRustSourceRecordFieldImage(visitor,root,actual,node);retired.push({root:prior,whole:true});return root;}
  const error=readRustBorrowedSourceReadError(visitor.states[prior]??'absent',[]);if(error)fail('a changed ancestor retains its actual initialized storage: '+error,node);
  const image=visitor.fieldImages.get(prior);if(!image)fail('a selected revision retains its actual declaration inventory at every ancestor',node);const positions=fields(visitor,image,node);
  visitor.fieldLifetimes.set(root,readRustSourceFieldLifetime(visitor,prior,node));const children=image.fields.map((field,index)=>selection.children.has(index)?build(field,positions[index],selection.children.get(index),depth+1):field);
  if([...selection.children.keys()].some(index=>!Number.isSafeInteger(index)||index<0||index>=children.length))fail('a selected revision uses actual declaration-order field coordinates',node);
  visitor.fieldImages.set(root,{position:visitor.position(image.position),fields:children});retired.push({root:prior,whole:false});return root;
 };
 const root=build(source,position,tree,0);
 for(const entry of retired){visitor.states[entry.root]=consumeRustBorrowedSourceRegion(visitor.states[entry.root]);if(entry.whole)retireRustSourceFieldLifetimes(visitor,[entry.root],'move',node);}
 return root;
}
export function joinRustSourceFieldImages(visitor,roots,position,target,node){
 if(!Array.isArray(roots)||!roots.length||roots.length>65536||roots.some(root=>!Number.isSafeInteger(root)||root<0||root>=visitor.states.length))fail('a component join retains its actual allocated bounded source alternatives',node);
 let visits=0;
 const build=(sources,actual,depth,allocated=null)=>{
  const budget=readRustOwnedRecordStateStorageError(visits+=1+sources.length,depth);if(budget)fail(budget,node);
  if(sources.every(root=>root===sources[0])&&allocated===null)return sources[0];
  const root=allocated??visitor.root(visitor.lifetimes[sources[0]]??''),images=sources.map(source=>visitor.fieldImages.get(source)),first=images[0],lifetimes=sources.map(source=>visitor.fieldLifetimes.get(source));
  if(lifetimes[0]!==undefined&&lifetimes.every(lifetime=>lifetime===lifetimes[0]))visitor.fieldLifetimes.set(root,lifetimes[0]);else readRustSourceFieldLifetime(visitor,root,node);
  if(images.some(image=>Boolean(image)!==Boolean(first)||image&&(!sameRustSourceRecordImagePosition(image.position,first.position)||image.fields.length!==first.fields.length)))fail('joined component revisions retain the same actual declaration inventory and complete field order',node);
  if(first){const positions=fields(visitor,first,node),children=positions.map((field,index)=>build(images.map(image=>image.fields[index]),field,depth+1));visitor.fieldImages.set(root,{position:visitor.position(first.position),fields:children});}
  return root;
 };
 return build(roots,position,0,target);
}
