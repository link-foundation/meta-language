// Proven standard source owners into the explicit UTF-8 count operation.
import {unsupported,typeError} from './diagnostics.js';
import {STRING,rustFixedType} from './types.js';
import {readRustSourceTypeOwner} from './rust-source-signatures.js';
import {acceptRustReadOnlyQueryOwner,acceptRustStringEmptyReceiver} from './rust-read-only-query-rules.js';
import {readRustStringByteLengthOperation} from './rust-string-byte-length-rules.js';
import {acceptRustStringByteLengthOperation,readRustStringByteLengthArgumentError,renderRustStringByteLength} from './rust-string-byte-length-rules.js';
export function readRustStringByteLength(parser,receiver,args,span,path){
 const origin=parser.origins.origin(receiver,path);if(origin.arraySourceModel||origin.type?.kind!=='string'||!parser.ordinaryCompletionContext())return null;
 if(args.length)throw typeError('len expects 0 arguments but got '+args.length,span);
 const owner=origin.sourceOwner??readRustSourceTypeOwner(parser,origin.textSourceModel==='string slice'?['str']:['String'],[],STRING),standard=acceptRustReadOnlyQueryOwner('string empty',owner.kind,owner.path.join('::'));
 if(!parser.origins.collecting&&!acceptRustStringEmptyReceiver(origin.type.kind,origin.textSourceModel??'',origin.proven,origin.wrappers.every(layer=>layer==='reference'),standard))throw unsupported('Rust String byte length','the receiver needs a proven standard Unicode scalar str or String owner',span);
 const node={k:'math',op:readRustStringByteLengthOperation(),name:'Rust String byte length',args:[{value:receiver,spread:false}],span};
 return parser.origins.mark(node,parser.origins.typeOrigin(rustFixedType('usize'),path));
}
export function emitRustStringByteLength(emitter,node,target,render){
 if(!acceptRustStringByteLengthOperation(node.k,node.op))return null;
 const error=readRustStringByteLengthArgumentError(emitter.program.sourceLanguage,emitter.utf16Strings===true||emitter.program.utf16Strings===true,node.args.length,false,node.args[0]?.type.kind??'');if(error)throw unsupported('Rust String byte length',error,node.span);
 return renderRustStringByteLength(target,render(node.args[0]));
}
