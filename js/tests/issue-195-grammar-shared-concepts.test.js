// Shared concepts of the native grammars: every rule of every native grammar
// names a concept record, a construct that corresponds one to one across
// native grammars resolves to one concept identity, lookalike constructs stay
// distinct (I195-GRAMMAR-SHARED-CONCEPTS); the per-language reuse report is
// current (I195-GRAMMAR-CONCEPT-REUSE-REPORT); a shared construct translates
// between two native grammars through its concept with no rule for the pair
// of languages (I195-GRAMMAR-CONCEPT-TRANSLATION); and native rule names are
// readable English with their tree-sitter names kept as aliases
// (I195-NAMING-NATIVE-GRAMMARS). The Rust twin is
// rust/tests/unit/issue_195_grammar_shared_concepts.rs.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  CONCEPT_RECORDS,
  LANGUAGE_CATALOG,
  checkConceptDistinctions,
  checkNativeGrammarConcepts,
  conceptCorrespondence,
  nativeConstructTree,
  nativeGrammarConceptReuse,
  nativeGrammarIds,
  nativeGrammarLanguage,
  nativeGrammarRuleConcepts,
  nativeGrammarSource,
  parseGrammarLinks,
  renderGrammarLinks,
  translateNativeConstruct,
  translateNativeConstructTree,
} from '../src/index.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { CONCEPT_REUSE_DOCUMENT, CONCEPT_REUSE_FIXTURE, formatConceptReuse, renderConceptReuseDocument } from '../scripts/build-native-grammar-concept-reuse.mjs';
import { extractNameInventory } from '../scripts/issue-195-naming.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const FIXTURE = 'parity/fixtures/grammar-shared-concepts.json';
const NAMING_FIXTURE = 'parity/naming/canonical-concepts.json';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const fixture = JSON.parse(readFileSync(path.join(root, FIXTURE), 'utf8'));

function observe(requirementId, fixtureFile, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile,
    assertions,
    testName,
  });
}

const alias = ([grammar, name]) => ({ source: nativeGrammarSource(grammar), name });
const conceptOf = (grammar, rule) => nativeGrammarRuleConcepts(grammar).find((entry) => entry.rule === rule)?.concept;
const shape = (tree) => [tree.kind, ...tree.children.map(shape)];

test('every rule of every native grammar names a concept record whose native aliases are exactly its rules', () => {
  assert.deepEqual(nativeGrammarIds(), Object.keys(LANGUAGE_CATALOG.nativeGrammars).sort());
  assert.deepEqual(checkNativeGrammarConcepts(), []);
  let rules = 0;
  for (const grammar of nativeGrammarIds()) {
    for (const { rule, concept } of nativeGrammarRuleConcepts(grammar)) {
      rules += 1;
      assert.ok(CONCEPT_RECORDS.some((record) => record.id === concept), `${grammar} ${rule} names ${concept}`);
    }
  }
  assert.ok(rules >= 127, `${rules} native rules`);

  // A rule without a concept, a concept without a record, a stale alias and a
  // concept from another language's namespace each fail the check.
  const json = nativeGrammarRuleConcepts('native-json');
  const broken = (change) => checkNativeGrammarConcepts({
    grammars: ['native-json', 'native-diff'],
    ruleConcepts: (grammar) => (grammar === 'native-json' ? change(structuredClone(json)) : nativeGrammarRuleConcepts(grammar)),
  }).map(({ kind, rule }) => `${kind} ${rule}`);
  assert.deepEqual(broken((rules) => rules.map((entry) => (entry.rule === 'pair' ? { ...entry, concept: null } : entry))),
    ['rule-without-concept pair', 'alias-without-rule pair']);
  assert.deepEqual(broken((rules) => rules.map((entry) => (entry.rule === 'pair' ? { ...entry, concept: 'grammar.pair-of-things' } : entry))),
    ['concept-without-record pair', 'alias-without-rule pair']);
  assert.deepEqual(broken((rules) => [...rules, { rule: 'extra', concept: 'grammar.diff.hunk' }]),
    ['rule-without-alias extra', 'namespace-mismatch extra']);
  observe('I195-GRAMMAR-SHARED-CONCEPTS', FIXTURE, ['everyRuleHasConceptRecord', 'recordAliasesMatchRules'], 'every native rule names a concept record');
});

