// Actual source types/scopes into the existing shared callable representation.
import {unsupported} from './diagnostics.js';
import {readCallableStatementCaptures} from './callable-source.js';
import {readRustOrdinaryFunctionCompletion} from './rust-ordinary-completions.js';
import {readRustCopyClosureHeaderError,readRustCopyClosureCaptureError,acceptRustCopyClosureInvocation} from './rust-copy-closure-rules.js';
export function readRustCopyClosure(parser,path){
 const c=parser.cursor,start=c.peek(),parameters=[],origins=[];c.eat('move');
 if(!c.eat('||')){c.expect('|','closure');while(!c.is('|')){const token=c.peek();if(c.is('mut')||c.is('ref')||c.is('&')||c.is('_'))throw unsupported('Rust closure parameter','a closure needs a named direct immutable typed parameter',token);const name=c.identifier('closure parameter').value;c.expect(':','closure parameter');const type=parser.type();parameters.push({name,type,span:{start:token.start,end:c.peek(-1).end}});origins.push(parser.origins.type);if(!c.eat(','))break;}c.expect('|','closure');}
 const written=Boolean(c.eat('->'));if(!written)throw unsupported('Rust Copy closure','the closure result requires its actual explicit type',start);
 const ret=parser.type(),resultOrigin=parser.origins.type,reason=readRustCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>parser.origins.immutableProduct(origin)),written,parser.origins.immutableProduct(resultOrigin));
 if(reason)throw unsupported('Rust Copy closure',reason,start);
 const outer=parser.origins.enter();parameters.forEach((parameter,index)=>parser.origins.scope.set(parameter.name,origins[index]));
 let body;
 try{body=parser.ordinaryCompletionSelected()?readRustOrdinaryFunctionCompletion(parser,path,ret,resultOrigin):parser.block(path,true);parser.origins.argument(parser.origins.origin(body,path),resultOrigin,body.span);}
 finally{parser.origins.scope=outer;}
 const {captures,assignedCaptures}=readCallableStatementCaptures([{s:'return',expr:body}],parameters.map(param=>param.name),outer.keys());
 const captureError=readRustCopyClosureCaptureError(parser.origins.collecting,captures.map(name=>parser.origins.immutableProduct(outer.get(name))),assignedCaptures.length);if(captureError)throw unsupported('Rust Copy closure',captureError,start);
 const span={start:start.start,end:c.peek(-1).end},node={k:'lambda',params:parameters,ret,captures,body,span};
 return parser.origins.mark(node,{...parser.origins.typeOrigin({kind:'function',params:parameters.map(param=>param.type),ret},path),copyClosureSpan:span});
}
export function readRustCopyClosureCall(parser,fn,args,path,span){
 const origin=parser.origins.origin(fn,path);if(!acceptRustCopyClosureInvocation(origin.type?.kind??''))return null;
 for(const arg of args)parser.origins.direct(arg,path,arg.span??span);
 return parser.origins.mark({k:'app',fn,args,span},parser.origins.typeOrigin(origin.type.ret,path));
}
