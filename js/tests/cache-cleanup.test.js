// Behavior of scripts/clean-caches.mjs on temporary fixture repositories:
// every cache class, idempotence, reports, safety and the disk budget
// (requirements I195-CACHE-CLEANUP-ENTRY-POINT, -SAFETY and -BUDGET).
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { REQUIRED_CATEGORIES, checkCategories } from '../../scripts/check-cache-policy.mjs';
import { CACHE_CLASSES, CONTAINER_LABEL } from '../../scripts/lib/cache-classes.mjs';
import { acquireLease, diskUsage, dockerCommands, runCleanup, runningProcesses } from '../../scripts/lib/cache-cleanup.mjs';
import {
  CLEAN_CACHES, REPOSITORY_ROOT, exitedPid, git, makeBase, makeCargoTarget, makeRepository, observeCacheCleanup,
  populateCaches, put, readJson, runScript, toolDirectory, trySymlink, worktreeState,
} from './support/cache-fixtures.js';

const POSIX = process.platform !== 'win32';
const quiet = { docker: false, preflight: false };

function fixtureWithCaches(t) {
  const fixture = makeRepository(makeBase(t));
  return { ...fixture, ...populateCaches(fixture) };
}

const clean = (fixture, options = {}) => runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, ...quiet, ...options });
const removedPaths = (report) => Object.values(report.classes).flatMap((entry) => (entry.removed ?? []).map((removed) => removed.path));
const skippedReasons = (report) => Object.fromEntries(
  Object.values(report.classes).flatMap((entry) => (entry.skipped ?? []).map((skipped) => [skipped.path, skipped.reason])),
);

function assertKept(fixture) {
  for (const [role, file] of Object.entries(fixture.kept)) assert.ok(existsSync(file), `${role} ${file} must survive`);
}

/** A fake docker that logs its arguments and reports a reachable daemon and builder. */
function fakeDocker(base) {
  const bin = path.join(base, 'fake-docker');
  const log = path.join(base, 'docker.log');
  mkdirSync(bin, { recursive: true });
  writeFileSync(path.join(bin, 'docker'), [
    '#!/bin/sh',
    'printf "%s\\n" "$*" >> "$FAKE_DOCKER_LOG"',
    'if [ "$1" = info ]; then echo 27.0.0; exit 0; fi',
    'if [ "$1" = buildx ] && [ "$2" = inspect ]; then exit 0; fi',
    'echo "Total reclaimed space: 1.5MB"',
    '',
  ].join('\n'));
  chmodSync(path.join(bin, 'docker'), 0o755);
  return { env: { ...process.env, PATH: `${bin}${path.delimiter}${process.env.PATH}`, FAKE_DOCKER_LOG: log }, log };
}

test('clean-caches is the single documented entry point and every required category has a class', () => {
  const help = runScript(CLEAN_CACHES, ['--help'], { cwd: REPOSITORY_ROOT });
  assert.equal(help.status, 0, help.stderr);
  assert.match(help.stdout, /--mode prune\|full/u);
  const listed = runScript(CLEAN_CACHES, ['--list-classes'], { cwd: REPOSITORY_ROOT });
  assert.equal(listed.status, 0, listed.stderr);
  const documentation = readFileSync(path.join(REPOSITORY_ROOT, 'docs/cache-cleanup.md'), 'utf8');
  assert.match(documentation, /node scripts\/clean-caches\.mjs/u);
  for (const cacheClass of CACHE_CLASSES) {
    assert.match(listed.stdout, new RegExp(`^${cacheClass.id}\\t`, 'mu'));
    assert.ok(documentation.includes(`\`${cacheClass.id}\``), `docs/cache-cleanup.md documents ${cacheClass.id}`);
  }
  assert.deepEqual(checkCategories(CACHE_CLASSES), []);
  const covered = new Set(CACHE_CLASSES.flatMap((cacheClass) => cacheClass.covers));
  for (const category of REQUIRED_CATEGORIES) assert.ok(covered.has(category), category);
  const bad = runScript(CLEAN_CACHES, ['--mode', 'everything'], { cwd: REPOSITORY_ROOT });
  assert.equal(bad.status, 2);
  observeCacheCleanup('I195-CACHE-CLEANUP-ENTRY-POINT', ['singleDocumentedEntryPoint', 'everyRequiredCategoryRegistered'],
    'clean-caches is the single documented entry point and every required category has a class');
});

