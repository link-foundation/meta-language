// Actual field/default operands disappear into the existing binding default IR.
import {acceptJavaScriptRecordFormalDefaultSource,readJavaScriptRecordFormalDefaultFieldForm} from './javascript-record-parameter-rules.js';
import {typeError,unsupported} from './diagnostics.js';
import {UNIT,option} from './types.js';
import {checkJavaScriptBindingDefault,checkJavaScriptBindingDefaultValue} from './javascript-binding-defaults.js';
export function readJavaScriptRecordFormalDefaultOperands(node,language='JavaScript'){
 const args=node.args;
 if(!acceptJavaScriptRecordFormalDefaultSource(language,args.length,args.some(argument=>argument.spread),args[1]?.value?.k??''))throw typeError('a record formal default needs its exact receiver, literal property and original initializer',node.span);
 return {value:{k:'field',object:args[0].value,field:args[1].value.value,span:node.span},fallback:args[2].value,span:node.span};
}
export function checkJavaScriptRecordFormalDefault(checker,node,env,path){
 const operands=readJavaScriptRecordFormalDefaultOperands(node,checker.language),owner=checker.expr(operands.value.object,env,path,undefined),field=operands.value.field;
 const form=readJavaScriptRecordFormalDefaultFieldForm(owner.type.kind,owner.type.open===true,owner.type.fields?.some(item=>item.name===field)??false,field);
 if(form==='prototype')throw unsupported('record formal inherited property','an absent own field cannot replace the actual prototype property '+field,node.span);
 if(form!=='absent')return checkJavaScriptBindingDefault(checker,operands,env,path);
 const body=checkJavaScriptBindingDefaultValue(checker,{k:'none',absence:'undefined',type:option(UNIT,'undefined')},operands.fallback,node.span,env,path);
 return {k:'let',name:checker.freshName('ml_record_default_receiver'),value:owner,body,type:body.type};
}
