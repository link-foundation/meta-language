// Merging real grammars of one language reconciles their corresponding rules
// instead of concatenating them (requirement I195-MERGE-REAL-RECONCILIATION).
// Each pair of parity/grammars/reconcile/pairs.json is a shipped native
// grammar and the grammars-v4 ANTLR grammar of its language. With
// `reconcile: true` the merge unites the listed rules under one canonical
// name and one concept record, so the pair shares rules; the strict merge of
// the same pair shares none, and `assertMergeShares` rejects it as a
// concatenation. The Rust twin is
// rust/tests/unit/issue_195_merge_real_reconciliation.rs.
import { resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import {
  GrammarMergeError,
  assertMergeShares,
  importAntlr,
  importPest,
  mergeGrammars,
  parseGrammarLinks,
  sharedRuleDecisions,
  compileGrammar,
} from '../src/index.js';
import { Grammar, deserializeGrammar } from '../src/grammar.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import { readExternalGrammarAsset } from '../scripts/external-grammar-cache.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

const FIXTURE = 'parity/grammars/reconcile/pairs.json';
const root = new URL('../../', import.meta.url);
const read = (file) => readFileSync(resolveProjectDataPath(new URL(file, root)), 'utf8');
const { pairs } = readProjectData(new URL(FIXTURE, root));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-MERGE-REAL-RECONCILIATION',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-merge-real-reconciliation',
    fixtureFile: FIXTURE,
    assertions,
    testName,
  });
}

function sources(pair) {
  const antlr = importAntlr(readExternalGrammarAsset(pair.grammarsV4).toString('utf8'));
  return [
    { id: 'native', language: pair.language, precedence: 0, grammar: parseGrammarLinks(read(pair.native)) },
    {
      id: 'grammars-v4',
      language: pair.language,
      precedence: 1,
      grammar: new Grammar(pair.entryPoint, antlr.rules, antlr.sourceFormat, antlr.declarations),
    },
  ];
}

const ruleCount = (list) => list.reduce((sum, { grammar }) => sum + grammar.rules.size, 0);

for (const pair of pairs) {
  test(`${pair.language}: corresponding rules of the native and grammars-v4 grammars are reconciled`, () => {
    const list = sources(pair);
    const result = mergeGrammars(list, { reconcile: true });
    assert.equal(result.status, 'complete');
    const [group] = result.groups;
    for (const [native, antlr, basis] of pair.corresponding) {
      const name = group.identities[`native:${native}`];
      assert.equal(group.identities[`grammars-v4:${antlr}`], name, `${antlr} is reconciled with ${native}`);
      const decision = group.decisions.find((entry) => entry.name === name && entry.kind === 'reconciled');
      assert.ok(decision, `${name} is a reconciled decision`);
      assert.equal(decision.basis, basis);
      assert.deepEqual(decision.members, [`native:${native}`, `grammars-v4:${antlr}`]);
      // The merged rule is the native rule, and both sources' rules now
      // stand for one concept record.
      assert.equal(group.grammar.rules.get(name).concept, list[0].grammar.rules.get(native).concept);
    }
    for (const [antlr, concept] of Object.entries(pair.concepts)) {
      assert.equal(group.grammar.rules.get(group.identities[`grammars-v4:${antlr}`]).concept, concept);
    }
    // A reconciled pair is no evidence about the other rules of the sources:
    // every other rule of a source stays its own.
    const reconciled = new Set(pair.corresponding.flatMap(([native, antlr]) => [`native:${native}`, `grammars-v4:${antlr}`]));
    for (const decision of group.decisions.filter(({ kind }) => kind === 'reconciled')) {
      for (const member of decision.members) assert.ok(reconciled.has(member), `${member} is reconciled only as listed`);
    }
    // The merge resolves references through the reconciled names: the
    // grammars-v4 rules that are kept refer to the native rules.
    assert.equal(group.grammar.rules.size, ruleCount(list) - pair.corresponding.length);
    observe(['correspondingRulesReconciled'], `${pair.language} corresponding rules are reconciled`);
  });

  test(`${pair.language}: the reconciled merge shares rules and the strict merge is rejected as a concatenation`, () => {
    const list = sources(pair);
    const reconciled = mergeGrammars(list, { reconcile: true });
    assert.equal(sharedRuleDecisions(reconciled.groups[0]).length, pair.corresponding.length);
    assert.doesNotThrow(() => assertMergeShares(reconciled));
    observe(['sharedRulesNonZero'], `${pair.language} reconciled merge shares rules`);

    const strict = mergeGrammars(list);
    assert.equal(sharedRuleDecisions(strict.groups[0]).length, 0);
    assert.equal(strict.groups[0].grammar.rules.size, ruleCount(list));
    assert.throws(() => assertMergeShares(strict), (error) => error instanceof GrammarMergeError
      && /only concatenates its sources/u.test(error.message)
      && error.failures.length === 1);
    observe(['concatenationRejected'], `${pair.language} strict merge is rejected as a concatenation`);
  });
}