test('a full clean removes every cache class, reports its bytes, keeps evidence and work, and a repeat is a no-op', { skip: !POSIX }, (t) => {
  const fixture = fixtureWithCaches(t);
  const docker = fakeDocker(fixture.base);
  const stateBefore = worktreeState(fixture.root);
  const reportFile = path.join(fixture.base, 'reports', 'first.json');
  const first = runScript(CLEAN_CACHES, ['--full', '--json', reportFile], {
    cwd: fixture.root,
    env: { ...docker.env, TMPDIR: fixture.tmpRoot, TMP: fixture.tmpRoot, TEMP: fixture.tmpRoot },
  });
  assert.equal(first.status, 0, first.stderr);
  const report = readJson(reportFile);
  assert.equal(report.status, 'ok');
  assert.match(first.stdout, /cache cleanup \(manual, full\): before .+, after .+, reclaimed .+/u);

  for (const [role, file] of Object.entries(fixture.caches)) assert.ok(!existsSync(file), `${role} ${file} must be removed`);
  for (const cacheClass of CACHE_CLASSES.filter(({ docker: isDocker }) => !isDocker)) {
    assert.ok(report.classes[cacheClass.id].removed.length > 0, `class ${cacheClass.id} removed something`);
    assert.ok(report.classes[cacheClass.id].reclaimedBytes > 0, `class ${cacheClass.id} reclaimed bytes`);
  }
  assert.equal(report.classes.containers.status, 'ok');
  assert.equal(report.classes.containers.commands.length, 4);
  assertKept(fixture);
  assert.equal(worktreeState(fixture.root), stateBefore);

  assert.ok(report.beforeBytes > 0);
  assert.equal(report.afterBytes, 0);
  assert.equal(report.reclaimedBytes, report.beforeBytes - report.afterBytes);
  assert.equal(report.withinBudget, true);
  const history = readFileSync(path.join(fixture.root, '.git/meta-language-cache/history.jsonl'), 'utf8').trim().split('\n');
  assert.equal(JSON.parse(history.at(-1)).reclaimedBytes, report.reclaimedBytes);
  observeCacheCleanup('I195-CACHE-CLEANUP-ENTRY-POINT', ['everyClassRemovedFromFixture', 'reportsBeforeAfterReclaimed'],
    'a full clean removes every cache class and reports its bytes');

  const again = clean(fixture, { mode: 'full' });
  assert.equal(again.status, 'ok');
  assert.deepEqual(removedPaths(again), []);
  assert.equal(again.reclaimedBytes, 0);
  assert.equal(again.beforeBytes, 0);
  assertKept(fixture);
  assert.equal(worktreeState(fixture.root), stateBefore);
  observeCacheCleanup('I195-CACHE-CLEANUP-ENTRY-POINT', ['repeatedRunIsIdempotent'], 'a repeated full clean is a no-op');
});

test('cleanup keeps source, tracked and uncommitted files, evidence and paths outside the worktree', (t) => {
  const fixture = fixtureWithCaches(t);
  const outside = makeCargoTarget(path.join(fixture.base, 'sibling project', 'target'));
  const stateBefore = worktreeState(fixture.root);
  const trackedBefore = git(fixture.root, ['ls-files', '-s']);
  const report = clean(fixture, { mode: 'full' });
  const reasons = skippedReasons(report);

  assertKept(fixture);
  assert.equal(worktreeState(fixture.root), stateBefore);
  assert.equal(git(fixture.root, ['ls-files', '-s']), trackedBefore);
  assert.equal(reasons['rust/web/pkg'], 'holds tracked files');
  assert.ok(existsSync(path.join(outside, 'debug/deps/libfixture.rlib')));
  assert.ok(existsSync(fixture.kept.environment), 'ignored files outside every class stay');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['sourceAndTrackedFilesPreserved', 'ignoredOnlyInsideRepository'],
    'cleanup keeps source, tracked and uncommitted files and paths outside the worktree');

  assert.equal(reasons['issue-195-results/work/execution-records.jsonl'], 'evidence record');
  assert.equal(reasons['issue-195-results/work/native'], 'native validation evidence');
  assert.ok(existsSync(fixture.kept.results) && existsSync(fixture.kept.artifacts) && existsSync(fixture.kept.ciLogs));
  assert.ok(existsSync(fixture.kept.liveScratch), 'scratch of a live process stays');
  assert.ok(existsSync(fixture.kept.foreignScratch), 'scratch of another worktree stays');
  assert.ok(existsSync(fixture.kept.unmarkedScratch), 'unmarked temporary directories stay');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['evidencePreserved'], 'cleanup keeps every evidence file the gate reads');
});

