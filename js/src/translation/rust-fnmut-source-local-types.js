// A real local Vec<_> annotation is solved from its fully admitted initializer;
// no placeholder datatype or invented element producer enters the core graph.
import {unsupported} from './diagnostics.js';
const hints=new WeakMap();
const fail=(reason,span)=>{throw unsupported('Rust source FnMut local type inference',reason,span);};
export function readRustFnMutSourceLocalVectorAnnotation(parser,path){
 if(!parser.fnMutSourceSpecialization?.runtime||!parser.cursor.is(':'))return null;
 const c=parser.cursor,segments=[];let at=1;if(c.peek(at).kind!=='identifier')return null;
 for(;;){segments.push(c.peek(at++).value);if(c.is('::',at)&&c.peek(at+1).kind==='identifier')at++;else break;}
 if(!c.is('<',at)||!c.is('_',at+1)||!c.is('>',at+2)||!c.is('=',at+3))return null;
 const resolved=parser.resolve(segments,path),canonical=resolved.join('::');if(!['Vec','std::vec::Vec','alloc::vec::Vec'].includes(canonical)||parser.origins.lookupTypeOwner(segments,path))return null;
 const span={start:c.peek().start,end:c.peek(at+2).end};parser.origins.vector(segments,resolved,path,span);for(let index=0;index<at+3;index++)c.next();
 const handle=Object.freeze({start:span.start});hints.set(handle,{parser,context:parser.fnMutSourceSpecialization,span});return handle;
}
export function finishRustFnMutSourceLocalVectorAnnotation(parser,handle,value,path){
 const hint=hints.get(handle);if(!hint||hint.parser!==parser||hint.context!==parser.fnMutSourceSpecialization)fail('a local inference hint retains its actual source annotation and specialization');
 const origin=parser.origins.origin(value,path),owner=origin.expandedSourceOwner??origin.sourceOwner;
 if(!origin.proven||origin.type?.kind!=='array'||origin.wrappers.length||origin.arraySourceModel!=='owned vector'||owner?.kind!=='library'||!['std::vec::Vec','alloc::vec::Vec'].includes(owner.path.join('::'))||owner.arguments.length!==1)fail('Vec<_> requires its actual complete direct owned Vec initializer',hint.span);
 hints.delete(handle);return {type:structuredClone(origin.type),origin:structuredClone(origin)};
}
