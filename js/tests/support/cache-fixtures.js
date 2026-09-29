// Temporary git repositories for the cache cleanup tests
// (js/tests/cache-cleanup*.test.js). Every fixture lives under one temporary
// directory whose name contains spaces; the real worktree is never cleaned.
import { execFileSync, spawnSync } from 'node:child_process';
import {
  copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync,
} from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { SCRATCH_MARKER } from '../../../scripts/lib/cache-classes.mjs';
import { recordIssue195Observations } from './issue-195-observations.js';

export const REPOSITORY_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..');
export const CLEAN_CACHES = path.join(REPOSITORY_ROOT, 'scripts/clean-caches.mjs');
export const WITH_CACHE_CLEANUP = path.join(REPOSITORY_ROOT, 'scripts/with-cache-cleanup.mjs');
export const INSTALL_DEV_HOOKS = path.join(REPOSITORY_ROOT, 'scripts/install-dev-hooks.mjs');
const KIB = 1024;

/** Records the cells of a cache cleanup requirement once its assertions held. */
export function observeCacheCleanup(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:cache-cleanup:${requirementId.toLowerCase()}`,
    fixtureFile: 'scripts/lib/cache-classes.mjs',
    assertions,
    testName,
    runtime: 'tooling',
  });
}

export function git(cwd, args, env = process.env) {
  return execFileSync('git', args, { cwd, env, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
}

/** Writes `bytes` bytes (or text) to `root/relative`, creating the parents. */
export function put(root, relative, content = 4 * KIB) {
  const file = path.join(root, relative);
  mkdirSync(path.dirname(file), { recursive: true });
  writeFileSync(file, typeof content === 'number' ? Buffer.alloc(content, 1) : content);
  return file;
}

/** An isolated global git configuration, so no test reads or writes the developer's. */
export function isolatedGitEnvironment(base, extra = {}) {
  const home = path.join(base, 'home');
  mkdirSync(home, { recursive: true });
  const globalConfig = path.join(home, '.gitconfig');
  if (!existsSync(globalConfig)) writeFileSync(globalConfig, '[user]\n\tname = Cache Test\n\temail = cache-test@example.invalid\n');
  return {
    ...process.env,
    HOME: home,
    GIT_CONFIG_GLOBAL: globalConfig,
    GIT_CONFIG_NOSYSTEM: '1',
    GIT_AUTHOR_NAME: 'Cache Test',
    GIT_AUTHOR_EMAIL: 'cache-test@example.invalid',
    GIT_COMMITTER_NAME: 'Cache Test',
    GIT_COMMITTER_EMAIL: 'cache-test@example.invalid',
    ...extra,
  };
}

/** A base directory with spaces in its path, removed after the test. */
export function makeBase(t) {
  const base = realpathSync(mkdtempSync(path.join(os.tmpdir(), 'cache cleanup test ')));
  t.after(() => rmSync(base, { recursive: true, force: true }));
  return base;
}

// Tracked files that no cleanup may touch: source, vendored grammar source,
// fixtures, proofs, documentation and a Lean and a Rocq project.
const TRACKED = {
  'README.md': '# fixture\n',
  'docs/guide.md': '# guide\n',
  'rust/Cargo.toml': '[package]\nname = "fixture"\nversion = "0.1.0"\n',
  'rust/src/lib.rs': 'pub fn f() {}\n',
  'js/package.json': '{ "name": "fixture", "version": "0.1.0" }\n',
  'js/src/index.js': 'export const f = 1;\n',
  'js/src/vendor/tree-sitter-demo/src/parser.c': 'int parser;\n',
  'rust/web/pkg/README.md': '# kept\n',
  'parity/fixtures/case.json': '{}\n',
  'proofs/lean/lakefile.toml': 'name = "proofs"\n',
  'proofs/lean/Main.lean': 'theorem t : True := trivial\n',
  'proofs/rocq/Main.v': 'Theorem t : True. Proof. exact I. Qed.\n',
};

/**
 * A committed repository at `<base>/fixture repo` with the real .gitignore.
 * `rust/web/pkg/README.md` is force-added, so that ignored build directory
 * holds a tracked file.
 */
export function makeRepository(base, name = 'fixture repo') {
  const root = path.join(base, name);
  mkdirSync(root, { recursive: true });
  const env = isolatedGitEnvironment(base);
  git(root, ['init', '-q', '-b', 'main'], env);
  copyFileSync(path.join(REPOSITORY_ROOT, '.gitignore'), path.join(root, '.gitignore'));
  for (const [relative, content] of Object.entries(TRACKED)) put(root, relative, content);
  git(root, ['add', '-A'], env);
  git(root, ['add', '-f', 'rust/web/pkg/README.md'], env);
  git(root, ['commit', '-q', '-m', 'fixture'], env);
  const tmpRoot = path.join(base, 'tmp');
  mkdirSync(tmpRoot, { recursive: true });
  return { base, root: realpathSync(root), tmpRoot: realpathSync(tmpRoot), env };
}

/** A Cargo target directory with debug and release profiles and the transient outputs. */
export function makeCargoTarget(target, { debug = 64 * KIB, release = 32 * KIB } = {}) {
  put(target, 'CACHEDIR.TAG', 'Signature: 8a477f597d28d172789f06886806bc55\n');
  put(target, 'debug/.fingerprint/fixture/hash', 'fingerprint');
  put(target, 'debug/.cargo-lock', '');
  put(target, 'debug/deps/libfixture.rlib', debug);
  put(target, 'debug/incremental/fixture/s-1/query-cache.bin', 16 * KIB);
  put(target, 'debug/examples/demo', 8 * KIB);
  put(target, 'release/.fingerprint/fixture/hash', 'fingerprint');
  put(target, 'release/deps/libfixture.rlib', release);
  put(target, 'llvm-cov-target/debug/deps/cov', 8 * KIB);
  put(target, 'doc/fixture/index.html', 4 * KIB);
  put(target, 'package/fixture-0.1.0.crate', 4 * KIB);
  put(target, 'criterion/bench/new/estimates.json', '{}');
  put(target, 'tmp/scratch', 4 * KIB);
  put(target, 'fixture-1.profraw', 4 * KIB);
  return target;
}

/** A marked scratch directory in the fixture's temporary root. */
export function makeScratch(fixture, name, { pid, root = fixture.root } = {}) {
  const directory = path.join(fixture.tmpRoot, name);
  put(directory, 'clone/rust/target/CACHEDIR.TAG', 'Signature: 8a477f597d28d172789f06886806bc55\n');
  put(directory, 'clone/rust/target/debug/deps/libclone.rlib', 16 * KIB);
  put(directory, SCRATCH_MARKER, `${JSON.stringify({ root, pid, label: name, createdAt: new Date().toISOString() })}\n`);
  return directory;
}

/** Process id of a process that has exited. */
export function exitedPid() {
  return spawnSync(process.execPath, ['-e', '0']).pid;
}

/**
 * Fills the fixture with one or more caches of every class and with the
 * evidence and uncommitted work that must survive. Returns the paths by role.
 */
export function populateCaches(fixture) {
  const { root } = fixture;
  makeCargoTarget(path.join(root, 'rust/target'));
  const caches = {
    rustTarget: path.join(root, 'rust/target'),
    jsCache: put(root, 'js/node_modules/.cache/babel/entry', 8 * KIB),
    jsCoverage: put(root, 'js/coverage/lcov.info', 4 * KIB),
    packedConsumer: put(root, 'js/fixture-0.1.0.tgz', 8 * KIB),
    parserBuild: put(root, 'js/src/vendor/tree-sitter-demo/node_modules/.bin/tree-sitter', 8 * KIB),
    parserObject: put(root, 'js/src/vendor/tree-sitter-demo/src/parser.o', 4 * KIB),
    leanBuild: put(root, 'proofs/lean/.lake/build/lib/Main.olean', 8 * KIB),
    rocqObject: put(root, 'proofs/rocq/Main.vo', 4 * KIB),
    rocqGlob: put(root, 'proofs/rocq/Main.glob', 4 * KIB),
    candidates: put(root, 'issue-195-results/work/candidates/npm/package.tgz', 8 * KIB),
    consumerTarget: makeCargoTarget(path.join(root, 'issue-195-results/work/consumer/rust/target'), { debug: 8 * KIB, release: 8 * KIB }),
    legacyClone: put(root, '.issue-195-work/clone/README.md', 4 * KIB),
    grammarCorpus: put(root, '.grammar-cache/corpora/tree-sitter-demo@abc123/grammar.js', 4 * KIB),
    oracleBuild: put(root, '.grammar-cache/oracles/demo-oracle/oracle', 4 * KIB),
    mergedGrammar: put(root, '.grammar-cache/merged/demo/merged.lino', 4 * KIB),
    scratch: makeScratch(fixture, 'meta-language-exited', { pid: exitedPid() }),
  };
  const kept = {
    uncommitted: put(root, 'notes.txt', 'uncommitted work\n'),
    modifiedSource: put(root, 'rust/src/lib.rs', 'pub fn f() { /* edited */ }\n'),
    trackedInIgnored: path.join(root, 'rust/web/pkg/README.md'),
    besideTracked: put(root, 'rust/web/pkg/fixture_bg.wasm', 8 * KIB),
    records: put(root, 'issue-195-results/work/execution-records.jsonl', '{}\n'),
    runLog: put(root, 'issue-195-results/work/javascript-suite.log', 'log\n'),
    nativeEvidence: put(root, 'issue-195-results/work/native/rust/target-a/emitted.rs', 'fn main() {}\n'),
    results: put(root, 'issue-195-results/requirement-results.json', '{}\n'),
    artifacts: put(root, 'issue-195-artifacts/report.md', '# report\n'),
    ciLogs: put(root, 'ci-logs/run.log', 'log\n'),
    environment: put(root, '.env', 'SECRET=1\n'),
    liveScratch: makeScratch(fixture, 'meta-language-live', { pid: process.ppid }),
    foreignScratch: makeScratch(fixture, 'meta-language-foreign', { pid: exitedPid(), root: path.join(fixture.base, 'elsewhere') }),
    unmarkedScratch: put(fixture.tmpRoot, 'meta-language-unmarked/file', 4 * KIB),
  };
  return { caches, kept };
}

/** Links `link` to `target`, or returns false where symlinks are not permitted. */
export function trySymlink(target, link) {
  try {
    mkdirSync(path.dirname(link), { recursive: true });
    symlinkSync(target, link, 'dir');
    return true;
  } catch {
    return false;
  }
}

/** Status of tracked and untracked, non-ignored files. */
export function worktreeState(root) {
  return git(root, ['status', '--porcelain=v1', '-z']);
}

export function readJson(file) {
  return JSON.parse(readFileSync(file, 'utf8'));
}

/** Runs a repository script with node and returns the spawnSync result. */
export function runScript(script, args, { cwd, env = process.env, timeout = 60000 } = {}) {
  return spawnSync(process.execPath, [script, ...args], { cwd, env, encoding: 'utf8', timeout });
}

/** A directory whose only executables are links to the named tools (and node). */
export function toolDirectory(base, tools) {
  const bin = path.join(base, `bin-${tools.join('-') || 'none'}`);
  mkdirSync(bin, { recursive: true });
  const which = (tool) => spawnSync('sh', ['-c', `command -v ${tool}`], { encoding: 'utf8' }).stdout.trim();
  for (const tool of ['node', ...tools]) {
    const source = tool === 'node' ? process.execPath : which(tool);
    if (source && !existsSync(path.join(bin, tool))) symlinkSync(source, path.join(bin, tool));
  }
  return bin;
}
