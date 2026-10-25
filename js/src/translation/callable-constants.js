// Actual source App nodes into the existing computed-application pipeline.
import {requireCallableConstantApplicationCapture} from './rust-module-value-rules.js';
export function captureCallableConstantApplicationHead(checker,head,env,path,span){
 if(checker.checkedProjection)return head;
 const local=head.k==='name'&&head.path.length===1&&env.has(head.path[0]);
 const entry=head.k==='name'&&!local?checker.lookup(head.path,path,span):null;
 if(!requireCallableConstantApplicationCapture(head.k,local,entry?.k??'',entry?.type?.kind??''))return head;
 const name=checker.freshName('ml_callable_constant');
 return {k:'let',name,value:head,body:{k:'name',path:[name],span},span};
}
