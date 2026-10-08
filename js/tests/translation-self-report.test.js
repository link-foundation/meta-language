import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { DecoratorSet } from '../src/decorators.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const script = path.join(root, 'js/scripts/generate-self-translation-report.mjs');
const commit = '1'.repeat(40);
const digest = (file) => createHash('sha256').update(readFileSync(path.join(root, file))).digest('hex');
const list = (directory) => readdirSync(path.join(root, directory), { withFileTypes: true }).flatMap((entry) => {
  const file = path.posix.join(directory, entry.name);
  return entry.isDirectory() ? list(file) : file.endsWith('.js') ? [file] : [];
});
const metrics = { functions: 0, matched: 0, identical: 0, codeLines: 0, sharedLines: 0, handWrittenLines: 0 };

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
