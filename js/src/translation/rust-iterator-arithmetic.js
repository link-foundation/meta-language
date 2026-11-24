// Actual item and explicit generic type facts; no source operand executes here.
import {sameType} from './types.js';
import {unsupported} from './diagnostics.js';
import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {readRustArrayLookupElementOwnerError} from './rust-array-lookup-rules.js';
import {readRustIteratorArithmeticError} from './rust-iterator-arithmetic-rules.js';
export function readRustIteratorArithmetic(parser,item,path,consumer){
 const c=parser.cursor,owner=item?.expandedSourceOwner??item?.sourceOwner,type=item?.type,knownOwner=Boolean(owner&&owner.kind==='primitive'&&owner.arguments.length===0&&type&&readRustArrayLookupElementOwnerError(type.kind,String(type.bits??''),type.signed===true,owner.path.join('::'))==='');
 let result={...structuredClone(item),wrappers:[],sourceLifetimes:[]};
 if(c.eat('::')){
  c.expect('<','iterator arithmetic result type');const previous=parser.iteratorItemTypeInferenceOrigin;parser.iteratorItemTypeInferenceOrigin={...structuredClone(item),wrappers:[],sourceLifetimes:[]};
  try{parser.type();result=structuredClone(parser.origins.type);}finally{parser.iteratorItemTypeInferenceOrigin=previous;}
  splitRustTypeClosingToken(parser);c.expect('>','iterator arithmetic result type');
 }
 c.expect('(','iterator arithmetic');const close=c.expect(')','iterator arithmetic');const resultOwner=result.expandedSourceOwner??result.sourceOwner;
 const agrees=Boolean(owner&&resultOwner&&resultOwner.arguments.length===0&&owner.kind===resultOwner.kind&&owner.path.join('::')===resultOwner.path.join('::'));
 const error=readRustIteratorArithmeticError(type?.kind??'',item?.proven===true,knownOwner,item?.wrappers.length??-1,result.proven===true,result.wrappers.length,Boolean(type&&result.type&&sameType(type,result.type)),agrees);if(error)throw unsupported('Rust iterator arithmetic',error,{start:consumer.start,end:close.end});
 return {origin:result,close};
}
