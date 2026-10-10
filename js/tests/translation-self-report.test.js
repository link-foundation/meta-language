import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { DecoratorSet } from '../src/decorators.js';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';
import { LinkNetwork } from '../src/network.js';
import { createModuleContext } from '../scripts/generate-self-translation-report.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const script = path.join(root, 'js/scripts/generate-self-translation-report.mjs');
const commit = '1'.repeat(40);
const digest = (file) => createHash('sha256').update(readFileSync(path.join(root, file))).digest('hex');
const list = (directory) => readdirSync(path.join(root, directory), { withFileTypes: true }).flatMap((entry) => {
  const file = path.posix.join(directory, entry.name);
  return entry.isDirectory() ? list(file) : file.endsWith('.js') ? [file] : [];
});
const metrics = { functions: 0, matched: 0, identical: 0, codeLines: 0, sharedLines: 0, handWrittenLines: 0 };

test('report contexts resolve real translated exports across relative module dependencies', () => {
  const modules = new Map([
    ['value.js', '/** @param {number} x @returns {number} */\nexport function increment(x) { return x + 1; }\n'],
    ['nested/second.js', "import { increment as next } from '../value.js';\n/** @param {number} x @returns {number} */\nexport function second(x) { return next(x) + 1; }\n"],
    ['main.js', "import { second } from './nested/second.js';\n/** @param {number} x @returns {number} */\nexport function compute(x) { return second(x) + 1; }\n"],
  ]);
  const reads = new Map();
  const context = createModuleContext((name) => {
    reads.set(name, (reads.get(name) ?? 0) + 1);
    return modules.get(name) ?? null;
  });
  for (const name of ['main.js', 'nested/second.js']) {
    const translation = selfTranslate(modules.get(name), 'JavaScript', 'Rust', context(name));
    assert.ok(translation.items.every((item) => item.status !== 'carried'), translation.code);
  }
  assert.deepEqual(context('nested/second.js').moduleDirectory, ['nested']);
  assert.deepEqual(context('main.js').imports['./nested/second.js'].map(({ name }) => name), ['second']);
  assert.equal(context('main.js'), context('main.js'));
  assert.ok([...reads.values()].every((count) => count === 1));
});

test('report contexts use syntax imports and refuse paths outside the source root', () => {
  const source = `// import { fake } from './fake.js'
import { outside } from '../outside.js';
import { absent } from './missing.js';
export const message = 'import { pretend } from "./pretend.js"';
`;
  const reads = [];
  const context = createModuleContext((name) => {
    reads.push(name);
    return name === 'main.js' ? source : null;
  });
  assert.deepEqual(context('main.js').imports, {});
  assert.deepEqual(reads, ['main.js', 'missing.js']);
});

test('report contexts keep missing exports and import cycles unbound', () => {
  const modules = new Map([
    ['provider.js', 'export class Unsupported {}\n'],
    ['missing.js', "import { absent } from './provider.js';\n/** @returns {number} */\nexport function read() { return absent(); }\n"],
    ['first.js', "import { second } from './second.js';\n/** @returns {number} */\nexport function first() { return second(); }\n"],
    ['second.js', "import { first } from './first.js';\n/** @returns {number} */\nexport function second() { return first(); }\n"],
  ]);
  const context = createModuleContext((name) => modules.get(name) ?? null);
  for (const name of ['missing.js', 'first.js']) {
    const translation = selfTranslate(modules.get(name), 'JavaScript', 'Rust', context(name));
    assert.ok(translation.items.some((item) => item.status === 'carried'), translation.code);
  }
});

