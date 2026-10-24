// Type/child transport into the existing checked string or callable algebra.
import{unsupported}from'./diagnostics.js';
import{checkJavaScriptStringRange}from'./javascript-string-ranges.js';
import{checkJavaScriptConcatenation}from'./javascript-concatenation.js';
import{checkJavaScriptStringSearch,checkJavaScriptStringMapCall}from'./javascript-string-search.js';
import{readJavaScriptSourcePropertyOperationKind,readJavaScriptResolvedPropertyCallForm}from'./javascript-property-call-rules.js';

export function checkJavaScriptSourcePropertyCall(checker,node,env,path){
 if(checker.language==='JavaScript'&&node.op==='concat')return checkJavaScriptConcatenation(checker,node,env,path);
 if(checker.language!=='JavaScript'||!readJavaScriptSourcePropertyOperationKind(node.op))return null;
 const receiver=checker.expr(node.object,env,path,undefined),object=checker.referenceValue(receiver);
 const form=readJavaScriptResolvedPropertyCallForm(object.type.kind,node.op);
 if(form==='record'||form==='')return null;
 if(form==='range')return checkJavaScriptStringRange(checker,node,object,env,path);
 if(form==='test'||form==='index')return checkJavaScriptStringSearch(checker,node,object,env,path);
 if(form==='map')return checkJavaScriptStringMapCall(checker,node,object,env,path);
 if(node.args.length)throw unsupported('toString with a radix','only decimal text is portable',node.span);
 return checker.expr({k:'toString',arg:node.object,span:node.span},env,path,undefined);
}
