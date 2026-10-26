// Actual source ownership facts into the shared scalar read admission policy.
import {unsupported} from './diagnostics.js';
import {rustFixedType} from './types.js';
import {readRustScalarArrayIndexReceiverError,readRustScalarArrayIndexOperandError} from './rust-array-index-rules.js';
export function readRustScalarArrayIndex(parser,object,path){
 const open=parser.cursor.next(),index=parser.expr(path),close=parser.cursor.expect(']','array index');
 const span={start:object.span?.start??open.start,end:close.end},receiver=parser.origins.origin(object,path),argument=parser.origins.origin(index,path);
 const element=receiver.type?.kind==='array'?receiver.type.element:null;
 const receiverError=readRustScalarArrayIndexReceiverError(receiver.type?.kind??'',element?.kind??'',receiver.arraySourceModel??'',receiver.proven===true,receiver.wrappers.every(layer=>layer==='reference'||layer==='box'),receiver.arrayElementCopyEvidence===true);
 if(receiverError)throw unsupported('Rust scalar array index',receiverError,span);
 const owner=argument.sourceOwner?.kind==='primitive'?argument.sourceOwner.path.join('::'):'';
 const literal=index.k==='num'&&!index.type&&!index.negative;
 const operandError=readRustScalarArrayIndexOperandError(literal,argument.type?.kind??'',argument.type?.bits??0,argument.type?.signed===true,owner,argument.wrappers.length,argument.proven===true);
 if(operandError)throw unsupported('Rust scalar array index',operandError,index.span??span);
 // Bound the source usize domain before exact conversion to the existing
 // arbitrary-precision index operation; no f64 coordinate loses large bits.
 const position={k:'cast',arg:{k:'cast',arg:index,to:rustFixedType('usize'),...(literal&&{from:rustFixedType('usize')}),flavor:'exact',span:index.span},to:{kind:'nat'},flavor:'exact',span:index.span};
 const result={k:'index',object,index:position,span};
 const origin=element?parser.origins.typeOrigin(element,path):{type:null,path:[...path],wrappers:[],fields:null,textSourceModel:null,copyEvidence:true,proven:true,diverging:false,uncertain:false};
 return parser.origins.mark(result,origin);
}
