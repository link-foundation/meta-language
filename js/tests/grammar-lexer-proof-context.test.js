import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { deserializeGrammar, Grammar, GrammarBuilder as G } from '../src/grammar.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';

const fixture = readProjectData(new URL('../../parity/fixtures/global-concept-space.lino', import.meta.url));
const options = { records: fixture.records, equivalences: fixture.equivalences, formalDefinitions: fixture.formalDefinitions };
const inputs = () => fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
const sourcePlan = (binding) => ({ name: 'source lexer', expression: { kind: 'seq', items: [
  { kind: 'lexicalMode', name: 'quoted' }, { kind: 'lexicalChannel', name: 'CUSTOM' },
  { kind: 'lexicalRule', name: 'ATOM', type: 'ATOM', mode: 'default', channel: 'default', disposition: 'emit', item: G.literal('a'), action: [] },
  { kind: 'lexicalRule', name: 'FALLBACK', type: 'FALLBACK', mode: 'default', channel: 'CUSTOM', disposition: 'emit', item: G.any(), action: [] },
  { kind: 'lexicalBinding', name: 'ATOM', item: G.ref(binding) },
  { kind: 'lexicalObligation', name: '', category: 'option', source: ' superClass = FirstBase; /* original context */ ' },
] } });
const scoped = () => inputs().map((source, index) => ({ ...source, grammar: new Grammar(source.grammar.start,
  source.grammar.rules, source.grammar.sourceFormat, { ...source.grammar.declarations, lexers: [sourcePlan(index === 0 ? 'atom' : 'letter')] }) }));

test('closed global correspondence retains lexical competition, channel, modes and unresolved action context', () => {
  const baseline = scoped();
  const shared = createGlobalConceptSpace(baseline, options);
  assert.equal(shared.mappings['first:atom'], shared.mappings['second:letter']);
  assert.equal(shared.mappings['first:root'], shared.mappings['second:page']);
  assert.ok(shared.proofs.length > 0);
  const changes = [
    (entries) => { [entries[2], entries[3]] = [entries[3], entries[2]]; },
    (entries) => { entries[2].channel = 'CUSTOM'; },
    (entries) => { entries[2].mode = 'quoted'; },
    (entries) => { entries[2].disposition = 'skip'; },
    (entries) => { entries[2].action = [{ operation: 'pushMode', mode: 'quoted' }]; },
    (entries) => { entries.push({ kind: 'lexicalObligation', name: 'ATOM', category: 'predicate', source: 'unresolvedGuard()' }); },
    (entries) => { entries[5].source = entries[5].source.replace('FirstBase', 'SecondBase'); },
  ];
  for (const change of changes) {
    const current = scoped();
    change(current[1].grammar.declarations.lexers[0].expression.items);
    const compared = createGlobalConceptSpace(current, options);
    assert.notEqual(compared.mappings['first:atom'], compared.mappings['second:letter']);
    assert.notEqual(compared.mappings['first:root'], compared.mappings['second:page']);
    assert.equal(compared.proofs.length, 0);
    assert.equal(compared.failures.length, fixture.equivalences.length);
  }
});

test('lexical context reload preserves complete linked declarations and original global fixtures', () => {
  // The preexisting fixture is checked unchanged, including its exact context
  // schema. The new optional declaration must not add empty fields to it.
  assert.deepEqual(createGlobalConceptSpace(inputs(), options), fixture.expectedSpace);
  const space = createGlobalConceptSpace(scoped(), options);
  const serialized = globalConceptSpaceToLinks(space);
  assert.match(serialized, /lexicalRule/u);
  assert.match(serialized, /lexicalBinding/u);
  assert.deepEqual(readGlobalConceptSpaceLinks(serialized), space);
  assert.deepEqual(scoped().map(({ grammar }) => grammar.ruleNames()), fixture.sources.map(({ grammar }) => grammar.rules.map(({ name }) => name)));
});
