import assert from 'node:assert/strict';
import test from 'node:test';
import { generateCollectionSizeRules } from '../scripts/generate-collection-size-rules.mjs';
import { renderCollectionSize, selectCollectionSizeField } from '../src/translation/collection-size-rules.js';

test('collection size retains its Map and Set owners and one receiver evaluation in each target', () => {
  for (const kind of ['map', 'set', 'record', 'array', 'string']) {
    assert.equal(selectCollectionSizeField('size', kind), kind === 'map' || kind === 'set');
    assert.equal(selectCollectionSizeField('length', kind), false);
  }
  assert.equal(renderCollectionSize('JavaScript', 'read()'), '(read()).size');
  assert.equal(renderCollectionSize('Rust', '(read())'), '((read()).len() as f64)');
  assert.equal(renderCollectionSize('Lean', 'read ()'), '(Float.ofNat read ().size)');
  assert.equal(renderCollectionSize('Rocq', '(read tt)'), '(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length (read tt)))))');
});

test('the complete collection size rules self-translate and replay through both semantic codecs', () => {
  const artifact = generateCollectionSizeRules({ check: true });
  assert.deepEqual(artifact.inventory.map(owner => owner.name), ['selectCollectionSizeField', 'renderCollectionSize']);
  assert.deepEqual(artifact.replays.map(replay => replay.format), ['text', 'binary']);
});
