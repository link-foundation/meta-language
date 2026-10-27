// Source paths/types and exact shared bound magnitudes into existing ASTs.
import {unsupported} from './diagnostics.js';
import {rustFixedType} from './types.js';
import {readRustPrimitiveIntegerConstantOwner,readRustPrimitiveIntegerConstantTerm,readRustPrimitiveIntegerConstantResultOwner} from './rust-primitive-integer-constant-rules.js';
import {readRustSaturatingIntegerBoundMagnitude,acceptRustSaturatingIntegerBoundNegative} from './rust-saturating-integer-rules.js';
export function readRustPrimitiveIntegerConstant(parser,path){
 const c=parser.cursor,start=c.peek(),absolute=c.is('::');let offset=absolute?1:0;const segments=[];
 while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;if(c.peek(offset+1).kind!=='identifier')return null;offset+=1;}
 if(segments.length<2)return null;
 const resolved=absolute?segments:parser.resolve(segments,path),root=segments[0],target=resolved[0];
 const shadowed=!absolute&&Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root)||parser.origins.lookup([target],path)||parser.origins.scope.has(target));
 const owner=readRustPrimitiveIntegerConstantOwner(resolved,parser.ordinaryCompletionContext(),shadowed,absolute),term=readRustPrimitiveIntegerConstantTerm(resolved.at(-1)??'');
 if(!owner||!term)return null;
 const range={start:start.start,end:c.peek(offset-1).end};
 if(c.is('(',offset))throw unsupported('Rust primitive integer constant','an associated integer constant requires value syntax rather than a call',range);
 for(let index=0;index<offset;index+=1)c.next();
 const source=rustFixedType(owner),resultOwner=readRustPrimitiveIntegerConstantResultOwner(owner,term),type=rustFixedType(resultOwner),magnitude=term==='width'?String(source.bits):readRustSaturatingIntegerBoundMagnitude(term,source.bits,source.signed);
 if(!magnitude||!type)throw new TypeError('the shared integer constant lost its actual primitive bound or result type');
 const literal={k:'num',value:magnitude,type,span:range},node=term!=='width'&&acceptRustSaturatingIntegerBoundNegative(term,source.signed)?{k:'unary',op:'neg',arg:literal,span:range}:literal;
 return parser.origins.mark(node,{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[resultOwner],arguments:[]}});
}
