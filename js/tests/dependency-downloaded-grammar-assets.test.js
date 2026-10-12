import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { collectDependencies, checkInventory, reconcileLocalDeclarations } from '../scripts/dependency-inventory.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const grammarLockPath = 'js/src/vendor/grammars/grammar-lock.json';
const runtimeLockPath = 'js/src/vendor/web-tree-sitter/runtime-lock.json';
const grammarLock = readProjectData(path.join(root, grammarLockPath));
const runtimeLock = readProjectData(path.join(root, runtimeLockPath));

function isolatedRepository(t, mutate = () => {}) {
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-dependency-grammar-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const put = (file, text) => {
    mkdirSync(path.dirname(path.join(directory, file)), { recursive: true });
    writeFileSync(path.join(directory, file), text);
  };
  for (const file of ['js/package.json', 'js/package-lock.json', 'rust/Cargo.toml', 'rust/Cargo.lock',
    'rust/web/Cargo.toml', 'rust/web/Cargo.lock']) put(file, readFileSync(path.join(root, file)));
  mkdirSync(path.join(directory, '.github/workflows'), { recursive: true });
  const grammar = structuredClone(grammarLock);
  const runtime = structuredClone(runtimeLock);
  mutate(grammar, runtime);
  for (const [file, value] of [[grammarLockPath, grammar], ['rust/src/data/grammar-lock.json', grammar],
    [runtimeLockPath, runtime]]) {
    const lino = path.join(directory, file.replace(/\.json$/u, '.lino'));
    mkdirSync(path.dirname(lino), { recursive: true });
    writeProjectData(lino, value);
  }
  return directory;
}

test('LiNo metadata inventories all sixty grammar downloads after raw assets are removed', (t) => {
  const directory = isolatedRepository(t);
  const items = collectDependencies(directory);
  const grammars = items.filter(({ category }) => category === 'downloaded-grammar');
  assert.equal(grammars.length, 60);
  assert.deepEqual(grammars.map(({ provenance }) => provenance.id).sort(), Object.keys(grammarLock.grammars).sort());
  assert.equal(items.filter(({ category }) => category === 'downloaded-parser').length, 5);
  assert.equal(items.some(({ category }) => category === 'vendored-parser' || category === 'vendored-grammar'), false);
  const byId = new Map(items.map((item) => [item.id, item]));
  for (const grammar of grammars) {
    const locked = grammarLock.grammars[grammar.provenance.id];
    assert.equal(grammar.pinned, locked.version);
    assert.equal(grammar.provenance.parserSha256, locked.parserSha256);
    assert.equal(grammar.provenance.wasmSha256, locked.wasmSha256);
    assert.equal(grammar.provenance.oracle, locked.oracle);
    assert.equal(grammar.dependsOn.length, 2);
    assert.equal(byId.get(grammar.dependsOn[0]).pinned, locked.version);
    assert.equal(byId.get(grammar.dependsOn[1]).name, 'tree-sitter-cli');
    assert.ok(grammar.declaredIn.every((file) => file.endsWith('.lino')));
    assert.equal(grammar.aliases.length, 1, 'the previous audit identity remains explicit');
  }
  const runtime = items.find(({ category }) => category === 'vendored-runtime');
  assert.equal(runtime.provenance.wasmSha256, runtimeLock.wasmSha256);
  assert.equal(runtime.provenance.upstreamWasmSha256, runtimeLock.upstreamWasmSha256);
  assert.equal(runtime.provenance.patchSha256, runtimeLock.patchSha256);
  assert.equal(runtime.provenance.revision, runtimeLock.revision);
  assert.ok(runtime.declaredIn[0].endsWith('.lino'));
});

test('grammar metadata refuses lost hashes, floating tools and crate pin changes', (t) => {
  for (const mutate of [
    (grammar) => { grammar.grammars.agda.parserSha256 = ''; },
    (grammar) => { grammar.grammars.agda.wasmSha256 = 'not-a-hash'; },
    (grammar) => { grammar.treeSitterCli = 'tree-sitter latest'; },
    (grammar) => { grammar.grammars.agda.version = '99.0.0'; },
    (_grammar, runtime) => { runtime.patchSha256 = ''; },
  ]) {
    assert.throws(() => collectDependencies(isolatedRepository(t, mutate)), /sha256|exact tree-sitter CLI|Cargo\.lock/u);
  }
});

test('relocation retains exact registry receipts and remaps inherited generator edges', (t) => {
  const downloaded = collectDependencies(isolatedRepository(t)).filter(({ category }) =>
    category === 'downloaded-parser' || category === 'downloaded-grammar');
  const prior = downloaded.map((item) => ({
    ...item, id: item.aliases[0], category: item.category.replace('downloaded', 'vendored'),
    name: item.category === 'downloaded-parser' ? item.provenance.vendored : item.name,
    aliases: undefined, provenance: undefined, current: item.pinned, evidence: 'exact registry audit receipt',
  }));
  const parser = downloaded.find(({ category }) => category === 'downloaded-parser');
  const generator = { id: 'generator own.mjs@script', category: 'generator', name: 'own.mjs',
    declaredIn: ['own.mjs'], pinned: 'script', compare: 'derived', source: { type: 'none' } };
  const inventory = { auditDate: '2026-10-06', items: [...prior,
    { ...generator, dependsOn: [parser.aliases[0]], evidence: 'existing tools' }] };
  const migrated = reconcileLocalDeclarations(inventory, [...downloaded, generator]);
  assert.equal(migrated.auditDate, inventory.auditDate);
  for (const item of migrated.items.filter(({ category }) => category !== 'generator')) {
    assert.equal(item.evidence, 'exact registry audit receipt');
    assert.equal(item.current, item.pinned);
    assert.ok(item.aliases.includes(prior.find((old) => old.id === item.aliases[0]).id));
  }
  assert.deepEqual(migrated.items.at(-1).dependsOn, [parser.id]);
  assert.deepEqual(inventory.items.at(-1).dependsOn, [parser.aliases[0]], 'migration does not mutate the prior receipt');
  const tampered = structuredClone(parser);
  tampered.source.repository = 'foreign/repository';
  assert.throws(() => reconcileLocalDeclarations(inventory, [tampered]), /audited source or pin|live dependency refresh/u);
  const movedPin = { ...parser, pinned: 'other-revision' };
  assert.throws(() => reconcileLocalDeclarations(inventory, [movedPin]), /audited source or pin|live dependency refresh/u);
});

test('recorded grammar provenance is compared even when dependency pins do not change', (t) => {
  const collected = collectDependencies(isolatedRepository(t));
  const grammar = collected.find(({ category }) => category === 'downloaded-grammar');
  const inventory = { auditDate: '2026-10-06', items: [{ ...grammar }] };
  inventory.items[0].provenance = { ...grammar.provenance, wasmSha256: '0'.repeat(64) };
  const problems = checkInventory(inventory, [grammar], { today: '2026-10-12' });
  assert.ok(problems.some(({ kind, message }) => kind === 'changed' && message.includes('provenance')));
});
