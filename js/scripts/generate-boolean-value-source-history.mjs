// Preserve every complete predecessor before giving changed owners new identities.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

/** Capture full original modules and exact current source facts, without equality claims. */
export function generateBooleanValueSourceHistory({check=false}={}) {
  const revision='76e27c44c5854a2cb547aea5cef06ea80adf729c',root=new URL('../../',import.meta.url);
  const files=['js/src/formal-concept-definition.js','js/src/shared-numeric-function-model.js',
    'js/src/shared-numeric-function-rules.js','js/scripts/generate-shared-numeric-function-rules.mjs',
    'rust/src/formal_concept_definition.rs','rust/src/shared_numeric_function_model.rs',
    'rust/src/shared_numeric_function_rules.rs','rust/tests/unit/shared_numeric_function_links.rs',
    'js/src/project-data-files.js','js/scripts/generate-project-data-tooling-fixture.mjs',
    'experiments/grammar-context-source-metadata/generate.mjs','js/scripts/grammar-context-metadata-history.mjs',
    'js/tests/support/raw-contract-partition-history.js','js/scripts/current-ci-repair-source-history.mjs'];
  const sources=files.map(file=>{
    const original=captureGitArtifactCertificate(revision,file);
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
    const current=readFileSync(new URL(file,root));
    return{file,commit:revision,bytes:original.blob.length,sha256:createHash('sha256').update(original.blob).digest('hex'),
      sourceText,certificate:original.certificate,current:{bytes:current.length,sha256:createHash('sha256').update(current).digest('hex')}};
  });
  const value={schemaVersion:1,revision,sources,
    boundary:'Complete certified original source definitions remain independently recoverable. Current Boolean callable operations, exact-data read reuse and workflow adapters receive separate source-qualified identities. This archive supplies no blanket equality, native execution, corrected CI timing or requirement completion evidence.'};
  const file=new URL('parity/naming/boolean-value-source-history.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateBooleanValueSourceHistory({check:process.argv.includes('--check')});
}
