// Actual IR stack/argument transport; shared plans own every range decision.
import {BOOL,FLOAT,STRING} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {acceptJavaScriptNumberConversionType,materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {planJavaScriptStringRangeWindow} from './javascript-string-range-rules.js';
const number=value=>({k:'lit',value,type:FLOAT});
const variable=(name,type)=>({k:'var',name,type});
function window(operation,receiver,start,end){
 const stack=[];
 for(const instruction of planJavaScriptStringRangeWindow(operation)){
  if(instruction==='start'||instruction==='end'){stack.push(instruction==='start'?start:end);continue;}
  if(instruction==='length'){stack.push({k:'length',array:receiver,type:FLOAT});continue;}
  if(instruction==='0'||instruction==='Infinity'){stack.push(number(instruction));continue;}
  if(instruction==='trunc'){stack.push({k:'math',op:'trunc',args:[stack.pop()],type:FLOAT});continue;}
  if(instruction==='if'){const otherwise=stack.pop(),then=stack.pop(),cond=stack.pop();stack.push({k:'if',cond,then,else:otherwise,type:FLOAT});continue;}
  const right=stack.pop(),left=stack.pop();
  if(instruction==='min'||instruction==='max')stack.push({k:'math',op:instruction,args:[left,right],type:FLOAT});
  else stack.push({k:'binary',op:instruction,left,right,domain:FLOAT,type:instruction==='eq'||instruction==='lt'?BOOL:FLOAT});
 }
 return stack;
}
export function checkJavaScriptStringRange(checker,node,receiver,env,path){
 const values=[receiver,...node.args.map(argument=>{if(!argument)throw typeError('a string range must retain every actual argument',node.span);return checker.expr(argument,env,path,undefined);})];
 const names=values.map(()=>checker.freshName('ml_string_range_operand'));
 const bound=values.map((value,index)=>variable(names[index],value.type));
 const converted=[];
 for(let index=0;index<2;index+=1){
  const value=bound[index+1],fallback=number(index===0?'0':'Infinity');
  if(!value||value.type.kind==='unit'){converted.push(fallback);continue;}
  if(!acceptJavaScriptNumberConversionType(value.type))throw unsupported('string range position','the actual object or symbol needs its complete numeric conversion',node.span);
  let result=materializeJavaScriptNumberConversion(checker,value,'coerceJavaScriptNumber');
  if(value.type.kind==='option'&&value.type.absence!=='null')result={k:'if',cond:{k:'strictEqual',left:value,right:{k:'none',absence:'undefined',type:value.type},negate:false,type:BOOL},then:fallback,else:result,type:FLOAT};
  converted.push(result);
 }
 const positions=converted.map(()=>checker.freshName('ml_string_range_position'));
 let body={k:'method',op:'slice',object:bound[0],args:window(node.op,bound[0],variable(positions[0],FLOAT),variable(positions[1],FLOAT)),type:STRING};
 for(let index=1;index>=0;index-=1)body={k:'let',name:positions[index],value:converted[index],body,type:STRING};
 for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:STRING};
 return body;
}
