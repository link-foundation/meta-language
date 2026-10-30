import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { structuredDataToLino, readStructuredDataLino, encodeStructuredDataLinks } from '../src/structured-data-links.js';
import { structuredDataRecordToLino, readStructuredDataRecords } from '../src/structured-data-records.js';
import { readBootstrapStructuredDataLino, readBootstrapStructuredDataRecords, inspectBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { structuredDataGraphCost } from '../src/structured-data-graph.js';
import { runCleanup, acquireLease, runningProcesses } from '../../scripts/lib/cache-cleanup.mjs';
import { makeScratchDirectory } from '../../scripts/lib/scratch.mjs';
import { SCRATCH_MARKER } from '../../scripts/lib/cache-classes.mjs';
import { REPOSITORY_ROOT, copyBootstrapDataModules, makeBase, makeRepository, runScript } from './support/cache-fixtures.js';

const namespace = '01234567-89ab-4cde-8f01-23456789abcd';

test('bootstrap canonical data readers share exact values and strict graph validation', () => {
  const value = { text: 'é😀\ud800', absence: null, values: [-0, true, 3.25], shared: [] };
  value.alsoShared = value.shared;
  const source = structuredDataToLino(value);
  assert.deepEqual(readBootstrapStructuredDataLino(source), readStructuredDataLino(source));
  const restored = readBootstrapStructuredDataLino(source);
  assert.strictEqual(restored.shared, restored.alsoShared, 'the actual shared container remains shared');
  const distinct = readBootstrapStructuredDataLino(structuredDataToLino({first: [], second: []}));
  assert.notStrictEqual(distinct.first, distinct.second, 'equal private metadata never merges distinct data containers');
  const encoded = encodeStructuredDataLinks(value);
  assert.deepEqual(inspectBootstrapStructuredDataLino(source).cost, structuredDataGraphCost(encoded.network, encoded.root));
  const arrays = source.split('\n').filter(line => line.includes('(term: array%20value)'));
  assert.ok(arrays.length >= 2);
  const alteredLanguage = source.replace(arrays.at(-1), arrays.at(-1).replace('(lang: meta-language%20structured%20data)', '(lang: altered)'));
  assert.throws(() => readBootstrapStructuredDataLino(alteredLanguage), /invalid typed link/);
  const records = structuredDataRecordToLino(value, namespace) + structuredDataRecordToLino({ next: 1 });
  assert.deepEqual(readBootstrapStructuredDataRecords(records), readStructuredDataRecords(records));
  assert.throws(() => readBootstrapStructuredDataLino(source.replace('(n: 1)', '(n: 0)')), /canonical/);
  assert.throws(() => readBootstrapStructuredDataLino(source.slice(0, -1)), /incomplete/);
  assert.throws(() => readBootstrapStructuredDataRecords(records + structuredDataRecordToLino(null, namespace)), /duplicate/);
  const shared = ['x'.repeat(32)];
  const two = structuredDataRecordToLino([shared, shared], namespace) + structuredDataRecordToLino([shared, shared]);
  assert.ok(Buffer.byteLength(two) < 4096);
  assert.throws(() => readBootstrapStructuredDataRecords(two, { nodeLimit: 10, byteLimit: 4096 }), /expanded.*resource/);
  for(const text of ['plain\n','é\n','🌍\n','\ud800\n','\udc00\n','\ud800\udc00\n','\0\n']){
    const bytes=new TextEncoder().encode(text).length;
    assert.throws(()=>readBootstrapStructuredDataLino(text,{byteLimit:bytes-1}),/input byte resource limit exceeded/);
    assert.throws(()=>readBootstrapStructuredDataLino(text,{byteLimit:bytes}),/not a canonical owned data statement/);
  }
});

test('cleanup before npm installation writes canonical reports using only repository leaf modules', t => {
  const fixture = makeRepository(makeBase(t));
  cpSync(path.join(REPOSITORY_ROOT, 'scripts'), path.join(fixture.root, 'scripts'), { recursive: true });
  copyBootstrapDataModules(fixture.root);
  assert.equal(existsSync(path.join(fixture.root, 'node_modules')), false);
  assert.equal(existsSync(path.join(fixture.root, 'js/node_modules')), false);
  const report = path.join(fixture.base, 'bootstrap-report.lino');
  const run = runScript(path.join(fixture.root, 'scripts/clean-caches.mjs'), [
    '--no-preflight', '--no-docker', '--class', 'javascript', '--report', report,
  ], { cwd: fixture.root, env: { ...process.env, NODE_PATH: '' } });
  assert.equal(run.status, 0, run.stderr);
  const value = readBootstrapStructuredDataLino(readFileSync(report, 'utf8'));
  assert.equal(value.status, 'ok');
  assert.equal(value.reclaimedBytes, 0);
  const state = path.join(fixture.root, '.git/meta-language-cache');
  assert.equal(existsSync(path.join(state, 'last-run.json')), false);
  assert.equal(existsSync(path.join(state, 'history.jsonl')), false);
  assert.deepEqual(readBootstrapStructuredDataLino(readFileSync(path.join(state, 'last-run.lino'), 'utf8')), value);
  assert.equal(readBootstrapStructuredDataRecords(readFileSync(path.join(state, 'history.lino'), 'utf8')).length, 1);
  const json = path.join(fixture.base, 'external-report.json');
  const legacy = runScript(path.join(fixture.root, 'scripts/clean-caches.mjs'), [
    '--dry-run', '--no-preflight', '--no-docker', '--class', 'javascript', '--json', json,
  ], { cwd: fixture.root, env: { ...process.env, NODE_PATH: '' } });
  assert.equal(legacy.status, 0, legacy.stderr);
  assert.equal(JSON.parse(readFileSync(json, 'utf8')).status, 'dry-run');
});

test('owned history migrates legacy records once and keeps exactly the latest 200', t => {
  const fixture = makeRepository(makeBase(t));
  const state = path.join(fixture.root, '.git/meta-language-cache');
  mkdirSync(state, { recursive: true });
  writeFileSync(path.join(state, 'history.jsonl'), Array.from({ length: 205 }, (_, sequence) => JSON.stringify({ sequence })).join('\n') + '\n');
  writeFileSync(path.join(state, 'last-run.json'), '{}\n');
  const report = runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, classes: ['javascript'], docker: false, preflight: false });
  const history = readBootstrapStructuredDataRecords(readFileSync(path.join(state, 'history.lino'), 'utf8'));
  assert.equal(history.length, 200);
  assert.deepEqual(history[0], { sequence: 6 });
  assert.equal(history.at(-1).reclaimedBytes, report.reclaimedBytes);
  assert.equal(existsSync(path.join(state, 'history.jsonl')), false);
  assert.equal(existsSync(path.join(state, 'last-run.json')), false);
  runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, classes: ['javascript'], docker: false, preflight: false });
  const next = readBootstrapStructuredDataRecords(readFileSync(path.join(state, 'history.lino'), 'utf8'));
  assert.equal(next.length, 200);
  assert.deepEqual(next[0], { sequence: 7 });
});

