// Hosts transport actual checked types, expressions and target representations.
// Syntax, rounding and scalar target semantics live in the shared definitions.
import {FLOAT,BOOL} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
import {readJavaScriptNumberConversionForm,renderJavaScriptNumberScalarConversion,
  readJavaScriptNumberConversionStatement} from './string-number-rules.js';
import {renderOptionalMatch,readOptionalOutputAbsences} from './optional-value-rules.js';

export function acceptJavaScriptNumberConversionType(type) {
  const form=readJavaScriptNumberConversionForm(type.kind);
  return form==='option'?acceptJavaScriptNumberConversionType(type.element):form!=='unsupported';
}

export function materializeJavaScriptNumberConversion(checker,value,operation) {
  if(value.type.kind!=='option')return{k:'math',op:operation,args:[value],type:FLOAT};
  const name=checker.freshName('ml_numeric_optional');
  const variable={k:'var',name,type:value.type};
  let body=materializeJavaScriptNumberConversion(checker,{k:'unwrapOption',arg:variable,type:value.type.element},operation);
  for(const absence of readOptionalOutputAbsences(value.type.absence).reverse()){
    body={k:'if',cond:{k:'strictEqual',left:variable,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
      then:{k:'lit',value:absence==='null'?'0':'NaN',type:FLOAT},else:body,type:FLOAT};
  }
  return{k:'let',name,value,body,type:FLOAT};
}

export function checkJavaScriptMathematicalArguments(checker,node,env,path,form) {
  const values=node.args.map(item=>checker.expr(item.value,env,path,undefined));
  const capture=values.length>1&&(form==='unary'||values.some(value=>value.type.kind!=='float'));
  const names=capture?values.map(()=>checker.freshName('ml_mathematical_argument')):[];
  const bound=capture?values.map((value,index)=>({k:'var',name:names[index],type:value.type})):values;
  const selected=form==='unary'?bound.slice(0,1):bound;
  const args=selected.map(value=>{
    if(!acceptJavaScriptNumberConversionType(value.type))throw unsupported('mathematical numeric conversion operand',
      'the actual object primitive conversion, union or symbol must have its own represented behavior',node.span);
    return value.type.kind==='float'?value:materializeJavaScriptNumberConversion(checker,value,'coerceJavaScriptNumber');
  });
  let body=form==='unary'&&args.length===0?{k:'lit',value:'NaN',type:FLOAT}:{k:'math',op:node.op,args,type:FLOAT};
  if(capture)for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:FLOAT};
  else if(form==='unary')for(let index=values.length-1;index>=1;index-=1)body={k:'let',name:checker.freshName('ml_mathematical_argument_unused'),value:values[index],body,type:FLOAT};
  return body;
}

export function checkJavaScriptNumberBuiltin(checker,node,env,path) {
  if(checker.language!=='JavaScript')throw unsupported('JavaScript Number conversion','the operation needs its actual JavaScript source owner',node.span);
  if(node.args.some(item=>item.spread))throw unsupported('spread numeric conversion arguments',
    'the actual spread iterator and argument evaluations need to be represented',node.span);
  const coercing=node.op!=='convertJavaScriptNumber';
  const predicate=node.op==='testCoercedJavaScriptNumberFinite'?'isFinite':node.op==='testCoercedJavaScriptNumberNaN'?'isNaN':'';
  if(node.op==='coerceJavaScriptNumber'&&node.args.length!==1)throw typeError('ToNumber needs exactly one represented operand',node.span);
  const args=node.args.map(item=>checker.expr(item.value,env,path,undefined));
  const binding=args.length>1?checker.freshName('ml_numeric_argument'):'';
  const first=args.length>1?{k:'var',name:binding,type:args[0].type}:args[0];
  if(first&&!acceptJavaScriptNumberConversionType(first.type))throw unsupported('numeric conversion operand',
    'the actual object primitive conversion, union or symbol must have its own represented behavior',node.span);
  let body=first?materializeJavaScriptNumberConversion(checker,first,coercing?'coerceJavaScriptNumber':'convertJavaScriptNumber')
    :{k:'lit',value:coercing?'NaN':'0',type:FLOAT};
  if(predicate)body={k:'math',op:predicate,args:[body],type:BOOL};
  for(let index=args.length-1;args.length>1&&index>=0;index-=1){
    body={k:'let',name:index===0?binding:checker.freshName('ml_numeric_argument_unused'),value:args[index],body,type:body.type};
  }
  return body;
}

export function emitRustJavaScriptNumberConversion(emitter,expression) {
  const argument=expression.args[0];
  const render=(type,value,depth)=>{
    const form=readJavaScriptNumberConversionForm(type.kind);
    if(form==='option'){
      if(type.absence==='both')emitter.usesOptional=true;
      const name='ml_numeric_present'+depth;
      return renderOptionalMatch('Rust',type.absence,value,name,render(type.element,name,depth+1),'0.0f64','f64::NAN');
    }
    if(form==='string'||((form==='int'||form==='nat')&&expression.op==='convertJavaScriptNumber')){
      emitter.usesBig=true;emitter.usesStringNumber=true;
    }
    if((form==='int'||form==='nat')&&expression.op==='coerceJavaScriptNumber')emitter.usesSourceException=true;
    const text=renderJavaScriptNumberScalarConversion('Rust',expression.op,form,value,emitter.utf16Strings);
    if(!text)throw unsupported('numeric conversion operand','a checked scalar conversion needs its complete shared operation',expression.span);
    return text;
  };
  const text=render(argument.type,emitter.expr(argument),0);
  emitter.state.encode('javascript-number-conversion',readJavaScriptNumberConversionStatement());
  return text;
}
