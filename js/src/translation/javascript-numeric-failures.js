// Actual checked IR adapter. Shared rules select the complete source error.
import {FLOAT,STRING,BOOL} from './types.js';
import {readJavaScriptNumericOperationFailure,readJavaScriptNumericOperationMessage} from './source-exception-rules.js';
export function materializeJavaScriptNumericFailure(checker,operation,args,expected){
 if(checker.language!=='JavaScript'||args.length!==2)return null;
 const failure=readJavaScriptNumericOperationFailure(operation,args[0].type.kind,args[1].type.kind);
 if(!failure)return null;
 const type=expected??FLOAT;
 let body={k:'math',op:'raiseJavaScriptTypeError',args:[{k:'lit',value:readJavaScriptNumericOperationMessage(failure),type:STRING},{k:'lit',value:true,type:BOOL}],type};
 const names=args.map(()=>checker.freshName('ml_numeric_failure_operand'));
 for(let index=args.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:args[index],body,type};
 return body;
}
