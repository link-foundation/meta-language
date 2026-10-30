// Actual IR/type adapters; shared source rules own comparison admission/order.
import {BOOL,FLOAT,INT,UNIT,option} from './types.js';
import {acceptJavaScriptComparisonPrimitive,requireJavaScriptPrimitiveComparison,readJavaScriptPrimitiveComparisonForm,readJavaScriptComparisonAcceptedOrders,readJavaScriptComparisonAbsentNumber,readJavaScriptComparisonUnitAbsence} from './javascript-comparison-rules.js';
import {readOptionalOutputAbsences} from './optional-value-rules.js';
import {readStringOrderOperation} from './string-order-rules.js';
import {planJavaScriptPrimitiveSameValue,readJavaScriptPrimitiveLooseEqualityForm,requireJavaScriptLooseEqualityNegation} from './javascript-comparison-rules.js';
import {typeError,unsupported} from './diagnostics.js';

const literal=(type,value)=>({k:'lit',type,value});
const variable=(name,type)=>({k:'var',name,type});
function strictValue(value){const absence=readJavaScriptComparisonUnitAbsence(value.type.kind);return absence?{k:'none',absence,type:option(UNIT,absence)}:value;}
function accepts(type){return type.kind==='option'?accepts(type.element):acceptJavaScriptComparisonPrimitive(type.kind);}
function integer(value){return value.type.kind==='int'?value:{k:'cast',arg:value,from:value.type,to:INT,flavor:'exact',type:INT};}
function ordered(checker,operation,comparison,reverse){
 const name=checker.freshName('ml_comparison_order'),order=variable(name,FLOAT);
 const accepted=readJavaScriptComparisonAcceptedOrders(operation,reverse);
 const tests=accepted.map(value=>({k:'binary',op:'eq',left:order,right:literal(FLOAT,String(value)),type:BOOL,domain:FLOAT}));
 const body=tests.reduce((left,right)=>({k:'binary',op:'or',left,right,type:BOOL}));
 return{k:'let',name,value:comparison,body,type:BOOL};
}
function compare(checker,operation,left,right){
 if(operation==='eq'||operation==='ne')return{k:'strictEqual',left:strictValue(left),right:strictValue(right),negate:operation==='ne',type:BOOL};
 if(left.type.kind==='option'||right.type.kind==='option'){
  const first=left.type.kind==='option',value=first?left:right;
  const present={k:'unwrapOption',arg:value,type:value.type.element};
  let body=first?compare(checker,operation,present,right):compare(checker,operation,left,present);
  for(const absence of readOptionalOutputAbsences(value.type.absence).reverse()){
   const empty=literal(FLOAT,readJavaScriptComparisonAbsentNumber(absence));
   body={k:'if',cond:{k:'strictEqual',left:value,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
    then:first?compare(checker,operation,empty,right):compare(checker,operation,left,empty),else:body,type:BOOL};
  }
  return body;
 }
 const form=readJavaScriptPrimitiveComparisonForm(operation,left.type.kind,right.type.kind);
 if(form==='string')return{k:'stringTest',op:readStringOrderOperation('JavaScript',operation,'string'),string:left,search:right,type:BOOL};
 if(form==='number'||form==='integer'){
  const domain=form==='number'?FLOAT:INT;
  return{k:'binary',op:operation,left:form==='integer'?integer(left):left,right:form==='integer'?integer(right):right,type:BOOL,domain};
 }
 if(form==='coerce')return compare(checker,operation,checker.numberValue(left),checker.numberValue(right));
 const reverse=form==='integer-number'||form==='integer-string';
 const text=form==='integer-string'||form==='string-integer';
 const a=reverse?right:left,b=integer(reverse?left:right);
 const comparison={k:'math',op:text?'compareJavaScriptStringBigIntegerValue':'compareJavaScriptNumberBigIntegerValue',args:[a,b],type:FLOAT};
 return ordered(checker,operation,comparison,reverse);
}
export function materializeJavaScriptPrimitiveComparison(checker,operation,left,right){
 if(checker.language!=='JavaScript'||!accepts(left.type)||!accepts(right.type))return null;
 if(!requireJavaScriptPrimitiveComparison(left.type.kind,right.type.kind))return null;
 const names=[checker.freshName('ml_comparison_operand'),checker.freshName('ml_comparison_operand')];
 let body=compare(checker,operation,variable(names[0],left.type),variable(names[1],right.type));
 for(const [name,value] of [[names[1],right],[names[0],left]])body={k:'let',name,value,body,type:BOOL};
 return body;
}

function looselyEqual(checker,left,right){
 const first=left.type.kind==='option'&&left.k!=='none',second=right.type.kind==='option'&&right.k!=='none';
 if(first||second){
  const value=first?left:right,present={k:'unwrapOption',arg:value,type:value.type.element};
  let body=first?looselyEqual(checker,present,right):looselyEqual(checker,left,present);
  for(const absence of readOptionalOutputAbsences(value.type.absence).reverse()){
   const empty={k:'none',absence,type:option(UNIT,absence)};
   body={k:'if',cond:{k:'strictEqual',left:value,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
    then:first?looselyEqual(checker,empty,right):looselyEqual(checker,left,empty),else:body,type:BOOL};
  }
  return body;
 }
 const absent=value=>value.k==='none'||value.type.kind==='unit';
 if(absent(left)||absent(right))return literal(BOOL,absent(left)&&absent(right));
 const form=readJavaScriptPrimitiveLooseEqualityForm(left.type.kind,right.type.kind);
 if(form==='different')return literal(BOOL,false);
 if(form==='strict'||form==='integer')return{k:'strictEqual',left:form==='integer'?integer(left):left,right:form==='integer'?integer(right):right,negate:false,type:BOOL};
 if(form==='left-number')return looselyEqual(checker,checker.numberValue(left),right);
 if(form==='right-number')return looselyEqual(checker,left,checker.numberValue(right));
 const reverse=form==='integer-number'||form==='integer-string',text=form==='string-integer'||form==='integer-string';
 return ordered(checker,'eq',{k:'math',op:text?'compareJavaScriptStringBigIntegerValue':'compareJavaScriptNumberBigIntegerValue',args:[reverse?right:left,integer(reverse?left:right)],type:FLOAT},false);
}
export function checkJavaScriptLooseComparison(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.length!==2||node.args.some(argument=>argument.spread))throw typeError('loose equality needs two actual JavaScript operands',node.span);
 const values=node.args.map(argument=>checker.expr(argument.value,env,path,undefined));
 if(values.some(value=>!accepts(value.type)))throw unsupported('JavaScript loose equality operand','the actual object or symbol needs its complete identity and primitive conversion',node.span);
 const names=[checker.freshName('ml_comparison_operand'),checker.freshName('ml_comparison_operand')];
 let body=looselyEqual(checker,variable(names[0],values[0].type),variable(names[1],values[1].type));
 if(requireJavaScriptLooseEqualityNegation(node.op))body={k:'unary',op:'not',arg:body,type:BOOL};
 for(let index=1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:BOOL};
 return body;
}

function sameValue(checker,left,right){
 const first=left.type.kind==='option'&&left.k!=='none',second=right.type.kind==='option'&&right.k!=='none';
 if(first||second){
  const value=first?left:right,present={k:'unwrapOption',arg:value,type:value.type.element};
  let body=first?sameValue(checker,present,right):sameValue(checker,left,present);
  for(const absence of readOptionalOutputAbsences(value.type.absence).reverse()){
   const empty={k:'none',absence,type:option(UNIT,absence)};
   body={k:'if',cond:{k:'strictEqual',left:value,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
    then:first?sameValue(checker,empty,right):sameValue(checker,left,empty),else:body,type:BOOL};
  }
  return body;
 }
 const stack=[];
 for(const operation of planJavaScriptPrimitiveSameValue(left.type.kind,right.type.kind)){
  if(operation==='strict'||operation==='integer')return{k:'strictEqual',left:operation==='integer'?integer(left):strictValue(left),right:operation==='integer'?integer(right):strictValue(right),negate:false,type:BOOL};
  if(operation==='left'||operation==='right'){stack.push(operation==='left'?left:right);continue;}
  if(operation==='one'||operation==='zero'){stack.push(literal(FLOAT,operation==='one'?'1':'0'));continue;}
  const b=stack.pop(),a=stack.pop(),boolean=operation==='and'||operation==='or';
  stack.push({k:'binary',op:operation,left:a,right:b,type:operation==='div'?FLOAT:BOOL,...(!boolean&&{domain:FLOAT})});
 }
 return stack[0];
}
export function checkJavaScriptSameValue(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.some(argument=>argument.spread))throw unsupported('Object.is arguments','spread arguments require the complete actual iterator effects',node.span);
 const values=node.args.map(argument=>checker.expr(argument.value,env,path,undefined));
 if(values.slice(0,2).some(value=>!accepts(value.type)))throw unsupported('Object.is operand','the actual object or symbol needs its complete identity representation',node.span);
 const names=values.map(()=>checker.freshName('ml_same_value_operand'));
 const missing={k:'none',absence:'undefined',type:option(UNIT,'undefined')};
 let body=sameValue(checker,values[0]?variable(names[0],values[0].type):missing,values[1]?variable(names[1],values[1].type):missing);
 for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:BOOL};
 return body;
}
