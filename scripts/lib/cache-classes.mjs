// Registry of the regenerable cache classes that scripts/clean-caches.mjs owns.
//
// Each class discovers candidate paths; the engine in cache-cleanup.mjs decides
// what may be deleted. A candidate is only a proposal: the engine still checks
// that it is git-ignored, holds no tracked file, stays inside the repository
// (or inside a scratch directory this repository marked as its own), is not a
// symlink, is not evidence, and is not in use by an active build.
//
// Tiers:
// - transient: removed on every run (incremental state, linked examples,
//   coverage data, scratch trees, generated intermediates);
// - warm: kept while the aggregate cache stays within the disk budget, then
//   removed in ascending `priority` until it does;
// - full: removed only by `--mode full` or a low-disk preflight.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

/** File that marks a temporary directory as scratch owned by this repository. */
export const SCRATCH_MARKER = '.meta-language-scratch.json';

/** Label that marks Docker containers, images and volumes this repository owns. */
export const CONTAINER_LABEL = 'org.link-foundation.meta-language=cache';

/** BuildKit builder this repository owns, if one was ever created. */
export const BUILDKIT_BUILDER = 'meta-language';

/** Prefixes of the scratch directories the repository scripts create in the OS temp directory. */
export const SCRATCH_PREFIXES = Object.freeze([
  'grammar-',
  'web-tree-sitter-',
  'default-cst-cli-',
  'issue-195-',
  'lean-roots-',
  'meta-language-',
]);

const ROCQ_OUTPUT = /(?:\.(?:vo|vok|vos|glob|aux)|^\.(?:lia|nia)\.cache|^Makefile\.coq(?:\.conf)?|^\.Makefile\.coq\.d)$/u;
const NATIVE_INTERMEDIATE = /\.(?:o|obj|a|lib|so|dylib|dll|exp|pdb|wasm)$/u;
const INTERMEDIATE_DIRECTORIES = new Set(['build', '.build', 'node_modules', 'target']);
const VENDOR_ROOTS = ['rust/vendor/', 'js/src/vendor/'];

const isDirectory = (candidate) => {
  try {
    return statSync(candidate).isDirectory();
  } catch {
    return false;
  }
};

const children = (directory) => {
  try {
    return readdirSync(directory, { withFileTypes: true });
  } catch {
    return [];
  }
};

/** A Cargo target directory is recognized by the files Cargo writes into it. */
export function isCargoTargetDirectory(directory) {
  return isDirectory(directory) &&
    (existsSync(path.join(directory, 'CACHEDIR.TAG')) || existsSync(path.join(directory, '.rustc_info.json')));
}

/** Profile directories of a target: `debug`, `release`, custom profiles, and `<triple>/<profile>`. */
function profileDirectories(target) {
  const profiles = [];
  for (const entry of children(target)) {
    if (!entry.isDirectory()) continue;
    const candidate = path.join(target, entry.name);
    if (existsSync(path.join(candidate, '.fingerprint')) || existsSync(path.join(candidate, 'deps'))) {
      profiles.push(candidate);
      continue;
    }
    for (const nested of children(candidate)) {
      const inner = path.join(candidate, nested.name);
      if (nested.isDirectory() && existsSync(path.join(inner, '.fingerprint'))) profiles.push(inner);
    }
  }
  return profiles;
}

/** Every Cargo target directory in scope: the defaults, nested ignored targets, and custom ones. */
export function rustTargetDirectories(context) {
  const found = new Map();
  const add = (directory, explicit) => {
    const resolved = path.resolve(directory);
    if (!found.has(resolved) && isCargoTargetDirectory(resolved)) found.set(resolved, { directory: resolved, explicit });
  };
  for (const relative of ['rust/target', 'target', 'rust/web/target']) add(path.join(context.root, relative), false);
  for (const entry of context.ignored.directories) {
    if (path.posix.basename(entry) === 'target') add(path.join(context.root, entry), false);
  }
  for (const directory of context.targetDirectories) add(directory, true);
  return [...found.values()];
}

