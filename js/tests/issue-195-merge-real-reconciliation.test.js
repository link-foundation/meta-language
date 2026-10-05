// Merging real grammars of one language reconciles their corresponding rules
// instead of concatenating them (requirement I195-MERGE-REAL-RECONCILIATION).
// Each pair of parity/grammars/reconcile/pairs.json is a shipped native
// grammar and the grammars-v4 ANTLR grammar of its language. With
// `reconcile: true` the merge unites the listed rules under one canonical
// name and one concept record, so the pair shares rules; the strict merge of
// the same pair shares none, and `assertMergeShares` rejects it as a
// concatenation. The Rust twin is
// rust/tests/unit/issue_195_merge_real_reconciliation.rs.
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
} from '../src/index.js';
import { Grammar } from '../src/grammar.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const FIXTURE = 'parity/grammars/reconcile/pairs.json';
const root = new URL('../../', import.meta.url);
const read = (file) => readFileSync(new URL(file, root), 'utf8');
const { pairs } = JSON.parse(read(FIXTURE));

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
  const antlr = importAntlr(read(pair.grammarsV4));
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
