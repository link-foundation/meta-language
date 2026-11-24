// Actual receiver/function terms are supplied by the ordinary inferencer and
// checker. Shared source rules select a declared helper, never a guessed type.
import {readJavaScriptClosedClassMethodTarget,acceptJavaScriptClosedClassMethodDefinition} from './javascript-class-source-rules.js';
import {unsupported} from './diagnostics.js';
export function readClosedClassMethodTarget(methods,type,method){return type.kind==='record'?readJavaScriptClosedClassMethodTarget(methods,type.fields.map(field=>field.name),method):'';}
export function validateClosedClassMethodDefinitions(checker,methods){
 if(!Array.isArray(methods)||methods.length%3!==0||methods.length>12288||methods.some(value=>typeof value!=='string'||value.length===0))throw unsupported('closed class method table','complete bounded source-owned brand, method and helper triples are required');
 if(methods.length&&(checker.language!=='JavaScript'||checker.checkedProjection))throw unsupported('closed class method table','only actual JavaScript source may supply a prototype dispatch table');
 const owners=new Set();
 for(let at=0;at<methods.length;at+=3){
  const [brand,method,helper]=methods.slice(at,at+3),entry=checker.items.get(helper),key=brand+'\0'+method;
  const marker=brand.lastIndexOf('class_identity_'),prefix=marker<0?'':brand.slice(0,marker);
  const constructor=[...checker.items.values()].some(item=>item.k==='fn'&&!item.name.startsWith(prefix+'class_method_')&&item.ret?.kind==='record'&&item.ret.fields.some(field=>field.name===brand));
  if(owners.has(key)||entry?.k!=='fn'||entry.modulePath.length!==0||!acceptJavaScriptClosedClassMethodDefinition(brand,method,helper,entry.params[0]?.name??'',entry.params[0]?.type.kind==='record'?entry.params[0].type.fields.map(field=>field.name):[],constructor))throw unsupported('closed class method table','every actual brand needs its constructor and declared receiver helper');
  owners.add(key);
 }
}
export function checkClosedClassMethodCall(checker,node,receiver,env,path,expected){
 const helper=readClosedClassMethodTarget(checker.sourceClassMethods,receiver.type,node.op);if(!helper)return null;
 const name=checker.freshName('ml_class_receiver'),inner=new Map(env);inner.set(name,receiver.type);
 const body=checker.application({k:'name',path:[helper]},[{k:'name',path:[name],span:node.object.span},...node.args],inner,path,expected,node.span);
 return {k:'let',name,value:receiver,body,type:body.type,span:node.span};
}