function discoverRustTargets(context) {
  const candidates = [];
  for (const { directory: target, explicit } of rustTargetDirectories(context)) {
    const external = explicit ? target : undefined;
    const push = (candidate, tier, note, priority = 0) => {
      if (existsSync(candidate)) candidates.push({ path: candidate, tier, note, priority, externalRoot: external, target });
    };
    for (const profile of profileDirectories(target)) {
      push(path.join(profile, 'incremental'), 'transient', 'incremental compilation state');
      push(path.join(profile, 'examples'), 'transient', 'linked examples');
    }
    push(path.join(target, 'llvm-cov-target'), 'transient', 'coverage build');
    push(path.join(target, 'tmp'), 'transient', 'test scratch (CARGO_TARGET_TMPDIR)');
    push(path.join(target, 'package'), 'transient', 'cargo package output');
    push(path.join(target, 'criterion'), 'transient', 'benchmark scratch');
    push(path.join(target, 'nextest'), 'transient', 'test runner scratch');
    for (const entry of children(target)) {
      if (entry.isFile() && entry.name.endsWith('.profraw')) push(path.join(target, entry.name), 'transient', 'coverage profile');
    }
    push(path.join(target, 'doc'), 'warm', 'rustdoc output', 10);
    for (const profile of profileDirectories(target)) {
      const debug = path.basename(profile) === 'debug';
      push(profile, 'warm', debug ? 'debug and test build (warm cache)' : 'optimized build', debug ? 30 : 20);
    }
    push(target, 'full', 'whole target directory');
  }
  for (const entry of context.ignored.files) {
    if (entry.endsWith('.profraw') || path.posix.basename(entry) === 'lcov.info') {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'coverage report', priority: 0 });
    }
  }
  return candidates;
}

function discoverJavaScriptCaches(context) {
  const candidates = [];
  for (const relative of [
    'js/node_modules/.cache', 'rust/web/node_modules/.cache', 'js/coverage', 'coverage', 'js/.nyc_output',
  ]) {
    const candidate = path.join(context.root, relative);
    if (existsSync(candidate)) candidates.push({ path: candidate, tier: 'transient', note: 'JavaScript build or test cache' });
  }
  for (const entry of context.ignored.files) {
    if (/^(?:js\/)?[^/]+\.tgz$/u.test(entry)) {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'npm pack output for a temporary consumer' });
    }
  }
  return candidates;
}

function discoverGeneratedIntermediates(context) {
  const candidates = [];
  for (const relative of ['rust/web/pkg', '_site']) {
    const candidate = path.join(context.root, relative);
    if (existsSync(candidate)) candidates.push({ path: candidate, tier: 'transient', note: 'generated build output' });
  }
  const inVendor = (entry) => VENDOR_ROOTS.some((prefix) => entry.startsWith(prefix));
  for (const entry of context.ignored.directories) {
    if (inVendor(entry) && INTERMEDIATE_DIRECTORIES.has(path.posix.basename(entry))) {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'generated parser build directory' });
    }
  }
  for (const entry of context.ignored.files) {
    if (inVendor(entry) && NATIVE_INTERMEDIATE.test(entry)) {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'compiled parser intermediate' });
    }
  }
  return candidates;
}

function discoverProofBuilds(context) {
  const candidates = [];
  for (const entry of context.ignored.directories) {
    const name = path.posix.basename(entry);
    const parent = path.join(context.root, path.posix.dirname(entry));
    const lakeProject = existsSync(path.join(parent, 'lakefile.toml')) || existsSync(path.join(parent, 'lakefile.lean'));
    if (name === '.lake' || (name === 'build' && lakeProject)) {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'Lean build output' });
    }
  }
  for (const entry of context.ignored.files) {
    if (ROCQ_OUTPUT.test(path.posix.basename(entry))) {
      candidates.push({ path: path.join(context.root, entry), tier: 'transient', note: 'Rocq build output' });
    }
  }
  return candidates;
}

