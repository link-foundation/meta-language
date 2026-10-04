// I195-DEPENDENCY-CURRENT-STABLE-DELIVERY: every retained dependency, transitive
// resolution, toolchain, action, image and version-coupled artifact the
// repository delivers is at its current stable release, or at the verified
// newest release its recorded holders admit; the delivery gate rejects
// anything else (docs/vision.md#dependencies). Every assertion reads the
// committed inventory and the repository files it describes.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { gunzipSync } from 'node:zlib';

import {
  INVENTORY_FILE,
  checkDeliveredDependencies,
  collectDependencies,
  compareAudits,
  deliveredStatuses,
  ownStatus,
  parseCargoLock,
} from '../scripts/dependency-inventory.mjs';
import { grammarFile } from '../scripts/grammar-files.mjs';
import { compareVersions, isPrerelease, satisfies } from '../scripts/semver-range.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-DEPENDENCY-CURRENT-STABLE-DELIVERY';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const read = (file) => readFileSync(path.join(root, file), 'utf8');
const readJson = (file) => JSON.parse(read(file));
const inventory = readJson(INVENTORY_FILE);
const byId = new Map(inventory.items.map((item) => [item.id, item]));
const verdicts = deliveredStatuses(inventory.items);
const collectedOf = (items) => items.map(({ current, evidence, reason, ...item }) => item);

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    runtime: 'tooling',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: INVENTORY_FILE,
    assertions,
    testName,
  });
}

// The lockfile a clean consumer installs from the committed npm resolution:
// the package itself under node_modules/meta-language, with every package the
// repository lock resolves. The acceptance workflow checks the lockfile of a
// real clean install of the packed artifact instead.
function consumerLock() {
  const lock = readJson('js/package-lock.json');
  const { '': own, ...packages } = lock.packages;
  return {
    name: 'consumer',
    lockfileVersion: lock.lockfileVersion,
    packages: {
      '': { dependencies: { 'meta-language': own.version } },
      'node_modules/meta-language': { version: own.version, dependencies: own.dependencies ?? {} },
      ...packages,
    },
  };
}

const staleMessages = (problems) => problems.filter(({ kind }) => kind === 'stale-delivered-dependency').map(({ message }) => message);
const rejected = (problems, id) => staleMessages(problems).some((message) => message.startsWith(`${id}:`));
const deliver = (mutation) => checkDeliveredDependencies(mutation, collectedOf(mutation.items), { npmConsumerLock: consumerLock() });
const item = (items, predicate, label) => {
  const found = items.find(predicate);
  assert.ok(found, `the inventory records ${label}`);
  return found;
};

test('every retained item the repository delivers is current or at its verified newest compatible release', () => {
  assert.match(inventory.auditDate, /^\d{4}-\d{2}-\d{2}$/u);
  assert.equal(inventory.auditDate <= new Date().toISOString().slice(0, 10), true, 'the audit date is not in the future');
  assert.deepEqual(checkDeliveredDependencies(inventory, collectDependencies(root), { npmConsumerLock: consumerLock() }), []);
  const counts = { current: 0, compatible: 0, 'not applicable': 0, behind: 0 };
  for (const entry of inventory.items) {
    const { status } = verdicts.get(entry.id);
    counts[status] += 1;
    if (status === 'compatible') {
      // A compatible item is pinned at the release its holders admit, never a
      // reason over an older pin.
      assert.equal(ownStatus(entry), 'behind', `${entry.id} is only compatible because it is behind`);
      assert.equal(compareVersions(entry.pinned, entry.compatible, { major: 1, minor: 2 }[entry.compare] ?? 3) >= 0, true,
        `${entry.id} is pinned at its newest compatible release ${entry.compatible}`);
      assert.equal(isPrerelease(entry.compatible), false, `${entry.id}: ${entry.compatible} is stable`);
      assert.equal(entry.reasonHeld, true, `${entry.id}: its reason was derived from the registries, not written by hand`);
    }
    if (!['floating', 'unversioned', 'derived'].includes(entry.compare)) {
      assert.equal(typeof entry.current, 'string', `${entry.id} records its current stable release`);
      if (entry.compare !== 'revision') assert.equal(isPrerelease(entry.pinned), false, `${entry.id} pins a stable release`);
    }
  }
  assert.equal(counts.behind, 0);
  assert.equal(counts.current + counts.compatible + counts['not applicable'], inventory.items.length);
  assert.equal(counts.current > counts.compatible, true);
  observe(['allRetainedItemsCurrent'], 'every retained item the repository delivers is current or at its verified newest compatible release');
});

