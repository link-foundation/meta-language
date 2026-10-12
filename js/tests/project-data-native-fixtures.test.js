import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';
import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import { renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';

const logical = name => new URL(`../../parity/fixtures/native-grammars/${name}.json`, import.meta.url);

test('the Agda corpus retains its original values through its logical fixture alias', () => {
  const file = logical('agda-corpus');
  const physical = resolveProjectDataPath(file);
  assert.match(physical, /agda-corpus\.lino$/u);
  assert.equal(existsSync(file), false);
  const fixture = readProjectData(file);
  assert.equal(fixture.cases.length, 75);
  assert.deepEqual(Object.keys(fixture), ['generatedBy', 'sourceSha256', 'cases']);
  assert.deepEqual([fixture.cases[0].file, fixture.cases[0].title], ['agda-stdlib/Algebra.txt', 'module imports']);
  assert.deepEqual([fixture.cases.at(-1).file, fixture.cases.at(-1).title], ['expression/record.txt', 'Record update in fields']);
  // Captured from the decoded JSON before automatic conversion, not recaptured
  // from a grammar or an oracle. This covers every retained source byte.
  assert.equal(createHash('sha256').update(JSON.stringify(fixture)).digest('hex'),
    '303b1de714a54fc54bbcd5263af62e05e750e3d6edeaab960eb8298aaebd175d');
  assert.equal(renderFixture(fixture), readFileSync(physical, 'utf8'));
});

test('native fixture generation keeps canonical inspectable storage and oracle rows', () => {
  const file = logical('csv');
  const fixture = readProjectData(file);
  assert.equal(fixture.matches.length, 123);
  assert.equal(createHash('sha256').update(JSON.stringify(fixture)).digest('hex'),
    'b9f492298f8810c314054b11139fd719da61259ef72760909796716348466d7d');
  const text = readFileSync(resolveProjectDataPath(file), 'utf8');
  assert.equal(renderFixture(fixture), text);
  assert.ok(text.includes('(term: structured%20data)'), 'the data root is explicit');
  assert.ok(text.includes('(term: key%20value%20pair)'), 'object entries are inspectable pairs');
  assert.ok(text.includes('(term: array%20value)'), 'oracle rows are inspectable arrays');
});
