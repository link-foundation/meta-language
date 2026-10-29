// Actual tokens/origins project complete shared plans into the existing core.
import {BOOL,FLOAT,rustFixedType} from './types.js';
import {readRustPrimitiveConversionHead} from './rust-primitive-try-from.js';
import {readRustPrimitiveFromOwner,planRustPrimitiveFrom} from './rust-primitive-from-rules.js';
import {projectRustPrimitiveFrom} from './rust-primitive-from.js';
import {TranslationError,unsupported} from './diagnostics.js';
import {readRustArrayLookupArity,acceptRustArrayLookupPresenceConsumer,readRustArrayLookupConsumerArity,readRustArrayLookupReceiverError,readRustArrayLookupElementOwnerError,readRustArrayLookupClonedElementError,readRustArrayLookupDefaultTerm,planRustArrayLookupCondition,planRustArrayLookupIndex,planRustArrayLookupConsumer} from './rust-array-lookup-rules.js';
import {projectRustArrayLookupElementOwner} from './rust-array-lookup-source-owners.js';
import {readRustUsizeIndexExpression} from './rust-usize-index-expressions.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {projectRustPrimitiveDefault} from './rust-primitive-default.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {testRustArrayLookupLazyFallback,readRustArrayLookupLazyFallbackError,testRustArrayLookupPredicateConsumer,readRustArrayLookupPredicateError,readRustArrayLookupCallableContextArity,readRustArrayLookupMapperError} from './rust-array-lookup-rules.js';
function callableContext(parser,receiver,consumer,path,mappingResult=null){
 const source=parser.origins.origin(receiver,path),type=source.type,element=type?.kind==='array'?type.element:null;
 if(!element)return null;
 const owner=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner);
 const knownElement=Boolean(owner&&owner.arguments.length===0&&(owner.kind==='primitive'||owner.kind==='unit'&&owner.path.length===0)&&readRustArrayLookupElementOwnerError(element.kind,String(element.bits??''),element.signed===true,owner.path.join('::'))==='');
 const knownReceiver=readRustArrayLookupReceiverError(type.kind,element.kind,source.arraySourceModel??'',source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'),source.arrayElementCopyEvidence===true,true)==='';
 const arity=readRustArrayLookupCallableContextArity(consumer,knownReceiver,knownElement);
 if(arity<0)return null;
 const selected={...parser.origins.typeOrigin(element,path),sourceOwner:structuredClone(owner)};
 if(consumer==='map_or'&&(!mappingResult?.proven||!mappingResult.type))return null;
 const result=consumer==='map_or'?mappingResult:arity===0?selected:{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 const parameters=arity===0?[]:[selected];
 return {...parser.origins.typeOrigin({kind:'function',params:parameters.map(parameter=>parameter.type),ret:result.type},path),copyEvidence:true,callableParameters:parameters,callableResult:result};
}
export function readRustArrayLookup(parser,receiver,method,args,range,path){
 const arity=readRustArrayLookupArity(method.value,parser.ordinaryCompletionContext());if(arity<0)return null;
 if(args.length!==arity)throw new TranslationError('type',method.value+' needs '+arity+' actual array lookup arguments',range);
 const c=parser.cursor;let copied=false,cloned=false;
 if(!c.eat('.')||c.peek().kind!=='identifier')throw unsupported('Rust array lookup Option','an immediate standard presence or copied-value consumer is required',range);
 let consumer=c.next();
 if(consumer.value==='copied'||consumer.value==='cloned'){cloned=consumer.value==='cloned';copied=true;c.expect('(','copied lookup');c.expect(')','copied lookup');if(!c.eat('.')||c.peek().kind!=='identifier')throw unsupported('Rust array lookup Option','an immediate standard presence or copied-value consumer is required',range);consumer=c.identifier('copied lookup consumer');}
 const consumerArity=readRustArrayLookupConsumerArity(consumer.value,copied);
 if(consumerArity<0)throw unsupported('Rust array lookup Option','an immediate standard presence or copied-value consumer is required',range);
 c.expect('(','array lookup consumer');const defaults=[];let mapperOwner='';
 if(consumerArity>0){const context=consumer.value==='map_or_else'?null:callableContext(parser,receiver,consumer.value,path);defaults.push(context?readRustExpectedCopyClosure(parser,path,context):parser.expr(path));
  if(consumerArity===2){c.expect(',','array lookup mapper');const mapper=readRustPrimitiveConversionHead(parser,path),standardOwner=mapper?.resolved.at(-1)==='from'?readRustPrimitiveFromOwner(mapper.resolved,parser.ordinaryCompletionContext(),mapper.shadowed,mapper.absolute):'';
   if(standardOwner){mapperOwner=standardOwner;for(let at=0;at<mapper.offset;at+=1)c.next();}
   else {const fallback=parser.origins.origin(defaults[0],path),result=consumer.value==='map_or_else'?fallback.callableResult:fallback;
    // The second argument uses the same one-element mapper context in both
    // methods; its result comes from the actual default value/callable result.
    const context=callableContext(parser,receiver,'map_or',path,result);defaults.push(context?readRustExpectedCopyClosure(parser,path,context):parser.expr(path));}}
  c.eat(',');}
 const close=c.expect(')','array lookup consumer');range={start:receiver.span?.start??range.start,end:close.end};
 const source=parser.origins.origin(receiver,path),element=source.type?.kind==='array'?source.type.element:null,leaf=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner),cloneError=cloned?readRustArrayLookupClonedElementError(element?.kind??'',String(element?.bits??''),element?.signed===true,leaf?.kind??'',leaf?.path.join('::')??'',leaf?.arguments.length??-1):'',stringClone=cloned&&element?.kind==='string'&&cloneError==='',error=readRustArrayLookupReceiverError(source.type?.kind??'',element?.kind??'',source.arraySourceModel??'',source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'),source.arrayElementCopyEvidence===true,copied&&!stringClone);
 if(error)throw unsupported('Rust array lookup receiver',error,range);
 if(cloneError)throw unsupported('Rust array lookup clone',cloneError,range);
 if(arity&&readRustUsizeIndexExpression(parser,args[0],path))throw unsupported('Rust array lookup index','the actual index needs its complete direct usize source meaning',args[0].span??range);
 const presence=acceptRustArrayLookupPresenceConsumer(consumer.value);
 if(!presence&&!element)throw unsupported('Rust array lookup result','a copied value needs its complete actual scalar element type',range);
 const owner=readRustPointerIntegerContractOwner(source.expandedSourceOwner??source.sourceOwner);
 if(!presence&&!cloned){const error=readRustArrayLookupElementOwnerError(element.kind,String(element.bits??''),element.signed===true,owner);if(error)throw unsupported('Rust array lookup result',error,range);}
 const selectedOrigin=presence?{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}}:{...parser.origins.typeOrigin(element,path),sourceOwner:stringClone?structuredClone(leaf):element.kind==='unit'?{kind:'unit',path:[],arguments:[]}:owner?{kind:'primitive',path:[owner],arguments:[]}:null};
 if(mapperOwner&&!planRustPrimitiveFrom(mapperOwner,owner))throw unsupported('Rust array lookup mapper','the actual standard primitive From implementation must exist for the selected element',range);
 const mapperOrigin=defaults.length===2?parser.origins.origin(defaults[1],path):null;
 if(mapperOrigin){const parameter=mapperOrigin.callableParameters?.[0],result=mapperOrigin.callableResult,error=readRustArrayLookupMapperError(mapperOrigin.type?.kind??'',mapperOrigin.type?.params?.length??-1,mapperOrigin.wrappers.length,parameter?.proven===true,result?.proven===true&&Boolean(result.type));if(error)throw unsupported('Rust array lookup mapper',error,range);parser.origins.argument(selectedOrigin,parameter,defaults[1].span??range);}
 const resultType=mapperOrigin?mapperOrigin.callableResult.type:mapperOwner?(mapperOwner==='bool'?BOOL:mapperOwner==='f64'?FLOAT:rustFixedType(mapperOwner)):selectedOrigin.type;
 const resultOrigin=mapperOrigin?structuredClone(mapperOrigin.callableResult):mapperOwner?{...parser.origins.typeOrigin(resultType,path),sourceOwner:{kind:'primitive',path:[mapperOwner],arguments:[]}}:selectedOrigin;
 const lazy=testRustArrayLookupLazyFallback(consumer.value),predicate=testRustArrayLookupPredicateConsumer(consumer.value),fallbackOrigin=defaults.length?parser.origins.origin(defaults[0],path):null;
 const booleanOrigin={...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
 if(lazy){const error=readRustArrayLookupLazyFallbackError(fallbackOrigin?.type?.kind??'',fallbackOrigin?.type?.params?.length??-1,fallbackOrigin?.wrappers.length??-1,fallbackOrigin?.callableResult?.proven===true);if(error)throw unsupported('Rust array lookup lazy fallback',error,range);}
 if(predicate){const parameter=fallbackOrigin?.callableParameters?.[0],result=fallbackOrigin?.callableResult,error=readRustArrayLookupPredicateError(fallbackOrigin?.type?.kind??'',fallbackOrigin?.type?.params?.length??-1,fallbackOrigin?.wrappers.length??-1,parameter?.proven===true,result?.type?.kind??'',result?.proven===true);if(error)throw unsupported('Rust array lookup predicate',error,range);parser.origins.argument(resultOrigin,parameter,defaults[0].span??range);parser.origins.argument(result,booleanOrigin,defaults[0].span??range);}
 else if(defaults.length)parser.origins.argument(lazy?fallbackOrigin.callableResult:fallbackOrigin,resultOrigin,defaults[0].span??range);
 const usize=rustFixedType('usize'),captures=[{name:parser.freshProductBinding(),value:receiver,type:source.type},...(arity?[{name:parser.freshProductBinding(),value:args[0],type:usize}]:[]),...(defaults.length?[{name:parser.freshProductBinding(),value:defaults[0],type:lazy||predicate?fallbackOrigin.type:resultType}]:[]),...(mapperOrigin?[{name:parser.freshProductBinding(),value:defaults[1],type:mapperOrigin.type}]:[])],local=index=>({k:'name',path:[captures[index].name],span:captures[index].value.span??range});
 const length={k:'cast',arg:{k:'length',object:local(0),integer:true,span:range},to:usize,flavor:'exact',span:range},number=value=>({k:'num',value,type:usize,span:range});
 const values={index:arity?local(1):null,length,zero:number('0'),one:number('1')};
 function project(plan){const stack=[],pop=()=>{if(!stack.length)throw new TypeError('an array lookup source plan lost an earlier term');return stack.pop();};for(const term of plan){if(values[term])stack.push(values[term]);else if(term==='lt'||term==='gt'||term==='sub'){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}else if(term==='not')stack.push({k:'unary',op:'not',arg:pop(),span:range});else if(term==='if'){const otherwise=pop(),then=pop(),cond=pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}else throw new TypeError('unknown array lookup source term');}if(stack.length!==1)throw new TypeError('an array lookup source plan needs one complete result');return stack[0];}
 values.condition=project(planRustArrayLookupCondition(method.value));
 if(!presence){const index=project(planRustArrayLookupIndex(method.value)),term=readRustArrayLookupDefaultTerm(element.kind,stringClone);values.selected={k:'index',object:local(0),index:{k:'cast',arg:index,to:{kind:'nat'},flavor:'exact',span:range},span:range};values.fallback=defaults.length?(lazy?{k:'app',fn:local(captures.length-1),args:[],span:range}:local(captures.length-1)):term==='unit'?{k:'unit',span:range}:term==='empty string'?{k:'str',value:'',span:range}:projectRustPrimitiveDefault(parser,owner,path,range);}
 if(predicate){values.predicate={k:'app',fn:local(captures.length-1),args:[values.selected],span:range};values.true={k:'bool',value:true,span:range};values.false={k:'bool',value:false,span:range};}
 if(mapperOwner)values.selected=projectRustPrimitiveFrom(parser,parser.origins.mark(values.selected,selectedOrigin),mapperOwner,range,path);
 if(mapperOrigin){const fallback=local(captures.length-2);values.fallback=lazy?{k:'app',fn:fallback,args:[],span:range}:fallback;values.selected={k:'app',fn:local(captures.length-1),args:[values.selected],span:range};}
 let body=project(planRustArrayLookupConsumer(consumer.value));for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,...(capture.type?{type:capture.type}:{}),value:capture.value,body,span:range};}
 return parser.origins.mark(body,predicate?booleanOrigin:resultOrigin);
}
