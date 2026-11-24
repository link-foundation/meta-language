// Source facts and mechanical application of the complete shared encoder lambda.
import {readRuntimeProjectData} from '../project-data-loader.js';
import {FLOAT,array,rustFixedType} from './types.js';
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {projectRustOwnedLoanBindings} from './rust-owned-binding-rules.js';
import {readRustIteratorReduction} from './rust-iterator-reductions.js';
import {acceptRustStringByteViewCandidate,readRustStringByteViewError,readRustStringByteViewStorage,readRustStringByteViewLayers} from './rust-string-byte-view-rules.js';
let policy;
function template(span){
 policy??=readRuntimeProjectData(new URL('./data/rust-string-byte-template.lino',import.meta.url));
 if(policy.schemaVersion!==1||policy.model!=='portable Rust scalar string UTF8 byte lambda 1'||policy.template?.k!=='lambda')throw new TypeError('the packaged Rust UTF8 byte lambda is malformed');
 const instantiate=value=>Array.isArray(value)?value.map(instantiate):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).map(([key,child])=>[key,key==='span'?span:instantiate(child)])):value;
 return instantiate(policy.template);
}
export function readRustStringByteView(parser,receiver,method,args,range,path){
 if(!acceptRustStringByteViewCandidate(method.value,parser.ordinaryCompletionContext()))return null;
 let actual=parser.origins.origin(receiver,path);
 const error=readRustStringByteViewError(method.value,args.length,actual.type?.kind??'',actual.textSourceModel??'',actual.wrappers.length,actual.proven===true,actual.wrappers.every(layer=>layer==='reference'));
 if(error)throw unsupported('Rust string byte view',error,range);
 checkRustOwnedOrigin(parser.origins,actual,range);
 if(method.value==='into_bytes')parser.origins.consumeOwnedBinding(actual,range);else actual=parser.origins.captureOwnedBorrow(actual);
 const element=rustFixedType('u8'),type=array(element),name=parser.freshProductBinding(),storage=readRustStringByteViewStorage(method.value),layers=readRustStringByteViewLayers(method.value),owner={kind:'primitive',path:['u8'],arguments:[]};
 const encoded={k:'app',fn:template(range),args:[receiver],span:range},mapped={k:'method',object:encoded,op:'map',args:[{k:'lambda',params:[{name,type:FLOAT,span:range}],ret:element,captures:[],body:{k:'cast',arg:{k:'name',path:[name],span:range},from:FLOAT,to:element,flavor:'exact',span:range},span:range}],span:range};
 const origin={...parser.origins.typeOrigin(type,path),arraySourceModel:storage,arrayElementCopyEvidence:true,wrappers:Array(layers).fill('reference'),sourceLifetimes:layers?[actual.sourceLifetimes?.[0]??null]:[],sourceOwner:{kind:storage==='owned vector'?'library':'slice',path:storage==='owned vector'?['std','vec','Vec']:[],arguments:[owner]},sourceBindingIdentity:layers?actual.sourceBindingIdentity:null,sourceLoanBindingIdentities:layers?projectRustOwnedLoanBindings(actual.sourceBindingIdentity??'',actual.sourceLoanBindingIdentities??[]):[],sourceExtendingTemporaryBindings:[],sourceByteIterator:method.value==='bytes',freshArrayValue:storage==='owned vector'};
 const result=storage==='owned vector'?parser.origins.markVectorConstructor(mapped,origin):parser.origins.mark(mapped,origin);
 return method.value==='bytes'?readRustIteratorReduction(parser,result,method,args,range,path):result;
}
