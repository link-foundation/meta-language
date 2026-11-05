// Actual operands/types into the existing Let/If/Apply/optional/record algebra.
import {BOOL,STRING,UNIT,option} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {coerce} from './check.js';
import {readCallableApplicationError} from './callable-source-rules.js';
import {mergeOptionalAbsence} from './structured-type-rules.js';
import {acceptJavaScriptOptionalCallHead,readJavaScriptOptionalCallForm,readJavaScriptOptionalCallResultForm,readJavaScriptOptionalCallField} from './javascript-optional-call-rules.js';
const absent=type=>({k:'none',absence:'undefined',type});
export function checkJavaScriptOptionalCall(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.length===0||node.args.some(argument=>argument.spread))throw typeError('an optional call needs its actual receiver and ordered non-spread arguments',node.span);
 const source=node.args[0].value;
 if(!acceptJavaScriptOptionalCallHead(source.k,source.path?.length??0))throw unsupported('optional method call','the property reference needs its complete original receiver and chain extent',node.span);
 const value=checker.referenceValue(checker.expr(source,env,path,undefined));
 const form=readJavaScriptOptionalCallForm(value.type.kind,value.type.element?.kind??'');
 if(form==='unsupported')throw unsupported('optional callable','this source call needs its actual callable or nullish value and complete receiver protocol',node.span);
 const binding=checker.freshName('ml_optional_call_receiver'),field=readJavaScriptOptionalCallField();
 const type={kind:'record',fields:[{name:field,type:value.type}]};
 const captured={k:'var',name:binding,type};
 const receiver={k:'recordField',object:captured,field,index:0,type:value.type};
 let body;
 if(form==='absent')body=absent(option(UNIT,'undefined'));
 else{
  const signature=form==='direct'?value.type:value.type.element;
  const error=readCallableApplicationError(signature.kind,signature.params.length,node.args.length-1);
  if(error)throw typeError(error,node.span);
  const fn=form==='direct'?receiver:{k:'unwrapOption',arg:receiver,type:signature};
  const args=node.args.slice(1).map((argument,index)=>coerce(checker.expr(argument.value,env,path,signature.params[index]),signature.params[index],checker.language,argument.value.span??node.span));
  const call={k:'apply',fn,args,type:signature.ret};
  if(form==='direct')body=call;
  else{
   const outer=option(signature.ret,'undefined'),resultForm=readJavaScriptOptionalCallResultForm(signature.ret.kind);
   const success=resultForm==='undefined'?coerce(call,outer,checker.language,node.span):{k:'some',value:call,type:outer};
   const presence={k:'optionalTypeOf',arg:receiver,present:'function',type:STRING};
   const condition={k:'binary',op:'eq',left:presence,right:{k:'lit',value:'function',type:STRING},domain:STRING,type:BOOL};
   body={k:'if',cond:condition,then:success,else:absent(outer),type:outer};
   if(resultForm==='flatten')body={k:'flattenOption',value:body,type:option(signature.ret.element,mergeOptionalAbsence(signature.ret.absence,'undefined'))};
  }
 }
 return {k:'let',name:binding,value:{k:'record',entries:[{name:field,value}],type},body,type:body.type};
}
