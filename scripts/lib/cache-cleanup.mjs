// Engine behind scripts/clean-caches.mjs: discovers the cache classes of
// cache-classes.mjs, keeps only candidates that are provably regenerable, and
// removes them within the disk budget. See docs/cache-cleanup.md.
//
// A candidate is removed only when every check passes:
// - it is not a symlink and its real path is its own path, so nothing escapes
//   through a link;
// - it lies inside this worktree, inside a Cargo target directory the caller
//   named explicitly (recognized by CACHEDIR.TAG), or is a temporary directory
//   this repository marked as scratch and whose creator has exited;
// - git reports it ignored, and no tracked file lies under it, so source,
//   vendored grammars, fixtures, proofs, corpora and uncommitted work stay;
// - it is not evidence (results other than their `work` scratch, artifacts,
//   logs) and holds no other worktree;
// - no build is active on it: no live lease, and no Cargo lock held.
import { execFileSync, spawnSync } from 'node:child_process';
import {
  appendFileSync,
  existsSync,
  linkSync,
  lstatSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  readlinkSync,
  realpathSync,
  rmSync,
  statSync,
  statfsSync,
  writeFileSync,
} from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import {
  BUILDKIT_BUILDER,
  CACHE_CLASSES,
  CONTAINER_LABEL,
  isCargoTargetDirectory,
} from './cache-classes.mjs';

export const DEFAULT_BUDGET_MB = 4096;
export const DEFAULT_MIN_FREE_MB = 2048;
const MB = 1024 * 1024;
const HISTORY_LIMIT = 200;
const DEFAULT_RESULTS_DIRECTORIES = ['issue-195-results'];
const PROTECTED_DIRECTORIES = ['issue-195-artifacts', 'ci-logs', 'logs', 'dev/log', 'docs', 'parity', '.git'];
const CARGO_LOCK_BUSY = 75;

/** Runs git and returns stdout, or null when git is missing or fails. */
function git(cwd, args) {
  try {
    return execFileSync('git', args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] });
  } catch {
    return null;
  }
}

/** Real path, or the resolved path when it does not exist. */
function real(candidate) {
  try {
    return realpathSync(candidate);
  } catch {
    return path.resolve(candidate);
  }
}

const inside = (child, parent) => child === parent || child.startsWith(parent.endsWith(path.sep) ? parent : parent + path.sep);
const strictlyInside = (child, parent) => child !== parent && inside(child, parent);
const toPosix = (relative) => relative.split(path.sep).join('/');

/** Whether a process id belongs to a live process. */
export function processAlive(pid) {
  if (!Number.isInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code === 'EPERM';
  }
}

/** The worktree root and its git directories, or null outside a git worktree. */
export function resolveRepository(cwd) {
  const top = git(cwd, ['rev-parse', '--show-toplevel']);
  const gitDirectory = git(cwd, ['rev-parse', '--absolute-git-dir']);
  if (!top || !gitDirectory) return null;
  const root = real(top.trim());
  const gitDir = real(gitDirectory.trim());
  return { root, gitDir, stateDir: path.join(gitDir, 'meta-language-cache') };
}

/** Rejects user-supplied paths that climb with `..`, before anything resolves them. */
export function assertNoTraversal(value, flag) {
  if (value.split(/[\\/]+/u).includes('..')) {
    throw new UsageError(`${flag} must not contain '..' path segments: ${value}`);
  }
  return value;
}

export class UsageError extends Error {}

// Leases mark active builds, tests and acceptance runs of this worktree.

const leaseDirectory = (stateDir) => path.join(stateDir, 'leases');

/** Records that `pid` is building in this worktree until the lease is released. */
export function acquireLease({ cwd = process.cwd(), label = 'build', pid = process.pid } = {}) {
  const repository = resolveRepository(cwd);
  if (!repository) return { file: null, release() {} };
  const directory = leaseDirectory(repository.stateDir);
  mkdirSync(directory, { recursive: true });
  const file = path.join(directory, `${pid}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}.json`);
  writeFileSync(file, `${JSON.stringify({ pid, label, root: repository.root, createdAt: new Date().toISOString() })}\n`);
  return {
    file,
    release() {
      rmSync(file, { force: true });
    },
  };
}

/** Live leases of other processes; stale leases are removed. */
export function liveLeases(stateDir) {
  const directory = leaseDirectory(stateDir);
  const leases = [];
  let entries = [];
  try {
    entries = readdirSync(directory);
  } catch {
    return leases;
  }
  for (const name of entries) {
    const file = path.join(directory, name);
    let lease;
    try {
      lease = JSON.parse(readFileSync(file, 'utf8'));
    } catch {
      continue;
    }
    if (lease.pid === process.pid) continue;
    if (processAlive(lease.pid)) leases.push(lease);
    else rmSync(file, { force: true });
  }
  return leases;
}