test('cleanup rejects symlink escapes and path traversal', { skip: !POSIX }, (t) => {
  const fixture = makeRepository(makeBase(t));
  const victim = put(fixture.base, 'victim/cache/precious.txt', 'must survive\n');
  const victimTarget = makeCargoTarget(path.join(fixture.base, 'victim target'));
  assert.ok(trySymlink(path.dirname(victim), path.join(fixture.root, 'js/node_modules/.cache')));
  assert.ok(trySymlink(victimTarget, path.join(fixture.root, 'target')));
  const report = clean(fixture, { mode: 'full' });
  const reasons = skippedReasons(report);

  assert.ok(existsSync(victim));
  assert.ok(existsSync(path.join(victimTarget, 'debug/incremental')));
  assert.ok(existsSync(path.join(victimTarget, 'release/deps/libfixture.rlib')));
  assert.equal(reasons['js/node_modules/.cache'], 'symlink');
  assert.match(reasons.target, /symlink/u);

  for (const flag of ['--target-dir', '--results-dir', '--protect']) {
    const traversal = runScript(CLEAN_CACHES, [flag, `rust/../../${path.basename(fixture.base)}`], { cwd: fixture.root });
    assert.equal(traversal.status, 2, `${flag} traversal is a usage error`);
    assert.match(traversal.stderr, /must not contain '\.\.'/u);
  }
  assert.ok(existsSync(victim));
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['symlinkAndTraversalRejected'], 'cleanup rejects symlink escapes and path traversal');
});

test('cleanup leaves other worktrees and their targets alone', (t) => {
  const fixture = fixtureWithCaches(t);
  const nested = path.join(fixture.root, '.issue-195-work', 'other worktree');
  const sibling = path.join(fixture.base, 'sibling worktree');
  git(fixture.root, ['worktree', 'add', '-q', '--detach', nested], fixture.env);
  git(fixture.root, ['worktree', 'add', '-q', '--detach', sibling], fixture.env);
  makeCargoTarget(path.join(nested, 'rust/target'));
  makeCargoTarget(path.join(sibling, 'rust/target'));

  const report = clean(fixture, { mode: 'full' });
  assert.equal(skippedReasons(report)['.issue-195-work'], 'belongs to another worktree');
  for (const worktree of [nested, sibling]) {
    assert.ok(existsSync(path.join(worktree, 'README.md')));
    assert.ok(existsSync(path.join(worktree, 'rust/target/debug/incremental')), `${worktree} keeps its target`);
  }
  assert.ok(!existsSync(fixture.caches.rustTarget), 'the worktree still cleans its own target');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['otherWorktreesPreserved'], 'cleanup leaves other worktrees and their targets alone');
});

