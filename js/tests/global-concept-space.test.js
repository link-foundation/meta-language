import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { Parser } from 'links-notation';
import { deserializeGrammar, Grammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { GLOBAL_CONCEPT_PROOF_METHOD, createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks, translateGlobalConstruct } from '../src/global-concept-space.js';
import { verifyGlobalConceptCorrespondence } from '../src/global-concept-rules.js';
import { encodeFormalConceptDefinition } from '../src/formal-concept-definition.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/global-concept-space.json', import.meta.url), 'utf8'));
const sources = () => fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
const options = (extra = {}) => ({ records: fixture.records, equivalences: fixture.equivalences, formalDefinitions: fixture.formalDefinitions, ...extra });
const build = (input = sources(), extra = {}) => createGlobalConceptSpace(input, options(extra));
const rewritten = (source, transform, declarations = source.grammar.declarations) => ({ ...source, grammar: new Grammar(source.grammar.start, [...source.grammar.rules].map(([name, rule]) => [name, transform(rule)]), source.grammar.sourceFormat, declarations) });
const observe = (requirementId, assertions, testName) => recordIssue195Observations({
  requirementId, suffix: 'behavior', fixtureId: 'global-concept-space',
  fixtureFile: 'parity/fixtures/global-concept-space.json', assertions, testName,
});

test('every imported language contributes all productions before proven deduplication', () => {
  const input = sources();
  const before = input.map(({ grammar }) => JSON.stringify([...grammar.rules]));
  const space = build(input);
  assert.deepEqual(space, fixture.expectedSpace);
  assert.equal(space.sources.length, 2);
  assert.equal(Object.keys(space.mappings).length, 6);
  assert.equal(space.concepts.flatMap(({ origins }) => origins).length, 6);
  assert.equal(space.concepts.flatMap(({ definitions }) => definitions).length, 6);
  assert.equal(space.concepts.length, 4);
  assert.equal(space.mappings['first:root'], space.mappings['second:page']);
  assert.equal(space.mappings['first:atom'], space.mappings['second:letter']);
  assert.notEqual(space.mappings['first:unique'], space.mappings['second:unique']);
  assert.deepEqual(space.failures, []);
  assert.deepEqual(input.map(({ grammar }) => JSON.stringify([...grammar.rules])), before);
  observe('I195-GLOBAL-CONCEPT-SPACE', ['allRulesInOneSpace'], 'every imported language contributes all productions before proven deduplication');
  observe('I195-GLOBAL-CONCEPT-COPY', ['copiesBeforeDeduplication'], 'every imported language contributes all productions before proven deduplication');
  observe('I195-GLOBAL-CONCEPT-REUSE', ['reuseEveryVerifiedCorrespondence'], 'every imported language contributes all productions before proven deduplication');
});

test('recursive concept sharing carries a closed one-to-one witness with qualified origins', () => {
  const space = build();
  const proof = space.proofs.find(({ correspondence }) => correspondence.some(([first, second]) => first === 'first:root' && second === 'second:page'));
  assert.equal(proof.method, GLOBAL_CONCEPT_PROOF_METHOD);
  assert.deepEqual(proof.correspondence, [['first:root', 'second:page'], ['first:atom', 'second:letter']]);
  const concept = space.concepts.find(({ id }) => id === space.mappings['first:root']);
  assert.deepEqual(concept.origins.map(({ language, edition, rule }) => ({ language, edition, rule })), [
    { language: 'first-language', edition: '1', rule: 'root' },
    { language: 'second-language', edition: '2', rule: 'page' },
  ]);
  assert.deepEqual(concept.origins[0].sourceNames, [{ source: 'upstream:first', name: 'original_root' }]);
  observe('I195-GLOBAL-CONCEPT-EQUIVALENCE', ['recursiveOneToOneProof'], 'recursive concept sharing carries a closed one-to-one witness with qualified origins');
  observe('I195-GLOBAL-CONCEPT-PROVENANCE', ['preserveQualifiedProvenance'], 'recursive concept sharing carries a closed one-to-one witness with qualified origins');
});

