import {acceptRustPrimitiveFromMethod,readRustPrimitiveFromOwner} from './rust-primitive-from-rules.js';
import {projectRustPrimitiveFrom} from './rust-primitive-from.js';
// Token/type/origin transport only; all operation semantics use shared rules.
import {TranslationError,unsupported} from './diagnostics.js';
import {rustFixedType} from './types.js';
import {readRustPrimitiveAssociatedOwner,readRustPrimitiveAssociatedArity,acceptRustPrimitiveAssociatedLiteral,acceptRustPrimitiveAssociatedReceiver} from './rust-primitive-associated-call-rules.js';
import {readRustPrimitiveSelection,readRustSignedIntegerProperty,readRustSaturatingInteger,readRustSaturatingMultiplication,readRustWrappingInteger} from './rust-primitive-selections.js';
export function readRustPrimitiveAssociatedImport(parser,segments,path){
 const root=segments[0]??'',shadowed=Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root));
 return Boolean(readRustPrimitiveAssociatedOwner([...segments,'min'],parser.ordinaryCompletionContext(),shadowed,false)||readRustPrimitiveFromOwner([...segments,'from'],parser.ordinaryCompletionContext(),shadowed,false));
}
export function readRustPrimitiveAssociatedCall(parser,path){
 const c=parser.cursor,start=c.peek(),absolute=c.is('::');let offset=absolute?1:0;
 const segments=[];
 while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;if(c.peek(offset+1).kind!=='identifier')return null;offset+=1;}
 if(segments.length<2)return null;
 const resolved=absolute?segments:parser.resolve(segments,path),root=segments[0]??'',target=resolved[0]??'';
 const shadowed=!absolute&&Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root)||parser.origins.lookup([target],path)||parser.origins.scope.has(target));
 const owner=(resolved.at(-1)==='from'?readRustPrimitiveFromOwner(resolved,parser.ordinaryCompletionContext(),shadowed,absolute):readRustPrimitiveAssociatedOwner(resolved,parser.ordinaryCompletionContext(),shadowed,absolute)),method=resolved.at(-1)??'',conversion=acceptRustPrimitiveFromMethod(owner,method),arity=conversion?1:readRustPrimitiveAssociatedArity(owner,method);
 if(arity<0)return null;
 if(!c.is('(',offset))throw unsupported('Rust primitive associated call','a primitive associated function value needs its own callable interpretation',{start:start.start,end:c.peek(offset).end});
 for(let index=0;index<=offset;index+=1)c.next();
 const values=[];while(!c.is(')')){values.push(parser.expr(path));if(!c.eat(','))break;}
 const close=c.expect(')','primitive associated call'),range={start:start.start,end:close.end};
 if(values.length!==arity)throw new TranslationError('type',`primitive associated ${method} expects ${arity} arguments but got ${values.length}`,range);
 if(parser.origins.collecting)return{k:'app',fn:{k:'name',path:resolved,span:range},args:values,span:range};
 if(conversion)return projectRustPrimitiveFrom(parser,values[0],owner,range,path);
 let receiver=values[0];const negative=receiver.k==='unary'&&receiver.op==='neg'&&receiver.arg.k==='num',number=negative?receiver.arg:receiver;
 if(acceptRustPrimitiveAssociatedLiteral(owner,negative?'negative numeral':receiver.k==='num'?'numeral':'',Boolean(number.type))){
  const type=rustFixedType(owner),literal={...number,type};receiver=negative?{...receiver,arg:literal}:literal;
  parser.origins.mark(receiver,{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]}});
 }
 const origin=parser.origins.origin(receiver,path),actual=origin.sourceOwner??(receiver.k==='bool'?{kind:'primitive',path:['bool'],arguments:[]}:null),type=origin.type;
 if(!acceptRustPrimitiveAssociatedReceiver(owner,type?.kind??'',String(type?.bits??''),type?.signed===true,actual?.kind??'',actual?.path.join('::')??'',actual?.arguments.length??-1,origin.proven===true,origin.wrappers.length))throw unsupported('Rust primitive associated call','the explicit receiver must retain the selected standard primitive type and source owner',range);
 const token={...start,value:method},args=values.slice(1);
 for(const read of [readRustPrimitiveSelection,readRustSignedIntegerProperty,readRustSaturatingInteger,readRustSaturatingMultiplication,readRustWrappingInteger]){const value=read(parser,receiver,token,args,range,path);if(value)return value;}
 throw new TypeError('the registered primitive associated call lost its shared method interpretation');
}
