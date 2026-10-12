import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { corpusCases, importSource } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformFieldExpressionContext } from '../scripts/native-field-expression-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const alias = (value, name) => ({ type: 'ALIAS', named: true, value, content: symbol(name) });

test('ordered source alternatives test their following boundary after preserving each alias', () => {
  const grammar = { name: 'following', rules: {
    document: symbol('left'), left: { type: 'CHOICE', members: [alias('arguments', 'argument'), alias('guard', 'condition')] },
    argument: { type: 'STRING', value: 'a' }, condition: { type: 'STRING', value: 'a when b' },
  } };
  const decision = { family: 'ordered-variants', rule: 'left', variants: ['alias:guard:condition', 'alias:arguments:argument'], followingPatterns: { 'alias:guard:condition': '->' } };
  const result = transformNativeSource(grammar, [decision]);
  assert.deepEqual(result.rules.left.members[0].members, [
    { type: 'SEQ', members: [alias('guard', 'condition'), { type: 'NATIVE_PATTERN_LOOKAHEAD', content: { type: 'BLANK' }, pattern: '->' }] },
    alias('arguments', 'argument'),
  ]);
  for (const followingPatterns of [{ missing: '->' }, { 'alias:guard:condition': '' }]) {
    assert.throws(() => transformNativeSource(grammar, [{ ...decision, followingPatterns }]), TypeError);
  }
  assert.deepEqual(grammar.rules.left.members, [alias('arguments', 'argument'), alias('guard', 'condition')]);
});

test('field expression projections retain aliases surrounding a selected source reference', () => {
  const grammar = { name: 'aliased_field', rules: {
    document: symbol('predicate'), argument: { type: 'STRING', value: 'a' },
    predicate: { type: 'FIELD', name: 'left', content: alias('arguments', 'argument') },
  } };
  transformFieldExpressionContext(grammar, {
    clones: [{ rule: 'argument', helper: '_restricted_argument' }],
    fields: [{ rule: 'predicate', field: 'left', symbol: 'argument', helper: '_restricted_argument', count: 1 }],
  });
  assert.deepEqual(grammar.rules.predicate, { type: 'FIELD', name: 'left', content: alias('arguments', '_restricted_argument') });
  assert.deepEqual(grammar.rules.argument, { type: 'STRING', value: 'a' });
});

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const fixture = JSON.parse(read('parity/fixtures/native-grammars/elixir-guard-precedence-context.json'));
const naming = JSON.parse(read('parity/naming/grammar-name-expansions.json'));
const entry = JSON.parse(read('parity/grammars/sources.json')).sources.find(source => source.language === 'elixir');
const profile = structuredClone(naming.grammars.elixir);
profile.sourceTransforms = profile.sourceTransforms.filter(decision => !(decision.family === 'field-expression-context' && decision.clones.some(clone => clone.helper === '_guard_argument_list'))
  && !(decision.family === 'literal-continuation-exclusion' && decision.rules.includes('_guard_binary_expression')));
const ordering = profile.sourceTransforms.find(decision => decision.family === 'ordered-variants' && decision.rule === fixture.orderedRule);
ordering.followingPatterns = fixture.followingPatterns;
profile.sourceTransforms.push(fixture.context, fixture.boundary);
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), profile);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...JSON.parse(read('parity/fixtures/native-grammars/elixir-profile-sources.json')), oracleKinds: nativeOracleKinds(imported.text) };
const originalCases = corpusCases(entry).filter(item => fixture.corpus.some(({ file, title }) => item.file === file && item.title === title));
const text = node => node.type === 'node' ? node.children.map(text).join('') : node.text;

test('guard contexts preserve operator precedence and complete parenthesized arrow clauses', () => {
  assert.equal(originalCases.length, 2);
  for (const source of [...fixture.matches, ...originalCases.map(item => item.source)]) {
    const oracle = oracleSnapshot(source, 'Elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(text(actual.tree), source);
  }
});
