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

// --- Refresh (online) -----------------------------------------------------

async function mapLimit(values, limit, task) {
  const results = new Array(values.length);
  let next = 0;
  const worker = async () => {
    while (next < values.length) {
      const index = next;
      next += 1;
      results[index] = await task(values[index], index);
    }
  };
  await Promise.all(Array.from({ length: Math.min(limit, values.length) }, worker));
  return results;
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
        return { current: document['dist-tags'].latest, evidence: `npm registry, ${name} latest` };
      }),
    crate: ({ crate }) =>
      once(`crate ${crate}`, async () => {
        const lines = (await http(`https://index.crates.io/${crateIndexPath(crate)}`)).trim().split('\n').map((line) => JSON.parse(line));
        return { current: newestStable(lines.filter((line) => !line.yanked).map((line) => line.vers)), evidence: `crates.io, ${crate}` };
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

function cargoDependents(metadata) {
  const dependents = new Map();
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
  const items = await mapLimit(collected, 16, async (item) => {
    const before = prior.get(item.id) ?? priorByName.get(nameKey(item));
    const resolver = resolvers[item.source.type];
    if (!resolver && item.source.type !== 'rust-required') throw new Error(`${item.id}: no resolver for ${item.source.type}`);
    const resolved = item.source.type === 'rust-required' ? requiredRustVersion(cargoMetadata(item.source.manifest)) : await resolver(item.source, item);
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
  for (const item of items) {
    if (!needsReason(item)) {
      delete item.reason;
      delete item.reasonHeld;
      continue;
    }
    let reason = null;
    if (item.category === 'npm') {
      reason = heldReason(item, npmHolders.get(item.source.package) ?? [], { syntax: 'npm', rootName: manifests['js/package-lock.json'].lock.name });
    } else if (item.category === 'crate') {
      const manifest = item.scope.replace(/Cargo\.lock$/u, 'Cargo.toml');
      if (!cargoHolders.has(manifest)) cargoHolders.set(manifest, cargoDependents(cargoMetadata(manifest)));
      const holders = (cargoHolders.get(manifest).get(item.name) ?? []).filter(({ requirement }) => satisfies(item.pinned, requirement, 'cargo'));
      const rootName = parseCargoManifest(readText(root, manifest)).settings.name;
      reason = heldReason(item, holders, { syntax: 'cargo', rootName });
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
  push('## Behind the current stable release', '');
  if (behind.length === 0) push('No retained item is behind its current stable release.');
  else {
    push(`${behind.length} retained items are behind their current stable release on ${inventory.auditDate}, each for the recorded reason:`, '');
    for (const item of behind) push(`- ${code(item.name)} ${code(item.pinned)} → ${code(item.current)} (${item.declaredIn.map(code).join(', ')}): ${item.reason ?? 'no recorded reason'}`);
  }
  push('');
  statusCache = null;
  return lines.join('\n');
}
