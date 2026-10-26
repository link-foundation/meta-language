// Actual source ownership facts into the shared scalar read admission policy.
import {unsupported} from './diagnostics.js';
import {rustFixedType} from './types.js';
import {readRustScalarArrayIndexReceiverError,readRustScalarArrayIndexOperandError} from './rust-array-index-rules.js';
import {readRustUsizeIndexExpression} from './rust-usize-index-expressions.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
export function readRustScalarArrayIndex(parser,object,path){
 const open=parser.cursor.next(),index=parser.expr(path),close=parser.cursor.expect(']','array index');
 const span={start:object.span?.start??open.start,end:close.end},receiver=parser.origins.origin(object,path);
 const element=receiver.type?.kind==='array'?receiver.type.element:null;
 const receiverError=readRustScalarArrayIndexReceiverError(receiver.type?.kind??'',element?.kind??'',receiver.arraySourceModel??'',receiver.proven===true,receiver.wrappers.every(layer=>layer==='reference'||layer==='box'),receiver.arrayElementCopyEvidence===true);
 if(receiverError)throw unsupported('Rust scalar array index',receiverError,span);
 const expressionError=readRustUsizeIndexExpression(parser,index,path);
 if(expressionError)throw unsupported('Rust scalar array index',readRustScalarArrayIndexOperandError(false,'',0,false,'',0,false),index.span??span);
 // Bound the source usize domain before exact conversion to the existing
 // arbitrary-precision index operation; no f64 coordinate loses large bits.
 const position={k:'cast',arg:{k:'cast',arg:index,to:rustFixedType('usize'),from:rustFixedType('usize'),flavor:'exact',span:index.span},to:{kind:'nat'},flavor:'exact',span:index.span};
 const result={k:'index',object,index:position,span};
 const origin=element?parser.origins.typeOrigin(element,path):{type:null,path:[...path],wrappers:[],fields:null,textSourceModel:null,copyEvidence:true,proven:true,diverging:false,uncertain:false};
 const owner=readRustPointerIntegerContractOwner(receiver.sourceOwner,receiver.integerSourceOwnerConstraint??'');
 return parser.origins.mark(result,owner?{...origin,sourceOwner:{kind:'primitive',path:[owner],arguments:[]}}:origin);
}
