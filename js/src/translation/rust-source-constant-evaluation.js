// Source declaration/typed-expression transport. All value operations are shared.
import {checkProgram} from './check.js';
import {createSurfaceReferenceFacts} from './surface-reference-facts.js';
import {readRustSourceValueName} from './rust-source-constants.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {rustFixedType} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {evaluateRustSourceConstant,readRustSourceConstantInitializerBoundary} from './rust-source-constant-evaluation-rules.js';
export function captureRustSourceConstantInitializer(parser,name,path,start,result){
 const c=parser.cursor,first=c.index,boundary=readRustSourceConstantInitializerBoundary(c.tokens.map(token=>token.kind),c.tokens.map(token=>token.value),first);
 if(boundary<0)throw parser.fail('a source constant initializer needs its complete declaration',c.peek());
 const row={kind:'constant',owner:[...path,name].join('.'),name,path:[...path],keyword:start.start,first:c.peek().start,end:c.tokens[boundary].start,result:parser.origins.copy(result)};
 parser.sourceConstantInitializers.set(row.owner,row);
 if(parser.headerDiscovery)c.index=boundary+1;
 return row;
}
export function captureRustSourceConstantFunction(parser,name,path,functionKeyword,constKeyword,parameters,result){
 const row={kind:'function',owner:[...path,name].join('.'),name,path:[...path],keyword:functionKeyword.start,constKeyword:constKeyword?.start??null,isConst:Boolean(constKeyword),inherent:parser.inherentOwner?structuredClone(parser.inherentOwner):null,parameters:parameters.map(origin=>parser.origins.copy(origin)),result:parser.origins.copy(result)};
 parser.sourceConstantFunctions.set(row.owner,row);
}
function constantDomain(origin){
 if(origin.type?.kind==='fixed'){const owner=readRustPointerIntegerContractOwner(origin.expandedSourceOwner??origin.sourceOwner);if(rustFixedType(owner))return owner;}
 if(origin.type?.kind==='bool')return 'bool';
 if(origin.type?.kind==='unit'&&!origin.wrappers.length)return 'unit';
 if(origin.type?.kind==='string'&&origin.textSourceModel==='borrowed str'&&origin.wrappers[0]==='reference')return 'str';
 throw unsupported('Rust source constant pattern','the declaration needs its actual structural primitive source type');
}
export function evaluateRustSourceConstantPattern(parser,entry,where){
 const pending=[entry.fullName],records=new Map(),ownersBySpan=new Map();
 for(let at=0;at<pending.length;at+=1){
  const owner=pending[at];if(records.has(owner))continue;
  const row=parser.sourceConstantInitializers.get(owner)??parser.sourceConstantFunctions.get(owner);if(!row)throw unsupported('Rust source constant pattern','the actual constant initializer must be retained',where);
  const {declaration,reader,result,parameters}=parser.readSourceConstantInitializer(row),dependencies=new Set();
  function project(value,locals=new Set()){
   if(!value||typeof value!=='object')return value;if(Array.isArray(value))return value.map(item=>project(item,locals));
   const node=Object.fromEntries(Object.entries(value).map(([key,item])=>[key,key==='body'&&value.k==='let'?project(item,new Set([...locals,value.name])):project(item,locals)]));
   if(value.k==='name'&&!(value.path.length===1&&locals.has(value.path[0]))){const found=reader.origins.lookup(readRustSourceValueName(reader,value.path,row.path),row.path);if(found?.k==='const'||found?.k==='fn'){if(found.k==='fn'&&!reader.sourceConstantFunctions.get(found.fullName)?.isConst)throw typeError('Rust source constant pattern: a constant function call needs its actual const declaration',value.span);node.path=['crate',...found.fullName.split('.')];dependencies.add(found.fullName);}}
   if(value.k==='num'||value.k==='cast'){const origin=reader.origins.origin(value,row.path),integer=readRustPointerIntegerContractOwner(origin.expandedSourceOwner??origin.sourceOwner,origin.integerSourceOwnerConstraint??'');if(rustFixedType(integer)&&value.span)ownersBySpan.set(value.span.start+':'+value.span.end,integer);}
   return node;
  }
  records.set(owner,{owner,kind:row.kind??'constant',isConst:row.kind==='function'&&row.isConst,parameterDomains:parameters.map(constantDomain),domain:constantDomain(result),declaration:{...declaration,name:owner,body:project(declaration.body,new Set(declaration.params.map(parameter=>parameter.name)))}});for(const dependency of dependencies)if(!records.has(dependency))pending.push(dependency);
  if(records.size>65536)throw unsupported('Rust source constant pattern','constant pattern evaluation exceeds its source resource budget',where);
 }
 const checked=checkProgram({language:'Rust',items:[...records.values()].map(row=>row.declaration),main:null});
 const packet={constants:[...records.values()].map(row=>{const declaration=checked.declarations.get(row.owner);if(declaration.params.length!==row.parameterDomains.length)throw typeError('a constant declaration must retain every actual parameter source owner',where);return{owner:row.owner,kind:row.kind,const:row.isConst,domain:row.domain,parameters:declaration.params.map((parameter,index)=>({name:parameter.name,domain:row.parameterDomains[index]})),body:declaration.body};})};
 function retain(value){if(!value||typeof value!=='object')return;if(value.span){const owner=ownersBySpan.get(value.span.start+':'+value.span.end);if(owner)value.sourceIntegerOwner=owner;}for(const item of Object.values(value))retain(item);}
 retain(packet);const facts=createSurfaceReferenceFacts(packet),result=evaluateRustSourceConstant(facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,facts.root,entry.fullName);
 if(result[0]==='error')throw typeError('Rust source constant pattern: '+result[1],where);
 if(result[0]==='integer'){const type=rustFixedType(result[1]);if(!type)throw typeError('a constant pattern lost its actual integer source owner',where);return{k:'typedNumLit',owner:result[1],type,value:result[2].startsWith('-')?result[2].slice(1):result[2],negative:result[2].startsWith('-'),span:where};}
 if(result[0]==='unit')return{k:'tuple',args:[],span:where};
 if(result[0]==='boolean')return{k:'boolLit',value:result[2]==='true',span:where};
 if(result[0]==='string')return{k:'strLit',value:result[2],span:where};
 throw unsupported('Rust source constant pattern','the value needs its structural source pattern interpretation',where);
}
export function readRustSourceConstantPattern(parser,path){
 if(!parser.ordinaryCompletionContext())return null;
 const c=parser.cursor,start=c.peek(),segments=[];let offset=0;
 while(c.peek(offset).kind==='identifier'){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;if(c.peek(offset+1).kind!=='identifier')return null;offset+=1;}
 if(!segments.length||c.is('@',offset))return null;
 const entry=parser.origins.lookup(readRustSourceValueName(parser,segments,path),path);if(entry?.k!=='const')return null;
 const where={start:start.start,end:c.peek(offset-1).end},pattern=evaluateRustSourceConstantPattern(parser,entry,where);for(let at=0;at<offset;at+=1)c.next();return pattern;
}
