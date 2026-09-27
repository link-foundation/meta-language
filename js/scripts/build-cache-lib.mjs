/**
 * Registry, discovery and pruning of every regenerable build cache this repository creates.
 *
 * Safety model (see docs/build-cache.md):
 *
 * - Repository scoped. Only paths that git reports as *ignored and untracked* under the
 *   repository root are candidates, so tracked sources, vendored grammars, fixtures,
 *   proof/corpus data and untracked (uncommitted) work are never deleted. Without git the
 *   cleaner only acts inside Cargo target directories that carry Cargo's CACHEDIR.TAG.
 * - Symlinks are never followed and every path is re-validated (realpath containment,
 *   no symlinked parents) immediately before it is removed.
 * - Nested repositories and registered worktrees are never entered.
 * - Active builds are left alone: entries touched after the activity threshold, Cargo
 *   profiles whose `.cargo-lock` is held, and anything newer than a live foreign build
 *   lease are skipped.
 * - Only one cleanup runs per repository at a time; a busy lock makes a cleanup skip.
 * - Global Cargo/npm caches, unrelated Docker resources and volumes are never touched;
 *   containers are limited to the project label and the project BuildKit builder.
 */
import { spawnSync } from 'node:child_process';
import { randomBytes } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

export const DEFAULTS = Object.freeze({
  budgetMb: 4096,
  minFreeMb: 2048,
  activeSeconds: 120,
  staleHours: 48,
  lockStaleMinutes: 60,
});

export const CONTAINER_LABEL = 'io.github.link-foundation.meta-language.cache=owned';
export const CONTAINER_BUILDER = 'meta-language-cache';

const MB = 1024 * 1024;
const CACHEDIR_SIGNATURE = 'Signature: 8a477f597d28d172789f06886806bc55';
const STATE_DIRECTORY = 'meta-language-build-cache';

/**
 * Every cache class the cleaner owns. `policy` is `disposable` (removed by every prune
 * unless active), `warm` (kept within the aggregate budget, least recently used and
 * lowest `priority` evicted first) or `full` (only removed by an explicit full clean).
 */
export const CACHE_CLASSES = Object.freeze([
  { id: 'rust-build', policy: 'warm', priority: 5,
    description: 'Cargo compilation units (deps, build-script binaries, fingerprints) of every debug, release, test and custom profile' },
  { id: 'rust-build-script-output', policy: 'warm', priority: 5,
    description: 'Generated parser/compiler intermediates written by build scripts into build/*/out, such as the compiled tree-sitter grammars' },
  { id: 'rust-incremental', policy: 'disposable', priority: 0,
    description: 'Incremental compilation state of every Cargo profile' },
  { id: 'rust-linked-examples', policy: 'disposable', priority: 0,
    description: 'Linked example binaries and their fingerprints' },
  { id: 'rust-coverage', policy: 'warm', priority: 1,
    description: 'The instrumented cargo-llvm-cov target tree' },
  { id: 'coverage-data', policy: 'disposable', priority: 0,
    description: 'Raw and merged coverage profiles (*.profraw, *.profdata), lcov reports, coverage/ and .nyc_output' },
  { id: 'rust-package', policy: 'disposable', priority: 0,
    description: 'target/package trees left by cargo package and cargo publish verification builds' },
  { id: 'rust-target-scratch', policy: 'disposable', priority: 0,
    description: 'target/tmp scratch written by integration tests' },
  { id: 'rust-mutants', policy: 'disposable', priority: 0,
    description: 'cargo-mutants mutants.out trees' },
  { id: 'bench-output', policy: 'warm', priority: 2,
    description: 'Criterion benchmark baselines and reports' },
  { id: 'docs-build', policy: 'warm', priority: 3,
    description: 'Generated documentation and site output: target/doc, _site and rust/web/pkg' },
  { id: 'proof-build', policy: 'warm', priority: 3,
    description: 'Lean (.lake, *.olean, *.ilean) and Rocq (*.vo, *.vok, *.vos, *.glob, .*.aux, .lia.cache) build output' },
  { id: 'js-dependencies', policy: 'full', priority: 9,
    description: 'Installed JavaScript dependencies of the project package (npm ci restores them)' },
  { id: 'js-test-cache', policy: 'disposable', priority: 0,
    description: 'JavaScript tool caches (node_modules/.cache, .eslintcache, *.tsbuildinfo)' },
  { id: 'package-scratch', policy: 'disposable', priority: 0,
    description: 'Packed npm tarballs and crates, and nested Cargo target and node_modules trees inside scratch directories (temporary clones and package builds)' },
  { id: 'acceptance-scratch', policy: 'disposable', priority: 0,
    description: 'Issue #195 acceptance scratch: clean package consumers, candidate packs, crate package target and temporary downstream clones' },
  { id: 'container', policy: 'warm', priority: 4,
    description: `Docker images labelled ${CONTAINER_LABEL} and the ${CONTAINER_BUILDER} BuildKit builder cache` },
]);

const CLASS_BY_ID = new Map(CACHE_CLASSES.map((klass) => [klass.id, klass]));

/**
 * Ignored paths that are deliberately *not* build caches. Anything ignored that is
 * neither claimed by a class nor listed here is reported as unclaimed, so a new cache
 * category cannot silently escape cleanup.
 */
