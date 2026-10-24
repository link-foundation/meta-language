// Actual checked expression/type adapters for source exception intrinsics.
import {STRING,BOOL,UNIT} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
import {readJavaScriptSourceExceptionMessageForm,readJavaScriptSourceExceptionAbsentMessage,readJavaScriptExceptionConstructorClass,readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readOptionalOutputAbsences} from './optional-value-rules.js';

function acceptsOptions(type) {
  return type.kind==='option'?acceptsOptions(type.element):['string','bool','float','int','nat','unit'].includes(type.kind);
}
function messageValue(checker,value,span) {
  const form=readJavaScriptSourceExceptionMessageForm(value.type.kind);
  if(form==='identity')return value;
  if(form==='text')return{k:'toString',arg:value,type:STRING};
  if(form==='empty')return{k:'let',name:checker.freshName('ml_exception_message_unused'),value,body:{k:'lit',value:'',type:STRING},type:STRING};
  if(form!=='optional')throw unsupported('exception message primitive conversion',
    'the actual object or symbol conversion needs its complete source operation',span);
  const name=checker.freshName('ml_exception_message');
  const variable={k:'var',name,type:value.type};
  let body=messageValue(checker,{k:'unwrapOption',arg:variable,type:value.type.element},span);
  for(const absence of readOptionalOutputAbsences(value.type.absence).reverse())body={k:'if',
    cond:{k:'strictEqual',left:variable,right:{k:'none',absence,type:value.type},negate:false,type:BOOL},
    then:{k:'lit',value:readJavaScriptSourceExceptionAbsentMessage(absence),type:STRING},else:body,type:STRING};
  return{k:'let',name,value,body,type:STRING};
}

export function checkJavaScriptSourceException(checker,node,env,path,expected) {
  if(checker.language!=='JavaScript')throw unsupported('JavaScript exception operation','the operation needs its actual JavaScript source owner',node.span);
  expected??=UNIT;
  if(node.args.some(item=>item.spread))throw unsupported('spread exception arguments','the source iterator and complete argument effects need their represented operations',node.span);
  const values=node.args.map(item=>checker.expr(item.value,env,path,undefined));
  if(values.length>1&&!acceptsOptions(values[1].type))throw unsupported('exception cause options',
    'the options object and cause property need their actual property effects',node.span);
  const capture=values.length>1||values[0]?.type.kind==='option';
  const names=capture?values.map(()=>checker.freshName('ml_exception_argument')):[];
  const first=capture?{k:'var',name:names[0],type:values[0].type}:values[0];
  const message=first?messageValue(checker,first,node.span):{k:'lit',value:'',type:STRING};
  const present=first?.type.kind==='option'&&first.type.absence!=='null'
    ?{k:'strictEqual',left:first,right:{k:'none',absence:'undefined',type:first.type},negate:true,type:BOOL}
    :{k:'lit',value:Boolean(first&&first.type.kind!=='unit'),type:BOOL};
  let body={k:'math',op:readJavaScriptSourceExceptionOperation(readJavaScriptExceptionConstructorClass(node.op)),args:[message,present],type:expected};
  for(let index=values.length-1;capture&&index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:expected};
  return body;
}

export function checkJavaScriptSourceExceptionValue(checker,node,env,path,expected) {
  if(checker.language!=='JavaScript'||node.args.length!==2||node.args.some(item=>item.spread))throw typeError('a source exception needs its checked message and property presence',node.span);
  const args=node.args.map(item=>checker.expr(item.value,env,path,undefined));
  if(args[0].type.kind!=='string'||args[1].type.kind!=='bool')throw typeError('a source exception needs a String message and Boolean property presence',node.span);
  return{k:'math',op:node.op,args,type:expected??UNIT};
}
