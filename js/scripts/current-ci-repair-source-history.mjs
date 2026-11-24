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
  source = restoreDiagnosticOrderFixture(source, path);
  source = restoreCargoTestHarnessRecognition(source, path);
  if (path === '.github/workflows/rust.yml'
    && pin(source).sha256 === '9ea1cf322e18b37306b02d3f5d6752a7de26fccbd3e591fbc76f7b1ee9b9296f') {
    source = source.replace(
      '        # The experiment\'s receipt accepts Cargo and rustc >=1.90.0 <1.100.0.\n        uses: dtolnay/rust-toolchain@1.99.0\n',
      '        uses: dtolnay/rust-toolchain@stable\n',
    );
    assert.equal(pin(source).sha256, 'e006605c0c28530a461717e64cf031f42087b9199a8604231bb035a8dce58eba');
  }
  if (path === 'experiments/grammar-outcome-network/native-transport/runner/compiler-profile.test.mjs'
    && pin(source).sha256 === '5d1e10f93075a0a69c8aadeef7b8ded0d5f32040cba9cea0464e28540c44b33c') {
    const appendedTest = "\ntest('the native outcome workflow selects one explicit toolchain";
    const start = source.indexOf(appendedTest);
    assert.ok(start > 0);
    source = source.slice(0, start);
    assert.equal(pin(source).sha256, 'dbb8772b661085514493fd4dd22cbbbd6da13eaf47a52f46dacbb49bfa4b56a2');
  }
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

/** Preserve the historical fixture before its Cargo manifest receipt was refreshed. */
function restoreDiagnosticOrderFixture(source, path) {
  if (path !== 'parity/fixtures/grammar-selection-diagnostic-order.lino'
    || pin(source).sha256 !== 'bd6aeec0e7a923ea4b96f33f8c0baac99bf708006043edd88f2bcb8534dd3543') return source;

  for (const [currentDefinition, previousDefinition] of [
    ['8769', '8719'],
    ['acec16f4f563c7bd92a97e50b5efd59f0a3f3740b49a7c5d308edac62282a5dd', '216c4bbdfc6b033bb2020c1d6b6b3bdff9148d87ccc3df27bcbd386a9071a270'],
    ['4770', '4720'],
    ['6100', '6050'],
  ]) {
    const definition = `(def: ${currentDefinition})`;
    assert.equal(source.split(definition).length, 2);
    source = source.replace(definition, `(def: ${previousDefinition})`);
  }
  assert.equal(pin(source).sha256, '7460e44f2f3a8794c4b8c50f1e7ff00b87c1149659c1ceadf4203e2f1ddb76e8');
  return source;
}

/** Recover the complete original runner or regression source for historical controls. */
function restoreCargoTestHarnessRecognition(source, path) {
  if (path === 'js/scripts/rust-test-shards.mjs'
    && pin(source).sha256 === 'caae12ca5f69f0e69a780e365b984129fece80c80c24de7798f0fc6c26b8c2ce') {
    const start = source.indexOf("  if (relative.startsWith('..') || paths.isAbsolute(relative)) return false;");
    const end = source.indexOf('\n}', start);
    assert.ok(start > 0 && end > start);
    const previousRecognition = String.raw`  return !relative.startsWith('..') && !paths.isAbsolute(relative)
    && relative.split(paths.sep).includes('deps')
    && /-[0-9a-f]+(?:\.exe)?$/u.test(paths.basename(binary));`;
    source = source.slice(0, start) + previousRecognition + source.slice(end);
    assert.equal(pin(source).sha256, '711ef7aa292d9512dcdee2b33fc84e4c6f030181284f3c6114c4e4726a895d43');
  }
  if (path === 'js/tests/issue-195-evidence-rust-shards.test.js'
    && pin(source).sha256 === '55cecf0ccaab886b024c6bdc6a9ce5fdd8deb3dee7b157c6a21d0b6e690d72bf') {
    const start = source.indexOf("\ntest('Cargo 1.100 test unit paths");
    assert.ok(start > 0);
    source = source.slice(0, start);
    assert.equal(pin(source).sha256, '55ec90a1af432aecfed11e8ff8fc1298cacf8fba4edb6f6b5ce846800f68306f');
  }
  if (path === 'experiments/grammar-outcome-network/native-transport/inverse/generate-publication-runner.mjs'
    && pin(source).sha256 === '8d606fafd21a0e418da10f443a9e8a709e54cac6a4226908fabd8044865d9847') {
    const start = source.indexOf('  const previousRecognition = ');
    const end = source.indexOf('\n  return receipt;', start);
    assert.ok(start > 0 && end > start);
    const previousPublication = `  if(check){assert.equal(readFileSync(new URL(file,root),'utf8'),current);checkProjectData(output,receipt);}
  else{writeFileSync(new URL(file,root),current);writeProjectData(output,receipt);}`;
    source = source.slice(0, start) + previousPublication + source.slice(end);
    assert.equal(pin(source).sha256, 'b1d49b0139edea9be0830de02e3312b5918ef9e9b4b9efcf00882e734bd0b0c3');
  }
  if (path === 'js/tests/translation-native-number-retention.test.js'
    && pin(source).sha256 === 'db246e7ef2b186ebfa23f9781743e2fdb1dc938c4dc6cebdf62b4387b5867cbd') {
    source = source.replace(
      "  const source=readFileSync(new URL(original.file,root),'utf8');\n  let current=restoreIntegrationSource(source,original.file);",
      "  let current=readFileSync(new URL(original.file,root),'utf8');",
    ).replace(
      '  assert.ok(source.includes(\'verifyHarnessExecution(receipt);',
      "  const source=readFileSync(new URL(original.file,root),'utf8');\n  assert.ok(source.includes('verifyHarnessExecution(receipt);",
    ).replace("\nimport {restoreIntegrationSource} from '../scripts/current-ci-repair-source-history.mjs';\n", '');
    assert.equal(pin(source).sha256, '5d8fd3256c8b58f229a6f78c13084138f1ad9ad484455851d477fd3cd4f0c5e9');
  }
  return source;
}
