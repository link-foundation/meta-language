import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { consumeWorkloadWireData, consumeWorkloadWireEvents } from '../scripts/workload-wire-data.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { readStructuredDataRecords } from '../src/structured-data-records.js';

async function scratch(body) {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'meta-language-workload-wire-'));
  try { await body(directory); }
  finally { await rm(directory, { recursive: true, force: true }); }
}

test('workload wire observations retain every field in inspectable links and remove only the consumed wire', () => scratch(async directory => {
  const wire = path.join(directory, 'observations.wire.json');
  const linked = path.join(directory, 'observations.lino');
  const value = JSON.parse('{"schemaVersion":1,"__proto__":{"kept":true},"checks":[{"holds":false,"detail":"λ\\nfailed"}],"counts":{"passed":0,"failed":1},"unknown":null}');
  await writeFile(wire, JSON.stringify(value));
  const observed = await consumeWorkloadWireData(wire, linked);
  assert.deepEqual(observed, value);
  assert.deepEqual(readProjectData(linked), value);
  assert.deepEqual(Object.keys(readProjectData(linked)), Object.keys(value));
  assert.equal(existsSync(wire), false);
  assert.match(await readFile(linked, 'utf8'), /key%20value%20pair/u);
  assert.equal(Object.prototype.kept, undefined);
}));

test('malformed workload wire data cannot replace a previous linked observation', () => scratch(async directory => {
  const wire = path.join(directory, 'observations.wire.json');
  const linked = path.join(directory, 'observations.lino');
  writeProjectData(linked, { previous: true });
  const previous = await readFile(linked);
  await writeFile(wire, '{"checks":');
  await assert.rejects(consumeWorkloadWireData(wire, linked), SyntaxError);
  assert.deepEqual(await readFile(linked), previous);
  assert.equal(await readFile(wire, 'utf8'), '{"checks":');
}));

test('workload event streams keep passed and failed observations in fully qualified linked frames', () => scratch(async directory => {
  const wire = path.join(directory, 'events.wire.jsonl');
  const linked = path.join(directory, 'events.lino');
  const events = [
    { kind: 'test', name: 'actual callback', outcome: 'passed', file: 'tests/callback.js' },
    { kind: 'test', name: 'rejected callback', outcome: 'failed', error: 'line one\nline two', process: { exit: 1 } },
  ];
  await writeFile(wire, events.map(value => JSON.stringify(value)).join('\r\n') + '\r\n');
  assert.deepEqual(await consumeWorkloadWireEvents(wire, linked), events);
  const text = await readFile(linked, 'utf8');
  assert.deepEqual(readStructuredDataRecords(text), events);
  assert.equal(text.trim().split('\n').length, events.length);
  assert.equal(existsSync(wire), false);
  assert.doesNotMatch(text, /\(\d+:/u);
  assert.match(text, /[0-9a-f-]{36}\/record:/u);
}));

test('a malformed late event cannot publish a partial successful workload stream', () => scratch(async directory => {
  const wire = path.join(directory, 'events.wire.jsonl');
  const linked = path.join(directory, 'events.lino');
  await writeFile(linked, 'previous published bytes');
  const source = '{"kind":"test","outcome":"passed"}\n{"truncated":\n';
  await writeFile(wire, source);
  await assert.rejects(consumeWorkloadWireEvents(wire, linked), SyntaxError);
  assert.equal(await readFile(linked, 'utf8'), 'previous published bytes');
  assert.equal(await readFile(wire, 'utf8'), source);
  assert.deepEqual((await readdir(directory)).sort(), ['events.lino', 'events.wire.jsonl']);
}));
