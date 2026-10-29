// Actual source operands and typed-node transport; shared rules select the call.
import {BOOL,FLOAT,STRING,array} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {acceptJavaScriptNumberConversionType,materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {materializeJavaScriptPrimitiveText} from './javascript-string-search.js';
import {readJavaScriptStringUnitCallForm,planJavaScriptStringPaddingUnchanged,planJavaScriptStringRepetitionInvalid} from './javascript-property-call-rules.js';
const number=value=>({k:'lit',value,type:FLOAT});
const variable=(name,type)=>({k:'var',name,type});
function undefinedCondition(value){
 if(value.type.kind==='unit')return {k:'lit',value:true,type:BOOL};
 return value.type.kind==='option'&&value.type.absence!=='null'
  ?{k:'strictEqual',left:value,right:{k:'none',absence:'undefined',type:value.type},negate:false,type:BOOL}:null;
}
function numeric(checker,value,fallback,span){
 if(!value)return fallback;
 if(!acceptJavaScriptNumberConversionType(value.type))throw unsupported('string unit method numeric operand','the actual object or symbol needs its complete numeric conversion',span);
 const converted=materializeJavaScriptNumberConversion(checker,value,'coerceJavaScriptNumber');
 const absent=undefinedCondition(value);
 return absent?{k:'if',cond:absent,then:fallback,else:converted,type:FLOAT}:converted;
}
function expansionPredicate(plan,count,receiver){
 const stack=[];
 for(const instruction of plan){
  if(instruction==='count')stack.push(count);
  else if(instruction==='length')stack.push({k:'length',array:receiver,type:FLOAT});
  else if(instruction==='true')stack.push({k:'lit',value:true,type:BOOL});
  else if(instruction==='zero'||instruction==='infinity')stack.push(number(instruction==='zero'?'0':'Infinity'));
  else if(instruction==='isNaN'||instruction==='trunc')stack.push({k:'math',op:instruction,args:[stack.pop()],type:instruction==='isNaN'?BOOL:FLOAT});
  else if(['le','lt','eq'].includes(instruction)){const right=stack.pop(),left=stack.pop();stack.push({k:'binary',op:instruction,left,right,type:BOOL,domain:FLOAT});}
  else if(instruction==='or'){const right=stack.pop(),left=stack.pop();stack.push({k:'if',cond:left,then:{k:'lit',value:true,type:BOOL},else:right,type:BOOL});}
  else if(instruction==='if'){const otherwise=stack.pop(),then=stack.pop(),cond=stack.pop();stack.push({k:'if',cond,then,else:otherwise,type:BOOL});}
  else throw new TypeError('unregistered shared padding predicate instruction');
 }
 if(stack.length!==1)throw new TypeError('incomplete shared padding predicate');
 return stack[0];
}
export function checkJavaScriptStringUnitCall(checker,node,receiver,env,path){
 const values=[receiver,...node.args.map(argument=>{if(!argument)throw typeError('a string unit call must retain every actual argument',node.span);return checker.expr(argument,env,path,undefined);})];
 const names=values.map(()=>checker.freshName('ml_string_unit_operand'));
 const bound=values.map((value,index)=>variable(names[index],value.type));
 let body;
 if(readJavaScriptStringUnitCallForm(node.op)==='position'){
  body=checker.materializePrimitiveMethod(node,bound[0],[numeric(checker,bound[1],number('0'),node.span)],path);
 }else if(readJavaScriptStringUnitCallForm(node.op)==='repetition'){
  const countName=checker.freshName('ml_string_repetition_bound'),count=variable(countName,FLOAT);
  const call=checker.materializePrimitiveMethod(node,bound[0],[count],path);
  const original=bound[1]??{k:'unit',type:{kind:'unit'}};
  const message={k:'binary',op:'concat',left:{k:'lit',value:'Invalid count value: ',type:STRING},right:materializeJavaScriptPrimitiveText(original,node.span),type:STRING};
  const failure={k:'math',op:'raiseJavaScriptRangeError',args:[message,{k:'lit',value:true,type:BOOL}],type:STRING};
  body={k:'let',name:countName,value:numeric(checker,bound[1],number('0'),node.span),body:{k:'if',cond:expansionPredicate(planJavaScriptStringRepetitionInvalid(),count,bound[0]),then:failure,else:call,type:STRING},type:STRING};
 }else if(readJavaScriptStringUnitCallForm(node.op)==='padding'){
  const countName=checker.freshName('ml_string_padding_bound'),fillerName=checker.freshName('ml_string_padding_filler');
  const count=variable(countName,FLOAT),space={k:'lit',value:' ',type:STRING},filler=bound[2];
  let converted=space;
  if(filler&&filler.type.kind!=='unit'){
   converted=materializeJavaScriptPrimitiveText(filler,node.span);
   const absent=undefinedCondition(filler);
   if(absent)converted={k:'if',cond:absent,then:space,else:converted,type:STRING};
  }
  const call=checker.materializePrimitiveMethod(node,bound[0],[count,variable(fillerName,STRING)],path);
  const present={k:'let',name:fillerName,value:converted,body:call,type:STRING};
  const unchanged=expansionPredicate(planJavaScriptStringPaddingUnchanged(),count,bound[0]);
  body={k:'let',name:countName,value:numeric(checker,bound[1],number('0'),node.span),body:{k:'if',cond:unchanged,then:bound[0],else:present,type:STRING},type:STRING};
 }else if(readJavaScriptStringUnitCallForm(node.op)==='replacement'){
  const searchName=checker.freshName('ml_string_replace_search'),replacementName=checker.freshName('ml_string_replace_template');
  const missing={k:'unit',type:{kind:'unit'}};
  const search=materializeJavaScriptPrimitiveText(bound[1]??missing,node.span),replacement=materializeJavaScriptPrimitiveText(bound[2]??missing,node.span);
  body=checker.materializePrimitiveMethod(node,bound[0],[variable(searchName,STRING),variable(replacementName,STRING)],path);
  body={k:'let',name:replacementName,value:replacement,body,type:STRING};
  body={k:'let',name:searchName,value:search,body,type:STRING};
 }else{
  const type=array(STRING),limitName=checker.freshName('ml_string_split_limit');
  const limit=numeric(checker,bound[2],number('4294967295'),node.span),limitValue=variable(limitName,FLOAT);
  const method=separator=>checker.materializePrimitiveMethod(node,bound[0],[separator,limitValue],path);
  const separator=bound[1];
  if(!separator||separator.type.kind==='unit')body=method(null);
  else{
   const separatorName=checker.freshName('ml_string_split_separator');
   body={k:'let',name:separatorName,value:materializeJavaScriptPrimitiveText(separator,node.span),body:method(variable(separatorName,STRING)),type};
   const absent=undefinedCondition(separator);
   if(absent)body={k:'if',cond:absent,then:method(null),else:body,type};
  }
  body={k:'let',name:limitName,value:limit,body,type};
 }
 for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:body.type};
 return body;
}
