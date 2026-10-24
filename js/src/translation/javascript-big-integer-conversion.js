// Actual checked type and intrinsic result-column adapters.
import {INT,FLOAT,BOOL,STRING,array,option} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
import {readJavaScriptBigIntegerConversionForm,readJavaScriptBigIntegerResultOperation,
  readJavaScriptBigIntegerConversionErrorClass,readJavaScriptBigIntegerConversionMessagePieces,
  requireJavaScriptBigIntegerDivision,readJavaScriptBigIntegerDivisionMessage} from './big-integer-conversion-rules.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readOptionalOutputAbsences} from './optional-value-rules.js';

const text=value=>({k:'lit',value,type:STRING});
const concat=(left,right)=>({k:'binary',op:'concat',left,right,type:STRING});
function failure(form,value) {
  const pieces=readJavaScriptBigIntegerConversionMessagePieces(form);
  const message=concat(concat(text(pieces[0]),value),text(pieces[1]));
  return{k:'math',op:readJavaScriptSourceExceptionOperation(readJavaScriptBigIntegerConversionErrorClass(form)),args:[message,{k:'lit',value:true,type:BOOL}],type:INT};
}
function materialize(checker,value,span) {
  const form=readJavaScriptBigIntegerConversionForm(value.type.kind);
  if(form==='identity')return value.type.kind==='int'?value:{k:'cast',arg:value,from:value.type,to:INT,flavor:'exact',type:INT};
  if(form==='boolean')return{k:'math',op:'convertJavaScriptBooleanBigIntegerValue',args:[value],type:INT};
  if(form==='undefined')return{k:'let',name:checker.freshName('ml_integer_undefined'),value,body:failure(form,text('undefined')),type:INT};
  if(form==='optional'){
    const name=checker.freshName('ml_integer_optional'),variable={k:'var',name,type:value.type};
    let body=materialize(checker,{k:'unwrapOption',arg:variable,type:value.type.element},span);
    for(const absence of readOptionalOutputAbsences(value.type.absence).reverse())body={k:'if',
      cond:{k:'strictEqual',left:variable,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
      then:failure(absence,text(absence)),else:body,type:INT};
    return{k:'let',name,value,body,type:INT};
  }
  if(form!=='string'&&form!=='number')throw unsupported('BigInt primitive conversion',
    'the actual object primitive conversion or symbol must have its complete source behavior',span);
  const name=checker.freshName('ml_integer_operand'),bound={k:'var',name,type:value.type};
  const resultName=checker.freshName('ml_integer_result'),result={k:'var',name:resultName,type:array(INT)};
  const conversion={k:'math',op:form==='string'?'parseJavaScriptStringBigInteger':'convertJavaScriptNumberBigIntegerResult',args:[bound],type:array(INT)};
  const condition={k:'binary',op:'eq',left:{k:'length',array:result,type:FLOAT},right:{k:'lit',value:'1',type:FLOAT},type:BOOL,domain:FLOAT};
  const success={k:'coalesce',left:{k:'optionalIndex',array:result,index:{k:'lit',value:'0',type:FLOAT},type:option(INT,'undefined')},right:{k:'lit',value:'0',type:INT},type:INT};
  const display=form==='string'?bound:{k:'toString',arg:bound,type:STRING};
  const body={k:'if',cond:condition,then:success,else:failure(form,display),type:INT};
  return{k:'let',name,value,body:{k:'let',name:resultName,value:conversion,body,type:INT},type:INT};
}
export function checkJavaScriptBigIntegerBuiltin(checker,node,env,path) {
  if(checker.language!=='JavaScript')throw unsupported('JavaScript BigInt conversion','the operation needs its actual JavaScript source owner',node.span);
  if(node.args.some(item=>item.spread))throw unsupported('spread BigInt arguments','the actual iterator and complete argument effects need their represented operations',node.span);
  const values=node.args.map(item=>checker.expr(item.value,env,path,undefined));
  const names=values.length>1?values.map(()=>checker.freshName('ml_integer_argument')):[];
  const first=values.length>1?{k:'var',name:names[0],type:values[0].type}:values[0];
  let body=first?materialize(checker,first,node.span):failure('undefined',text('undefined'));
  for(let index=values.length-1;values.length>1&&index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:INT};
  return body;
}
export function checkJavaScriptBigIntegerResult(checker,node,env,path) {
  if(readJavaScriptBigIntegerResultOperation(node.op)==='division'){
    if(checker.language!=='JavaScript'||node.args.length!==2||node.args.some(argument=>argument.spread))throw typeError('a JavaScript BigInt division intrinsic needs two owned source operands',node.span);
    const args=node.args.map(argument=>checker.expr(argument.value,env,path,INT));
    if(args.some(argument=>argument.type.kind!=='int'))throw typeError('a JavaScript BigInt division intrinsic needs its actual integer operands',node.span);
    return{k:'math',op:node.op,args,type:array(INT)};
  }
  if(checker.language!=='JavaScript'||node.args.length!==1||node.args[0].spread)throw typeError('a JavaScript BigInt intrinsic needs one owned source operand',node.span);
  const form=readJavaScriptBigIntegerResultOperation(node.op),value=checker.expr(node.args[0].value,env,path,undefined);
  const kind=form==='string'?'string':form==='number'?'float':'bool';
  if(value.type.kind!==kind)throw typeError('a JavaScript BigInt intrinsic needs its actual operand type',node.span);
  return{k:'math',op:node.op,args:[value],type:form==='boolean'?INT:array(INT)};
}

/** Build actual checked IR; shared source rules own admission and division. */
export function materializeJavaScriptBigIntegerDivision(checker,operation,left,right){
  if(!requireJavaScriptBigIntegerDivision(checker.language,operation,left.type.kind,right.k==='lit'?String(right.value):''))return null;
  const name=checker.freshName('ml_integer_division'),result={k:'var',name,type:array(INT)};
  const conversion={k:'math',op:'divideJavaScriptBigIntegerResult',args:[left,right],type:array(INT)};
  const condition={k:'binary',op:'eq',left:{k:'length',array:result,type:FLOAT},right:{k:'lit',value:'2',type:FLOAT},type:BOOL,domain:FLOAT};
  const value={k:'coalesce',left:{k:'optionalIndex',array:result,index:{k:'lit',value:operation==='div'?'0':'1',type:FLOAT},type:option(INT,'undefined')},right:{k:'lit',value:'0',type:INT},type:INT};
  const failure={k:'math',op:'raiseJavaScriptRangeError',args:[text(readJavaScriptBigIntegerDivisionMessage()),{k:'lit',value:true,type:BOOL}],type:INT};
  return{k:'let',name,value:conversion,body:{k:'if',cond:condition,then:value,else:failure,type:INT},type:INT};
}
