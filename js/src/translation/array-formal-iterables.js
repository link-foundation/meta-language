// Source wrappers disappear into established typed operations before archiving.
import {typeError,unsupported} from './diagnostics.js';
import {readArrayIterableForm} from './array-value-rules.js';
import {readJavaScriptArrayFormalIterableForm} from './array-formal-parameter-rules.js';
import {array,STRING} from './types.js';
export function checkJavaScriptIterableValue(object,where){
 const form=readArrayIterableForm(object.type.kind,object.type.reference===true);
 if(form==='array')return object;
 if(form==='string')return {k:'stringPoints',object,type:array(STRING)};
 if(form==='mutable')throw unsupported('mutable collection iteration','live collection iteration needs ordered entries and tombstones rather than a snapshot',where);
 if(form==='set')return {k:'collectionItems',object,op:'values',type:array(object.type.element)};
 if(form==='map')return {k:'collectionItems',object,op:'entries',type:array({kind:'tuple',elements:[object.type.key,object.type.value]})};
 throw unsupported('for-of iterator','this value needs an operational iterator representation',where);
}
export function checkJavaScriptArrayFormalIterable(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.length!==1||node.args[0].spread)throw typeError('an array formal iterator needs one actual operand',node.span);
 const object=checker.expr(node.args[0].value,env,path,undefined);
 return readJavaScriptArrayFormalIterableForm(object.type.kind,object.type.reference===true)==='tuple'?object:checkJavaScriptIterableValue(object,node.span);
}
