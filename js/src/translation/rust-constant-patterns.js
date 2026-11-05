// Actual standard primitive constant expression evidence into literal patterns.
import {readRustPrimitiveIntegerConstant} from './rust-primitive-integer-constants.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {typeError} from './diagnostics.js';
export function readRustPrimitiveConstantPattern(parser,path){
 if(!parser.ordinaryCompletionContext())return null;
 const value=readRustPrimitiveIntegerConstant(parser,path);if(!value)return null;
 const origin=parser.origins.origin(value,path),owner=readRustPointerIntegerContractOwner(origin.expandedSourceOwner??origin.sourceOwner),negative=value.k==='unary'&&value.op==='neg',literal=negative?value.arg:value;
 if(literal.k!=='num'||literal.type?.kind!=='fixed'||!owner)throw typeError('a primitive constant pattern needs its actual checked integer value and source owner',value.span);
 return {k:'typedNumLit',value:literal.value,type:literal.type,owner,...(negative?{negative:true}:{}),span:value.span};
}
