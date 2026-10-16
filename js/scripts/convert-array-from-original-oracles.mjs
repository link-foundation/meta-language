// One-time authenticated conversion of the retained pre-extraction snapshots.
// The oracle stores its inspectable parts as Links records, preserving source bytes.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, unlinkSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';

const originalFile = new URL('../tests/support/array-from-original.txt', import.meta.url);
const mapFile = new URL('../tests/support/array-map-checker-original.txt', import.meta.url);
const outputFile = new URL('../tests/fixtures/array-from-original-oracles.lino', import.meta.url);
const originalHash = 'f5acea4921002a50008087829601931fa40794deb019546c3649e1770ec4870e';
const mapHash = '615d52168b1061e3cb726d161f747a7947da85091cac300090d7d3a05b7e3b8d';
const hash = source => createHash('sha256').update(source).digest('hex');

function createArrayFromOriginalOracles(snapshot, mapSource) {
  const originalPayload = JSON.stringify(snapshot, null, 2) + '\n';
  assert.equal(hash(originalPayload), originalHash, 'ordered source records reconstruct the original payload bytes');
  assert.equal(hash(mapSource), mapHash, 'the authenticated original complete method checker is unchanged before conversion');
  return { schemaVersion: 1, model: 'Array.from original behavior oracles 1',
    originalPayloadSha256: originalHash, originalPayloadBytes: Buffer.byteLength(originalPayload),
    snapshot, partSha256: Object.fromEntries(Object.entries(snapshot.parts).map(([name, source]) => [name, hash(source)])),
    mapSource, mapSourceSha256: mapHash };
}

export function checkArrayFromOriginalOracles() {
  const value = readProjectData(outputFile);
  assert.deepEqual(value, createArrayFromOriginalOracles(value.snapshot, value.mapSource), 'the preserved Links oracle authenticates every original part and ordered payload');
  return value;
}

export function convertArrayFromOriginalOracles({ removeInputs = false } = {}) {
  let value;
  if (existsSync(outputFile)) value = checkArrayFromOriginalOracles();
  if (existsSync(originalFile) && existsSync(mapFile)) {
    const originalPayload = readFileSync(originalFile, 'utf8');
    assert.equal(hash(originalPayload), originalHash, 'the authenticated original payload is unchanged before conversion');
    const converted = createArrayFromOriginalOracles(JSON.parse(originalPayload), readFileSync(mapFile, 'utf8'));
    if (value) assert.deepEqual(converted, value, 'temporary inputs agree with the already authenticated Links record');
    else { value = converted; writeProjectData(outputFile, value); }
  }
  assert.ok(value, 'conversion needs both original inputs or the preserved authenticated Links oracle');
  for (const [file, expectedHash] of [[originalFile, originalHash], [mapFile, mapHash]]) {
    if (existsSync(file)) assert.equal(hash(readFileSync(file, 'utf8')), expectedHash, 'each remaining temporary source retains its authenticated bytes');
  }
  assert.deepEqual(readProjectData(outputFile), value, 'Links conversion retains every original source part and its identity/order');
  if (removeInputs) for (const input of [originalFile, mapFile]) if (existsSync(input)) unlinkSync(input);
  return value;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--check')) checkArrayFromOriginalOracles();
  else convertArrayFromOriginalOracles({removeInputs: process.argv.includes('--remove-inputs')});
  console.log('Array.from original oracles: authenticated source parts converted to Links');
}
