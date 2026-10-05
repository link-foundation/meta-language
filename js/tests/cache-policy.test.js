// scripts/check-cache-policy.mjs passes on this checkout and detects each way
// the cache cleanup can be disabled, bypassed or left incomplete
// (requirement I195-CACHE-CLEANUP-POLICY).
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  CACHE_PRODUCING_COMMAND, REQUIRED_CATEGORIES, checkCachePolicy, checkCargoProfiles, checkWorkflow, parseWorkflow,
} from '../../scripts/check-cache-policy.mjs';
import { CACHE_CLASSES } from '../../scripts/lib/cache-classes.mjs';
import { REPOSITORY_ROOT, observeCacheCleanup } from './support/cache-fixtures.js';

const read = (relative) => readFileSync(path.join(REPOSITORY_ROOT, relative), 'utf8');
const WORKFLOWS = ['.github/workflows/rust.yml', '.github/workflows/js.yml', '.github/workflows/ci.yml'];

/** The policy problems of this checkout with `files` replaced. */
const problemsWith = (files, overrides = {}) => checkCachePolicy({ overrides: { files, ...overrides } });

function mutate(relative, from, to) {
  const text = read(relative);
  assert.ok(text.includes(from), `${relative} contains ${JSON.stringify(from)}`);
  return { [relative]: text.replaceAll(from, to) };
}

const matching = (problems, pattern) => problems.filter((problem) => pattern.test(problem));

test('the cache policy holds on this checkout', () => {
  assert.deepEqual(checkCachePolicy(), []);
  for (const file of WORKFLOWS) {
    const jobs = parseWorkflow(read(file));
    assert.ok(jobs.length > 0, `${file} has jobs`);
    for (const job of jobs) {
      for (const step of job.steps) {
        if (CACHE_PRODUCING_COMMAND.test(step.run)) assert.match(step.run, /scripts\/with-cache-cleanup\.mjs --event \S+/u);
      }
    }
  }
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['policyPassesOnRepository'], 'the cache policy holds on this checkout');
});

test('a disabled, restricted or non-executable hook is detected', () => {
  assert.deepEqual(problemsWith({ '.githooks/pre-commit': null }), ['.githooks/pre-commit is missing']);
  const commented = read('.githooks/pre-commit').replace(/^(.*scripts\/clean-caches\.mjs.*)$/mu, '# $1');
  assert.ok(matching(problemsWith({ '.githooks/pre-commit': commented }), /does not run scripts\/clean-caches\.mjs/u).length > 0);
  assert.deepEqual(problemsWith({}, { hookExecutable: false }), ['.githooks/pre-commit is not executable']);

  const config = '.pre-commit-config.yaml';
  assert.deepEqual(problemsWith(mutate(config, '- id: clean-caches', '- id: something-else')), ['.pre-commit-config.yaml has no clean-caches hook']);
  assert.ok(matching(problemsWith(mutate(config, 'always_run: true', 'always_run: false')), /always_run/u).length === 1);
  const restricted = mutate(config, 'entry: node scripts/clean-caches.mjs --event pre-commit --quiet',
    'entry: node scripts/clean-caches.mjs --event pre-commit --quiet\n        files: ^rust/');
  assert.ok(matching(problemsWith(restricted), /must not be restricted to some files/u).length === 1);
  const bootstrap = mutate('CONTRIBUTING.md', 'node scripts/install-dev-hooks.mjs', 'node scripts/other.mjs');
  assert.ok(matching(problemsWith(bootstrap), /Development Setup does not run/u).length === 1);
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['disabledHookDetected'], 'a disabled, restricted or non-executable hook is detected');
});

test('an unwrapped cache-producing command is detected', () => {
  const config = '.pre-commit-config.yaml';
  const unwrappedHook = mutate(config, "cd rust && node ../scripts/with-cache-cleanup.mjs --event test -- cargo test'", "cd rust && cargo test'");
  assert.deepEqual(problemsWith(unwrappedHook), ['.pre-commit-config.yaml hook cargo-test runs a cache-producing command without scripts/with-cache-cleanup.mjs']);

  const rust = '.github/workflows/rust.yml';
  const unwrappedStep = mutate(rust, 'node ../scripts/with-cache-cleanup.mjs --event coverage --protect lcov.info -- cargo llvm-cov', 'cargo llvm-cov');
  assert.deepEqual(problemsWith(unwrappedStep), [
    '.github/workflows/rust.yml job coverage step "Generate code coverage" runs a cache-producing command without scripts/with-cache-cleanup.mjs',
  ]);
  const js = '.github/workflows/js.yml';
  const unwrappedPack = mutate(js, 'node ../scripts/with-cache-cleanup.mjs --event package -- npm pack --dry-run', 'npm pack --dry-run');
  assert.equal(matching(problemsWith(unwrappedPack), /job test step "Verify npm package contents"/u).length, 1);
  // A wrapper without an event, or with the command outside it, does not count.
  const noEvent = checkWorkflow('w.yml', 'jobs:\n  a:\n    steps:\n      - run: node scripts/with-cache-cleanup.mjs -- cargo build\n      - if: always()\n        run: node scripts/clean-caches.mjs --event ci-teardown\n');
  assert.equal(noEvent.length, 1);
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['unwrappedCommandDetected'], 'an unwrapped cache-producing command is detected');
});

