// The dependency inventory of docs/vision.md#dependencies: every retained
// runtime, development, build and optional dependency, lockfile resolution,
// vendored grammar asset, generator, toolchain, workflow action, build image
// and published artifact entry, each with its current stable release on the
// recorded audit date (requirement I195-DEPENDENCY-INVENTORY).
//
// collectDependencies() reads the pins from the repository offline.
// parity/dependency-inventory.json stores them with the current stable release
// and, for an item that is behind it, the recorded compatibility reason;
// refreshInventory() queries the registries to rewrite it, and
// checkInventory() fails when the stored inventory no longer matches the
// repository or when a retained item is behind without a reason.
// docs/dependency-audit.md is rendered from the inventory.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';

import { compareVersions, isPrerelease, newestStable, satisfies, versionParts } from './semver-range.mjs';

export const INVENTORY_FILE = 'parity/dependency-inventory.json';
export const AUDIT_DOCUMENT = 'docs/dependency-audit.md';

/** The categories in the order the audit document lists them. */
export const CATEGORIES = [
  { id: 'engine', title: 'JavaScript engines' },
  { id: 'npm', title: 'npm packages' },
  { id: 'setting', title: 'Rust package settings' },
  { id: 'crate', title: 'Rust crates' },
  { id: 'experiment', title: 'Experiment manifests' },
  { id: 'vendored-parser', title: 'Vendored generated parsers' },
  { id: 'vendored-runtime', title: 'Vendored runtime' },
  { id: 'vendored-grammar', title: 'Vendored WebAssembly grammars' },
  { id: 'generator', title: 'Generators' },
  { id: 'toolchain', title: 'Toolchains and tools' },
  { id: 'action', title: 'GitHub Actions' },
  { id: 'image', title: 'Build images' },
  { id: 'runner', title: 'Runners' },
  { id: 'published', title: 'Published artifact contents' },
];

/**
 * How an item's pin is compared with its current stable release:
 * `version` (the pin is at least the release), `major` and `minor` (a moving
 * tag such as `v7` or `5.4` that follows the newest release of that line),
 * `floor` (a supported minimum that must not admit an unmaintained or
 * insufficient release), `revision` (a commit that must contain the newest
 * release), `floating` (always installs the newest release), `derived` (built
 * from the items in `dependsOn`) and `unversioned`.
 */
export const COMPARISONS = ['version', 'major', 'minor', 'floor', 'revision', 'floating', 'derived', 'unversioned'];

const RUST_EDITIONS = ['2015', '2018', '2021', '2024'];
const WORKFLOWS = '.github/workflows';
const PLACEHOLDER_REASON = /no recorded compatibility reason|must be updated|\bTODO\b|\bTBD\b|^\s*$/iu;

const readText = (root, file) => readFileSync(path.join(root, file), 'utf8');
const readJson = (root, file) => JSON.parse(readText(root, file));
const unique = (values) => [...new Set(values)].sort();

function itemId({ category, scope, name, pinned }) {
  return `${category} ${scope ? `${scope} ` : ''}${name}@${pinned}`;
}

function makeItem(fields) {
  const item = { id: itemId(fields), ...fields, declaredIn: unique([].concat(fields.declaredIn)) };
  if (!item.scope) delete item.scope;
  return item;
}

/** Adds an item, or merges the declaring files of an item already collected under the same id. */
function addItem(items, fields) {
  const item = makeItem(fields);
  const existing = items.get(item.id);
  if (existing) existing.declaredIn = unique([...existing.declaredIn, ...item.declaredIn]);
  else items.set(item.id, item);
  return item;
}

// --- Manifests -------------------------------------------------------------

/** The `[[package]]` entries of a Cargo.lock. */
export function parseCargoLock(text) {
  return text
    .split(/^\[\[package\]\]$/mu)
    .slice(1)
    .map((block) => {
      const field = (name) => block.match(new RegExp(`^${name} = "([^"]*)"$`, 'mu'))?.[1] ?? null;
      const list = block.match(/^dependencies = \[([\s\S]*?)\]$/mu)?.[1] ?? '';
      return {
        name: field('name'),
        version: field('version'),
        source: field('source'),
        dependencies: [...list.matchAll(/"([^"]+)"/gu)].map((match) => match[1]),
      };
    });
}

/** The dependency requirements a Cargo.toml declares, by section. */
export function parseCargoManifest(text) {
  const dependencies = [];
  const settings = {};
  let section = null;
  for (const line of text.split('\n')) {
    const header = line.match(/^\[([^\]]+)\]\s*$/u);
    if (header) {
      section = header[1];
      continue;
    }
    const entry = line.match(/^([A-Za-z0-9_-]+)\s*=\s*(.*)$/u);
    if (!entry) continue;
    const [, key, value] = entry;
    if (section === 'package') {
      const string = value.match(/^"([^"]*)"/u);
      if (string) settings[key] = string[1];
    } else if (/(?:^|\.)(?:dev-|build-)?dependencies$/u.test(section ?? '')) {
      const requirement = value.match(/^"([^"]+)"/u)?.[1] ?? value.match(/version\s*=\s*"([^"]+)"/u)?.[1] ?? null;
      const renamed = value.match(/package\s*=\s*"([^"]+)"/u)?.[1];
      const kind = section.endsWith('dev-dependencies') ? 'development' : section.endsWith('build-dependencies') ? 'build' : 'runtime';
      dependencies.push({ name: renamed ?? key, requirement, kind, optional: /optional\s*=\s*true/u.test(value) });
    }
  }
  return { settings, dependencies };
}

function trackedFiles(root, directory, pattern) {
  const found = [];
  const walk = (relative) => {
    const absolute = path.join(root, relative);
    if (!existsSync(absolute)) return;
    for (const entry of readdirSync(absolute, { withFileTypes: true })) {
      if (entry.name === 'node_modules' || entry.name === 'target' || entry.name.startsWith('.')) continue;
      const child = path.posix.join(relative, entry.name);
      if (entry.isDirectory()) walk(child);
      else if (pattern.test(entry.name)) found.push(child);
    }
  };
  walk(directory);
  return found.sort();
}

function workflowFiles(root) {
  return readdirSync(path.join(root, WORKFLOWS))
    .filter((name) => /\.ya?ml$/u.test(name))
    .sort()
    .map((name) => `${WORKFLOWS}/${name}`);
}

// --- Collection -----------------------------------------------------------