export const NOT_CACHE_PATTERNS = Object.freeze([
  /^ci-logs(\/|$)/u,
  /(^|\/)[^/]+\.log$/u,
  /(^|\/)logs\/$/u,
  /(^|\/)\.env(\.local)?$/u,
  /(^|\/)[^/]+\.local$/u,
  /(^|\/)\.?venv\/$/u,
  /(^|\/)__pycache__\/$/u,
  /\.py[co]$/u,
  /(^|\/)\.(idea|vscode)\/$/u,
  /\.sw[po]$/u,
  /~$/u,
  /(^|\/)(\.DS_Store|Thumbs\.db)$/u,
  /(^|\/)[^/]+\.rs\.bk$/u,
  /\.pdb$/u,
  /^\.playwright-mcp\/$/u,
  /^landing-(hero|demo)\.png$/u,
  /^dev\/log\/issues\/[^/]+\/pulls\/[^/]+\/templates\/$/u,
  // Acceptance evidence: reports, logs, artifacts and inputs the gate still reads.
  /^issue-195-(results|artifacts|candidates|platform|platform-reports)(\/|$)/u,
  /^tarpaulin-report\.html$/u,
]);

const ACCEPTANCE_SCRATCH = [
  /^issue-195-results\/work\/(consumer|candidates|crate-package|relative-meta-logic[^/]*)$/u,
  /^\.issue-195-work$/u,
];
const PROOF_FILE = /(\.(olean|ilean|vo|vok|vos|glob)$)|(^\..+\.aux$)|(^\.(lia|nia)\.cache$)/u;
const COVERAGE_FILE = /(\.(profraw|profdata|lcov)$)|(^lcov\.info$)/u;
const JS_CACHE_FILE = /(^\.eslintcache$)|(\.tsbuildinfo$)/u;
const PACKED_FILE = /\.(tgz|crate)$/u;
const UNIT_DIRECTORY = /^(.+)-([0-9a-f]{16})$/u;
const UNIT_FILE = /-([0-9a-f]{16})(\.[^/]*)?$/u;

export function classById(id) {
  return CLASS_BY_ID.get(id);
}

export function environmentNumber(env, name, fallback) {
  const raw = env[name];
  if (raw === undefined || raw === '') return fallback;
  const value = Number(raw);
  if (!Number.isFinite(value) || value < 0) throw new Error(`${name} must be a non-negative number, got ${JSON.stringify(raw)}`);
  return value;
}

function toPosix(relative) {
  return relative.split(path.sep).join('/');
}

function fromPosix(root, relative) {
  return path.join(root, ...relative.split('/').filter(Boolean));
}

function within(parent, child) {
  const relative = path.relative(parent, child);
  return relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative));
}

function runGit(root, args) {
  const result = spawnSync('git', ['-C', root, ...args], { encoding: 'buffer', maxBuffer: 256 * MB });
  if (result.error || result.status !== 0) return null;
  return result.stdout.toString('utf8');
}

/** The git worktree containing `root`, or null outside git or when git is missing. */
export function gitInfo(root) {
  const output = runGit(root, ['rev-parse', '--show-toplevel', '--absolute-git-dir', '--git-common-dir']);
  if (output === null) return null;
  const [toplevel, gitDir, commonDir] = output.trim().split(/\r?\n/u);
  return {
    toplevel: fs.realpathSync(toplevel),
    gitDir: fs.realpathSync(gitDir),
    commonDir: fs.realpathSync(path.resolve(root, commonDir)),
  };
}

/** Ignored, untracked paths below `root`; directories end with `/`. */
function ignoredPaths(root) {
  const output = runGit(root, ['ls-files', '-z', '--others', '--ignored', '--exclude-standard', '--directory', '--no-empty-directory']);
  if (output === null) throw new Error(`git could not list ignored paths under ${root}`);
  return output.split('\0').filter(Boolean);
}

function worktreePaths(root) {
  const output = runGit(root, ['worktree', 'list', '--porcelain']) ?? '';
  return output.split(/\r?\n/u)
    .filter((line) => line.startsWith('worktree '))
    .map((line) => {
      try {
        return fs.realpathSync(line.slice('worktree '.length));
      } catch {
        return null;
      }
    })
    .filter(Boolean);
}

/**
 * Builds the cleanup context for `root`, which must be the top of its git worktree
 * (or a plain directory, in which case only Cargo target directories are handled).
 * Extra `targetDirs` outside the root must carry Cargo's CACHEDIR.TAG.
 */
