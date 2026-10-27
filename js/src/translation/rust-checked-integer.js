// Actual tokens, source types and origin transport; algorithms are shared data.
import {BOOL,FLOAT,rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustCheckedIntegerArity,acceptRustCheckedIntegerReceiver,readRustCheckedIntegerConsumerArity,planRustCheckedIntegerCondition,planRustCheckedIntegerSuccess,planRustCheckedIntegerConsumer,readRustCheckedIntegerResultOwner} from './rust-checked-integer-rules.js';
import {acceptRustPrimitiveInherentCopy} from './rust-primitive-inherent-copy-rules.js';
import {acceptRustPrimitiveSelectionArgument} from './rust-primitive-selection-rules.js';
import {readRustPrimitiveFromOwner,acceptRustPrimitiveFromSource,planRustPrimitiveFrom} from './rust-primitive-from-rules.js';
import {readRustPrimitiveConversionHead,readRustPrimitiveConversionFacts} from './rust-primitive-try-from.js';
import {projectRustPrimitiveFrom} from './rust-primitive-from.js';
import {readRustSaturatingIntegerBoundMagnitude,acceptRustSaturatingIntegerBoundNegative} from './rust-saturating-integer-rules.js';
function admitted(value){return acceptRustPrimitiveFromSource(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length);}
function literal(node){return node.k==='num'&&!node.type||node.k==='unary'&&node.op==='neg'&&node.arg.k==='num'&&!node.arg.type;}
export function readRustCheckedInteger(parser,receiver,method,args,range,path){
 const arity=readRustCheckedIntegerArity(method.value,parser.ordinaryCompletionContext());if(arity<0)return null;
 if(args.length!==arity)throw new TranslationError('type',method.value+' needs '+arity+' actual source arguments',range);
 const c=parser.cursor;
 if(!c.eat('.')||c.peek().kind!=='identifier')throw unsupported('Rust checked integer Option','the actual Option needs an immediate admitted standard consumer',range);
 const consumer=c.next(),consumerArity=readRustCheckedIntegerConsumerArity(consumer.value);
 if(consumerArity<0)throw unsupported('Rust checked integer Option','the actual Option needs an immediate admitted standard consumer',range);
 c.expect('(','checked integer consumer');const defaults=[];let mapperOwner='';
 if(consumerArity>0){defaults.push(parser.expr(path));if(consumerArity===2){c.expect(',','checked integer mapper');const mapper=readRustPrimitiveConversionHead(parser,path);if(!mapper||mapper.resolved.at(-1)!=='from')throw unsupported('Rust checked integer mapper','an actual admitted standard primitive From function item is required',range);mapperOwner=readRustPrimitiveFromOwner(mapper.resolved,parser.ordinaryCompletionContext(),mapper.shadowed,mapper.absolute);if(!mapperOwner)throw unsupported('Rust checked integer mapper','an actual admitted standard primitive From function item is required',range);for(let at=0;at<mapper.offset;at+=1)c.next();}}
 if(consumerArity>0)c.eat(',');const close=c.expect(')','checked integer consumer');range={start:receiver.span?.start??range.start,end:close.end};
 if(parser.origins.collecting)return {k:'app',fn:{k:'name',path:[method.value],span:range},args:[receiver,...args,...defaults],span:range};
 const source=readRustPrimitiveConversionFacts(parser,receiver,path),old=source.origin;
 if(acceptRustPrimitiveInherentCopy(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',source.owner?.path.join('::')??'',source.owner?.arguments.length??-1,old.proven===true,old.wrappers.length,old.wrappers.every(layer=>layer==='reference'),old.copyEvidence===true))source.origin={...old,wrappers:[],sourceLifetimes:[],copyEvidence:true};
 const owner=source.owner?.path.join('::')??'';
 if(!acceptRustCheckedIntegerReceiver(method.value,source.kind,source.bits,source.signed,source.owner?.kind??'',owner,source.owner?.arguments.length??-1,source.origin.proven===true,source.origin.wrappers.length))throw unsupported('Rust checked integer arithmetic','a proven standard integer receiver with the actual signed operation is required',range);
 if(arity>0){const right=readRustPrimitiveConversionFacts(parser,args[0],path);if(!acceptRustPrimitiveSelectionArgument(source.kind,source.bits,source.signed,owner,right.kind,right.bits,right.signed,right.owner?.path.join('::')??'',right.origin.wrappers.length,right.origin.proven===true,admitted(right),Boolean(literal(args[0]))))throw unsupported('Rust checked integer argument','the actual argument must retain the receiver type and source owner',range);}
 if(consumer.value==='map_or'&&!planRustPrimitiveFrom(mapperOwner,owner))throw unsupported('Rust checked integer mapper','the actual standard primitive From implementation must exist for the successful value',range);
 const resultOwner=readRustCheckedIntegerResultOwner(consumer.value,owner,mapperOwner),resultType=resultOwner==='bool'?BOOL:resultOwner==='f64'?FLOAT:rustFixedType(resultOwner);
 const captures=[receiver,...args].map(value=>({name:parser.freshProductBinding(),type:source.type,value}));
 const owned=(type,name)=>({...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[name],arguments:[]}});
 const local=(capture,origin)=>parser.origins.mark({k:'name',path:[capture.name],span:capture.value.span??range},origin);
 if(consumerArity>0){const actual=readRustPrimitiveConversionFacts(parser,defaults[0],path);if(!acceptRustPrimitiveSelectionArgument(resultType.kind,String(resultType.bits??''),resultType.signed===true,resultOwner,actual.kind,actual.bits,actual.signed,actual.owner?.path.join('::')??'',actual.origin.wrappers.length,actual.origin.proven===true,admitted(actual),Boolean(literal(defaults[0]))))throw unsupported('Rust checked integer default','the actual default must retain the selected result type and source owner',range);captures.push({name:parser.freshProductBinding(),type:resultType,value:defaults[0]});}
 const number=value=>({k:'num',value,type:source.type,span:range});
 function project(plan,values={}){const stack=[],pop=()=>{if(!stack.length)throw new TypeError('the checked source plan lost an earlier term');return stack.pop();};for(const term of plan){
  if(term==='operand'||term==='right')stack.push(local(captures[term==='operand'?0:1],owned(source.type,owner)));
  else if(term==='zero')stack.push(number('0'));
  else if(term==='negative unit')stack.push({k:'unary',op:'neg',arg:number('1'),span:range});
  else if(term==='minimum'||term==='maximum'){const value=number(readRustSaturatingIntegerBoundMagnitude(term,source.type.bits,source.signed));stack.push(acceptRustSaturatingIntegerBoundNegative(term,source.signed)?{k:'unary',op:'neg',arg:value,span:range}:value);}
  else if(term==='true'||term==='false')stack.push({k:'bool',value:term==='true',span:range});
  else if(term==='condition'||term==='success'||term==='fallback'){if(!values[term])throw new TypeError('the checked source plan lost its actual consumer value');stack.push(values[term]);}
  else if(['lt','gt','eq','ne','le','ge','add','sub','mul','div','rem'].includes(term)){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}
  else if(term==='neg'||term==='not')stack.push({k:'unary',op:term,arg:pop(),span:range});
  else if(term==='if'){const otherwise=pop(),then=pop(),cond=pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}
  else throw new TypeError('unknown checked source plan term');
 }if(stack.length!==1)throw new TypeError('the checked source plan needs one complete result');return stack[0];}
 const condition=project(planRustCheckedIntegerCondition(method.value,source.signed));
 // A source term may be constructed eagerly; its represented arithmetic remains
 // inside the success branch, after the complete guard and eager captures.
 let success=parser.origins.mark(project(planRustCheckedIntegerSuccess(method.value,source.signed)),owned(source.type,owner));
 if(consumer.value==='map_or')success=projectRustPrimitiveFrom(parser,success,mapperOwner,range,path);
 const fallback=consumerArity>0?local(captures.at(-1),owned(resultType,resultOwner)):null;
 let body=project(planRustCheckedIntegerConsumer(consumer.value),{condition,success,fallback});
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};}
 return parser.origins.mark(body,owned(resultType,resultOwner));
}
