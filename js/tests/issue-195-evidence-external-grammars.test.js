import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const jobs = (file) => new Map(parseWorkflow(read(`.github/workflows/${file}`)).map((job) => [job.id, job]));
const downloader = 'external-grammar-cache.mjs';

function teardown(job) {
  const remove = job.steps.findIndex((step) => step.run.includes(`${downloader} --clean-materialized`));
  const caches = job.steps.findIndex((step) => /scripts\/clean-caches\.mjs/u.test(step.run));
  assert.ok(remove >= 0 && caches > remove, `${job.id} removes verified temporary inputs before ordinary cache cleanup`);
  assert.ok(job.steps[remove].afterFailure, `${job.id} removes temporary inputs after failed checks`);
}

function provisionBefore(job, command, selection) {
  assert.ok(job, 'the required job exists');
  const provision = job.steps.findIndex((step) => step.run.includes(downloader));
  const consumer = job.steps.findIndex((step) => command.test(step.run));
  assert.ok(provision >= 0 && consumer > provision, `${job.id} provisions authenticated inputs before use`);
  const step = job.steps[provision];
  assert.match(step.run, /--materialize\b/u, `${job.id} stages the verified temporary files`);
  assert.match(step.run, selection, `${job.id} includes every required input class`);
  assert.doesNotMatch(step.text, /continue-on-error:\s*true/u, `${job.id} refuses unauthenticated or missing inputs`);
  assert.match(step.text, /working-directory: \$\{\{ github\.workspace \}\}/u, `${job.id} resolves repository-relative inputs from the checkout`);
  const dependencies = job.steps.findIndex((item) => /\bnpm ci\b/u.test(item.run));
  assert.ok(dependencies >= 0 && dependencies < provision, `${job.id} installs the linked manifest reader first`);
  teardown(job);
}

test('every Rust target check provisions its independent source oracles before compilation', () => {
  const rust = jobs('rust.yml');
  for (const [id, command] of [
    ['check', /cargo check --locked --all-targets --all-features/u],
    ['msrv', /cargo check --locked --all-features --all-targets/u],
    ['lint', /cargo clippy --all-targets --all-features/u],
  ]) provisionBefore(rust.get(id), command, /--group native-oracles\b/u);
  for (const [id, command] of [
    ['test', /cargo test --no-fail-fast/u],
    ['coverage', /cargo llvm-cov/u],
  ]) provisionBefore(rust.get(id), command, /--all\b/u);
  const merged = read('rust/scripts/simulate-fresh-merge.sh');
  const provision = merged.indexOf(`node scripts/${downloader} --all --materialize`);
  assert.ok(provision > merged.indexOf('git merge --no-edit'), 'the simulated merge selects its own pinned manifest');
  assert.ok(provision > merged.indexOf('npm ci --ignore-scripts'), 'the merged checkout installs its linked manifest reader');
  assert.ok(provision < merged.indexOf('cargo fmt --all'), 'the original full checks run after provisioning');
  assert.ok(merged.indexOf('--clean-materialized') < merged.lastIndexOf('git checkout --force -'), 'temporary inputs leave before restoring the original checkout');
});

test('grammar and runtime evidence stages explicitly provision pinned inputs before their producers', () => {
  const ci = jobs('ci.yml');
  for (const [id, selection, command] of [
    ['grammar-bulk-pipeline', /--all\b/u, /run-grammar-bulk-pipeline\.mjs/u],
    ['runtime-parity', /--group wasm\b/u, /run-issue-195-evidence\.mjs/u],
    ['javascript-suite', /--all\b/u, /run-issue-195-evidence\.mjs/u],
    ['rust-suite', /--all\b/u, /run-issue-195-evidence\.mjs/u],
  ]) {
    const job = ci.get(id);
    const provision = job.steps.findIndex((step) => step.run.includes(downloader));
    const producer = job.steps.findIndex((step) => command.test(step.run));
    assert.ok(provision >= 0 && producer > provision, `${id} provisions before producing evidence`);
    assert.match(job.steps[provision].run, selection);
    assert.match(job.steps[provision].run, /--materialize\b/u);
    assert.doesNotMatch(job.steps[provision].text, /continue-on-error:\s*true/u);
    teardown(job);
  }
});

test('temporary package grammar staging preserves script-free offline clean consumers', () => {
  const ci = jobs('ci.yml');
  const candidate = ci.get('candidates');
  const provision = candidate.steps.findIndex((step) => step.run.includes(downloader));
  const pack = candidate.steps.findIndex((step) => /--prepare-candidates\b/u.test(step.run));
  assert.ok(provision >= 0 && pack > provision, 'compiled package assets are staged before packaging');
  assert.match(candidate.steps[provision].run, /--group wasm --materialize/u);
  assert.match(candidate.steps[provision].text, /github\.event_name != 'release'/u, 'published release artifacts are consumed intact');
  teardown(candidate);
  assert.ok(ci.get('consumers').steps.every((step) => !step.run.includes(downloader)), 'installed consumers receive only their exact package artifacts');
  const consumer = read('js/scripts/run-issue-195-consumer.mjs');
  assert.match(consumer, /'install', '--ignore-scripts'/u);
  assert.match(consumer, /'--import', '\.\/deny-network\.mjs', 'consumer\.mjs'/u);
  assert.doesNotMatch(consumer, /external-grammar-cache\.mjs/u);
  const metadata = JSON.parse(read('js/package.json'));
  for (const hook of ['install', 'postinstall', 'prepack', 'prepare']) assert.equal(metadata.scripts[hook], undefined, hook);
});

test('JavaScript grammar checks and publication provision their assets explicitly', () => {
  const javascript = jobs('js.yml');
  for (const [id, command] of [
    ['tests', /build-agda-layout-oracle\.mjs/u],
    ['test', /test-groups\.mjs --check/u],
    ['publish', /npm publish --provenance/u],
  ]) {
    const job = javascript.get(id);
    const provision = job.steps.findIndex((step) => step.run.includes(downloader));
    const consumer = job.steps.findIndex((step) => command.test(step.run));
    assert.ok(provision >= 0 && consumer > provision, `${id} provisions before consuming grammar assets`);
    assert.match(job.steps[provision].run, id === 'publish' ? /--group wasm --materialize/u : /--all --materialize/u);
    assert.doesNotMatch(job.steps[provision].text, /continue-on-error:\s*true/u);
    const install = job.steps.findIndex((step) => /npm ci\b/u.test(step.run));
    assert.ok(install >= 0 && install < provision, `${id} installs the linked manifest reader before provisioning`);
  }
});
