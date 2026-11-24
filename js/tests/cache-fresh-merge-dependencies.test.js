import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {restoreAutomatedTranslationPauseSource} from '../scripts/automated-translation-pause-source-history.mjs';
import {captureGitArtifactCertificate} from '../scripts/capture-git-artifact-certificate.mjs';
import {restoreIntegrationSource,restoreVersionedFixtureCompilerWorkflow} from '../scripts/current-ci-repair-source-history.mjs';
const file=new URL('../../.github/workflows/rust.yml',import.meta.url);
const workflowPath='.github/workflows/rust.yml';
const coverageProfile="          CARGO_PROFILE_TEST_OPT_LEVEL: 3\n"
  +"          CARGO_PROFILE_TEST_DEBUG_ASSERTIONS: 'true'\n"
  +"          CARGO_PROFILE_TEST_OVERFLOW_CHECKS: 'true'\n";

test('fresh merge restores the existing successful check cache before every unchanged native check under its original deadline',()=>{
  const source=readFileSync(file,'utf8'),jobs=parseWorkflow(source);
  const fresh=jobs.find(row=>row.id==='fresh-merge'),check=jobs.find(row=>row.id==='check');assert.ok(fresh&&check);
  const restore=fresh.steps.find(row=>row.name==='Restore successful native check dependencies for the simulated merge');assert.ok(restore);
  assert.equal(fresh.steps.filter(row=>row.name===restore.name).length,1);
  assert.ok(fresh.steps.indexOf(restore)<fresh.steps.findIndex(row=>row.name==='Simulate fresh merge with base branch'));
  assert.match(restore.text,/actions\/cache\/restore@v6/u);assert.doesNotMatch(restore.text,/fail-on-cache-miss:\s*true/u);
  for(const value of ['~/.cargo/registry','~/.cargo/git','rust/target'])assert.ok(restore.text.includes(value));
  const producer=check.steps.find(row=>row.name==='Cache cargo registry');assert.ok(producer);
  const key=/key: (.+)/u.exec(restore.text)[1];assert.equal(key,/key: (.+)/u.exec(producer.text)[1]);
  assert.match(key,/-cargo-check-1\.99\.0-/u);
  const archived=restoreAutomatedTranslationPauseSource(source,workflowPath);
  const block="      - name: Restore successful native check dependencies for the simulated merge\n"
    +"        uses: actions/cache/restore@v6\n        with:\n          path: |\n"
    +"            ~/.cargo/registry\n            ~/.cargo/git\n            rust/target\n"
    +"          key: ${{ runner.os }}-cargo-check-${{ hashFiles('**/Cargo.lock') }}\n\n";
  assert.equal(archived.split(block).length,2);
  const original=captureGitArtifactCertificate('90636f9fb52b1b9095defbac041c8089608b8576','.github/workflows/rust.yml');
  assert.equal(source.split(coverageProfile).length,2);
  const scoped=jobs.find(row=>row.id==='native-grammar-outcome-experiment');assert.ok(scoped);
  assert.match(scoped.steps.find(step=>step.name==='Setup Rust').text,/uses: dtolnay\/rust-toolchain@1\.99\.0/u);
  const archivedScoped=parseWorkflow(archived).find(row=>row.id==='native-grammar-outcome-experiment');
  const setup=archivedScoped.steps.find(step=>step.name==='Setup Rust');assert.ok(setup);
  const scopedCompiler="        # The experiment's receipt accepts Cargo and rustc >=1.90.0 <1.100.0.\n"
    +"        uses: dtolnay/rust-toolchain@1.99.0";
  assert.ok(setup.text.includes(scopedCompiler));
  assert.equal(archived.split(setup.text).length,2);
  const beforeScopedCompiler=restoreIntegrationSource(source,workflowPath);
  assert.equal(beforeScopedCompiler,restoreVersionedFixtureCompilerWorkflow(archived).replace(setup.text,
    setup.text.replace(scopedCompiler,'        uses: dtolnay/rust-toolchain@stable')));
  const predecessor=removePartitionUploadRetry(beforeScopedCompiler.replace(block,'').replace(coverageProfile,''));
  assert.equal(predecessor,original.blob.toString('utf8'),'every original job, deadline, matrix cell, gate, command and artifact step remains exact');
  assert.match(source,/fresh-merge:[\s\S]*?timeout-minutes: 20/u);
});

