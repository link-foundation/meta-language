// Actual field/default operands disappear into the existing binding default IR.
import {acceptJavaScriptRecordFormalDefaultSource} from './javascript-record-parameter-rules.js';
import {typeError} from './diagnostics.js';
export function readJavaScriptRecordFormalDefaultOperands(node,language='JavaScript'){
 const args=node.args;
 if(!acceptJavaScriptRecordFormalDefaultSource(language,args.length,args.some(argument=>argument.spread),args[1]?.value?.k??''))throw typeError('a record formal default needs its exact receiver, literal property and original initializer',node.span);
 return {value:{k:'field',object:args[0].value,field:args[1].value.value,span:node.span},fallback:args[2].value,span:node.span};
}
