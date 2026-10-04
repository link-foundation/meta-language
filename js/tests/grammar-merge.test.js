// Multi-source grammar merge and binding-aware rename: equivalent rules are
// merged only on a recursive structural proof, homonyms and lookalikes stay
// distinct and explicit, merging is reproducible, idempotent, order
// independent and incremental, and renaming follows references, recursion,
// qualified names and reloads while keeping source aliases (requirements
// I195-MERGE-DETERMINISM, I195-MERGE-MEANING-AWARE-DEDUPLICATION,
// I195-MERGE-BINDING-AWARE-RENAME and I195-MERGE-UNCERTAINTY-PRESERVED). The
// Rust twin is rust/tests/unit/grammar_merge.rs.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  GRAMMAR_MERGE_METHOD,
  GrammarMergeError,
  GrammarRenameError,
  assertMergeComplete,
  deserializeGrammar,
  importBnf,
  importPest,
  mergeGrammars,
  normalizedRuleDefinition,
  parseWithGrammar,
  renameGrammarRule,
  restoreSourceNames,
  serializeGrammar,
} from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const FIXTURE = 'parity/fixtures/grammar-merge.json';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const fixture = JSON.parse(readFileSync(path.join(root, FIXTURE), 'utf8'));

function observe(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: FIXTURE,
    assertions,
    testName,
  });
}

const sources = (list = fixture.sources) => list.map(({ text, format, ...rest }) => {
  assert.equal(format, 'pest');
  return { ...rest, grammar: importPest(text) };
});
const changedSources = () => sources(fixture.sources.map((source) =>
  source.id === fixture.upstreamChange.source ? { ...source, text: fixture.upstreamChange.text } : source));
const merge = (list = sources(), options = {}) => mergeGrammars(list, { samples: fixture.samples, ...options });
const group = (result, key) => result.groups.find((entry) => entry.key === key);
const decisions = (entry, kind) => entry.decisions.filter((decision) => decision.kind === kind);

test('the merge decides every fixture group exactly as recorded', () => {
  const result = merge();
  assert.equal(result.status, 'complete');
  assert.deepEqual(result.alternatives, fixture.expected.alternatives);
  assert.deepEqual(result.groups.map(({ key }) => key), fixture.expected.groups.map(({ key }) => key));
  for (const expected of fixture.expected.groups) {
    const actual = group(result, expected.key);
    assert.deepEqual(actual.sources, expected.sources, expected.key);
    assert.equal(actual.grammar.start, expected.start, expected.key);
    assert.deepEqual(actual.grammar.ruleNames(), expected.rules, expected.key);
    assert.deepEqual(actual.identities, expected.identities, expected.key);
    assert.deepEqual(actual.decisions, expected.decisions, expected.key);
    assert.deepEqual(actual.nominations, expected.nominations, expected.key);
    assert.deepEqual(actual.alternatives, expected.alternatives, expected.key);
  }
});

test('merging is reproducible, idempotent and independent of the source order', () => {
  const first = merge();
  const second = merge();
  assert.deepEqual(second, first);
  assert.equal(second.groups[0].fingerprint, first.groups[0].fingerprint);
  observe('I195-MERGE-DETERMINISM', ['reproducibleOutput'], 'merging is reproducible');

  for (const entry of first.groups) {
    const again = merge([{ id: 'merged', language: entry.language, edition: entry.edition, grammar: entry.grammar }]);
    assert.deepEqual(again.groups[0].grammar.normalized(), entry.grammar.normalized(), entry.key);
    assert.deepEqual(decisions(again.groups[0], 'merged'), [], entry.key);
  }
  const reused = merge(sources(), { previous: first });
  assert.deepEqual(reused.groups, first.groups);
  assert.deepEqual(reused.recomputed, []);
  observe('I195-MERGE-DETERMINISM', ['idempotent'], 'merging a merged grammar changes nothing');

  const reversed = merge(sources().reverse());
  assert.deepEqual(reversed, first);
  const reorderedRules = sources().map((source) => {
    const rules = new Map([...source.grammar.rules].reverse());
    return { ...source, grammar: new source.grammar.constructor(source.grammar.startRule().name, rules, source.grammar.sourceFormat) };
  });
  const reorderedResult = merge(reorderedRules);
  for (const entry of first.groups) {
    const other = group(reorderedResult, entry.key);
    assert.deepEqual(other.identities, Object.fromEntries(Object.keys(other.identities).map((alias) => [alias, entry.identities[alias]])));
    assert.deepEqual(new Set(other.grammar.ruleNames()), new Set(entry.grammar.ruleNames()));
    for (const name of entry.grammar.ruleNames()) {
      assert.deepEqual(other.grammar.rule(name).expression, entry.grammar.rule(name).expression, name);
    }
  }
  observe('I195-MERGE-DETERMINISM', ['stableUnderReordering'], 'merging ignores the order of sources and rules');
});

