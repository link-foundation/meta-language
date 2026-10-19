#!/usr/bin/env node
// Pin the shared host representation for the small, independently asserted
// correspondence fixture. This does not run the grammar registry pipeline.
import { readProjectData, writeProjectData } from './project-data.mjs';

import { deserializeGrammar } from '../src/grammar.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks } from '../src/global-concept-space.js';

const identityPath = new URL('../../parity/fixtures/global-concept-identities.lino', import.meta.url);
const identityFixture = {
  schemaVersion: 1,
  restoration: { removedSource: 0, changedRule: 'atom', changedLiteral: 'β', concepts: 6, uniqueIdentities: 6, proofs: 0, failures: 2 },
  histories: [
    { identities: ['old'], aliases: ['source:rule'], uniqueIdentities: 1, uniqueAliases: 1, expected: true },
    { identities: ['old', 'old'], aliases: ['a', 'b'], uniqueIdentities: 1, uniqueAliases: 2, expected: false },
    { identities: ['a', 'b'], aliases: ['same', 'same'], uniqueIdentities: 2, uniqueAliases: 1, expected: false },
    { identities: [''], aliases: ['a'], uniqueIdentities: 1, uniqueAliases: 1, expected: false },
    { identities: ['a'], aliases: [''], uniqueIdentities: 1, uniqueAliases: 1, expected: false },
  ],
  selections: [
    { preferred: '', fallback: 'base', reserved: [], assigned: [], expected: 'base' },
    { preferred: '', fallback: 'base', reserved: ['base', 'base.new'], assigned: ['base.new.new'], expected: 'base.new.new.new' },
    { preferred: 'old', fallback: 'base', reserved: ['old'], assigned: [], expected: 'old' },
    { preferred: 'old', fallback: 'base', reserved: ['old'], assigned: ['old'], expected: '' },
    { preferred: '', fallback: 'base', reserved: ['base.10', 'base.2'], assigned: [], expected: 'base' },
  ],
};
if (process.argv.includes('--check')) {
  if (JSON.stringify(readProjectData(identityPath)) !== JSON.stringify(identityFixture)) throw new Error('global concept identity controls are stale');
} else writeProjectData(identityPath, identityFixture);

const path = new URL('../../parity/fixtures/global-concept-space.json', import.meta.url);
const fixture = readProjectData(path);
const sources = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
const space = createGlobalConceptSpace(sources, { records: fixture.records, equivalences: fixture.equivalences, formalDefinitions: fixture.formalDefinitions });
const links = globalConceptSpaceToLinks(space);
if (process.argv.includes('--check')) {
  if (JSON.stringify(fixture.expectedSpace) !== JSON.stringify(space) || fixture.expectedLinks !== links) throw new Error('global concept fixture drift; run node js/scripts/generate-global-concept-space-fixture.mjs');
  console.log('global concept host fixture matches the shared source grammars');
} else if (JSON.stringify(fixture.expectedSpace) !== JSON.stringify(space) || fixture.expectedLinks !== links) {
  fixture.expectedSpace = space;
  fixture.expectedLinks = links;
  writeProjectData(path, fixture);
  console.log('wrote the shared global concept host fixture');
}
