// Structural source projection only. The existing shared primitive readers
// construct the complete operation body; no source operand executes here.
import {BOOL,FLOAT,rustFixedType} from './types.js';
import {readRustPrimitiveConversionHead} from './rust-primitive-try-from.js';
import {readRustPrimitiveAssociatedOwner} from './rust-primitive-associated-call-rules.js';
import {readRustPrimitiveDefaultOwner,acceptRustPrimitiveDefaultImport} from './rust-primitive-default-rules.js';
import {projectRustPrimitiveDefault} from './rust-primitive-default.js';
import {projectRustPrimitiveAssociatedBody} from './rust-primitive-associated-calls.js';
import {readRustPrimitiveFromOwner} from './rust-primitive-from-rules.js';
import {projectRustPrimitiveFrom} from './rust-primitive-from.js';
import {readRustPrimitiveFunctionValueArity,readRustPrimitiveFunctionValueParameterOwner,readRustPrimitiveFunctionValueResultOwner,readRustPrimitiveFromFunctionSource} from './rust-primitive-function-value-rules.js';
export function readRustPrimitiveFunctionValue(parser,path,expected=null){
 const head=readRustPrimitiveConversionHead(parser,path),c=parser.cursor;if(!head||c.is('(',head.offset))return null;
 const method=head.resolved.at(-1),trait=parser.aliases.get(path.join('.'))?.get('Default'),traitConflict=method==='default'&&Boolean(parser.origins.lookup(['Default'],path)||parser.origins.scope.has('Default')||trait&&!acceptRustPrimitiveDefaultImport(trait.join('::'),true,false));
 const owner=method==='default'?readRustPrimitiveDefaultOwner(head.resolved,parser.ordinaryCompletionContext(),head.shadowed||traitConflict,head.absolute):method==='from'?readRustPrimitiveFromOwner(head.resolved,parser.ordinaryCompletionContext(),head.shadowed,head.absolute):readRustPrimitiveAssociatedOwner(head.resolved,parser.ordinaryCompletionContext(),head.shadowed,head.absolute);
 const parameter=expected?.type?.kind==='function'?expected.callableParameters?.[0]:null,parameterOwner=parameter?.expandedSourceOwner??parameter?.sourceOwner,source=method==='from'?readRustPrimitiveFromFunctionSource(owner,expected?.type?.kind==='function'?expected.type.params.length:-1,parameter?.type?.kind??'',String(parameter?.type?.bits??''),parameter?.type?.signed===true,parameterOwner?.kind??'',parameterOwner?.path.join('::')??'',parameterOwner?.arguments.length??-1,parameter?.proven===true,parameter?.wrappers.length??-1):'';
 const conversion=method==='from'&&source!=='',arity=conversion?1:readRustPrimitiveFunctionValueArity(owner,method);if(arity<0)return null;
 const end=c.peek(head.offset-1),range={start:head.start.start,end:end.end},parameters=[],origins=[],args=[];
 const type=name=>name==='bool'?BOOL:name==='f64'?FLOAT:rustFixedType(name),owned=name=>({...parser.origins.typeOrigin(type(name),path),sourceOwner:{kind:'primitive',path:[name],arguments:[]},...(name==='f64'?{floatSourceModel:'binary64 scalar'}:{})});
 for(let index=0;index<arity;index+=1){const name=parser.freshProductBinding(),parameterOwner=conversion?source:readRustPrimitiveFunctionValueParameterOwner(owner,method,index),origin=owned(parameterOwner);const sourceSpan=index===0?{start:head.start.start,end:head.start.end}:{start:end.start,end:end.end};parameters.push({name,type:type(parameterOwner),span:null});origins.push(origin);args.push(parser.origins.mark({k:'name',path:[name],span:sourceSpan},origin));}
 const previous=parser.origins.enter();let body;
 try{parameters.forEach((parameter,index)=>parser.origins.scope.set(parameter.name,origins[index]));body=conversion?projectRustPrimitiveFrom(parser,args[0],owner,range,path):method==='default'?projectRustPrimitiveDefault(parser,owner,path,range):projectRustPrimitiveAssociatedBody(parser,args[0],{...head.start,value:method},args.slice(1),range,path);}
 finally{parser.origins.scope=previous;}
 for(let index=0;index<head.offset;index+=1)c.next();
 const resultOwner=conversion?owner:readRustPrimitiveFunctionValueResultOwner(owner,method),ret=type(resultOwner),node={k:'lambda',params:parameters,ret,captures:[],body,span:range},origin=parser.origins.typeOrigin({kind:'function',params:parameters.map(parameter=>parameter.type),ret},path);
 return parser.origins.mark(node,{...origin,copyEvidence:true,callableParameters:origins,callableResult:owned(resultOwner)});
}