test('an upstream change re-merges only its group and keeps established canonical identities', () => {
  const previous = merge();
  const remerged = merge(changedSources(), { previous });
  const expected = fixture.expected.remerge;
  assert.deepEqual(remerged.reused, expected.reused);
  assert.deepEqual(remerged.recomputed, expected.recomputed);
  for (const key of expected.reused) assert.equal(group(remerged, key), group(previous, key));
  const changed = group(remerged, expected.recomputed[0]);
  assert.notEqual(changed.fingerprint, group(previous, expected.recomputed[0]).fingerprint);
  assert.deepEqual(changed.identities, expected.identities);
  assert.deepEqual(changed.grammar.ruleNames(), expected.rules);
  assert.ok(changed.grammar.rule('product'), 'the new upstream rule is merged in');
  observe('I195-MERGE-DETERMINISM', ['incrementalRemerge'], 'an upstream change re-merges only its group');

  // Without the previous result the renamed upstream rule would found a new
  // canonical name; with it, the established equivalence keeps its identity.
  const fresh = merge(changedSources());
  assert.deepEqual(group(fresh, expected.recomputed[0]).identities, expected.identitiesWithoutPrevious);
  assert.equal(changed.identities['upstream-a:numeral'], previous.groups[0].identities['upstream-a:number']);
  assert.equal(changed.grammar.rule('numeral'), undefined);
  for (const [alias, name] of Object.entries(previous.groups[0].identities)) {
    if (alias in changed.identities) assert.equal(changed.identities[alias], name, alias);
  }
  for (const entry of remerged.groups) {
    const classes = new Map();
    for (const decision of entry.decisions.filter(({ kind }) => kind === 'merged' || kind === 'kept-unique')) {
      assert.equal(classes.has(decision.name), false, `${decision.name} names one class`);
      classes.set(decision.name, decision.members);
      for (const alias of decision.members) assert.equal(entry.identities[alias], decision.name, alias);
    }
    assert.deepEqual([...classes.keys()].sort(), entry.grammar.ruleNames().sort());
  }
  observe('I195-MERGE-DETERMINISM', ['noDuplicateCanonicalIdentities'], 'established equivalences keep one canonical identity');
});

test('language editions are merged separately and listed as explicit alternatives', () => {
  const result = merge();
  const editions = result.groups.filter(({ language }) => language === 'arithmetic');
  assert.deepEqual(editions.map(({ edition }) => edition), ['2024', '2025']);
  const older = group(result, 'arithmetic@2024');
  const newer = group(result, 'arithmetic@2025');
  assert.ok(Object.keys(older.identities).every((alias) => !alias.startsWith('upstream-c:')));
  assert.deepEqual(Object.keys(newer.identities), ['upstream-c:expression', 'upstream-c:term', 'upstream-c:number']);
  assert.notDeepEqual(newer.grammar.rule('number').expression, older.grammar.rule('number').expression);
  assert.deepEqual(result.alternatives, [{ reason: 'edition', name: 'arithmetic', options: ['2024', '2025'] }]);
  observe('I195-MERGE-DETERMINISM', ['editionBoundariesPreserved'], 'language editions are merged separately');
});

