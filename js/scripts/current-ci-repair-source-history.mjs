// Exact source views for the current CI repair; current execution stays separate.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCurrentNativeHostSource} from './current-native-host-source-history.mjs';
import {restoreNativeCompilerSource} from './native-compiler-source-history.mjs';
const root=new URL('../../',import.meta.url);
const file='parity/self-translation/ci-39422441941-repair-source-history.lino';
const archiveSha256='07ae94b5c36a92e8d07dc312e9079ec83622bf606822ec6ebe4fafe8f3ca2e3d';
const revision='90636f9fb52b1b9095defbac041c8089608b8576';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let history;
let integrationSources;

/** Preserve the exact workflow before selecting the supported observation toolchain. */
export function restoreRustObservationToolchain(source, path) {
  if (path !== '.github/workflows/rust.yml' ||
      pin(source).sha256 !== 'f75fb07e2cb4da8896bb6152649499f124c8352905d42de700c1fa5f43335df6') return source;
  const selected = '      # The native assertions cover Rust >=1.90.0 <1.100.0.\n      - name: Setup Rust\n        uses: dtolnay/rust-toolchain@1.99.0';
  const previous = '      - name: Setup Rust\n        uses: dtolnay/rust-toolchain@stable';
  assert.equal(source.split(selected).length, 5);
  const restored = source.replaceAll(selected, previous);
  assert.equal(pin(restored).sha256, 'e006605c0c28530a461717e64cf031f42087b9199a8604231bb035a8dce58eba');
  return restored;
}

/** Select certified predecessors for exact integrated source files. */
export function restoreIntegrationSource(source, path) {
  if (path !== 'rust/Cargo.toml' && path !== 'js/src/translation/types.js') return source;
  if (!integrationSources) {
    const bytes = readFileSync(new URL('parity/self-translation/integration-source-history.lino', root));
    assert.equal(pin(bytes).sha256, '01f7a8e0aef7556c5f54d65377ea0e0a508917b74c2a88bbd829347ed6c53fba');
    const archive = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(archive.schemaVersion, 1);
    for (const row of archive.sources) {
      verifyGitArtifactCertificate(row.certificate, Buffer.from(row.source), { revision: row.revision, path: row.path });
    }
    integrationSources = new Map(archive.sources.map(row => [row.path, row]));
  }
  const row = integrationSources.get(path);
  assert.ok(row);
  return pin(source).sha256 === row.current ? row.source : source;
}

/** Unknown bytes remain visible to the original historical validator. Only a
 * complete independently pinned current source gets an original source view. */
export function restoreCurrentCiRepairSource(source,path) {
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  source=restoreRustObservationToolchain(source,path);
  if(path==='js/scripts/record-rule-library.mjs')source=restoreNativeCompilerSource(source,path);
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