test('cleanup keeps the outputs of active builds', async (t) => {
  const fixture = fixtureWithCaches(t);
  const lease = acquireLease({ cwd: fixture.root, label: 'cargo test', pid: process.ppid });
  const leased = clean(fixture, { mode: 'full' });
  lease.release();
  assert.ok(existsSync(fixture.caches.rustTarget));
  assert.ok(existsSync(fixture.caches.leanBuild));
  assert.match(skippedReasons(leased)['rust/target'], /active build \(cargo test pid \d+\)/u);
  assert.deepEqual(leased.activeBuilds.leases.map(({ label }) => label), ['cargo test']);
  assert.ok(!existsSync(fixture.caches.jsCache), 'caches no build writes to are still cleaned');

  const stale = acquireLease({ cwd: fixture.root, label: 'crashed build', pid: exitedPid() });
  const flock = spawnSync('flock', ['--version'], { stdio: 'ignore' }).status === 0;
  if (flock) {
    const lockFile = path.join(fixture.caches.rustTarget, 'debug/.cargo-lock');
    // A process group, so killing it also ends the `sleep` that inherited the lock.
    const holder = spawn('flock', [lockFile, 'sleep', '30'], { stdio: 'ignore', detached: true });
    const stop = () => {
      try {
        process.kill(-holder.pid, 'SIGKILL');
      } catch {
        // Already gone.
      }
    };
    t.after(stop);
    const deadline = Date.now() + 10000;
    while (spawnSync('flock', ['--nonblock', lockFile, 'true']).status === 0) {
      assert.ok(Date.now() < deadline, 'the lock holder started');
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
    const locked = clean(fixture, { mode: 'full' });
    const reasons = skippedReasons(locked);
    assert.equal(reasons['rust/target'], 'a running Cargo build holds its lock');
    assert.equal(reasons['rust/target/debug'], 'a running Cargo build holds its lock');
    assert.ok(existsSync(path.join(fixture.caches.rustTarget, 'debug/deps/libfixture.rlib')));
    assert.ok(!existsSync(fixture.caches.consumerTarget), 'unlocked targets are still cleaned');
    const exited = new Promise((resolve) => holder.once('exit', resolve));
    stop();
    await exited;
  }
  const released = clean(fixture, { mode: 'full' });
  assert.ok(!existsSync(stale.file), 'a stale lease is dropped');
  assert.deepEqual(released.activeBuilds.leases, []);
  assert.ok(!existsSync(fixture.caches.rustTarget), 'the target is removed once no build is active');
  if (flock) {
    observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['activeBuildOutputsPreserved'], 'cleanup keeps the outputs of active builds');
  }
});

// Cargo releases its lock before `cargo test` runs the binaries it built; a
// commit hook cleanup once emptied rust/target under a running test suite.
test('cleanup keeps a target whose built binary is still running', { skip: !POSIX }, async (t) => {
  const fixture = fixtureWithCaches(t);
  const binary = path.join(fixture.caches.rustTarget, 'debug/deps/unit-fixture');
  copyFileSync(spawnSync('sh', ['-c', 'command -v sleep'], { encoding: 'utf8' }).stdout.trim(), binary);
  chmodSync(binary, 0o755);
  const running = spawn(binary, ['30'], { stdio: 'ignore' });
  const exited = new Promise((resolve) => running.once('exit', resolve));
  t.after(() => running.kill('SIGKILL'));
  const deadline = Date.now() + 10000;
  while (!runningProcesses().processes.some(({ pid }) => pid === running.pid)) {
    assert.ok(Date.now() < deadline, 'the binary started');
    await new Promise((resolve) => setTimeout(resolve, 50));
  }

  const busy = clean(fixture, { mode: 'full' });
  assert.match(busy.activeBuilds.processProbe, /^(?:proc|ps)$/u);
  const reasons = skippedReasons(busy);
  assert.equal(reasons['rust/target'], `a running process (pid ${running.pid}) executes rust/target/debug/deps/unit-fixture`);
  assert.ok(existsSync(binary), 'the running binary stays');
  assert.ok(existsSync(path.join(fixture.caches.rustTarget, 'debug/deps/libfixture.rlib')), 'its target directory stays');
  assert.ok(!existsSync(path.join(fixture.caches.rustTarget, 'debug/incremental')), 'incremental state beside it is still removed');
  assert.ok(!existsSync(fixture.caches.consumerTarget), 'other targets are still cleaned');

  running.kill('SIGKILL');
  await exited;
  clean(fixture, { mode: 'full' });
  assert.ok(!existsSync(fixture.caches.rustTarget), 'the target is removed once the binary exits');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['activeBuildOutputsPreserved'], 'cleanup keeps a target whose built binary is still running');
});

