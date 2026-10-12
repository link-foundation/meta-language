#!/usr/bin/env node
// Pin the shared host representation for the small, independently asserted
// correspondence fixture. This does not run the grammar registry pipeline.
import { readFileSync, writeFileSync } from 'node:fs';

import { deserializeGrammar } from '../src/grammar.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks } from '../src/global-concept-space.js';

const path = new URL('../../parity/fixtures/global-concept-space.json', import.meta.url);
const fixture = JSON.parse(readFileSync(path, 'utf8'));
const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
const space = createGlobalConceptSpace(sources, { records: fixture.records, equivalences: fixture.equivalences });
const links = globalConceptSpaceToLinks(space);
if (process.argv.includes('--check')) {
  if (JSON.stringify(fixture.expectedSpace) !== JSON.stringify(space) || fixture.expectedLinks !== links) throw new Error('global concept fixture drift; run node js/scripts/generate-global-concept-space-fixture.mjs');
  console.log('global concept host fixture matches the shared source grammars');
} else {
  fixture.expectedSpace = space;
  fixture.expectedLinks = links;
  writeFileSync(path, `${JSON.stringify(fixture, null, 2)}\n`);
  console.log('wrote the shared global concept host fixture');
}
