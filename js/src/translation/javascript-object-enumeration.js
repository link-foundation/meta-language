// Source operands are captured before ToObject; ordinary kernel calls own keys.
import {BOOL,FLOAT,STRING,UNIT,array,option,tuple,sameType} from './types.js';
import {coerce} from './check.js';
import {javaScriptConstructorFieldValueType,readJavaScriptOwnedRecordField} from './javascript-constructor-presence.js';
import {mergeOptionalAbsence} from './structured-type-rules.js';
import {typeError,unsupported} from './diagnostics.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readJavaScriptObjectCoercionError} from './javascript-own-property-rules.js';
import {readJavaScriptEnumerationForm,readJavaScriptOwnEnumerationReceiverForm,readJavaScriptObjectEntryPairingOperation} from './javascript-enumeration-rules.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
const literal=(type,value)=>({k:'lit',value,type}),variable=(name,type)=>({k:'var',name,type});
function valueElement(type,fallback,span){
 if(type.kind==='option')return valueElement(type.element,fallback,span);
 if(type.kind==='array')return type.element;
 if(type.kind==='string')return STRING;
 const fields=type.kind==='record'?type.fields.map(javaScriptConstructorFieldValueType):type.kind==='tuple'?type.elements:[];
 if(!fields.length)return fallback??FLOAT;
 if(type.kind==='record'&&type.fields.some(field=>field.optional)){
  const base=fields[0].kind==='option'?fields[0].element:fields[0];
  let absence='undefined';
  for(const field of fields){
   if(!sameType(field.kind==='option'?field.element:field,base))throw unsupported('heterogeneous object values','every actual field needs its shared dynamic value union',span);
   if(field.kind==='option')absence=mergeOptionalAbsence(absence,field.absence);
  }
  return option(base,absence);
 }
 if(fields.some(field=>!sameType(field,fields[0])))throw unsupported('heterogeneous object values','every actual field needs its shared dynamic value union',span);
 return fields[0];
}
function recordValues(checker,object,result,span){
 const name=checker.freshName('ml_enumeration_record');
 const value=checker.referenceValue(object),owned=variable(name,value.type);
 const items=value.type.fields.map(field=>coerce(readJavaScriptOwnedRecordField(owned,field),result.element,'JavaScript',span));
 const values={k:'array',items,type:result};
 const query={k:'method',op:'readJavaScriptRecordPropertyOrder',object:owned,args:[],type:array(FLOAT)};
 const op=value.type.fields.some(field=>field.optional)?'reorderJavaScriptPresentRecordPropertyValues':'reorderJavaScriptRecordPropertyValues';
 return {k:'let',name,value,body:{k:'math',op,args:[values,query],type:result},type:result,...(span?{span}:{})};
}
function enumerate(checker,object,form,result,span){
 const kind=object.type.kind;
 const receiver=readJavaScriptOwnEnumerationReceiverForm(kind,object.type.element?.kind??'');
 if(receiver==='absent')return {k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[literal(STRING,readJavaScriptObjectCoercionError()),literal(BOOL,true)],type:result};
 if(receiver==='guarded'){
  const type=object.type.element;
  return {k:'if',cond:constructJavaScriptOptionalPresence(object,type,span),then:enumerate(checker,{k:'unwrapOption',arg:object,type},form,result,span),else:enumerate(checker,{k:'unit',type:UNIT},form,result,span),type:result};
 }
 if(form==='entries'){
  const values=enumerate(checker,object,'values',array(result.element.elements[1]),span);
  const keys=enumerate(checker,object,'keys',array(STRING),span);
  return {k:'math',op:readJavaScriptObjectEntryPairingOperation(),args:[keys,values],type:result};
 }
 if(form!=='keys'&&form!=='values')throw unsupported('Object.'+form,'the actual mutable pair domain requires its complete enumeration operations',span);
 if(receiver==='record'){
  if(object.type.open||object.type.ordered!==true)throw unsupported('Object.keys record','every actual own field and its value order must be retained',span);
  const owned=checker.referenceValue(object);
  if(form==='values')return recordValues(checker,object,result,span);
  const order={k:'method',op:'readJavaScriptRecordPropertyOrder',object:owned,args:[],type:array(FLOAT)};
  const op=object.type.fields.some(field=>field.optional)?'readJavaScriptPresentRecordPropertyKeyTexts':'readJavaScriptRecordPropertyKeyTexts';
  return {k:'math',op,args:[constructRecordPropertyNameRows(object.type.fields),order],type:result};
 }
 if(receiver==='indexed'){
  if(form==='values'){
   if(kind==='string')return {k:'math',op:'createJavaScriptStringOwnPropertyValues',args:[object],type:result};
   const owned=checker.referenceValue(object);
   if(kind==='array')return {k:'append',left:{k:'array',items:[],type:result},right:owned,type:result};
   return {k:'array',items:object.type.elements.map((type,index)=>({k:'tupleIndex',object:owned,index,type})),type:result};
  }
  const length=kind==='tuple'?literal(FLOAT,String(object.type.elements.length)):{k:'length',array:checker.referenceValue(object),type:FLOAT};
  return {k:'math',op:'createJavaScriptDensePropertyKeyTexts',args:[length],type:result};
 }
 if(receiver==='empty')return {k:'array',items:[],type:result};
 throw unsupported('Object.keys receiver','the actual nominal, callable, Symbol, proxy or exotic own-property protocol must be represented',span);
}
export function checkJavaScriptObjectEnumeration(checker,node,env,path,expected){
 const form=readJavaScriptEnumerationForm(node.op??'');if(!form)return null;
 if(checker.language!=='JavaScript'||node.args.some(argument=>argument.spread))throw unsupported('Object.'+form+' arguments','every actual iterator argument effect must be represented',node.span);
 const values=node.args.map(argument=>checker.expr(argument.value,env,path,undefined));
 if(!values.length)values.push({k:'unit',type:UNIT});
 const contextual=expected?.kind==='array'?expected.element:undefined;
 const fallback=form==='entries'&&contextual?.kind==='tuple'?contextual.elements[1]:contextual;
 const element=form==='keys'?STRING:valueElement(values[0].type,fallback,node.span);
 const result=array(form==='entries'?tuple([STRING,element]):element);
 if(!result)throw unsupported('Object.'+form+' result','the complete value element domain is required',node.span);
 const name=checker.freshName('ml_enumeration_operands'),width=String(values.length-1).length;
 const field=index=>'argument'+String(index).padStart(width,'0');
 const type={kind:'record',fields:values.map((value,index)=>({name:field(index),type:value.type}))};
 const object={k:'recordField',object:variable(name,type),field:field(0),index:0,type:values[0].type};
 const body=enumerate(checker,object,form,result,node.span);
 if(body.type.kind!=='array')throw typeError('an object enumeration must retain its actual array result',node.span);
 return {k:'let',name,value:{k:'record',entries:values.map((value,index)=>({name:field(index),value})),type},body,type:result};
}
