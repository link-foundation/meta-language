import {readRustWrappingIntegerArity,acceptRustWrappingIntegerReceiver,planRustWrappingInteger} from './rust-wrapping-integer-rules.js';
import {acceptRustSaturatingMultiplicationMethod,acceptRustSaturatingMultiplicationReceiver,planRustSaturatingMultiplication} from './rust-saturating-multiplication-rules.js';
import {readRustSaturatingIntegerArity,acceptRustSaturatingIntegerReceiver,readRustSaturatingIntegerBoundMagnitude,acceptRustSaturatingIntegerBoundNegative,planRustSaturatingInteger} from './rust-saturating-integer-rules.js';
import {acceptRustSignedIntegerPropertyMethod,acceptRustSignedIntegerPropertyReceiver,planRustSignedIntegerProperty,readRustSignedIntegerPropertyResult} from './rust-signed-integer-property-rules.js';
// Actual source values/origins and existing conditional AST projection only.
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustPrimitiveSelectionCallRole,readRustPrimitiveSelectionComparison,readRustPrimitiveSelectionCondition,acceptRustPrimitiveSelectionReceiver,acceptRustPrimitiveSelectionArgument} from './rust-primitive-selection-rules.js';
function facts(node,parser,path){
 const origin=parser.origins.origin(node,path);
 const owner=origin.sourceOwner??(node.k==='bool'?{kind:'primitive',path:['bool'],arguments:[]}:null);
 const type=origin.type;
 return {origin,owner,kind:type?.kind??'',bits:String(type?.bits??''),signed:type?.signed===true};
}
function admitted(value){return acceptRustPrimitiveSelectionReceiver(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length);}
export function readRustPrimitiveSelectionImport(parser,segments,path){
 const shadowed=Boolean(parser.origins.lookup(['std'],path)||parser.origins.scope.has('std')||[...parser.aliases.values()].some(aliases=>aliases.has('std')));
 return readRustPrimitiveSelectionCallRole(segments.join('::'),parser.ordinaryCompletionContext(),shadowed);
}
export function readRustPrimitiveSelectionCall(parser,path){
 const c=parser.cursor,start=c.peek();let offset=c.is('::')?1:0;
 const first=c.peek(offset),aliases=parser.aliases.get(path.join('.'));
 if(first.kind!=='identifier'||c.is('::')&&first.value!=='std'||first.value!=='std'&&!aliases?.has(first.value)||parser.origins.scope.has(first.value))return null;
 const segments=[];while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;offset+=1;}
 const role=readRustPrimitiveSelectionImport(parser,parser.resolve(segments,path),path);
 if(!role)return null;
 if(!c.is('(',offset))throw unsupported('Rust primitive selection call','a standard comparison function value needs its own callable interpretation',{start:start.start,end:c.peek(offset).end});
 for(let index=0;index<=offset;index+=1)c.next();
 const args=[];while(!c.is(')')){args.push(parser.expr(path));if(!c.eat(','))break;}
 const close=c.expect(')','primitive selection call'),range={start:start.start,end:close.end};
 if(args.length!==2)throw new TranslationError('type','a standard primitive selection call needs two actual operands',range);
 return readRustPrimitiveSelection(parser,args[0],{...start,value:role},[args[1]],range,path);
}
export function readRustPrimitiveSelection(parser,receiver,method,args,range,path){
 const comparison=readRustPrimitiveSelectionComparison(method.value,parser.ordinaryCompletionContext());
 if(!comparison)return null;
 if(args.length!==1)throw new TranslationError('type',`${method.value} expects 1 arguments but got ${args.length}`,range);
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args],span:range};
 const left=facts(receiver,parser,path),right=facts(args[0],parser,path);
 if(!admitted(left))throw unsupported('Rust primitive selection','a proven direct standard fixed integer or Boolean receiver is required',range);
 const literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;
 if(!acceptRustPrimitiveSelectionArgument(left.kind,left.bits,left.signed,left.owner.path.join('::'),right.kind,right.bits,right.signed,right.owner?.path.join('::')??'',right.origin.wrappers.length,right.origin.proven===true,admitted(right),Boolean(literal)))throw unsupported('Rust primitive selection','the argument must retain the actual receiver type and primitive source owner',range);
 const first=parser.freshProductBinding(),second=parser.freshProductBinding();
 const origin={...left.origin,path:[...path],sourceOwner:left.owner};
 const local=name=>parser.origins.mark({k:'name',path:[name],span:range},origin);
 const one=local(first),two=local(second);
 const condition=readRustPrimitiveSelectionCondition(method.value,left.kind);
 const cond=condition==='identity'?one:condition==='not'?{k:'unary',op:'not',arg:one,span:range}:{k:'binary',op:condition,left:one,right:two,span:range};
 const body={k:'if',cond,then:local(first),else:local(second),span:range};
 return parser.origins.mark({k:'let',name:first,type:left.origin.type,value:receiver,body:{k:'let',name:second,type:left.origin.type,value:args[0],body,span:range},span:range},origin);
}

