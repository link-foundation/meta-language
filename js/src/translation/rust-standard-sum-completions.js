// Actual source question-mark transport into the ordinary callable completion
// tree. Private markers are consumed before checked semantic IR is admitted.
import {unsupported} from './diagnostics.js';
import {readRustStandardSumSourceOrigin,readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {acceptRustStandardSumQuestionContext,readRustStandardSumSourceConstructors,readRustStandardSumQuestionBranch} from './rust-standard-sum-source-rules.js';

const name=(path,span)=>({k:'name',path,span});
const refuse=(message,span)=>{throw unsupported('Rust question-mark completion',message,span);};

/** Called by the actual postfix parser at the owned question-mark token. */
export function readRustStandardSumTryExpression(parser,operand,path,token){
 if(!parser.ordinaryCompletionContext())refuse('question-mark propagation requires the ordinary callable completion interpretation',token);
 const origin=readRustStandardSumSourceOrigin(parser,parser.origins.origin(operand,path));
 if(!origin)refuse('the operand needs its actual direct standard Option or Result source owner',operand.span);
 const callable=readRustStandardSumSourceOrigin(parser,parser.standardSumReturnOrigin);
 if(!callable||callable.wrappers.length!==0)refuse('the nearest actual callable must return its own direct standard sum',token);
 const sum=origin.standardSumSource,payloadOrigin=sum.arguments[0];
 const span={start:operand.span?.start??token.start,end:token.end};
 parser.cursor.expect('?','question-mark completion');
 const payload=parser.origins.mark(name([parser.freshProductBinding()],span),payloadOrigin);
 return parser.origins.mark({k:'rustStandardSumTry',operand,payload,operandOrigin:origin,callableResultOrigin:callable,span},payloadOrigin);
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
 if(!acceptRustStandardSumQuestionContext(source.owner,result.owner,true,directCopy,sourceError??'',resultError??''))refuse('only direct standard sums of the same family and exactly identical Result error owners can propagate; custom Try, FromResidual and error conversions require their complete interpretation',attempt.span);
 const captured=parser.freshProductBinding(),cases=[];
 for(const variant of readRustStandardSumSourceConstructors(source.owner)){
  const branch=readRustStandardSumQuestionBranch(source.owner,variant);
  if(branch==='continue field'){
   cases.push({pattern:{k:'ctor',path:source.constructorPaths[variant],binds:[attempt.payload.path[0]]},body:continued});
   continue;
  }
  if(branch!=='return absent'&&branch!=='return error field')refuse('the actual standard sum branch has no represented completion',attempt.span);
  const error=branch==='return error field'?parser.freshProductBinding():null;
  const args=error===null?[]:[parser.origins.mark(name([error],attempt.span),source.arguments[1])];
  const failureValue=parser.origins.mark({k:'app',fn:name(result.constructorPaths[variant],attempt.span),args,span:attempt.span},target);
  cases.push({pattern:{k:'ctor',path:source.constructorPaths[variant],binds:error===null?[]:[error]},body:returned(failureValue,target)});
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