// The cleanup lock serializes concurrent cleanups of one worktree. The lock
// file appears with its holder's pid already in it (a hard link of a written
// temporary file), so a contender never reads a half-written lock, and only
// the contender holding the `.break` guard may remove a stale lock.

const STALE_GUARD_MS = 60_000;

/** Creates `file` holding this pid, or returns false when it already exists. */
function createLockFile(file) {
  const temporary = `${file}.${process.pid}.tmp`;
  writeFileSync(temporary, `${process.pid}\n`);
  try {
    linkSync(temporary, file);
    return true;
  } catch (error) {
    if (error.code === 'EEXIST') return false;
    if (!['EPERM', 'ENOTSUP', 'EOPNOTSUPP', 'ENOSYS'].includes(error.code)) throw error;
    // File systems without hard links fall back to an exclusive create.
    try {
      writeFileSync(file, `${process.pid}\n`, { flag: 'wx' });
      return true;
    } catch (fallbackError) {
      if (fallbackError.code === 'EEXIST') return false;
      throw fallbackError;
    }
  } finally {
    rmSync(temporary, { force: true });
  }
}

/** The pid in the lock file, null while it is unreadable, undefined when it is gone. */
function lockHolder(file) {
  let text;
  try {
    text = readFileSync(file, 'utf8');
  } catch (error) {
    if (error.code === 'ENOENT') return undefined;
    throw error;
  }
  const pid = Number.parseInt(text, 10);
  return Number.isInteger(pid) ? pid : null;
}

const olderThan = (file, milliseconds) => {
  try {
    return Date.now() - statSync(file).mtimeMs > milliseconds;
  } catch {
    return false;
  }
};

/** Removes the lock when it still names the exited `holder`; false when another contender is at it. */
function breakStaleLock(file, holder) {
  const guard = `${file}.break`;
  try {
    mkdirSync(guard);
  } catch (error) {
    if (error.code !== 'EEXIST') throw error;
    // A contender that died while holding the guard leaves it behind.
    if (olderThan(guard, STALE_GUARD_MS)) rmSync(guard, { recursive: true, force: true });
    return false;
  }
  try {
    if (lockHolder(file) === holder) rmSync(file, { force: true });
    return true;
  } finally {
    rmSync(guard, { recursive: true, force: true });
  }
}

function acquireCleanupLock(stateDir) {
  mkdirSync(stateDir, { recursive: true });
  const file = path.join(stateDir, 'cleanup.lock');
  for (let attempt = 0; attempt < 3; attempt += 1) {
    if (createLockFile(file)) return { file, release: () => rmSync(file, { force: true }) };
    const holder = lockHolder(file);
    if (holder === undefined) continue;
    // Only the exclusive-create fallback can leave a lock without a pid; it is stale once old.
    if (holder === null && !olderThan(file, STALE_GUARD_MS)) return { busy: -1 };
    if (holder !== null && holder !== process.pid && processAlive(holder)) return { busy: holder };
    if (!breakStaleLock(file, holder)) return { busy: holder ?? -1 };
  }
  return { busy: -1 };
}

// Cargo holds `<target>/<profile>/.cargo-lock` with flock(2) while it builds.

function flockAvailable() {
  return spawnSync('flock', ['--version'], { stdio: 'ignore' }).status === 0;
}

function cargoLockHeld(lockFile) {
  const probe = spawnSync(
    'flock',
    ['--nonblock', '--conflict-exit-code', String(CARGO_LOCK_BUSY), lockFile, 'true'],
    { stdio: 'ignore' },
  );
  return probe.status === CARGO_LOCK_BUSY;
}

/** Cargo target directories at `directory` or up to `depth` levels below it. */
function nestedCargoTargets(directory, depth) {
  const targets = [];
  const visit = (current, remaining) => {
    if (isCargoTargetDirectory(current)) {
      targets.push(current);
      return;
    }
    if (remaining === 0) return;
    let entries = [];
    try {
      entries = readdirSync(current, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.isDirectory()) visit(path.join(current, entry.name), remaining - 1);
    }
  };
  visit(directory, depth);
  return targets;
}