/** Project the shared property plan into the existing source AST algebra. */
export function readRustSignedIntegerProperty(parser,receiver,method,args,range,path){
 if(!acceptRustSignedIntegerPropertyMethod(method.value,parser.ordinaryCompletionContext()))return null;
 if(args.length!==0)throw new TranslationError('type',`${method.value} expects 0 arguments but got ${args.length}`,range);
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver],span:range};
 const value=facts(receiver,parser,path);
 if(!acceptRustSignedIntegerPropertyReceiver(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length))throw unsupported('Rust signed integer property','a proven direct standard signed integer receiver is required',range);
 const origin={...value.origin,path:[...path],sourceOwner:value.owner};
 const result=projectRustPrimitiveIntegerPlan(parser,receiver,[],planRustSignedIntegerProperty(method.value),value,range,path);
 const resultOrigin=readRustSignedIntegerPropertyResult(method.value)==='receiver'?origin:{...parser.origins.typeOrigin({kind:'bool'},path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 return parser.origins.mark(result,resultOrigin);
}

/** Generic source-term projection shared by exact primitive integer plans. */
function projectRustPrimitiveIntegerPlan(parser,receiver,args,plan,value,range,path){
 const origin={...value.origin,path:[...path],sourceOwner:value.owner},stack=[];
 const captures=[receiver,...args].map(input=>({name:parser.freshProductBinding(),value:input}));
 const literal=magnitude=>({k:'num',value:magnitude,type:value.origin.type,span:range});
 const pop=()=>{if(!stack.length)throw new TypeError('the shared primitive source plan lost an actual operand');return stack.pop();};
 for(const term of plan){
  if(term==='operand'||term==='right'){
   const binding=captures[term==='operand'?0:1];
   if(!binding)throw new TypeError('the shared primitive source plan lost an actual capture');
   stack.push(parser.origins.mark({k:'name',path:[binding.name],span:binding.value.span??range},origin));
  }else if(term==='zero'||term==='positive unit')stack.push(literal(term==='zero'?'0':'1'));
  else if(term==='negative unit')stack.push({k:'unary',op:'neg',arg:literal('1'),span:range});
  else if(term==='minimum'||term==='maximum'){
   const magnitude=readRustSaturatingIntegerBoundMagnitude(term,Number(value.bits),value.signed);
   if(!magnitude)throw new TypeError('the shared primitive source plan lost its exact bound');
   const node=literal(magnitude);stack.push(acceptRustSaturatingIntegerBoundNegative(term,value.signed)?{k:'unary',op:'neg',arg:node,span:range}:node);
  }else if(['lt','gt','eq','add','sub','mul','div'].includes(term)){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}
  else if(term==='neg')stack.push({k:'unary',op:'neg',arg:pop(),span:range});
  else if(term==='if'){const otherwise=pop(),then=pop(),cond=pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}
  else throw new TypeError('unknown shared primitive source plan term');
 }
 if(stack.length!==1)throw new TypeError('the shared primitive source plan needs one complete result');
 let body=stack[0];for(let at=captures.length-1;at>=0;at-=1){const binding=captures[at];body={k:'let',name:binding.name,type:value.origin.type,value:binding.value,body,span:range};}
 return body;
}
export function readRustSaturatingInteger(parser,receiver,method,args,range,path){
 const arity=readRustSaturatingIntegerArity(method.value,parser.ordinaryCompletionContext());
 if(arity<0)return null;
 if(args.length!==arity)throw new TranslationError('type',`${method.value} expects ${arity} arguments but got ${args.length}`,range);
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args],span:range};
 const value=facts(receiver,parser,path);
 if(!acceptRustSaturatingIntegerReceiver(method.value,value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length))throw unsupported('Rust saturating integer arithmetic','a proven direct standard fixed integer receiver with the actual signed operation is required',range);
 if(arity===1){
  const right=facts(args[0],parser,path),literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;
  if(!acceptRustPrimitiveSelectionArgument(value.kind,value.bits,value.signed,value.owner.path.join('::'),right.kind,right.bits,right.signed,right.owner?.path.join('::')??'',right.origin.wrappers.length,right.origin.proven===true,admitted(right),Boolean(literal)))throw unsupported('Rust saturating integer arithmetic','the argument must retain the actual receiver type and primitive source owner',range);
 }
 const result=projectRustPrimitiveIntegerPlan(parser,receiver,args,planRustSaturatingInteger(method.value,value.signed),value,range,path);
 return parser.origins.mark(result,{...value.origin,path:[...path],sourceOwner:value.owner});
}

