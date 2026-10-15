// Host primitive graph access and exact scalar transport; no owner classifier.
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {originalLinkIdentityIndex,originalLinkIdentityWords} from '../original-link-identity/policy.mjs';
import {readOriginalLinkIdentityOrdinals} from '../original-link-identity/rules.mjs';
import {decodeRustTextCertificateFacet} from '../rust-text-certificate-adapter/raw-identity.mjs';
import {projectOriginalLinkRoleRegistry} from './rules.mjs';
export const ORIGINAL_LINK_ROLE_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
const policy=ORIGINAL_LINK_ROLE_POLICY;
const fail=message=>{throw new TypeError('Original link certificate roles: '+message);};
const fieldName=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());

export function readOriginalLinkRoleTransport(network,root){
 // Existing complete owner/grammar/condition decisions remain mandatory.
 const checked=decodeRustTextCertificateFacet(network,root);if(!checked)fail('the actual selected text certificate is absent');
 const index=originalLinkIdentityIndex(network.links().map(link=>link.id()));
 const ordinal=ids=>{
  const result=[];
  for(let start=0;start<ids.length;start+=policy.maximumMembership){
   const words=ids.slice(start,start+policy.maximumMembership).map(originalLinkIdentityWords);
   const values=readOriginalLinkIdentityOrdinals(index.high,index.low,words.map(row=>row[0]),words.map(row=>row[1]));
   if(values.some(value=>value<=0))fail('an original reference has no exact registered membership');result.push(...values);
  }
  return result;
 };
 const nodes=index.anchors.map(row=>network.link(row.original));
 const relations=network.fieldRelations().map(row=>({relation:row.link.id(),parent:row.parent,label:row.link.references()[1],child:row.child,name:row.label}));
 const references=nodes.flatMap(node=>node.references().map(child=>({parent:node.id(),child})));
 if(relations.length>policy.maximumEdges||references.length>policy.maximumEdges)fail('primitive transport exceeds its edge bound');
 return {
  checked,anchors:index.anchors.map(row=>row.original),relations,references,
  arguments:[index.anchors.length,ordinal([root])[0],ordinal(relations.map(row=>row.parent)),relations.map(row=>policy.fields.indexOf(fieldName(row.name))+1),ordinal(relations.map(row=>row.child)),ordinal(references.map(row=>row.parent)),ordinal(references.map(row=>row.child)),nodes.map(node=>node.metadata().term===schema.scalars.string?policy.stringSymbols.indexOf(node.metadata().definition)+1:0)],
 };
}

export function projectOriginalLinkCertificateRoles(network,root){
 const transport=readOriginalLinkRoleTransport(network,root),result=projectOriginalLinkRoleRegistry(...transport.arguments);
 if(result[0]!==1)fail('the generated role registry refused at '+result.join(','));
 const count=result[1],occurrences=result[2],end=3+count;
 const members=result.slice(3,end),kinds=result.slice(end,end+count+1),handles=result.slice(end+count+1);
 if(handles.length!==occurrences)fail('the generated registry changed its declared output layout');
 return {transport,members,kinds,handles,anchors:members.map(member=>transport.anchors[member-1])};
}
