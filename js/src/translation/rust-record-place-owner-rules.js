// Source place ownership follows actual shared references and canonical Box
// layers. The complete original position is retained by its caller; this view
// selects the nominal record underneath without granting a new source owner.
import {acceptRustOwnedBoxTypeNode} from './rust-owned-recursive-datatype-rules.js';
/** @param {object} position @param {boolean} [references] @returns {object|null} */
export function readRustRecordPlaceOwner(position,references=true){
 if(!position||!Array.isArray(position.wrappers)||position.wrappers.length>256||!Array.isArray(position.lifetimes))return null;
 let owner=position.owner,boxed=false,borrowed=false,count=0;
 for(const layer of position.wrappers){
  if(layer==='reference'&&references&&!boxed){borrowed=true;count++;continue;}
  if(layer!=='box'||!owner||!Array.isArray(owner.path)||!Array.isArray(owner.arguments)||!acceptRustOwnedBoxTypeNode(['box'],owner.kind,owner.path.join('::'),owner.arguments.length))return null;
  boxed=true;owner=owner.arguments[0];
 }
 if(count!==position.lifetimes.length||!owner||!Array.isArray(owner.path)||!Array.isArray(owner.arguments)||acceptRustOwnedBoxTypeNode(['box'],owner.kind,owner.path.join('::'),owner.arguments.length))return null;
 return {position:{...position,owner,wrappers:[],lifetimes:[]},borrowed,boxed};
}
/** Exact source owner transport, or an actual partial canonical Box projection.
 * Core type agreement belongs to the independent expression checker.
 * @param {object} source @param {object} target @returns {number} */
export function readRustRecordPlaceBoxProjectionDepth(source,target){
 if(!readRustRecordPlaceOwner(source,false)||!readRustRecordPlaceOwner(target,false)||source.arrayElementPosition||target.arrayElementPosition)return -1;
 const depth=source.wrappers.length-target.wrappers.length;if(depth<0)return -1;
 let owner=source.owner;for(let index=0;index<depth;index+=1)owner=owner.arguments[0];
 const pending=[owner,target.owner];let visits=0;
 while(pending.length){
  if(++visits>65536)return -1;
  const right=pending.pop(),left=pending.pop();
  if(!left||!right||left.kind!==right.kind||!Array.isArray(left.path)||!Array.isArray(right.path)||left.path.length!==right.path.length||left.path.some((part,index)=>part!==right.path[index])||!Array.isArray(left.arguments)||!Array.isArray(right.arguments)||left.arguments.length!==right.arguments.length)return -1;
  for(let index=0;index<left.arguments.length;index+=1)pending.push(left.arguments[index],right.arguments[index]);
 }
 return depth;
}
