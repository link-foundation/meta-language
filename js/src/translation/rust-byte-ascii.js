// Original source facts and generic shared-term projection only.
import {BOOL,rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {projectRustArrayLookupElementOwner} from './rust-array-lookup-source-owners.js';
import {planRustByteAscii,readRustByteAsciiArity,readRustByteAsciiReceiverError} from './rust-byte-ascii-rules.js';
export function readRustByteAscii(parser,receiver,method,args,range,path){
 const arity=readRustByteAsciiArity(method.value,parser.ordinaryCompletionContext());if(arity<0)return null;
 const source=parser.origins.origin(receiver,path);if(source.type?.kind==='string')return null;
 if(args.length!==arity)throw new TranslationError('type',method.value+' needs no actual byte predicate arguments',range);
 if(parser.origins.collecting)return{k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver],span:range};
 const owner=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner),error=readRustByteAsciiReceiverError(source.type?.kind??'',String(source.type?.bits??''),source.type?.signed===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'));
 if(error)throw unsupported('Rust byte ASCII receiver',error,range);
 const name=parser.freshProductBinding(),stack=[],pop=()=>{if(!stack.length)throw new TypeError('a byte predicate source plan lost an earlier term');return stack.pop();};
 for(const term of planRustByteAscii(method.value)){if(term==='operand')stack.push({k:'name',path:[name],span:receiver.span??range});else if(['ge','le','and','or'].includes(term)){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}else if(/^[0-9]+$/u.test(term))stack.push({k:'num',value:term,type:rustFixedType('u8'),span:range});else throw new TypeError('unknown byte predicate source term');}
 if(stack.length!==1)throw new TypeError('a byte predicate source plan needs one complete result');
 return parser.origins.mark({k:'let',name,type:rustFixedType('u8'),value:receiver,body:stack[0],span:range},{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}});
}
