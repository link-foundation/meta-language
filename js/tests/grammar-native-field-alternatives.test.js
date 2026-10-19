import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { importSource, nativeName, scannerText } from '../scripts/import-native-grammars.mjs';
import { transformOrderedFieldAlternatives } from '../scripts/native-field-alternatives.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = path => readProjectData(new URL(`../../${path}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/elixir-remote-property-order.json');

test('ordered field projections match aliases by source content and preserve every alternative', () => {
  const symbol = name => ({ type: 'SYMBOL', name });
  const literals = values => ({ type: 'CHOICE', members: values.map(value => ({ type: 'STRING', value })) });
  const members = [symbol('identifier'), { type: 'ALIAS', named: true, value: 'identifier', content: literals(['and', 'or']) },
    { type: 'ALIAS', named: true, value: 'text', content: symbol('quoted') }];
  const source = { rules: { source: { type: 'FIELD', name: 'right', content: { type: 'CHOICE', members } },
    identifier: { type: 'PATTERN', value: '[a-z]+' }, quoted: { type: 'STRING', value: 'quoted' } } };
  const decision = { rule: 'source', field: 'right', variants: [
    { alias: 'identifier', named: true, literals: ['and', 'or'] }, { symbol: 'identifier' },
    { alias: 'text', named: true, contentSymbol: 'quoted' },
  ], count: 1 };
  const transformed = structuredClone(source);
  transformOrderedFieldAlternatives(transformed, decision);
  assert.deepEqual(transformed.rules.source.content.members, [members[1], members[0], members[2]]);
  assert.equal(transformed.rules.source.content.type, 'NATIVE_ORDERED_CHOICE');
  assert.equal(source.rules.source.content.type, 'CHOICE');
  for (const change of [{ count: 2 }, { field: 'missing' }, { variants: decision.variants.slice(0, 2) },
    { variants: [decision.variants[0], decision.variants[0], decision.variants[2]] },
    { variants: [{ ...decision.variants[0], literals: ['and'] }, ...decision.variants.slice(1)] }]) {
    assert.throws(() => transformOrderedFieldAlternatives(structuredClone(source), { ...decision, ...change }), TypeError);
  }
});

test('Elixir reserved remote properties follow source aliases without ambiguity or loss', () => {
  const naming = read('parity/naming/grammar-name-expansions.json');
  const entry = read('parity/grammars/sources.json').sources.find(source => source.id === fixture.source);
  assert.ok(entry);
  const decisions = structuredClone(naming.grammars.elixir);
  decisions.sourceTransforms = decisions.sourceTransforms.filter(decision => decision.family !== fixture.decision.family);
  const expansions = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const graph = importSource(entry, expansions, decisions).grammar;
  transformOrderedFieldAlternatives(graph, fixture.decision);
  const imported = importTreeSitterNative(graph, { nameOf: name => nativeName(name, expansions, decisions),
    scanners: scannerText(entry), immediate: entry.scanner.immediate });
  assert.deepEqual(imported.report.unsupported, []);
  const listing = renderTreeSitterNative(imported);
  const parser = compileGrammar(parseGrammarLinks(listing));
  const projection = { ...read('parity/fixtures/native-grammars/elixir-profile.json'), oracleKinds: nativeOracleKinds(listing) };
  const upstream = read('parity/fixtures/native-grammars/elixir-profile-corpus.json').cases.find(test => test.file === fixture.corpus.file && test.title === fixture.corpus.title);
  assert.ok(upstream, 'the original failing CI case is still independently pinned');
  const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
  for (const source of [upstream.source, ...fixture.matches]) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, projection), oracle.rows, source);
    assert.equal(text(native.tree), source);
  }
});
