// Source/coordinate transport only; shared rules own syntax admission.
import {readInertModuleSourceFacts} from './inert-module-bindings.js';
import {readEmptyStandardCollectionModuleBinding} from './empty-module-collection-rules.js';
export function readEmptyStandardCollectionBindingDeclaration(source){
 const facts=readInertModuleSourceFacts(source);if(!facts)return null;
 const selected=readEmptyStandardCollectionModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
 if(selected.length!==2)return null;
 const[at,owner]=selected,node=facts.nodes[at];if(!node||node.term!=='identifier'||node.children.length||node.end<=node.start)return null;
 const name=Buffer.from(source).subarray(node.start,node.end).toString('utf8');if(name!==facts.texts[at])return null;
 return{name,owner,nameRange:{start:node.start,end:node.end},coordinateUnit:'UTF-8 bytes'};
}