function fixture() {
  const modules = list('js/src').sort().map((module) => {
    const stem = path.posix.relative('js/src', module).replace(/\.js$/u, '').replaceAll('-', '_');
    const rust = [`rust/src/${stem}.rs`, `rust/src/${stem}/mod.rs`].find((file) => existsSync(path.join(root, file))) ?? null;
    return {
      module, rust, sourceSha256: digest(module), rustSha256: rust ? digest(rust) : null,
      items: { translated: 0, carried: 0, comment: 0 }, milliseconds: 0,
      ...metrics, decorated: { ...metrics },
    };
  });
  const decorators = DecoratorSet.fromLino(readFileSync(path.join(root, 'parity/self-translation/decorators.lino'), 'utf8')).ids();
  return { schemaVersion: 1, commit, decorators, modules, failures: [] };
}

function verify(report, markdown) {
  const directory = mkdtempSync(path.join(tmpdir(), 'self-report-validation-'));
  try {
    const shard = path.join(directory, 'shard');
    mkdirSync(shard);
    writeFileSync(path.join(shard, 'self-translation-report.json'), JSON.stringify(report));
    writeFileSync(path.join(shard, 'self-translation-report.md'), markdown ?? report.modules.map(({ module }) => `| ${module} |`).join('\n'));
    return spawnSync(process.execPath, [script, '--verify-reports', directory, '--commit', commit], { encoding: 'utf8' });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

test('the published report validator covers every source module at the observed commit', () => {
  const report = fixture();
  const result = verify(report);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, new RegExp(`${report.modules.length} modules at ${commit}`, 'u'));
});

test('module identities use forward slashes on every host platform', () => {
  for (const args of [[], ['--modules', 'translation/frontend-rules.js,language-support.js']]) {
    const result = spawnSync(process.execPath, [script, '--list', ...args], { encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
    const modules = result.stdout.trim().split('\n');
    assert.ok(modules.length > 0);
    for (const module of modules) {
      assert.ok(module.startsWith('js/src/'));
      assert.ok(!module.includes('\\'), module);
    }
  }
});

test('incomplete, stale or inconsistent reports cannot record the reporting assertion', () => {
  for (const [mutate, message] of [
    [(report) => { report.modules.pop(); }, /missing module/u],
    [(report) => { report.modules.push(report.modules[0]); }, /duplicate module/u],
    [(report) => { report.commit = '2'.repeat(40); }, /commit/u],
    [(report) => { report.modules[0].sourceSha256 = '0'.repeat(64); }, /source hash/u],
    [(report) => { report.modules.find(({ rust }) => rust).rustSha256 = '0'.repeat(64); }, /Rust hash/u],
    [(report) => { report.modules[0].decorated.sharedLines = 1; }, /measurements/u],
    [(report) => { report.decorators = []; }, /decorators/u],
    [(report) => { report.failures.push({ module: report.modules[0].module, error: 'refused' }); }, /failed modules/u],
  ]) {
    const report = fixture();
    mutate(report);
    const result = verify(report);
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, message);
  }
  const result = verify(fixture(), 'No per-module rows.');
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Markdown row/u);
});


test('signature discovery checks exports without constructing another complete source graph', () => {
  const source = '/** @returns {number} */\nexport function count() { return 2; }\nexport const limit = 3;\nexport class Unsupported {}\n';
  const original = LinkNetwork.parse;
  let signatures;
  try {
    LinkNetwork.parse = () => { throw new Error('signature discovery constructed a source graph'); };
    signatures = selfTranslationSignatures(source, 'JavaScript');
  } finally { LinkNetwork.parse = original; }
  assert.deepEqual(signatures, [
    { k: 'fn', name: 'count', params: [], ret: { kind: 'float' } },
    { k: 'const', name: 'limit', type: { kind: 'float' }, literal: true },
  ]);
  const translation = selfTranslate(source, 'JavaScript', 'Rust');
  assert.ok(translation.items.some(({ status }) => status === 'translated'));
  assert.ok(translation.items.some(({ status }) => status === 'carried'));
  assert.throws(() => selfTranslationSignatures("export const invalid = '\ud800';", 'JavaScript'), /do not reproduce/u);
});
