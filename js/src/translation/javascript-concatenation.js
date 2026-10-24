// Capture actual handles/values before reading payloads; existing IR supplies
// evaluation, reference reads, primitive text conversion and array append.
import{coerce}from'./check.js';
import{unsupported,typeError}from'./diagnostics.js';
import{STRING,array}from'./types.js';
import{materializeJavaScriptPrimitiveText}from'./javascript-string-search.js';
import{readJavaScriptConcatenationArgumentRole}from'./javascript-property-call-rules.js';

export function checkJavaScriptConcatenation(checker,node,env,path){
 const receiver=checker.expr(node.object,env,path,undefined),kind=receiver.type.kind;
 if(kind!=='array'&&kind!=='string')return null;
 const values=[receiver];
 for(const arg of node.args){
  if(!arg)throw typeError('a concat call must retain every actual argument',node.span);
  values.push(checker.expr(arg,env,path,undefined));
 }
 const inner=new Map(env),bindings=[];
 for(const value of values){
  let name;do{name=checker.freshName('ml_concat_value');}while(inner.has(name));
  inner.set(name,value.type);bindings.push({name,value});
 }
 const variable=index=>({k:'var',name:bindings[index].name,type:values[index].type});
 const type=kind==='array'?array(receiver.type.element):STRING;
 let body=kind==='array'?{k:'append',left:{k:'array',items:[],type},right:checker.referenceValue(variable(0)),type}:variable(0);
 for(let index=1;index<values.length;index+=1){
  const role=readJavaScriptConcatenationArgumentRole(kind,values[index].type.kind,true);
  if(role==='unsupported')throw unsupported('String concat coercion','this argument needs its complete ToPrimitive interpretation',node.span);
  if(kind==='string'){
   const text=materializeJavaScriptPrimitiveText(variable(index),node.span);
   body={k:'binary',op:'concat',left:body,right:text,type};
  }else{
   const value=role==='spread'?coerce(checker.referenceValue(variable(index)),type,checker.language,node.span):coerce(variable(index),receiver.type.element,checker.language,node.span);
   const part=role==='spread'?value:{k:'array',items:[value],type};
   body={k:'append',left:body,right:part,type};
  }
 }
 for(const binding of bindings.reverse())body={k:'let',name:binding.name,value:binding.value,body,type};
 return body;
}
