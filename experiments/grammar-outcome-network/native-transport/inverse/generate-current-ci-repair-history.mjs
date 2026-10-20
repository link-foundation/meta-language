// Authenticate all predecessor source definitions behind the complete CI repair batch.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
const root=new URL('../../../../',import.meta.url),revision='d1232503fb38246f7e28f5d414c882a70944b38b';
const sources=[
  "experiments/grammar-outcome-network/native-transport/inverse/generate-integer-decimals.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-integer-projections.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-native-constructor-storage.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-number-publication.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/generate-static-borrow-module-history.mjs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/mod.rs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/part_0.rs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/part_1.rs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/part_2.rs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/part_3.rs",
  "experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/part_4.rs",
  "experiments/grammar-outcome-network/native-transport/runner/provision-assets.mjs",
  "experiments/grammar-outcome-network/native-transport/runner/run.mjs",
  "js/scripts/generate-root-type-option-history.mjs",
  "js/scripts/portable-conditional-source-style.mjs",
  "js/scripts/record-rule-library.mjs",
  "js/tests/translation-native-number-publication.test.js",
  "js/tests/translation-owned-original-numbers.test.js",
  "js/tests/translation-root-type-option-history.test.js",
  "js/tests/translation-static-borrow-module-history.test.js",
  "rust/src/translation/array_from_rules/part_1.rs",
  "rust/src/translation/collection_construction_rules/part_1.rs",
  "rust/src/translation/javascript/infer/methods.rs",
  "rust/src/translation/owned_outcome_integer_decimal_rules/part_1.rs",
  "rust/src/translation/owned_outcome_integer_projection_rules/part_1.rs",
  "rust/tests/unit/grammar_outcome_constructor_storage.rs",
  "rust/tests/unit/grammar_outcome_integer_decimal_rules.rs",
  "rust/tests/unit/grammar_outcome_integer_projections.rs",
  "rust/tests/unit/grammar_outcome_original_numbers.rs",
  "rust/tests/unit/grammar_outcome_static_borrows.rs",
  "rust/tests/unit/native_json_number_observation_visitors.rs",
  "rust/tests/unit/parity_manifest.rs",
  "rust/tests/unit/root_types.rs",
  "rust/tests/unit/translation_primitive_methods.rs"
];
const fixtures=['parity/fixtures/grammar-outcome-integer-decimal-rules.lino','parity/fixtures/grammar-outcome-integer-projections.lino','parity/fixtures/grammar-selection-diagnostic-order.lino'];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
/** Capture exact old modules and current digests; this proves preservation only. */
export function generateCurrentCiRepairHistory({check=false}={}) {
  const packets=[];
  for(const [kind,paths]of [['sources',sources],['fixtures',fixtures]]){
    const rows=paths.map(file=>{
      const original=captureGitArtifactCertificate(revision,file);
      verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
      const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
      const current=Buffer.from(restoreNativeCompatibilitySourceHistory(file,readFileSync(new URL(file,root),'utf8')));assert.deepEqual(Buffer.from(current.toString('utf8')),current);
      return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)};
    });
    const packet={schemaVersion:1,revision,run:39372455987,sources:rows,
      boundary:'Complete authenticated original modules and fixtures remain available after CI repairs. Different current operations have separate identities. Source conservation establishes neither native execution nor general semantic equivalence; unchanged fixture inputs and independent controls remain mandatory.'};
    const output=new URL('parity/self-translation/ci-39372455987-'+kind+'-history.lino',root);
    if(check)checkProjectData(output,packet);else writeProjectData(output,packet);packets.push(packet);
  }
  return packets;
}

import {restoreNativeCompatibilitySourceHistory} from '../../../../js/scripts/native-compatibility-source-history.mjs';
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const packets=generateCurrentCiRepairHistory({check:process.argv.includes('--check')});
  console.log('Authenticated '+packets.reduce((count,packet)=>count+packet.sources.length,0)+' complete predecessor files and current source digests.');
}
