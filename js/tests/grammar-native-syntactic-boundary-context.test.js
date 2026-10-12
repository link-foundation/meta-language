import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import test from 'node:test';
import { transformSyntacticBoundaryContext } from '../scripts/native-syntactic-boundary-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeName } from '../scripts/import-native-grammars.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const literal = value => ({ type: 'STRING', value });
const optional = name => ({ type: 'CHOICE', members: [symbol(name), { type: 'BLANK' }] });
const miniature = () => ({ name: 'syntactic_boundary', extras: [{ type: 'PATTERN', value: '[ \\t]' }], rules: {
  document: { type: 'SEQ', members: [symbol('body'), symbol('_separator'), literal('a'), literal('b'), literal('='), literal('z')] },
  body: { type: 'CHOICE', members: [symbol('_separator'), { type: 'SEQ', members: [optional('_separator'),
    { type: 'SEQ', members: [literal('x'), { type: 'REPEAT', content: { type: 'SEQ', members: [symbol('_separator'), literal('x')] } }] }, optional('_separator')] }] },
  _separator: literal(';'),
} });
const miniatureDecision = { rule: 'body', symbol: '_separator', occurrence: 4, count: 4,
  boundary: { type: 'SEQ', members: [symbol('_separator'), literal('a'), literal('b'), literal('=')] } };
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('syntactic boundary preferences leave separators outside the body and allow spaces in the boundary', () => {
  const grammar = miniature();
  const original = structuredClone(grammar.rules.body);
  transformSyntacticBoundaryContext(grammar, miniatureDecision);
  assert.deepEqual(grammar.rules.body.content, original);
  assert.deepEqual(grammar.rules.body.preferred.members[1].members.at(-1), { type: 'BLANK' });
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar))));
  for (const source of ['x;ab=z', 'x ; a b = z', 'x;x ; a b = z']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    assert.equal(outcome.tree.children.find(({ kind }) => kind === 'body').end, source.indexOf('; a') < 0 ? 1 : source.indexOf(' ; a'));
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), source);
  }
  const fallback = parser.parseTree('x ;', { startRule: 'body' });
  assert.equal(fallback.ok, true);
  assert.equal(leaves(fallback.tree).some(({ text }) => text === ';'), true);
});

test('syntactic boundary contexts reject changed counts and nonoptional occurrences without mutation', () => {
  for (const change of [{ rule: 'missing' }, { symbol: 'missing' }, { count: 3 }, { occurrence: 1 }, { occurrence: 5 }, { occurrence: 0 }, { boundary: symbol('missing') }, { boundary: null }]) {
    const grammar = miniature();
    const original = structuredClone(grammar);
    assert.throws(() => transformSyntacticBoundaryContext(grammar, { ...miniatureDecision, ...change }), TypeError);
    assert.deepEqual(grammar, original);
  }
});

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const naming = JSON.parse(read('parity/naming/grammar-name-expansions.json'));
const profile = naming.grammars.elixir;
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const entry = JSON.parse(read('parity/grammars/sources.json')).sources.find(({ language }) => language === 'elixir');
const upstream = JSON.parse(gunzipSync(readFileSync(new URL(`../../${entry.file}`, import.meta.url))));
const projection = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile.json'));
const candidate = transformNativeSource(upstream, profile.sourceTransforms.filter(({ family }) => family !== 'syntactic-boundary-context'));
const contextDecision = { rule: 'body', symbol: '_terminator', occurrence: 4, count: 4,
  boundary: { type: 'SEQ', members: [symbol('_terminator'), optional('_stab_clause_left'), literal('->')] } };
transformSyntacticBoundaryContext(candidate, contextDecision);
const imported = importTreeSitterNative(candidate, {
  nameOf: name => nativeName(name, words, profile), scanners: nativeScannerProfile(profile.scannerFamilies), immediate: entry.scanner.immediate,
});
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
const corpus = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile-corpus.json'));
const trailingCalls = corpus.cases.find(({ file, title }) => file === 'do_end.txt' && title === 'stab clause / edge cases / trailing call in multiline clause');

test('arrow clause bodies retain their exact spans before further clauses and closing keywords', () => {
  for (const source of [
    'fun do\n 1 ->\n  1\n  x\n\n 1 ->\n  1\nend',
    'fn a ->x\n\n b ->y end',
    'fn a ->x\n\n\n b ->y\n\nend',
    'fun do\n 1 ->\n  x\n\n  x 1 ->\n  y\nend',
    trailingCalls.source,
  ]) {
    const oracle = oracleSnapshot(source, 'elixir');
    assert.equal(oracle.recovers, false, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), oracle.rows, source);
    assert.equal(leaves(actual.tree).map(({ text }) => text).join(''), source);
  }
});
