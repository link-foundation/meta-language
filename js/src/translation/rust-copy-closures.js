import {readRustNominalBindingPattern,rustNominalBindingNames} from './rust-records.js';
import {beginRustOwnedRecordStateCallable,endRustOwnedRecordStateCallable} from './rust-owned-record-state.js';
import {beginRustFnMutSourceEnvironment,finishRustFnMutSourceEnvironment,endRustFnMutSourceEnvironment,readRustFnMutSourceClosureParameterRole} from './rust-fnmut-source-environments.js';
import {withRustStandardSumSourceContext,readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {registerRustOwnedBinding,checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {captureRustOwnedStandardSumMethodValue} from './rust-owned-standard-sum-methods.js';
import {cloneRustCallableTypeOrigin} from './rust-function-pointers.js';
import {assignRustCopyClosureLexicalBinding,readRustCopyClosureCaptureBorrows,refuseRustCopyClosureReturnedBorrows,validateRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {readRustOwnedStandardSumCallbackClosureHeaderError,readRustOwnedStandardSumCallbackMoveCaptureError,acceptRustOwnedStandardSumCallbackMoveClosureCopy} from './rust-owned-standard-sum-callback-rules.js';
import {readRustCopyClosureParameterHeaderError,testRustCopyClosureDeferredResult,readRustCopyClosureDeferredResultError,readRustCopyClosureResultContextRole,readRustCopyClosureLiteralResultError,testRustCopyClosureResultBinding,acceptRustCopyClosureValue,readRustCopyClosureLoanCaptureError} from './rust-copy-closure-rules.js';
import {readRustCopyClosureResultContext} from './rust-copy-closure-parameter-rules.js';
import {acceptRustCopyClosureParameterOrigin} from './rust-copy-closure-parameter-origins.js';
import {requireRustNonconstantBinding,readRustSourceValueName} from './rust-source-constants.js';
import {readRustPrimitiveFunctionValue} from './rust-primitive-function-values.js';
import {beginRustStandardStringViewArgument,endRustStandardStringViewArgument,checkRustStandardStringViewArgument} from './rust-standard-string-views.js';
// Actual source types/scopes into the existing shared callable representation.
import {unsupported} from './diagnostics.js';
import {readCallableStatementCaptures} from './callable-source.js';
import {readRustOrdinaryFunctionCompletion} from './rust-ordinary-completions.js';
import {readRustCopyClosureHeaderError,readRustCopyClosureCaptureError,acceptRustCopyClosureInvocation,readRustCopyClosureParameterForm,readRustCopyClosureReferencePatternDepth,readRustCopyClosureReferencePatternError,readRustCopyClosurePatternBindingError} from './rust-copy-closure-rules.js';
import {acceptRustContextualCopyClosure,readRustContextualCopyClosureHeaderError,findRustContextualClosureParenthesisCount,acceptRustContextualCopyPointerResultBlock} from './rust-contextual-closure-rules.js';
export function readRustCopyClosureResultContextError(node,result){
 const pending=[{value:node,scope:null}],scopes=[],visited=new WeakMap();while(pending.length){const {value,scope}=pending.pop(),seen=visited.get(value)??new Set();if(seen.has(scope))continue;seen.add(scope);visited.set(value,seen);
  const error=readRustCopyClosureLiteralResultError(value.k,value.k==='unary'&&value.op==='neg',Boolean(value.type),result.type?.kind??'',result.type?.signed===true);if(error)return error;
  const role=readRustCopyClosureResultContextRole(value.k,value.op??''),push=child=>pending.push({value:child,scope});
  if(role==='binding'){const frame=scopes.length;scopes.push({name:value.name,value:value.value,parent:scope});pending.push({value:value.body,scope:frame});}
  else if(role==='name'){for(let frame=scope;frame!==null;frame=scopes[frame].parent){const entry=scopes[frame];if(testRustCopyClosureResultBinding(value.path,entry.name)){pending.push({value:entry.value,scope:entry.parent});break;}}}
  else if(role==='branches'){push(value.then);push(value.else);}else if(role==='rows')for(const row of value.rows)push(row.body);else if(role==='operand')push(value.arg);else if(role==='operands'){push(value.left);push(value.right);}else if(role!=='leaf')throw new TypeError('unknown shared closure result context role');
 }return '';
}
export function readRustExpectedCopyClosure(parser,path,expected,parametersOnly=false,deferResult=false){
 const primitive=readRustPrimitiveFunctionValue(parser,path,expected);if(primitive)return primitive;
 const c=parser.cursor,kind=expected?.type?.kind??'',copy=parser.origins.immutableProduct(expected),role=['|','||','move'].find(value=>c.is(value))??'';
 if(acceptRustContextualCopyClosure(kind,copy,role))return readRustCopyClosure(parser,path,expected,parametersOnly,deferResult);
 if(kind==='function'&&copy&&c.is('(')){
  let count=64;
  for(;;){const tokens=c.tokens.slice(c.index,c.index+count),syntax=tokens.map((_,index)=>['(','|','||','move'].find(value=>c.is(value,index))??''),depth=findRustContextualClosureParenthesisCount(syntax);
   if(depth>0){for(let index=0;index<depth;index+=1)c.next();const node=readRustCopyClosure(parser,path,expected,parametersOnly,deferResult);for(let index=0;index<depth;index+=1)c.expect(')','contextual closure');return node;}
   if(depth!==-2||tokens.length<count)break;count*=2;
  }
 }
 return withRustStandardSumSourceContext(parser,expected,()=>parser.expr(path));
}
// Actual parameter context with no invented callable result type. Deferred
// literal results retain the existing private lambda protocol and must later
// pass finishRustDeferredCopyClosure with an independently supplied result.
export function readRustParameterOnlyCopyClosure(parser,path,parameters,deferResult=false){
 if(parameters.some(origin=>!origin?.type||!acceptRustCopyClosureParameterOrigin(parser,origin)))throw unsupported('Rust parameter-only Copy closure','each parameter needs its actual admitted Copy source origin',parser.cursor.peek());
 const expected={...parser.origins.unknownOrigin(path),callableParameters:parameters.map(origin=>parser.origins.copy(origin)),callableResult:null};
 const primitive=readRustPrimitiveFunctionValue(parser,path,expected);if(primitive)return primitive;
 const c=parser.cursor;
 if(['|','||','move'].some(role=>c.is(role)))return readRustCopyClosure(parser,path,expected,true,deferResult);
 if(c.is('(')){
  let count=64;for(;;){const tokens=c.tokens.slice(c.index,c.index+count),syntax=tokens.map((_,index)=>['(','|','||','move'].find(value=>c.is(value,index))??''),depth=findRustContextualClosureParenthesisCount(syntax);
   if(depth>0){for(let at=0;at<depth;at+=1)c.next();const node=readRustCopyClosure(parser,path,expected,true,deferResult);for(let at=0;at<depth;at+=1)c.expect(')','parameter-only closure');return node;}
   if(depth!==-2||tokens.length<count)break;count*=2;
  }
 }
 return withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));
}
export function readRustCopyPointerResultBlock(parser,path,expected){
 const c=parser.cursor,role=['|','||','move','(','return'].find(value=>c.is(value,1))??'';
 if(!parser.ordinaryCompletionContext()||!acceptRustContextualCopyPointerResultBlock(expected?.type?.kind??'',parser.origins.immutableProduct(expected),c.is('{'),role))return null;
 c.expect('{','function pointer result');const returned=Boolean(c.eat('return')),value=readRustExpectedCopyClosure(parser,path,expected);
 if(returned)while(c.eat(';')){}
 c.expect('}','function pointer result');parser.origins.argument(parser.origins.origin(value,path),expected,value.span);return value;
}
export function readRustContextualCallArguments(parser,callee,path){
 let parameters=null,result=null;
 if(callee.k==='name'&&!(callee.path.length===1&&parser.origins.scope.has(callee.path[0]))){const entry=parser.origins.lookupValueOwner(readRustSourceValueName(parser,callee.path,path),path);if(entry?.k==='fn'||entry?.k==='ctor'){parameters=entry.k==='fn'?entry.parameters:entry.fields;result=entry.result;}}
 if(parameters===null){const origin=parser.origins.origin(callee,path);result??=origin.callableResult??null;parameters=origin.type?.kind==='function'?origin.type.params.map((type,index)=>origin.callableParameters?.[index]??({...parser.origins.typeOrigin(type,path),...(type.kind==='function'?{copyEvidence:true}:{})})):[];}
 const args=[],c=parser.cursor;
 while(!c.is(')')){const frame=beginRustStandardStringViewArgument(parser,path,parameters[args.length],result);let value;try{value=readRustExpectedCopyClosure(parser,path,parameters[args.length]);}finally{endRustStandardStringViewArgument(parser,frame);}const argument=checkRustStandardStringViewArgument(parser,frame,value,path);parser.origins.prepareOwnedArgument(argument,parameters[args.length],path);args.push(argument);if(!c.eat(','))break;}
 return args;
}
export function readRustOwnedCallbackClosure(parser,path,parameters,result=null){
 const expected=result?{...parser.origins.typeOrigin({kind:'function',params:parameters.map(origin=>origin.type),ret:result.type},path),copyEvidence:true,callableParameters:parameters.map(cloneRustCallableTypeOrigin),callableResult:cloneRustCallableTypeOrigin(result)}:{...parser.origins.unknownOrigin(path),callableParameters:parameters.map(cloneRustCallableTypeOrigin),callableResult:null};
 const c=parser.cursor;
 if(['|','||','move'].some(role=>c.is(role)))return readRustCopyClosure(parser,path,expected,result===null,false,true);
 if(c.is('(')){
  let count=64;for(;;){const tokens=c.tokens.slice(c.index,c.index+count),syntax=tokens.map((_,index)=>['(','|','||','move'].find(value=>c.is(value,index))??''),depth=findRustContextualClosureParenthesisCount(syntax);
   if(depth>0){for(let at=0;at<depth;at+=1)c.next();const value=readRustCopyClosure(parser,path,expected,result===null,false,true);for(let at=0;at<depth;at+=1)c.expect(')','owned-domain callback');return value;}
   if(depth!==-2||tokens.length<count)break;count*=2;
  }
 }
 return withRustStandardSumSourceContext(parser,null,()=>parser.expr(path));
}
export function readRustCopyClosure(parser,path,expected=null,parametersOnly=false,deferResult=false,ownedDomains=false){
 const c=parser.cursor,start=c.peek(),parameters=[],origins=[],bindings=[],patterns=[],patternNames=[],explicitMove=Boolean(c.eat('move'));
 if(!c.eat('||')){
  c.expect('|','closure');
  while(!c.is('|')){
   const token=c.peek();let referenceDepth=0;for(;;){const depth=readRustCopyClosureReferencePatternDepth(c.peek().value);if(depth===0)break;referenceDepth+=depth;c.next();}
   const nominal=readRustNominalBindingPattern(parser),role=['mut','ref','&','_','('].find(value=>c.is(value))??'binding';
   const form=nominal?'product':readRustCopyClosureParameterForm(role);
   if(form==='unsupported')throw unsupported('Rust closure parameter','a closure needs a named direct immutable typed parameter',token);
   let name,pattern=null;
   if(form==='product'){pattern=nominal??parser.productBindingPattern();name=parser.freshProductBinding();}
   else if(form==='wildcard'){c.next();name=parser.freshProductBinding();}
   else{name=c.identifier('closure parameter').value;requireRustNonconstantBinding(parser,name,path,token);}
   const names=[];const pending=pattern?[pattern]:[{k:'name',name,span:token}];while(pending.length){const current=pending.pop();if(current.k==='nominal')for(const entry of rustNominalBindingNames(parser,current.pattern))pending.push({k:'name',...entry});else if(current.k==='name')names.push(current);else for(const child of current.items??[])pending.push(child);}
   for(const entry of names){requireRustNonconstantBinding(parser,entry.name,path,entry.span);patternNames.push(entry.name);}
   const bindingError=readRustCopyClosurePatternBindingError(patternNames);if(bindingError)throw unsupported('Rust Copy closure',bindingError,token);
   patterns.push(pattern);
   let type,origin;
   if(c.eat(':')){type=parser.type();origin=parser.origins.type;}
   else{
    type=parametersOnly&&expected?.type==null?expected?.callableParameters?.[parameters.length]?.type:expected?.type?.params[parameters.length];
    if(!type){c.expect(':','closure parameter');throw parser.fail('a closure parameter needs its source type');}
    origin=parser.origins.copy(expected?.callableParameters?.[parameters.length])??parser.origins.typeOrigin(type,path);
   }
   const referenceError=readRustCopyClosureReferencePatternError(referenceDepth,origin.wrappers.length,origin.wrappers.every(layer=>layer==='reference'),acceptRustCopyClosureParameterOrigin(parser,origin));if(referenceError)throw unsupported('Rust closure parameter',referenceError,token);
   const binding=parser.origins.copy(origin);binding.wrappers=binding.wrappers.slice(referenceDepth);binding.sourceLifetimes=binding.sourceLifetimes?.slice(referenceDepth);bindings.push(binding);
   parameters.push({name,type,span:{start:token.start,end:c.peek(-1).end}});origins.push(origin);
   if(!c.eat(','))break;
  }
  c.expect('|','closure');
 }
 const written=Boolean(c.eat('->'));let ret=written?parser.type():readRustCopyClosureResultContext(written,parametersOnly)==='inferred'?null:expected?.type?.ret??null,resultOrigin=written?parser.origins.type:ret?parser.origins.copy(expected?.callableResult)??parser.origins.typeOrigin(ret,path):null;
 const headerRoles=ownedDomains?origins.map((origin,index)=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile||readRustFnMutSourceClosureParameterRole(parser,origin,index)):[];
 if(written){const reason=ownedDomains?readRustOwnedStandardSumCallbackClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),headerRoles,readRustStandardSumOwnershipFacts(parser.origins,resultOrigin).profile):readRustCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin)),written,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
 const ownership=parser.origins.snapshotOwnedBindings(),temporaries=new Set(parser.origins.sourceTemporaryBindings);
 const recordStateCallable=beginRustOwnedRecordStateCallable(parser);
 const fnMutEnvironment=beginRustFnMutSourceEnvironment(parser);let fnMutNode=null;
 const outer=parser.origins.enter();parameters.forEach((parameter,index)=>parser.origins.scope.set(parameter.name,assignRustCopyClosureLexicalBinding(parser.origins,registerRustOwnedBinding(parser.origins,bindings[index]))));
 let body;const projections=[],outerSumReturn=parser.standardSumReturnOrigin;
 parser.standardSumReturnOrigin=resultOrigin;
 try{
  for(let index=0;index<patterns.length;index+=1)if(patterns[index])parser.lowerProductBinding(patterns[index],{k:'name',path:[parameters[index].name],span:parameters[index].span},path,projections);
  body=withRustStandardSumSourceContext(parser,resultOrigin,()=>written||c.is('{')?readRustCopyPointerResultBlock(parser,path,resultOrigin)??(ret&&parser.ordinaryCompletionSelected()?readRustOrdinaryFunctionCompletion(parser,path,ret,resultOrigin):parser.block(path,!fnMutEnvironment)):parser.expr(path));
  if(!ret){resultOrigin=parser.origins.origin(body,path);ret=resultOrigin.type??null;}
  if(!written){const deferred=testRustCopyClosureDeferredResult(deferResult,written,ret!==null),reason=ownedDomains?readRustOwnedStandardSumCallbackClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),headerRoles,readRustStandardSumOwnershipFacts(parser.origins,resultOrigin).profile):deferred?readRustCopyClosureParameterHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin))):readRustContextualCopyClosureHeaderError(parser.ordinaryCompletionContext(),parameters.map(param=>param.name),origins.map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin)),ret!==null,parser.origins.immutableProduct(resultOrigin));if(reason)throw unsupported('Rust Copy closure',reason,start);}
  if(ret){const error=readRustCopyClosureResultContextError(body,resultOrigin);if(error)throw unsupported('Rust Copy closure',error,body.span);parser.origins.argument(parser.origins.origin(body,path),resultOrigin,body.span);}
  refuseRustCopyClosureReturnedBorrows(parser.origins,parser.origins.origin(body,path),body.span);
  if(ownedDomains&&readRustStandardSumOwnershipFacts(parser.origins,parser.origins.origin(body,path)).profile==='owned')captureRustOwnedStandardSumMethodValue(parser,body,path);
  parser.origins.expireOwnedBindings(ownership,parser.origins.origin(body,path));
  const bodyOrigin=parser.origins.origin(body,path);for(const projection of projections.toReversed())body={k:'let',name:projection.name,value:projection.value,...(projection.type?{type:projection.type}:{}),body,span:projection.span};parser.origins.mark(body,bodyOrigin);
  if(fnMutEnvironment)fnMutNode=finishRustFnMutSourceEnvironment(parser,fnMutEnvironment,parameters,resultOrigin,body,path,{start:start.start,end:c.peek(-1).end},outer);
 }
 finally{parser.origins.scope=outer;parser.standardSumReturnOrigin=outerSumReturn;parser.origins.restoreOwnedBindings(ownership);parser.origins.sourceTemporaryBindings=temporaries;endRustFnMutSourceEnvironment(parser,fnMutEnvironment);endRustOwnedRecordStateCallable(parser,recordStateCallable);}
 if(fnMutNode)return fnMutNode;
 const {captures,assignedCaptures}=readCallableStatementCaptures([{s:'return',expr:body}],parameters.map(param=>param.name),outer.keys());
 const captureError=readRustCopyClosureCaptureError(parser.origins.collecting,captures.map(name=>parser.origins.immutableProduct(outer.get(name))),assignedCaptures.length);if(captureError)throw unsupported('Rust Copy closure',captureError,start);
 const ordinaryCaptured=ownedDomains?[]:captures.map(name=>outer.get(name)),ordinaryCopies=ordinaryCaptured.map(origin=>parser.origins.immutableProduct(origin)),ordinaryLoans=ordinaryCaptured.map(origin=>origin?.sourceBorrowedPlace===true||origin?.sourceBindingIdentity!=null||origin?.sourcePlaceBindingIdentity!=null||(origin?.sourceLoanBindingIdentities??[]).length>0||(origin?.sourcePlaceLoanBindingIdentities??[]).length>0);
 if(!ownedDomains){for(const origin of ordinaryCaptured)checkRustOwnedOrigin(parser.origins,origin,start);const error=readRustCopyClosureLoanCaptureError(captures.length,ordinaryLoans);if(error)throw unsupported('Rust Copy closure',error,start);}
 const captured=ownedDomains?captures.map(name=>outer.get(name)):[],captureRoles=captured.map(origin=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile),captureIdentities=captured.map(origin=>readRustStandardSumSourceIdentity(parser,origin)??''),captureLayers=captured.map(origin=>origin?.wrappers.length??-1),captureBorrowed=captured.map(origin=>origin?.sourceBorrowedPlace===true||origin?.sourceBindingIdentity!=null||origin?.sourcePlaceBindingIdentity!=null||(origin?.sourceLoanBindingIdentities??[]).length>0||(origin?.sourcePlaceLoanBindingIdentities??[]).length>0);
 if(ownedDomains){for(const origin of captured)checkRustOwnedOrigin(parser.origins,origin,start);const error=readRustOwnedStandardSumCallbackMoveCaptureError(explicitMove,captures,captureRoles,captureIdentities,captureLayers,captureBorrowed,assignedCaptures.length);if(error)throw unsupported('Rust owned-domain closure',error,start);}
 const span={start:start.start,end:c.peek(-1).end},node={k:'lambda',params:parameters,ret,captures,body,span};
 const sourceValueBorrowBindings=readRustCopyClosureCaptureBorrows(parser.origins,captures.map(name=>outer.get(name)),explicitMove,span);
 const copyEvidence=ownedDomains?acceptRustOwnedStandardSumCallbackMoveClosureCopy(origins.map(origin=>readRustStandardSumOwnershipFacts(parser.origins,origin).profile),readRustStandardSumOwnershipFacts(parser.origins,resultOrigin).profile,explicitMove,captures,captureRoles,captureIdentities,captureLayers,captureBorrowed,assignedCaptures.length):acceptRustCopyClosureValue(parser.ordinaryCompletionContext(),parameters.map(parameter=>parameter.name),origins.map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin)),Boolean(ret&&resultOrigin?.proven&&resultOrigin.type),parser.origins.immutableProduct(resultOrigin),ordinaryCopies,ordinaryLoans,assignedCaptures.length);
 return parser.origins.mark(node,{...(ret?parser.origins.typeOrigin({kind:'function',params:parameters.map(param=>param.type),ret},path):parser.origins.unknownOrigin(path)),copyEvidence,sourceValueBorrowBindings,sourceValueFieldBorrowBindings:null,copyClosureSpan:span,copyClosureCaptureCount:captures.length,callableParameters:origins.map(cloneRustCallableTypeOrigin),callableResult:cloneRustCallableTypeOrigin(resultOrigin)});
}
// Complete the same actual lambda after a sibling argument supplies its type.
export function finishRustDeferredCopyClosure(parser,node,result,path){
 validateRustCopyClosureValueBorrows(parser.origins,parser.origins.origin(node,path),node.span);
 const prior=parser.origins.origin(node,path),error=readRustCopyClosureDeferredResultError(node.k,node.ret==null&&prior.type===null&&Boolean(prior.copyClosureSpan)&&Boolean(node.span)&&prior.copyClosureSpan?.start===node.span?.start&&prior.copyClosureSpan?.end===node.span?.end,result?.proven===true&&Boolean(result.type),parser.origins.immutableProduct(result));
 if(error)throw unsupported('Rust Copy closure',error,node.span);
 const contextError=readRustCopyClosureResultContextError(node.body,result);if(contextError)throw unsupported('Rust Copy closure',contextError,node.body.span);
 parser.origins.argument(parser.origins.origin(node.body,path),result,node.body.span);
 const observed=readCallableStatementCaptures([{s:'return',expr:node.body}],node.params.map(parameter=>parameter.name),parser.origins.scope.keys()),captured=observed.captures.map(name=>parser.origins.scope.get(name)),copies=captured.map(origin=>parser.origins.immutableProduct(origin)),loans=captured.map(origin=>origin?.sourceBorrowedPlace===true||origin?.sourceBindingIdentity!=null||origin?.sourcePlaceBindingIdentity!=null||(origin?.sourceLoanBindingIdentities??[]).length>0||(origin?.sourcePlaceLoanBindingIdentities??[]).length>0);
 for(const origin of captured)checkRustOwnedOrigin(parser.origins,origin,node.span);
 const captureError=readRustCopyClosureCaptureError(parser.origins.collecting,copies,observed.assignedCaptures.length)||readRustCopyClosureLoanCaptureError(observed.captures.length,loans);if(captureError)throw unsupported('Rust Copy closure',captureError,node.span);
 const copyEvidence=prior.copyClosureCaptureCount===node.captures.length&&observed.captures.length===node.captures.length&&node.captures.every((name,index)=>name===observed.captures[index])&&acceptRustCopyClosureValue(parser.ordinaryCompletionContext(),node.params.map(parameter=>parameter.name),(prior.callableParameters??[]).map(origin=>acceptRustCopyClosureParameterOrigin(parser,origin)),result.proven&&Boolean(result.type),parser.origins.immutableProduct(result),copies,loans,observed.assignedCaptures.length);
 if(!copyEvidence)throw unsupported('Rust Copy closure','the deferred callable needs its actual complete header and immutable capture value witness',node.span);
 const ret=result.type,finished={...node,ret};return parser.origins.mark(finished,{...parser.origins.typeOrigin({kind:'function',params:node.params.map(param=>param.type),ret},path),copyEvidence,sourceValueBorrowBindings:[...(prior.sourceValueBorrowBindings??[])],sourceValueFieldBorrowBindings:null,copyClosureSpan:prior.copyClosureSpan,copyClosureCaptureCount:prior.copyClosureCaptureCount,callableParameters:prior.callableParameters?.map(cloneRustCallableTypeOrigin)??[],callableResult:cloneRustCallableTypeOrigin(result)});
}
export function readRustCopyClosureCall(parser,fn,args,path,span){
 const origin=parser.origins.origin(fn,path);if(!acceptRustCopyClosureInvocation(origin.type?.kind??''))return null;
 args.forEach((arg,index)=>{const parameter=origin.type.params[index];if(parameter?.kind==='function')parser.origins.argument(parser.origins.origin(arg,path),{...(parser.origins.copy(origin.callableParameters?.[index])??parser.origins.typeOrigin(parameter,path)),copyEvidence:true},arg.span??span);else if(origin.callableParameters?.[index])parser.origins.argument(parser.origins.origin(arg,path),origin.callableParameters[index],arg.span??span);else parser.origins.direct(arg,path,arg.span??span);});
 const result=parser.origins.copy(origin.callableResult)??parser.origins.typeOrigin(origin.type.ret,path);if(origin.type.ret.kind==='function')result.copyEvidence=true;
 return parser.origins.mark({k:'app',fn,args,span},result);
}
