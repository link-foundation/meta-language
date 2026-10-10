import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { selfTranslate } from '../src/self-translation.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/self-translation-line-endings.json', import.meta.url), 'utf8'));

test('carried and translated sources retain mixed line endings and layout byte for byte', () => {
  for (const entry of fixture.cases) {
    const translated = selfTranslate(entry.source, entry.language, entry.target);
    assert.ok(translated.items.some(item => item.status === entry.status), entry.name);
    assert.match(translated.code, / envelope-sha256=[a-f0-9]{64} original=/u, entry.name);
    assert.equal(selfTranslate(translated.code, entry.target, entry.language).code, entry.source, entry.name);
    assert.equal(selfTranslate(entry.source, entry.language, entry.language).code, entry.source, entry.name);
  }
});

test('edited bodies and corrupt source envelopes cannot restore stale sources', () => {
  const entry = fixture.cases.find(item => item.name === 'translated-crlf-function');
  const translated = selfTranslate(entry.source, entry.language, entry.target).code;
  const edited = translated.replace('2f64', '9f64');
  assert.notEqual(edited, translated);
  const restored = selfTranslate(edited, entry.target, entry.language).code;
  assert.notEqual(restored, entry.source);
  assert.match(restored, /9f64/u);
  for (const changed of [
    translated.replace('original="', 'original="X'),
    translated.replace(/envelope-sha256=[a-f0-9]{64}/u, `envelope-sha256=${'0'.repeat(64)}`),
    translated.replace('original="', 'original={'),
  ]) assert.notEqual(selfTranslate(changed, entry.target, entry.language).code, entry.source);
});
