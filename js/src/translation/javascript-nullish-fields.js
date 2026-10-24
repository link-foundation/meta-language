import {unsupported} from './diagnostics.js';
import {materializeJavaScriptPrimitiveText} from './javascript-string-search.js';
// Actual checked receiver/field operands into existing branch/error/value IR.
import {BOOL,FLOAT,INT,STRING} from './types.js';
import {bindLocal,coerce} from './check.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readTypeQueryResult} from './frontend-rules.js';
import {readJavaScriptNullishFieldForm,readJavaScriptNullishFieldMessage,
 readJavaScriptNullishFieldCapture,readJavaScriptNullishPropertyMessagePrefix,readJavaScriptNullishPropertyMessageSuffix,readJavaScriptNullishKeyDescription,readJavaScriptNullishIndexKeyField,acceptJavaScriptNullishKeyDescriptionOrigin,readJavaScriptNullishKeyDescriptionText,readJavaScriptBigIntegerPropertyDescriptionShift,readJavaScriptBigIntegerPropertyDescriptionOverflow} from './javascript-nullish-field-rules.js';
const literal=(type,value)=>({k:'lit',value,type});
const concatenate=(left,right)=>({k:'binary',op:'concat',left,right,type:STRING});
function primitiveKeyText(value,span){
 if(value.type.kind==='option'){
  const present=value.type.element;
  if(present.kind==='unit')return materializeJavaScriptPrimitiveText(value,span);
  const text=primitiveKeyText({k:'unwrapOption',arg:value,type:present},span);
  return {k:'if',cond:constructJavaScriptOptionalPresence(value,present,span),then:text,else:materializeJavaScriptPrimitiveText(value,span),type:STRING};
 }
 if(value.type.kind!=='int'&&value.type.kind!=='nat')return materializeJavaScriptPrimitiveText(value,span);
 const integer=coerce(value,INT,'JavaScript',span),zero=literal(INT,'0');
 const negative={k:'binary',op:'lt',left:integer,right:zero,domain:INT,type:BOOL};
 const magnitude={k:'if',cond:negative,then:{k:'unary',op:'neg',arg:integer,type:INT},else:integer,type:INT};
 const shifted={k:'math',op:'shiftBigIntegerBitsRight',args:[magnitude,literal(INT,readJavaScriptBigIntegerPropertyDescriptionShift())],type:INT};
 const exceeds={k:'binary',op:'ne',left:shifted,right:zero,domain:INT,type:BOOL};
 return {k:'if',cond:exceeds,then:literal(STRING,readJavaScriptBigIntegerPropertyDescriptionOverflow()),else:materializeJavaScriptPrimitiveText(value,span),type:STRING};
}
function describeKey(value,span,source){
 const fields=value.type.fields??[];
 const form=readJavaScriptNullishKeyDescription(value.type.kind,fields.some(field=>field.name==='toString'),fields.some(field=>field.name==='constructor'));
 if(!acceptJavaScriptNullishKeyDescriptionOrigin(form,source.k))throw unsupported('computed property key diagnostic','the actual key needs its constructor provenance and side-effect-free source description',span);
 if(form==='text')return {named:true,value:primitiveKeyText(value,span)};
 if(form==='omitted')return {named:false,value:literal(STRING,'')};
 if(form==='object'||form==='map'||form==='set')return {named:true,value:literal(STRING,readJavaScriptNullishKeyDescriptionText(form))};
 throw unsupported('computed property key diagnostic','the actual key needs its side-effect-free source description and constructor protocol',span);
}

export function checkJavaScriptNullishAccess(checker,node,value,env,path,expected){
 if(checker.language!=='JavaScript')return null;
 const computed=node.k==='index';
 const member=node.k==='method'?node.op:node.field;
 const form=readJavaScriptNullishFieldForm(value.type.kind,value.type.element?.kind??'');
 if(form==='direct')return null;
 const keyValue=computed?checker.expr(node.index,env,path,undefined):null;
 const capture=checker.freshName('ml_nullish_field_receiver'),field=readJavaScriptNullishFieldCapture();
 const keyField=readJavaScriptNullishIndexKeyField();
 const capturedType={kind:'record',fields:[{name:field,type:value.type},...(computed?[{name:keyField,type:keyValue.type}]:[])]};
 const keyName=computed?checker.freshName('ml_computed_property_key'):null;
 const key=computed?{k:'recordField',object:{k:'var',name:capture,type:capturedType},field:keyField,index:1,type:keyValue.type}:null;
 const sourceScope=new Map(env);if(computed)bindLocal(sourceScope,keyName,keyValue.type);
 const receiver={k:'recordField',object:{k:'var',name:capture,type:capturedType},field,index:0,type:value.type};
 let present,success,type=expected??FLOAT;
 if(form==='guarded'){
  present=value.type.element;
  const binding=checker.freshName('ml_nullish_field_present'),scope=new Map(sourceScope);bindLocal(scope,binding,present);
  const object={k:'name',path:[binding],span:node.object.span};
  const index=computed&&(node.index.k==='num'||node.index.k==='unary'&&node.index.op==='neg'&&node.index.arg.k==='num')?node.index:computed?{k:'name',path:[keyName],span:node.index.span}:null;
  const source=node.k==='method'?{...node,object}:computed?{...node,object,index}:{k:'field',object,field:member,span:node.span};
  const body=checker.expr(source,scope,path,expected);
  type=body.type;
  success={k:'let',name:binding,value:{k:'unwrapOption',arg:receiver,type:present},body,type};
 }
 const policy=value.type.kind==='unit'?'undefined':value.type.absence;
 const description=computed?describeKey(key,node.index.span??node.span,node.index):null;
 const messageFor=absence=>computed?description.named?concatenate(concatenate(literal(STRING,readJavaScriptNullishPropertyMessagePrefix(absence,true)),description.value),literal(STRING,readJavaScriptNullishPropertyMessageSuffix(true))):literal(STRING,readJavaScriptNullishPropertyMessagePrefix(absence,false)):literal(STRING,readJavaScriptNullishFieldMessage(absence,member));
 const nullMessage=messageFor('null');
 const undefinedMessage=messageFor('undefined');
 let message=policy==='null'?nullMessage:undefinedMessage;
 if(policy==='both'){
  const tag=present?readTypeQueryResult(present.kind)||'object':'undefined';
  const query={k:'optionalTypeOf',arg:receiver,present:tag,type:STRING};
  const nullish={k:'binary',op:'eq',left:query,right:literal(STRING,'object'),domain:STRING,type:BOOL};
  message={k:'if',cond:nullish,then:nullMessage,else:undefinedMessage,type:STRING};
 }
 const failure={k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[message,literal(BOOL,true)],type};
 let body=form==='guarded'?{k:'if',cond:constructJavaScriptOptionalPresence(receiver,present,node.span),then:success,else:failure,type}:failure;
 if(computed)body={k:'let',name:keyName,value:{k:'recordField',object:{k:'var',name:capture,type:capturedType},field:keyField,index:1,type:keyValue.type},body,type};
 return {k:'let',name:capture,value:{k:'record',entries:[{name:field,value},...(computed?[{name:keyField,value:keyValue}]:[])],type:capturedType},body,type};
}

export function checkJavaScriptNullishField(checker,node,value,env,path,expected){return checkJavaScriptNullishAccess(checker,node,value,env,path,expected);}

export function checkJavaScriptNullishIndex(checker,node,value,env,path,expected){return checkJavaScriptNullishAccess(checker,node,value,env,path,expected);}
