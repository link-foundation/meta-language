// Retain complete original inputs to strict historical gates, without repins.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';
const archive='parity/self-translation/ci-39422441941-repair-source-history.lino';
const helper='js/scripts/current-ci-repair-source-history.mjs';
const sourceFiles=[
  'js/src/grammar-runtime/grammar-selection.js',
  'rust/src/grammar/feature_runtime/grammar_selection.rs',
  'js/tests/global-concept-archive-parser.test.js',
  'rust/tests/unit/global_concept_archive_parser.rs',
  'experiments/global-grammar-context-reuse/generate.mjs',
  'experiments/global-grammar-context-reuse/proposal.mjs',
  'js/tests/grammar-global-context-reuse.test.js',
  'js/tests/grammar-global-context-source-profiles.test.js',
  'experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs',
  'experiments/grammar-outcome-network/native-transport/diagnostic-order/publication-edits.mjs',
  'experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.test.mjs',
  'experiments/grammar-outcome-network/native-transport/wire/generate.mjs',
  'experiments/grammar-outcome-network/native-transport/wire/original-authoring.lino',
  'parity/fixtures/grammar-selection-diagnostic-order.lino',
  '.github/workflows/rust.yml',
  'js/scripts/check-bulk-translation-rules.mjs',
  'js/src/typed-program-checked-projection.js',
  'js/src/typed-program-links-validation.js',
  'rust/src/translation/check.rs','rust/src/translation/emit_rust.rs','rust/src/translation/ir.rs',
  'rust/src/translation/mod.rs','rust/src/typed_program_links/checked.rs','rust/tests/unit/mod.rs',
  'rust/tests/unit/translation_array_relative_element.rs',
  'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
  'js/scripts/translation-source-history.mjs','js/scripts/continuation-source-history.mjs',
  'js/scripts/complete-declaration-receipts.mjs','js/scripts/current-translation-source-descriptors.mjs',
  'experiments/grammar-outcome-network/native-transport/inverse/generate-library-relocation-history.mjs',
  'js/tests/translation-native-library-receipt-history.test.js',
  'experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino',
  'parity/self-translation/complete-declaration-receipts.lino',
  'experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino',
];
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** The diff is computed on exact bytes, retaining UTF-8 boundary integrity. */
export function generateCurrentCiRepairSourceHistory({check=false}={}) {
  const sources=sourceFiles.map(file=>{
    const{blob,certificate}=captureGitArtifactCertificate(revision,file),current=readFileSync(new URL(file,root));
    assert.deepEqual(Buffer.from(blob.toString('utf8')),blob);assert.deepEqual(Buffer.from(current.toString('utf8')),current);
    let start=0,oldEnd=blob.length,end=current.length;
    while(start<oldEnd&&start<end&&blob[start]===current[start])start++;
    while(start>0&&(current[start]&192)===128)start--;
    while(oldEnd>start&&end>start&&blob[oldEnd-1]===current[end-1]){oldEnd--;end--;}
    while(end<current.length&&((current[end]&192)===128||(blob[oldEnd]&192)===128)){end++;oldEnd++;}
    const removed=blob.subarray(start,oldEnd),inserted=current.subarray(start,end);
    assert.deepEqual(Buffer.from(removed.toString('utf8')),removed);assert.deepEqual(Buffer.from(inserted.toString('utf8')),inserted);
    const restored=Buffer.concat([current.subarray(0,start),removed,current.subarray(end)]);assert.deepEqual(restored,blob);
    return{file,commit:revision,...pin(blob),sourceText:blob.toString('utf8'),certificate,current:pin(current),
      edit:{offsetUnit:'UTF-8 bytes',start,end,removed:removed.toString('utf8'),inserted:inserted.toString('utf8')}};
  });
  const packet={schemaVersion:1,revision,run:39422441941,sources,
    boundary:'Complete immutable source and data conservation for historical controls only. Current execution, actual source descriptors, every original assertion and full native CI remain separate mandatory checks. Unknown source bytes cannot acquire a historical identity.'};
  const output=new URL(archive,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const path=new URL(helper,root),source=readFileSync(path,'utf8'),pattern=/const archiveSha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(source.split(pattern).length,2);const expected=source.replace(pattern,"const archiveSha256='"+pin(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packet=generateCurrentCiRepairSourceHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packet.sources.length+' complete CI-repair predecessor sources and exact byte inverses.');
}
