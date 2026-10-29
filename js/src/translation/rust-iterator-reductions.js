import {splitRustTypeClosingToken} from './rust-type-tokens.js';
import {acceptRustCopyClosureParameterOrigin} from './rust-copy-closure-parameter-origins.js';
// Actual iterator tokens, source owners and checked callback facts. Runtime
// reduction/short-circuit behavior belongs to the existing common array terms.
import {BOOL,FLOAT,NAT,rustFixedType,sameType} from './types.js';
import {unsupported} from './diagnostics.js';
import {projectRustStaticByteArrayReadOrigin} from './rust-byte-string-expressions.js';
import {readRustScalarArrayIndexReceiverError} from './rust-array-index-rules.js';
import {readRustArrayLookupElementOwnerError} from './rust-array-lookup-rules.js';
import {projectRustArrayLookupElementOwner} from './rust-array-lookup-source-owners.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {readRustIteratorOwnedReceiverError,readRustIteratorReductionInitialState,readRustIteratorCountReceiverError,readRustIteratorCollectionError,readRustIteratorReductionCountResultOwner,planRustIteratorReductionCount,acceptRustIteratorReductionStart,acceptRustIteratorReductionAdapter,readRustIteratorReductionAdapterState,planRustIteratorReductionReceiver,planRustIteratorReductionReverseIndex,readRustIteratorReductionCallbackArity,readRustIteratorReductionOperation,readRustIteratorReductionCaptureRoles,readRustIteratorReductionReceiverError,readRustIteratorReductionAccumulatorError,readRustIteratorReductionCallbackError} from './rust-iterator-reduction-rules.js';
export function readRustIteratorReduction(parser,receiver,method,args,range,path){
 if(!acceptRustIteratorReductionStart(method.value,args.length,parser.ordinaryCompletionContext()))return null;
 const refuse=(reason,where=range)=>{throw unsupported('Rust iterator reduction',reason,where);},actual=parser.origins.origin(receiver,path),source=projectRustStaticByteArrayReadOrigin(actual)??actual,element=source.type?.kind==='array'?source.type.element:null,owner=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner);
 const knownReceiver=readRustScalarArrayIndexReceiverError(source.type?.kind??'',element?.kind??'',source.arraySourceModel??'',source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'),source.arrayElementCopyEvidence===true)==='';
 const knownOwner=Boolean(element&&owner&&owner.arguments.length===0&&(owner.kind==='primitive'||owner.kind==='unit'&&owner.path.length===0)&&readRustArrayLookupElementOwnerError(element.kind,String(element.bits??''),element.signed===true,owner.path.join('::'))==='');
 const owned=method.value==='into_iter';if(owned){const error=readRustIteratorOwnedReceiverError(source.type?.kind??'',source.arraySourceModel??'',source.proven===true,source.wrappers.length,parser.origins.freshVectorArgument(receiver,path,true));if(error)refuse(error,method);}
 const c=parser.cursor;if(!c.eat('.')||c.peek().kind!=='identifier')refuse('an iterator requires its immediate copied or cloned reduction');
 let consumer=c.next(),state=readRustIteratorReductionInitialState(method.value),direction='forward';
 while(acceptRustIteratorReductionAdapter(consumer.value)){
  const next=readRustIteratorReductionAdapterState(consumer.value,state,direction);if(next.length!==2)refuse('the iterator adapter needs its actual item indirection and direction',consumer);
  [state,direction]=next;c.expect('(','iterator adapter');c.expect(')','iterator adapter');if(!c.eat('.')||c.peek().kind!=='identifier')refuse('an iterator requires its complete immediate reduction');consumer=c.next();
 }
 const borrowed=state==='borrowed';
 const operation=readRustIteratorReductionOperation(consumer.value);if(!operation)refuse('the complete standard iterator reduction needs a registered operation',consumer);
 const receiverError=operation==='count'?readRustIteratorCountReceiverError(source.type?.kind??'',source.arraySourceModel??'',source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'),owned||state==='borrowed'||knownReceiver&&knownOwner):readRustIteratorReductionReceiverError(knownReceiver,knownOwner);if(receiverError)refuse(receiverError);
 if(operation==='collect'){
  const copied=state==='copied'||state==='owned',early=copied?'':readRustIteratorCollectionError(false,false,'','','','',0,false,0);if(early)refuse(early,consumer);
  c.expect('::','iterator collection type');c.expect('<','iterator collection type');
  const previous=parser.iteratorCollectionElementOrigin,item={...parser.origins.typeOrigin(element,path),sourceOwner:structuredClone(owner)};let type,collected;
  parser.iteratorCollectionElementOrigin=item;
  try{type=parser.type();collected=structuredClone(parser.origins.type);}finally{parser.iteratorCollectionElementOrigin=previous;}
  splitRustTypeClosingToken(parser);c.expect('>','iterator collection type');c.expect('(','iterator collect');const close=c.expect(')','iterator collect');range={start:receiver.span?.start??range.start,end:close.end};
  const actualOwner=collected.expandedSourceOwner??collected.sourceOwner,child=actualOwner?.arguments[0];
  const agrees=type.kind==='array'&&sameType(type.element,element)&&Boolean(child&&child.kind===owner.kind&&child.path.join('::')===owner.path.join('::')&&child.arguments.length===0);
  const error=readRustIteratorCollectionError(copied,collected.proven===true,type.kind,collected.arraySourceModel??'',actualOwner?.kind??'',actualOwner?.path.join('::')??'',actualOwner?.arguments.length??0,agrees,collected.wrappers.length);if(error)refuse(error,consumer);
  const name=parser.freshProductBinding(),local=()=>({k:'name',path:[name],span:range}),object=constructIteratorReceiver(parser,local,name,element,consumer.value,direction,range,refuse),origin={...collected,path:[...path],sourceLifetimes:[],arrayElementCopyEvidence:true,freshArrayValue:true,freshVectorResultContract:false};
  const value=parser.origins.markVectorConstructor({k:'method',object,op:'slice',args:[],span:range},origin);
  return parser.origins.mark({k:'let',name,type:source.type,value:receiver,body:value,span:range},origin);
 }
 if(operation==='count'){
  c.expect('(','iterator count');const close=c.expect(')','iterator count');range={start:receiver.span?.start??range.start,end:close.end};
  const name=readRustIteratorReductionCountResultOwner(),type=rustFixedType(name),stack=[];
  for(const step of planRustIteratorReductionCount()){
   if(step==='receiver')stack.push(receiver);
   else if(step==='integer length')stack.push({k:'length',object:stack.pop(),integer:true,span:range});
   else if(step==='result cast')stack.push({k:'cast',arg:stack.pop(),to:type,flavor:'exact',span:range});
   else refuse('an iterator count plan needs its registered common operation');
  }
  if(stack.length!==1)refuse('an iterator count plan needs exactly one common term');
  return parser.origins.mark(stack[0],{...parser.origins.typeOrigin(type,path),sourceOwner:{kind:'primitive',path:[name],arguments:[]}});
 }
 c.expect('(','iterator reduction');let initial=null,initialOrigin=null;
 if(readRustIteratorReductionCallbackArity(consumer.value)===2){initial=parser.expr(path);initialOrigin=parser.origins.origin(initial,path);const error=readRustIteratorReductionAccumulatorError(Boolean(initialOrigin.type)&&initialOrigin.proven===true,parser.origins.immutableProduct(initialOrigin),initialOrigin.wrappers.length);if(error)refuse(error,initial.span);c.expect(',','iterator fold');}
 const itemOrigin={...parser.origins.typeOrigin(element,path),sourceOwner:structuredClone(owner),wrappers:borrowed?['reference']:[],sourceLifetimes:borrowed?[null]:[]},resultOrigin=initialOrigin??{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}},parameters=initialOrigin?[initialOrigin,itemOrigin]:[itemOrigin];
 const expected={...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:resultOrigin.type},path),copyEvidence:true,callableParameters:parameters,callableResult:resultOrigin};
 const callback=readRustExpectedCopyClosure(parser,path,expected);c.eat(',');const close=c.expect(')','iterator reduction');range={start:receiver.span?.start??range.start,end:close.end};
 const callable=parser.origins.origin(callback,path),error=readRustIteratorReductionCallbackError(consumer.value,callable.type?.kind??'',callable.type?.params?.length??-1,callable.wrappers.length,(callable.callableParameters??[]).map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin)),callable.callableResult?.proven===true&&Boolean(callable.callableResult.type));if(error)refuse(error,callback.span);
 for(let index=0;index<parameters.length;index+=1)parser.origins.argument(callable.callableParameters[index],parameters[index],callback.span);parser.origins.argument(callable.callableResult,resultOrigin,callback.span);
 const values={receiver,initial,callback},types={receiver:source.type,initial:initialOrigin?.type,callback:callable.type},captures=readRustIteratorReductionCaptureRoles(consumer.value).map(role=>({name:parser.freshProductBinding(),value:values[role],type:types[role],role})),names=new Map(captures.map(capture=>[capture.role,capture.name])),local=role=>({k:'name',path:[names.get(role)],span:range});
 const object=constructIteratorReceiver(parser,local,names.get('receiver'),element,consumer.value,direction,range,refuse);
 let body={k:'method',object,op:operation,args:[local('callback'),...(initial?[local('initial')]:[])],span:range};
 for(const capture of captures.toReversed())body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};
 return parser.origins.mark(body,structuredClone(resultOrigin));
}