test('older active owners can still read the three coordination protocols as JSON', t => {
  const fixture = makeRepository(makeBase(t));
  const lease = acquireLease({ cwd: fixture.root, label: 'legacy reader' });
  t.after(() => lease.release());
  assert.equal(JSON.parse(readFileSync(lease.file, 'utf8')).pid, process.pid);
  const directory = makeScratchDirectory('meta-language-bootstrap-', 'legacy scratch reader');
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const marker = JSON.parse(readFileSync(path.join(directory, SCRATCH_MARKER), 'utf8'));
  assert.equal(marker.pid, process.pid);
  const state = path.join(fixture.root, '.git/meta-language-cache');
  const lock = path.join(state, 'cleanup.lock');
  writeFileSync(lock, JSON.stringify({ pid: process.ppid }));
  const report = runCleanup({ cwd: fixture.root, docker: false, preflight: false });
  assert.equal(report.status, 'busy');
  assert.equal(JSON.parse(readFileSync(lock, 'utf8')).pid, process.ppid);
});

test('the POSIX process probe resolves an executable invoked through a path with spaces', { skip: process.platform === 'win32' }, async t => {
  const directory = makeBase(t);
  const binary = path.join(directory, 'node with spaces');
  symlinkSync(process.execPath, binary);
  const running = spawn(binary, ['-e', 'process.stdout.write("ready\\n"); setInterval(() => {}, 1000);'], { stdio: ['ignore', 'pipe', 'pipe'] });
  const exited = new Promise(resolve => running.once('exit', resolve));
  t.after(async () => { running.kill('SIGKILL'); await exited; });
  await new Promise((resolve, reject) => { running.stdout.once('data', resolve); running.once('error', reject); });
  const actual = runningProcesses().processes.find(entry => entry.pid === running.pid);
  assert.ok(actual, 'the ready fixture process is listed');
  assert.equal(actual.executable, realpathSync(process.execPath));
});
