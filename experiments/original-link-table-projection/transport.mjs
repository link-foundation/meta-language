// Primitive raw payload collection only; no source operation/ownership decisions.
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {projectOriginalLinkCertificateRoles} from '../original-link-role-projection/transport.mjs';
import {projectOriginalLinkCertificateTables} from './rules.mjs';
export const ORIGINAL_LINK_TABLE_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
const policy=ORIGINAL_LINK_TABLE_POLICY;
const fieldName=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
export function readOriginalLinkTableTransport(network,root){
 const role=projectOriginalLinkCertificateRoles(network,root),transport=role.transport;
 const payloads=transport.anchors.map(id=>{
  const metadata=network.link(id).metadata();
  if(metadata.term===schema.scalars.null)return{kind:1,number:0,symbol:0};
  if(metadata.term===schema.scalars.string)return{kind:2,number:0,symbol:policy.symbols.indexOf(metadata.definition)+1};
  if(metadata.term===schema.scalars.boolean)return{kind:4,number:0,symbol:0};
  if(metadata.term===schema.scalars.number){try{const value=JSON.parse(metadata.definition);if(typeof value==='number')return{kind:3,number:value,symbol:0};}catch{}}
  return{kind:0,number:0,symbol:0};
 });
 return {role,arguments:[...transport.arguments.slice(0,3),transport.relations.map(row=>policy.fields.indexOf(fieldName(row.name))+1),...transport.arguments.slice(4,7),payloads.map(row=>row.kind),payloads.map(row=>row.number),payloads.map(row=>row.symbol)]};
}
export function projectOriginalLinkFixedSymbolTables(network,root){
 const transport=readOriginalLinkTableTransport(network,root),rows=projectOriginalLinkCertificateTables(...transport.arguments);
 if(rows.length!==policy.output.length)throw new TypeError('Original link fixed-symbol tables: projection refused at '+rows[0]?.join(','));
 return {transport,tables:Object.fromEntries(policy.output.map((name,index)=>[name,rows[index]]))};
}
