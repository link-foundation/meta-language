// Reuse the existing raw-LinkId codec and portable certificate. This adapter
// changes only the entry path of the proof, never its nodes or ownership.
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../../js/src/typed-program-links.js';
import {LinkMetadata,LinkType} from '../../js/src/primitives.js';
import {acceptProgramSchemaRoot,readProgramContainerKind} from '../../js/src/typed-program-link-rules.js';
import {decodeGeneratedOwnedRust} from '../generated-owned-rust/codec.mjs';
import {checkPortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';

const facet='sourceTextValueProjection';
const key=id=>id.asU64().toString();
const fieldName=label=>Object.entries(schema.fields).find(([,value])=>value===label)?.[0]??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
const fail=reason=>{throw new TypeError('Rust text certificate facet: '+reason);};

export function decodeRustTextCertificateFacet(network,root){
 const fields=new Map();for(const row of network.fieldRelations()){const rows=fields.get(key(row.parent))??[];rows.push(row);fields.set(key(row.parent),rows);}
 const rootLink=network.link(root),body=rootLink?.references()[0];if(!body)return null;
 const markers=[],visited=new Set();let visits=0;
 function scan(id,depth=0){
  const idKey=key(id);if(visited.has(idKey))return;if(depth>256||++visits>100_000)fail('the selected raw graph exceeds its traversal bound');visited.add(idKey);
  const link=network.link(id);if(!link)return;
  for(const row of fields.get(idKey)??[]){if(fieldName(row.label)===facet)markers.push(row);scan(row.child,depth+1);}
  for(const child of link.references())scan(child,depth+1);
 }
 // User provenance is metadata, never an interpretation selector. Only the
 // actual program body and its reachable owned graph activate this adapter.
 scan(body);if(!markers.length)return null;
 if(!rootLink||!acceptProgramSchemaRoot(rootLink.metadata().term??'',rootLink.metadata().definition??'',String(rootLink.references().length)))fail('the selected typed-program root is invalid');
 if(markers.length!==1||!markers[0].parent.equals(body))fail('the source facet is duplicated or outside the actual program root');
 const rootFields=fields.get(key(root))??[];
 if(rootFields.length>1||rootFields.some(row=>fieldName(row.label)!=='provenance'))fail('the selected archive envelope contains an unknown or duplicate field');
 const marker=markers[0],proofLink=network.link(marker.child);
 if(!proofLink||proofLink.metadata().language!==schema.language||readProgramContainerKind(proofLink.metadata().term??'')!=='record'||proofLink.references().length)fail('the selected facet is not an actual proof record');
 const models=(fields.get(key(marker.child))??[]).filter(row=>fieldName(row.label)==='model');
 if(models.length!==1)fail('the selected proof must have one actual model field');
 // A private graph view retains every original LinkId. Only the selected
 // relation is hidden from ordinary core validation. The old codec reads the
 // SAME proof LinkId via its established inspectable envelope and reconstructs
 // aliases with its bounded core anchors and memoized proof traversal.
 const view=network.clone();view.deleteLink(marker.link.id());
 const record=LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language).withNamed(true).withTerm(schema.containers.record);
 const wrapper=view.insertLink([],record),entry=view.insertLink([body],rootLink.metadata().clone());
 view.insertFieldRelation(wrapper,'experimental source model',models[0].child);
 view.insertFieldRelation(wrapper,'source interpretation',marker.child);
 view.insertFieldRelation(entry,'provenance',wrapper);
 const value=decodeGeneratedOwnedRust(view,entry);
 return{...value,...checkPortableRustTextCertificate(value.program,value.proof)};
}
