// The developer bootstrap, the every-commit hook and the cleanup wrapper, on
// temporary clones (requirements I195-CACHE-CLEANUP-HOOK and -EVENTS).
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { chmodSync, cpSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { checkBootstrap } from '../../scripts/check-cache-policy.mjs';
import { boundedEnvironment, boundedJobs } from '../../scripts/with-cache-cleanup.mjs';
import {
  INSTALL_DEV_HOOKS, REPOSITORY_ROOT, WITH_CACHE_CLEANUP, git, isolatedGitEnvironment, makeBase, makeCargoTarget,
  makeRepository, observeCacheCleanup, put, readJson,
} from './support/cache-fixtures.js';

const POSIX = process.platform !== 'win32';
const nodeOnPath = (env) => ({ ...env, PATH: `${path.dirname(process.execPath)}${path.delimiter}${env.PATH ?? ''}` });

/**
 * An upstream repository holding the real cleanup scripts, hook and
 * .gitignore, and a fresh clone of it, both with an isolated global config.
 */
function freshClone(t) {
  const base = makeBase(t);
  const env = nodeOnPath(isolatedGitEnvironment(base));
  const upstream = path.join(base, 'upstream repo');
  mkdirSync(upstream);
  git(upstream, ['init', '-q', '-b', 'main'], env);
  cpSync(path.join(REPOSITORY_ROOT, 'scripts'), path.join(upstream, 'scripts'), { recursive: true });
  cpSync(path.join(REPOSITORY_ROOT, '.githooks'), path.join(upstream, '.githooks'), { recursive: true });
  cpSync(path.join(REPOSITORY_ROOT, '.gitignore'), path.join(upstream, '.gitignore'));
  chmodSync(path.join(upstream, '.githooks/pre-commit'), 0o755);
  put(upstream, 'README.md', '# upstream\n');
  put(upstream, 'docs/guide.md', '# guide\n');
  git(upstream, ['add', '-A'], env);
  git(upstream, ['commit', '-q', '-m', 'upstream'], env);
  const clone = path.join(base, 'fresh clone');
  git(base, ['clone', '-q', upstream, clone], env);
  return { base, env, upstream, clone, globalConfig: env.GIT_CONFIG_GLOBAL };
}

function commit(cwd, env, message) {
  const result = spawnSync('git', ['commit', '-q', '-am', message], { cwd, env, encoding: 'utf8' });
  return { status: result.status, output: `${result.stdout}${result.stderr}` };
}

function lastRun(clone) {
  const file = path.join(clone, '.git/meta-language-cache/last-run.json');
  return existsSync(file) ? readJson(file) : null;
}

test('the bootstrap installs the hook in a fresh clone, and a documentation-only commit cleans', { skip: !POSIX }, (t) => {
  const { env, upstream, clone, globalConfig } = freshClone(t);
  const globalBefore = readFileSync(globalConfig, 'utf8');
  const before = spawnSync(process.execPath, [path.join(clone, 'scripts/install-dev-hooks.mjs'), '--check'], { cwd: clone, env, encoding: 'utf8' });
  assert.equal(before.status, 1, 'a fresh clone has no active hook yet');

  const install = spawnSync(process.execPath, [path.join(clone, 'scripts/install-dev-hooks.mjs')], { cwd: clone, env, encoding: 'utf8' });
  assert.equal(install.status, 0, install.stderr);
  const check = spawnSync(process.execPath, [path.join(clone, 'scripts/install-dev-hooks.mjs'), '--check'], { cwd: clone, env, encoding: 'utf8' });
  assert.equal(check.status, 0, check.stderr);
  assert.equal(git(clone, ['config', '--local', '--get', 'core.hooksPath'], env).trim(), '.githooks');
  assert.equal(readFileSync(globalConfig, 'utf8'), globalBefore, 'the global git config is untouched');
  assert.equal(spawnSync('git', ['config', '--local', '--get', 'core.hooksPath'], { cwd: upstream, env }).status, 1, 'the upstream is untouched');
  observeCacheCleanup('I195-CACHE-CLEANUP-HOOK', ['freshCloneBootstrapInstallsHook'], 'the bootstrap installs the hook in a fresh clone');

  makeCargoTarget(path.join(clone, 'rust/target'));
  const jsCache = put(clone, 'js/node_modules/.cache/entry', 4096);
  writeFileSync(path.join(clone, 'docs/guide.md'), '# guide\n\nOnly documentation changed.\n');
  const result = commit(clone, env, 'docs: documentation only');
  assert.equal(result.status, 0, result.output);
  assert.match(result.output, /cache cleanup \(pre-commit, prune\)/u);
  assert.equal(lastRun(clone).event, 'pre-commit');
  assert.ok(!existsSync(path.join(clone, 'rust/target/debug/incremental')));
  assert.ok(!existsSync(jsCache));
  assert.ok(existsSync(path.join(clone, 'rust/target/debug/deps/libfixture.rlib')), 'the warm cache stays within the budget');
  assert.equal(git(clone, ['log', '-1', '--format=%s'], env).trim(), 'docs: documentation only');
  observeCacheCleanup('I195-CACHE-CLEANUP-HOOK', ['nonCodeCommitRunsCleanup'], 'a documentation-only commit cleans the caches');
});

test('the hook runs an existing pre-commit hook first and keeps its status', { skip: !POSIX }, (t) => {
  const { base, env, clone } = freshClone(t);
  const install = spawnSync(process.execPath, [path.join(clone, 'scripts/install-dev-hooks.mjs')], { cwd: clone, env, encoding: 'utf8' });
  assert.equal(install.status, 0, install.stderr);
  const chainedLog = path.join(base, 'chained.log');
  const chained = path.join(clone, '.git/hooks/pre-commit');
  const writeChained = (status) => {
    writeFileSync(chained, `#!/bin/sh\necho "chained $META_LANGUAGE_HOOK_CHAINED" >> "${chainedLog}"\nexit ${status}\n`);
    chmodSync(chained, 0o755);
  };

  writeChained(1);
  const cache = put(clone, 'js/node_modules/.cache/entry', 4096);
  writeFileSync(path.join(clone, 'README.md'), '# upstream\n\nrejected change\n');
  const rejected = commit(clone, env, 'docs: rejected by the chained hook');
  assert.notEqual(rejected.status, 0, 'the chained hook status is kept');
  assert.equal(readFileSync(chainedLog, 'utf8'), 'chained 1\n');
  assert.ok(!existsSync(cache), 'the cleanup still runs');
  assert.equal(git(clone, ['rev-list', '--count', 'HEAD'], env).trim(), '1');

  writeChained(0);
  const accepted = commit(clone, env, 'docs: accepted');
  assert.equal(accepted.status, 0, accepted.output);
  assert.equal(readFileSync(chainedLog, 'utf8'), 'chained 1\nchained 1\n');
  assert.equal(git(clone, ['rev-list', '--count', 'HEAD'], env).trim(), '2');

  const foreign = spawnSync('git', ['config', '--local', 'core.hooksPath', 'other-hooks'], { cwd: clone, env });
  assert.equal(foreign.status, 0);
  const refused = spawnSync(process.execPath, [path.join(clone, 'scripts/install-dev-hooks.mjs')], { cwd: clone, env, encoding: 'utf8' });
  assert.equal(refused.status, 1);
  assert.match(refused.stderr, /already 'other-hooks'/u);
  assert.equal(git(clone, ['config', '--local', '--get', 'core.hooksPath'], env).trim(), 'other-hooks', 'another hooks path is not replaced');
  observeCacheCleanup('I195-CACHE-CLEANUP-HOOK', ['existingHooksComposed'], 'the hook runs an existing pre-commit hook first and keeps its status');
});

test('no package, build script or lifecycle script installs hooks into a consumer', () => {
  const read = (relative) => readFileSync(path.join(REPOSITORY_ROOT, relative), 'utf8');
  assert.deepEqual(checkBootstrap({ contributing: read('CONTRIBUTING.md'), packageJson: read('js/package.json'), buildScript: read('rust/build.rs') }), []);
  const packageJson = JSON.parse(read('js/package.json'));
  for (const file of packageJson.files) assert.doesNotMatch(file, /githooks|scripts/u, 'the npm package ships no hook or bootstrap');
  const manifest = read('rust/Cargo.toml');
  assert.doesNotMatch(manifest, /githooks|install-dev-hooks/u);
  assert.deepEqual(
    checkBootstrap({ contributing: read('CONTRIBUTING.md'), packageJson: '{"scripts":{"postinstall":"git config core.hooksPath .githooks"}}', buildScript: '' }).length,
    1,
    'a lifecycle script that changes hooks is detected',
  );
  const installer = read('scripts/install-dev-hooks.mjs');
  assert.doesNotMatch(installer, /'--global'|'--system'/u);
  observeCacheCleanup('I195-CACHE-CLEANUP-HOOK', ['downstreamGitConfigUntouched'], 'no package, build script or lifecycle script installs hooks into a consumer');
  assert.ok(existsSync(INSTALL_DEV_HOOKS));
});

/** Runs the wrapper in `cwd` around `node -e code args...`. */
function wrap(cwd, code, args = [], { env = process.env, event = 'test' } = {}) {
  return spawn(process.execPath, [WITH_CACHE_CLEANUP, '--event', event, '--no-docker', '--no-preflight', '--', process.execPath, '-e', code, ...args], {
    cwd,
    env,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function finished(child) {
  let output = '';
  child.stdout.on('data', (chunk) => { output += chunk; });
  child.stderr.on('data', (chunk) => { output += chunk; });
  return new Promise((resolve) => child.on('close', (code, signal) => resolve({ code, signal, output })));
}

async function waitFor(file, timeout = 20000) {
  const deadline = Date.now() + timeout;
  while (!existsSync(file)) {
    assert.ok(Date.now() < deadline, `${file} appeared`);
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
}

// The wrapped command writes a cache and reports its environment and the leases it sees.
const REPORTING_COMMAND = `
const fs = require('fs');
const path = require('path');
const [report, cache, status] = process.argv.slice(1);
fs.mkdirSync(path.dirname(cache), { recursive: true });
fs.writeFileSync(cache, 'cache');
const leases = fs.readdirSync('.git/meta-language-cache/leases');
const pick = ['CARGO_BUILD_JOBS', 'RUST_TEST_THREADS', 'CARGO_INCREMENTAL', 'SCCACHE_CACHE_SIZE'];
fs.writeFileSync(report, JSON.stringify({ leases: leases.length, env: Object.fromEntries(pick.map((name) => [name, process.env[name]])) }));
process.exit(Number(status));
`;

test('the wrapper cleans after success and failure, keeps the exit status and bounds the command', async (t) => {
  const fixture = makeRepository(makeBase(t));
  const env = { ...process.env };
  for (const name of ['CARGO_BUILD_JOBS', 'RUST_TEST_THREADS', 'CARGO_INCREMENTAL', 'SCCACHE_CACHE_SIZE', 'META_LANGUAGE_JOBS']) delete env[name];
  const cache = path.join(fixture.root, 'js/node_modules/.cache/wrapped');
  for (const [status, event] of [[0, 'test'], [3, 'build']]) {
    const report = path.join(fixture.base, `wrapped-${status}.json`);
    const run = await finished(wrap(fixture.root, REPORTING_COMMAND, [report, cache, String(status)], { env, event }));
    assert.equal(run.code, status, run.output);
    assert.match(run.output, new RegExp(`with-cache-cleanup: cache cleanup \\(${event}, prune\\).*\\(command exited ${status}\\)`, 'u'));
    assert.ok(!existsSync(cache), `the cache is removed after exit ${status}`);
    const seen = readJson(report);
    assert.equal(seen.leases, 1, 'the command runs under a lease');
    assert.deepEqual(seen.env, {
      CARGO_BUILD_JOBS: String(boundedJobs()),
      RUST_TEST_THREADS: String(boundedJobs()),
      CARGO_INCREMENTAL: '0',
      SCCACHE_CACHE_SIZE: '2G',
    });
    assert.equal(readJson(path.join(fixture.root, '.git/meta-language-cache/last-run.json')).event, event);
    assert.deepEqual(readdirSync(path.join(fixture.root, '.git/meta-language-cache/leases')), [], 'the lease is released');
  }
  observeCacheCleanup('I195-CACHE-CLEANUP-EVENTS', ['cleanupAfterSuccess', 'cleanupAfterFailure'], 'the wrapper cleans after success and failure');

  const missing = await finished(spawn(process.execPath, [WITH_CACHE_CLEANUP, '--event', 'test', '--no-docker', '--', 'definitely-missing-command-195'], {
    cwd: fixture.root,
  }));
  assert.equal(missing.code, 127);
  const usage = spawnSync(process.execPath, [WITH_CACHE_CLEANUP, '--event', 'test'], { cwd: fixture.root, encoding: 'utf8' });
  assert.equal(usage.status, 2);

  // CI keeps an output it uploads afterwards (the WebAssembly pkg) with --protect, relative to the working directory.
  const jsDirectory = path.join(fixture.root, 'js');
  const writeReport = "require('fs').mkdirSync('coverage', { recursive: true }); require('fs').writeFileSync('coverage/lcov.info', 'TN:')";
  const report = path.join(jsDirectory, 'coverage/lcov.info');
  const protectedRun = spawnSync(process.execPath, [
    WITH_CACHE_CLEANUP, '--event', 'coverage', '--no-docker', '--protect', 'coverage', '--', process.execPath, '-e', writeReport,
  ], { cwd: jsDirectory, encoding: 'utf8' });
  assert.equal(protectedRun.status, 0, protectedRun.stderr);
  assert.ok(existsSync(report), 'the protected report stays');
  const unprotectedRun = spawnSync(process.execPath, [
    WITH_CACHE_CLEANUP, '--event', 'coverage', '--no-docker', '--', process.execPath, '-e', writeReport,
  ], { cwd: jsDirectory, encoding: 'utf8' });
  assert.equal(unprotectedRun.status, 0, unprotectedRun.stderr);
  assert.ok(!existsSync(report), 'an unprotected report is a regenerable cache');

  assert.equal(boundedJobs(1), 1);
  assert.equal(boundedJobs(4), 3);
  assert.equal(boundedJobs(64), 8);
  const chosen = boundedEnvironment({ CARGO_BUILD_JOBS: '2', CARGO_INCREMENTAL: '1', SCCACHE_CACHE_SIZE: '500M' });
  assert.equal(chosen.CARGO_BUILD_JOBS, '2', 'an explicit choice is kept');
  assert.equal(chosen.CARGO_INCREMENTAL, '1');
  assert.equal(chosen.SCCACHE_CACHE_SIZE, '500M');
  assert.equal(boundedEnvironment({ META_LANGUAGE_JOBS: '5' }).RUST_TEST_THREADS, '5');
  observeCacheCleanup('I195-CACHE-CLEANUP-EVENTS', ['boundedGrowthEnvironment'], 'the wrapper bounds parallelism, incremental state and the compiler cache');
});

// The wrapped command reports that it is ready, and on SIGTERM takes a while
// to stop, writing a cache just before it dies of the signal. With `stubborn`
// it ignores SIGTERM.
const INTERRUPTIBLE_COMMAND = `
const fs = require('fs');
const path = require('path');
const [ready, stopped, cache, stubborn] = process.argv.slice(1);
process.on('SIGTERM', () => {
  if (stubborn === 'yes') return;
  setTimeout(() => {
    fs.mkdirSync(path.dirname(cache), { recursive: true });
    fs.writeFileSync(cache, 'written while stopping');
    fs.writeFileSync(stopped, 'stopped');
    process.removeAllListeners('SIGTERM');
    process.kill(process.pid, 'SIGTERM');
  }, 300);
});
fs.writeFileSync(ready, 'ready');
setInterval(() => {}, 1000);
`;

test('the wrapper forwards an interruption, waits for the command, then cleans and exits 128 plus the signal', { skip: !POSIX }, async (t) => {
  const fixture = makeRepository(makeBase(t));
  const cache = path.join(fixture.root, 'js/node_modules/.cache/late');
  const ready = path.join(fixture.base, 'ready');
  const stopped = path.join(fixture.base, 'stopped');
  const child = wrap(fixture.root, INTERRUPTIBLE_COMMAND, [ready, stopped, cache, 'no']);
  const done = finished(child);
  await waitFor(ready);
  child.kill('SIGTERM');
  const run = await done;
  assert.equal(run.code, 143, run.output);
  assert.ok(existsSync(stopped), 'the wrapper waited for the command to stop');
  assert.ok(!existsSync(cache), 'the cleanup ran after the command wrote its last cache');
  assert.match(run.output, /\(command killed by SIGTERM\)/u);
  assert.deepEqual(readdirSync(path.join(fixture.root, '.git/meta-language-cache/leases')), []);
  observeCacheCleanup('I195-CACHE-CLEANUP-EVENTS', ['cleanupAfterInterruption', 'childrenAwaited'],
    'the wrapper forwards an interruption, waits for the command, then cleans');

  const stubbornReady = path.join(fixture.base, 'stubborn-ready');
  const stubborn = wrap(fixture.root, INTERRUPTIBLE_COMMAND, [stubbornReady, stopped, cache, 'yes']);
  const stubbornDone = finished(stubborn);
  await waitFor(stubbornReady);
  stubborn.kill('SIGTERM');
  await new Promise((resolve) => setTimeout(resolve, 200));
  stubborn.kill('SIGTERM');
  const killed = await stubbornDone;
  assert.equal(killed.code, 137, killed.output);
  assert.match(killed.output, /\(command killed by SIGKILL\)/u);
  observeCacheCleanup('I195-CACHE-CLEANUP-EVENTS', ['exitStatusPreserved'], 'the wrapper exits with the status of the command');
});