export function createContext({
  root,
  env = process.env,
  now = Date.now(),
  targetDirs = [],
  log = () => {},
} = {}) {
  if (!root) throw new Error('a repository root is required');
  const realRoot = fs.realpathSync(root);
  const git = gitInfo(realRoot);
  if (git && git.toplevel !== realRoot) {
    throw new Error(`${realRoot} is inside the git worktree ${git.toplevel}; pass the worktree root`);
  }
  const extraTargets = [];
  for (const candidate of targetDirs) {
    if (candidate.split(/[\\/]/u).includes('..')) throw new Error(`target directory ${candidate} contains a parent traversal`);
    const absolute = path.resolve(realRoot, candidate);
    let real;
    try {
      real = fs.realpathSync(absolute);
    } catch {
      log(`build-cache: custom target directory ${absolute} does not exist; nothing to clean there`);
      continue;
    }
    if (!isCargoTarget(real)) throw new Error(`${real} is not a Cargo target directory (no Cargo CACHEDIR.TAG)`);
    if (real === path.parse(real).root || real === realRoot) throw new Error(`refusing to use ${real} as a target directory`);
    extraTargets.push(real);
  }
  return {
    root: realRoot,
    git,
    env,
    now,
    log,
    extraTargets,
    activeMs: environmentNumber(env, 'META_LANGUAGE_CACHE_ACTIVE_SECONDS', DEFAULTS.activeSeconds) * 1000,
    staleMs: environmentNumber(env, 'META_LANGUAGE_CACHE_STALE_HOURS', DEFAULTS.staleHours) * 3600 * 1000,
    budgetBytes: environmentNumber(env, 'META_LANGUAGE_CACHE_BUDGET_MB', DEFAULTS.budgetMb) * MB,
    stateDirectory: git ? path.join(git.commonDir, STATE_DIRECTORY) : null,
    worktrees: git ? worktreePaths(realRoot).filter((tree) => tree !== realRoot && within(realRoot, tree)) : [],
  };
}

function isCargoTarget(directory) {
  try {
    return fs.readFileSync(path.join(directory, 'CACHEDIR.TAG'), 'utf8').startsWith(CACHEDIR_SIGNATURE);
  } catch {
    return false;
  }
}

function readDirectory(directory) {
  try {
    return fs.readdirSync(directory, { withFileTypes: true });
  } catch {
    return [];
  }
}

/**
 * Disk usage of a path without following symlinks: bytes (hardlinked inodes counted
 * once per `seen` set), the newest modification time and the latest use time.
 */
export function measure(target, seen = new Set(), exclude = null) {
  const totals = { bytes: 0, files: 0, newestMs: 0, lastUsedMs: 0 };
  const visit = (current) => {
    let stat;
    try {
      stat = fs.lstatSync(current);
    } catch {
      return;
    }
    totals.newestMs = Math.max(totals.newestMs, stat.mtimeMs);
    totals.lastUsedMs = Math.max(totals.lastUsedMs, stat.mtimeMs, stat.atimeMs);
    if (stat.isDirectory()) {
      for (const child of readDirectory(current)) {
        const next = path.join(current, child.name);
        if (exclude && exclude(next)) continue;
        visit(next);
      }
      return;
    }
    const key = `${stat.dev}:${stat.ino}`;
    if (stat.nlink > 1 && seen.has(key)) return;
    seen.add(key);
    totals.files += 1;
    totals.bytes += typeof stat.blocks === 'number' && stat.blocks > 0 ? stat.blocks * 512 : stat.size;
  };
  visit(target);
  return totals;
}

/** Inodes of files currently locked with flock (Linux only); cargo holds `.cargo-lock` while building. */
function heldLockInodes() {
  try {
    const inodes = new Set();
    for (const line of fs.readFileSync('/proc/locks', 'utf8').split('\n')) {
      const device = line.trim().split(/\s+/u)[5];
      if (device) inodes.add(device.split(':').at(-1));
    }
    return inodes;
  } catch {
    return new Set();
  }
}

function lockHeld(lockFile, inodes) {
  try {
    return inodes.has(String(fs.statSync(lockFile).ino));
  } catch {
    return false;
  }
}

function parseFingerprint(file) {
  try {
    const value = JSON.parse(fs.readFileSync(file, 'utf8'));
    return {
      rustc: String(value.rustc ?? ''),
      features: String(value.features ?? ''),
      target: String(value.target ?? ''),
      profile: String(value.profile ?? ''),
      path: String(value.path ?? ''),
    };
  } catch {
    return null;
  }
}

/**
 * Discovers every cache entry under the context root (plus custom target directories).
 * Returns `{ entries, targets, unclaimed, notes }`.
 */