// Between compiling and running tests Cargo runs rustdoc, which neither holds
// the lock nor executes from the target, so a Cargo process working in the
// workspace keeps its target too.
test('cleanup keeps the target of a Cargo process running in its workspace', { skip: process.platform !== 'linux' }, async (t) => {
  const fixture = fixtureWithCaches(t);
  const tools = mkdtempSync(path.join(fixture.base, 'tools-'));
  const cargo = path.join(tools, 'cargo');
  copyFileSync(spawnSync('sh', ['-c', 'command -v sleep'], { encoding: 'utf8' }).stdout.trim(), cargo);
  chmodSync(cargo, 0o755);
  const running = spawn(cargo, ['30'], { cwd: path.dirname(fixture.caches.rustTarget), stdio: 'ignore' });
  const exited = new Promise((resolve) => running.once('exit', resolve));
  t.after(() => running.kill('SIGKILL'));
  const deadline = Date.now() + 10000;
  while (!runningProcesses().processes.some(({ pid, executable }) => pid === running.pid && executable === cargo)) {
    assert.ok(Date.now() < deadline, 'the process started');
    await new Promise((resolve) => setTimeout(resolve, 50));
  }

  const busy = clean(fixture, { mode: 'full' });
  assert.equal(skippedReasons(busy)['rust/target'], `a running Cargo process (pid ${running.pid}) builds in its workspace`);
  assert.ok(existsSync(path.join(fixture.caches.rustTarget, 'debug/deps/libfixture.rlib')), 'its target stays');
  assert.ok(!existsSync(fixture.caches.consumerTarget), 'targets of other workspaces are still cleaned');

  running.kill('SIGKILL');
  await exited;
  clean(fixture, { mode: 'full' });
  assert.ok(!existsSync(fixture.caches.rustTarget), 'the target is removed once Cargo exits');
});

test('cleanup handles custom Cargo target directories and paths with spaces', (t) => {
  const fixture = fixtureWithCaches(t);
  const custom = makeCargoTarget(path.join(fixture.base, 'custom target dir'));
  const fromEnvironment = makeCargoTarget(path.join(fixture.base, 'env target dir'));
  const notATarget = put(fixture.base, 'not a target/debug/incremental/keep', 4 * 1024);
  const result = runScript(CLEAN_CACHES, [
    '--no-docker', '--no-preflight', '--target-dir', custom, '--target-dir', path.dirname(path.dirname(path.dirname(notATarget))),
    '--json', path.join(fixture.base, 'custom report.json'),
  ], {
    cwd: path.join(fixture.root, 'docs'),
    env: { ...process.env, CARGO_TARGET_DIR: fromEnvironment, TMPDIR: fixture.tmpRoot },
  });
  assert.equal(result.status, 0, result.stderr);
  const report = readJson(path.join(fixture.base, 'custom report.json'));
  const removed = removedPaths(report);
  for (const target of [custom, fromEnvironment]) {
    assert.ok(!existsSync(path.join(target, 'debug/incremental')), `${target} incremental state removed`);
    assert.ok(existsSync(path.join(target, 'debug/deps/libfixture.rlib')), `${target} keeps its warm cache`);
    assert.ok(removed.includes(path.join(target, 'debug', 'incremental')));
  }
  assert.ok(existsSync(notATarget), 'a directory without CACHEDIR.TAG is not a target');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['customTargetDirectoriesHandled'], 'cleanup handles custom Cargo target directories');

  assert.match(fixture.root, / /u);
  assert.ok(!existsSync(path.join(fixture.caches.rustTarget, 'debug/incremental')));
  assert.ok(!existsSync(fixture.caches.scratch), 'marked scratch under a temporary directory with spaces is removed');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['pathsWithSpacesHandled'], 'cleanup handles paths with spaces');
});

test('cleanup tolerates missing git, docker and flock', { skip: !POSIX }, (t) => {
  const fixture = fixtureWithCaches(t);
  const noGit = toolDirectory(fixture.base, []);
  const withoutGit = runScript(CLEAN_CACHES, ['--json', path.join(fixture.base, 'no-git.json')], {
    cwd: fixture.root,
    env: { ...process.env, PATH: noGit },
  });
  assert.equal(withoutGit.status, 0, withoutGit.stderr);
  assert.equal(readJson(path.join(fixture.base, 'no-git.json')).status, 'skipped');
  assert.ok(existsSync(fixture.caches.jsCache), 'nothing is removed without git');

  const gitOnly = toolDirectory(fixture.base, ['git']);
  const withGit = runScript(CLEAN_CACHES, ['--no-preflight', '--json', path.join(fixture.base, 'git-only.json')], {
    cwd: fixture.root,
    env: { ...process.env, PATH: gitOnly, TMPDIR: fixture.tmpRoot },
  });
  assert.equal(withGit.status, 0, withGit.stderr);
  const report = readJson(path.join(fixture.base, 'git-only.json'));
  assert.equal(report.status, 'ok');
  assert.equal(report.classes.containers.status, 'skipped');
  assert.match(report.classes.containers.reason, /docker is not installed/u);
  assert.equal(report.activeBuilds.cargoLockProbe, 'unavailable');
  assert.ok(!existsSync(fixture.caches.jsCache));
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['missingToolsTolerated'], 'cleanup tolerates missing git, docker and flock');
});