test('differently named equivalent rules are merged, recursively and after normalization', () => {
  const older = group(merge(), 'arithmetic@2024');
  assert.deepEqual(
    decisions(older, 'merged').map(({ name, members }) => [name, members]),
    [
      ['expression', ['upstream-a:expression', 'upstream-b:sum']],
      ['term', ['upstream-a:term', 'upstream-b:operand']],
      ['number', ['upstream-a:number', 'upstream-b:integer']],
    ],
  );
  // `ASCII_DIGIT ~ ASCII_DIGIT*` and `ASCII_DIGIT+` mean the same, and the
  // mutually recursive pair sum/operand matches expression/term.
  const [a, b] = sources();
  assert.equal(normalizedRuleDefinition(a.grammar.rule('number')), normalizedRuleDefinition(b.grammar.rule('integer')));
  assert.notEqual(normalizedRuleDefinition(a.grammar.rule('expression')), normalizedRuleDefinition(b.grammar.rule('sum')));
  assert.equal(older.grammar.rule('sum'), undefined);
  assert.equal(older.grammar.rule('operand'), undefined);
  assert.equal(older.grammar.rule('integer'), undefined);
  observe('I195-MERGE-MEANING-AWARE-DEDUPLICATION', ['equivalentConceptsMerged'], 'differently named equivalent rules are merged');

  for (const decision of decisions(older, 'merged')) {
    assert.equal(decision.basis, GRAMMAR_MERGE_METHOD);
    assert.match(decision.definition, /^(normal|atomic):/u);
    assert.equal(decision.definition, normalizedRuleDefinition(older.grammar.rule(decision.name)));
  }
  observe('I195-MERGE-MEANING-AWARE-DEDUPLICATION', ['equivalenceJustified'], 'every merge records its proof method and definition');
});

test('rules with the same name and different meanings are kept apart', () => {
  const older = group(merge(), 'arithmetic@2024');
  assert.deepEqual(decisions(older, 'homonym-kept-distinct'), [{
    kind: 'homonym-kept-distinct',
    name: 'identifier',
    members: ['identifier', 'identifier_from_upstream_b'],
    basis: 'different-definitions',
    definition: null,
  }]);
  assert.equal(older.identities['upstream-a:identifier'], 'identifier');
  assert.equal(older.identities['upstream-b:identifier'], 'identifier_from_upstream_b');
  assert.notDeepEqual(older.grammar.rule('identifier').expression, older.grammar.rule('identifier_from_upstream_b').expression);
  assert.deepEqual(decisions(older, 'renamed-for-collision').map(({ members }) => members), [['upstream-b:identifier']]);
  observe('I195-MERGE-MEANING-AWARE-DEDUPLICATION', ['homonymsKeptDistinct'], 'rules with the same name and different meanings are kept apart');
});

test('similar names and identical samples nominate candidates but never decide a merge', () => {
  const older = group(merge(), 'arithmetic@2024');
  assert.deepEqual(older.nominations.map(({ basis, outcome }) => [basis, outcome]), [
    ['name-similarity', 'unproven'],
    ['identical-samples', 'proven'],
    ['identical-samples', 'unproven'],
  ]);
  // The proven nomination was merged by the structural proof, the unproven
  // ones stay separate rules.
  assert.equal(older.identities['upstream-a:number'], older.identities['upstream-b:integer']);
  assert.notEqual(older.identities['upstream-a:sign'], older.identities['upstream-b:operator']);
  const withoutSamples = group(merge(sources(), { samples: {} }), 'arithmetic@2024');
  assert.deepEqual(withoutSamples.identities, older.identities);
  assert.deepEqual(withoutSamples.grammar.normalized(), older.grammar.normalized());
  observe('I195-MERGE-MEANING-AWARE-DEDUPLICATION', ['samplesOnlyNominate'], 'samples only nominate candidates');
});

