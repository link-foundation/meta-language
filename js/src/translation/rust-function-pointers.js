import {readRustSourceConstantValue,readRustSourceValueName} from './rust-source-constants.js';
// Real Rust signatures and declaration owners into existing callable types.
import {unsupported} from './diagnostics.js';
import {UNIT} from './types.js';
export {cloneRustSourceTypeOrigin as cloneRustCallableTypeOrigin} from './rust-source-type-origins.js';
import {cloneRustSourceTypeOrigin as cloneRustCallableTypeOrigin} from './rust-source-type-origins.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {acceptRustFunctionPointerSafeSource,readRustFunctionPointerDomainError,readRustFunctionPointerHeaderDomainError} from './rust-function-pointer-rules.js';
import {readRustFunctionPointerObservationError} from './rust-function-pointer-rules.js';
export function readRustFunctionPointerType(parser){
 const c=parser.cursor,start=c.next(),parameters=[],copies=[],parameterOrigins=[];c.expect('(','function pointer');
 while(!c.is(')')){if(c.isKind('identifier')&&c.is(':',1)){c.next();c.next();}parameters.push(parser.type());parameterOrigins.push(cloneRustCallableTypeOrigin(parser.origins.type));copies.push(parser.origins.immutableProduct(parser.origins.type));if(!c.eat(','))break;}
 c.expect(')','function pointer');let ret;if(c.eat('->'))ret=parser.type();else{ret=parser.origins.typed(UNIT,parser.modulePath);parser.origins.type.sourceOwner={kind:'unit',path:[],arguments:[]};}const result=parser.origins.type;
 const safe=acceptRustFunctionPointerSafeSource(start.value,start.raw),roles=safe?parameterOrigins.map(origin=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile):[],resultRole=safe?readRustStandardSumOwnershipFacts(parser.origins,result).profile:'';
 const actual=readRustFunctionPointerDomainError(parser.ordinaryCompletionContext(),copies,parser.origins.immutableProduct(result),roles,resultRole,parser.sourceVersionMinor),reason=readRustFunctionPointerHeaderDomainError(parser.ordinaryCompletionContext(),copies,parser.origins.immutableProduct(result),roles,resultRole,parser.sourceVersionMinor,parser.headerDiscovery===true);if(reason)throw unsupported('Rust function pointer',reason,start);
 const resultOrigin=cloneRustCallableTypeOrigin(result),type={kind:'function',params:parameters,ret};parser.origins.typed(type,parser.modulePath);parser.origins.type.copyEvidence=actual==='';parser.origins.type.proven=actual==='';if(actual!=='')parser.origins.type.sourceFunctionPointerHeader=true;parser.origins.type.callableParameters=parameterOrigins;parser.origins.type.callableResult=resultOrigin;return type;
}
export function readRustFunctionReference(parser,segments,path,span){
 if(segments.length===1&&parser.origins.scope.has(segments[0])){const origin=parser.origins.scope.get(segments[0]);return origin.type?.kind==='function'?parser.origins.mark({k:'name',path:segments,span},origin):null;}
 const entry=parser.origins.lookupValueOwner(readRustSourceValueName(parser,segments,path),path);if(entry?.k==='const')return readRustSourceConstantValue(parser,entry,path,span);if(entry?.k!=='fn')return null;
 const parameters=entry.parameters.map(cloneRustCallableTypeOrigin),result=cloneRustCallableTypeOrigin(entry.result),copies=parameters.map(origin=>parser.origins.immutableProduct(origin)),resultCopy=parser.origins.immutableProduct(result),roles=parameters.map(origin=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile),resultRole=readRustStandardSumOwnershipFacts(parser.origins,result).profile;
 const actual=readRustFunctionPointerDomainError(parser.ordinaryCompletionContext(),copies,resultCopy,roles,resultRole,parser.sourceVersionMinor);
 const reason=readRustFunctionPointerHeaderDomainError(parser.ordinaryCompletionContext(),copies,resultCopy,roles,resultRole,parser.sourceVersionMinor,parser.headerDiscovery===true);if(reason)throw unsupported('Rust function reference',reason,span);
 const type={kind:'function',params:entry.parameters.map(origin=>origin.type),ret:entry.result.type};
 const owners=[...parameters,result].map(origin=>origin.expandedSourceOwner??origin.sourceOwner);
 if(owners.some(owner=>!owner))throw unsupported('Rust function reference','the complete source function needs every actual parameter and result owner',span);
 return parser.origins.mark({k:'functionValue',path:['crate',...entry.fullName.split('.')],span},{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'function',path:[],arguments:owners.map(owner=>structuredClone(owner))},proven:actual==='',copyEvidence:actual==='',sourceFunctionPointerHeader:actual!=='',callableParameters:parameters,callableResult:result});
}
export function refuseRustCallableObservation(parser,value,path,span){
 const reason=readRustFunctionPointerObservationError(parser.origins.origin(value,path).type?.kind??'');if(reason)throw unsupported('Rust callable observation',reason,span);
}
