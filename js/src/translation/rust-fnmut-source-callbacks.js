// Actual source callback binding inventory. Fn closures may instantiate FnMut;
// an owned FnOnce environment must never be relabelled as a reusable callback.
import {unsupported} from './diagnostics.js';
import {readRustCopyClosure} from './rust-copy-closures.js';
import {readRustFnMutSourceDeclaration} from './rust-fnmut-source-declarations.js';
import {cloneRustCallableTypeOrigin} from './rust-function-pointers.js';
import {withRustFnMutSourceClosure,readRustFnMutSourceClosureRecord} from './rust-fnmut-source-environments.js';
import {readRustRecordSourceType} from './rust-records.js';
import {eraseRustFnMutSourceOpacity} from './rust-fnmut-source-opacity.js';
const bindings=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut callback',reason,span);};
export function readRustFnMutSourceCallback(parser,declaration,parameters,result,path){
 const original=readRustFnMutSourceDeclaration(parser,declaration);if(!Array.isArray(parameters)||parameters.length!==original.callback.parameterRanges.length||parameters.length>256||!result?.proven||!result.type||parameters.some(value=>!value?.proven||!value.type))fail('the actual callback needs its complete concretely resolved source header',parser.cursor.peek());
 parameters=parameters.map(value=>({...cloneRustCallableTypeOrigin(eraseRustFnMutSourceOpacity(parser,value)),type:readRustRecordSourceType(parser,value.type,value.path),path:[]}));result={...cloneRustCallableTypeOrigin(eraseRustFnMutSourceOpacity(parser,result)),type:readRustRecordSourceType(parser,result.type,result.path),path:[]};
 const start=parser.cursor.peek(),expected={...parser.origins.typeOrigin({kind:'function',params:parameters.map(value=>value.type),ret:result.type},path),copyEvidence:true,callableParameters:parameters.map(cloneRustCallableTypeOrigin),callableResult:cloneRustCallableTypeOrigin(result)};
 if(start.value==='move')fail('move captures need their actual owned callback environment and creation-time transfer, not caller-root writeback',start);
 if(!['|','||','move'].includes(start.value))fail('this source specialization requires its actual immediate nonescaping callback expression',start);
 const node=withRustFnMutSourceClosure(parser,expected,()=>readRustCopyClosure(parser,path,expected,false,false,true)),environment=readRustFnMutSourceClosureRecord(parser,node);
 const binding=Object.freeze({identity:node.span.start}),record={parser,declaration,node,parameters:environment.parameterOrigins.map(cloneRustCallableTypeOrigin),result:cloneRustCallableTypeOrigin(environment.result),captures:environment.captures,immutableCaptures:environment.immutableCaptures,path:[...path],range:{start:node.span.start,end:node.span.end},mode:'exclusive'};bindings.set(binding,record);return binding;
}
export function testRustFnMutSourceCallbackBinding(parser,declaration,binding){const record=bindings.get(binding);return Boolean(record&&record.parser===parser&&record.declaration===declaration);}
export function readRustFnMutSourceCallbackBinding(parser,declaration,binding){const record=bindings.get(binding);if(!record||record.parser!==parser||record.declaration!==declaration)fail('the callback binding must originate from this actual source declaration and parser');return record;}
