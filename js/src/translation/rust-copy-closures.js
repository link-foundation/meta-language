import {requireRustNonconstantBinding,readRustSourceValueName} from './rust-source-constants.js';
import {beginRustStandardStringViewArgument,endRustStandardStringViewArgument,checkRustStandardStringViewArgument} from './rust-standard-string-views.js';
// Actual source types/scopes into the existing shared callable representation.
import {unsupported} from './diagnostics.js';
import {readCallableStatementCaptures} from './callable-source.js';
import {readRustOrdinaryFunctionCompletion} from './rust-ordinary-completions.js';
import {readRustCopyClosureHeaderError,readRustCopyClosureCaptureError,acceptRustCopyClosureInvocation} from './rust-copy-closure-rules.js';
import {acceptRustContextualCopyClosure,readRustContextualCopyClosureHeaderError,findRustContextualClosureParenthesisCount,acceptRustContextualCopyPointerResultBlock} from './rust-contextual-closure-rules.js';
export function readRustExpectedCopyClosure(parser,path,expected){
 const c=parser.cursor,kind=expected?.type?.kind??'',copy=parser.origins.immutableProduct(expected),role=['|','||','move'].find(value=>c.is(value))??'';
 if(acceptRustContextualCopyClosure(kind,copy,role))return readRustCopyClosure(parser,path,expected);
 if(kind==='function'&&copy&&c.is('(')){
  let count=64;
  for(;;){const tokens=c.tokens.slice(c.index,c.index+count),syntax=tokens.map((_,index)=>['(','|','||','move'].find(value=>c.is(value,index))??''),depth=findRustContextualClosureParenthesisCount(syntax);
   if(depth>0){for(let index=0;index<depth;index+=1)c.next();const node=readRustCopyClosure(parser,path,expected);for(let index=0;index<depth;index+=1)c.expect(')','contextual closure');return node;}
   if(depth!==-2||tokens.length<count)break;count*=2;
  }
 }
 return parser.expr(path);
}
export function readRustCopyPointerResultBlock(parser,path,expected){
 const c=parser.cursor,role=['|','||','move','(','return'].find(value=>c.is(value,1))??'';
 if(!parser.ordinaryCompletionContext()||!acceptRustContextualCopyPointerResultBlock(expected?.type?.kind??'',parser.origins.immutableProduct(expected),c.is('{'),role))return null;
 c.expect('{','function pointer result');const returned=Boolean(c.eat('return')),value=readRustExpectedCopyClosure(parser,path,expected);
 if(returned)while(c.eat(';')){}
 c.expect('}','function pointer result');parser.origins.argument(parser.origins.origin(value,path),expected,value.span);return value;
}
export function readRustContextualCallArguments(parser,callee,path){
 let parameters=null,result=null;
 if(callee.k==='name'&&!(callee.path.length===1&&parser.origins.scope.has(callee.path[0]))){const entry=parser.origins.lookup(readRustSourceValueName(parser,callee.path,path),path);if(entry?.k==='fn'){parameters=entry.parameters;result=entry.result;}}
 if(parameters===null){const origin=parser.origins.origin(callee,path);result??=origin.callableResult??null;parameters=origin.type?.kind==='function'?origin.type.params.map((type,index)=>origin.callableParameters?.[index]??({...parser.origins.typeOrigin(type,path),...(type.kind==='function'?{copyEvidence:true}:{})})):[];}
 const args=[],c=parser.cursor;
 while(!c.is(')')){const frame=beginRustStandardStringViewArgument(parser,path,parameters[args.length],result);let value;try{value=readRustExpectedCopyClosure(parser,path,parameters[args.length]);}finally{endRustStandardStringViewArgument(parser,frame);}args.push(checkRustStandardStringViewArgument(parser,frame,value,path));if(!c.eat(','))break;}
 return args;
}
export function readRustCopyClosure(parser,path,expected=null){
 const c=parser.cursor,start=c.peek(),parameters=[],origins=[];c.eat('move');
 if(!c.eat('||')){c.expect('|','closure');while(!c.is('|')){const token=c.peek();if(c.is('mut')||c.is('ref')||c.is('&')||c.is('_'))throw unsupported('Rust closure parameter','a closure needs a named direct immutable typed parameter',token);const name=c.identifier('closure parameter').value;requireRustNonconstantBinding(parser,name,path,token);let type,origin;if(c.eat(':')){type=parser.type();origin=parser.origins.type;}else{type=expected?.type?.params[parameters.length];if(!type){c.expect(':','closure parameter');throw parser.fail('a closure parameter needs its source type');}origin=parser.origins.copy(expected?.callableParameters?.[parameters.length])??parser.origins.typeOrigin(type,path);}parameters.push({name,type,span:{start:token.start,end:c.peek(-1).end}});origins.push(origin);if(!c.eat(','))break;}c.expect('|','closure');}
 const written=Boolean(c.eat('->'));let ret=written?parser.type():expected?.type?.ret??null,resultOrigin=written?parser.origins.type:ret?parser.origins.copy(expected?.callableResult)??parser.origins.typeOrigin(ret,path):null;
 if(written){const reason=readRustCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>parser.origins.immutableProduct(origin)),written,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
 const outer=parser.origins.enter();parameters.forEach((parameter,index)=>parser.origins.scope.set(parameter.name,origins[index]));
 let body;
 try{
  if(written||c.is('{'))body=readRustCopyPointerResultBlock(parser,path,resultOrigin)??(ret&&parser.ordinaryCompletionSelected()?readRustOrdinaryFunctionCompletion(parser,path,ret,resultOrigin):parser.block(path,true));else body=parser.expr(path);
  if(!ret){resultOrigin=parser.origins.origin(body,path);ret=resultOrigin.type??null;}
  if(!written){const reason=readRustContextualCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>parser.origins.immutableProduct(origin)),ret!==null,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
  parser.origins.argument(parser.origins.origin(body,path),resultOrigin,body.span);
 }
 finally{parser.origins.scope=outer;}
 const {captures,assignedCaptures}=readCallableStatementCaptures([{s:'return',expr:body}],parameters.map(param=>param.name),outer.keys());
 const captureError=readRustCopyClosureCaptureError(parser.origins.collecting,captures.map(name=>parser.origins.immutableProduct(outer.get(name))),assignedCaptures.length);if(captureError)throw unsupported('Rust Copy closure',captureError,start);
 const span={start:start.start,end:c.peek(-1).end},node={k:'lambda',params:parameters,ret,captures,body,span};
 return parser.origins.mark(node,{...parser.origins.typeOrigin({kind:'function',params:parameters.map(param=>param.type),ret},path),copyClosureSpan:span,copyClosureCaptureCount:captures.length,callableParameters:origins.map(origin=>parser.origins.copy(origin)),callableResult:parser.origins.copy(resultOrigin)});
}
export function readRustCopyClosureCall(parser,fn,args,path,span){
 const origin=parser.origins.origin(fn,path);if(!acceptRustCopyClosureInvocation(origin.type?.kind??''))return null;
 args.forEach((arg,index)=>{const parameter=origin.type.params[index];if(parameter?.kind==='function')parser.origins.argument(parser.origins.origin(arg,path),{...(parser.origins.copy(origin.callableParameters?.[index])??parser.origins.typeOrigin(parameter,path)),copyEvidence:true},arg.span??span);else if(origin.callableParameters?.[index])parser.origins.argument(parser.origins.origin(arg,path),origin.callableParameters[index],arg.span??span);else parser.origins.direct(arg,path,arg.span??span);});
 const result=parser.origins.copy(origin.callableResult)??parser.origins.typeOrigin(origin.type.ret,path);if(origin.type.ret.kind==='function')result.copyEvidence=true;
 return parser.origins.mark({k:'app',fn,args,span},result);
}