test('rules are shared when two or more native grammars name their concept and language-specific otherwise', () => {
  const report = nativeGrammarConceptReuse();
  const languages = new Set(nativeGrammarIds().map(nativeGrammarLanguage));
  for (const { grammar, language, rules, shared, specific } of report.grammars) {
    assert.equal(rules, nativeGrammarRuleConcepts(grammar).length);
    assert.equal(shared.length + specific.length, rules);
    for (const entry of shared) {
      assert.ok(entry.languages.length > 1 && entry.languages.includes(language), `${grammar} ${entry.rule}`);
      // A shared concept is in no language's namespace.
      assert.ok(!languages.has(entry.concept.split('.')[1]) || entry.concept.split('.').length < 3, entry.concept);
    }
    for (const entry of specific) {
      const named = report.concepts.find(({ concept }) => concept === entry.concept);
      assert.deepEqual(named.languages, [language], `${grammar} ${entry.rule}`);
    }
  }
  for (const record of CONCEPT_RECORDS) {
    const named = report.concepts.find(({ concept }) => concept === record.id);
    if (!named) continue;
    const constraint = named.shared
      ? 'Native grammars share this concept only where the construct means the same in each language.'
      : `Only the native ${named.languages[0]}`;
    assert.ok(record.constraints.some((text) => text.toLowerCase().startsWith(constraint.toLowerCase())), `${record.id} states how it is used`);
  }
  observe('I195-GRAMMAR-SHARED-CONCEPTS', FIXTURE, ['languageSpecificConceptsStayInTheirLanguage'], 'shared and language-specific rules');
});

test('every construct that corresponds one to one across native grammars resolves to one concept identity', () => {
  for (const { construct, concept, rules } of fixture.sharedConstructs) {
    for (const rule of rules) {
      assert.equal(conceptOf(...rule), concept, `${construct}: ${rule.join(' ')}`);
      const records = CONCEPT_RECORDS.filter(({ sourceAliases }) =>
        sourceAliases.some(({ source, name }) => source === alias(rule).source && name === alias(rule).name));
      assert.deepEqual(records.map(({ id }) => id), [concept], `${construct}: ${rule.join(' ')}`);
    }
    for (const first of rules) {
      for (const second of rules) {
        const relation = conceptCorrespondence(alias(first), alias(second));
        assert.equal(relation.relation, 'shared', `${construct}: ${first.join(' ')} / ${second.join(' ')}`);
        assert.equal(relation.shared, concept);
      }
    }
  }
  const wide = fixture.sharedConstructs.filter(({ rules }) => new Set(rules.map(([grammar]) => grammar)).size >= 3);
  assert.ok(wide.length >= 5, 'five constructs correspond in three or more native grammars');
  // Every concept three or more native grammars name is listed.
  const listed = new Set(fixture.sharedConstructs.map(({ concept }) => concept));
  for (const { concept, languages } of nativeGrammarConceptReuse().concepts) {
    if (languages.length >= 3) assert.ok(listed.has(concept), `${concept} is listed`);
  }
  observe('I195-GRAMMAR-SHARED-CONCEPTS', FIXTURE, ['sharedConstructsResolveToOneIdentity'], 'shared constructs resolve to one identity');
});

test('lookalike constructs of different native grammars stay distinct concepts', () => {
  for (const { first, second, concepts } of fixture.lookalikes) {
    const label = `${first.join(' ')} / ${second.join(' ')}`;
    assert.deepEqual([conceptOf(...first), conceptOf(...second)], concepts, label);
    const relation = conceptCorrespondence(alias(first), alias(second));
    assert.equal(relation.relation, 'distinct', label);
    assert.ok(relation.justification?.endsWith('.'), `${label} carries the reason they stay distinct`);
    assert.equal(translateNativeConstruct(first[0], first[1], second[0]).rules.includes(second[1]), false, label);
  }
  // Merging a lookalike pair into one record breaks a required distinction.
  const merged = structuredClone(CONCEPT_RECORDS);
  const linked = merged.find(({ id }) => id === 'grammar.linked-list');
  merged.find(({ id }) => id === 'grammar.list').sourceAliases.push(...linked.sourceAliases);
  merged.splice(merged.indexOf(linked), 1);
  assert.deepEqual(checkConceptDistinctions(merged).map(({ kind }) => kind), ['unknown-concept']);
  // So does a grammar that names the lookalike's concept.
  const scheme = nativeGrammarRuleConcepts('native-scheme').map((entry) =>
    (entry.rule === 'list' ? { ...entry, concept: 'grammar.list' } : entry));
  assert.deepEqual(
    checkNativeGrammarConcepts({ grammars: ['native-scheme'], ruleConcepts: () => scheme }).map(({ kind, rule }) => `${kind} ${rule}`),
    ['rule-without-alias list', 'alias-without-rule list'],
  );
  observe('I195-GRAMMAR-SHARED-CONCEPTS', FIXTURE, ['lookalikeConstructsStayDistinct'], 'lookalike constructs stay distinct');
});

