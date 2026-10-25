// Actual source types/scopes into the existing shared callable representation.
import {unsupported} from './diagnostics.js';
import {readCallableStatementCaptures} from './callable-source.js';
import {readRustOrdinaryFunctionCompletion} from './rust-ordinary-completions.js';
import {readRustCopyClosureHeaderError,readRustCopyClosureCaptureError,acceptRustCopyClosureInvocation} from './rust-copy-closure-rules.js';
import {acceptRustContextualCopyClosure,readRustContextualCopyClosureHeaderError} from './rust-contextual-closure-rules.js';
export function readRustExpectedCopyClosure(parser,path,expected){
 return acceptRustContextualCopyClosure(expected?.type?.kind??'',parser.origins.immutableProduct(expected),parser.cursor.peek().value)?readRustCopyClosure(parser,path,expected):parser.expr(path);
}
export function readRustCopyClosure(parser,path,expected=null){
 const c=parser.cursor,start=c.peek(),parameters=[],origins=[];c.eat('move');
 if(!c.eat('||')){c.expect('|','closure');while(!c.is('|')){const token=c.peek();if(c.is('mut')||c.is('ref')||c.is('&')||c.is('_'))throw unsupported('Rust closure parameter','a closure needs a named direct immutable typed parameter',token);const name=c.identifier('closure parameter').value;let type,origin;if(c.eat(':')){type=parser.type();origin=parser.origins.type;}else{type=expected?.type?.params[parameters.length];if(!type){c.expect(':','closure parameter');throw parser.fail('a closure parameter needs its source type');}origin=parser.origins.typeOrigin(type,path);}parameters.push({name,type,span:{start:token.start,end:c.peek(-1).end}});origins.push(origin);if(!c.eat(','))break;}c.expect('|','closure');}
 const written=Boolean(c.eat('->'));let ret=written?parser.type():expected?.type?.ret??null,resultOrigin=written?parser.origins.type:ret?parser.origins.typeOrigin(ret,path):null;
 if(written){const reason=readRustCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>parser.origins.immutableProduct(origin)),written,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
 const outer=parser.origins.enter();parameters.forEach((parameter,index)=>parser.origins.scope.set(parameter.name,origins[index]));
 let body;
 try{
  if(written||c.is('{'))body=ret&&parser.ordinaryCompletionSelected()?readRustOrdinaryFunctionCompletion(parser,path,ret,resultOrigin):parser.block(path,true);else body=parser.expr(path);
  if(!ret){resultOrigin=parser.origins.origin(body,path);ret=resultOrigin.type??null;}
  if(!written){const reason=readRustContextualCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>parser.origins.immutableProduct(origin)),ret!==null,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
  parser.origins.argument(parser.origins.origin(body,path),resultOrigin,body.span);
 }
 finally{parser.origins.scope=outer;}
 const {captures,assignedCaptures}=readCallableStatementCaptures([{s:'return',expr:body}],parameters.map(param=>param.name),outer.keys());
 const captureError=readRustCopyClosureCaptureError(parser.origins.collecting,captures.map(name=>parser.origins.immutableProduct(outer.get(name))),assignedCaptures.length);if(captureError)throw unsupported('Rust Copy closure',captureError,start);
 const span={start:start.start,end:c.peek(-1).end},node={k:'lambda',params:parameters,ret,captures,body,span};
 return parser.origins.mark(node,{...parser.origins.typeOrigin({kind:'function',params:parameters.map(param=>param.type),ret},path),copyClosureSpan:span,copyClosureCaptureCount:captures.length});
}
export function readRustCopyClosureCall(parser,fn,args,path,span){
 const origin=parser.origins.origin(fn,path);if(!acceptRustCopyClosureInvocation(origin.type?.kind??''))return null;
 args.forEach((arg,index)=>{const parameter=origin.type.params[index];if(parameter?.kind==='function')parser.origins.argument(parser.origins.origin(arg,path),{...parser.origins.typeOrigin(parameter,path),copyEvidence:true},arg.span??span);else parser.origins.direct(arg,path,arg.span??span);});
 const result=parser.origins.typeOrigin(origin.type.ret,path);if(origin.type.ret.kind==='function')result.copyEvidence=true;
 return parser.origins.mark({k:'app',fn,args,span},result);
}
