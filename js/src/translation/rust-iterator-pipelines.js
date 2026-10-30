import {planRustIteratorArithmeticSeed} from './rust-iterator-arithmetic-rules.js';
// Actual syntax/origin facts and common-node construction. Shared rules own
// stage meaning, lazy branching, terminal terms and capture order.
import {BOOL,UNIT,rustFixedType} from './types.js';
import {unsupported} from './diagnostics.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {acceptRustCopyClosureParameterOrigin} from './rust-copy-closure-parameter-origins.js';
import {readRustArrayLookupElementOwnerError} from './rust-array-lookup-rules.js';
import {readRustIteratorReductionCountResultOwner} from './rust-iterator-reduction-rules.js';
import {readRustIteratorPipelineContinuationItem,readRustIteratorPipelineSeed,readRustIteratorPipelineParameterLayers,readRustIteratorPipelineResultContext,readRustIteratorPipelineCallableError,planRustIteratorPipelineStage,planRustIteratorPipelineTerminal,readRustIteratorPipelineInactive,readRustIteratorPipelineConsumerOperation,planRustIteratorPipelineCaptures} from './rust-iterator-pipeline-rules.js';
function refuse(reason,span){throw unsupported('Rust iterator pipeline',reason,span);}
function ownerKnown(origin){const type=origin?.type,owner=origin?.expandedSourceOwner??origin?.sourceOwner;return Boolean(type&&owner&&owner.arguments.length===0&&(owner.kind==='primitive'||owner.kind==='unit'&&owner.path.length===0)&&readRustArrayLookupElementOwnerError(type.kind,String(type.bits??''),type.signed===true,owner.path.join('::'))==='');}
export function readRustIteratorPipelineStage(parser,item,method,path){
 const c=parser.cursor,layers=readRustIteratorPipelineParameterLayers(method.value,item.wrappers.length),parameter={...structuredClone(item),wrappers:Array(layers).fill('reference'),sourceLifetimes:Array(layers).fill(null)},context=readRustIteratorPipelineResultContext(method.value),result=context==='bool'||context==='unit'?{...parser.origins.typeOrigin(context==='unit'?UNIT:BOOL,path),sourceOwner:{kind:context==='unit'?'unit':'primitive',path:context==='unit'?[]:['bool'],arguments:[]}}:null;
 const expected={...parser.origins.typeOrigin({kind:'function',params:[parameter.type],ret:result?.type??UNIT},path),copyEvidence:true,callableParameters:[parameter],callableResult:result};
 c.expect('(','iterator stage');const callback=readRustExpectedCopyClosure(parser,path,expected,context==='infer');c.eat(',');c.expect(')','iterator stage');
 const callable=parser.origins.origin(callback,path),actual=callable.callableParameters?.[0],output=callable.callableResult;
 const error=readRustIteratorPipelineCallableError(callable.type?.kind??'',callable.type?.params?.length??-1,callable.wrappers.length,acceptRustCopyClosureParameterOrigin(parser,actual),output?.proven===true,parser.origins.immutableProduct(output),ownerKnown(output));if(error)refuse(error,callback.span);
 parser.origins.argument(actual,parameter,callback.span);if(result)parser.origins.argument(output,result,callback.span);
 return {operation:method.value,callback,type:callable.type,input:structuredClone(item),output:structuredClone(context==='infer'?output:item)};
}
export function createRustIteratorPipelineCountOrigin(parser,path){const owner=readRustIteratorReductionCountResultOwner(),type=rustFixedType(owner);return {...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[owner],arguments:[]}};}
export function constructRustIteratorPipeline(parser,{receiver,source,stages,consumer,direction,inputElement,item,initial,initialOrigin,callback,callable,resultOrigin,range,constructReceiver}){
 const values={receiver:{value:receiver,type:source.type},initial:{value:initial,type:initialOrigin?.type},callback:{value:callback,type:callable?.type}},captures=[],stageNames=[];let stageIndex=0;
 for(const role of planRustIteratorPipelineCaptures(consumer,stages.length)){
  const actual=role==='stage'?{value:stages[stageIndex].callback,type:stages[stageIndex++].type}:values[role],name=parser.freshProductBinding();
  if(!actual?.value)refuse('a pipeline capture needs its complete actual source operand',range);
  captures.push({name,...actual,role});if(role==='stage')stageNames.push(name);
 }
 const names=new Map(captures.filter(capture=>capture.role!=='stage').map(capture=>[capture.role,capture.name])),localName=name=>({k:'name',path:[name],span:range}),local=role=>localName(names.get(role)),operation=readRustIteratorPipelineConsumerOperation(consumer),reducing=operation==='reduce',elementName=parser.freshProductBinding(),accumulatorName=reducing?parser.freshProductBinding():null,accumulator=()=>localName(accumulatorName),element=()=>localName(elementName),ret=resultOrigin.type;
 const inactive=()=>{const role=readRustIteratorPipelineInactive(consumer);if(role==='true'||role==='false')return {k:'bool',value:role==='true',span:range};if(role==='accumulator')return accumulator();refuse('a pipeline terminal needs its registered inactive term',range);};
 const project=(plan,terms,binding)=>{const stack=[];for(const step of plan){if(Object.hasOwn(terms,step))stack.push(terms[step]);else if(step==='call one'){const value=stack.pop(),fn=stack.pop();stack.push({k:'app',fn,args:[value],span:range});}else if(step==='call two'){const value=stack.pop(),acc=stack.pop(),fn=stack.pop();stack.push({k:'app',fn,args:[acc,value],span:range});}else if(step==='add'||step==='multiply'){const right=stack.pop(),left=stack.pop();stack.push({k:'binary',op:step==='add'?'add':'mul',left,right,span:range});}else if(step==='append'){const value=stack.pop(),acc=stack.pop();stack.push({k:'array',element:item.type,items:[{value:acc,spread:true},{value,spread:false}],span:range});}else if(step==='bind'){const body=stack.pop(),value=stack.pop();stack.push({k:'let',name:binding.name,type:binding.type,value,body,span:range});}else if(step==='if'){const otherwise=stack.pop(),then=stack.pop(),cond=stack.pop();stack.push({k:'if',cond,then,else:otherwise,span:range});}else refuse('a pipeline plan needs its registered common operation',range);}if(stack.length!==1)refuse('a pipeline plan needs exactly one common term',range);return stack[0];};
 const process=(index,value)=>{
  if(index===stages.length)return project(planRustIteratorPipelineTerminal(consumer),{callback:callback?local('callback'):null,item:value,accumulator:reducing?accumulator():null,one:{k:'num',value:'1',type:ret,span:range}});
  const stage=stages[index],application={k:'app',fn:localName(stageNames[index]),args:[value],span:range},binding={name:parser.freshProductBinding(),type:stage.type.ret},continued=process(index+1,readRustIteratorPipelineContinuationItem(stage.operation)==='mapped'?localName(binding.name):value);
  return project(planRustIteratorPipelineStage(stage.operation),{application,continuation:continued,inactive:inactive()},binding);
 };
 const fused={k:'lambda',params:[...(reducing?[{name:accumulatorName,type:ret,span:range}]:[]),{name:elementName,type:inputElement,span:range}],ret,captures:[...stageNames,...(callback?[names.get('callback')]:[])],body:process(0,element()),span:range},object=constructReceiver(local,names.get('receiver'),inputElement,consumer,direction,range);
 const seedRole=readRustIteratorPipelineSeed(consumer),arithmeticSeed=seedRole==='numeric identity'?planRustIteratorArithmeticSeed(consumer,ret.kind):[],seed=seedRole==='unit'?{k:'unit',span:range}:seedRole==='numeric identity'?{k:'num',value:arithmeticSeed[0],negative:arithmeticSeed[1]==='negative',type:ret,span:range}:seedRole==='zero'?{k:'num',value:'0',type:ret,span:range}:seedRole==='empty array'?{k:'array',items:[],element:item.type,span:range}:initial?local('initial'):null;
 let body={k:'method',object,op:operation,args:[fused,...(reducing?[seed]:[])],span:range};
 if(consumer==='collect')body=parser.origins.markVectorConstructor(body,resultOrigin);
 for(const capture of captures.toReversed())body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};
 return parser.origins.mark(body,structuredClone(resultOrigin));
}
