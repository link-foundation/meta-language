// Actual source operands and typed-node transport; shared rules select the call.
import {BOOL,FLOAT,STRING,array} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {acceptJavaScriptNumberConversionType,materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {materializeJavaScriptPrimitiveText} from './javascript-string-search.js';
import {readJavaScriptStringUnitCallForm} from './javascript-property-call-rules.js';
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
export function checkJavaScriptStringUnitCall(checker,node,receiver,env,path){
 const values=[receiver,...node.args.map(argument=>{if(!argument)throw typeError('a string unit call must retain every actual argument',node.span);return checker.expr(argument,env,path,undefined);})];
 const names=values.map(()=>checker.freshName('ml_string_unit_operand'));
 const bound=values.map((value,index)=>variable(names[index],value.type));
 let body;
 if(readJavaScriptStringUnitCallForm(node.op)==='position'){
  body=checker.materializePrimitiveMethod(node,bound[0],[numeric(checker,bound[1],number('0'),node.span)],path);
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