test('a shared construct translates between two native grammars through its concept, with no rule for the pair', () => {
  for (const { from, source, to, target, renamed } of fixture.translations) {
    const tree = nativeConstructTree(from, source);
    const { tree: translated, problems } = translateNativeConstructTree(tree, from, to);
    assert.deepEqual(problems, [], `${from} → ${to}`);
    // The translated tree is the tree the target grammar parses.
    assert.deepEqual(translated, nativeConstructTree(to, target), `${from} → ${to}`);
    for (const [kind, targetKind] of renamed) {
      const { relation, concept, rules } = translateNativeConstruct(from, kind, to);
      assert.deepEqual({ relation, rules }, { relation: 'translated', rules: [targetKind] });
      assert.equal(concept, conceptOf(to, targetKind));
    }
    // The translation reads the concept records alone: without the target's
    // source aliases, nothing translates.
    const targetSource = nativeGrammarSource(to);
    const records = structuredClone(CONCEPT_RECORDS).map((record) =>
      ({ ...record, sourceAliases: record.sourceAliases.filter((entry) => entry.source !== targetSource) }));
    const blind = translateNativeConstructTree(tree, from, to, { records });
    assert.equal(blind.tree, null);
    assert.ok(blind.problems.every(({ relation }) => relation === 'untranslatable'));
  }
  for (const { from, source, to, problems } of fixture.untranslatable) {
    const result = translateNativeConstructTree(nativeConstructTree(from, source), from, to);
    assert.equal(result.tree, null);
    assert.deepEqual(result.problems.map(({ kind, concept }) => [kind, concept]), problems);
    assert.ok(result.problems.every(({ relation }) => relation === 'untranslatable'));
  }
  observe('I195-GRAMMAR-CONCEPT-TRANSLATION', FIXTURE,
    ['constructsTranslateThroughConcepts', 'noPairwiseRuleUsed', 'untranslatableConstructsReported', 'translatedTreesMatchTargetParses'],
    'shared constructs translate through concepts');
});

test('the per-language concept reuse report is current and published', () => {
  const report = nativeGrammarConceptReuse();
  assert.equal(readFileSync(path.join(root, CONCEPT_REUSE_FIXTURE), 'utf8'), formatConceptReuse(report));
  const document = readFileSync(path.join(root, CONCEPT_REUSE_DOCUMENT), 'utf8');
  assert.equal(document, renderConceptReuseDocument(report));
  assert.match(document, /I195-MERGE-QUALITY-EVIDENCE/u);
  for (const { grammar, shared, specific } of report.grammars) {
    assert.match(document, new RegExp(`\\| \`${grammar}\` \\| ${shared.length + specific.length} \\| ${shared.length} \\| ${specific.length} \\|`, 'u'));
  }
  const index = readFileSync(path.join(root, 'docs/grammar/native-grammars.md'), 'utf8');
  assert.match(index, /native-grammar-concept-reuse\.md/u, 'the native grammar guide links the report');
  observe('I195-GRAMMAR-CONCEPT-REUSE-REPORT', CONCEPT_REUSE_FIXTURE,
    ['reuseReportCurrent', 'sharedAndSpecificRulesListed', 'reportPublishedWithMergeQualityEvidence', 'runtimesAgreeOnReport'],
    'the concept reuse report is current');
});

test('native rule names are readable English and keep their tree-sitter names as aliases', () => {
  const english = /^[a-z]+(?:_[a-z]+)*$/u;
  for (const grammar of nativeGrammarIds()) {
    const text = nativeGrammarText(grammar);
    const parsed = parseGrammarLinks(text);
    assert.equal(renderGrammarLinks(parsed), text, `${grammar} round-trips its concepts and source names`);
    const { oracleKinds } = LANGUAGE_CATALOG.nativeGrammars[grammar];
    const named = [...parsed.rules, ...parsed.kinds.map((kind) => [kind.name, kind])];
    for (const [name, { sourceNames }] of named) {
      assert.match(name, english, `${grammar} ${name}`);
      const original = sourceNames?.find(({ source }) => source === 'tree-sitter')?.name;
      assert.equal(oracleKinds[name], original, `${grammar} ${name} keeps its tree-sitter kind`);
    }
    assert.equal(Object.keys(oracleKinds).length, named.filter(([, { sourceNames }]) => sourceNames?.length > 0).length);
  }
  assert.deepEqual(LANGUAGE_CATALOG.nativeGrammars['native-json'].oracleKinds, { value: '_value' });
  assert.equal(LANGUAGE_CATALOG.nativeGrammars['native-racket'].oracleKinds.datum_comment, 'sexp_comment');
  // check:naming reads every native rule name and every concept reference.
  const inventory = extractNameInventory(root);
  const rules = inventory.filter(({ inventory: name }) => name === 'native grammar rule names');
  const references = inventory.filter(({ inventory: name }) => name === 'native grammar concept references');
  const ruleCount = nativeGrammarIds().reduce((sum, grammar) => sum + nativeGrammarRuleConcepts(grammar).length, 0);
  assert.equal(references.length, ruleCount);
  assert.ok(rules.length >= ruleCount);
  assert.deepEqual(new Set(references.map(({ file }) => file)).size, nativeGrammarIds().length);
  observe('I195-NAMING-NATIVE-GRAMMARS', NAMING_FIXTURE,
    ['nativeRuleNamesAreEnglish', 'sourceNamesKeptAsAliases', 'oracleKindsPreserved', 'namingCheckCoversNativeGrammars'],
    'native rule names are readable English');
});
