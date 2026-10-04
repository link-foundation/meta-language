// Adds the rename case and the reviewed merge expectations to
// parity/fixtures/grammar-merge.json. The merge expectations were checked by
// hand against the sources before they were written (see the PR notes).
import { readFileSync, writeFileSync } from 'node:fs';
import { importPest, mergeGrammars } from '../js/src/index.js';

const file = new URL('../parity/fixtures/grammar-merge.json', import.meta.url);
const fixture = JSON.parse(readFileSync(file, 'utf8'));
const sources = (list) => list.map(({ text, format, ...rest }) => ({ ...rest, grammar: importPest(text) }));
const result = mergeGrammars(sources(fixture.sources), { samples: fixture.samples });
const changed = fixture.sources.map((source) =>
  fixture.upstreamChange.source === source.id ? { ...source, text: fixture.upstreamChange.text } : source);
const remerged = mergeGrammars(sources(changed), { samples: fixture.samples, previous: result });
const fresh = mergeGrammars(sources(changed), { samples: fixture.samples });
const required = mergeGrammars(sources(fixture.sources), {
  samples: fixture.samples,
  requiredEquivalences: fixture.requiredEquivalences,
});

const ref = (name) => ({ kind: 'ref', name });
const lit = (value) => ({ kind: 'literal', value });
const seq = (...items) => ({ kind: 'seq', items });
const first = (...items) => ({ kind: 'choice', items, ordered: true });
const rename = {
  namespace: 'arithmetic',
  grammar: {
    schemaVersion: 1,
    start: 'expression',
    sourceFormat: 'peg',
    rules: [
      { name: 'expression', kind: 'normal', expression: seq({ kind: 'capture', label: 'value', item: ref('value') }, { kind: 'optional', item: seq(lit('+'), ref('expression')) }) },
      { name: 'value', kind: 'normal', expression: first(ref('number'), seq(lit('('), ref('expression'), lit(')'))) },
      { name: 'qualified', kind: 'normal', expression: first(ref('arithmetic.value'), ref('arithmetic::value'), ref('library.value')) },
      { name: 'number', kind: 'atomic', expression: { kind: 'repeat1', item: { kind: 'charRange', start: '0', end: '9' } } },
      { name: 'spaced', kind: 'normal', expression: seq(ref('digit'), ref('value')) },
    ],
  },
  steps: [{ from: 'value', to: 'operand' }, { from: 'expression', to: 'sum' }],
  afterReload: { from: 'operand', to: 'term' },
  collisions: [{ from: 'value', to: 'number' }, { from: 'value', to: 'digit' }, { from: 'missing', to: 'other' }],
  expected: {
    grammar: {
      schemaVersion: 1,
      start: 'sum',
      sourceFormat: 'peg',
      rules: [
        { name: 'sum', kind: 'normal', expression: seq({ kind: 'capture', label: 'value', item: ref('term') }, { kind: 'optional', item: seq(lit('+'), ref('sum')) }) },
        { name: 'term', kind: 'normal', expression: first(ref('number'), seq(lit('('), ref('sum'), lit(')'))) },
        { name: 'qualified', kind: 'normal', expression: first(ref('arithmetic.term'), ref('arithmetic::term'), ref('library.value')) },
        { name: 'number', kind: 'atomic', expression: { kind: 'repeat1', item: { kind: 'charRange', start: '0', end: '9' } } },
        { name: 'spaced', kind: 'normal', expression: seq(ref('digit'), ref('term')) },
      ],
    },
    aliases: [{ canonical: 'term', original: 'value' }, { canonical: 'sum', original: 'expression' }],
    collisionKinds: ['collision', 'collision', 'unknown-rule'],
  },
};

const pick = (group) => ({
  key: group.key,
  sources: group.sources,
  start: group.grammar.start,
  rules: group.grammar.ruleNames(),
  identities: group.identities,
  decisions: group.decisions,
  nominations: group.nominations,
  alternatives: group.alternatives,
});
fixture.rename = rename;
fixture.expected = {
  groups: result.groups.map(pick),
  alternatives: result.alternatives,
  failures: required.failures,
  remerge: {
    reused: remerged.reused,
    recomputed: remerged.recomputed,
    identities: remerged.groups[0].identities,
    rules: remerged.groups[0].grammar.ruleNames(),
    identitiesWithoutPrevious: fresh.groups[0].identities,
  },
};
writeFileSync(file, `${JSON.stringify(fixture, null, 2)}\n`);
