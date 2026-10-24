// Actual source property/key/type operands into existing field and string IR.
import {BOOL,FLOAT,STRING,option} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {readJavaScriptLiteralPropertyIndex,readJavaScriptLiteralPropertyForm,readJavaScriptStringIndexNumberForm} from './javascript-literal-property-rules.js';
import {materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
const literal=(type,value)=>({k:'lit',value,type});
const variable=(name,type)=>({k:'var',name,type});
const binary=(op,left,right,domain,type)=>({k:'binary',op,left,right,...(domain?{domain}:{}),type});
export function checkJavaScriptNamedProperty(value,field,span){
 if(value.type.kind==='record'){
  const selected=value.type.fields.find(item=>item.name===field);
  if(!selected)throw typeError('record has no field '+field,span);
  return {k:'recordField',object:value,field,index:value.type.fields.indexOf(selected),type:selected.type};
 }
 if(field==='size'&&(value.type.kind==='map'||value.type.kind==='set'))return {k:'collectionSize',object:value,type:FLOAT};
 if(field==='length'&&(value.type.kind==='array'||value.type.kind==='string'))return {k:'length',array:value,type:FLOAT};
 throw unsupported('field access','field '+field+' is only portable inside a constructor match',span);
}
export function readJavaScriptLiteralIndexSource(node,kind){
 if(node.index.k!=='str')return null;
 const key=node.index.value;
 const index=readJavaScriptLiteralPropertyIndex(Array.from({length:key.length},(_,at)=>key.charCodeAt(at)));
 return readJavaScriptLiteralPropertyForm(kind,index)==='index'?{...node,index:{k:'num',value:String(index),span:node.index.span}}:{k:'field',object:node.object,field:node.index.value,span:node.span};
}
function numeric(checker,value,span){
 const form=readJavaScriptStringIndexNumberForm(value.type.kind,value.type.element?.kind??'');
 if(form==='identity')return value;
 if(form==='convert')return materializeJavaScriptNumberConversion(checker,value,'convertJavaScriptNumber');
 if(form==='absent')return literal(FLOAT,'NaN');
 if(form==='optional')return {k:'if',cond:constructJavaScriptOptionalPresence(value,value.type.element,span),then:numeric(checker,{k:'unwrapOption',arg:value,type:value.type.element},span),else:literal(FLOAT,'NaN'),type:FLOAT};
 throw unsupported('indexed String key','the actual property key needs its represented coercion, named property and prototype behavior',span);
}
export function checkJavaScriptStringIndex(checker,node,receiver,env,path){
 const key=checker.expr(node.index,env,path,undefined),capture=checker.freshName('ml_string_index_operands');
 const capturedType={kind:'record',fields:[{name:'base',type:receiver.type},{name:'key',type:key.type}]};
 const read=(field,index,type)=>({k:'recordField',object:variable(capture,capturedType),field,index,type});
 const text=read('base',0,receiver.type),raw=read('key',1,key.type);
 const name=checker.freshName('ml_string_index_number'),at=variable(name,FLOAT),number=numeric(checker,raw,node.index.span??node.span);
 const integral={k:'math',op:'isInteger',args:[at],type:BOOL};
 const bounds=binary('and',binary('ge',at,literal(FLOAT,'0'),FLOAT,BOOL),binary('lt',at,{k:'length',array:text,type:FLOAT},FLOAT,BOOL),null,BOOL);
 const condition=binary('and',integral,bounds,null,BOOL),type=option(STRING,'undefined');
 const character=checker.materializePrimitiveMethod({k:'method',op:'charAt',span:node.span},text,[at],path);
 const selected={k:'if',cond:condition,then:{k:'some',value:character,type},else:{k:'none',absence:'undefined',type},type};
 const body={k:'let',name,value:number,body:selected,type};
 return {k:'let',name:capture,value:{k:'record',entries:[{name:'base',value:receiver},{name:'key',value:key}],type:capturedType},body,type};
}
