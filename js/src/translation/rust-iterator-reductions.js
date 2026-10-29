// Actual iterator tokens, source owners and checked callback facts. Runtime
// reduction/short-circuit behavior belongs to the existing common array terms.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
import {projectRustStaticByteArrayReadOrigin} from './rust-byte-string-expressions.js';
import {readRustScalarArrayIndexReceiverError} from './rust-array-index-rules.js';
import {readRustArrayLookupElementOwnerError} from './rust-array-lookup-rules.js';
import {projectRustArrayLookupElementOwner} from './rust-array-lookup-source-owners.js';
import {readRustExpectedCopyClosure} from './rust-copy-closures.js';
import {acceptRustIteratorReductionStart,acceptRustIteratorCopyAdapter,readRustIteratorReductionOperation,readRustIteratorReductionCaptureRoles,readRustIteratorReductionReceiverError,readRustIteratorReductionAccumulatorError,readRustIteratorReductionCallbackError} from './rust-iterator-reduction-rules.js';
export function readRustIteratorReduction(parser,receiver,method,args,range,path){
 if(!acceptRustIteratorReductionStart(method.value,args.length,parser.ordinaryCompletionContext()))return null;
 const refuse=(reason,where=range)=>{throw unsupported('Rust iterator reduction',reason,where);},actual=parser.origins.origin(receiver,path),source=projectRustStaticByteArrayReadOrigin(actual)??actual,element=source.type?.kind==='array'?source.type.element:null,owner=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner);
 const knownReceiver=readRustScalarArrayIndexReceiverError(source.type?.kind??'',element?.kind??'',source.arraySourceModel??'',source.proven===true,source.wrappers.every(layer=>layer==='reference'||layer==='box'),source.arrayElementCopyEvidence===true)==='';
 const knownOwner=Boolean(element&&owner&&owner.arguments.length===0&&(owner.kind==='primitive'||owner.kind==='unit'&&owner.path.length===0)&&readRustArrayLookupElementOwnerError(element.kind,String(element.bits??''),element.signed===true,owner.path.join('::'))==='');
 const receiverError=readRustIteratorReductionReceiverError(knownReceiver,knownOwner);if(receiverError)refuse(receiverError);
 const c=parser.cursor;if(!c.eat('.')||c.peek().kind!=='identifier')refuse('an iterator requires its immediate copied or cloned reduction');
 const adapter=c.next();if(!acceptRustIteratorCopyAdapter(adapter.value))refuse('an iterator requires its immediate copied or cloned reduction',adapter);
 c.expect('(','iterator copy adapter');c.expect(')','iterator copy adapter');if(!c.eat('.')||c.peek().kind!=='identifier')refuse('an iterator requires its complete immediate reduction');
 const consumer=c.next(),operation=readRustIteratorReductionOperation(consumer.value);if(!operation)refuse('the complete standard iterator reduction needs a registered operation',consumer);
 c.expect('(','iterator reduction');let initial=null,initialOrigin=null;
 if(consumer.value==='fold'){initial=parser.expr(path);initialOrigin=parser.origins.origin(initial,path);const error=readRustIteratorReductionAccumulatorError(Boolean(initialOrigin.type)&&initialOrigin.proven===true,parser.origins.immutableProduct(initialOrigin),initialOrigin.wrappers.length);if(error)refuse(error,initial.span);c.expect(',','iterator fold');}
 const itemOrigin={...parser.origins.typeOrigin(element,path),sourceOwner:structuredClone(owner)},resultOrigin=initialOrigin??{...parser.origins.typeOrigin(BOOL,path),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}},parameters=initialOrigin?[initialOrigin,itemOrigin]:[itemOrigin];
 const expected={...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:resultOrigin.type},path),copyEvidence:true,callableParameters:parameters,callableResult:resultOrigin};
 const callback=readRustExpectedCopyClosure(parser,path,expected);c.eat(',');const close=c.expect(')','iterator reduction');range={start:receiver.span?.start??range.start,end:close.end};
 const callable=parser.origins.origin(callback,path),error=readRustIteratorReductionCallbackError(consumer.value,callable.type?.kind??'',callable.type?.params?.length??-1,callable.wrappers.length,(callable.callableParameters??[]).map(origin=>origin.proven===true&&parser.origins.immutableProduct(origin)),callable.callableResult?.proven===true&&Boolean(callable.callableResult.type));if(error)refuse(error,callback.span);
 for(let index=0;index<parameters.length;index+=1)parser.origins.argument(callable.callableParameters[index],parameters[index],callback.span);parser.origins.argument(callable.callableResult,resultOrigin,callback.span);
 const values={receiver,initial,callback},types={receiver:source.type,initial:initialOrigin?.type,callback:callable.type},captures=readRustIteratorReductionCaptureRoles(consumer.value).map(role=>({name:parser.freshProductBinding(),value:values[role],type:types[role],role})),names=new Map(captures.map(capture=>[capture.role,capture.name])),local=role=>({k:'name',path:[names.get(role)],span:range});
 let body={k:'method',object:local('receiver'),op:operation,args:[local('callback'),...(initial?[local('initial')]:[])],span:range};
 for(const capture of captures.toReversed())body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};
 return parser.origins.mark(body,structuredClone(resultOrigin));
}