function constructIteratorReceiver(parser,local,receiverName,element,consumer,direction,range,refuse){
 let object=local('receiver');for(const operation of planRustIteratorReductionReceiver(consumer,direction)){
  if(operation!=='map reversed indices')refuse('an iterator receiver plan needs its registered common operation');
  const unused=parser.freshProductBinding(),index=parser.freshProductBinding(),stack=[];
  for(const step of planRustIteratorReductionReverseIndex()){
   if(step==='receiver')stack.push(local('receiver'));
   else if(step==='index')stack.push({k:'name',path:[index],span:range});
   else if(step==='one')stack.push({k:'num',value:'1',type:FLOAT,span:range});
   else if(step==='length')stack.push({k:'length',object:stack.pop(),integer:false,span:range});
   else if(step==='natural')stack.push({k:'cast',arg:stack.pop(),to:NAT,flavor:'exact',span:range});
   else if(step==='sub'){const right=stack.pop(),left=stack.pop();stack.push({k:'binary',op:'sub',left,right,span:range});}
   else if(step==='lookup'){const index=stack.pop(),object=stack.pop();stack.push({k:'index',object,index,span:range});}
   else refuse('an iterator index plan needs its registered common operation');
  }
  if(stack.length!==1)refuse('an iterator index plan needs exactly one common term');
  const callback={k:'lambda',params:[{name:unused,type:element,span:range},{name:index,type:FLOAT,span:range}],ret:element,captures:[receiverName],body:stack[0],span:range};
  object={k:'method',object,op:'map',args:[callback],span:range};
 }
 return object;
}
