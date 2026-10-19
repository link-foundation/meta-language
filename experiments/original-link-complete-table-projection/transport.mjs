// Only primitive metadata/UTF16 collection. No old projected table is an input.
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {readOriginalLinkTableTransport} from '../original-link-table-projection/transport.mjs';
import {evaluateRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {projectOriginalLinkCompleteCertificateTables} from './rules.mjs';
export const ORIGINAL_LINK_COMPLETE_TABLE_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
const policy=ORIGINAL_LINK_COMPLETE_TABLE_POLICY;
const fieldName=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
function primitiveStrings(values){const starts=[],lengths=[],units=[];for(const value of values){starts.push(units.length);lengths.push(typeof value==='string'?value.length:-1);if(typeof value==='string')for(let at=0;at<value.length;at++)units.push(value.charCodeAt(at));}return[starts,lengths,units];}
export function readOriginalLinkCompleteTableTransport(network,root){
 // This unchanged entry authenticates the original raw graph first. Its old
 // projected tables are retained only for callers' independent comparisons.
 const fixed=readOriginalLinkTableTransport(network,root),base=fixed.role.transport;
 const nodes=base.anchors.map(id=>network.link(id));
 const strings=primitiveStrings(nodes.map(node=>node.metadata().term===schema.scalars.string?(node.metadata().definition??''):undefined));
 const booleans=nodes.map(node=>node.metadata().term===schema.scalars.boolean?(node.metadata().definition==='false'?0:node.metadata().definition==='true'?1:-1):-1);
 const dictionary=primitiveStrings(policy.fixedSymbols);
 return {fixed,arguments:[...fixed.arguments.slice(0,3),base.relations.map(row=>policy.fields.indexOf(fieldName(row.name))+1),...fixed.arguments.slice(4),booleans,...strings,...dictionary]};
}
export function projectOriginalLinkCompleteTables(network,root){const transport=readOriginalLinkCompleteTableTransport(network,root),rows=projectOriginalLinkCompleteCertificateTables(...transport.arguments);if(rows.length!==policy.output.length)throw new TypeError('Complete original-link table projection refused at '+rows[0]?.join(','));const tables=Object.fromEntries(policy.output.map((name,index)=>[name,rows[index]])),decision=evaluateRustTextCertificate(tables);if(decision[0]!==1)throw new TypeError('Complete original-link certificate refused at '+decision.join(','));return{transport,tables,decision};}
