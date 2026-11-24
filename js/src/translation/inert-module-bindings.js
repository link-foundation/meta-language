// Actual complete CST facts; the shared rule owns every form decision.
import {parseNative} from '../native-grammar-parser.js';
import {readInertModuleBinding} from './inert-module-binding-rules.js';
export function readInertModuleSourceFacts(source){
 if(typeof source!=='string'||Buffer.byteLength(source)>65536)return null;
 const bytes=Buffer.from(source),root=parseNative('native-javascript',source),nodes=[],terms=[],fields=[],parents=[],starts=[],ends=[],texts=[],errors=[];
 const visit=(node,field,parent)=>{const at=nodes.length;nodes.push(node);terms.push(node.term);fields.push(field??'');parents.push(parent);starts.push(node.start);ends.push(node.end);texts.push(node.children.length===0?bytes.subarray(node.start,node.end).toString('utf8'):'');errors.push(node.isError||node.isMissing||node.hasError?1:0);for(const child of node.children)visit(child.node,child.field,at);};
 visit(root,'',-1);return{nodes,terms,fields,parents,starts,ends,texts,errors,sourceBytes:bytes.length};
}
export function readInertModuleBindingDeclaration(source){
 const facts=readInertModuleSourceFacts(source);if(!facts)return null;
 const selected=readInertModuleBinding(facts.terms,facts.fields,facts.parents,facts.starts,facts.ends,facts.texts,facts.errors,facts.sourceBytes);
 if(selected.length!==2)return null;
 const[at,owner]=selected,node=facts.nodes[at];if(!node||node.term!=='identifier'||node.children.length||node.end<=node.start)return null;
 const name=Buffer.from(source).subarray(node.start,node.end).toString('utf8');if(name!==facts.texts[at])return null;
 return{name,owner,nameRange:{start:node.start,end:node.end},coordinateUnit:'UTF-8 bytes'};
}
