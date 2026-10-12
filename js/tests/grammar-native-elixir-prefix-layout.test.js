import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import test from 'node:test';
import { transformSymbolChoiceContext } from '../scripts/native-symbol-choice-context.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeName } from '../scripts/import-native-grammars.mjs';
import { nativeScannerProfile } from '../scripts/generate-native-scanner-profiles.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const symbol = name => ({ type: 'SYMBOL', name });
const miniature = () => ({
  name: 'initial_layout', externals: [symbol('_layout')],
  extras: [{ type: 'PATTERN', value: '[ \\t]' }, symbol('_layout')],
  rules: {
    document: { type: 'SEQ', members: [{ type: 'CHOICE', members: [symbol('_separator'), { type: 'BLANK' }] }, { type: 'STRING', value: 'x' }] },
    _separator: { type: 'STRING', value: ';' },
  },
});
const miniatureDecision = { rule: 'document', symbol: '_separator', occurrence: 1, count: 1, helper: '_initial_separator', alternatives: ['_layout'] };
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);

test('initial symbol choices retain scanner-owned layout spans and separate skipped prefixes', () => {
  const grammar = miniature();
  const original = structuredClone(grammar.rules._separator);
  transformSymbolChoiceContext(grammar, miniatureDecision);
  assert.deepEqual(grammar.rules._separator, original);
  assert.deepEqual(grammar.rules._initial_separator, { type: 'CHOICE', members: [symbol('_separator'), symbol('_layout')] });
  const scanners = '(scanner layout (tokens _layout) (operations (while (next (literal %20)) (do (skip (literal %20)))) (consume (literal %0A)) (emit _layout)))\n';
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(grammar, { scanners, immediate: ['_layout'] }))));
  for (const source of ['\nx', ' \nx']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    const tokens = leaves(outcome.tree);
    const layout = tokens.find(({ text }) => text === '\n');
    assert.equal(layout.kind, 'unnamed_token');
    assert.equal(layout.start, source.indexOf('\n'));
    assert.equal(layout.end, source.indexOf('\n') + 1);
    assert.equal(tokens.map(({ text }) => text).join(''), source);
  }
});

test('symbol choice contexts validate source identities and exact occurrences before mutation', () => {
  for (const change of [{ rule: 'missing' }, { symbol: 'missing' }, { occurrence: 0 }, { count: 2 }, { occurrence: 2 }, { helper: 'visible' }, { helper: '_separator' }, { alternatives: ['missing'] }, { alternatives: ['_layout', '_layout'] }, { alternatives: ['_separator'] }]) {
    const grammar = miniature();
    const original = structuredClone(grammar);
    assert.throws(() => transformSymbolChoiceContext(grammar, { ...miniatureDecision, ...change }), TypeError);
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
// The generic scanner boundary already reserves newline continuations. A
// blanket slash guard would also reject valid local and remote operator arity.
const decisions = profile.sourceTransforms.filter(decision => decision.family !== 'symbol-choice-context'
  && !(decision.family === 'literal-continuation-exclusion' && decision.literal === '/' && decision.rules.includes('operator_identifier')));
const candidate = transformNativeSource(upstream, decisions);
const contextDecision = {
  rule: 'source', symbol: '_terminator', occurrence: 1, count: 3, helper: '_initial_separator',
  alternatives: ['_newline_before_binary_operator', '_newline_before_comment'],
};
transformSymbolChoiceContext(candidate, contextDecision);
const imported = importTreeSitterNative(candidate, {
  nameOf: name => nativeName(name, words, profile), scanners: nativeScannerProfile(profile.scannerFamilies), immediate: entry.scanner.immediate,
});
const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
const corpus = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile-corpus.json'));
const fixtures = [
  corpus.cases.find(({ file, title }) => file === 'expression/capture.txt' && title === 'local operator MFA'),
  corpus.cases.find(({ file, title }) => file === 'expression/operator.txt' && title === 'nullary range'),
];

test('operator captures and initial range layout retain every oracle row and source byte', () => {
  for (const source of [
    ...fixtures.map(fixture => fixture.source), '&//2', '&Kernel.//2', '1..2\n// 4\n',
    '\n..', ' \n ..', '\nnot ..', '\nEnum.to_list(..)', '\n# c\n', ' \n # c\n', '\n1', '\nfun do\n x\nend',
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
