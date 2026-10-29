// Match the complete checked result-column protocol, including its failure.
// A name/signature/marker never authorizes replacing an arbitrary source body.
import {readJavaScriptBigIntegerResultOperation,readJavaScriptBigIntegerDivisionMessage,readJavaScriptBigIntegerExponentMessage,readJavaScriptBigIntegerConversionErrorClass,readJavaScriptBigIntegerConversionMessagePieces} from './big-integer-conversion-rules.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';
const owns=(node,keys)=>node&&typeof node==='object'&&Object.keys(node).every(key=>[...keys,'span'].includes(key));
const scalar=(node,kind,value)=>owns(node,['k','type','value'])&&node.k==='lit'&&node.type?.kind===kind&&node.value===value;
const integer=type=>type?.kind==='int';
const column=type=>type?.kind==='array'&&!type.reference&&integer(type.element);
const reference=(node,name)=>owns(node,['k','type','name'])&&node.k==='var'&&node.name===name&&column(node.type);
function sameOperand(first,second){return owns(first,['k','type','name'])&&owns(second,['k','type','name'])&&first.k==='var'&&second.k==='var'&&first.name===second.name&&first.type?.kind===second.type?.kind;}
function message(node,form,operand){
 if(form==='power')return scalar(node,'string',readJavaScriptBigIntegerExponentMessage());
 if(form==='division')return scalar(node,'string',readJavaScriptBigIntegerDivisionMessage());
 const pieces=readJavaScriptBigIntegerConversionMessagePieces(form);
 if(!owns(node,['k','op','left','right','type'])||node.k!=='binary'||node.op!=='concat'||node.type?.kind!=='string'||!scalar(node.right,'string',pieces[1]))return false;
 const prefix=node.left;
 if(!owns(prefix,['k','op','left','right','type'])||prefix.k!=='binary'||prefix.op!=='concat'||prefix.type?.kind!=='string'||!scalar(prefix.left,'string',pieces[0]))return false;
 const displayed=prefix.right;
 return form==='string'?sameOperand(displayed,operand):owns(displayed,['k','arg','type'])&&displayed.k==='toString'&&displayed.type?.kind==='string'&&sameOperand(displayed.arg,operand);
}
export function readCheckedJavaScriptIntegerResult(node){
 if(!owns(node,['k','name','value','body','type'])||node.k!=='let'||!integer(node.type)||typeof node.name!=='string'||!node.name)return null;
 const operation=node.value,body=node.body;
 if(!owns(operation,['k','op','args','type'])||operation.k!=='math'||!column(operation.type)||!Array.isArray(operation.args))return null;
 const form=readJavaScriptBigIntegerResultOperation(operation.op);
 if(!['power','division','string','number'].includes(form))return null;
 const count=form==='power'||form==='division'?2:1;
 if(operation.args.length!==count||operation.args.some(arg=>arg.type?.kind!==(form==='string'?'string':form==='number'?'float':'int')))return null;
 if(!owns(body,['k','cond','then','else','type'])||body.k!=='if'||!integer(body.type))return null;
 const condition=body.cond;
 if(!owns(condition,['k','op','left','right','type','domain'])||condition.k!=='binary'||condition.op!=='eq'||condition.type?.kind!=='bool'||condition.domain?.kind!=='float'||!scalar(condition.right,'float',form==='division'?'2':'1'))return null;
 const length=condition.left;
 if(!owns(length,['k','array','type'])||length.k!=='length'||length.type?.kind!=='float'||!reference(length.array,node.name))return null;
 const success=body.then;
 if(!owns(success,['k','left','right','type'])||success.k!=='coalesce'||!integer(success.type)||!scalar(success.right,'int','0'))return null;
 const read=success.left;
 if(!owns(read,['k','array','index','type'])||read.k!=='optionalIndex'||read.type?.kind!=='option'||read.type.absence!=='undefined'||!integer(read.type.element)||!reference(read.array,node.name))return null;
 if(!scalar(read.index,'float','0')&&!(form==='division'&&scalar(read.index,'float','1')))return null;
 const failure=body.else,errorClass=form==='power'||form==='division'?'RangeError':readJavaScriptBigIntegerConversionErrorClass(form);
 if(!owns(failure,['k','op','args','type'])||failure.k!=='math'||failure.op!==readJavaScriptSourceExceptionOperation(errorClass)||!integer(failure.type)||!Array.isArray(failure.args)||failure.args.length!==2||!scalar(failure.args[1],'bool',true)||!message(failure.args[0],form,operation.args[0]))return null;
 return {operation:form==='division'?(read.index.value==='0'?'divide integers':'read integer remainder'):form==='power'?'raise integer power':'convert explicit integer',arguments:operation.args};
}
