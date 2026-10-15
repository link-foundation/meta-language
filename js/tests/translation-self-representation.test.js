import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateSelfTranslationRepresentation} from '../scripts/generate-self-translation-representation.mjs';
import {acceptSelfTranslationUtf16Member, requireSelfTranslationUtf16Representation, requireSelfTranslationUtf16Text} from '../src/self-translation-representation-rules.js';
import {selfTranslate} from '../src/self-translation.js';
import {STRING_PRELUDE} from '../src/translation/string-runtime.js';

const policy = readProjectData(new URL('../src/data/self-translation-representation.lino', import.meta.url));
const digest = value => createHash('sha256').update(value).digest('hex');

test('shared representation data retains the actual original selector and complete prelude', () => {
  assert.deepEqual([...policy.originalSelector.source.matchAll(/'([^']+)'/gu)].map(match => match[1]), policy.memberNames);
  assert.equal(digest(policy.originalSelector.source), policy.originalSelector.sha256);
  assert.equal(policy.prelude, STRING_PRELUDE);
  assert.equal(digest(STRING_PRELUDE), policy.preludeSha256);
  assert.equal(readFileSync(new URL('../src/data/self-translation-representation.lino', import.meta.url), 'utf8'), readFileSync(new URL('../../rust/src/data/self-translation-representation.lino', import.meta.url), 'utf8'));
  generateSelfTranslationRepresentation(true);
});

test('shared selection preserves every exact member name and explicit-family Boolean case', () => {
  for (const name of policy.memberNames) assert.equal(acceptSelfTranslationUtf16Member(name), true);
  for (const name of ['', 'Slice', 'slice ', 'length', 'includes', 'startsWith', 'endsWith', 'toUpperCase', 'notSlice']) assert.equal(acceptSelfTranslationUtf16Member(name), false);
  for (const explicit of [false, true]) for (const family of ['JavaScript', 'Rust', 'Lean', 'Rocq']) for (const member of [false, true]) {
    assert.equal(requireSelfTranslationUtf16Representation(explicit, family, member), explicit || family === 'JavaScript' && member);
  }
  assert.equal(requireSelfTranslationUtf16Text('string'), true);
  for (const kind of ['', 'float', 'bool', 'array', 'option', 'data']) assert.equal(requireSelfTranslationUtf16Text(kind), false);
});

test('complete actual numeric slice sibling fixture retains the selected full prelude and roundtrip', () => {
  const fixture = readProjectData(new URL('../../parity/fixtures/self-translation-bindings.lino', import.meta.url));
  const row = fixture.cases.find(value => value.name === 'array-slice-copy');
  assert.ok(row);
  assert.equal(digest(row.source), '2516e06000b6eb83f19b43266fcbda6f97a21e1b9d422ddbc560fbb5af6d814f');
  const emitted = selfTranslate(row.source, 'JavaScript', 'Rust');
  assert.equal(digest(emitted.code), '243076e865f01be5acdeb53604ad4ab2dc91baa4defac337a3138125a2058310');
  assert.ok(emitted.code.includes(STRING_PRELUDE));
  assert.equal(selfTranslate(emitted.code, 'Rust', 'JavaScript').code, row.source);
});

test('unselected numeric source remains unchanged while comments do not select text representation', () => {
  const source = '/** @param {number} value @returns {number} */\nexport function keep(value) { return value; }\n';
  assert.equal(selfTranslate(source, 'JavaScript', 'Rust').code.includes(STRING_PRELUDE), false);
  assert.equal(selfTranslate('// value.slice() in a comment\n' + source, 'JavaScript', 'Rust').code.includes(STRING_PRELUDE), false);
  assert.ok(selfTranslate(source, 'JavaScript', 'Rust', {utf16Strings: true}).code.includes(STRING_PRELUDE));
});