test('the gate accepts the committed delivery offline in CI and goes live, failing closed without the registries, only with --live', () => {
  const directory = mkdtempSync(path.join(os.tmpdir(), 'dependency-delivery-'));
  try {
    const lock = path.join(directory, 'package-lock.json');
    writeFileSync(lock, JSON.stringify(consumerLock()));
    const script = path.join(root, 'js/scripts/check-dependencies.mjs');
    const run = (args, env) => spawnSync(process.execPath, [script, ...args], {
      encoding: 'utf8',
      env: { ...process.env, CI: '', ACCEPTANCE_CHECKPOINT: '', ...env },
    });
    const offline = run(['--delivery', '--offline', '--consumer-lock', lock], { CI: 'true' });
    assert.equal(offline.status, 0, offline.stderr);
    assert.match(offline.stdout, /delivered current or at the newest compatible release/u);
    assert.doesNotMatch(offline.stdout, /live registries/u);
    // A pull request check never goes live implicitly: CI alone keeps the
    // offline comparison against the committed audit.
    const implicit = run(['--delivery', '--consumer-lock', lock], { CI: 'true', ACCEPTANCE_CHECKPOINT: 'pre-merge', PATH: '' });
    assert.equal(implicit.status, 0, implicit.stderr);
    assert.doesNotMatch(implicit.stdout, /live registries/u);
    // With --live (main only) the gate compares the audit with the live
    // registries; without gh and cargo it cannot, and it fails rather than
    // trusting the recorded audit.
    const unreachable = run(['--delivery', '--live', '--consumer-lock', lock], { CI: 'true', PATH: '' });
    assert.equal(unreachable.status, 1);
    assert.match(unreachable.stderr, /^live-registry-unavailable: /mu);
    assert.doesNotMatch(unreachable.stderr, /stale-delivered-dependency/u);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
  observe(['allRetainedItemsCurrent', 'staleDeliveredItemRejected'],
    'the gate accepts the committed delivery offline in CI and goes live, failing closed without the registries, only with --live');
});

test('transitive npm and Cargo resolutions are current or held by a delivered dependent that excludes the current release', () => {
  const transitive = inventory.items.filter(({ category, role }) => ['npm', 'crate'].includes(category) && role === 'transitive');
  assert.equal(transitive.some(({ category }) => category === 'npm'), true);
  assert.equal(transitive.some(({ category }) => category === 'crate'), true);
  for (const entry of transitive) {
    assert.equal(['current', 'compatible'].includes(verdicts.get(entry.id).status), true, `${entry.id}: ${verdicts.get(entry.id).cause}`);
  }
  const npmLock = readJson('js/package-lock.json');
  const cargoLocks = new Map();
  const cargoLock = (scope) => {
    if (!cargoLocks.has(scope)) cargoLocks.set(scope, parseCargoLock(read(scope)));
    return cargoLocks.get(scope);
  };
  const held = inventory.items.filter(({ category, heldBy }) => ['npm', 'crate'].includes(category) && heldBy);
  assert.equal(held.length > 0, true, 'the audit records held lockfile items');
  for (const entry of held) {
    assert.equal(verdicts.get(entry.id).status, 'compatible', entry.id);
    const syntax = entry.category === 'crate' ? 'cargo' : 'npm';
    for (const holder of entry.heldBy) {
      const dependent = byId.get(holder.id);
      assert.ok(dependent, `${entry.id}: its holder ${holder.id} is inventoried`);
      assert.equal(dependent.scope, entry.scope, `${holder.id} resolves in the same lockfile`);
      assert.equal(['current', 'compatible'].includes(verdicts.get(holder.id).status), true, `${holder.id} is itself delivered`);
      assert.equal(satisfies(entry.pinned, holder.requirement, syntax), true, `${holder.id} ${holder.requirement} admits the pin ${entry.pinned}`);
      assert.equal(satisfies(entry.current, holder.requirement, syntax), false, `${holder.id} ${holder.requirement} excludes ${entry.current}`);
      // The holder really depends on the held resolution in the lockfile.
      if (entry.category === 'crate') {
        const packages = cargoLock(entry.scope);
        const own = packages.find(({ name, version }) => name === dependent.name && version === dependent.pinned);
        assert.ok(own, `${entry.scope} resolves ${dependent.name} ${dependent.pinned}`);
        const versions = packages.filter(({ name }) => name === entry.name).map(({ version }) => version);
        const edge = own.dependencies.map((text) => text.split(' ')).find(([name, version]) =>
          name === entry.name && (version ? version === entry.pinned : versions.length === 1 && versions[0] === entry.pinned));
        assert.ok(edge, `${dependent.name} ${dependent.pinned} depends on ${entry.name} ${entry.pinned} in ${entry.scope}`);
      } else {
        const location = Object.entries(npmLock.packages).find(([key, value]) =>
          key.endsWith(`node_modules/${dependent.source.package}`) && value.version === dependent.pinned);
        assert.ok(location, `js/package-lock.json resolves ${dependent.source.package} ${dependent.pinned}`);
        assert.equal(location[1].dependencies?.[entry.source.package] ?? location[1].peerDependencies?.[entry.source.package], holder.requirement);
      }
    }
  }
  observe(['transitiveResolutionsCurrent'], 'transitive npm and Cargo resolutions are current or held by a delivered dependent that excludes the current release');
});

test('toolchains, workflow actions, runners and build images are current, and the held ones are held by delivered pins', () => {
  const tools = inventory.items.filter(({ category }) => ['toolchain', 'action', 'image', 'runner', 'engine', 'setting'].includes(category));
  for (const category of ['toolchain', 'action', 'image', 'runner']) {
    assert.equal(tools.some((entry) => entry.category === category), true, `the inventory records ${category} items`);
  }
  for (const entry of tools) {
    const { status, cause } = verdicts.get(entry.id);
    assert.notEqual(status, 'behind', `${entry.id}: ${cause}`);
    // Only an unversioned or floating pin has no release to compare with.
    if (status === 'not applicable') assert.equal(['unversioned', 'floating'].includes(entry.compare), true, entry.id);
  }
  // Every workflow pins the Rust toolchain the audit found current.
  const rust = item(tools, ({ category, name, compare }) => category === 'action' && name === 'dtolnay/rust-toolchain' && compare === 'version', 'the pinned Rust toolchain');
  assert.equal(rust.pinned, rust.current);
  assert.equal(verdicts.get(rust.id).status, 'current');
  for (const workflow of ['.github/workflows/issue-195-acceptance.yml']) {
    for (const [, version] of read(workflow).matchAll(/dtolnay\/rust-toolchain@(\d[^\s]*)/gu)) assert.equal(version, rust.pinned, workflow);
  }
  // OCaml is held by the newest ocamlfind, which Rocq requires.
  const ocaml = item(tools, ({ category, name }) => category === 'toolchain' && name === 'ocaml', 'the OCaml toolchain');
  if (verdicts.get(ocaml.id).status === 'compatible') {
    const [ocamlfind] = ocaml.heldBy;
    assert.equal(ocamlfind.version, ocamlfind.newest, 'the holder is the newest ocamlfind');
    assert.equal(verdicts.get(ocamlfind.requiredBy).status, 'current', `${ocamlfind.requiredBy} is current`);
    assert.equal(satisfies(ocaml.compatible, ocamlfind.requirement, 'npm'), true);
    assert.equal(satisfies(ocaml.current, ocamlfind.requirement, 'npm'), false);
  }
  // emscripten is the one the delivered web-tree-sitter release pins.
  const emsdk = item(tools, ({ category, name }) => category === 'image' && name === 'emscripten/emsdk', 'the emscripten image');
  if (verdicts.get(emsdk.id).status === 'compatible') {
    assert.equal(emsdk.compatible, emsdk.pinned);
    const [tree] = emsdk.heldBy;
    assert.equal(tree.requirement, `=${emsdk.pinned}`);
    assert.equal(verdicts.get(tree.id).status, 'current', `${tree.id} is current`);
    assert.match(tree.evidence, new RegExp(`v${byId.get(tree.id).pinned} crates/loader/emscripten-version$`, 'u'));
  }
  observe(['buildToolsAndImagesCurrent'], 'toolchains, workflow actions, runners and build images are current, and the held ones are held by delivered pins');
});

test('version-coupled artifacts are regenerated from the delivered pins', () => {
  const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
  const crates = new Map(inventory.items.filter(({ scope }) => scope === 'rust/Cargo.lock').map((entry) => [entry.name, entry]));
  const lock = readJson('rust/src/data/grammar-lock.json');
  assert.equal(read('js/src/vendor/grammars/grammar-lock.json'), read('rust/src/data/grammar-lock.json'), 'both runtimes ship one grammar lock');
  const cli = item(inventory.items, ({ category, name }) => category === 'toolchain' && name === 'tree-sitter-cli', 'the tree-sitter CLI');
  const runtime = item(inventory.items, ({ category, source }) => category === 'npm' && source.package === 'web-tree-sitter', 'web-tree-sitter');
  const binding = crates.get('tree-sitter');
  assert.equal(lock.treeSitterCli, `tree-sitter ${cli.pinned}`);
  assert.deepEqual(lock.treeSitterRuntime, { rust: binding.pinned, javascript: runtime.pinned });
  for (const entry of [cli, runtime, binding]) assert.equal(verdicts.get(entry.id).status, 'current', entry.id);
  const grammars = Object.values(lock.grammars);
  assert.equal(grammars.length > 0, true);
  for (const grammar of grammars) {
    const vendored = item(inventory.items, ({ category, name }) => category === 'vendored-grammar' && name === grammarFile(grammar, `${grammar.id}.wasm.gz`), `the ${grammar.id} grammar build`);
    assert.equal(vendored.pinned, grammar.version);
    const source = grammar.crate ? crates.get(grammar.crate) : byId.get(vendored.dependsOn.find((id) => id.startsWith(`vendored-parser ${grammar.vendored}@`)));
    assert.ok(source, `${grammar.id} is built from an inventoried source`);
    assert.equal(source.pinned, grammar.version, `${grammar.id} is built from the delivered ${source.id}`);
    assert.deepEqual([...vendored.dependsOn].sort(), [source.id, cli.id].sort());
    assert.equal(verdicts.get(vendored.id).status, 'current', `${vendored.id}: ${verdicts.get(vendored.id).cause}`);
    const wasm = gunzipSync(readFileSync(path.join(root, grammarFile(grammar, `${grammar.id}.wasm.gz`))));
    assert.equal(sha256(wasm), grammar.wasmSha256, `${grammar.id}.wasm.gz is the recorded build`);
  }
  const runtimeLock = readJson('js/src/vendor/web-tree-sitter/runtime-lock.json');
  const vendoredRuntime = item(inventory.items, ({ category }) => category === 'vendored-runtime', 'the vendored runtime');
  const emsdk = item(inventory.items, ({ category, name }) => category === 'image' && name === 'emscripten/emsdk', 'the emscripten image');
  assert.equal(runtimeLock.version, runtime.pinned);
  assert.equal(vendoredRuntime.pinned, runtime.pinned);
  assert.equal(runtimeLock.emscripten, emsdk.pinned);
  assert.equal(vendoredRuntime.dependsOn.includes(emsdk.id), true);
  assert.equal(verdicts.get(vendoredRuntime.id).status, 'current');
  assert.equal(sha256(gunzipSync(readFileSync(path.join(root, 'js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz')))), runtimeLock.wasmSha256);
  assert.equal(sha256(readFileSync(path.join(root, runtimeLock.patch))), runtimeLock.patchSha256);
  observe(['versionCoupledArtifactsRegenerated'], 'version-coupled artifacts are regenerated from the delivered pins');
});

test('the gate rejects a stale delivered item, a stale holder, an unverified hold and an outdated audit', () => {
  const clone = () => structuredClone(inventory);
  const find = (copy, category, name) => item(copy.items, (entry) => entry.category === category && entry.name === name, `${category} ${name}`);
  const heldCrate = inventory.items.find(({ category, heldBy }) => category === 'crate' && heldBy?.every(({ id }) => id));
  assert.ok(heldCrate, 'the inventory holds a crate');
  const crate = (copy) => copy.items.find(({ id }) => id === heldCrate.id);

  // The committed inventory passes; each mutation below fails it.
  assert.deepEqual(staleMessages(deliver(clone())), []);

  // A pin behind its current release with only a compatibility reason.
  let copy = clone();
  const direct = item(copy.items, ({ category, role, compare, heldBy }) => category === 'crate' && role === 'direct' && compare === 'version' && !heldBy, 'a direct crate');
  direct.current = `${Number(direct.pinned.split('.')[0]) + 1}.0.0`;
  direct.reason = 'The next major release needs integration work, so this pin intentionally stays behind.';
  assert.equal(rejected(deliver(copy), direct.id), true, 'a reason alone does not deliver a stale pin');

  // A held crate without the verified newest compatible release and holders.
  copy = clone();
  delete crate(copy).compatible;
  delete crate(copy).heldBy;
  assert.equal(rejected(deliver(copy), heldCrate.id), true, 'a hold is verified, not asserted');

  // A newer release the holders admit makes the pin stale.
  copy = clone();
  const [major, minor, patch] = heldCrate.pinned.split(/[.+]/u).map(Number);
  crate(copy).compatible = `${major}.${minor}.${patch + 1}`;
  const newer = deliver(copy);
  if (heldCrate.heldBy.every(({ requirement }) => satisfies(crate(copy).compatible, requirement, 'cargo'))
    && compareVersions(crate(copy).compatible, heldCrate.current) <= 0) {
    assert.equal(rejected(newer, heldCrate.id), true, 'a pin behind its newest compatible release is stale');
  }

  // A hold that does not exclude the current release.
  copy = clone();
  crate(copy).heldBy = crate(copy).heldBy.map((holder) => ({ ...holder, requirement: `>=${heldCrate.pinned}` }));
  assert.equal(rejected(deliver(copy), heldCrate.id), true, 'a holder must exclude the current release');

  // A stale holder makes everything it holds stale.
  copy = clone();
  const holder = copy.items.find(({ id }) => id === heldCrate.heldBy[0].id);
  holder.current = `${Number(holder.pinned.split('.')[0]) + 1}.0.0`;
  delete holder.compatible;
  delete holder.heldBy;
  const cascade = deliver(copy);
  assert.equal(rejected(cascade, holder.id), true);
  assert.equal(staleMessages(cascade).some((message) => message.startsWith(`${heldCrate.id}:`) && message.includes(`the holder ${holder.id} is itself stale`)), true);

  // An external holder behind its newest release does not hold anything.
  copy = clone();
  const ocaml = find(copy, 'toolchain', 'ocaml');
  if (ocaml.heldBy) {
    ocaml.heldBy[0].newest = ocaml.heldBy[0].version.replace(/\d+$/u, (digit) => String(Number(digit) + 1));
    assert.equal(rejected(deliver(copy), ocaml.id), true, 'ocaml is held only by the newest ocamlfind');
  }

  // emscripten held by a release other than the delivered tree-sitter's, and the runtime built with it.
  copy = clone();
  const emsdk = find(copy, 'image', 'emscripten/emsdk');
  if (emsdk.heldBy) {
    emsdk.heldBy[0].requirement = `=${emsdk.pinned.replace(/\d+$/u, (digit) => String(Number(digit) + 1))}`;
    const coupled = deliver(copy);
    assert.equal(rejected(coupled, emsdk.id), true);
    const vendoredRuntime = copy.items.find(({ category }) => category === 'vendored-runtime');
    assert.equal(rejected(coupled, vendoredRuntime.id), true, 'the runtime built with a stale emscripten is stale');
  }

  // A consumer that installs an older runtime dependency than the audit's current release.
  const runtimePackage = inventory.items.find(({ category, kind, role }) => category === 'npm' && kind === 'runtime' && role === 'direct');
  if (runtimePackage) {
    const lock = consumerLock();
    const location = Object.keys(lock.packages).find((key) => key.endsWith(`node_modules/${runtimePackage.source.package}`));
    lock.packages[location] = { ...lock.packages[location], version: '0.0.1' };
    assert.equal(checkDeliveredDependencies(inventory, collectDependencies(root), { npmConsumerLock: lock })
      .some(({ kind, message }) => kind === 'stale-consumer-resolution' && message.startsWith(`${runtimePackage.source.package}:`)), true);
  }

  // A live refresh that disagrees with the recorded audit.
  assert.deepEqual(compareAudits(inventory, inventory), []);
  copy = clone();
  const moved = copy.items.find(({ id }) => id === direct.id);
  moved.current = `${moved.current}.1`;
  crate(copy).heldBy = [];
  const outdated = compareAudits(inventory, copy);
  assert.equal(outdated.every(({ kind }) => kind === 'audit-outdated'), true);
  assert.deepEqual(outdated.map(({ message }) => message.split(':')[0]).sort(), [direct.id, heldCrate.id].sort());
  observe(['staleDeliveredItemRejected'], 'the gate rejects a stale delivered item, a stale holder, an unverified hold and an outdated audit');
});
