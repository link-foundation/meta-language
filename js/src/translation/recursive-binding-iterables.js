// Actual typed source operands into existing iterable and exception terms.
import {BOOL,STRING} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {checkJavaScriptIterableValue} from './array-formal-iterables.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
import {readJavaScriptRecursiveBindingIterableForm,readJavaScriptRecursiveBindingIterableError} from './recursive-binding-rules.js';
const literal=(type,value)=>({k:'lit',type,value});
const iterable=(value,span)=>value.type.kind==='tuple'?value:checkJavaScriptIterableValue(value,span);
export function checkJavaScriptRecursiveBindingIterable(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.length!==1||node.args[0].spread)throw typeError('a recursive binding iterator needs one actual operand',node.span);
 const value=checker.expr(node.args[0].value,env,path,undefined),form=readJavaScriptRecursiveBindingIterableForm(value.type.kind,value.type.element?.kind??'');
 if(form==='wait'||form==='refuse')throw unsupported('recursive binding iterator','the actual source value needs its complete operational iterator',node.span);
 if(form!=='optional')return iterable(value,node.span);
 const name=checker.freshName('ml_recursive_binding_iterable'),bound={k:'var',name,type:value.type},present=value.type.element;
 const body=iterable({k:'unwrapOption',arg:bound,type:present},node.span);
 let message=literal(STRING,readJavaScriptRecursiveBindingIterableError(value.type.absence==='null'?'null':'undefined'));
 if(value.type.absence==='both')message={k:'if',cond:{k:'strictEqual',left:bound,right:{k:'none',absence:'null',type:value.type},negate:false,type:BOOL},then:literal(STRING,readJavaScriptRecursiveBindingIterableError('null')),else:message,type:STRING};
 const failure={k:'math',op:readJavaScriptSourceExceptionOperation('TypeError'),args:[message,literal(BOOL,true)],type:body.type};
 return {k:'let',name,value,body:{k:'if',cond:constructJavaScriptOptionalPresence(bound,present,node.span),then:body,else:failure,type:body.type},type:body.type};
}
