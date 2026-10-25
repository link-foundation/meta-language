// Real Rust signatures and declaration owners into existing callable types.
import {unsupported} from './diagnostics.js';
import {UNIT} from './types.js';
import {readRustFunctionPointerTypeError,readRustFunctionPointerObservationError} from './rust-function-pointer-rules.js';
export function readRustFunctionPointerType(parser){
 const c=parser.cursor,start=c.next(),parameters=[],copies=[];c.expect('(','function pointer');
 while(!c.is(')')){if(c.isKind('identifier')&&c.is(':',1)){c.next();c.next();}parameters.push(parser.type());copies.push(parser.origins.immutableProduct(parser.origins.type));if(!c.eat(','))break;}
 c.expect(')','function pointer');const ret=c.eat('->')?parser.type():parser.origins.typed(UNIT,parser.modulePath),result=parser.origins.type;
 const reason=readRustFunctionPointerTypeError(parser.ordinaryCompletionContext(),copies,parser.origins.immutableProduct(result));if(reason)throw unsupported('Rust function pointer',reason,start);
 const type={kind:'function',params:parameters,ret};parser.origins.typed(type,parser.modulePath);parser.origins.type.copyEvidence=true;return type;
}
export function readRustFunctionReference(parser,segments,path,span){
 if(segments.length===1&&parser.origins.scope.has(segments[0])){const origin=parser.origins.scope.get(segments[0]);return origin.type?.kind==='function'?parser.origins.mark({k:'name',path:segments,span},origin):null;}
 const entry=parser.origins.lookup(parser.resolve(segments,path),path);if(entry?.k!=='fn')return null;
 const reason=readRustFunctionPointerTypeError(parser.ordinaryCompletionContext(),entry.parameters.map(origin=>parser.origins.immutableProduct(origin)),parser.origins.immutableProduct(entry.result));if(reason)throw unsupported('Rust function reference',reason,span);
 const type={kind:'function',params:entry.parameters.map(origin=>origin.type),ret:entry.result.type};
 return parser.origins.mark({k:'functionValue',path:['crate',...entry.fullName.split('.')],span},{...parser.origins.typeOrigin(type,path),copyEvidence:true});
}
export function refuseRustCallableObservation(parser,value,path,span){
 const reason=readRustFunctionPointerObservationError(parser.origins.origin(value,path).type?.kind??'');if(reason)throw unsupported('Rust callable observation',reason,span);
}