export function discover(ctx) {
  const entries = [];
  const targets = [];
  const unclaimed = new Set();
  const notes = [];
  const lockInodes = heldLockInodes();
  const add = (klass, absolute, extra = {}) => entries.push({ class: klass, path: absolute, paths: [absolute], ...extra });

  const scanFile = (absolute, relative, name, scratch, owner) => {
    if (PROOF_FILE.test(name)) return add('proof-build', absolute);
    if (COVERAGE_FILE.test(name)) return add('coverage-data', absolute);
    if (JS_CACHE_FILE.test(name)) return add('js-test-cache', absolute);
    if (PACKED_FILE.test(name) && (scratch || /^js\/[^/]+\.tgz$/u.test(relative))) return add('package-scratch', absolute);
    if (!scratch && !NOT_CACHE_PATTERNS.some((pattern) => pattern.test(relative))) unclaimed.add(owner);
    return undefined;
  };

  const classifyDirectory = (absolute, relative, name, scratch) => {
    if (ACCEPTANCE_SCRATCH.some((pattern) => pattern.test(relative))) return add('acceptance-scratch', absolute);
    if (name === 'node_modules') {
      if (scratch) return add('package-scratch', absolute);
      const cache = path.join(absolute, '.cache');
      if (fs.existsSync(cache)) add('js-test-cache', cache);
      return add('js-dependencies', absolute, { exclude: (next) => next === cache });
    }
    if (name === '.lake') return add('proof-build', absolute);
    if (name === 'coverage' || name === '.nyc_output') return add('coverage-data', absolute);
    if (/^mutants\.out/u.test(name)) return add('rust-mutants', absolute);
    if (name === 'criterion') return add('bench-output', absolute);
    if (relative === '_site' || relative === 'rust/web/pkg' || name === 'doc') return add('docs-build', absolute);
    return null;
  };

  // `owner` is the ignored path an unclaimed file is reported under; `scratch` marks
  // temporary trees, whose nested Cargo targets and node_modules are disposable.
  const scan = (absolute, relative, scratch, owner) => {
    if (ctx.worktrees.some((tree) => within(absolute, tree) || within(tree, absolute))) {
      notes.push(`skipped ${relative}: contains another worktree`);
      return;
    }
    const children = readDirectory(absolute);
    const names = new Set(children.map((child) => child.name));
    if (names.has('.git')) {
      notes.push(`skipped ${relative}: nested repository`);
      return;
    }
    if (names.has('CACHEDIR.TAG') && isCargoTarget(absolute)) {
      targets.push({ path: absolute, relative, scratch: scratch || relative !== owner });
      return;
    }
    for (const child of children) {
      if (child.isSymbolicLink()) continue;
      const childAbsolute = path.join(absolute, child.name);
      const childRelative = `${relative}/${child.name}`;
      if (child.isDirectory()) {
        if (classifyDirectory(childAbsolute, childRelative, child.name, scratch)) continue;
        scan(childAbsolute, childRelative, scratch, owner);
      } else if (child.isFile()) {
        scanFile(childAbsolute, childRelative, child.name, scratch, owner);
      }
    }
  };

  if (ctx.git) {
    for (const item of ignoredPaths(ctx.root)) {
      const directory = item.endsWith('/');
      const relative = directory ? item.slice(0, -1) : item;
      const absolute = fromPosix(ctx.root, relative);
      let stat;
      try {
        stat = fs.lstatSync(absolute);
      } catch {
        continue;
      }
      if (stat.isSymbolicLink()) {
        notes.push(`skipped ${relative}: symbolic link`);
        continue;
      }
      const name = path.basename(absolute);
      if (stat.isDirectory()) {
        if (classifyDirectory(absolute, relative, name, false)) continue;
        if (relative === 'issue-195-results') {
          // Reports, logs and artifacts are evidence the gate reads; only work/ holds scratch.
          const work = path.join(absolute, 'work');
          if (fs.existsSync(work)) scan(work, `${relative}/work`, true, `${relative}/work`);
          continue;
        }
        if (NOT_CACHE_PATTERNS.some((pattern) => pattern.test(`${relative}/`))) continue;
        scan(absolute, relative, false, relative);
      } else if (stat.isFile()) {
        scanFile(absolute, relative, name, false, relative);
      }
    }
  } else {
    notes.push(`${ctx.root} is not a git worktree: only Cargo target directories carrying CACHEDIR.TAG are cleaned`);
    const findTargets = (absolute, relative, depth) => {
      if (depth > 3) return;
      for (const child of readDirectory(absolute)) {
        if (!child.isDirectory() || child.isSymbolicLink() || child.name === 'node_modules' || child.name === '.git') continue;
        const childAbsolute = path.join(absolute, child.name);
        const childRelative = relative ? `${relative}/${child.name}` : child.name;
        if (isCargoTarget(childAbsolute)) targets.push({ path: childAbsolute, relative: childRelative, scratch: false });
        else findTargets(childAbsolute, childRelative, depth + 1);
      }
    };
    findTargets(ctx.root, '', 0);
  }
  for (const extra of ctx.extraTargets) {
    if (!targets.some((target) => target.path === extra)) {
      targets.push({ path: extra, relative: extra, scratch: false, external: true });
    }
  }
  for (const target of targets) cargoTargetEntries(target, entries, add, lockInodes, ctx, unclaimed);
  return { entries, targets, unclaimed: [...unclaimed].sort(), notes };
}

function cargoTargetEntries(target, entries, add, lockInodes, ctx, unclaimed, coverage = false) {
  if (target.scratch) {
    add('package-scratch', target.path, { target: target.path });
    return;
  }
  for (const child of readDirectory(target.path)) {
    if (child.isSymbolicLink()) continue;
    const absolute = path.join(target.path, child.name);
    if (child.isFile()) {
      if (COVERAGE_FILE.test(child.name)) add('coverage-data', absolute, { target: target.path });
      continue;
    }
    if (!child.isDirectory()) continue;
    if (child.name === 'package') add('rust-package', absolute, { target: target.path });
    else if (child.name === 'tmp') add('rust-target-scratch', absolute, { target: target.path });
    else if (child.name === 'doc') add('docs-build', absolute, { target: target.path });
    else if (child.name === 'criterion') add('bench-output', absolute, { target: target.path });
    else if (isCargoTarget(absolute)) {
      cargoTargetEntries({ path: absolute, scratch: false }, entries, add, lockInodes, ctx, unclaimed,
        coverage || child.name === 'llvm-cov-target');
    } else if (fs.existsSync(path.join(absolute, '.fingerprint'))) {
      profileEntries(absolute, target.path, coverage, entries, lockInodes, ctx);
    } else {
      let profiles = 0;
      for (const grandchild of readDirectory(absolute)) {
        const nested = path.join(absolute, grandchild.name);
        if (grandchild.isDirectory() && !grandchild.isSymbolicLink() && fs.existsSync(path.join(nested, '.fingerprint'))) {
          profileEntries(nested, target.path, coverage, entries, lockInodes, ctx);
          profiles += 1;
        }
      }
      if (profiles === 0) add(coverage ? 'rust-coverage' : 'rust-build', absolute, { target: target.path });
    }
  }
}

