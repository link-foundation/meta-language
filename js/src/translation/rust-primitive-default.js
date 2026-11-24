// Actual paths and source types project the shared primitive Default terms.
import {BOOL,FLOAT,rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustPrimitiveConversionHead} from './rust-primitive-try-from.js';
import {readRustPrimitiveDefaultOwner,readRustPrimitiveDefaultTerm,acceptRustPrimitiveDefaultImport} from './rust-primitive-default-rules.js';
export function projectRustPrimitiveDefault(parser,owner,path,range){
 const term=readRustPrimitiveDefaultTerm(owner),type=term==='false'?BOOL:term==='positive floating zero'?FLOAT:term==='integer zero'?rustFixedType(owner):null;
 if(!type)throw new TypeError('the shared primitive default lost its actual source term');
 const value=term==='false'?{k:'bool',value:false,span:range}:{k:'num',value:term==='positive floating zero'?'0.0':'0',type,span:range};
 return parser.origins.mark(value,{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]},...(term==='positive floating zero'?{floatSourceModel:'binary64 scalar'}:{})});
}
export function readRustPrimitiveDefaultImport(parser,segments,path){const root=segments[0]??'',shadowed=Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root)||parser.aliases.get(path.join('.'))?.has(root));return acceptRustPrimitiveDefaultImport(segments.join('::'),parser.ordinaryCompletionContext(),shadowed);}
export function readRustPrimitiveDefault(parser,path){
 const c=parser.cursor,head=readRustPrimitiveConversionHead(parser,path);if(!head||head.resolved.at(-1)!=='default')return null;
 const aliases=parser.aliases.get(path.join('.')),trait=aliases?.get('Default'),traitConflict=Boolean(parser.origins.lookup(['Default'],path)||parser.origins.scope.has('Default')||trait&&!acceptRustPrimitiveDefaultImport(trait.join('::'),true,false));
 const owner=readRustPrimitiveDefaultOwner(head.resolved,parser.ordinaryCompletionContext(),head.shadowed||traitConflict,head.absolute);if(!owner)return null;
 const range={start:head.start.start,end:c.peek(head.offset).end};
 if(!c.is('(',head.offset))throw unsupported('Rust primitive Default','a primitive default function value needs its own callable interpretation',range);
 for(let at=0;at<=head.offset;at+=1)c.next();
 const args=[];while(!c.is(')')){args.push(parser.expr(path));if(!c.eat(','))break;}
 const close=c.expect(')','primitive default call');range.end=close.end;
 if(args.length!==0)throw new TranslationError('type','a primitive Default call needs no actual arguments',range);
 return projectRustPrimitiveDefault(parser,owner,path,range);
}