test('reconciliation is deterministic and keeps a reconciled group reusable', () => {
  const [pair] = pairs;
  const first = mergeGrammars(sources(pair), { reconcile: true });
  const second = mergeGrammars(sources(pair).reverse(), { reconcile: true });
  assert.deepEqual(second.groups[0].identities, first.groups[0].identities);
  const again = mergeGrammars(sources(pair), { reconcile: true, previous: first });
  assert.deepEqual(again.reused, [first.groups[0].key]);
  // The fingerprint covers the mode: a strict merge never reuses a
  // reconciled group.
  const strict = mergeGrammars(sources(pair), { previous: first });
  assert.deepEqual(strict.reused, []);
  observe(['correspondingRulesReconciled'], 'reconciliation is deterministic');
});

test('the name tier compares rule names without a leading name of their language', () => {
  const native = importPest(`document = { element* }
element = { "<" ~ name ~ attribute* ~ ">" ~ document ~ "</" ~ name ~ ">" }
attribute = { name ~ "=" ~ name }
name = @{ ASCII_ALPHA+ }`);
  // grammars-v4's HTML grammar names its rules `htmlDocument`, `htmlElement`
  // and `htmlAttribute`; they decompose the constructs differently, so only
  // their names correspond.
  const prefixed = (prefix) => importPest(`${prefix}Document = { ${prefix}Element+ ~ EOI }
${prefix}Element = { "<" ~ tag ~ ${prefix}Attribute* ~ "/>" | "<" ~ tag ~ ">" ~ ${prefix}Element* ~ "</" ~ tag ~ ">" }
${prefix}Attribute = { tag ~ ("=" ~ tag)? }
tag = @{ ASCII_ALPHA ~ ASCII_ALPHANUMERIC* }`);
  const merge = (prefix) => mergeGrammars([
    { id: 'native', language: 'HTML', precedence: 0, grammar: native },
    { id: 'grammars-v4', language: 'HTML', precedence: 1, grammar: prefixed(prefix) },
  ], { reconcile: true });
  assert.deepEqual(
    sharedRuleDecisions(merge('html').groups[0]).map(({ name, members, basis }) => [name, members, basis]),
    ['document', 'element', 'attribute'].map((name) => [
      name,
      [`native:${name}`, `grammars-v4:html${name[0].toUpperCase()}${name.slice(1)}`],
      'name-correspondence',
    ]),
  );
  // Another prefix is part of the name, so nothing corresponds.
  assert.deepEqual(sharedRuleDecisions(merge('xml').groups[0]), []);
  observe(['sharedRulesNonZero'], 'the name tier strips a leading language name');
});

test('reconciliation preserves every source production as an executable alternative', () => {
  const first = importPest('document = { "first" }');
  const second = importPest('document = { "second" }');
  const result = mergeGrammars([
    { id: 'first-source', language: 'Example', grammar: first },
    { id: 'second-source', language: 'Example', precedence: 1, grammar: second },
  ], { reconcile: true });
  const [group] = result.groups;
  assert.equal(group.grammar.rules.size, 1);
  const compiled = compileGrammar(group.grammar);
  for (const source of ['first', 'second']) assert.ok(compiled.parseTree(source).tree, source);
  assert.equal(compiled.parseTree('third').tree ?? null, null);
  assert.deepEqual(group.sourceProductions.map(({ alias, canonical }) => [alias, canonical]), [
    ['first-source:document', 'document'],
    ['second-source:document', 'document'],
  ]);
  assert.deepEqual(group.grammar.rule('document').sourceNames, [
    { source: 'first-source', name: 'document' },
    { source: 'second-source', name: 'document' },
  ]);
  assert.deepEqual(group.sourceProductions[0].rule, first.rule('document'));
  assert.deepEqual(group.sourceProductions[1].rule, second.rule('document'));
});

