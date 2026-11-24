// Actual parser-owned source/coordinate transport only.
import {readInertModuleSourceFacts} from './inert-module-bindings.js';
import {readIntrinsicModuleInitializer} from './intrinsic-initializer-rules.js';
export function readIntrinsicModuleInitializerDeclaration(source){
 const facts=readInertModuleSourceFacts(source);if(!facts)return null;
 const selected=readIntrinsicModuleInitializer(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
 if(selected.length!==2)return null;
 const [at,owner]=selected,node=facts.nodes[at];
 if(!node||node.term!=='identifier'||node.children.length||node.end<=node.start)return null;
 const name=Buffer.from(source).subarray(node.start,node.end).toString('utf8');if(name!==facts.texts[at])return null;
 return {name,owner,nameRange:{start:node.start,end:node.end},coordinateUnit:'UTF-8 bytes'};
}
/** Inspect actual name/required-owner facts only; no signature or execution. */
export function readIntrinsicModuleInitializerBinding(source){
 const value=readIntrinsicModuleInitializerDeclaration(source);
 return value?[value.name,value.owner]:null;
}