function collectNpm(root, items) {
  const manifestFile = 'js/package.json';
  const lockFile = 'js/package-lock.json';
  const manifest = readJson(root, manifestFile);
  const lock = readJson(root, lockFile);
  const floor = manifest.engines?.node;
  if (floor) {
    addItem(items, {
      category: 'engine',
      name: 'node',
      declaredIn: manifestFile,
      pinned: floor,
      compare: 'floor',
      source: { type: 'node-maintained' },
    });
  }
  const direct = {
    ...Object.fromEntries(Object.entries(manifest.dependencies ?? {}).map(([name, range]) => [name, { range, section: 'dependencies' }])),
    ...Object.fromEntries(Object.entries(manifest.devDependencies ?? {}).map(([name, range]) => [name, { range, section: 'devDependencies' }])),
    ...Object.fromEntries(Object.entries(manifest.optionalDependencies ?? {}).map(([name, range]) => [name, { range, section: 'optionalDependencies' }])),
    ...Object.fromEntries(Object.entries(manifest.peerDependencies ?? {}).map(([name, range]) => [name, { range, section: 'peerDependencies' }])),
  };
  for (const [key, entry] of Object.entries(lock.packages ?? {})) {
    if (!key.startsWith('node_modules/')) continue;
    const name = key.slice(key.lastIndexOf('node_modules/') + 'node_modules/'.length);
    const nested = key.split('node_modules/').length > 2;
    const declared = nested ? undefined : direct[name];
    addItem(items, {
      category: 'npm',
      scope: lockFile,
      name: key.replace(/^node_modules\//u, '').replaceAll('/node_modules/', ' > '),
      declaredIn: declared ? [manifestFile, lockFile] : lockFile,
      pinned: entry.version,
      compare: 'version',
      source: { type: 'npm', package: name },
      role: declared ? 'direct' : 'transitive',
      kind: entry.dev ? 'development' : entry.optional ? 'optional' : entry.peer ? 'peer' : 'runtime',
      ...(declared ? { requirement: declared.range } : {}),
    });
  }
}

function collectCargo(root, items, manifestFile) {
  const lockFile = manifestFile.replace(/Cargo\.toml$/u, 'Cargo.lock');
  const { settings, dependencies } = parseCargoManifest(readText(root, manifestFile));
  for (const setting of ['edition', 'rust-version']) {
    if (!settings[setting]) continue;
    addItem(items, {
      category: 'setting',
      scope: manifestFile,
      name: setting,
      declaredIn: manifestFile,
      pinned: settings[setting],
      compare: setting === 'edition' ? 'version' : 'floor',
      source: setting === 'edition' ? { type: 'rust-edition' } : { type: 'rust-required', manifest: manifestFile },
    });
  }
  if (!existsSync(path.join(root, lockFile))) {
    for (const dependency of dependencies) {
      addItem(items, {
        category: 'experiment',
        scope: manifestFile,
        name: dependency.name,
        declaredIn: manifestFile,
        pinned: dependency.requirement,
        compare: 'version',
        source: { type: 'crate', crate: dependency.name },
        kind: dependency.kind,
      });
    }
    return;
  }
  const packages = parseCargoLock(readText(root, lockFile));
  const rootPackage = packages.find((entry) => entry.name === settings.name && entry.source === null);
  const directNames = new Set((rootPackage?.dependencies ?? []).map((dependency) => dependency.split(' ')[0]));
  const declared = new Map();
  for (const dependency of dependencies) {
    const entry = declared.get(dependency.name) ?? { requirements: [], kinds: [], optional: dependency.optional };
    entry.requirements.push(dependency.requirement);
    entry.kinds.push(dependency.kind);
    declared.set(dependency.name, entry);
  }
  for (const entry of packages) {
    if (!entry.source?.startsWith('registry+')) continue;
    const direct = directNames.has(entry.name) ? declared.get(entry.name) : undefined;
    addItem(items, {
      category: 'crate',
      scope: lockFile,
      name: entry.name,
      declaredIn: direct ? [manifestFile, lockFile] : lockFile,
      pinned: entry.version,
      compare: 'version',
      source: { type: 'crate', crate: entry.name },
      role: direct ? 'direct' : 'transitive',
      ...(direct
        ? { requirement: unique(direct.requirements).join('; '), kind: unique(direct.kinds).join(', ') + (direct.optional ? ' (optional)' : '') }
        : {}),
    });
  }
}

function collectExperiments(root, items) {
  for (const file of trackedFiles(root, 'experiments', /^Cargo\.toml$/u)) collectCargo(root, items, file);
  for (const file of trackedFiles(root, 'experiments', /^package\.json$/u)) {
    const manifest = readJson(root, file);
    for (const section of ['dependencies', 'devDependencies', 'optionalDependencies']) {
      for (const [name, range] of Object.entries(manifest[section] ?? {})) {
        addItem(items, {
          category: 'experiment',
          scope: file,
          name,
          declaredIn: file,
          pinned: range,
          compare: 'version',
          source: { type: 'npm', package: name },
        });
      }
    }
  }
}

function collectLeanToolchains(root, items) {
  for (const file of ['lean-toolchain', ...trackedFiles(root, 'experiments', /^lean-toolchain$/u)]) {
    if (!existsSync(path.join(root, file))) continue;
    const pinned = readText(root, file).trim().replace(/^leanprover\/lean4:/u, '');
    addItem(items, leanItem(file, pinned));
  }
}

const leanItem = (file, pinned) => ({
  category: 'toolchain',
  name: 'lean',
  declaredIn: file,
  pinned,
  compare: 'version',
  source: { type: 'github-release', repository: 'leanprover/lean4' },
});

function collectVendored(root, items) {
  const grammarLockFile = 'js/src/vendor/grammars/grammar-lock.json';
  const runtimeLockFile = 'js/src/vendor/web-tree-sitter/runtime-lock.json';
  const grammarLock = readJson(root, grammarLockFile);
  const cli = versionParts(grammarLock.treeSitterCli) ? grammarLock.treeSitterCli.match(/\d+\.\d+\.\d+/u)[0] : grammarLock.treeSitterCli;
  const cliItem = addItem(items, {
    category: 'toolchain',
    name: 'tree-sitter-cli',
    declaredIn: [grammarLockFile, 'rust/src/data/grammar-lock.json'],
    pinned: cli,
    compare: 'version',
    source: { type: 'npm', package: 'tree-sitter-cli' },
  });
  const crateItems = [...items.values()].filter((item) => item.category === 'crate' && item.scope === 'rust/Cargo.lock');
  for (const [id, grammar] of Object.entries(grammarLock.grammars).sort(([a], [b]) => a.localeCompare(b))) {
    let parent;
    if (grammar.vendored) {
      parent = addItem(items, {
        category: 'vendored-parser',
        name: grammar.vendored,
        declaredIn: [grammarLockFile, `${grammar.vendored}/NOTICE.md`],
        pinned: grammar.version,
        compare: 'revision',
        source: { type: 'github-revision', repository: grammar.upstream },
        ...(grammar.patch ? { patch: grammar.patch } : {}),
      });
    } else {
      parent = crateItems.find((item) => item.name === grammar.crate && item.pinned === grammar.version);
      if (!parent) throw new Error(`grammar ${id} names ${grammar.crate} ${grammar.version}, which rust/Cargo.lock does not lock`);
    }
    addItem(items, {
      category: 'vendored-grammar',
      name: `js/src/vendor/grammars/${id}.wasm.gz`,
      declaredIn: grammarLockFile,
      pinned: grammar.version,
      compare: 'derived',
      source: { type: 'none' },
      dependsOn: [parent.id, cliItem.id],
    });
  }
  const runtimeLock = readJson(root, runtimeLockFile);
  const image = addItem(items, {
    category: 'image',
    name: 'emscripten/emsdk',
    declaredIn: [runtimeLockFile, 'js/scripts/build-web-tree-sitter-runtime.mjs'],
    pinned: runtimeLock.emscripten,
    compare: 'version',
    source: { type: 'docker-hub', repository: 'emscripten/emsdk' },
  });
  addItem(items, {
    category: 'vendored-runtime',
    name: 'js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz',
    declaredIn: runtimeLockFile,
    pinned: runtimeLock.version,
    compare: 'version',
    source: { type: 'github-release', repository: runtimeLock.upstream },
    dependsOn: [image.id],
    ...(runtimeLock.patch ? { patch: runtimeLock.patch } : {}),
  });
}

function collectGenerators(root, items) {
  const scripts = [
    ...trackedFiles(root, 'js/scripts', /^(?:build|generate)-.*\.mjs$/u).filter((file) => path.posix.dirname(file) === 'js/scripts'),
    ...trackedFiles(root, 'rust/scripts', /^build-.*\.rs$/u),
  ];
  for (const file of scripts) {
    addItem(items, { category: 'generator', name: file, declaredIn: file, pinned: 'script', compare: 'derived', source: { type: 'none' } });
  }
}

function actionSource(action, ref) {
  if (action === 'dtolnay/rust-toolchain' && versionParts(ref) && /^\d/u.test(ref)) {
    return { compare: 'version', source: { type: 'github-release', repository: 'rust-lang/rust' } };
  }
  if (/^v\d+$/u.test(ref)) return { compare: 'major', source: { type: 'github-release', repository: action } };
  if (/^v?\d+\.\d+/u.test(ref)) return { compare: 'version', source: { type: 'github-release', repository: action } };
  return { compare: 'floating', source: { type: 'none' } };
}

function collectWorkflows(root, items) {
  const engineFloor = [...items.values()].find((item) => item.category === 'engine' && item.name === 'node')?.pinned;
  for (const file of workflowFiles(root)) {
    const text = readText(root, file);
    for (const match of text.matchAll(/^\s*(?:-\s*)?uses:\s*([^\s#@]+)@([^\s#]+)/gmu)) {
      const [, action, ref] = match;
      if (action.startsWith('./')) continue;
      addItem(items, { category: 'action', name: action, declaredIn: file, pinned: ref, ...actionSource(action, ref) });
    }
    for (const match of text.matchAll(/^\s*node-version:\s*['"]?([^'"\s#]+)/gmu)) {
      const floorTest = engineFloor && versionParts(engineFloor)?.[0] === versionParts(match[1])?.[0];
      addItem(items, {
        category: 'toolchain',
        name: 'node',
        declaredIn: file,
        pinned: match[1],
        compare: floorTest ? 'floor' : 'major',
        source: { type: floorTest ? 'node-maintained' : 'node-lts' },
      });
    }
    for (const match of text.matchAll(/npm install -g npm@(\S+)/gu)) {
      addItem(items, { category: 'toolchain', name: 'npm', declaredIn: file, pinned: match[1], compare: 'major', source: { type: 'npm', package: 'npm' } });
    }
    for (const match of text.matchAll(/^\s*ocaml-compiler:\s*['"]?([^'"\s#]+)/gmu)) {
      addItem(items, { category: 'toolchain', name: 'ocaml', declaredIn: file, pinned: match[1], compare: 'minor', source: { type: 'github-release', repository: 'ocaml/ocaml' } });
    }
    for (const match of text.matchAll(/'(rocq-[a-z-]+)=([^']+)'/gu)) {
      const [, name, pinned] = match;
      const repository = { 'rocq-core': 'rocq-prover/rocq', 'rocq-stdlib': 'rocq-prover/stdlib' }[name];
      addItem(items, {
        category: 'toolchain',
        name,
        declaredIn: file,
        pinned,
        compare: repository ? 'version' : 'unversioned',
        source: repository ? { type: 'github-release', repository } : { type: 'none' },
      });
    }
    for (const match of text.matchAll(/ELAN_VERSION:\s*(\S+)/gu)) {
      addItem(items, { category: 'toolchain', name: 'elan', declaredIn: file, pinned: match[1], compare: 'version', source: { type: 'github-release', repository: 'leanprover/elan' } });
    }
    for (const match of text.matchAll(/--default-toolchain leanprover\/lean4:(\S+)/gu)) addItem(items, leanItem(file, match[1]));
    collectFloatingTools(items, file, text);
    for (const match of text.matchAll(/\b((?:ubuntu|macos|windows)-(?:latest|\d[\d.]*))\b/gu)) {
      const floating = match[1].endsWith('-latest');
      addItem(items, {
        category: 'runner',
        name: match[1],
        declaredIn: file,
        pinned: floating ? 'latest' : match[1].replace(/^[a-z]+-/u, ''),
        compare: floating ? 'floating' : 'unversioned',
        source: { type: 'none' },
      });
    }
    for (const match of text.matchAll(/^\s*(?:image|container):\s*['"]?([^'"\s#]+):([^'"\s#]+)/gmu)) {
      addItem(items, { category: 'image', name: match[1], declaredIn: file, pinned: match[2], compare: 'version', source: { type: 'docker-hub', repository: match[1] } });
    }
  }
}

function collectFloatingTools(items, file, text) {
  const tools = [
    ...[...text.matchAll(/npx --yes((?:\s+-p\s+\S+)+)/gu)].flatMap((match) => [...match[1].matchAll(/-p\s+(\S+)/gu)].map((tool) => tool[1])),
    ...[...text.matchAll(/cargo install ([A-Za-z0-9_-]+)/gu)].map((match) => match[1]),
    ...[...text.matchAll(/^\s*tool:\s*([A-Za-z0-9_-]+)/gmu)].map((match) => match[1]),
    ...[...text.matchAll(/taiki-e\/install-action@([A-Za-z][A-Za-z0-9_-]*)/gu)].filter((match) => !/^v\d/u.test(match[1])).map((match) => match[1]),
  ];
  for (const tool of tools) {
    addItem(items, { category: 'toolchain', name: tool, declaredIn: file, pinned: 'latest', compare: 'floating', source: { type: 'none' } });
  }
}

function collectPreCommit(root, items) {
  const file = '.pre-commit-config.yaml';
  if (!existsSync(path.join(root, file))) return;
  for (const match of readText(root, file).matchAll(/repo:\s*https:\/\/github\.com\/(\S+?)(?:\.git)?\s*\n\s*rev:\s*(\S+)/gu)) {
    addItem(items, {
      category: 'toolchain',
      name: match[1],
      declaredIn: file,
      pinned: match[2],
      compare: 'version',
      source: { type: 'github-release', repository: match[1] },
    });
  }
}

function collectScripts(root, items) {
  for (const file of trackedFiles(root, 'rust/scripts', /\.sh$/u)) collectFloatingTools(items, file, readText(root, file));
  for (const file of ['Dockerfile', 'rust/Dockerfile', 'js/Dockerfile'].filter((candidate) => existsSync(path.join(root, candidate)))) {
    for (const match of readText(root, file).matchAll(/^FROM\s+([^\s:]+):(\S+)/gmu)) {
      addItem(items, { category: 'image', name: match[1], declaredIn: file, pinned: match[2], compare: 'version', source: { type: 'docker-hub', repository: match[1] } });
    }
  }
}

function collectPublished(root, items) {
  for (const entry of readJson(root, 'js/package.json').files ?? []) {
    addItem(items, { category: 'published', scope: 'npm', name: entry, declaredIn: 'js/package.json', pinned: 'files', compare: 'unversioned', source: { type: 'none' } });
  }
  const include = readText(root, 'rust/Cargo.toml').match(/^include = \[([\s\S]*?)^\]/mu)?.[1] ?? '';
  for (const match of include.matchAll(/"([^"]+)"/gu)) {
    addItem(items, { category: 'published', scope: 'crate', name: match[1], declaredIn: 'rust/Cargo.toml', pinned: 'include', compare: 'unversioned', source: { type: 'none' } });
  }
}

/** Every inventoried item the repository declares, read offline. */
export function collectDependencies(root) {
  const items = new Map();
  collectNpm(root, items);
  collectCargo(root, items, 'rust/Cargo.toml');
  collectCargo(root, items, 'rust/web/Cargo.toml');
  collectExperiments(root, items);
  collectLeanToolchains(root, items);
  collectVendored(root, items);
  collectGenerators(root, items);
  collectWorkflows(root, items);
  collectPreCommit(root, items);
  collectScripts(root, items);
  collectPublished(root, items);
  return [...items.values()].sort(compareItems);
}

const categoryOrder = new Map(CATEGORIES.map(({ id }, index) => [id, index]));

function compareItems(a, b) {
  return categoryOrder.get(a.category) - categoryOrder.get(b.category) || a.id.localeCompare(b.id, 'en');
}

// --- Status ---------------------------------------------------------------

/** The item's own comparison with its current stable release: `current`, `behind` or `not applicable`. */
export function ownStatus(item) {
  switch (item.compare) {
    case 'floating':
      return 'current';
    case 'unversioned':
    case 'derived':
      return 'not applicable';
    case 'revision':
      return item.containsCurrent === true ? 'current' : 'behind';
    case 'major':
      return compareVersions(item.pinned, item.current, 1) >= 0 ? 'current' : 'behind';
    case 'minor':
      return compareVersions(item.pinned, item.current, 2) >= 0 ? 'current' : 'behind';
    case 'version':
    case 'floor':
      return compareVersions(item.pinned, item.current) >= 0 ? 'current' : 'behind';
    default:
      throw new Error(`${item.id}: unknown comparison ${item.compare}`);
  }
}

/** The status of every item: behind when its own pin or anything it is built from is behind. */
export function statuses(items) {
  const byId = new Map(items.map((item) => [item.id, item]));
  const result = new Map();
  const visit = (item, trail = []) => {
    if (result.has(item.id)) return result.get(item.id);
    if (trail.includes(item.id)) throw new Error(`dependency cycle through ${item.id}`);
    let status = ownStatus(item);
    for (const dependency of item.dependsOn ?? []) {
      const parent = byId.get(dependency);
      if (parent && visit(parent, [...trail, item.id]) === 'behind') status = 'behind';
      else if (parent && status === 'not applicable' && item.compare === 'derived') status = 'current';
    }
    if (item.compare === 'derived' && status === 'not applicable' && (item.dependsOn ?? []).length === 0) status = 'current';
    result.set(item.id, status);
    return status;
  };
  for (const item of items) visit(item);
  return result;
}

/** Whether an item's own pin is behind and so needs its own recorded reason (derived items inherit theirs). */
const needsReason = (item) => ownStatus(item) === 'behind';

// --- Delivery -------------------------------------------------------------

const DEPTH = { major: 1, minor: 2 };
const syntaxOf = (item) => (item.category === 'crate' ? 'cargo' : 'npm');

/**
 * Why a held item's pin is not its verified newest compatible release, or
 * null when it is. A pin behind its current stable release is delivered only
 * when the refresh recorded `compatible`, the newest stable release every
 * dependent's requirement admits, and `heldBy`, the recorded requirements
 * that exclude the current release: each names an inventoried holder that is
 * itself delivered, or an external package at its newest release. A reason
 * alone never satisfies this.
 */
function heldCause(item, verdictOf) {
  if (typeof item.compatible !== 'string' || item.compatible === '') return 'no newest compatible release is recorded';
  if (!Array.isArray(item.heldBy) || item.heldBy.length === 0) return 'no dependent requirement that holds it is recorded';
  const syntax = syntaxOf(item);
  try {
    if (isPrerelease(item.compatible)) return `the recorded compatible release ${item.compatible} is a prerelease`;
    if (compareVersions(item.compatible, item.current) > 0) return `the recorded compatible release ${item.compatible} is newer than the current release ${item.current}`;
    if (compareVersions(item.pinned, item.compatible, DEPTH[item.compare]) < 0) return `the newest compatible release is ${item.compatible}`;
    let excludesCurrent = false;
    for (const holder of item.heldBy) {
      const name = holder.id ?? holder.external;
      if (typeof name !== 'string' || typeof holder.requirement !== 'string') return 'a holder records no id or requirement';
      if (!satisfies(item.compatible, holder.requirement, syntax)) return `${name} requires ${holder.requirement}, which does not admit the recorded compatible release ${item.compatible}`;
      if (!satisfies(item.current, holder.requirement, syntax)) excludesCurrent = true;
      if (holder.external) {
        if (!holder.version || !holder.newest || compareVersions(holder.version, holder.newest) < 0) {
          return `the external holder ${holder.external} ${holder.version} is behind its newest release ${holder.newest}`;
        }
      }
      for (const id of [holder.id, holder.requiredBy].filter(Boolean)) {
        const verdict = verdictOf(id);
        if (verdict === undefined) return `the holder ${id} is not inventoried`;
        if (verdict === 'behind') return `the holder ${id} is itself stale`;
      }
    }
    if (!excludesCurrent) return `no recorded holder excludes the current release ${item.current}`;
  } catch (error) {
    return error.message;
  }
  return null;
}

/**
 * The delivery verdict of every item: `current`, `compatible` (behind its
 * current stable release but at its verified newest compatible release),
 * `behind` or `not applicable`, with the cause of every `behind`. Anything
 * built from a behind item is behind.
 */
export function deliveredStatuses(items) {
  const byId = new Map(items.map((item) => [item.id, item]));
  const result = new Map();
  const visiting = new Set();
  const visit = (id) => {
    if (result.has(id)) return result.get(id).status;
    const item = byId.get(id);
    if (!item) return undefined;
    if (visiting.has(id)) return 'behind';
    visiting.add(id);
    let status = ownStatus(item);
    let cause = status === 'behind' ? 'behind its current stable release' : null;
    if (status === 'behind') {
      cause = heldCause(item, visit);
      if (cause === null) status = 'compatible';
    }
    for (const dependency of item.dependsOn ?? []) {
      const parent = visit(dependency);
      if (parent === 'behind') {
        status = 'behind';
        cause ??= `built from ${dependency}, which is stale`;
      } else if (parent !== undefined && status === 'not applicable' && item.compare === 'derived') status = 'current';
    }
    if (item.compare === 'derived' && status === 'not applicable' && (item.dependsOn ?? []).length === 0) status = 'current';
    visiting.delete(id);
    result.set(id, { status, cause: status === 'behind' ? cause : null });
    return status;
  };
  for (const item of items) visit(item.id);
  return result;
}

// --- Check ----------------------------------------------------------------

const COLLECTED_FIELDS = ['category', 'name', 'scope', 'declaredIn', 'pinned', 'compare', 'source', 'role', 'kind', 'requirement', 'patch'];

/**
 * The problems of an inventory against the collected items: missing,
 * unexpected or changed items, a missing or invalid audit date, items without
 * a current release, and retained items behind their current stable release
 * without a recorded compatibility reason.
 */
export function checkInventory(inventory, collected, { today = new Date().toISOString().slice(0, 10) } = {}) {
  const problems = [];
  const problem = (kind, message) => problems.push({ kind, message });
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(inventory.auditDate ?? '') || Number.isNaN(Date.parse(inventory.auditDate))) {
    problem('audit-date', `the inventory has no valid audit date (${JSON.stringify(inventory.auditDate)})`);
  } else if (inventory.auditDate > today) {
    problem('audit-date', `the audit date ${inventory.auditDate} is in the future`);
  }
  const stored = new Map((inventory.items ?? []).map((item) => [item.id, item]));
  const found = new Map(collected.map((item) => [item.id, item]));
  for (const item of collected) {
    const entry = stored.get(item.id);
    if (!entry) {
      problem('not-inventoried', `${item.id} (${item.declaredIn.join(', ')}) is not in the inventory`);
      continue;
    }
    for (const field of COLLECTED_FIELDS) {
      if (JSON.stringify(entry[field]) !== JSON.stringify(item[field])) {
        problem('changed', `${item.id}: the inventory records ${field} ${JSON.stringify(entry[field])}, the repository ${JSON.stringify(item[field])}`);
      }
    }
    if (item.dependsOn && JSON.stringify(entry.dependsOn) !== JSON.stringify(item.dependsOn)) {
      problem('changed', `${item.id}: the inventory records dependsOn ${JSON.stringify(entry.dependsOn)}, the repository ${JSON.stringify(item.dependsOn)}`);
    }
  }
  for (const id of stored.keys()) if (!found.has(id)) problem('not-in-repository', `${id} is inventoried but the repository no longer declares it`);
  const items = [...stored.values()];
  for (const item of items) {
    if (!COMPARISONS.includes(item.compare)) {
      problem('comparison', `${item.id}: unknown comparison ${JSON.stringify(item.compare)}`);
      continue;
    }
    for (const dependency of item.dependsOn ?? []) {
      if (!stored.has(dependency)) problem('depends-on', `${item.id} is built from ${dependency}, which is not inventoried`);
    }
    if (item.category === 'generator' && !Array.isArray(item.dependsOn)) problem('depends-on', `${item.id} does not record the tools it runs (dependsOn)`);
    if (!['floating', 'unversioned', 'derived'].includes(item.compare) && !item.current) {
      problem('no-current-release', `${item.id} records no current stable release`);
      continue;
    }
    if (item.current && isPrerelease(item.current)) problem('no-current-release', `${item.id}: ${item.current} is a prerelease, not a stable release`);
    let behind;
    try {
      behind = needsReason(item);
    } catch (error) {
      problem('comparison', `${item.id}: ${error.message}`);
      continue;
    }
    if (behind && (typeof item.reason !== 'string' || PLACEHOLDER_REASON.test(item.reason))) {
      problem('behind-without-reason', `${item.id} is behind its current stable release ${item.current} without a recorded compatibility reason`);
    }
    if (!behind && item.reason) problem('stale-reason', `${item.id} is not behind its current stable release but records a reason: ${item.reason}`);
  }
  return problems;
}

/**
 * Delivery requires every retained item at its current stable release, or at
 * the verified newest release its holders' requirements admit, including
 * transitive resolutions and generated descendants. A compatibility reason
 * explains an inventory item; it cannot waive the separate delivery
 * obligation.
 */
export function checkDeliveredDependencies(inventory, collected, options = {}) {
  const problems = checkInventory(inventory, collected, options);
  let effective;
  try {
    statuses(inventory.items ?? []);
    effective = deliveredStatuses(inventory.items ?? []);
  } catch (error) {
    problems.push({ kind: 'delivery-comparison', message: error.message });
    return problems;
  }
  for (const item of inventory.items ?? []) {
    const verdict = effective.get(item.id);
    if (verdict.status === 'behind') problems.push({
      kind: 'stale-delivered-dependency',
      message: `${item.id}: delivered pin ${item.pinned} is behind the current stable release or its generating dependency (${verdict.cause}); compatibility reasons do not satisfy delivery`,
    });
  }
  const runtimePackages = (inventory.items ?? []).filter(({ category, kind }) =>
    category === 'npm' && kind === 'runtime');
  if (runtimePackages.length > 0) {
    const packages = options.npmConsumerLock?.packages;
    if (!packages?.['node_modules/meta-language']) {
      problems.push({ kind: 'missing-delivery-consumer', message: 'provide the lockfile of a clean installed meta-language candidate; repository overrides do not establish delivered npm resolutions' });
    } else {
      const versions = new Map();
      const visited = new Set();
      const pending = ['node_modules/meta-language'];
      while (pending.length > 0) {
        const location = pending.pop();
        if (visited.has(location)) continue;
        visited.add(location);
        const entry = packages[location];
        for (const name of Object.keys(entry.dependencies ?? {})) {
          let parent = location;
          let resolved;
          for (;;) {
            const candidate = `${parent ? `${parent}/` : ''}node_modules/${name}`;
            if (packages[candidate]) { resolved = candidate; break; }
            if (!parent) break;
            parent = parent.slice(0, parent.lastIndexOf('node_modules/')).replace(/\/$/u, '');
          }
          if (!resolved) {
            problems.push({ kind: 'missing-consumer-resolution', message: `${location}: installed dependency ${name} is missing` });
            continue;
          }
          versions.set(name, [...new Set([...(versions.get(name) ?? []), packages[resolved].version])]);
          pending.push(resolved);
        }
      }
      for (const name of versions.keys()) {
        if (!runtimePackages.some((item) => item.source.package === name)) {
          problems.push({ kind: 'uninventoried-consumer-dependency', message: `${name}: installed runtime dependency has no audited current release` });
        }
      }
      for (const item of runtimePackages) {
        const installed = versions.get(item.source.package) ?? [];
        const target = effective.get(item.id).status === 'compatible' ? item.compatible : item.current;
        let stale = installed.length === 0;
        try {
          stale ||= installed.some((version) => !version || isPrerelease(version) || compareVersions(version, target) < 0);
        } catch {
          stale = true;
        }
        if (stale) {
          problems.push({ kind: 'stale-consumer-resolution', message: `${item.source.package}: clean consumer resolves ${installed.join(', ') || 'no installed version'}, audited current release is ${target}` });
        }
      }
    }
  }
  return problems;
}

/** The fields a live refresh must reproduce for the recorded audit to stand. */
const AUDITED_FIELDS = ['current', 'containsCurrent', 'compatible', 'heldBy'];

/**
 * The differences between the recorded inventory and a live refresh of it:
 * an item whose current release, compatible release or holders changed since
 * the audit date, or that the live refresh no longer resolves.
 */
export function compareAudits(recorded, live) {
  const problems = [];
  const stored = new Map((recorded.items ?? []).map((item) => [item.id, item]));
  for (const item of live.items ?? []) {
    const entry = stored.get(item.id);
    if (!entry) continue;
    for (const field of AUDITED_FIELDS) {
      if (JSON.stringify(entry[field]) !== JSON.stringify(item[field])) {
        problems.push({
          kind: 'audit-outdated',
          message: `${item.id}: the audit of ${recorded.auditDate} records ${field} ${JSON.stringify(entry[field])}, the registries now report ${JSON.stringify(item[field])}; update the item and run node js/scripts/check-dependencies.mjs --refresh`,
        });
      }
    }
  }
  return problems;
}

// --- Refresh (online) -----------------------------------------------------

async function mapLimit(values, limit, task) {
  const results = new Array(values.length);
  let next = 0;
  let failed = false;
  // After the first failure no further task starts, so a failed live
  // comparison fails closed without querying the remaining registries.
  const worker = async () => {
    while (!failed && next < values.length) {
      const index = next;
      next += 1;
      try {
        results[index] = await task(values[index], index);
      } catch (error) {
        failed = true;
        throw error;
      }
    }
  };
  await Promise.all(Array.from({ length: Math.min(limit, values.length) }, worker));
  return results;
}

/**
 * The `http(url, headers)` of the refresh. A live comparison issues hundreds
 * of registry requests, so one dropped connection must not fail it: fetch
 * rejects with `TypeError: fetch failed` on network errors (reset, DNS,
 * timeout), and those are retried like HTTP 429 and 5xx answers. The final
 * error names the URL and the underlying cause.
 */
export function registryHttp({ fetch = globalThis.fetch, sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms)), attempts = 4, delayMs = 1000 } = {}) {
  return async function http(url, headers = {}) {
    for (let attempt = 1; ; attempt += 1) {
      let failure;
      try {
        const response = await fetch(url, { headers: { 'user-agent': 'meta-language-dependency-audit', ...headers } });
        if (response.ok) return await response.text();
        failure = new Error(`${url}: HTTP ${response.status}`);
        if (response.status !== 429 && response.status < 500) throw failure;
      } catch (error) {
        if (error === failure) throw error;
        const cause = error?.cause?.code ?? error?.cause?.message;
        failure = new Error(`${url}: ${error?.message ?? error}${cause ? ` (${cause})` : ''}`, { cause: error });
      }
      if (attempt >= attempts) throw failure;
      await sleep(delayMs * attempt);
    }
  };
}

function crateIndexPath(name) {
  const lower = name.toLowerCase();
  if (lower.length <= 2) return `${lower.length}/${lower}`;
  if (lower.length === 3) return `3/${lower[0]}/${lower}`;
  return `${lower.slice(0, 2)}/${lower.slice(2, 4)}/${lower}`;
}

/**
 * The registry queries of the refresh. `github(route)` answers a GitHub REST
 * route; `http(url, headers)` returns the response text.
 */
export function releaseResolvers({ http, github, today }) {
  const cache = new Map();
  const once = (key, compute) => {
    if (!cache.has(key)) cache.set(key, compute());
    return cache.get(key);
  };
  const githubRelease = (repository) =>
    once(`github ${repository}`, async () => {
      try {
        const release = await github(`repos/${repository}/releases/latest`);
        if (release?.tag_name && !isPrerelease(release.tag_name)) return release.tag_name;
      } catch {
        // No release: fall back to the newest stable tag.
      }
      const tags = await github(`repos/${repository}/tags?per_page=100`);
      return newestStable(tags.map((tag) => tag.name));
    });
  const nodeIndex = () => once('node index', async () => JSON.parse(await http('https://nodejs.org/dist/index.json')));
  const nodeSchedule = () => once('node schedule', async () => JSON.parse(await http('https://raw.githubusercontent.com/nodejs/Release/main/schedule.json')));
  return {
    npm: ({ package: name }) =>
      once(`npm ${name}`, async () => {
        const document = JSON.parse(await http(`https://registry.npmjs.org/${name.replace('/', '%2f')}`, { accept: 'application/vnd.npm.install-v1+json' }));
        const versions = Object.entries(document.versions ?? {}).filter(([, entry]) => !entry.deprecated).map(([version]) => version);
        return { current: document['dist-tags'].latest, evidence: `npm registry, ${name} latest`, versions };
      }),
    crate: ({ crate }) =>
      once(`crate ${crate}`, async () => {
        const lines = (await http(`https://index.crates.io/${crateIndexPath(crate)}`)).trim().split('\n').map((line) => JSON.parse(line));
        const versions = lines.filter((line) => !line.yanked).map((line) => line.vers);
        return { current: newestStable(versions), evidence: `crates.io, ${crate}`, versions };
      }),
    'github-release': async ({ repository }) => ({ current: await githubRelease(repository), evidence: `GitHub release, ${repository}` }),
    'github-revision': async ({ repository }, item) => {
      const current = await githubRelease(repository);
      const comparison = await github(`repos/${repository}/compare/${current}...${item.pinned}`);
      return {
        current,
        containsCurrent: ['identical', 'ahead'].includes(comparison.status),
        evidence: `GitHub release, ${repository} (the pin is ${comparison.status} of it)`,
      };
    },
    'node-lts': async () => {
      const release = (await nodeIndex()).find((entry) => entry.lts);
      return { current: release.version.replace(/^v/u, ''), evidence: `nodejs.org, newest LTS release (${release.lts})` };
    },
    'node-maintained': async () => {
      const lines = Object.entries(await nodeSchedule())
        .filter(([, dates]) => dates.lts && dates.lts <= today && dates.end > today)
        .map(([line]) => line.replace(/^v/u, ''))
        .sort((a, b) => Number(a) - Number(b));
      return { current: lines[0], evidence: `nodejs/Release schedule, oldest maintained LTS line (maintained: ${lines.join(', ')})` };
    },
    'docker-hub': ({ repository }) =>
      once(`docker ${repository}`, async () => {
        const page = JSON.parse(await http(`https://hub.docker.com/v2/repositories/${repository}/tags?page_size=100&ordering=last_updated`));
        const tags = page.results.map((tag) => tag.name).filter((name) => /^\d+\.\d+\.\d+$/u.test(name));
        return { current: newestStable(tags), evidence: `Docker Hub, ${repository} tags` };
      }),
    'rust-edition': async () => ({ current: RUST_EDITIONS.at(-1), evidence: 'Rust editions, the newest stable edition' }),
    none: async () => ({}),
    couplings: couplingResolvers({ github }),
  };
}

const githubFile = async (github, repository, file, ref) => {
  const document = await github(`repos/${repository}/contents/${file}${ref ? `?ref=${ref}` : ''}`);
  return Buffer.from(document.content, document.encoding ?? 'base64').toString('utf8');
};

/**
 * The constraint an opam file places on one package, at the top level of its
 * filter: `"ocaml" {>= "3.08.0" & (os != "cygwin" | < "5.0") & < "5.5.0~"}`
 * gives `>=3.08.0 <5.5.0`. Parenthesised alternatives are platform-specific
 * and dropped; a `~` suffix only admits the bound's prereleases.
 */
export function opamConstraint(opam, name) {
  const match = opam.match(new RegExp(`"${name}"\\s*\\{([^}]*)\\}`, 'u'));
  if (!match) return opam.includes(`"${name}"`) ? '' : null;
  let filter = match[1];
  while (/\([^()]*\)/u.test(filter)) filter = filter.replace(/\([^()]*\)/gu, '');
  return [...filter.matchAll(/(>=|<=|>|<|=)\s*"([^"]+)"/gu)].map(([, operator, version]) => `${operator}${version.replace(/~.*$/u, '')}`).join(' ');
}

/**
 * The resolvers of items a coupled artifact or toolchain holds rather than a
 * package requirement in a lockfile. Each returns the newest compatible
 * release, the requirements that exclude the current release, and the
 * reason, all queried live.
 */
export function couplingResolvers({ github }) {
  return {
    // The vendored web-tree-sitter runtime is built with the emscripten that
    // its tree-sitter release pins; another emscripten changes the runtime.
    'image emscripten/emsdk': async (item, find) => {
      const holder = find('npm', 'web-tree-sitter');
      if (!holder) return {};
      const tag = `v${holder.pinned}`;
      const pinned = (await githubFile(github, 'tree-sitter/tree-sitter', 'crates/loader/emscripten-version', tag)).trim();
      return {
        compatible: pinned,
        heldBy: [{ id: holder.id, requirement: `=${pinned}`, evidence: `tree-sitter/tree-sitter ${tag} crates/loader/emscripten-version` }],
        reason: `Held by tree-sitter ${tag}, whose \`crates/loader/emscripten-version\` pins emscripten ${pinned} for the web-tree-sitter ${holder.pinned} runtime; another emscripten produces a different runtime, so the image moves with tree-sitter.`,
      };
    },
    // Rocq builds with ocamlfind, and the newest ocamlfind release bounds the
    // OCaml compiler.
    'toolchain ocaml': async (item, find) => {
      const rocq = find('toolchain', 'rocq-core');
      if (!rocq) return {};
      const opam = (file) => githubFile(github, 'ocaml/opam-repository', `packages/${file}`);
      const findlib = opamConstraint(await opam(`rocq-runtime/rocq-runtime.${rocq.pinned}/opam`), 'ocamlfind');
      if (findlib === null) return {};
      const releases = (await github('repos/ocaml/opam-repository/contents/packages/ocamlfind'))
        .map((entry) => entry.name.replace(/^ocamlfind\./u, ''))
        .filter((version) => versionParts(version) && !isPrerelease(version));
      const newest = newestStable(releases);
      const admitted = newestStable(releases.filter((version) => findlib === '' || satisfies(version, findlib, 'npm')));
      const bound = opamConstraint(await opam(`ocamlfind/ocamlfind.${admitted}/opam`), 'ocaml');
      const upper = (bound ?? '').split(' ').filter((part) => part.startsWith('<')).join(' ');
      if (!upper) return {};
      const tags = (await github('repos/ocaml/ocaml/tags?per_page=100')).map((tag) => tag.name).filter((name) => /^\d+\.\d+\.\d+$/u.test(name));
      return {
        compatible: newestStable(tags.filter((version) => satisfies(version, upper, 'npm'))),
        heldBy: [{
          external: 'opam ocamlfind',
          version: admitted,
          newest,
          requirement: upper,
          requiredBy: rocq.id,
          evidence: `opam-repository, rocq-runtime ${rocq.pinned} requires ocamlfind ${findlib || 'any'}; ocamlfind ${admitted} requires ocaml ${bound}`,
        }],
        reason: `Held by ocamlfind ${admitted}, the newest ocamlfind release in opam-repository, which requires OCaml ${upper}; rocq-runtime ${rocq.pinned} needs ocamlfind (${findlib || 'any'}), so the Rocq acceptance job builds with the newest OCaml it admits and moves when ocamlfind admits a newer line.`,
      };
    },
  };
}

/** The newest `rust-version` any crate resolved for a manifest declares, from `cargo metadata`. */
export function requiredRustVersion(metadata) {
  const versions = metadata.packages.map((entry) => entry.rust_version).filter(Boolean);
  const newest = versions.sort((a, b) => compareVersions(b, a))[0];
  const holders = metadata.packages.filter((entry) => entry.rust_version === newest).map((entry) => `${entry.name} ${entry.version}`);
  return { current: newest, evidence: `cargo metadata, the highest rust-version of the resolved crates (${holders.slice(0, 3).join(', ')})` };
}

/**
 * The reason a locked package is behind when a dependent's requirement holds
 * it: the dependent is named with its requirement, or null when nothing in the
 * lockfile holds it (only the repository's own manifest can).
 */
export function heldReason(item, dependents, { syntax, rootName }) {
  const holders = dependents.filter(({ requirement }) => !satisfies(item.current, requirement, syntax));
  if (holders.length === 0 || holders.some(({ dependent }) => dependent === rootName)) return null;
  const names = holders.map(({ dependent, version, requirement }) => `\`${dependent}\` ${version} (\`${requirement}\`)`);
  return `Held by the requirement of ${names.join(', ')}, which does not admit ${item.current}; it moves when that dependent does.`;
}

function npmDependents(lock) {
  const dependents = new Map();
  for (const [key, entry] of Object.entries(lock.packages ?? {})) {
    const dependent = key === '' ? lock.name : key.slice(key.lastIndexOf('node_modules/') + 'node_modules/'.length);
    for (const section of ['dependencies', 'optionalDependencies', 'peerDependencies', 'devDependencies']) {
      if (key !== '' && section === 'devDependencies') continue;
      for (const [name, requirement] of Object.entries(entry[section] ?? {})) {
        const list = dependents.get(name) ?? [];
        list.push({ dependent, version: entry.version, requirement });
        dependents.set(name, list);
      }
    }
  }
  return dependents;
}

/**
 * The dependents of every resolved crate with their requirements, keyed by
 * crate name; each records the `target` version it resolves to. The edges come
 * from the resolve graph of `cargo metadata`, so a disabled optional
 * dependency or a dependency crate's own dev-dependency does not hold
 * anything. Metadata without a resolve graph falls back to every declared
 * dependency.
 */
export function cargoDependents(metadata) {
  const dependents = new Map();
  if (metadata.resolve?.nodes) {
    const packages = new Map(metadata.packages.map((entry) => [entry.id, entry]));
    for (const node of metadata.resolve.nodes) {
      const dependent = packages.get(node.id);
      for (const edge of node.deps ?? []) {
        const target = packages.get(edge.pkg);
        if (!dependent || !target) continue;
        const kinds = new Set((edge.dep_kinds ?? [{ kind: null }]).map(({ kind }) => kind ?? 'normal'));
        const requirements = dependent.dependencies
          .filter((dependency) => dependency.name === target.name && kinds.has(dependency.kind ?? 'normal') && satisfies(target.version, dependency.req, 'cargo'))
          .map(({ req }) => req);
        const list = dependents.get(target.name) ?? [];
        for (const requirement of new Set(requirements)) list.push({ dependent: dependent.name, version: dependent.version, requirement, target: target.version });
        dependents.set(target.name, list);
      }
    }
    return dependents;
  }
  for (const entry of metadata.packages) {
    for (const dependency of entry.dependencies) {
      const key = dependency.name;
      const list = dependents.get(key) ?? [];
      list.push({ dependent: entry.name, version: entry.version, requirement: dependency.req });
      dependents.set(key, list);
    }
  }
  return dependents;
}

/**
 * Rewrites the inventory from the collected items and the registries: the
 * current stable release of every item, the reason of every item a
 * dependent's requirement holds, and the recorded reasons of the items still
 * behind. `cargoMetadata(manifest)` returns `cargo metadata` for a manifest.
 */
export async function refreshInventory({ root, previous, collected, resolvers, cargoMetadata, today, onProgress = () => {} }) {
  const prior = new Map((previous?.items ?? []).map((item) => [item.id, item]));
  const nameKey = (item) => `${item.category} ${item.scope ?? ''} ${item.name}`;
  const priorByName = new Map((previous?.items ?? []).map((item) => [nameKey(item), item]));
  // A generator names the tools it runs by id, and an id carries the pin: follow a tool to its new pin.
  const collectedByName = new Map(collected.map((item) => [nameKey(item), item]));
  const followPin = (id) => (prior.has(id) ? (collectedByName.get(nameKey(prior.get(id)))?.id ?? id) : id);
  let done = 0;
  const releases = new Map();
  const items = await mapLimit(collected, 16, async (item) => {
    const before = prior.get(item.id) ?? priorByName.get(nameKey(item));
    const resolver = resolvers[item.source.type];
    if (!resolver && item.source.type !== 'rust-required') throw new Error(`${item.id}: no resolver for ${item.source.type}`);
    const { versions, ...resolved } = item.source.type === 'rust-required' ? requiredRustVersion(cargoMetadata(item.source.manifest)) : await resolver(item.source, item);
    if (versions) releases.set(item.id, versions);
    done += 1;
    onProgress(done, collected.length, item.id);
    const entry = { ...item, ...resolved };
    if (item.category === 'generator') entry.dependsOn = (prior.get(item.id)?.dependsOn ?? []).map(followPin);
    if (before?.note) entry.note = before.note;
    if (before?.reason && !before.reasonHeld && prior.has(item.id)) entry.reason = before.reason;
    return entry;
  });
  const manifests = { 'js/package-lock.json': { syntax: 'npm', lock: readJson(root, 'js/package-lock.json') } };
  const npmHolders = npmDependents(manifests['js/package-lock.json'].lock);
  const cargoHolders = new Map();
  const ids = new Set(items.map(({ id }) => id));
  const find = (category, name) => items.find((entry) => entry.category === category && (entry.source?.package ?? entry.name) === name);
  for (const item of items) {
    delete item.compatible;
    delete item.heldBy;
    if (!needsReason(item)) {
      delete item.reason;
      delete item.reasonHeld;
      continue;
    }
    let reason = null;
    let holders = [];
    let syntax = 'npm';
    if (item.category === 'npm') {
      holders = (npmHolders.get(item.source.package) ?? []).filter(({ requirement }) => satisfies(item.pinned, requirement, 'npm'));
      reason = heldReason(item, holders, { syntax, rootName: manifests['js/package-lock.json'].lock.name });
    } else if (item.category === 'crate') {
      syntax = 'cargo';
      const manifest = item.scope.replace(/Cargo\.lock$/u, 'Cargo.toml');
      if (!cargoHolders.has(manifest)) cargoHolders.set(manifest, cargoDependents(cargoMetadata(manifest)));
      holders = (cargoHolders.get(manifest).get(item.name) ?? [])
        .filter(({ requirement, target }) => (target ? target === item.pinned : satisfies(item.pinned, requirement, 'cargo')));
      const rootName = parseCargoManifest(readText(root, manifest)).settings.name;
      reason = heldReason(item, holders, { syntax, rootName });
    } else if (resolvers.couplings?.[`${item.category} ${item.name}`]) {
      const coupled = await resolvers.couplings[`${item.category} ${item.name}`](item, find);
      if (coupled.compatible && coupled.heldBy?.length) {
        Object.assign(item, { compatible: coupled.compatible, heldBy: coupled.heldBy });
        reason = coupled.reason;
      }
    }
    // A lockfile holder is an inventoried package of the same lockfile; the
    // newest compatible release satisfies every dependent's requirement.
    if (reason && holders.length > 0 && releases.has(item.id)) {
      const admitted = releases.get(item.id).filter((version) => !isPrerelease(version) && holders.every(({ requirement }) => satisfies(version, requirement, syntax)));
      const heldBy = holders
        .filter(({ requirement }) => !satisfies(item.current, requirement, syntax))
        .map(({ dependent, version, requirement }) => ({ id: itemId({ category: item.category, scope: item.scope, name: dependent, pinned: version }), requirement }))
        .filter(({ id }) => ids.has(id));
      const compatible = newestStable(admitted);
      if (compatible && heldBy.length > 0) Object.assign(item, { compatible, heldBy });
    }
    if (reason) {
      item.reason = reason;
      item.reasonHeld = true;
    } else {
      delete item.reasonHeld;
    }
  }
  return { auditDate: today, items: items.sort(compareItems) };
}

// --- Audit document -------------------------------------------------------

const code = (text) => (text === undefined || text === null || text === '' ? '' : `\`${String(text).replaceAll('|', '\\|')}\``);
const cell = (text) => String(text ?? '').replaceAll('|', '\\|').replaceAll('\n', ' ');

function reasonFor(item, status, byId) {
  if (item.reason) return item.reason;
  if (status !== 'behind') return '';
  const behindParents = (item.dependsOn ?? []).filter((id) => byId.get(id) && statusOf(byId, id) === 'behind');
  return behindParents.length ? `Built from ${behindParents.map(code).join(', ')}, which is behind (see its reason).` : '';
}

let statusCache = null;
const statusOf = (byId, id) => statusCache.get(id);

/** Renders docs/dependency-audit.md from the inventory. */
export function renderAuditDocument(inventory) {
  const items = inventory.items;
  statusCache = statuses(items);
  const byId = new Map(items.map((item) => [item.id, item]));
  const lines = [];
  const push = (...text) => lines.push(...text);
  push(
    '> This document is subordinate to the authoritative',
    '> [vision and architecture specification](vision.md)',
    '> and is the dependency inventory that its Dependencies section refers to.',
    '',
    '<!-- Generated from parity/dependency-inventory.json by `npm run dependencies:refresh` (js/scripts/check-dependencies.mjs). Do not edit by hand. -->',
    '',
    '# Dependency audit',
    '',
    `Audit date: ${inventory.auditDate}`,
    '',
    'This inventory lists every runtime, development, build and optional dependency',
    'of the two published packages and the website crate, every lockfile',
    'resolution, the experiment manifests, the vendored grammar assets, the',
    'generators, the toolchains, the GitHub Actions, the build images, the runners',
    'and the published artifact contents, each with the current stable release its',
    'source reported on the audit date.',
    '',
    '## Method',
    '',
    '`node js/scripts/check-dependencies.mjs --refresh` (`npm run dependencies:refresh`)',
    'reads every pin from the repository, queries the npm registry, the crates.io',
    'index, GitHub releases, nodejs.org and Docker Hub for the current stable',
    'release, derives the reason of a locked package that a dependent\'s requirement',
    'holds from `js/package-lock.json` and `cargo metadata`, and rewrites',
    '`parity/dependency-inventory.json` and this document. Every other reason is',
    'recorded by hand in the inventory.',
    '',
    '`npm run check:dependencies`, which CI runs, is offline. It fails when the',
    'repository declares an item the inventory does not list, or a pin the',
    'inventory does not record; when the audit date is missing or invalid; and when',
    'a retained item is behind its current stable release without a recorded',
    'compatibility reason.',
    '',
    '`npm run check:dependencies:delivery` is the delivery gate. A recorded reason',
    'does not count as an upgrade: an item behind its current stable release is',
    'delivered only at its newest compatible release, the newest stable release',
    'that every requirement holding it admits, which the refresh records with the',
    'holders whose requirements exclude the current release. Each holder must be an',
    'inventoried item that is itself delivered, or an external package at its',
    'newest release; anything else, and anything built from it, is stale. In',
    'acceptance and CI runs (and with `--live`) the gate refreshes the inventory',
    'from the registries in memory and fails when a current or compatible release',
    'differs from this audit, so a release published after the audit date fails',
    'delivery until the item moves to it.',
    '',
    'Comparisons: `version` means the pin must be at least the current release;',
    '`major` and `minor` mean a moving tag (`v7`, `5.4`) that must name the current',
    'release line; `floor` means a supported minimum (the `engines` floor must be a',
    'maintained Node.js line, `rust-version` must be at least what the resolved',
    'crates declare); `revision` means a vendored commit that must contain the',
    'upstream release; `floating` means the newest release is installed on every',
    'run; `derived` means an artifact built from the items it names, behind when',
    'they are.',
    '',
    '## Summary',
    '',
    '| Category | Items | Current | Behind | Not applicable |',
    '|---|---|---|---|---|',
  );
  for (const { id, title } of CATEGORIES) {
    const members = items.filter((item) => item.category === id);
    if (members.length === 0) continue;
    const count = (status) => members.filter((item) => statusCache.get(item.id) === status).length;
    push(`| ${title} | ${members.length} | ${count('current')} | ${count('behind')} | ${count('not applicable')} |`);
  }
  push('');
  for (const { id, title } of CATEGORIES) {
    const members = items.filter((item) => item.category === id);
    if (members.length === 0) continue;
    push(`## ${title}`, '');
    const scoped = members.some((item) => item.scope);
    const detail = members.some((item) => item.role || item.requirement || item.kind);
    const header = ['Item', ...(scoped ? ['Scope'] : []), 'Declared in', 'Pinned', ...(detail ? ['Role', 'Requirement'] : []), 'Current stable release', 'Comparison', 'Status', 'Reason'];
    push(`| ${header.join(' | ')} |`, `|${header.map(() => '---').join('|')}|`);
    for (const item of members) {
      const status = statusCache.get(item.id);
      const row = [
        code(item.name),
        ...(scoped ? [code(item.scope)] : []),
        item.declaredIn.map(code).join(', '),
        code(item.pinned),
        ...(detail ? [cell([item.role, item.kind].filter(Boolean).join(', ')), code(item.requirement)] : []),
        cell([item.current, item.evidence ? `(${item.evidence})` : ''].filter(Boolean).join(' ')),
        item.compare,
        status,
        cell([reasonFor(item, status, byId), item.note].filter(Boolean).join(' ')),
      ];
      push(`| ${row.join(' | ')} |`);
    }
    push('');
  }
  const behind = items.filter((item) => needsReason(item));
  const delivered = deliveredStatuses(items);
  push('## Behind the current stable release', '');
  if (behind.length === 0) push('No retained item is behind its current stable release.');
  else {
    const held = behind.filter((item) => delivered.get(item.id).status === 'compatible').length;
    push(
      `${behind.length} retained items are behind their current stable release on ${inventory.auditDate}, each for the recorded reason.`,
      `${held} of them are at their newest compatible release, verified against the requirements that hold them;`,
      `${behind.length - held} are stale and fail the delivery check.`,
      '',
    );
    for (const item of behind) {
      const verdict = delivered.get(item.id);
      const holders = (item.heldBy ?? []).map((holder) => `${code(holder.id ?? `${holder.external} ${holder.version}`)} (${code(holder.requirement)})`).join(', ');
      const delivery = verdict.status === 'compatible'
        ? `Delivered at its newest compatible release ${code(item.compatible)}, held by ${holders}.`
        : `Stale: ${verdict.cause}.`;
      push(`- ${code(item.name)} ${code(item.pinned)} → ${code(item.current)} (${item.declaredIn.map(code).join(', ')}): ${item.reason ?? 'no recorded reason'} ${delivery}`);
    }
  }
  push('');
  statusCache = null;
  return lines.join('\n');
}