test('names, equal accepted text and unregistered concept claims cannot prove shared meaning', () => {
  const input = sources().map((source) => rewritten(source, (rule) => ({ ...rule, concept: undefined })));
  const unknown = build(input, { records: [] });
  assert.equal(unknown.proofs.length, 0);
  assert.equal(unknown.concepts.length, 6);
  assert.equal(unknown.failures.length, 2);
  const claimed = build(sources(), { records: [] });
  assert.equal(claimed.proofs.length, 0);
  assert.equal(claimed.concepts.length, 6);
  const prose = build(sources(), { formalDefinitions: {} });
  assert.equal(prose.proofs.length, 0, 'matching catalog identifiers and glosses do not prove operational meaning');
  const counterfeit = build(sources(), { formalDefinitions: { 'fixture.letter': { links: '(claim same-meaning)', root: 1 } } });
  assert.equal(counterfeit.proofs.length, 0, 'unsupported linked claims are unknown');
  observe('I195-GLOBAL-CONCEPT-UNPROVED', ['rejectUnprovedEquivalence'], 'names, equal accepted text and unregistered concept claims cannot prove shared meaning');
});

test('distinct or ambiguous meanings remain separate even for identical definitions', () => {
  const [first, second] = sources();
  const changed = rewritten(second, (rule) => rule.name === 'letter' ? { ...rule, concept: 'fixture.other-letter' } : rule);
  const otherRecord = { id: 'fixture.other-letter', definition: 'A letter with a separate semantic role.', constraints: ['This semantic role is distinct.'], sourceAliases: [{ source: 'second', name: 'letter' }] };
  const records = fixture.records.map((record) => ({ ...record, sourceAliases: record.sourceAliases.filter(({ source, name }) => !(source === 'second' && name === 'letter')) }));
  const distinct = build([first, changed], { records: [...records, otherRecord] });
  assert.notEqual(distinct.mappings['first:atom'], distinct.mappings['second:letter']);
  assert.notEqual(distinct.mappings['first:root'], distinct.mappings['second:page']);
  const ambiguous = build([first, second], { records: [...fixture.records, otherRecord] });
  assert.deepEqual(ambiguous.ambiguities, [{ alias: 'second:letter', meanings: ['fixture.letter', 'fixture.other-letter'] }]);
  assert.notEqual(ambiguous.mappings['first:atom'], ambiguous.mappings['second:letter']);
  observe('I195-GLOBAL-CONCEPT-DISTINCTIONS', ['preserveDistinctMeanings'], 'distinct or ambiguous meanings remain separate even for identical definitions');
  observe('I195-GLOBAL-CONCEPT-UNPROVED', ['preserveAmbiguity'], 'distinct or ambiguous meanings remain separate even for identical definitions');
});

test('operational matching, scanner, binding and action context constrain correspondence', () => {
  const [first, second] = sources();
  const declarations = [
    { matching: 'peg' },
    { extras: [{ kind: 'literal', value: ' ' }] },
    { scanners: [{ name: 'scanner', tokens: ['external'], operations: [{ operation: 'advance' }] }] },
    { imports: ['external-grammar'] },
  ];
  for (const context of declarations) {
    const space = build([first, rewritten(second, (rule) => rule, context)]);
    assert.equal(space.proofs.length, 0);
    assert.equal(space.failures.length, 2);
  }
  for (const attribute of [{ parameters: ['value'] }, { channel: 'comment' }, { modes: ['quoted'] }, { action: [{ operation: 'advance' }] }]) {
    const space = build([first, rewritten(second, (rule) => rule.name === 'letter' ? { ...rule, ...attribute } : rule)]);
    assert.notEqual(space.mappings['first:atom'], space.mappings['second:letter']);
    assert.notEqual(space.mappings['first:root'], space.mappings['second:page']);
  }
  observe('I195-GLOBAL-CONCEPT-EQUIVALENCE', ['matchingOperationalContext'], 'operational matching, scanner, binding and action context constrain correspondence');
});

test('proofs cannot collapse a many-to-one recursive rule correspondence', () => {
  const records = [
    { id: 'fixture.root', definition: 'A pair of letter references.', constraints: ['The references retain their bindings.'], sourceAliases: [] },
    { id: 'fixture.leaf', definition: 'The letter x.', constraints: ['One scalar is consumed.'], sourceAliases: [] },
  ];
  const leaf = { kind: 'normal', concept: 'fixture.leaf', expression: { kind: 'literal', value: 'x' } };
  const make = (id, references, names) => ({ id, language: id, grammar: new Grammar('root', [
    ['root', { kind: 'normal', concept: 'fixture.root', expression: { kind: 'seq', items: references.map((name) => ({ kind: 'ref', name })) } }],
    ...names.map((name) => [name, leaf]),
  ]) });
  const primitive = fixture.formalDefinitions['fixture.letter'];
  const space = createGlobalConceptSpace([make('one', ['leaf', 'leaf'], ['leaf']), make('two', ['first', 'second'], ['first', 'second'])], { records, formalDefinitions: { 'fixture.root': primitive, 'fixture.leaf': primitive }, equivalences: [['one:root', 'two:root']] });
  assert.notEqual(space.mappings['one:root'], space.mappings['two:root']);
  assert.deepEqual(space.failures, [{ members: ['one:root', 'two:root'], reason: 'no-one-to-one-proof' }]);
});