function profileEntries(profile, target, coverage, entries, lockInodes, ctx) {
  const baseClass = coverage ? 'rust-coverage' : 'rust-build';
  const active = lockHeld(path.join(profile, '.cargo-lock'), lockInodes);
  const push = (klass, paths, extra = {}) => entries.push({
    class: klass,
    path: paths[0],
    paths,
    target,
    profile,
    profileLocked: active,
    ...extra,
  });
  for (const special of ['incremental', 'examples']) {
    const absolute = path.join(profile, special);
    if (fs.existsSync(absolute)) push(special === 'incremental' ? 'rust-incremental' : 'rust-linked-examples', [absolute]);
  }
  const depsByHash = new Map();
  for (const file of readDirectory(path.join(profile, 'deps'))) {
    const match = UNIT_FILE.exec(file.name);
    if (!match || file.isSymbolicLink()) continue;
    const list = depsByHash.get(match[1]) ?? [];
    list.push(path.join(profile, 'deps', file.name));
    depsByHash.set(match[1], list);
  }
  const units = [];
  for (const directory of readDirectory(path.join(profile, '.fingerprint'))) {
    const match = UNIT_DIRECTORY.exec(directory.name);
    if (!match || !directory.isDirectory() || directory.isSymbolicLink()) continue;
    const fingerprint = path.join(profile, '.fingerprint', directory.name);
    const jsonName = readDirectory(fingerprint).map((file) => file.name).find((name) => name.endsWith('.json')) ?? '';
    const kind = jsonName.slice(0, -'.json'.length);
    const info = jsonName ? parseFingerprint(path.join(fingerprint, jsonName)) : null;
    const build = path.join(profile, 'build', directory.name);
    const paths = [fingerprint, ...(fs.existsSync(build) ? [build] : []), ...(depsByHash.get(match[2]) ?? [])];
    let klass = baseClass;
    if (/^(test-)?example-/u.test(kind)) klass = 'rust-linked-examples';
    else if (kind.startsWith('run-build-script')) klass = coverage ? baseClass : 'rust-build-script-output';
    units.push({ klass, paths, name: match[1], kind, info });
  }
  // A unit is stale when it was built by another toolchain than the newest unit of the
  // profile, or when a newer unit of the same package, kind, features, target and
  // profile was built from a different source path (a superseded package version).
  const measured = units.map((unit) => ({ ...unit, usage: measure(unit.paths[0]) }));
  const newest = measured.reduce((best, unit) => (unit.info && (!best || unit.usage.newestMs > best.usage.newestMs) ? unit : best), null);
  const groups = new Map();
  for (const unit of measured) {
    if (!unit.info) continue;
    const key = [unit.name, unit.kind, unit.info.features, unit.info.target, unit.info.profile].join('\0');
    const group = groups.get(key) ?? [];
    group.push(unit);
    groups.set(key, group);
  }
  for (const unit of measured) {
    let stale = null;
    if (unit.info && newest && unit.info.rustc !== newest.info.rustc) stale = 'built by another toolchain';
    else if (unit.info && unit.info.path !== '0') {
      const key = [unit.name, unit.kind, unit.info.features, unit.info.target, unit.info.profile].join('\0');
      const superseded = groups.get(key).some((other) =>
        other !== unit && other.info.path !== '0' && other.info.path !== unit.info.path && other.usage.newestMs > unit.usage.newestMs);
      if (superseded) stale = 'superseded package version';
    }
    push(unit.klass, unit.paths, { stale, unit: unit.name, lastUsedHint: unit.usage.lastUsedMs });
  }
  // Uplifted outputs, dep-info and lock files at the profile root are recreated by Cargo.
  push(baseClass, [profile], { profileRoot: true });
}

/** Measures every entry once; hardlinked inodes are attributed to the first entry. */
export function measureEntries(entries) {
  const seen = new Set();
  for (const entry of entries) {
    if (entry.profileRoot) continue;
    const usage = { bytes: 0, files: 0, newestMs: 0, lastUsedMs: 0 };
    for (const item of entry.paths) {
      const part = measure(item, seen, entry.exclude);
      usage.bytes += part.bytes;
      usage.files += part.files;
      usage.newestMs = Math.max(usage.newestMs, part.newestMs);
      usage.lastUsedMs = Math.max(usage.lastUsedMs, part.lastUsedMs);
    }
    Object.assign(entry, usage);
  }
  for (const entry of entries.filter((item) => item.profileRoot)) {
    const usage = measure(entry.path, seen, (next) => ['.fingerprint', 'deps', 'build', 'incremental', 'examples']
      .some((name) => next === path.join(entry.path, name)));
    Object.assign(entry, usage);
  }
  return entries;
}