export function readRustSaturatingMultiplication(parser,receiver,method,args,range,path){
 if(!acceptRustSaturatingMultiplicationMethod(method.value,parser.ordinaryCompletionContext()))return null;
 if(args.length!==1)throw new TranslationError('type',`saturating_mul expects 1 arguments but got ${args.length}`,range);
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args],span:range};
 const value=facts(receiver,parser,path),right=facts(args[0],parser,path);
 if(!acceptRustSaturatingMultiplicationReceiver(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length))throw unsupported('Rust saturating multiplication','a proven direct standard fixed integer receiver is required',range);
 const literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;
 if(!acceptRustPrimitiveSelectionArgument(value.kind,value.bits,value.signed,value.owner.path.join('::'),right.kind,right.bits,right.signed,right.owner?.path.join('::')??'',right.origin.wrappers.length,right.origin.proven===true,admitted(right),Boolean(literal)))throw unsupported('Rust saturating multiplication','the argument must retain the actual receiver type and primitive source owner',range);
 const result=projectRustPrimitiveIntegerPlan(parser,receiver,args,planRustSaturatingMultiplication(value.signed),value,range,path);
 return parser.origins.mark(result,{...value.origin,path:[...path],sourceOwner:value.owner});
}

export function readRustWrappingInteger(parser,receiver,method,args,range,path){
 const arity=readRustWrappingIntegerArity(method.value,parser.ordinaryCompletionContext());
 if(arity<0)return null;
 if(args.length!==arity)throw new TranslationError('type',`${method.value} expects ${arity} arguments but got ${args.length}`,range);
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args],span:range};
 const value=facts(receiver,parser,path);
 if(!acceptRustWrappingIntegerReceiver(method.value,value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length))throw unsupported('Rust wrapping integer arithmetic','a proven direct standard fixed integer receiver with the actual signed operation is required',range);
 if(arity===1){
  const right=facts(args[0],parser,path),literal=args[0].k==='num'&&!args[0].type||args[0].k==='unary'&&args[0].op==='neg'&&args[0].arg.k==='num'&&!args[0].arg.type;
  if(!acceptRustPrimitiveSelectionArgument(value.kind,value.bits,value.signed,value.owner.path.join('::'),right.kind,right.bits,right.signed,right.owner?.path.join('::')??'',right.origin.wrappers.length,right.origin.proven===true,admitted(right),Boolean(literal)))throw unsupported('Rust wrapping integer arithmetic','the argument must retain the actual receiver type and primitive source owner',range);
 }
 const result=projectRustPrimitiveIntegerPlan(parser,receiver,args,planRustWrappingInteger(method.value,value.signed),value,range,path);
 return parser.origins.mark(result,{...value.origin,path:[...path],sourceOwner:value.owner});
}