test('global identities are reproducible, incremental and stable under reordering', () => {
  const initial = build();
  const reordered = build(sources().reverse().map((source) => ({ ...source, grammar: new Grammar(source.grammar.start, [...source.grammar.rules].reverse(), source.grammar.sourceFormat) })));
  assert.deepEqual(reordered.mappings, initial.mappings);
  assert.deepEqual(build(sources(), { previous: initial }), initial);
  const third = { ...sources()[0], id: 'earlier', language: 'third-language' };
  const expanded = build([...sources(), third], { previous: initial });
  for (const [origin, identity] of Object.entries(initial.mappings)) assert.equal(expanded.mappings[origin], identity, origin);
  assert.equal(expanded.mappings['earlier:atom'], initial.mappings['first:atom']);
  const [first, second] = sources();
  const changed = rewritten(second, (rule) => rule.name === 'letter' ? { ...rule, expression: { kind: 'literal', value: 'β' } } : rule);
  const split = build([first, changed], { previous: initial });
  assert.equal(split.mappings['first:atom'], initial.mappings['first:atom']);
  assert.notEqual(split.mappings['second:letter'], initial.mappings['first:atom']);
  assert.equal(new Set(split.concepts.map(({ id }) => id)).size, split.concepts.length);
  observe('I195-GLOBAL-CONCEPT-SPACE', ['stableIdentities'], 'global identities are reproducible, incremental and stable under reordering');
});

