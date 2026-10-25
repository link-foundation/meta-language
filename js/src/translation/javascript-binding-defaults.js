// Actual checked children/types; shared source rules select absence operations.
import {BOOL,UNIT,option,sameType} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {coerce} from './check.js';
import {readJavaScriptBindingDefaultForm,readJavaScriptBindingDefaultResultAbsence,readJavaScriptBindingDefaultLiftForm,readJavaScriptBindingDefaultLiftAbsences} from './array-binding-rules.js';
const none=(absence,type)=>({k:'none',absence,type});
const equalAbsence=(value,absence)=>({k:'strictEqual',left:value,right:none(absence,value.type),negate:false,type:BOOL});
function lift(checker,value,type,dropUndefined,span){
 const form=readJavaScriptBindingDefaultLiftForm(value.type.kind,value.type.element?.kind??'',value.type.absence??'',dropUndefined);
 if(form==='present')return coerce(value,type,'JavaScript',span);
 const name=checker.freshName('ml_binding_default_value'),bound={k:'var',name,type:value.type};
 let body;
 if(form==='undefined'||form==='null')body=none(form,type);
 else if(form==='absences')body={k:'if',cond:equalAbsence(bound,'null'),then:none('null',type),else:none('undefined',type),type};
 else{
  const present={k:'unwrapOption',arg:bound,type:bound.type.element};
  body=coerce(present,type,'JavaScript',span);
  for(const absence of readJavaScriptBindingDefaultLiftAbsences(bound.type.absence,dropUndefined).reverse())body={k:'if',cond:equalAbsence(bound,absence),then:none(absence,type),else:body,type};
 }
 return{k:'let',name,value,body,type};
}
export function checkJavaScriptBindingDefault(checker,node,env,path){
 const left=checker.expr(node.value,env,path,undefined);
 return checkJavaScriptBindingDefaultValue(checker,left,node.fallback,node.span,env,path);
}
export function checkJavaScriptBindingDefaultValue(checker,left,fallback,span,env,path){
 const form=readJavaScriptBindingDefaultForm(checker.language,left.type.kind,left.type.absence??'');
 if(form==='refuse-language'||form==='defer')throw unsupported('array binding default','the actual JavaScript value domain must be known',span);
 const element=left.type.kind==='option'?left.type.element:left.type;
 const right=checker.expr(fallback,env,path,element.kind==='unit'?undefined:element);
 if(form==='identity')return left;
 const other=right.type.kind==='option'?right.type.element:right.type;
 if(element.kind!=='unit'&&other.kind!=='unit'&&!sameType(element,other))throw typeError('array binding default and present element need compatible represented types',span);
 const base=element.kind==='unit'?other:element;
 const absence=readJavaScriptBindingDefaultResultAbsence(left.type.absence,right.type.kind,right.type.absence??'');
 const type=absence?option(base,absence):base;
 const name=checker.freshName('ml_binding_default_operand'),bound={k:'var',name,type:left.type};
 const selected=lift(checker,right,type,false,span);
 const body=left.type.element.kind==='unit'&&left.type.absence==='undefined'?selected:
  {k:'if',cond:equalAbsence(bound,'undefined'),then:selected,else:lift(checker,bound,type,true,span),type};
 return{k:'let',name,value:left,body,type};
}