test('container cleanup only prunes resources with the project label and builder', { skip: !POSIX }, (t) => {
  const fixture = makeRepository(makeBase(t));
  const docker = fakeDocker(fixture.base);
  for (const mode of ['prune', 'full']) {
    writeFileSync(docker.log, '');
    const report = runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, env: docker.env, mode, preflight: false });
    assert.equal(report.classes.containers.status, 'ok');
    const calls = readFileSync(docker.log, 'utf8').trim().split('\n');
    const mutating = calls.filter((call) => !/^(?:info|buildx inspect)\b/u.test(call));
    assert.equal(mutating.length, 4);
    for (const call of mutating) {
      assert.match(call, /^(?:container|image|volume|buildx) prune /u, call);
      if (call.startsWith('buildx')) assert.match(call, /--builder meta-language\b/u);
      else assert.ok(call.includes(`--filter label=${CONTAINER_LABEL}`), call);
      assert.doesNotMatch(call, /\bsystem\b|\brm\b|\brmi\b/u);
    }
  }
  for (const mode of ['prune', 'full']) {
    for (const args of dockerCommands(mode)) {
      assert.ok(args.includes(`label=${CONTAINER_LABEL}`) || args.includes('meta-language'), args.join(' '));
    }
  }
  const skipped = runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, env: docker.env, docker: false, preflight: false });
  assert.equal(skipped.classes.containers.status, 'skipped');
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['onlyLabeledContainerResources'],
    'container cleanup only prunes resources with the project label and builder');
});

test('concurrent cleanups of one worktree are serialized', async (t) => {
  const fixture = fixtureWithCaches(t);
  const lockFile = path.join(fixture.root, '.git/meta-language-cache/cleanup.lock');
  put(fixture.root, '.git/meta-language-cache/cleanup.lock', `${process.ppid}\n`);
  const busy = clean(fixture, { mode: 'full' });
  assert.equal(busy.status, 'busy');
  assert.ok(existsSync(fixture.caches.rustTarget), 'a busy run removes nothing');
  writeFileSync(lockFile, `${exitedPid()}\n`);

  // A lock created but not yet written, or a lost race to break a stale
  // lock, must not let a second cleanup in.
  writeFileSync(lockFile, '');
  assert.equal(clean(fixture, { mode: 'full' }).status, 'busy', 'a lock without a pid yet is busy');
  writeFileSync(lockFile, `${exitedPid()}\n`);
  mkdirSync(`${lockFile}.break`);
  assert.equal(clean(fixture, { mode: 'full' }).status, 'busy', 'another contender is breaking the stale lock');
  rmSync(`${lockFile}.break`, { recursive: true });
  assert.ok(existsSync(fixture.caches.rustTarget));

  // Several rounds of eight contenders: before the lock was created with its
  // pid in place, a contender could read it while its holder released it,
  // fail with ENOENT and exit 1 (experiments/cache-cleanup-lock-race.mjs).
  for (let round = 0; round < 3; round += 1) {
    const contenders = fixtureWithCaches(t);
    const env = { ...process.env, TMPDIR: contenders.tmpRoot };
    const reports = [1, 2, 3, 4, 5, 6, 7, 8].map((index) => path.join(fixture.base, `round-${round}-run-${index}.json`));
    const runs = await Promise.all(reports.map((report) => new Promise((resolve) => {
      const child = spawn(process.execPath, [CLEAN_CACHES, '--full', '--no-docker', '--json', report], {
        cwd: contenders.root,
        env,
        stdio: 'ignore',
      });
      child.on('exit', (code) => resolve(code));
    })));
    assert.deepEqual(runs, reports.map(() => 0));
    const statuses = reports.map((report) => readJson(report).status);
    assert.ok(statuses.includes('ok'));
    assert.ok(statuses.every((status) => status === 'ok' || status === 'busy'), statuses.join(','));
    assert.ok(!existsSync(path.join(contenders.root, '.git/meta-language-cache/cleanup.lock')), 'the lock is released');
    for (const file of Object.values(contenders.caches)) assert.ok(!existsSync(file), file);
    assertKept(contenders);
  }
  observeCacheCleanup('I195-CACHE-CLEANUP-SAFETY', ['concurrentCleanupsSerialized'], 'concurrent cleanups of one worktree are serialized');
});

