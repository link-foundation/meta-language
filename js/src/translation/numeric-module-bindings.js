// Source/coordinate transport only; the shared complete-CST rule decides.
import {readInertModuleSourceFacts} from './inert-module-bindings.js';
import {readNumericModuleBinding,acceptNumericModuleBindingSignature} from './numeric-module-binding-rules.js';
export function readNumericModuleBindingDeclaration(source){
 const facts=readInertModuleSourceFacts(source);if(!facts)return null;
 const selected=readNumericModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
 if(selected.length!==2)return null;
 const[at,owner]=selected,node=facts.nodes[at];if(!node||node.term!=='identifier'||node.children.length||node.end<=node.start)return null;
 const name=Buffer.from(source).subarray(node.start,node.end).toString('utf8');if(name!==facts.texts[at])return null;
 return{name,owner,nameRange:{start:node.start,end:node.end},coordinateUnit:'UTF-8 bytes'};
}

// The signature is the actual checked/emitted sibling's existing record.
export function acceptNumericModuleBindingDeclarationSignature(source,signature){
 if(signature.k!=='const'||signature.literal||signature.type?.kind!=='float')return false;
 const actual=readNumericModuleBindingDeclaration(source);
 return acceptNumericModuleBindingSignature(signature.k,signature.type.kind,signature.literal??false,actual?.name??'',signature.name);
}