function busyCargoTarget(target, cache) {
  if (cache.has(target)) return cache.get(target);
  let busy = false;
  const stack = [[target, 2]];
  while (stack.length > 0 && !busy) {
    const [current, remaining] = stack.pop();
    let entries = [];
    try {
      entries = readdirSync(current, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      const child = path.join(current, entry.name);
      if (entry.isFile() && entry.name === '.cargo-lock' && cargoLockHeld(child)) {
        busy = true;
        break;
      }
      if (entry.isDirectory() && remaining > 0) stack.push([child, remaining - 1]);
    }
  }
  cache.set(target, busy);
  return busy;
}

// Cargo releases its lock once compilation ends, while `cargo test`, `cargo
// run` and `cargo bench` still run rustdoc and the binaries it built. So a
// directory is also busy while another process executes a binary inside it or
// names it in its command line, and a Cargo target while a Cargo, rustc or
// rustdoc process runs in its workspace or points CARGO_TARGET_DIR at it.

const CARGO_FAMILY = /^(?:cargo|rustc|rustdoc|clippy-driver|cargo-[\w-]+)(?:\.exe)?$/iu;

const attempt = (read) => {
  try {
    return read();
  } catch {
    return null;
  }
};

// A zombie runs nothing, and a process that exits while it is being read has
// already released its working directory, which Linux reports as ENOENT (a
// process of another user reports EACCES and may still use a cache).
const LIVE_STATE = /^[^ZX]/u;
const processState = (stat) => stat.slice(stat.lastIndexOf(')') + 2);
const EXITING = Symbol('exiting');

function workingDirectory(entry) {
  try {
    return readlinkSync(path.join(entry, 'cwd'));
  } catch (error) {
    return error.code === 'ENOENT' ? EXITING : null;
  }
}

/** Processes read from `procRoot` (`/proc`); the parameter lets tests supply a fake one. */
export function linuxProcesses(procRoot = '/proc') {
  const processes = [];
  for (const name of readdirSync(procRoot)) {
    if (!/^\d+$/u.test(name)) continue;
    const entry = path.join(procRoot, name);
    const stat = attempt(() => readFileSync(path.join(entry, 'stat'), 'utf8'));
    if (stat === null || !LIVE_STATE.test(processState(stat))) continue;
    const args = (attempt(() => readFileSync(path.join(entry, 'cmdline'), 'utf8')) ?? '').split('\0').filter(Boolean);
    const environment = attempt(() => readFileSync(path.join(entry, 'environ'), 'utf8')) ?? '';
    const targetDir = environment.split('\0').find((variable) => variable.startsWith('CARGO_TARGET_DIR='));
    const cwd = workingDirectory(entry);
    if (cwd === EXITING) continue;
    processes.push({
      pid: Number(name),
      ppid: Number(processState(stat).split(' ')[1]),
      executable: attempt(() => readlinkSync(path.join(entry, 'exe')).replace(/ \(deleted\)$/u, '')),
      cwd,
      args,
      program: args[0] ?? '',
      targetDir: targetDir ? targetDir.slice('CARGO_TARGET_DIR='.length) : null,
    });
  }
  return processes;
}

/** Processes from `ps` or PowerShell: whole command lines, but no working directories. */
function listedProcesses() {
  const windows = process.platform === 'win32';
  const listing = windows
    ? spawnSync('powershell', ['-NoProfile', '-Command',
      'Get-CimInstance Win32_Process | ForEach-Object { "$($_.ProcessId)`t$($_.ParentProcessId)`t$($_.ExecutablePath)`t$($_.CommandLine)" }'],
    { encoding: 'utf8', timeout: 20000, windowsHide: true })
    : spawnSync('ps', ['-axww', '-o', 'pid=,ppid=,command='], { encoding: 'utf8', timeout: 20000 });
  if (listing.status !== 0 || typeof listing.stdout !== 'string') return null;
  const processes = [];
  for (const line of listing.stdout.split(/\r?\n/u)) {
    const fields = windows ? line.split('\t') : /^\s*(\d+)\s+(\d+)\s+(.*)$/u.exec(line)?.slice(1);
    if (!fields || fields.length < 3 || !/^\d+$/u.test(fields[0])) continue;
    const command = (windows ? fields.slice(3).join('\t') : fields[2]).trim();
    const program = windows ? fields[2] : command.split(/\s+/u)[0];
    processes.push({
      pid: Number(fields[0]),
      ppid: Number(fields[1]),
      executable: program && path.isAbsolute(program) ? program : null,
      cwd: null,
      args: command ? [command] : [],
      targetDir: null,
      program: program || command.split(/\s+/u)[0] || '',
    });
  }
  return processes;
}

/** Running processes and how they were listed; `probe` is `unavailable` when they cannot be. */
export function runningProcesses() {
  let probe = 'unavailable';
  let processes = [];
  if (process.platform === 'linux' && existsSync('/proc/self/stat')) {
    probe = 'proc';
    processes = linuxProcesses();
  } else {
    const listed = listedProcesses();
    if (listed) {
      probe = process.platform === 'win32' ? 'powershell' : 'ps';
      processes = listed;
    }
  }
  // The cleanup itself, and whatever started it, may name the paths it cleans.
  const parents = new Map(processes.map(({ pid, ppid }) => [pid, ppid]));
  const ancestors = new Set();
  for (let pid = process.pid; pid > 0 && !ancestors.has(pid); pid = parents.get(pid) ?? (pid === process.pid ? process.ppid : 0)) {
    ancestors.add(pid);
  }
  return { probe, processes: processes.filter(({ pid }) => !ancestors.has(pid)) };
}

/** Whether `text` names `directory` or a path inside it. */
function mentions(text, directory) {
  for (let index = text.indexOf(directory); index !== -1; index = text.indexOf(directory, index + 1)) {
    const next = text[index + directory.length];
    if (next === undefined || next === '/' || next === '\\' || next === ' ' || next === '"' || next === "'") return true;
  }
  return false;
}

const cargoFamily = ({ executable, program }) => [executable, program]
  .some((name) => typeof name === 'string' && CARGO_FAMILY.test(path.basename(name)));

/** Why a running process uses `directory` or one of the Cargo `targets` in it, or null. */
function processUsing(directory, targets, running, root, display) {
  for (const entry of running.processes) {
    if (entry.executable && inside(entry.executable, directory)) {
      return `a running process (pid ${entry.pid}) executes ${display(entry.executable)}`;
    }
    if (entry.args.some((argument) => mentions(argument, directory))) {
      return `a running process (pid ${entry.pid}) names it in its command line`;
    }
    if (targets.length === 0 || !cargoFamily(entry)) continue;
    if (entry.args.some((argument) => targets.some((target) => mentions(argument, target)))) {
      return `a running Cargo process (pid ${entry.pid}) names its target directory`;
    }
    if (entry.cwd === null) return `a running Cargo process (pid ${entry.pid}) may use it`;
    const usesTarget = targets.some((target) => {
      const workspace = path.dirname(target);
      const fromEnvironment = entry.targetDir && path.resolve(entry.cwd, entry.targetDir) === target;
      const inWorkspace = inside(entry.cwd, workspace) || (inside(entry.cwd, root) && inside(workspace, entry.cwd));
      return fromEnvironment || inWorkspace;
    });
    if (usesTarget) return `a running Cargo process (pid ${entry.pid}) builds in its workspace`;
  }
  return null;
}

// Disk usage, counted in allocated blocks with hard links counted once.

export function diskUsage(target, seen = new Set()) {
  let total = 0;
  const stack = [target];
  while (stack.length > 0) {
    const current = stack.pop();
    let stats;
    try {
      stats = lstatSync(current);
    } catch {
      continue;
    }
    const key = `${stats.dev}:${stats.ino}`;
    if (stats.nlink > 1 && !stats.isDirectory()) {
      if (seen.has(key)) continue;
      seen.add(key);
    }
    total += typeof stats.blocks === 'number' && stats.blocks > 0 ? stats.blocks * 512 : stats.size;
    if (stats.isDirectory()) {
      let entries = [];
      try {
        entries = readdirSync(current);
      } catch {
        continue;
      }
      for (const entry of entries) stack.push(path.join(current, entry));
    }
  }
  return total;
}

/** Free bytes on the file system that holds `directory`, or null when unknown. */
export function freeBytes(directory) {
  try {
    const stats = statfsSync(directory);
    return Number(stats.bavail) * Number(stats.bsize);
  } catch {
    return null;
  }
}

// Discovery context and the safety checks.

function listIgnored(root) {
  const output = git(root, ['ls-files', '--others', '--ignored', '--exclude-standard', '--directory', '-z']) ?? '';
  const directories = [];
  const files = [];
  for (const entry of output.split('\0').filter(Boolean)) {
    if (entry.endsWith('/')) directories.push(entry.slice(0, -1));
    else files.push(entry);
  }
  return { directories, files };
}

function otherWorktrees(root) {
  const output = git(root, ['worktree', 'list', '--porcelain']) ?? '';
  return output
    .split('\n')
    .filter((line) => line.startsWith('worktree '))
    .map((line) => real(line.slice('worktree '.length)))
    .filter((worktree) => worktree !== root);
}

function buildContext(repository, options) {
  const { root } = repository;
  const tracked = (git(root, ['ls-files', '-z']) ?? '').split('\0').filter(Boolean).sort();
  const resolveUserPath = (value) => real(path.resolve(options.cwd, value));
  const resultsDirectories = [...DEFAULT_RESULTS_DIRECTORIES, ...options.resultsDirectories]
    .map((directory) => resolveUserPath(path.isAbsolute(directory) ? directory : path.join(root, directory)));
  const targetDirectories = [
    ...options.targetDirectories,
    ...['CARGO_TARGET_DIR', 'CARGO_BUILD_TARGET_DIR'].map((name) => options.env[name]).filter(Boolean),
  ].map((directory) => (path.isAbsolute(directory) ? real(directory) : resolveUserPath(directory)));
  const protectedPaths = [
    ...PROTECTED_DIRECTORIES.map((directory) => path.join(root, directory)),
    ...resultsDirectories,
    ...options.protect.map(resolveUserPath),
  ];
  return {
    root,
    tracked,
    ignored: listIgnored(root),
    worktrees: otherWorktrees(root),
    targetDirectories,
    evidenceDirectories: resultsDirectories.map((directory) => path.relative(root, directory)),
    resultsDirectories,
    scratchExceptions: resultsDirectories.map((directory) => path.join(directory, 'work')),
    protectedPaths,
    evidenceFiles: /\.(?:jsonl?|log)$/u,
    // Emitted translations and their reproducers, which native validation
    // records cite as artifacts.
    workEvidence: resultsDirectories.map((directory) => path.join(directory, 'work', 'native')),
    tmpRoot: real(options.tmpRoot ?? os.tmpdir()),
  };
}

function hasTrackedUnder(context, relative) {
  const { tracked } = context;
  const prefix = `${relative}/`;
  let low = 0;
  let high = tracked.length;
  while (low < high) {
    const middle = (low + high) >> 1;
    if (tracked[middle] < relative) low = middle + 1;
    else high = middle;
  }
  for (let index = low; index < tracked.length && tracked[index] <= `${relative}/￿`; index += 1) {
    if (tracked[index] === relative || tracked[index].startsWith(prefix)) return true;
  }
  return false;
}

function ignoredByGit(root, relatives) {
  if (relatives.length === 0) return new Set();
  const result = spawnSync('git', ['check-ignore', '--stdin', '-z'], {
    cwd: root,
    input: relatives.join('\0') + '\0',
    encoding: 'utf8',
    maxBuffer: 64 * MB,
  });
  return new Set((result.stdout ?? '').split('\0').filter(Boolean));
}

/** Why `candidate` must stay, or null when it may be removed. */
function refusal(candidate, context, ignored) {
  const target = candidate.path;
  let stats;
  try {
    stats = lstatSync(target);
  } catch {
    return 'missing';
  }
  if (stats.isSymbolicLink()) return 'symlink';
  if (real(target) !== target) return 'resolves through a symlink to another path';
  if (target === path.parse(target).root || target === os.homedir()) return 'is a file system or home root';
  if (inside(context.root, target)) return 'contains the repository';
  if (candidate.scratchMarker) {
    if (path.dirname(target) !== context.tmpRoot) return 'scratch marker outside the temporary directory';
    if (candidate.scratchMarker.pid !== process.pid && processAlive(candidate.scratchMarker.pid)) {
      return `scratch still in use by pid ${candidate.scratchMarker.pid}`;
    }
    return null;
  }
  if (strictlyInside(target, context.root)) {
    const relative = toPosix(path.relative(context.root, target));
    if (inside(target, path.join(context.root, '.git'))) return 'inside the git directory';
    if (hasTrackedUnder(context, relative)) return 'holds tracked files';
    if (!ignored.has(relative)) return 'not ignored by git';
  } else if (!(candidate.externalRoot && inside(target, candidate.externalRoot) && isCargoTargetDirectory(candidate.externalRoot))) {
    return 'outside the repository';
  }
  for (const protectedPath of context.protectedPaths) {
    const scratch = context.scratchExceptions.some((exception) => strictlyInside(target, exception));
    if (inside(target, protectedPath) && !scratch) return `evidence under ${path.relative(context.root, protectedPath) || protectedPath}`;
    if (strictlyInside(protectedPath, target)) return `holds evidence ${path.relative(context.root, protectedPath)}`;
  }
  if (context.evidenceFiles.test(target) && context.scratchExceptions.some((exception) => strictlyInside(target, exception))) {
    return 'evidence record';
  }
  for (const evidence of context.workEvidence) {
    if (inside(target, evidence) || strictlyInside(evidence, target)) return 'native validation evidence';
  }
  for (const worktree of context.worktrees) {
    if (inside(target, worktree) || strictlyInside(worktree, target)) return 'belongs to another worktree';
  }
  return null;
}

// Container resources carry the project label; nothing else is touched.

function dockerAvailable(env) {
  const probe = spawnSync('docker', ['info', '--format', '{{.ServerVersion}}'], {
    env,
    stdio: 'ignore',
    timeout: 5000,
  });
  return probe.status === 0;
}

export function dockerCommands(mode) {
  const filter = `label=${CONTAINER_LABEL}`;
  return [
    ['container', 'prune', '--force', '--filter', filter],
    ['image', 'prune', '--all', '--force', '--filter', filter],
    ['volume', 'prune', '--force', '--filter', filter],
    ['buildx', 'prune', '--builder', BUILDKIT_BUILDER, '--force', ...(mode === 'full' ? ['--all'] : ['--filter', 'until=24h'])],
  ];
}

function cleanContainers(mode, options) {
  if (!dockerAvailable(options.env)) return { status: 'skipped', reason: 'docker is not installed or its daemon is unreachable', commands: [] };
  const commands = [];
  for (const args of dockerCommands(mode)) {
    if (args[0] === 'buildx') {
      const builder = spawnSync('docker', ['buildx', 'inspect', BUILDKIT_BUILDER], { env: options.env, stdio: 'ignore', timeout: 10000 });
      if (builder.status !== 0) {
        commands.push({ args, skipped: `no ${BUILDKIT_BUILDER} builder` });
        continue;
      }
    }
    if (options.dryRun) {
      commands.push({ args, skipped: 'dry run' });
      continue;
    }
    const run = spawnSync('docker', args, { env: options.env, encoding: 'utf8', timeout: 120000 });
    const reclaimed = /Total reclaimed space:\s*(.+)/u.exec(run.stdout ?? '')?.[1]?.trim();
    commands.push({ args, exitCode: run.status, ...(reclaimed ? { reclaimed } : {}) });
  }
  return { status: 'ok', commands };
}

// The run.

function normalizeOptions(options) {
  const env = options.env ?? process.env;
  const budgetMb = options.budgetMb ?? Number(env.META_LANGUAGE_CACHE_BUDGET_MB ?? DEFAULT_BUDGET_MB);
  const minFreeMb = options.minFreeMb ?? Number(env.META_LANGUAGE_MIN_FREE_MB ?? DEFAULT_MIN_FREE_MB);
  if (!Number.isFinite(budgetMb) || budgetMb < 0) throw new UsageError(`invalid disk budget: ${budgetMb}`);
  if (!Number.isFinite(minFreeMb) || minFreeMb < 0) throw new UsageError(`invalid free-space floor: ${minFreeMb}`);
  const mode = options.mode ?? 'prune';
  if (!['prune', 'full'].includes(mode)) throw new UsageError(`invalid mode: ${mode}`);
  const event = options.event ?? 'manual';
  if (!/^[a-z][a-z0-9-]*$/u.test(event)) throw new UsageError(`invalid event name: ${event}`);
  const classes = options.classes?.length ? options.classes : null;
  for (const id of classes ?? []) {
    if (!CACHE_CLASSES.some((cacheClass) => cacheClass.id === id)) throw new UsageError(`unknown cache class: ${id}`);
  }
  return {
    cwd: path.resolve(options.cwd ?? process.cwd()),
    env,
    event,
    mode,
    budgetBytes: Math.round(budgetMb * MB),
    minFreeBytes: Math.round(minFreeMb * MB),
    preflight: options.preflight ?? true,
    dryRun: Boolean(options.dryRun),
    classes,
    targetDirectories: (options.targetDirectories ?? []).map((value) => assertNoTraversal(value, '--target-dir')),
    resultsDirectories: (options.resultsDirectories ?? []).map((value) => assertNoTraversal(value, '--results-dir')),
    protect: (options.protect ?? []).map((value) => assertNoTraversal(value, '--protect')),
    tmpRoot: options.tmpRoot,
    docker: options.docker ?? true,
  };
}

const outermost = (paths) => {
  const sorted = [...new Set(paths)].sort();
  return sorted.filter((candidate, index) => !sorted.slice(0, index).some((other) => strictlyInside(candidate, other)));
};

function aggregateBytes(paths) {
  const seen = new Set();
  return outermost(paths).reduce((total, candidate) => total + diskUsage(candidate, seen), 0);
}

/** Cleans the caches of the worktree at `options.cwd` and returns the report. */
export function runCleanup(rawOptions = {}) {
  const started = Date.now();
  const options = normalizeOptions(rawOptions);
  const base = { schemaVersion: 1, event: options.event, mode: options.mode, dryRun: options.dryRun, startedAt: new Date(started).toISOString() };
  const repository = resolveRepository(options.cwd);
  if (!repository) return { ...base, status: 'skipped', reason: 'not inside a git worktree, or git is not installed' };
  const lock = acquireCleanupLock(repository.stateDir);
  if (lock.busy) return { ...base, root: repository.root, status: 'busy', reason: `another cleanup (pid ${lock.busy}) is running` };
  try {
    const report = cleanWithLock(repository, options, base);
    report.durationMs = Date.now() - started;
    if (!options.dryRun) persistReport(repository.stateDir, report);
    return report;
  } finally {
    lock.release();
  }
}

function cleanWithLock(repository, options, base) {
  const context = buildContext(repository, options);
  const report = { ...base, root: context.root, budgetBytes: options.budgetBytes, status: 'ok', classes: {} };
  const freeBefore = freeBytes(context.root);
  let mode = options.mode;
  report.preflight = { enabled: options.preflight, minFreeBytes: options.minFreeBytes, freeBytesBefore: freeBefore, escalated: false };
  if (options.preflight && freeBefore !== null && freeBefore < options.minFreeBytes && mode !== 'full') {
    mode = 'full';
    report.preflight.escalated = true;
  }
  report.effectiveMode = mode;

  const leases = liveLeases(repository.stateDir);
  const lockProbe = flockAvailable();
  const running = runningProcesses();
  report.activeBuilds = {
    leases: leases.map(({ pid, label }) => ({ pid, label })),
    cargoLockProbe: lockProbe ? 'flock' : 'unavailable',
    processProbe: running.probe,
  };
  const busyTargets = new Map();

  const selected = CACHE_CLASSES.filter((cacheClass) => !options.classes || options.classes.includes(cacheClass.id));
  const candidates = [];
  for (const cacheClass of selected) {
    const entry = { title: cacheClass.title, removed: [], skipped: [], reclaimedBytes: 0 };
    report.classes[cacheClass.id] = entry;
    if (cacheClass.docker) continue;
    for (const candidate of cacheClass.discover(context)) {
      candidates.push({ ...candidate, path: path.resolve(candidate.path), classId: cacheClass.id, activeSensitive: cacheClass.activeSensitive });
    }
  }
  const relativeInside = candidates
    .filter((candidate) => strictlyInside(candidate.path, context.root))
    .map((candidate) => toPosix(path.relative(context.root, candidate.path)));
  const ignored = ignoredByGit(context.root, [...new Set(relativeInside)]);

  const viable = [];
  const seenPaths = new Set();
  for (const candidate of candidates) {
    const key = `${candidate.path}\0${candidate.tier}`;
    if (seenPaths.has(key)) continue;
    seenPaths.add(key);
    const entry = report.classes[candidate.classId];
    const display = displayPath(context.root, candidate.path);
    let reason = refusal(candidate, context, ignored);
    if (!reason && candidate.activeSensitive && leases.length > 0) {
      reason = `active build (${leases.map(({ pid, label }) => `${label} pid ${pid}`).join(', ')})`;
    }
    const targets = candidate.activeSensitive ? (candidate.target ? [candidate.target] : nestedCargoTargets(candidate.path, 3)) : [];
    if (!reason && candidate.activeSensitive && lockProbe) {
      if (targets.some((target) => busyCargoTarget(target, busyTargets))) reason = 'a running Cargo build holds its lock';
    }
    if (!reason && candidate.activeSensitive) {
      reason = processUsing(candidate.path, targets, running, context.root, (file) => displayPath(context.root, file));
    }
    if (reason === 'missing') continue;
    if (reason) entry.skipped.push({ path: display, reason });
    else viable.push(candidate);
  }

  report.beforeBytes = aggregateBytes(viable.map((candidate) => candidate.path));
  let current = report.beforeBytes;
  const removedPaths = [];
  const removedBytes = new Map();
  const remove = (candidate) => {
    if (removedPaths.some((removed) => inside(candidate.path, removed))) return;
    // A dry run leaves nested removals on disk, so they are not counted twice.
    const nested = options.dryRun
      ? removedPaths.filter((removed) => strictlyInside(removed, candidate.path)).reduce((total, removed) => total + removedBytes.get(removed), 0)
      : 0;
    const bytes = Math.max(0, diskUsage(candidate.path) - nested);
    if (!options.dryRun) rmSync(candidate.path, { recursive: true, force: true });
    removedPaths.push(candidate.path);
    removedBytes.set(candidate.path, bytes);
    current -= bytes;
    const entry = report.classes[candidate.classId];
    entry.removed.push({ path: displayPath(context.root, candidate.path), tier: candidate.tier, note: candidate.note, bytes });
    entry.reclaimedBytes += bytes;
  };
  const byTier = (tier) => viable.filter((candidate) => candidate.tier === tier);
  if (mode === 'full') {
    for (const candidate of [...byTier('full'), ...byTier('warm'), ...byTier('transient')]) remove(candidate);
  } else {
    for (const candidate of byTier('transient')) remove(candidate);
    const warm = byTier('warm')
      .map((candidate) => ({ candidate, bytes: diskUsage(candidate.path) }))
      .sort((left, right) => (left.candidate.priority ?? 0) - (right.candidate.priority ?? 0) || right.bytes - left.bytes);
    for (const { candidate } of warm) {
      if (current <= options.budgetBytes) break;
      remove(candidate);
    }
  }

  if (report.classes.containers) {
    const containers = options.docker ? cleanContainers(mode, options) : { status: 'skipped', reason: 'container cleanup disabled', commands: [] };
    Object.assign(report.classes.containers, containers);
  }

  report.afterBytes = options.dryRun
    ? Math.max(0, current)
    : aggregateBytes(viable.map((candidate) => candidate.path).filter((candidate) => existsSync(candidate)));
  report.reclaimedBytes = Object.values(report.classes).reduce((total, entry) => total + (entry.reclaimedBytes ?? 0), 0);
  report.withinBudget = report.afterBytes <= options.budgetBytes;
  report.preflight.freeBytesAfter = freeBytes(context.root);
  report.preflight.lowDiskAfter = report.preflight.freeBytesAfter !== null && report.preflight.freeBytesAfter < options.minFreeBytes;
  if (options.dryRun) report.status = 'dry-run';
  return report;
}

function displayPath(root, candidate) {
  return strictlyInside(candidate, root) ? toPosix(path.relative(root, candidate)) : candidate;
}

function persistReport(stateDir, report) {
  mkdirSync(stateDir, { recursive: true });
  writeFileSync(path.join(stateDir, 'last-run.json'), `${JSON.stringify(report, null, 2)}\n`);
  const historyFile = path.join(stateDir, 'history.jsonl');
  const summary = {
    event: report.event,
    mode: report.effectiveMode ?? report.mode,
    status: report.status,
    startedAt: report.startedAt,
    beforeBytes: report.beforeBytes,
    afterBytes: report.afterBytes,
    reclaimedBytes: report.reclaimedBytes,
  };
  appendFileSync(historyFile, `${JSON.stringify(summary)}\n`);
  const lines = readFileSync(historyFile, 'utf8').split('\n').filter(Boolean);
  if (lines.length > HISTORY_LIMIT) writeFileSync(historyFile, `${lines.slice(-HISTORY_LIMIT).join('\n')}\n`);
}

/** Human-readable byte count. */
export function formatBytes(bytes) {
  if (typeof bytes !== 'number') return 'n/a';
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
  let value = bytes;
  let unit = 0;
  while (Math.abs(value) >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${value.toFixed(unit === 0 ? 0 : 1)} ${units[unit]}`;
}

/** One-paragraph summary of a report. */
export function summarize(report) {
  if (report.status === 'skipped' || report.status === 'busy') return `cache cleanup ${report.status}: ${report.reason}`;
  const removed = Object.values(report.classes).reduce((count, entry) => count + (entry.removed?.length ?? 0), 0);
  const skipped = Object.values(report.classes).reduce((count, entry) => count + (entry.skipped?.length ?? 0), 0);
  return [
    `cache cleanup (${report.event}, ${report.effectiveMode}${report.preflight?.escalated ? ' after low-disk preflight' : ''}${report.dryRun ? ', dry run' : ''}):`,
    `before ${formatBytes(report.beforeBytes)}, after ${formatBytes(report.afterBytes)},`,
    `reclaimed ${formatBytes(report.reclaimedBytes)}; budget ${formatBytes(report.budgetBytes)};`,
    `${removed} removed, ${skipped} kept for safety`,
  ].join(' ');
}
