// Actual checked receiver/field operands into existing branch/error/value IR.
import {BOOL,FLOAT,STRING} from './types.js';
import {bindLocal} from './check.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readTypeQueryResult} from './frontend-rules.js';
import {readJavaScriptNullishFieldForm,readJavaScriptNullishFieldMessage,
 readJavaScriptNullishFieldCapture} from './javascript-nullish-field-rules.js';
const literal=(type,value)=>({k:'lit',value,type});
export function checkJavaScriptNullishAccess(checker,node,value,env,path,expected){
 if(checker.language!=='JavaScript')return null;
 const member=node.k==='method'?node.op:node.field;
 const form=readJavaScriptNullishFieldForm(value.type.kind,value.type.element?.kind??'');
 if(form==='direct')return null;
 const capture=checker.freshName('ml_nullish_field_receiver'),field=readJavaScriptNullishFieldCapture();
 const capturedType={kind:'record',fields:[{name:field,type:value.type}]};
 const receiver={k:'recordField',object:{k:'var',name:capture,type:capturedType},field,index:0,type:value.type};
 let present,success,type=expected??FLOAT;
 if(form==='guarded'){
  present=value.type.element;
  const binding=checker.freshName('ml_nullish_field_present'),scope=new Map(env);bindLocal(scope,binding,present);
  const object={k:'name',path:[binding],span:node.object.span};
  const source=node.k==='method'?{...node,object}:{k:'field',object,field:member,span:node.span};
  const body=checker.expr(source,scope,path,expected);
  type=body.type;
  success={k:'let',name:binding,value:{k:'unwrapOption',arg:receiver,type:present},body,type};
 }
 const policy=value.type.kind==='unit'?'undefined':value.type.absence;
 const nullMessage=literal(STRING,readJavaScriptNullishFieldMessage('null',member));
 const undefinedMessage=literal(STRING,readJavaScriptNullishFieldMessage('undefined',member));
 let message=policy==='null'?nullMessage:undefinedMessage;
 if(policy==='both'){
  const tag=present?readTypeQueryResult(present.kind)||'object':'undefined';
  const query={k:'optionalTypeOf',arg:receiver,present:tag,type:STRING};
  const nullish={k:'binary',op:'eq',left:query,right:literal(STRING,'object'),domain:STRING,type:BOOL};
  message={k:'if',cond:nullish,then:nullMessage,else:undefinedMessage,type:STRING};
 }
 const failure={k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[message,literal(BOOL,true)],type};
 const body=form==='guarded'?{k:'if',cond:constructJavaScriptOptionalPresence(receiver,present,node.span),then:success,else:failure,type}:failure;
 return {k:'let',name:capture,value:{k:'record',entries:[{name:field,value}],type:capturedType},body,type};
}

export function checkJavaScriptNullishField(checker,node,value,env,path,expected){return checkJavaScriptNullishAccess(checker,node,value,env,path,expected);}
