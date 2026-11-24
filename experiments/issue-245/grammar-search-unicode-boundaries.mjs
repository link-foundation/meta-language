import assert from 'node:assert/strict';
import { createGrammarParser, importEbnf } from '../../js/src/index.js';
import { readProjectData } from '../../js/scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/grammar-search-unicode-boundaries.lino', import.meta.url));
const parser = createGrammarParser(importEbnf(fixture.grammar));
for (const wordBoundaries of [false, true]) {
  const matches = parser.findAll(fixture.source, { wordBoundaries });
  const offsets = matches.map(({ start, end, tree }) => ({ utf16: [start, end], utf8: [tree.start, tree.end] }));
  console.log(JSON.stringify({ wordBoundaries, offsets }));
  assert.deepEqual(offsets, wordBoundaries ? fixture.matches : fixture.unrestrictedMatches);
}