test('uncertain matches stay separate and every alternative is explicit', () => {
  const older = group(merge(), 'arithmetic@2024');
  assert.deepEqual(decisions(older, 'uncertain').map(({ basis, members }) => [basis, members]), [
    ['name-similarity', ['upstream-a:string_literal', 'upstream-b:stringLiteral']],
    ['identical-samples', ['upstream-a:sign', 'upstream-b:operator']],
  ]);
  for (const name of ['string_literal', 'stringLiteral', 'sign', 'operator']) assert.ok(older.grammar.rule(name), name);
  observe('I195-MERGE-UNCERTAINTY-PRESERVED', ['uncertainMatchesNotConflated'], 'uncertain matches stay separate');

  assert.deepEqual(older.alternatives.map(({ reason, options }) => [reason, options]), [
    ['start-rule', ['statement']],
    ['distinct-meaning', ['identifier', 'identifier_from_upstream_b']],
    ['uncertain-match', ['string_literal', 'stringLiteral']],
    ['uncertain-match', ['sign', 'operator']],
  ]);
  assert.equal(older.grammar.start, 'expression');
  assert.ok(older.grammar.rule('statement'), 'the alternative start rule is kept');
  assert.deepEqual(merge().alternatives, [{ reason: 'edition', name: 'arithmetic', options: ['2024', '2025'] }]);
  observe('I195-MERGE-UNCERTAINTY-PRESERVED', ['alternativesExplicit'], 'every alternative is explicit');
});

test('an unresolved required equivalence fails the merge', () => {
  const result = merge(sources(), { requiredEquivalences: fixture.requiredEquivalences });
  assert.equal(result.status, 'incomplete');
  assert.deepEqual(result.failures, fixture.expected.failures);
  assert.throws(() => assertMergeComplete(result), (error) => {
    assert.ok(error instanceof GrammarMergeError);
    assert.deepEqual(error.failures, result.failures);
    return true;
  });
  const proven = merge(sources(), { requiredEquivalences: [['upstream-b:integer', 'upstream-a:number']] });
  assert.equal(proven.status, 'complete');
  assert.equal(assertMergeComplete(proven), proven);
  const unknown = merge(sources(), { requiredEquivalences: [['upstream-a:number', 'upstream-b:missing']] });
  assert.deepEqual(unknown.failures.map(({ reason }) => reason), ['unknown-rule']);
  observe('I195-MERGE-UNCERTAINTY-PRESERVED', ['unresolvedEquivalenceFails'], 'an unresolved required equivalence fails the merge');
});

test('a merged unordered choice keeps the source order of its alternatives', () => {
  // The comparison form of an unordered choice is order-free, but the merged
  // grammar is what parsers run: a PEG parser (an exported Peggy grammar, or
  // the native executor under `matching peg`) commits to the first
  // alternative that matches, so a sorted `letter | letter word` would stop
  // after one letter.
  const grammar = importBnf('<word> ::= <letter> <word> | <letter>\n<letter> ::= "b" | "a"\n');
  const [merged] = mergeGrammars([{ id: 'words', language: 'words', precedence: 0, grammar }]).groups;
  assert.deepEqual(merged.grammar.rule('word').expression, grammar.rule('word').expression);
  assert.deepEqual(merged.grammar.rule('letter').expression, grammar.rule('letter').expression);
  parseWithGrammar(merged.grammar, 'abba');
  const reversed = importBnf('<word> ::= <letter> | <letter> <word>\n<letter> ::= "a" | "b"\n');
  assert.equal(normalizedRuleDefinition(reversed.rule('word')), normalizedRuleDefinition(grammar.rule('word')));
});

const renameCase = fixture.rename;
const renameSteps = (grammar, steps, aliases = []) => steps.reduce(
  (state, { from, to }) => renameGrammarRule(state.grammar, from, to, { namespace: renameCase.namespace, aliases: state.aliases }),
  { grammar, aliases },
);

