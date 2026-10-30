import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {acceptRustCopyClosureParameterOrigin} from './rust-copy-closure-parameter-origins.js';
import {readRustPrimitiveTryFromPredicateError,readRustPrimitiveTryFromMapperError,readRustPrimitiveTryFromMapperOwner} from './rust-primitive-try-from-rules.js';
import {STRING} from './types.js';
import {readRustPanicValueOperation,readRustPanicMessageSourceError} from './rust-panic-value-rules.js';
import {testRustPrimitiveTryFromPanicConsumer,readRustPrimitiveTryFromVersionedFailureDebug,readRustPrimitiveTryFromPanicPrefix} from './rust-primitive-try-from-rules.js';
// Actual token/type/origin transport into shared standard Result plans.
import {BOOL,FLOAT,UNIT,rustFixedType} from './types.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustPrimitiveTryFromOwner,acceptRustPrimitiveTryFromImport,acceptRustPrimitiveTryFromSource,readRustPrimitiveTryFromConsumerArity,readRustPrimitiveTryFromGuards,planRustPrimitiveTryFromConsumer,readRustPrimitiveTryFromResultOwner} from './rust-primitive-try-from-rules.js';
import {readRustPrimitiveFromOwner,acceptRustPrimitiveFromSource,planRustPrimitiveFrom} from './rust-primitive-from-rules.js';
import {acceptRustPrimitiveSelectionArgument} from './rust-primitive-selection-rules.js';
import {readRustSaturatingIntegerBoundMagnitude,acceptRustSaturatingIntegerBoundNegative} from './rust-saturating-integer-rules.js';
import {projectRustPrimitiveFrom,prepareRustPrimitiveFromArgument} from './rust-primitive-from.js';
export function readRustPrimitiveConversionHead(parser,path){
 const c=parser.cursor,start=c.peek(),absolute=c.is('::');let offset=absolute?1:0;const segments=[];
 while(c.peek(offset).kind==='identifier'&&segments.length<16){segments.push(c.peek(offset).value);offset+=1;if(!c.is('::',offset))break;if(c.peek(offset+1).kind!=='identifier')return null;offset+=1;}
 if(segments.length<2)return null;
 const resolved=absolute?segments:parser.resolve(segments,path),root=segments[0],target=resolved[0],shadowed=!absolute&&Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root)||parser.origins.lookup([target],path)||parser.origins.scope.has(target));
 return {start,absolute,offset,segments,resolved,shadowed};
}
export function readRustPrimitiveOriginFacts(origin){const type=origin.type,owner=origin.sourceOwner??(type?.kind==='float'&&origin.floatSourceModel==='binary64 scalar'?{kind:'primitive',path:['f64'],arguments:[]}:null);return {origin,type,owner,kind:type?.kind??'',bits:String(type?.bits??''),signed:type?.signed===true};}
export function readRustPrimitiveConversionFacts(parser,value,path){const actual=parser.origins.origin(value,path),origin=value.k==='bool'&&actual.proven&&actual.wrappers.length===0&&!actual.sourceOwner?{...actual,sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}}:actual;return readRustPrimitiveOriginFacts(origin);}
export function readRustPrimitiveScalarResultOwner(origin){if(!origin)return '';const value=readRustPrimitiveOriginFacts(origin);return readRustPrimitiveTryFromMapperOwner(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length);}