export function summarize(entries) {
  const byClass = Object.fromEntries(CACHE_CLASSES.map((klass) => [klass.id, { bytes: 0, entries: 0 }]));
  for (const entry of entries) {
    byClass[entry.class].bytes += entry.bytes ?? 0;
    byClass[entry.class].entries += 1;
  }
  const total = Object.values(byClass).reduce((sum, item) => sum + item.bytes, 0);
  const budgeted = Object.entries(byClass)
    .filter(([id]) => classById(id).policy !== 'full')
    .reduce((sum, [, item]) => sum + item.bytes, 0);
  return { total, budgeted, byClass };
}

function processAlive(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code === 'EPERM';
  }
}

function readJson(file) {
  try {
    return JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch {
    return null;
  }
}

/** Live build leases of other processes; dead leases are removed. */
export function liveForeignLeases(ctx, ownLease = null) {
  if (!ctx.stateDirectory) return [];
  const directory = path.join(ctx.stateDirectory, 'leases');
  const leases = [];
  for (const file of readDirectory(directory)) {
    const leasePath = path.join(directory, file.name);
    const lease = readJson(leasePath);
    if (!lease || !processAlive(lease.pid)) {
      fs.rmSync(leasePath, { force: true });
      continue;
    }
    if (leasePath !== ownLease) leases.push(lease);
  }
  return leases;
}

/** Registers a build lease that makes concurrent cleanups leave newer outputs alone. */
export function acquireLease(ctx, command) {
  if (!ctx.stateDirectory) return { path: null, release() {} };
  const directory = path.join(ctx.stateDirectory, 'leases');
  fs.mkdirSync(directory, { recursive: true });
  const leasePath = path.join(directory, `${process.pid}-${Date.now()}-${randomBytes(4).toString('hex')}.json`);
  fs.writeFileSync(leasePath, JSON.stringify({ pid: process.pid, root: ctx.root, startedAt: Date.now(), command }));
  return {
    path: leasePath,
    release() {
      fs.rmSync(leasePath, { force: true });
    },
  };
}

/** Takes the repository cleanup lock, or returns null when another live cleanup holds it. */
export function acquireLock(ctx) {
  if (!ctx.stateDirectory) return { release() {} };
  fs.mkdirSync(ctx.stateDirectory, { recursive: true });
  const lock = path.join(ctx.stateDirectory, 'lock');
  for (let attempt = 0; attempt < 2; attempt += 1) {
    try {
      fs.mkdirSync(lock);
      fs.writeFileSync(path.join(lock, 'owner.json'), JSON.stringify({ pid: process.pid, startedAt: Date.now() }));
      return {
        release() {
          fs.rmSync(lock, { recursive: true, force: true });
        },
      };
    } catch (error) {
      if (error.code !== 'EEXIST') throw error;
      const owner = readJson(path.join(lock, 'owner.json'));
      const age = owner ? Date.now() - owner.startedAt : Infinity;
      const staleAfter = environmentNumber(ctx.env, 'META_LANGUAGE_CACHE_LOCK_STALE_MINUTES', DEFAULTS.lockStaleMinutes) * 60000;
      if (owner && processAlive(owner.pid) && age < staleAfter) return null;
      if (!owner) {
        // A lock directory without an owner is only abandoned once it is old.
        let created = Date.now();
        try {
          created = fs.statSync(lock).mtimeMs;
        } catch {
          continue;
        }
        if (Date.now() - created < 10000) return null;
      }
      fs.rmSync(lock, { recursive: true, force: true });
    }
  }
  return null;
}

/**
 * Validates a path immediately before deletion: it must be inside the root (or a
 * registered custom target directory), must not be a symlink, must not have a
 * symlinked parent, and must still be git-ignored.
 */
function assertRemovable(ctx, absolute, ignoredCheck) {
  const allowedRoots = [ctx.root, ...ctx.extraTargets];
  const parent = fs.realpathSync(path.dirname(absolute));
  const resolved = path.join(parent, path.basename(absolute));
  if (resolved !== absolute) throw new Error(`${absolute} resolves through a symbolic link to ${resolved}`);
  const container = allowedRoots.find((allowed) => within(allowed, resolved) && resolved !== allowed);
  if (!container) throw new Error(`${absolute} escapes the repository root ${ctx.root}`);
  if (ctx.worktrees.some((tree) => within(resolved, tree) || within(tree, resolved))) {
    throw new Error(`${absolute} overlaps another worktree`);
  }
  if (container === ctx.root && ctx.git && ignoredCheck && !ignoredCheck(toPosix(path.relative(ctx.root, resolved)))) {
    throw new Error(`${absolute} is not git-ignored`);
  }
}

function makeIgnoredCheck(ctx) {
  if (!ctx.git) return null;
  const ignored = new Set(ignoredPaths(ctx.root).map((item) => item.replace(/\/$/u, '')));
  return (relative) => {
    const parts = relative.split('/');
    for (let index = 1; index <= parts.length; index += 1) {
      if (ignored.has(parts.slice(0, index).join('/'))) return true;
    }
    return false;
  };
}

function removePath(ctx, absolute, ignoredCheck) {
  let stat;
  try {
    stat = fs.lstatSync(absolute);
  } catch {
    return;
  }
  assertRemovable(ctx, absolute, ignoredCheck);
  if (stat.isSymbolicLink()) throw new Error(`${absolute} is a symbolic link`);
  fs.rmSync(absolute, { recursive: true, force: true, maxRetries: 3, retryDelay: 100 });
}

/**
 * Decides and (unless `dryRun`) performs the cleanup. `mode` is `prune` (disposable and
 * stale entries, then least-recently-used warm entries until the budget holds) or `full`
 * (every registered entry). Returns a report with before/after/reclaimed bytes per class.
 */
export function runCleanup(ctx, { mode = 'prune', dryRun = false, ownLease = null, container = true, settled = false } = {}) {
  if (!['prune', 'full'].includes(mode)) throw new Error(`unknown cleanup mode ${mode}`);
  const lock = dryRun ? { release() {} } : acquireLock(ctx);
  if (!lock) {
    return { mode, skipped: 'another cleanup of this repository is running', removed: [], kept: [] };
  }
  try {
    const leases = liveForeignLeases(ctx, ownLease);
    const earliestLease = Math.min(...leases.map((lease) => lease.startedAt));
    const processes = runningBuildProcesses(ctx.env);
    const activeMs = activityWindow(ctx, processes, settled);
    const threshold = Math.min(ctx.now - activeMs, earliestLease - 1000);
    const discovery = discover(ctx);
    const entries = measureEntries(discovery.entries);
    // Any recent write in a Cargo profile protects the whole profile.
    const profileNewest = new Map();
    for (const entry of entries.filter((item) => item.profile)) {
      profileNewest.set(entry.profile, Math.max(profileNewest.get(entry.profile) ?? 0, entry.newestMs));
    }
    for (const entry of entries.filter((item) => item.profile)) entry.activityMs = profileNewest.get(entry.profile);
    const before = summarize(entries);
    const decisions = entries.map((entry) => decide(entry, mode, threshold, earliestLease, ctx));
    if (mode === 'prune') {
      // Enforce the aggregate budget: stale entries first, then lower-priority classes,
      // then least recently used entries, until the retained caches fit.
      let retained = decisions
        .filter(({ entry, action }) => action === 'keep' && classById(entry.class).policy !== 'full')
        .reduce((sum, { entry }) => sum + entry.bytes, 0);
      const evictable = decisions
        .filter(({ entry, action, protectedBy }) => action === 'keep' && !protectedBy && !entry.profileRoot &&
          classById(entry.class).policy === 'warm')
        .sort((left, right) =>
          Number(Boolean(right.entry.stale)) - Number(Boolean(left.entry.stale)) ||
          classById(left.entry.class).priority - classById(right.entry.class).priority ||
          left.entry.lastUsedMs - right.entry.lastUsedMs);
      for (const decision of evictable) {
        if (retained <= ctx.budgetBytes) break;
        decision.action = 'remove';
        decision.reason = `${decision.entry.stale ?? 'least recently used'}; over budget`;
        retained -= decision.entry.bytes;
      }
    }
    const errors = [];
    if (!dryRun) {
      const ignoredCheck = makeIgnoredCheck(ctx);
      // Units are removed fingerprint first, so an interrupted cleanup leaves Cargo
      // with a missing fingerprint (a rebuild) rather than a fresh-looking unit whose
      // artifacts are gone. Examples also drop the fingerprints of their units.
      for (const decision of decisions.filter(({ action }) => action === 'remove')) {
        try {
          const paths = decision.entry.profileRoot && mode === 'full' ? [decision.entry.path] : decision.entry.paths;
          for (const item of paths) removePath(ctx, item, ignoredCheck);
        } catch (error) {
          decision.action = 'keep';
          decision.reason = `not removed: ${error.message}`;
          errors.push(error.message);
        }
      }
      if (mode === 'full') {
        for (const target of discovery.targets) {
          const blocked = decisions.some(({ entry, action }) => entry.target === target.path && action === 'keep');
          if (!blocked && !target.external) {
            try {
              removePath(ctx, target.path, ignoredCheck);
            } catch (error) {
              errors.push(error.message);
            }
          }
        }
      }
    }
    const after = dryRun ? null : summarize(measureEntries(discover(ctx).entries));
    const report = {
      mode,
      dryRun,
      root: ctx.root,
      budgetBytes: ctx.budgetBytes,
      leases: leases.length,
      buildProcesses: processes === null ? null : processes.length,
      activeSeconds: activeMs / 1000,
      before,
      after,
      reclaimedBytes: after ? Math.max(0, before.total - after.total) : decisions
        .filter(({ action }) => action === 'remove')
        .reduce((sum, { entry }) => sum + entry.bytes, 0),
      removed: decisions.filter(({ action }) => action === 'remove').map((decision) => describe(ctx, decision)),
      kept: decisions.filter(({ action }) => action === 'keep').map((decision) => describe(ctx, decision)),
      unclaimed: discovery.unclaimed,
      notes: discovery.notes,
      errors,
      container: container ? containerCleanup(ctx, mode, dryRun) : { skipped: 'disabled' },
    };
    report.overBudget = after ? after.budgeted > ctx.budgetBytes : false;
    return report;
  } finally {
    lock.release();
  }
}

const BUILD_PROCESS = /^(cargo|rustc|rustdoc|clippy-driver|cargo-llvm-cov|lake|lean|rocq|coqc)(\.exe)?$/iu;

/**
 * Running Cargo, rustc, Lean or Rocq processes on this machine, or null when they
 * cannot be listed. `META_LANGUAGE_CACHE_PROCESS_LIST` replaces the listing command
 * output (one process name per line) for tests.
 */
export function runningBuildProcesses(env = process.env) {
  let output = env.META_LANGUAGE_CACHE_PROCESS_LIST;
  if (output === undefined) {
    const result = process.platform === 'win32'
      ? spawnSync('tasklist', ['/fo', 'csv', '/nh'], { encoding: 'utf8' })
      : spawnSync('ps', ['-A', '-o', 'comm='], { encoding: 'utf8' });
    if (result.error || result.status !== 0) return null;
    output = result.stdout;
  }
  return output.split(/\r?\n/u)
    .map((line) => path.basename(line.trim().replace(/^"([^"]*)".*$/u, '$1')))
    .filter((name) => BUILD_PROCESS.test(name));
}

/**
 * How recently written outputs are treated as part of a running build. While build
 * processes run the window widens to 30 minutes; after a wrapped command finished and
 * no build process remains (`settled`), nothing is considered active.
 */
function activityWindow(ctx, processes, settled) {
  if (processes !== null && processes.length > 0) return Math.max(ctx.activeMs, 30 * 60 * 1000);
  if (settled && processes !== null && ctx.env.META_LANGUAGE_CACHE_ACTIVE_SECONDS === undefined) return 0;
  return ctx.activeMs;
}

function decide(entry, mode, threshold, earliestLease, ctx) {
  const klass = classById(entry.class);
  const keep = (reason, protectedBy = null) => ({ entry, action: 'keep', reason, protectedBy });
  const remove = (reason) => ({ entry, action: 'remove', reason, protectedBy: null });
  if (entry.profileLocked) return keep('Cargo is building this profile', 'active-build');
  const activity = entry.activityMs ?? entry.newestMs;
  if (activity >= threshold) {
    return activity >= earliestLease - 1000
      ? keep('newer than a live build lease', 'build-lease')
      : keep('modified within the activity window', 'active-build');
  }
  if (mode === 'full') return remove('full clean');
  if (entry.profileRoot) return keep('profile root outputs');
  if (klass.policy === 'full') return keep('only removed by a full clean');
  if (klass.policy === 'disposable') return remove('disposable');
  if (entry.stale && ctx.now - (entry.lastUsedMs ?? 0) > ctx.staleMs) return remove(entry.stale);
  return keep('warm cache within budget');
}

function describe(ctx, { entry, action, reason }) {
  const relative = path.relative(ctx.root, entry.path);
  return {
    class: entry.class,
    path: within(ctx.root, entry.path) ? toPosix(relative) : entry.path,
    bytes: entry.bytes ?? 0,
    action,
    reason,
    stale: entry.stale ?? null,
  };
}

function docker(env, args) {
  const binary = env.META_LANGUAGE_DOCKER ?? 'docker';
  return spawnSync(binary, args, { encoding: 'utf8', env });
}

/**
 * Removes project-owned container caches only: images carrying the project label and the
 * project BuildKit builder. Never runs system/volume prunes or touches other resources.
 */
export function containerCleanup(ctx, mode, dryRun) {
  const probe = docker(ctx.env, ['version', '--format', '{{.Server.Version}}']);
  if (probe.error || probe.status !== 0) return { skipped: 'docker is not available' };
  const commands = [];
  const images = docker(ctx.env, ['image', 'ls', '--quiet', '--filter', `label=${CONTAINER_LABEL}`]);
  const imageIds = images.status === 0 ? [...new Set(images.stdout.split(/\s+/u).filter(Boolean))] : [];
  if (mode === 'full' && imageIds.length > 0) commands.push(['image', 'rm', '--force', ...imageIds]);
  if (mode === 'prune') commands.push(['image', 'prune', '--force', '--filter', `label=${CONTAINER_LABEL}`]);
  const builder = docker(ctx.env, ['buildx', 'inspect', CONTAINER_BUILDER]);
  if (builder.status === 0) {
    commands.push(mode === 'full'
      ? ['buildx', 'prune', '--builder', CONTAINER_BUILDER, '--all', '--force']
      : ['buildx', 'prune', '--builder', CONTAINER_BUILDER, '--force', '--filter', 'until=168h']);
  }
  const results = [];
  for (const command of commands) {
    if (dryRun) {
      results.push({ command, status: null });
      continue;
    }
    const result = docker(ctx.env, command);
    results.push({ command, status: result.status });
  }
  return { images: imageIds.length, builder: builder.status === 0, commands: results };
}

/** Free space of the file system holding `directory`, in bytes, or null if unknown. */
export function freeBytes(directory) {
  try {
    const stats = fs.statfsSync(directory);
    return Number(stats.bavail) * Number(stats.bsize);
  } catch {
    return null;
  }
}

export function formatBytes(bytes) {
  if (bytes === null || bytes === undefined) return 'unknown';
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${value.toFixed(unit === 0 ? 0 : 1)} ${units[unit]}`;
}
