// Actual source question-mark transport into the ordinary callable completion
// tree. Private markers are consumed before checked semantic IR is admitted.
import {unsupported} from './diagnostics.js';
import {checkRustOwnedOrigin,registerRustOwnedBinding} from './rust-owned-bindings.js';
import {readRustStandardSumOwnershipFacts,projectRustStandardSumPatternField,createRustStandardSumConstructorOrigin} from './rust-standard-sum-ownership.js';
import {captureRustOwnedStandardSumMethodValue,createRustOwnedStandardSumMethodResult} from './rust-owned-standard-sum-methods.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {acceptRustStandardSumQuestionContext,acceptRustStandardSumOwnedQuestionContext,readRustStandardSumSourceConstructors,readRustStandardSumQuestionBranch,readRustStandardSumPrimitiveQuestionConversionPlan,acceptRustStandardSumPrimitiveQuestionCompletion} from './rust-standard-sum-source-rules.js';
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
import {projectRustPrimitiveFrom} from './rust-primitive-from.js';

const name=(path,span)=>({k:'name',path,span});
const refuse=(message,span)=>{throw unsupported('Rust question-mark completion',message,span);};
function primitiveError(parser,origin){
 if(!origin?.type)return null;const owner=origin.sourceOwner,type=origin.type;
 if(!acceptRustPrimitiveFromSource(type.kind,String(type.bits??''),type.signed===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,origin.proven===true,origin.wrappers.length))return null;
 const identity=readRustStandardSumSourceIdentity(parser,origin),canonical={...parser.origins.typeOrigin(type,origin.path),sourceOwner:structuredClone(owner)};
 if(!identity||identity!==readRustStandardSumSourceIdentity(parser,canonical))return null;
 return owner.path.join('::');
}
function primitiveConversion(parser,operand,target){const source=operand.standardSumSource,result=target.standardSumSource,from=primitiveError(parser,source.arguments[1]),to=primitiveError(parser,result.arguments[1]);return readRustStandardSumPrimitiveQuestionConversionPlan(source.owner,result.owner,parser.ordinaryCompletionContext(),readRustStandardSumOwnershipFacts(parser.origins,operand).profile,operand.wrappers.length,parser.origins.immutableProduct(operand),from!==null,to!==null,from??'',to??'',parser.sourceVersionMinor);}

/** Called by the actual postfix parser at the owned question-mark token. */
export function readRustStandardSumTryExpression(parser,operand,path,token){
 if(!parser.ordinaryCompletionContext())refuse('question-mark propagation requires the ordinary callable completion interpretation',token);
 const actual=parser.origins.origin(operand,path),origin=readRustStandardSumSourceOrigin(parser,actual);
 if(!origin)refuse('the operand needs its actual direct standard Option or Result source owner',operand.span);
 const callable=readRustStandardSumSourceOrigin(parser,parser.standardSumReturnOrigin);
 if(!callable||callable.wrappers.length!==0)refuse('the nearest actual callable must return its own direct standard sum',token);
 const sum=origin.standardSumSource,target=callable.standardSumSource,role=readRustStandardSumOwnershipFacts(parser.origins,origin).profile;
 const error=owner=>owner.owner==='std::result::Result'?readRustStandardSumSourceIdentity(parser,owner.arguments[1])??'':'';
 const conversion=primitiveConversion(parser,origin,callable),owned=!parser.headerDiscovery&&(acceptRustStandardSumOwnedQuestionContext(sum.owner,target.owner,true,role,origin.wrappers.length,error(sum),error(target))||conversion[0]==='owned');
 if(!parser.headerDiscovery&&!owned&&conversion.length===0&&!acceptRustStandardSumQuestionContext(sum.owner,target.owner,true,origin.wrappers.length===0&&parser.origins.immutableProduct(origin),error(sum),error(target)))refuse('propagation needs identical errors or an actual standard primitive From implementation; custom Try, FromResidual and other conversions require their complete interpretation',token);
 const span={start:operand.span?.start??token.start,end:token.end};
 parser.cursor.expect('?','question-mark completion');
 checkRustOwnedOrigin(parser.origins,actual,operand.span);
 const captured=owned?captureRustOwnedStandardSumMethodValue(parser,operand,path):actual;
 const consumed=!owned||parser.origins.consumeOwnedBinding(captured,operand.span);
 let payloadOrigin=projectRustStandardSumPatternField(parser.origins,captured,sum.arguments[0],false);
 if(owned&&readRustStandardSumOwnershipFacts(parser.origins,payloadOrigin).profile==='owned'){
  // This identity exists on the successful continuation only. It is the
  // selected field of the actual consumed constructor, not a type-only owner.
  payloadOrigin=registerRustOwnedBinding(parser.origins,createRustOwnedStandardSumMethodResult(parser,payloadOrigin,consumed,true));
  parser.origins.sourceTemporaryBindings.add(payloadOrigin.sourceBindingIdentity);
 }
 const payload=parser.origins.mark(name([parser.freshProductBinding()],span),payloadOrigin);
 return parser.origins.mark({k:'rustStandardSumTry',operand,payload,operandOrigin:origin,operandValueOrigin:captured,callableResultOrigin:callable,ownedOperandConsumed:owned&&consumed,span},payloadOrigin);
}

