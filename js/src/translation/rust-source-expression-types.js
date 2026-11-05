// Type-only read facts, captured while the real source scope is live. This
// cache cannot supply ownership, Copy, loan, binding or production witnesses.
import {unsupported} from './diagnostics.js';
import {measureRustSourceExpressionType} from './rust-standard-sum-source-metadata.js';
import {readRustStandardSumSourceMetadataReservation} from './rust-standard-sum-source-metadata-rules.js';
import {selectRustPrimitiveExpressionOrigin} from './rust-primitive-expression-origins.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {readRustPointerIntegerOwnerJoinError,selectRustPointerIntegerResultOwner} from './rust-pointer-integer-owner-rules.js';
import {readRustSourceExpressionOrigin} from './rust-indirection-rules.js';
const caches=new WeakMap();
function cache(table){let value=caches.get(table);if(!value){value={types:new WeakMap(),nodes:0,units:0};caches.set(table,value);}return value;}
function operand(table,node,path,depth){if(depth>256)return null;const recorded=table.recordedExpressionOrigin(node);if(recorded)return recorded;if(node?.k==='name'&&node.path.length===1&&table.scope.has(node.path[0]))return table.scope.get(node.path[0]);if(node?.k==='num')return {type:node.type??null,path,wrappers:[],proven:true};return result(table,node,path,depth+1);}
function result(table,node,path,depth=0){
 if(depth>256)return null;
 if(node?.k==='binary'||node?.k==='unary'){
  if(readRustSourceExpressionOrigin(node.k,node.op??'')!=='primitive-operation')return null;
  const nodes=node.k==='binary'?[node.left,node.right]:[node.arg],operands=nodes.map(value=>operand(table,value,path,depth+1));if(operands.some(value=>!value?.proven||value.integerSourceOwnerConflict||value.wrappers.includes('box')))return null;
  const owners=operands.map(value=>readRustPointerIntegerContractOwner(value.sourceOwner,value.integerSourceOwnerConstraint??''));if(readRustPointerIntegerOwnerJoinError(owners))return null;
  if(node.op==='concat')return {type:{kind:'string'},path,wrappers:[],proven:true};
  const selected=selectRustPrimitiveExpressionOrigin(node,operands),pointer=selectRustPointerIntegerResultOwner(owners),actual=selected??(pointer?operands.find((value,index)=>owners[index]===pointer):null);if(!actual?.type)return null;
  return {type:actual.type,path:actual.path,wrappers:[],proven:true,sourceOwner:selected?.sourceOwner??{kind:'primitive',path:[pointer],arguments:[]},floatSourceModel:actual.type.kind==='float'?'binary64 scalar':null};
 }
 if(node?.k!=='app')return null;const callee=node.fn;
 let origin=table.recordedExpressionOrigin(callee);
 if(callee.k==='name'){
  if(callee.path.length===1&&table.scope.has(callee.path[0]))origin=table.scope.get(callee.path[0]);
  else{const entry=table.lookupValueOwner(callee.path,path);if(entry?.k==='fn'||entry?.k==='ctor')return entry.result;}
 }
 if(!origin?.proven||origin.sourceFunctionPointerHeader)return null;
 if(origin.callableResult)return origin.callableResult;
 return origin.type?.kind==='function'?{type:origin.type.ret,path:origin.path,proven:true}:null;
}
export function recordRustSourceExpressionType(table,node,path){
 const current=cache(table);if(table.recordedExpressionOrigin(node)){current.types.delete(node);return node;}if(table.collecting)return node;
 const actual=result(table,node,path);if(!actual?.proven||!actual.type||actual.sourceFunctionPointerHeader||actual.standardSumSource?.provisional){current.types.delete(node);return node;}
 if(current.types.has(node))return node;
 const cost=measureRustSourceExpressionType(actual.type,actual.path),next=readRustStandardSumSourceMetadataReservation(current.nodes,current.units,cost[0],cost[1]);if(next.length!==2)throw unsupported('Rust source expression type','the complete type read cache exceeds its source budget',node.span);current.nodes=next[0];current.units=next[1];current.types.set(node,{type:actual.type,path:actual.path,proven:true});return node;
}
export function readRustSourceExpressionType(table,node){return caches.get(table)?.types.get(node);}
export function reuseRustSourceExpressionType(table,from,to){const value=caches.get(table)?.types.get(from);if(value)cache(table).types.set(to,value);return to;}
