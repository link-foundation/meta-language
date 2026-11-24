import {checkJavaScriptOrdinaryClassSourceClosure,createJavaScriptOrdinaryClassDeclarationWitness} from './javascript-ordinary-class-closure-source.js';
import {prepareJavaScriptOrdinaryClassModule} from './javascript-ordinary-class-module.js';
export function prepareJavaScriptOrdinaryClassSourceClosure(input,{realmIdentity}={}){
 const archive=checkJavaScriptOrdinaryClassSourceClosure(input),declarationWitness=createJavaScriptOrdinaryClassDeclarationWitness(archive),module=prepareJavaScriptOrdinaryClassModule(archive.capture,{realmIdentity,declarationWitness});
 return {archive,capture:archive.capture,embedding:module.embedding,collection:module.collection,
  begin(exported,args){const entries=archive.entries.filter(row=>row.exported===exported);if(entries.length!==1)throw new TypeError('ordinary class source calls require their actual public resolved entry owner');return module.begin(entries[0].compiledName,args);}
 };
}
