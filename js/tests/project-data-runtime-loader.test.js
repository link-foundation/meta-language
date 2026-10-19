import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import test from 'node:test';
import { readRuntimeProjectData } from '../src/project-data-loader.js';
import { structuredDataToLino } from '../src/structured-data-links.js';
import * as runtimeFiles from '../src/project-data-files.js';
import * as toolingFiles from '../scripts/project-data.mjs';

function directory(run) {
  const root = mkdtempSync(join(tmpdir(), 'project-data-runtime-'));
  try { run(root); }
  finally { rmSync(root, { recursive: true, force: true }); }
}

test('runtime metadata retains its logical JSON identity after moving to Links Notation', () => directory(root => {
  const value = { aliases: ['one', 'two'], nested: { enabled: true, missing: null }, version: 1 };
  writeFileSync(join(root, 'metadata.lino'), structuredDataToLino(value));
  assert.deepEqual(readRuntimeProjectData(pathToFileURL(join(root, 'metadata.json'))), value);
  assert.deepEqual(readRuntimeProjectData(join(root, 'metadata.lino')), value);
}));

test('unmigrated metadata retains its original JSON value', () => directory(root => {
  const value = { source: 'before migration', preserved: [0, false, ''] };
  writeFileSync(join(root, 'metadata.json'), JSON.stringify(value));
  assert.deepEqual(readRuntimeProjectData(join(root, 'metadata.json')), value);
}));

test('duplicate and malformed runtime data never select a second representation', () => directory(root => {
  writeFileSync(join(root, 'metadata.json'), '{"original":true}');
  writeFileSync(join(root, 'metadata.lino'), structuredDataToLino({ changed: true }));
  for (const extension of ['json', 'lino']) {
    assert.throws(() => readRuntimeProjectData(join(root, `metadata.${extension}`)), /duplicate JSON and LiNo/u);
  }
  rmSync(join(root, 'metadata.json'));
  writeFileSync(join(root, 'metadata.lino'), 'malformed linked data');
  assert.throws(() => readRuntimeProjectData(join(root, 'metadata.json')));
}));

test('shipped runtime and tooling share identical file helpers and explicit format boundaries', () => directory(root => {
  assert.deepEqual(Object.keys(runtimeFiles).sort(), Object.keys(toolingFiles).sort());
  for (const name of Object.keys(runtimeFiles)) assert.equal(runtimeFiles[name], toolingFiles[name], `${name} delegates to the shipped implementation`);
  const json = join(root, 'external.json');
  const linked = join(root, 'external.lino');
  writeFileSync(json, '{"source":"external JSON protocol"}\n');
  writeFileSync(linked, structuredDataToLino({ source: 'structured project data' }));
  assert.deepEqual(readRuntimeProjectData(json, { format: 'json' }), { source: 'external JSON protocol' });
  assert.deepEqual(runtimeFiles.readProjectData(json, { format: 'json' }), readRuntimeProjectData(json, { format: 'json' }));
  assert.throws(() => readRuntimeProjectData(json), /duplicate JSON and LiNo/u);
  assert.throws(() => readRuntimeProjectData(json, { format: 'lino' }), /duplicate JSON and LiNo/u);
  rmSync(json);
  assert.deepEqual(readRuntimeProjectData(json, { format: 'lino' }), { source: 'structured project data' });
  assert.ok(runtimeFiles.readProjectFileBytes(json).equals(runtimeFiles.readProjectFileBytes(linked)));
}));