test('pruning enforces the budget, keeping warm caches that fit', (t) => {
  const fixture = fixtureWithCaches(t);
  const generous = clean(fixture, { budgetMb: 64 });
  assert.equal(generous.effectiveMode, 'prune');
  assert.ok(!existsSync(path.join(fixture.caches.rustTarget, 'debug/incremental')), 'transient state is always removed');
  assert.ok(!existsSync(fixture.caches.jsCache));
  for (const warm of ['debug/deps/libfixture.rlib', 'release/deps/libfixture.rlib', 'doc/fixture/index.html']) {
    assert.ok(existsSync(path.join(fixture.caches.rustTarget, warm)), `${warm} is kept within the budget`);
  }
  assert.ok(generous.withinBudget);
  assert.ok(generous.afterBytes > 0);
  observeCacheCleanup('I195-CACHE-CLEANUP-BUDGET', ['warmCacheKeptWithinBudget'], 'pruning keeps warm caches that fit the budget');

  const budgetBytes = 128 * 1024;
  const tight = clean(fixture, { budgetMb: budgetBytes / (1024 * 1024) });
  assert.ok(tight.afterBytes <= budgetBytes, `${tight.afterBytes} <= ${budgetBytes}`);
  assert.equal(tight.withinBudget, true);
  const warmRemoved = tight.classes['rust-target'].removed.map((removed) => removed.path);
  assert.deepEqual(warmRemoved, ['rust/target/doc', 'rust/target/release'], 'lowest priority first, stopping at the budget');
  assert.ok(existsSync(path.join(fixture.caches.rustTarget, 'debug/deps/libfixture.rlib')));

  const zero = clean(fixture, { budgetMb: 0 });
  assert.ok(!existsSync(path.join(fixture.caches.rustTarget, 'debug')));
  assert.ok(existsSync(fixture.caches.rustTarget), 'pruning never removes the whole target');
  assert.equal(zero.afterBytes, diskUsage(fixture.caches.rustTarget), 'only the empty target skeleton is left');
  observeCacheCleanup('I195-CACHE-CLEANUP-BUDGET', ['budgetEnforced'], 'pruning enforces the aggregate disk budget');
});

test('full mode removes every class and a low-disk preflight escalates to it', (t) => {
  const full = fixtureWithCaches(t);
  const report = clean(full, { mode: 'full', budgetMb: 1024 });
  for (const [role, file] of Object.entries(full.caches)) assert.ok(!existsSync(file), `${role} removed in full mode`);
  assert.equal(report.effectiveMode, 'full');
  observeCacheCleanup('I195-CACHE-CLEANUP-BUDGET', ['fullModeRemovesEveryClass'], 'full mode removes every class');

  const low = fixtureWithCaches(t);
  const relaxed = clean(low, { preflight: false, minFreeMb: Number.MAX_SAFE_INTEGER / (1024 * 1024) });
  assert.equal(relaxed.preflight.escalated, false);
  assert.ok(existsSync(low.caches.rustTarget));
  const escalated = runScript(CLEAN_CACHES, ['--no-docker', '--min-free-mb', String(1024 * 1024 * 1024), '--json', path.join(low.base, 'low.json')], {
    cwd: low.root,
    env: { ...process.env, TMPDIR: low.tmpRoot },
  });
  assert.equal(escalated.status, 0, escalated.stderr);
  assert.match(escalated.stdout, /after low-disk preflight/u);
  const lowReport = readJson(path.join(low.base, 'low.json'));
  assert.equal(lowReport.preflight.escalated, true);
  assert.equal(lowReport.effectiveMode, 'full');
  assert.equal(lowReport.preflight.lowDiskAfter, true);
  assert.ok(!existsSync(low.caches.rustTarget));
  assertKept(low);
  observeCacheCleanup('I195-CACHE-CLEANUP-BUDGET', ['lowDiskPreflightEscalates'], 'a low-disk preflight escalates to a full clean');
});
