// Type/child transport into the existing checked string or callable algebra.
import{unsupported}from'./diagnostics.js';
import{checkJavaScriptConcatenation}from'./javascript-concatenation.js';
import{readJavaScriptSourcePropertyOperationKind,readJavaScriptResolvedPropertyCallForm}from'./javascript-property-call-rules.js';

export function checkJavaScriptSourcePropertyCall(checker,node,env,path){
 if(checker.language==='JavaScript'&&node.op==='concat')return checkJavaScriptConcatenation(checker,node,env,path);
 if(checker.language!=='JavaScript'||!readJavaScriptSourcePropertyOperationKind(node.op))return null;
 const receiver=checker.expr(node.object,env,path,undefined),object=checker.referenceValue(receiver);
 const form=readJavaScriptResolvedPropertyCallForm(object.type.kind,node.op);
 if(form==='record'||form==='')return null;
 if(form==='test'){
  if(node.args.length!==1||!node.args[0])throw unsupported('.'+node.op+'() with '+node.args.length+' arguments','search the whole string, with one argument',node.span);
  return checker.expr({k:'stringTest',op:node.op,object:node.object,search:node.args[0],span:node.span},env,path,undefined);
 }
 if(form==='map'){
  if(node.args.length!==0)throw unsupported('.'+node.op+'() with '+node.args.length+' arguments','string maps take no arguments',node.span);
  return checker.expr({k:'stringMap',op:node.op,object:node.object,span:node.span},env,path,undefined);
 }
 if(node.args.length)throw unsupported('toString with a radix','only decimal text is portable',node.span);
 return checker.expr({k:'toString',arg:node.object,span:node.span},env,path,undefined);
}
