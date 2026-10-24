// Actual checked child transport into the existing shared Truthy algebra.
import {BOOL} from './types.js';
import {unsupported} from './diagnostics.js';
export function checkJavaScriptBooleanBuiltin(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.some(argument=>argument.spread))throw unsupported('Boolean arguments','the complete actual iterator effects must be represented',node.span);
 const values=node.args.map(argument=>checker.expr(argument.value,env,path,undefined));
 const names=values.map(()=>checker.freshName('ml_boolean_argument'));
 let body=values.length?checker.booleanValue({k:'var',name:names[0],type:values[0].type},node.span):{k:'lit',value:false,type:BOOL};
 for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type:BOOL};
 return body;
}
