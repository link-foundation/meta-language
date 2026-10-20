// Authenticate all predecessor source definitions behind the complete CI repair batch.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
const root=new URL('../../../../',import.meta.url),revision='2b4e77859ca607b57a037d532f704fc92aad363f';
const sources=[
  "experiments/concrete-program-graphs/origin.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-number-publication.mjs",
  "js/scripts/generate-concrete-program-native-inputs.mjs",
  "js/tests/translation-concrete-program-origin.test.js",
  "rust/tests/unit/native_json_number_publication.rs",
  "rust/tests/unit/translation_concrete_program_graphs.rs",
  "rust/tests/unit/translation_primitive_methods.rs",
  "rust/tests/unit/translation_typescript_typed_graph.rs",
  "js/scripts/continuation-source-history.mjs",
  "js/scripts/generate-continuation-historical-inputs.mjs",
  "js/tests/translation-continuation-historical-inputs.test.js"
];
const fixtures=[
  "parity/fixtures/concrete-program-graph-native/manifest.lino",
  "parity/self-translation/concrete-program-native-consumer-template.lino",
  "parity/self-translation/native-number-publication.lino"
];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
/** Capture exact old modules and current digests; this proves preservation only. */
export function generateNativeCompatibilitySourceHistory({check=false}={}) {
  const packets=[];
  for(const [kind,paths]of [['sources',sources],['fixtures',fixtures]]){
    const rows=paths.map(file=>{
      const original=captureGitArtifactCertificate(revision,file);
      verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
      const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
      const current=readFileSync(new URL(file,root));assert.deepEqual(Buffer.from(current.toString('utf8')),current);
      return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)};
    });
    const packet={schemaVersion:1,revision,sources:rows,
      boundary:'Complete authenticated original modules and fixtures remain available after CI repairs. Different current operations have separate identities. Source conservation establishes neither native execution nor general semantic equivalence; unchanged fixture inputs and independent controls remain mandatory.'};
    const output=new URL('parity/self-translation/native-compatibility-source-'+kind+'-history.lino',root);
    if(check)checkProjectData(output,packet);else writeProjectData(output,packet);packets.push(packet);
  }
  const helper=new URL('js/scripts/native-compatibility-source-history.mjs',root),source=readFileSync(helper,'utf8');
  const path=new URL('parity/self-translation/native-compatibility-source-sources-history.lino',root);
  const expected=source.replace(/const archiveHash='(?:PENDING|[a-f0-9]{64})';/u,"const archiveHash='"+pin(readFileSync(path)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helper,expected);
  return packets;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packets=generateNativeCompatibilitySourceHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packets.reduce((count,packet)=>count+packet.sources.length,0)+' complete predecessor files and current source digests.');
}