function admitted(value){return acceptRustPrimitiveFromSource(value.kind,value.bits,value.signed,value.owner?.kind??'',value.owner?.path.join('::')??'',value.owner?.arguments.length??-1,value.origin.proven===true,value.origin.wrappers.length);}
// Actual scalar callback syntax/origins; the shared rules own admission.
export function readRustPrimitiveScalarMapper(parser,owner,fallbackValue,path,feature,fallbackResult=null){
 const parameterType=rustFixedType(owner);if(!parameterType)throw unsupported(feature,'a scalar mapper needs its actual standard integer source owner',fallbackValue.span);
   const fallback=fallbackResult?readRustPrimitiveOriginFacts(fallbackResult):readRustPrimitiveConversionFacts(parser,fallbackValue,path),contextOwner=readRustPrimitiveTryFromMapperOwner(fallback.kind,fallback.bits,fallback.signed,fallback.owner?.kind??'',fallback.owner?.path.join('::')??'',fallback.owner?.arguments.length??-1,fallback.origin.proven===true,fallback.origin.wrappers.length),result=contextOwner?{...fallback.origin,sourceOwner:structuredClone(fallback.owner)}:null;
   const parameter={...parser.origins.typeOrigin(parameterType,path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]}},expected={...parser.origins.typeOrigin({kind:'function',params:[parameter.type],ret:result?.type??UNIT},path),copyEvidence:true,callableParameters:[parameter],callableResult:result};
   const mapperCallback=readRustExpectedCopyClosure(parser,path,expected,result===null);const mapperOrigin=parser.origins.origin(mapperCallback,path);
   const actual=mapperOrigin.callableParameters?.[0],output=mapperOrigin.callableResult,type=output?.type,sourceOwner=output?.sourceOwner??(type?.kind==='float'&&output.floatSourceModel==='binary64 scalar'?{kind:'primitive',path:['f64'],arguments:[]}:null);
   const mapperOwner=readRustPrimitiveTryFromMapperOwner(type?.kind??'',String(type?.bits??''),type?.signed===true,sourceOwner?.kind??'',sourceOwner?.path.join('::')??'',sourceOwner?.arguments.length??-1,output?.proven===true,output?.wrappers.length??-1);
   const error=readRustPrimitiveTryFromMapperError(mapperOrigin.type?.kind??'',mapperOrigin.type?.params?.length??-1,mapperOrigin.wrappers.length,acceptRustCopyClosureParameterOrigin(parser,actual),mapperOwner!=='');if(error)throw unsupported(feature,error,mapperCallback.span);
   parser.origins.argument(actual,parameter,mapperCallback.span);
 return {callback:mapperCallback,origin:mapperOrigin,owner:mapperOwner};
}
export function readRustPrimitiveTryFromImport(parser,segments,path){const root=segments[0]??'',shadowed=Boolean(parser.origins.lookup([root],path)||parser.origins.scope.has(root)||parser.aliases.get(path.join('.'))?.has(root));return acceptRustPrimitiveTryFromImport(segments.join('::'),parser.ordinaryCompletionContext(),shadowed);}
export function readRustPrimitiveTryFrom(parser,path){
 const c=parser.cursor,valueHead=readRustPrimitiveConversionHead(parser,path);if(!valueHead||valueHead.resolved.at(-1)!=='try_from')return null;
 const owner=readRustPrimitiveTryFromOwner(valueHead.resolved,parser.ordinaryCompletionContext(),valueHead.shadowed,valueHead.absolute);if(!owner)return null;
 if(!c.is('(',valueHead.offset))throw unsupported('Rust primitive TryFrom','an associated conversion function value needs its own callable interpretation',{start:valueHead.start.start,end:c.peek(valueHead.offset).end});
 for(let at=0;at<=valueHead.offset;at+=1)c.next();
 const inputArgs=[];while(!c.is(')')){inputArgs.push(parser.expr(path));if(!c.eat(','))break;}c.expect(')','primitive TryFrom call');
 if(inputArgs.length!==1)throw new TranslationError('type','a primitive TryFrom call needs one actual source argument',{start:valueHead.start.start,end:c.peek().end});
 if(!c.eat('.')||c.peek().kind!=='identifier')throw unsupported('Rust primitive TryFrom','the actual Result needs an immediate admitted standard consumer',{start:valueHead.start.start,end:c.peek().end});
 const consumer=c.next(),arity=readRustPrimitiveTryFromConsumerArity(consumer.value);
 if(arity<0)throw unsupported('Rust primitive TryFrom','the actual Result needs an immediate admitted standard consumer',{start:valueHead.start.start,end:consumer.end});
 c.expect('(','primitive TryFrom consumer');const defaults=[];let mapperOwner='';
 let predicate=null,predicateOrigin=null,mapperCallback=null,mapperOrigin=null;
 if(consumer.value==='is_ok_and'){
  const parameter={...parser.origins.typeOrigin(rustFixedType(owner),path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]}},result={...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}},expected={...parser.origins.typeOrigin({kind:'function',params:[parameter.type],ret:BOOL},path),copyEvidence:true,callableParameters:[parameter],callableResult:result};
  predicate=readRustExpectedCopyClosure(parser,path,expected);predicateOrigin=parser.origins.origin(predicate,path);
  const actual=predicateOrigin.callableParameters?.[0],output=predicateOrigin.callableResult,error=readRustPrimitiveTryFromPredicateError(predicateOrigin.type?.kind??'',predicateOrigin.type?.params?.length??-1,predicateOrigin.wrappers.length,acceptRustCopyClosureParameterOrigin(parser,actual),output?.proven===true&&output.type?.kind==='bool'&&output.wrappers.length===0);
  if(error)throw unsupported('Rust primitive TryFrom predicate',error,predicate.span);
  parser.origins.argument(actual,parameter,predicate.span);parser.origins.argument(output,result,predicate.span);
 }else if(arity>0){defaults.push(parser.expr(path));if(arity===2){c.expect(',','primitive TryFrom mapper');const mapper=readRustPrimitiveConversionHead(parser,path),standardOwner=mapper?.resolved.at(-1)==='from'?readRustPrimitiveFromOwner(mapper.resolved,parser.ordinaryCompletionContext(),mapper.shadowed,mapper.absolute):'';
  if(standardOwner){mapperOwner=standardOwner;if(!planRustPrimitiveFrom(mapperOwner,owner))throw unsupported('Rust primitive TryFrom mapper','the actual standard primitive From implementation must exist for the successful target',{start:mapper.start.start,end:c.peek(mapper.offset-1).end});for(let at=0;at<mapper.offset;at+=1)c.next();}
  else {
   const mapped=readRustPrimitiveScalarMapper(parser,owner,defaults[0],path,'Rust primitive TryFrom mapper');mapperCallback=mapped.callback;mapperOrigin=mapped.origin;mapperOwner=mapped.owner;
  }
 }}
 if(arity>0)c.eat(',');const close=c.expect(')','primitive TryFrom consumer'),range={start:valueHead.start.start,end:close.end};
 const input=prepareRustPrimitiveFromArgument(parser,inputArgs[0],path),source=readRustPrimitiveConversionFacts(parser,input,path),target=rustFixedType(owner);
 if(!acceptRustPrimitiveTryFromSource(source.kind,source.bits,source.signed,source.owner?.kind??'',source.owner?.path.join('::')??'',source.owner?.arguments.length??-1,source.origin.proven===true,source.origin.wrappers.length))throw unsupported('Rust primitive TryFrom','the actual input must retain a proven direct standard integer source owner',range);
 const resultOwner=readRustPrimitiveTryFromResultOwner(consumer.value,owner,mapperOwner),resultType=resultOwner==='bool'?BOOL:resultOwner==='f64'?FLOAT:rustFixedType(resultOwner),captures=[{name:parser.freshProductBinding(),type:source.type,value:input}];
 const local=(capture,origin)=>parser.origins.mark({k:'name',path:[capture.name],span:capture.value.span??range},origin);
 const inputValue=()=>local(captures[0],source.origin),targetOrigin={...parser.origins.typeOrigin(target,path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]}};
 const success=parser.origins.mark({k:'cast',arg:inputValue(),to:target,flavor:'exact',span:range},targetOrigin);
 const converted=consumer.value==='map_or'&&!mapperCallback?projectRustPrimitiveFrom(parser,success,mapperOwner,range,path):success;
 const panicking=testRustPrimitiveTryFromPanicConsumer(consumer.value);
 if(arity>0&&!predicate){const actual=readRustPrimitiveConversionFacts(parser,defaults[0],path),literal=defaults[0].k==='num'&&!defaults[0].type||defaults[0].k==='unary'&&defaults[0].op==='neg'&&defaults[0].arg.k==='num'&&!defaults[0].arg.type;
  if(panicking){const error=readRustPanicMessageSourceError(consumer.value,actual.kind,actual.origin.textSourceModel??'',actual.origin.wrappers.length,actual.origin.proven===true,actual.origin.wrappers.every(layer=>layer==='reference'));if(error)throw unsupported('Rust primitive TryFrom message',error,range);}
  if(!panicking&&!acceptRustPrimitiveSelectionArgument(resultType.kind,String(resultType.bits??''),resultType.signed===true,resultOwner,actual.kind,actual.bits,actual.signed,actual.owner?.path.join('::')??'',actual.origin.wrappers.length,actual.origin.proven===true,admitted(actual),Boolean(literal)))throw unsupported('Rust primitive TryFrom default','the actual default must retain the selected result type and primitive source owner',range);
  captures.push({name:parser.freshProductBinding(),type:panicking?STRING:resultType,value:defaults[0]});}
 if(mapperCallback)captures.push({name:parser.freshProductBinding(),type:mapperOrigin.type,value:mapperCallback});
 if(predicate)captures.push({name:parser.freshProductBinding(),type:predicateOrigin.type,value:predicate});
 const plan=planRustPrimitiveTryFromConsumer(consumer.value,readRustPrimitiveTryFromGuards(source.type.bits,source.signed,target.bits,target.signed)),stack=[];
 const pop=()=>{if(!stack.length)throw new TypeError('the shared primitive Result plan lost an earlier source term');return stack.pop();};
 for(const term of plan){
  if(term==='input')stack.push(inputValue());
  else if(term==='minimum'||term==='maximum'){const magnitude=readRustSaturatingIntegerBoundMagnitude(term,target.bits,target.signed),number={k:'num',value:magnitude,type:source.type,span:range};stack.push(acceptRustSaturatingIntegerBoundNegative(term,target.signed)?{k:'unary',op:'neg',arg:number,span:range}:number);}
  else if(term==='positive error'||term==='negative error')stack.push({k:'str',value:readRustPrimitiveTryFromVersionedFailureDebug(term,parser.sourceVersionMinor),span:range});
  else if(term==='panic prefix')stack.push({k:'str',value:readRustPrimitiveTryFromPanicPrefix(consumer.value),span:range});
  else if(term==='message')stack.push(local(captures[1],parser.origins.origin(defaults[0],path)));
  else if(term==='concat'){const right=pop(),left=pop();stack.push({k:'binary',op:'concat',left,right,span:range});}
  else if(term==='raise')stack.push({k:'math',op:readRustPanicValueOperation(),name:'Rust primitive conversion panic message',args:[{value:pop(),spread:false}],span:range});
  else if(term==='predicate')stack.push({k:'app',fn:local(captures[1],predicateOrigin),args:[success],span:range});
  else if(term==='false')stack.push({k:'bool',value:false,span:range});
  else if(term==='true')stack.push({k:'bool',value:true,span:range});
  else if(term==='success')stack.push(mapperCallback?{k:'app',fn:local(captures[2],mapperOrigin),args:[success],span:range}:converted);
  else if(term==='default')stack.push({k:'num',value:'0',type:target,span:range});
  else if(term==='fallback')stack.push(local(captures[1],{...parser.origins.typeOrigin(resultType,path),sourceOwner:{kind:'primitive',path:[resultOwner],arguments:[]}}));
  else if(['ge','le','and'].includes(term)){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}
  else if(term==='not')stack.push({k:'unary',op:'not',arg:pop(),span:range});
  else if(term==='if'){const otherwise=pop(),then=pop(),cond=pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}
  else throw new TypeError('unknown shared primitive Result source term');
 }
 if(stack.length!==1)throw new TypeError('the shared primitive Result plan needs one complete result');let body=stack[0];for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};}
 return parser.origins.mark(body,{...parser.origins.typeOrigin(resultType,path),sourceOwner:{kind:'primitive',path:[resultOwner],arguments:[]}});
}