test('correspondence cannot erase distinct semantic concepts or rule actions', () => {
  const literal = { kind: 'literal', value: 'value' };
  const source = (id, rule) => ({
    id, language: 'Example', grammar: new Grammar('value', new Map([['value', { kind: 'normal', expression: literal, ...rule }]])),
  });
  const concepts = mergeGrammars([
    source('unknown', { concept: 'unknown-value' }),
    source('pointer', { concept: 'null-pointer' }),
  ], { reconcile: true });
  assert.notEqual(concepts.groups[0].identities['unknown:value'], concepts.groups[0].identities['pointer:value']);
  assert.equal(concepts.groups[0].grammar.rules.size, 2);
  const actions = mergeGrammars([
    source('first', { action: [{ operation: 'set', variable: 'result', value: { operation: 'integer', value: 1 } }] }),
    source('second', { action: [{ operation: 'set', variable: 'result', value: { operation: 'integer', value: 2 } }] }),
  ], { reconcile: true });
  assert.notEqual(actions.groups[0].identities['first:value'], actions.groups[0].identities['second:value']);
  assert.equal(actions.groups[0].grammar.rules.size, 2);
});

test('concept and source alias changes invalidate reused merge groups', () => {
  const source = (concept, name) => ({
    id: 'source', language: 'Example', grammar: new Grammar('value', new Map([['value', {
      kind: 'normal', expression: { kind: 'literal', value: 'value' }, concept,
      sourceNames: [{ source: 'upstream', name }],
    }]])),
  });
  const previous = mergeGrammars([source('first-concept', 'original')]);
  const concept = mergeGrammars([source('second-concept', 'original')], { previous });
  assert.deepEqual(concept.recomputed, ['Example@']);
  assert.equal(concept.groups[0].grammar.rule('value').concept, 'second-concept');
  const alias = mergeGrammars([source('first-concept', 'renamed')], { previous });
  assert.deepEqual(alias.recomputed, ['Example@']);
  assert.deepEqual(alias.groups[0].grammar.rule('value').sourceNames, [{ source: 'upstream', name: 'renamed' }]);
});

for (const fixture of readProjectData(new URL('parity/fixtures/grammar-source-production-union.json', root)).cases) {
  test(`shared source union: ${fixture.name}`, () => {
    const list = fixture.sources.map(({ id, text, format }, precedence) => ({
      id, language: 'Example', precedence,
      grammar: format === 'pest' ? importPest(text) : parseGrammarLinks(text),
    }));
    const [group] = mergeGrammars(list, { reconcile: true }).groups;
    assert.equal(group.grammar.rules.size, fixture.rules);
    const compiled = compileGrammar(group.grammar);
    for (const source of fixture.accepted) assert.ok(compiled.parseTree(source).tree, source);
    for (const source of fixture.rejected) assert.equal(compiled.parseTree(source).tree ?? null, null, source);
    for (const [first, second] of fixture.shared) assert.equal(group.identities[first], group.identities[second]);
    for (const [first, second] of fixture.distinct ?? []) assert.notEqual(group.identities[first], group.identities[second]);
    assert.equal(group.sourceProductions.length, list.reduce((count, { grammar }) => count + grammar.rules.size, 0));
  });
}

test('the merge forwards explicit operational premises and retains unknown concepts without them', () => {
  const fixture = readProjectData(new URL('parity/fixtures/global-concept-space.json', root));
  const list = fixture.sources.map(({ grammar, ...source }) => ({ ...source, grammar: deserializeGrammar(grammar) }));
  const options = { conceptRecords: fixture.records, globalEquivalences: fixture.equivalences };
  const grounded = mergeGrammars(list, { ...options, formalDefinitions: fixture.formalDefinitions });
  assert.equal(grounded.status, 'complete');
  const expected = structuredClone(fixture.expectedSpace);
  for (const source of expected.sources) {
    const original = fixture.sources.find(entry => entry.id === source.id);
    assert.ok(original);
    if (original.grammar.sourceFormat != null) source.sourceFormat = original.grammar.sourceFormat;
  }
  assert.deepEqual(grounded.globalConceptSpace, expected);
  const unknown = mergeGrammars(list, options);
  assert.equal(unknown.status, 'incomplete');
  assert.equal(unknown.globalConceptSpace.proofs.length, 0);
  assert.equal(unknown.globalConceptSpace.concepts.length, 6);
  assert.deepEqual(unknown.globalConceptSpace.formalDefinitions, {});
  assert.notEqual(unknown.globalConceptSpace.mappings['first:atom'], unknown.globalConceptSpace.mappings['second:letter']);
  assert.equal(Object.keys(unknown.globalConceptSpace.mappings).length, 6);
});
