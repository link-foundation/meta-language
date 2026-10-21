// Normal schema author: capture whole committed predecessors, append shared entries, retain exact inverse and owners.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY,COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES,
 deriveCollectionSizeProjectionSchema,deriveCollectionSizeProjectionFixtureSource,deriveCollectionSizeProjectionValidationSource,readCollectionSizeProjectionSchemaPacket,verifyCollectionSizeProjectionSchemaPacket} from './collection-size-projection-schema.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/collection-size-projection-schema.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
export function generateCollectionSizeProjectionSchema({check=false}={}) {
 const policy=COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY;
 // Normal authoring captures each complete original once. Check-only freshness
 // consumes the pinned packet and independently verifies its stored Git objects;
 // it never reads Git, runs native tooling or accesses the network.
 const stored=check?readCollectionSizeProjectionSchemaPacket():null;
 const original=file=>{
  if(!stored)return captureGitArtifactCertificate(policy.revision,file);
  if(policy.files.includes(file)){
   const row=stored.predecessor.certificates.find(row=>row.file===file);assert.ok(row);
   return{blob:Buffer.from(stored.predecessor.source),certificate:row.certificate};
  }
  const row=stored.owners.find(row=>row.file===file);assert.ok(row);
  return{blob:Buffer.from(row.source),certificate:row.certificate};
 };
 const originals=policy.files.map(file=>{const row=original(file);assert.deepEqual(pin(row.blob),policy.predecessor);return{file,...row};});
 assert.equal(originals[0].blob.equals(originals[1].blob),true);
 const predecessor=originals[0].blob.toString('utf8'),source=deriveCollectionSizeProjectionSchema(predecessor);
 const owners=['js/src/translation/check.js','js/src/typed-program-expression-projection.js','rust/src/typed_program_links/projection.rs','js/scripts/generate-typed-program-links-fixtures.mjs','js/src/typed-program-links-validation.js'].map(file=>{
  const {blob,certificate}=original(file);
  const current=file==='js/scripts/generate-typed-program-links-fixtures.mjs'?deriveCollectionSizeProjectionFixtureSource(blob.toString('utf8'))
   :file==='js/src/typed-program-links-validation.js'?deriveCollectionSizeProjectionValidationSource(blob.toString('utf8')):blob.toString('utf8');
  assert.equal(readFileSync(new URL(file,root),'utf8'),current,'actual complete existing owner or exact fixture append is required: '+file);
  return{file,...pin(blob),source:blob.toString('utf8'),certificate,current:pin(current)};
 });
 const packet={schemaVersion:1,revision:policy.revision,predecessor:{...policy.predecessor,source:predecessor,
  certificates:originals.map(({file,certificate})=>({file,certificate}))},current:pin(source),extensions:COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES,owners,
  boundary:'Exactly the existing checked map/set size operation projects to the existing Surface field size through shared data. Earlier schema bytes and owner modules retain whole Git membership and exact inversion. No wider collection support, generic fallback, native execution or phase completion is claimed.'};
 verifyCollectionSizeProjectionSchemaPacket(packet);
 for(const file of policy.files){const actual=readFileSync(new URL(file,root),'utf8');assert.ok(actual===predecessor||actual===source,'unknown actual schema source refuses: '+file);if(check)assert.equal(actual,source);}
 const helperPath=new URL(helper,root),before=readFileSync(helperPath,'utf8'),pattern=/^const recipeSha256='(?:PENDING|[a-f0-9]{64})';$/mu;assert.equal(before.split(pattern).length,2);
 const output=new URL(policy.recipe,root);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
 const after=before.replace(pattern,"const recipeSha256='"+pin(readFileSync(output)).sha256+"';");
 if(check)assert.equal(before,after);else{for(const file of policy.files)writeFileSync(new URL(file,root),source);writeFileSync(helperPath,after);}
 return structuredClone(packet);
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
 generateCollectionSizeProjectionSchema({check:args.includes('--check')});console.log('Exact shared collection size projection appended; classifiers and native compilation remain separate normal generation and CI steps.');
}
