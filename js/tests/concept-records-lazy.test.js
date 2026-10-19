import test from 'node:test';
import assert from 'node:assert/strict';
import fs, { mkdtempSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { syncBuiltinESMExports } from 'node:module';
import { createHash } from 'node:crypto';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createLazyFrozenArray } from '../src/lazy-shipped-data.js';
import { writeProjectData } from '../scripts/project-data.mjs';

const runtimeSource = new URL('../src/concept-records.js', import.meta.url);

function fixture(t) {
  const directory = realpathSync(mkdtempSync(join(tmpdir(), 'concept-register-lazy-')));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  mkdirSync(join(directory, 'data'));
  const alias = { source: 'example', name: 'shared' };
  const concepts = ['first', 'second'].map((name, index) => ({
    id: `concept.${name}`, phrase: `${name} concept`, role: 'concept',
    definition: `The ${name} concept.`, constraints: ['immutable record'],
    sourceAliases: index === 0 ? [alias, alias] : [alias], formerNames: [`former.${name}`],
  }));
  const primary = writeProjectData(join(directory, 'data/concept-records.lino'), { concepts });
  const metadata = {
    schemaVersion: 1, source: 'concept-records.lino', conceptCount: concepts.length,
    sourceSha256: createHash('sha256').update(readFileSync(primary)).digest('hex'),
  };
  const metadataFile = join(directory, 'data/concept-records-metadata.lino');
  writeProjectData(metadataFile, metadata);
  const source = readFileSync(runtimeSource, 'utf8').replace(
    /from '(\.\/[^']+)'/gu,
    (_match, relative) => `from '${new URL(relative, runtimeSource).href}'`,
  );
  const module = join(directory, 'concept-records.mjs');
  writeFileSync(module, source);
  return { concepts, metadata, metadataFile, primary, module: pathToFileURL(module).href };
}

test('the frozen native array shape stays lazy, cloneable and stable across every lookup', async (t) => {
  const sample = fixture(t);
  const originalRead = fs.readFileSync;
  let primaryReads = 0;
  fs.readFileSync = (file, ...arguments_) => {
    const name = file instanceof URL ? fileURLToPath(file) : String(file);
    if (name === sample.primary) primaryReads += 1;
    return originalRead(file, ...arguments_);
  };
  syncBuiltinESMExports();
  try {
    const runtime = await import(sample.module);
    const records = runtime.CONCEPT_RECORDS;
    assert.equal(Array.isArray(records), true);
    assert.equal(Object.isFrozen(records), true);
    assert.equal(records.length, 2);
    assert.equal(runtime.conceptRecords(), records);
    assert.equal(primaryReads, 0);
    assert.throws(() => { records[0] = null; }, TypeError);
    assert.throws(() => records.push(null), TypeError);
    assert.equal(primaryReads, 0);
    const cloned = structuredClone(records);
    assert.deepEqual(cloned, sample.concepts);
    assert.equal(primaryReads, 1);
    const first = records[0];
    assert.equal(runtime.conceptRecord('concept.first'), first);
    assert.equal(runtime.conceptRecord('former.first'), first);
    assert.equal(runtime.currentConceptId('former.first'), 'concept.first');
    assert.equal(runtime.currentConceptId('unknown'), 'unknown');
    assert.deepEqual([...records], sample.concepts);
    assert.equal(first.sourceAliases[0], records[1].sourceAliases[0]);
    const named = runtime.conceptRecordsForSourceName('example', 'shared');
    assert.deepEqual(named, [first, records[1]]);
    named.reverse(); named.pop();
    assert.deepEqual(runtime.conceptRecordsForSourceName('example', 'shared'), [first, records[1]]);
    assert.equal(Object.isFrozen(first), true);
    assert.equal(Object.isFrozen(first.constraints), true);
    assert.equal(Object.isFrozen(first.sourceAliases[0]), true);
    assert.throws(() => first.constraints.push('changed'), TypeError);
    assert.throws(() => { first.sourceAliases[0].name = 'changed'; }, TypeError);
    assert.equal(primaryReads, 1);
  } finally {
    fs.readFileSync = originalRead;
    syncBuiltinESMExports();
  }
});

test('first use validates the full source digest and generated cardinality', async (t) => {
  for (const [name, tamper, expected] of [
    ['source digest', sample => { sample.metadata.sourceSha256 = '0'.repeat(64); }, /source digest/u],
    ['declared cardinality', sample => { sample.metadata.conceptCount = 3; }, /declared length/u],
    ['changed primary data', sample => {
      writeProjectData(sample.primary, { concepts: sample.concepts.map(concept => ({ ...concept, definition: 'Changed.' })) });
    }, /source digest/u],
  ]) {
    await t.test(name, async (context) => {
      const sample = fixture(context);
      tamper(sample);
      writeProjectData(sample.metadataFile, sample.metadata);
      const runtime = await import(sample.module);
      assert.equal(Object.isFrozen(runtime.CONCEPT_RECORDS), true);
      assert.throws(() => runtime.conceptRecord('concept.first'), expected);
    });
  }
});

test('invalid metadata and recursive or inconsistent loaders fail without publishing entries', async (t) => {
  for (const count of [-1, 0, 1.5, 1_000_001, '2']) {
    await t.test(`cardinality ${count}`, async (context) => {
      const sample = fixture(context);
      writeProjectData(sample.metadataFile, { ...sample.metadata, conceptCount: count });
      await assert.rejects(import(sample.module), /metadata is invalid|valid length/u);
    });
  }
  const wrong = createLazyFrozenArray(2, () => [1]);
  assert.throws(() => wrong[0], /declared length/u);
  const recursive = createLazyFrozenArray(1, () => recursive[0]);
  assert.throws(() => recursive[0], /recursive/u);
});