test('the global links representation retains definitions, source aliases and proof edges', () => {
  const space = build();
  const links = globalConceptSpaceToLinks(space);
  assert.equal(links, fixture.expectedLinks);
  const parsed = new Parser({ comments: false }).parse(links);
  assert.ok(parsed.length > space.concepts.length);
  assert.match(links, /\(concept-origin .*first%3Aroot/);
  assert.match(links, /\(source-alias first%3Aroot upstream%3Afirst original_root\)/);
  assert.match(links, /\(concept-proof recursive-structural-bijection/);
  assert.equal((links.match(/\(concept-definition /gu) ?? []).length, 6);
  assert.match(links, /\(literal %CE%B1\)/);
  assert.equal(globalConceptSpaceToLinks(build()), links);
  const reloaded = readGlobalConceptSpaceLinks(links, { records: fixture.records });
  assert.deepEqual(reloaded, space);
  assert.equal(globalConceptSpaceToLinks(reloaded), links);
  const changed = links.replace(/\(literal %CE%B1\)/u, '(literal %CE%B2)');
  assert.throws(() => readGlobalConceptSpaceLinks(changed, { records: fixture.records }), /unproved global concept correspondence/);
  const removedProofs = links.split('\n').filter((line) => !line.startsWith('(concept-proof ')).join('\n');
  assert.throws(() => readGlobalConceptSpaceLinks(removedProofs, { records: fixture.records }), /lacks its proof witnesses/);
  observe('I195-GLOBAL-CONCEPT-SERIALIZATION', ['linksRoundTrip'], 'the global links representation retains definitions, source aliases and proof edges');
});

test('shared proof fixtures validate and reject exact correspondences in the portable decision procedure', () => {
  for (const entry of fixture.proofCases) {
    assert.equal(verifyGlobalConceptCorrespondence(entry.shapes, entry.meanings, entry.offsets, entry.edges, entry.first, entry.second), entry.expected, entry.name);
  }
  observe('I195-GLOBAL-CONCEPT-RUNTIME-PARITY', ['generatedRuntimeDecisions'], 'shared proof fixtures validate and reject exact correspondences in the portable decision procedure');
});

test('source-free reload preserves explicit and implicit starts, rule order and empty grammars', () => {
  const [first] = sources();
  const roots = [
    { ...first, id: 'explicit', grammar: new Grammar('root', [...first.grammar.rules].reverse()) },
    { ...first, id: 'implicit', grammar: new Grammar(null, [...first.grammar.rules].reverse()) },
    { id: 'empty', language: 'empty-language', grammar: new Grammar(null, []) },
  ];
  const space = createGlobalConceptSpace(roots, { records: fixture.records });
  const loaded = readGlobalConceptSpaceLinks(globalConceptSpaceToLinks(space), { records: fixture.records });
  assert.deepEqual(loaded, space);
  assert.equal(loaded.sources.find(({ id }) => id === 'explicit').start, 'root');
  const implicit = loaded.sources.find(({ id }) => id === 'implicit');
  assert.equal(implicit.start, null);
  assert.equal(implicit.ruleOrder[0], 'unique');
  const empty = loaded.sources.find(({ id }) => id === 'empty');
  assert.equal(empty.start, null);
  assert.deepEqual(empty.ruleOrder, []);
  assert.throws(() => readGlobalConceptSpaceLinks(globalConceptSpaceToLinks(space).replace('(start defined root)', '(start defined missing)'), { records: fixture.records }), /start or rule order/);
});

test('automatic proof discovery uses catalog source identities for real native grammar concepts', () => {
  const imported = ['scheme', 'racket'].map((language) => {
    const grammar = parseGrammarLinks(readFileSync(new URL(`../src/data/native-grammars/${language}.lino`, import.meta.url), 'utf8'));
    // Raw source imports have no preassigned concept metadata. Native aliases
    // nominate candidates; explicit trivia interpretation is the semantic
    // premise, alongside the complete production and parser context.
    return { id: `pinned-${language}`, language: language === 'scheme' ? 'Scheme' : 'Racket', grammar: rewritten({ grammar }, (rule) => ({ ...rule, concept: undefined })).grammar };
  });
  const trivia = encodeFormalConceptDefinition(undefined, { model: 'source trivia interpretation' });
  const space = createGlobalConceptSpace(imported, { formalDefinitions: { 'pinned-scheme:block_comment': trivia, 'pinned-racket:block_comment': trivia } });
  const first = space.mappings['pinned-scheme:block_comment'];
  const second = space.mappings['pinned-racket:block_comment'];
  assert.equal(first, second);
  const shared = space.concepts.find(({ id }) => id === first);
  assert.equal(shared.origins.length, 2);
  assert.ok(shared.proofs.length > 0);
  assert.equal(Object.keys(space.mappings).length, imported.reduce((sum, { grammar }) => sum + grammar.rules.size, 0));
  assert.notEqual(space.mappings['pinned-scheme:program'], space.mappings['pinned-racket:program']);
  const translated = translateGlobalConstruct(space, 'pinned-scheme:block_comment', 'rkt');
  assert.equal(translated.relation, 'translated');
  assert.equal(translated.concept, first);
  assert.deepEqual(translated.origins.map(({ alias }) => alias), ['pinned-racket:block_comment']);
  assert.ok(translated.proofs.length > 0);
  assert.equal(translateGlobalConstruct(space, 'pinned-scheme:program', 'Racket').relation, 'untranslatable');
  assert.deepEqual(readGlobalConceptSpaceLinks(globalConceptSpaceToLinks(space)), space);
});

test('global construct queries preserve unknown meanings, target ambiguity and editions', () => {
  const first = sources();
  const third = { ...first[1], id: 'third', edition: '3' };
  const space = build([...first, third]);
  const ambiguous = translateGlobalConstruct(space, 'first:root', 'second-language');
  assert.equal(ambiguous.relation, 'ambiguous');
  assert.deepEqual(ambiguous.origins.map(({ alias }) => alias), ['second:page', 'third:page']);
  const edition = translateGlobalConstruct(space, 'first:root', 'second-language', { edition: '3' });
  assert.equal(edition.relation, 'translated');
  assert.deepEqual(edition.origins.map(({ alias }) => alias), ['third:page']);
  assert.equal(translateGlobalConstruct(space, 'first:unique', 'second-language').relation, 'untranslatable');
  assert.deepEqual(translateGlobalConstruct(space, 'missing:rule', 'second-language'), { relation: 'unknown', concept: null, origins: [], proofs: [] });
  const duplicate = { id: 'fixture.other-letter', definition: 'A separate meaning of the letter.', constraints: ['The role is distinct.'], sourceAliases: [{ source: 'first', name: 'root' }] };
  const uncertain = build(first, { records: [...fixture.records, duplicate] });
  assert.deepEqual(translateGlobalConstruct(uncertain, 'first:root', 'second-language'), { relation: 'ambiguous', concept: null, origins: [], proofs: [] });
});

test('invalid sources and unresolved required origins are explicit', () => {
  assert.throws(() => build([...sources(), sources()[0]]), /duplicate global concept source/);
  assert.throws(() => createGlobalConceptSpace([{ id: 'first', language: 'language', grammar: {} }]), /imported grammar/);
  assert.throws(() => build(sources(), { equivalences: [['first:root']] }), /exactly two/);
  assert.deepEqual(build(sources(), { equivalences: [['first:root', 'missing:rule']] }).failures, [{ members: ['first:root', 'missing:rule'], reason: 'unknown-origin' }]);
});
