import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';
import { checkProjectData, readProjectData, readProjectFileBytes, resolveProjectDataPath,
  serializeProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { issue195FixtureDigest } from './support/issue-195-observations.js';

function scratch(body) {
  const directory = mkdtempSync(join(tmpdir(), 'meta-project-data-'));
  try { body(directory); } finally { rmSync(directory, { recursive: true, force: true }); }
}

test('logical JSON paths and URLs resolve canonical structured LiNo without changing identities', () => scratch(directory => {
  const logical = join(directory, 'fixture with spaces.json');
  const value = JSON.parse('{"fixtureFile":"parity/fixtures/native-grammars/elixir.json","cases":[{"id":"café 😀","rows":[[0,"",true,null]]}],"__proto__":{"owned":true}}');
  const actual = writeProjectData(logical, value);
  assert.equal(actual, logical.replace(/\.json$/u, '.lino'));
  assert.equal(existsSync(logical), false);
  assert.equal(resolveProjectDataPath(pathToFileURL(logical)), actual);
  assert.deepEqual(readProjectData(logical), value);
  assert.deepEqual(readProjectData(pathToFileURL(logical)), value);
  assert.equal(checkProjectData(logical, value), true);
  assert.equal(readFileSync(actual, 'utf8'), serializeProjectData(value, logical));
  assert.ok(!readFileSync(actual, 'utf8').includes(JSON.stringify(value)), 'the record and ordered cases are not an opaque JSON payload');
  assert.equal(readProjectData(logical).fixtureFile, 'parity/fixtures/native-grammars/elixir.json');
  assert.deepEqual(readdirSync(directory), ['fixture with spaces.lino'], 'atomic writing leaves one canonical file');
}));

test('alias bytes retain exact physical integrity and never claim the old JSON digest', () => scratch(directory => {
  const logical = join(directory, 'fixture.json');
  const value = { id: 'case', values: [1, 2] };
  writeProjectData(logical, value);
  const bytes = readProjectFileBytes(logical);
  assert.ok(bytes.equals(readFileSync(logical.replace(/\.json$/u, '.lino'))));
  const digest = data => createHash('sha256').update(data).digest('hex');
  assert.notEqual(digest(bytes), digest(JSON.stringify(value, null, 2) + '\n'));
}));

test('execution observations hash the resolved LiNo while retaining the logical fixture reference', () => scratch(directory => {
  const logical = join(directory, 'evidence-fixture.json');
  const value = { fixtureFile: 'parity/fixtures/issue-195-evidence.json', cases: [{ id: 'unchanged' }] };
  const actual = writeProjectData(logical, value);
  const logicalReference = pathToFileURL(logical).href;
  const expected = createHash('sha256').update(readFileSync(actual)).digest('hex');
  assert.equal(issue195FixtureDigest(logicalReference), expected);
  assert.equal(readProjectData(logical).fixtureFile, value.fixtureFile);
  assert.notEqual(expected, createHash('sha256').update(JSON.stringify(value, null, 2) + '\n').digest('hex'));
}));

test('freshness checks reject changed values, noncanonical text and malformed LiNo', () => scratch(directory => {
  const file = join(directory, 'fixture.lino');
  const value = { count: 2, values: ['a', 'b'] };
  writeProjectData(file, value);
  assert.throws(() => checkProjectData(file, { ...value, count: 3 }), /stale/u);
  writeFileSync(file, readFileSync(file, 'utf8') + '\n');
  assert.deepEqual(readProjectData(file), value);
  assert.throws(() => checkProjectData(file, value), /stale/u);
  writeFileSync(file, 'this is not a structured data graph');
  assert.throws(() => readProjectData(file));
  assert.throws(() => checkProjectData(file, value), /stale/u);
}));

test('coexisting formats fail instead of accepting a stale fallback', () => scratch(directory => {
  const json = join(directory, 'fixture.json');
  const lino = join(directory, 'fixture.lino');
  writeProjectData(lino, { current: true });
  writeFileSync(json, '{"current":false}\n');
  assert.throws(() => resolveProjectDataPath(json), /duplicate JSON and LiNo/u);
  assert.throws(() => readProjectData(json), /duplicate JSON and LiNo/u);
  assert.throws(() => readProjectData(lino), /duplicate JSON and LiNo/u);
  assert.throws(() => writeProjectData(json, { current: true }), /duplicate JSON and LiNo/u);
  assert.deepEqual(readProjectData(json, { format: 'json' }), { current: false }, 'an explicit external JSON boundary reads only its requested bytes');
}));

test('legacy data remains readable until its owner converts storage and external JSON stays explicit', () => scratch(directory => {
  const json = join(directory, 'package.json');
  const value = { name: 'external-package', version: '1.0.0' };
  writeProjectData(json, value, { format: 'json' });
  assert.equal(resolveProjectDataPath(json), json);
  assert.deepEqual(readProjectData(json), value);
  assert.equal(checkProjectData(json, value, { format: 'json' }), true);
  assert.equal(writeProjectData(json, value), json, 'the loader does not perform an unrequested whole-file migration');
  assert.equal(existsSync(join(directory, 'package.lino')), false);
  assert.throws(() => readProjectData(json, { format: 'unknown' }), /format must/u);
}));

test('failed structured serialization leaves the previous data file intact', () => scratch(directory => {
  const file = join(directory, 'fixture.lino');
  writeProjectData(file, { value: 1 });
  const before = readFileSync(file);
  assert.throws(() => writeProjectData(file, { callback() {} }), /unsupported host value/u);
  assert.ok(readFileSync(file).equals(before));
  assert.deepEqual(readdirSync(directory), ['fixture.lino']);
}));

test('physical readers keep non-data source and specification bytes exact', () => scratch(directory => {
  const file = join(directory, 'source.rs');
  const source = 'fn café() { /* 😀 */ }\r\n';
  writeFileSync(file, source);
  assert.equal(resolveProjectDataPath(file), file);
  assert.equal(readProjectFileBytes(file).toString('utf8'), source);
  assert.throws(() => readProjectData(file), /unsupported storage extension/u);
}));
