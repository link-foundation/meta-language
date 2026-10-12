import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { acceptLexicalCacheContext } from '../src/translation/frontend-rules.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/lexical-cache-contexts.json', import.meta.url), 'utf8'));

test('shared fixed lexical cache eligibility retains automatic repair work accounting', () => {
  for (const entry of fixture) assert.equal(acceptLexicalCacheContext(entry.inToken, entry.quiet, entry.repairing), entry.accepted, JSON.stringify(entry));
});
