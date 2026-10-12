import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import test from 'node:test';
import { readRuntimeProjectData } from '../src/project-data-loader.js';
import { structuredDataToLino } from '../src/structured-data-links.js';

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
    assert.throws(() => readRuntimeProjectData(join(root, `metadata.${extension}`)), /both JSON and Links Notation/u);
  }
  rmSync(join(root, 'metadata.json'));
  writeFileSync(join(root, 'metadata.lino'), 'malformed linked data');
  assert.throws(() => readRuntimeProjectData(join(root, 'metadata.json')));
}));