// The evidence runner keeps its raw execution records in `<results>/work`, so
// only the scratch trees beside them are candidates; the engine also refuses
// `.json`, `.jsonl` and `.log` files there.
function discoverAcceptanceScratch(context) {
  const note = 'acceptance scratch: candidate packages, clean consumers and their Rust targets, downstream clones';
  const candidates = [];
  const legacy = path.join(context.root, '.issue-195-work');
  if (existsSync(legacy)) candidates.push({ path: legacy, tier: 'transient', note });
  for (const directory of context.evidenceDirectories) {
    const work = path.join(path.resolve(context.root, directory), 'work');
    for (const entry of children(work)) {
      candidates.push({ path: path.join(work, entry.name), tier: 'transient', note });
    }
  }
  return candidates;
}

/** Reads a scratch marker, or null when the directory is not marked. */
export function readScratchMarker(directory) {
  try {
    return JSON.parse(readFileSync(path.join(directory, SCRATCH_MARKER), 'utf8'));
  } catch {
    return null;
  }
}

function discoverTemporaryClones(context) {
  const candidates = [];
  for (const entry of children(context.tmpRoot)) {
    if (!entry.isDirectory() || !SCRATCH_PREFIXES.some((prefix) => entry.name.startsWith(prefix))) continue;
    const directory = path.join(context.tmpRoot, entry.name);
    const marker = readScratchMarker(directory);
    if (!marker || marker.root !== context.root) continue;
    candidates.push({
      path: directory,
      tier: 'transient',
      note: `temporary clone or scratch of ${marker.label ?? 'a repository script'}`,
      scratchMarker: marker,
    });
  }
  return candidates;
}

/**
 * The cache classes, in the order they are reported. `covers` names the
 * categories a class is responsible for; scripts/check-cache-policy.mjs fails
 * when a required category has no class.
 */
export const CACHE_CLASSES = Object.freeze([
  {
    id: 'rust-target',
    title: 'Cargo target directories, including custom CARGO_TARGET_DIR and --target-dir',
    covers: [
      'rust-debug', 'rust-release', 'rust-test', 'rust-coverage', 'rust-incremental',
      'rust-examples', 'rust-doc', 'rust-package', 'benchmark-scratch', 'custom-target-directory',
    ],
    activeSensitive: true,
    discover: discoverRustTargets,
  },
  {
    id: 'javascript',
    title: 'JavaScript build and test caches and temporary package consumers',
    covers: ['js-build-cache', 'js-test-coverage', 'js-package-consumer'],
    activeSensitive: false,
    discover: discoverJavaScriptCaches,
  },
  {
    id: 'generated-intermediates',
    title: 'Generated parser and compiler intermediates',
    covers: ['generated-parser-intermediates', 'generated-compiler-intermediates'],
    activeSensitive: true,
    discover: discoverGeneratedIntermediates,
  },
  {
    id: 'proof-build',
    title: 'Lean and Rocq build output',
    covers: ['lean-build', 'rocq-build'],
    activeSensitive: true,
    discover: discoverProofBuilds,
  },
  {
    id: 'acceptance-scratch',
    title: 'Acceptance and benchmark scratch trees',
    covers: ['acceptance-scratch', 'nested-consumer-target', 'nested-clone'],
    activeSensitive: true,
    discover: discoverAcceptanceScratch,
  },
  {
    id: 'temporary-clones',
    title: 'Temporary clones and scratch directories this repository marked in the OS temp directory',
    covers: ['nested-temporary-clone'],
    activeSensitive: false,
    discover: discoverTemporaryClones,
  },
  {
    id: 'containers',
    title: 'Project-owned container resources and BuildKit cache',
    covers: ['container', 'buildkit'],
    activeSensitive: false,
    discover: () => [],
    docker: true,
  },
]);
