// Typed host boundary for source-defined order operations and owned projection.
import {FLOAT,BOOL,STRING,array} from './types.js';
import {coerce} from './check.js';
import {typeError} from './diagnostics.js';
import {readJavaScriptRecordOrderKernelArity,readJavaScriptRecordOrderKernelParameter,readJavaScriptRecordOrderKernelResult,acceptLinkedOrderedRecordOrderRead} from './javascript-enumeration-rules.js';
export function readRecordOrderRoleType(role){
 if(role==='number')return FLOAT;
 if(role==='text')return STRING;
 if(role==='boolean')return BOOL;
 if(role==='positions')return array(FLOAT);
 if(role==='name rows')return array(array(FLOAT));
 if(role==='key texts')return array(STRING);
 throw typeError('an ordinary record order library operand has an unknown role');
}
export function checkRecordOrderOperation(checker,node,env,path,expected){
 const arity=readJavaScriptRecordOrderKernelArity(node.op??'');
 if(!arity)return null;
 if(checker.language!=='JavaScript'||node.args.length!==arity||node.args.some(argument=>argument.spread))throw typeError('an ordinary record order operation needs every owned source operand',node.span);
 if(node.op==='reorderJavaScriptRecordPropertyValues'){
  const values=checker.referenceValue(checker.expr(node.args[0].value,env,path,expected?.kind==='array'?expected:undefined));
  if(values.type.kind!=='array'||values.type.reference===true)throw typeError('record value ordering needs its actual owned value array',node.span);
  const positions=array(FLOAT),order=coerce(checker.referenceValue(checker.expr(node.args[1].value,env,path,positions)),positions,checker.language,node.span);
  return {k:'math',op:node.op,args:[values,order],type:values.type,...(node.span?{span:node.span}:{})};
 }
 const args=node.args.map((argument,index)=>{
  const type=readRecordOrderRoleType(readJavaScriptRecordOrderKernelParameter(node.op,index));
  return coerce(checker.referenceValue(checker.expr(argument.value,env,path,type)),type,checker.language,node.span);
 });
 return {k:'math',op:node.op,args,type:readRecordOrderRoleType(readJavaScriptRecordOrderKernelResult(node.op)),...(node.span?{span:node.span}:{})};
}
export function checkOrderedRecordOrderRead(node,object){
 if(node.op!=='readJavaScriptRecordPropertyOrder')return null;
 if(!acceptLinkedOrderedRecordOrderRead('method',node.op,object.type.kind,object.type.ordered===true,object.type.reference===true)||node.args.length)throw typeError('own-field order projection needs one actual owned ordered record',node.span);
 return {k:'method',op:node.op,object,args:[],type:array(FLOAT)};
}
