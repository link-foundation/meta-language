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