test('renaming follows recursive references, qualified names and captures', () => {
  const original = deserializeGrammar(renameCase.grammar);
  const renamed = renameSteps(original, renameCase.steps);
  const { rules } = renamed.grammar.normalized();
  const byName = new Map(rules.map((rule) => [rule.name, rule.expression]));
  assert.equal(renamed.grammar.start, 'sum');
  assert.deepEqual(byName.get('sum').items[1].item.items[1], { kind: 'ref', name: 'sum' });
  assert.deepEqual(byName.get('operand').items[1].items[1], { kind: 'ref', name: 'sum' });
  assert.deepEqual(byName.get('spaced').items[1], { kind: 'ref', name: 'operand' });
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['recursiveRulesRenamed'], 'renaming follows recursive references');

  assert.deepEqual(byName.get('qualified').items, [
    { kind: 'ref', name: 'arithmetic.operand' },
    { kind: 'ref', name: 'arithmetic::operand' },
    { kind: 'ref', name: 'library.value' },
  ]);
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['qualifiedReferencesRenamed'], 'renaming follows qualified references');

  // The capture label `value` names a local binding, not the rule: it stays,
  // while the reference it wraps follows the rule.
  assert.deepEqual(byName.get('sum').items[0], { kind: 'capture', label: 'value', item: { kind: 'ref', name: 'operand' } });
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['shadowingHandled'], 'capture labels shadowing a rule are not renamed');
});

test('renaming refuses collisions and unknown rules', () => {
  const original = deserializeGrammar(renameCase.grammar);
  const kinds = renameCase.collisions.map(({ from, to }) => {
    try {
      renameGrammarRule(original, from, to, { namespace: renameCase.namespace });
    } catch (error) {
      assert.ok(error instanceof GrammarRenameError);
      return error.kind;
    }
    return 'accepted';
  });
  assert.deepEqual(kinds, renameCase.expected.collisionKinds);
  assert.deepEqual(original.normalized(), renameCase.grammar, 'a refused rename leaves the grammar unchanged');
  assert.throws(
    () => restoreSourceNames(original, [{ canonical: 'value', original: 'number' }]),
    (error) => error instanceof GrammarRenameError && error.kind === 'collision',
  );
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['collisionsHandled'], 'renaming refuses collisions');
});

test('renaming works on a reloaded grammar and keeps source aliases for export', () => {
  const original = deserializeGrammar(renameCase.grammar);
  const renamed = renameSteps(original, renameCase.steps);
  const reloaded = deserializeGrammar(serializeGrammar(renamed.grammar));
  assert.deepEqual(reloaded.normalized(), renamed.grammar.normalized());
  const final = renameSteps(reloaded, [renameCase.afterReload], renamed.aliases);
  assert.deepEqual(final.grammar.normalized(), renameCase.expected.grammar);
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['referencesAfterReloadRenamed'], 'renaming works on a reloaded grammar');

  assert.deepEqual(final.aliases, renameCase.expected.aliases);
  const exported = restoreSourceNames(final.grammar, final.aliases, { namespace: renameCase.namespace });
  assert.deepEqual(exported.normalized(), renameCase.grammar);
  observe('I195-MERGE-BINDING-AWARE-RENAME', ['sourceAliasesKept'], 'source aliases restore the original names');
});

test('malformed merge input is rejected', () => {
  const [a] = sources();
  assert.throws(() => mergeGrammars([a, a]), GrammarMergeError);
  assert.throws(() => mergeGrammars([{ ...a, id: 'has:colon' }]), GrammarMergeError);
  assert.throws(() => mergeGrammars([{ ...a, language: '' }]), GrammarMergeError);
  assert.throws(() => mergeGrammars([{ ...a, grammar: renameCase.grammar }]), GrammarMergeError);
  assert.throws(() => mergeGrammars([a], { requiredEquivalences: [['upstream-a:number']] }), GrammarMergeError);
});