/** Build both alternatives from actual validated source owners. The operand
 * lives in exactly one ordered binding; only its matching branch executes.
 * `continued` and `returned` are the existing callable completion algebra. */
export function lowerRustStandardSumTryCompletion(parser,attempt,path,sourceResult,continued,returned){
 const operand=readRustStandardSumSourceOrigin(parser,attempt.operandOrigin);
 const target=readRustStandardSumSourceOrigin(parser,sourceResult);
 if(!operand||!target||target.wrappers.length!==0)refuse('propagation needs both actual operand and direct enclosing callable sum owners',attempt.span);
 const selectedIdentity=readRustStandardSumSourceIdentity(parser,attempt.callableResultOrigin),targetIdentity=readRustStandardSumSourceIdentity(parser,target);
 if(!selectedIdentity||selectedIdentity!==targetIdentity)refuse('propagation cannot cross its actual nearest callable source owner',attempt.span);
 const source=operand.standardSumSource,result=target.standardSumSource;
 const sourceError=source.owner==='std::result::Result'?readRustStandardSumSourceIdentity(parser,source.arguments[1]):'';
 const resultError=result.owner==='std::result::Result'?readRustStandardSumSourceIdentity(parser,result.arguments[1]):'';
 const directCopy=operand.proven===true&&operand.copyEvidence===true&&operand.wrappers.length===0;
 const conversion=primitiveConversion(parser,operand,target),converted=conversion.length>0&&acceptRustStandardSumPrimitiveQuestionCompletion(conversion[0],attempt.ownedOperandConsumed===true);
 const owned=attempt.ownedOperandConsumed===true&&acceptRustStandardSumOwnedQuestionContext(source.owner,result.owner,true,readRustStandardSumOwnershipFacts(parser.origins,operand).profile,operand.wrappers.length,sourceError??'',resultError??'');
 if(!owned&&!converted&&!acceptRustStandardSumQuestionContext(source.owner,result.owner,true,directCopy,sourceError??'',resultError??''))refuse('propagation needs identical errors or an actual standard primitive From implementation; custom Try, FromResidual and other conversions require their complete interpretation',attempt.span);
 const captured=parser.freshProductBinding(),cases=[];
 for(const variant of readRustStandardSumSourceConstructors(source.owner)){
  const branch=readRustStandardSumQuestionBranch(source.owner,variant);
  if(branch==='continue field'){
   cases.push({pattern:{k:'ctor',path:source.constructorPaths[variant],binds:[attempt.payload.path[0]]},body:continued});
   continue;
  }
  if(branch!=='return absent'&&branch!=='return error field')refuse('the actual standard sum branch has no represented completion',attempt.span);
  const error=branch==='return error field'?parser.freshProductBinding():null;
  const saved=[];let projected=null,convertedName=null;
  const bind=(binding,origin)=>{saved.push({binding,present:parser.origins.scope.has(binding),previous:parser.origins.scope.get(binding)});parser.origins.scope.set(binding,origin);};
  try{
   const args=[];
   if(error!==null){
    let field=projectRustStandardSumPatternField(parser.origins,attempt.operandValueOrigin,source.arguments[1],false);
    if(attempt.ownedOperandConsumed&&readRustStandardSumOwnershipFacts(parser.origins,field).profile==='owned')field=registerRustOwnedBinding(parser.origins,createRustOwnedStandardSumMethodResult(parser,field,true,true));
    bind(error,field);args.push(parser.origins.mark(name([error],null),field));
    if(converted){projected=projectRustPrimitiveFrom(parser,args[0],conversion[1],null,path);convertedName=parser.freshProductBinding();bind(convertedName,result.arguments[1]);args[0]=parser.origins.mark(name([convertedName],null),result.arguments[1]);}
   }
   const failureValue={k:'app',fn:name(result.constructorPaths[variant],null),args,span:null},produced=createRustStandardSumConstructorOrigin(parser,failureValue,target,args,path,null);parser.origins.mark(failureValue,produced);
   const body=returned(failureValue,produced);
   cases.push({pattern:{k:'ctor',path:source.constructorPaths[variant],binds:error===null?[]:[error]},body:projected?{k:'let',name:convertedName,type:result.arguments[1].type,value:projected,body,span:null}:body});
  }finally{for(const entry of saved.toReversed())if(entry.present)parser.origins.scope.set(entry.binding,entry.previous);else parser.origins.scope.delete(entry.binding);}
 }
 return {k:'let',name:captured,value:attempt.operand,body:{k:'match1',scrutinee:name([captured],attempt.span),cases,span:attempt.span},span:attempt.span};
}

/** Reject contexts not yet lifted into an ordinary callable continuation.
 * This final admission boundary prevents private markers reaching emitters. */
export function refuseRustStandardSumPrivateCompletions(root){
 const pending=[root],seen=new Set();
 while(pending.length){
  const value=pending.pop();
  if(value===null||typeof value!=='object'||seen.has(value))continue;
  if(seen.size>=100000)refuse('source completion admission exceeds its finite node budget',value.span);
  seen.add(value);
  if(value.k==='rustStandardSumTry')refuse('this question-mark position requires its complete ordered expression interpretation and actual nearest callable ownership',value.span);
  for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);
 }
}
