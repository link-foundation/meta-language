// Source operands are captured before ToObject; ordinary kernel calls own keys.
import {BOOL,FLOAT,STRING,UNIT,array} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readJavaScriptObjectCoercionError} from './javascript-own-property-rules.js';
import {readJavaScriptEnumerationForm,readJavaScriptOwnEnumerationReceiverForm} from './javascript-enumeration-rules.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
const literal=(type,value)=>({k:'lit',value,type}),variable=(name,type)=>({k:'var',name,type});
function enumerate(checker,object,form,result,span){
 const kind=object.type.kind;
 const receiver=readJavaScriptOwnEnumerationReceiverForm(kind,object.type.element?.kind??'');
 if(receiver==='absent')return {k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[literal(STRING,readJavaScriptObjectCoercionError()),literal(BOOL,true)],type:result};
 if(receiver==='guarded'){
  const type=object.type.element;
  return {k:'if',cond:constructJavaScriptOptionalPresence(object,type,span),then:enumerate(checker,{k:'unwrapOption',arg:object,type},form,result,span),else:enumerate(checker,{k:'unit',type:UNIT},form,result,span),type:result};
 }
 if(form!=='keys')throw unsupported('Object.'+form,'the actual value/entry element and mutable pair domains require their complete enumeration operations',span);
 if(receiver==='record'){
  if(object.type.open||object.type.ordered!==true)throw unsupported('Object.keys record','every actual own field and its value order must be retained',span);
  const owned=checker.referenceValue(object);
  const order={k:'method',op:'readJavaScriptRecordPropertyOrder',object:owned,args:[],type:array(FLOAT)};
  return {k:'math',op:'readJavaScriptRecordPropertyKeyTexts',args:[constructRecordPropertyNameRows(object.type.fields),order],type:result};
 }
 if(receiver==='indexed'){
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
 const result=form==='keys'?array(STRING):expected;
 if(!result)throw unsupported('Object.'+form+' result','the complete value element domain is required',node.span);
 const name=checker.freshName('ml_enumeration_operands'),width=String(values.length-1).length;
 const field=index=>'argument'+String(index).padStart(width,'0');
 const type={kind:'record',fields:values.map((value,index)=>({name:field(index),type:value.type}))};
 const object={k:'recordField',object:variable(name,type),field:field(0),index:0,type:values[0].type};
 const body=enumerate(checker,object,form,result,node.span);
 if(body.type.kind!=='array')throw typeError('an object enumeration must retain its actual array result',node.span);
 return {k:'let',name,value:{k:'record',entries:values.map((value,index)=>({name:field(index),value})),type},body,type:result};
}