test('a missing, misplaced or conditional CI teardown is detected', () => {
  const workflow = (steps) => `name: W\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n${steps}`;
  const wrapped = '      - run: node ../scripts/with-cache-cleanup.mjs --event build -- cargo build\n';
  const teardown = (condition) => `      - name: Clean\n${condition}        run: node scripts/clean-caches.mjs --event ci-teardown\n`;
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped + teardown('        if: ${{ !cancelled() }}\n'))), []);
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped + teardown('        if: always()\n'))), [], 'always() also runs after failures');
  const expected = ['w.yml job build runs cargo or npm but does not end with an "if: ${{ !cancelled() }}" step running scripts/clean-caches.mjs --event ci-teardown'];
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped)), expected, 'missing');
  assert.deepEqual(checkWorkflow('w.yml', workflow(teardown('        if: ${{ !cancelled() }}\n') + wrapped)), expected, 'not last');
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped + teardown(''))), expected, 'skipped after a failure');
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped + teardown('        if: success()\n'))), expected, 'only on success');
  assert.deepEqual(checkWorkflow('w.yml', workflow(wrapped + teardown('        if: cancelled()\n'))), expected, 'only on cancellation');
  assert.deepEqual(checkWorkflow('w.yml', workflow('      - run: npx --yes secretlint "**/*"\n')), expected, 'npx touches the npm cache');
  assert.deepEqual(checkWorkflow('w.yml', workflow('      - run: echo no caches\n')), [], 'a job without cargo or npm needs none');
  assert.deepEqual(checkWorkflow('w.yml', workflow(`${wrapped}      - if: \${{ always() }}\n        run: node scripts/clean-caches.mjs --event ci-teardown\n`)), [], 'if as the first key');

  const rust = '.github/workflows/rust.yml';
  const text = read(rust);
  const coverage = text.indexOf('  coverage:');
  const teardownStart = text.indexOf('      - name: Clean regenerable caches', coverage);
  const teardownEnd = text.indexOf('\n\n', teardownStart);
  const removed = { [rust]: text.slice(0, teardownStart) + text.slice(teardownEnd + 2) };
  assert.equal(matching(problemsWith(removed), /job coverage runs cargo or npm but does not end/u).length, 1);
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['missingTeardownDetected'], 'a missing, misplaced or conditional CI teardown is detected');
});

test('an omitted cache category is detected', () => {
  const covered = new Set(CACHE_CLASSES.flatMap(({ covers }) => covers));
  for (const category of REQUIRED_CATEGORIES) assert.ok(covered.has(category), `${category} has a class`);
  for (const omitted of ['rust-incremental', 'container', 'nested-consumer-target', 'lean-build']) {
    const classes = CACHE_CLASSES.map((cacheClass) => ({ ...cacheClass, covers: cacheClass.covers.filter((category) => category !== omitted) }));
    assert.deepEqual(problemsWith({}, { classes }), [`cache category ${omitted} has no class in scripts/lib/cache-classes.mjs`]);
  }
  const withoutContainers = CACHE_CLASSES.filter(({ id }) => id !== 'containers');
  assert.deepEqual(matching(problemsWith({}, { classes: withoutContainers }), /container|buildkit/u).length, 2);
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['omittedCategoryDetected'], 'an omitted cache category is detected');
});

test('a Cargo profile that re-enables heavy debug information or incremental state is detected', () => {
  const manifest = read('rust/Cargo.toml');
  assert.deepEqual(checkCargoProfiles(manifest), []);
  assert.deepEqual(checkCargoProfiles(manifest.replace('[profile.dev]', '[profile.unused]')), ['rust/Cargo.toml has no [profile.dev]']);
  assert.equal(checkCargoProfiles(manifest.replace('debug = "line-tables-only"', 'debug = true')).length, 1);
  assert.equal(checkCargoProfiles(manifest.replace('incremental = false', 'incremental = true')).length, 1);
  assert.equal(checkCargoProfiles(`${manifest}\n[profile.test]\nincremental = true\n`).length, 1);
  assert.equal(checkCargoProfiles(`${manifest}\n[profile.test]\ndebug = "full"\n`).length, 1);
  assert.deepEqual(checkCargoProfiles(`${manifest}\n[profile.test]\nopt-level = 1\n`), []);
  assert.deepEqual(problemsWith({ 'rust/Cargo.toml': manifest.replace('incremental = false', 'incremental = true') }),
    ['rust/Cargo.toml [profile.dev] must set incremental = false']);
  observeCacheCleanup('I195-CACHE-CLEANUP-POLICY', ['unboundedProfileDetected'], 'a Cargo profile that re-enables heavy debug information or incremental state is detected');
});
