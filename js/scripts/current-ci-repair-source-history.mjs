// Exact source views for the current CI repair; current execution stays separate.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCurrentNativeHostSource} from './current-native-host-source-history.mjs';
const root=new URL('../../',import.meta.url);
const file='parity/self-translation/ci-39422441941-repair-source-history.lino';
const archiveSha256='5b50046591ff194808c3d151e3a41df7de1c677230995f3e7356e498a7f161c6';
const revision='90636f9fb52b1b9095defbac041c8089608b8576';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let history;

let integrationSources;

/** Restore the complete workflow before the Rust admission repairs. */
export function restoreRustJobAdmissionSource(source) {
  const prior = '3b31f67ab3224accb94bbe2175594bdded76689f9c4bf2a2111f42bae13ee462';
  if (pin(source).sha256 === 'c3e84bbf746c8a1477a910750ff9165eb0443ed4ce7ed7660c75b6dc5dc69bc3') {
    const rustRan = "needs.rust.result != 'skipped' || needs.rust-suite.result != 'skipped'";
    const guards = [
      ['acceptance-checkout', `!cancelled() && (needs.candidates.result == 'success' || ${rustRan})`],
      ['acceptance-node', "!cancelled() && steps.acceptance-checkout.outcome == 'success'"],
      ['acceptance-dependencies', "!cancelled() && steps.acceptance-node.outcome == 'success'"],
    ];
    for (const [step, condition] of guards) {
      source = source.replace(`        id: ${step}\n        if: \${{ ${condition} }}\n`, '');
    }
    source = source.replace(
      `        if: \${{ !cancelled() && steps.acceptance-dependencies.outcome == 'success' && (${rustRan}) }}\n`,
      `        if: \${{ !cancelled() && (${rustRan}) }}\n`,
    );
    assert.equal(pin(source).sha256, prior);
  }
  if (pin(source).sha256 !== prior) return source;
  const restored = source.replace(
    /^ {10}if ! (gh api "[^\n]+\/jobs\?per_page=100" --paginate --slurp > "[^"\n]+"); then\n {12}gh api [^\n]+\n {10}fi$/mu,
    '          $1',
  );
  assert.equal(pin(restored).sha256, '3bc9784ef73d8e69cd693ce23884ff8d4454c7d8b9be8c98e4ebccccae331b3a');
  return restored;
}

/** Return the certified predecessor only for a complete integration successor. */
export function restoreIntegrationSource(source,path) {
  if (path === '.github/workflows/ci.yml') source = restoreRustJobAdmissionSource(source);
  if(!integrationSources){
    const bytes=readFileSync(new URL('parity/self-translation/integration-source-history.lino',root));
    assert.equal(pin(bytes).sha256,'a67a0934fcce2f2c6be67284f45370b83a56899198dd01092d5ec336e203d1d9');
    const archive=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(archive.schemaVersion,1);
    integrationSources=new Map(archive.sources.map(row=>[row.path,row]));
  }
  const row=integrationSources.get(path);
  if(!row||!row.current.includes(pin(source).sha256))return source;
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:row.revision,path});
  return row.source;
}

/** Unknown bytes remain visible to the original historical validator. Only a
 * complete independently pinned current source gets an original source view. */
export function restoreCurrentCiRepairSource(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  source=restoreIntegrationSource(source,path);
  source=restoreCurrentNativeHostSource(source,path);
  source=restoreBooleanValueGuardHistory(source,path);
  source=restoreRawPlanningWorkflowHistory(source,path);
  source=restoreGrammarContextMetadataHistory(source,path);
  if(!history){const bytes=readFileSync(new URL(file,root));assert.equal(pin(bytes).sha256,archiveSha256);history=readBootstrapStructuredDataLino(bytes.toString('utf8'));assert.equal(history.revision,revision);}
  const row=history.sources.find(row=>row.file===path);if(!row)return source;
  const actual=pin(source);
  if(actual.sha256!==row.current.sha256)return source;
  assert.deepEqual(actual,row.current);
  const bytes=Buffer.from(source),edit=row.edit;
  assert.equal(edit.offsetUnit,'UTF-8 bytes');
  assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.end<=bytes.length);
  assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.inserted);
  const original=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});assert.equal(original.toString('utf8'),row.sourceText);
  verifyGitArtifactCertificate(row.certificate,original,{revision,path});
  return row.sourceText;
}

import {restoreGrammarContextMetadataHistory} from './grammar-context-metadata-history.mjs';
import {restoreRawPlanningWorkflowHistory} from './raw-planning-workflow-history.mjs';
import {restoreBooleanValueGuardHistory} from './boolean-value-source-history.mjs';
