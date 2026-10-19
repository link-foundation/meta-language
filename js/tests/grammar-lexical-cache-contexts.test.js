import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import { acceptLexicalCacheContext } from '../src/translation/frontend-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/lexical-cache-contexts.json', import.meta.url));

test('shared fixed lexical cache eligibility retains automatic repair work accounting', () => {
  for (const entry of fixture) assert.equal(acceptLexicalCacheContext(entry.inToken, entry.quiet, entry.repairing), entry.accepted, JSON.stringify(entry));
});