test('physical native jobs compile and test using the declared fixture compiler',()=>{
  const source=readFileSync(file,'utf8'),jobs=parseWorkflow(source);
  const profiles=['fresh-merge','test','coverage'];
  const provision='Install the version-qualified fixture compiler';
  const archived=parseWorkflow(restoreAutomatedTranslationPauseSource(source,workflowPath));
  assert.deepEqual(archived.filter(job=>job.steps.some(step=>step.name===provision)).map(job=>job.id),profiles);
  for(const id of profiles){
    const current=jobs.find(row=>row.id===id);
    assert.match(current.header,/^ {6}META_LANGUAGE_VERSIONED_RUST_TOOLCHAIN: 1\.99\.0$/mu);
    assert.match(current.steps.find(step=>step.name==='Setup Rust').text,/uses: dtolnay\/rust-toolchain@1\.99\.0/u);
    const job=archived.find(row=>row.id===id);
    assert.match(job.header,/^ {6}META_LANGUAGE_VERSIONED_RUST_TOOLCHAIN: 1\.99\.0$/mu);
    const setup=job.steps.find(step=>step.name==='Setup Rust');
    const install=job.steps.find(step=>step.name===provision);
    assert.match(setup.text,/uses: dtolnay\/rust-toolchain@stable/u);
    assert.equal(job.steps.indexOf(install),job.steps.indexOf(setup)+1);
    assert.equal(install.run.trimEnd(),'rustup toolchain install "$META_LANGUAGE_VERSIONED_RUST_TOOLCHAIN" --profile minimal --no-self-update');
  }
  const traced=jobs.flatMap(job=>job.steps).filter(step=>step.text.includes('META_LANGUAGE_TRACE_RECURSIVE_DECLARATION'));
  assert.equal(traced.length,1);
  assert.equal(traced[0].name,'Run tests');
  assert.match(traced[0].text,/META_LANGUAGE_TRACE_RECURSIVE_DECLARATION: readCallableStatementUses/u);
  const changed=source.replace('META_LANGUAGE_VERSIONED_RUST_TOOLCHAIN: 1.99.0','META_LANGUAGE_VERSIONED_RUST_TOOLCHAIN: 1.100.0');
  assert.equal(restoreIntegrationSource(changed,workflowPath),changed);
});

test('instrumented native tests use optimization with debug assertions overflow checks original coverage floor and deadline',()=>{
  const source=readFileSync(file,'utf8'),jobs=parseWorkflow(source);
  const coverage=jobs.find(row=>row.id==='coverage'),report=jobs.find(row=>row.id==='coverage-report');assert.ok(coverage&&report);
  const step=coverage.steps.find(row=>row.name==='Generate code coverage');assert.ok(step);
  assert.ok(step.text.includes(coverageProfile));
  assert.match(coverage.header,/timeout-minutes: 15/u);
  assert.equal(step.run.trimEnd(),'node ../scripts/with-cache-cleanup.mjs --event coverage --protect lcov.info -- cargo llvm-cov --all-features --lcov --output-path lcov.info --no-fail-fast -- $SUITE_FILTERS');
  assert.ok(report.steps.some(row=>row.run.includes('--fail-under-lines 84.30')));
  assert.equal(jobs.filter(row=>row.steps.some(step=>step.text.includes('CARGO_PROFILE_TEST_OPT_LEVEL: 3'))).length,1);
});

// Reconstruct the exact original upload without accepting any payload change.
function removePartitionUploadRetry(source) {
  const start=source.indexOf('      - name: Retry authenticated native partition upload\n');
  assert.ok(start>=0);const end=source.indexOf('      - name: Remove the temporary external grammar inputs',start);assert.ok(end>start);
  const retry=source.slice(start,end),expected=retry.replace('Retry authenticated native partition upload','Retain authenticated native partition')
    .replace("if: ${{ !cancelled() && steps.native-partition-upload.outcome == 'failure' }}","if: ${{ !cancelled() }}")
    .replace('          overwrite: true\n','');
  const first=expected.replace('        if:','        id: native-partition-upload\n        continue-on-error: true\n        if:');
  assert.equal(source.split(first+retry).length,2);
  return source.replace(first+retry,expected);
}

test('native partition artifact recovery retries the identical complete payload and fails if the final upload fails',()=>{
  const source=readFileSync(file,'utf8'),job=parseWorkflow(source).find(row=>row.id==='test');assert.ok(job);
  const first=job.steps.find(row=>row.name==='Retain authenticated native partition');
  const retry=job.steps.find(row=>row.name==='Retry authenticated native partition upload');assert.ok(first&&retry);
  assert.match(first.text,/id: native-partition-upload\n\s+continue-on-error: true/u);
  assert.ok(retry.text.includes("if: ${{ !cancelled() && steps.native-partition-upload.outcome == 'failure' && (inputs.automated_translation || vars.META_LANGUAGE_AUTOMATED_TRANSLATION == 'true') }}"));
  assert.doesNotMatch(retry.text,/continue-on-error/u);assert.match(retry.text,/overwrite: true/u);
  assert.equal(job.steps.indexOf(retry),job.steps.indexOf(first)+1);
  const artifact=step=>step.text.split('        with:\n')[1].replace('          overwrite: true\n','');
  assert.equal(artifact(first),artifact(retry));
  for(const step of [first,retry])assert.match(step.text,/if-no-files-found: error/u);
  const archived=restoreAutomatedTranslationPauseSource(source,workflowPath);
  assert.notEqual(removePartitionUploadRetry(archived),archived);
});
