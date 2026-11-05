// Actual checked argument/type transport into ordinary Boolean/value/error IR.
import {BOOL,FLOAT,STRING,UNIT} from './types.js';
import {unsupported} from './diagnostics.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {materializeJavaScriptPrimitiveText} from './javascript-string-search.js';
import {materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readJavaScriptOwnPropertyReceiverForm,readJavaScriptObjectCoercionError,readJavaScriptArrayPropertyIndexLimit} from './javascript-own-property-rules.js';
const literal=(type,value)=>({k:'lit',value,type});
const variable=(name,type)=>({k:'var',name,type});
const binary=(op,left,right,domain,type)=>({k:'binary',op,left,right,...(domain?{domain}:{}),type});
const equal=(left,right)=>binary('eq',left,right,STRING,BOOL);
const both=(left,right)=>binary('and',left,right,null,BOOL);
const either=(left,right)=>binary('or',left,right,null,BOOL);
const failure=()=>({k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[literal(STRING,readJavaScriptObjectCoercionError()),literal(BOOL,true)],type:BOOL});
function membership(checker,receiver,key,form,span){
 if(form==='record'){
  if(receiver.type.open)throw unsupported('own property membership','the actual open record needs its complete runtime property domain',span);
  return receiver.type.fields.reduce((result,field)=>either(result,equal(key,literal(STRING,field.name))),literal(BOOL,false));
 }
 if(form==='empty')return literal(BOOL,false);
 if(form!=='indexed')throw unsupported('own property membership','the actual callable, nominal value, proxy or exotic object needs its complete own-property protocol',span);
 const name=checker.freshName('ml_own_property_index'),index=variable(name,FLOAT);
 const number=materializeJavaScriptNumberConversion(checker,key,'convertJavaScriptNumber');
 const length=receiver.type.kind==='tuple'?literal(FLOAT,String(receiver.type.elements.length)):{k:'length',array:checker.referenceValue(receiver),type:FLOAT};
 const canonical=equal(materializeJavaScriptPrimitiveText(index,span),key);
 const integral={k:'math',op:'isInteger',args:[index],type:BOOL};
 const bounds=both(binary('ge',index,literal(FLOAT,'0'),FLOAT,BOOL),binary('lt',index,length,FLOAT,BOOL));
 const arrayBound=receiver.type.kind==='string'?literal(BOOL,true):binary('lt',index,literal(FLOAT,readJavaScriptArrayPropertyIndexLimit()),FLOAT,BOOL);
 const selected=either(equal(key,literal(STRING,'length')),both(canonical,both(integral,both(bounds,arrayBound))));
 return {k:'let',name,value:number,body:selected,type:BOOL};
}
function converted(checker,receiver,key,span){
 const form=readJavaScriptOwnPropertyReceiverForm(receiver.type.kind,receiver.type.element?.kind??'');
 if(form==='absent')return failure();
 if(form==='guarded'){
  const present=receiver.type.element;
  return {k:'if',cond:constructJavaScriptOptionalPresence(receiver,present,span),then:converted(checker,{k:'unwrapOption',arg:receiver,type:present},key,span),else:failure(),type:BOOL};
 }
 const name=checker.freshName('ml_own_property_key'),text=materializeJavaScriptPrimitiveText(key,span);
 const type={kind:'record',fields:[{name:'key',type:STRING}]};
 const read={k:'recordField',object:variable(name,type),field:'key',index:0,type:STRING};
 const body=membership(checker,receiver,read,form,span);
 return {k:'let',name,value:{k:'record',entries:[{name:'key',value:text}],type},body,type:BOOL};
}
export function checkJavaScriptOwnProperty(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.some(item=>item.spread))throw unsupported('Object.hasOwn arguments','the actual argument iterator effects must be represented',node.span);
 const values=node.args.map(item=>checker.expr(item.value,env,path,undefined));
 while(values.length<2)values.push({k:'unit',type:UNIT});
 const name=checker.freshName('ml_own_property_operands');
 const width=String(values.length-1).length,field=index=>'argument'+String(index).padStart(width,'0');
 const type={kind:'record',fields:values.map((value,index)=>({name:field(index),type:value.type}))};
 const read=index=>({k:'recordField',object:variable(name,type),field:field(index),index,type:values[index].type});
 const body=converted(checker,read(0),read(1),node.span);
 return {k:'let',name,value:{k:'record',entries:values.map((value,index)=>({name:field(index),value})),type},body,type:BOOL};
}
