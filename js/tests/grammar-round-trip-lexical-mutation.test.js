import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar, Grammar, GrammarBuilder as G } from '../src/grammar.js';
import { emitAntlr } from '../src/grammar-emitters/antlr.js';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { mutateGrammarStartRule, GRAMMAR_ROUND_TRIP_MARKER, renderGrammarLexicalOwnerComparison } from '../src/grammar-round-trip.js';
import { renameGrammarRule } from '../src/grammar-rename.js';

test('ANTLR round-trip mutation inserts its literal before explicit rules and renumbers later implicit tokens', () => {
  const original = importAntlr(`grammar Start; start: A ' ' A EOF; later: 'later'; A: [a-z]+;`);
  const before = renderGrammarLinks(original);
  const mutated = mutateGrammarStartRule(original);
  const entries = mutated.declarations.lexers[0].expression.items;
  assert.deepEqual(entries.filter(({ kind }) => kind === 'lexicalLiteral').map(({ value, name }) => [value, name]), [
    [' ', '@literal:0'], [GRAMMAR_ROUND_TRIP_MARKER, '@literal:1'], ['later', '@literal:2'],
  ]);
  assert.deepEqual(entries.filter(({ kind }) => kind === 'lexicalRule').map(({ name }) => name), ['@literal:0', '@literal:1', '@literal:2', 'A']);
  const emitted = emitAntlr(mutated);
  assert.deepEqual(emitted.report.lossy, []);
  const reimported = importAntlr(emitted.source);
  assert.equal(renderGrammarLinks(reimported), renderGrammarLinks(mutated));
  for (const grammar of [mutated, parseGrammarLinks(renderGrammarLinks(mutated)), reimported]) {
    const parser = compileGrammar(grammar);
    assert.equal(parser.parseTree(GRAMMAR_ROUND_TRIP_MARKER).ok, true);
    assert.equal(parser.parseTree('a a').ok, true);
    assert.equal(parser.parseTree('a  a').ok, false);
    assert.equal(parser.parseTree('a').ok, false);
  }
  assert.equal(renderGrammarLinks(original), before);
});

test('ANTLR round-trip mutation reuses existing explicit or implicit identities without duplicating tokens', () => {
  for (const [source, marker] of [
    [`grammar Start; start: B EOF; A: 'a'; B: 'b';`, 'a'],
    [`grammar Start; start: B ' ' B EOF; A: 'a'; B: 'b';`, ' '],
  ]) {
    const original = importAntlr(source);
    const mutated = mutateGrammarStartRule(original, marker);
    assert.equal(renderGrammarLinks(importAntlr(emitAntlr(mutated).source)), renderGrammarLinks(mutated));
    const before = original.declarations.lexers[0].expression.items.filter(({ kind }) => kind === 'lexicalRule');
    const after = mutated.declarations.lexers[0].expression.items.filter(({ kind }) => kind === 'lexicalRule');
    assert.deepEqual(after, before);
    assert.equal(compileGrammar(mutated).parseTree(marker).ok, true);
    const again = mutateGrammarStartRule(mutated, marker);
    assert.deepEqual(again.declarations, mutated.declarations);
  }
});

test('ANTLR ambiguous marker aliases remain unresolved and native grammar declarations stay intact', () => {
  const original = importAntlr(`grammar Start; start: C EOF; A: 'a'; B: 'a'; C: 'c';`);
  const mutated = mutateGrammarStartRule(original, 'a');
  assert.equal(renderGrammarLinks(importAntlr(emitAntlr(mutated).source)), renderGrammarLinks(mutated));
  assert.throws(() => compileGrammar(mutated), /unresolved literal/u);
  const native = new Grammar('start', new Map([['start', { name: 'start', kind: 'normal', expression: G.literal('old') }]]), null,
    { modes: ['quoted'], extras: [G.literal(' ')] });
  const changed = mutateGrammarStartRule(native);
  assert.deepEqual(changed.declarations, native.declarations);
  assert.equal(compileGrammar(changed).parseTree(GRAMMAR_ROUND_TRIP_MARKER).ok, true);
  assert.equal(compileGrammar(changed).parseTree('old').ok, true);
});

test('ANTLR owner comparison proves a public-binding bijection while preserving actual original aliases', () => {
  const source = `grammar Start; start: word '!' EOF; word: A {effect();}; A: 'a';`;
  const original = importAntlr(source);
  const renamed = renameGrammarRule(original, 'word', 'term').grammar;
  const reimported = importAntlr(source.replaceAll('word', 'term'));
  const actual = renderGrammarLinks(renamed);
  assert.match(actual, /\(lexicalOwner word \(ref term\)\)/u);
  assert.match(actual, /\(lexicalObligation word action effect/u);
  assert.notEqual(actual, renderGrammarLinks(reimported));
  const expected = renderGrammarLexicalOwnerComparison(renamed);
  assert.equal(expected, renderGrammarLexicalOwnerComparison(reimported));
  assert.equal(renderGrammarLinks(renamed), actual);
  const change = (mutate) => {
    const copy = parseGrammarLinks(actual);
    mutate(copy.declarations.lexers[0].expression.items);
    return copy;
  };
  for (const mutate of [
    (entries) => { entries.find(({ kind }) => kind === 'lexicalLiteral').value = '?'; },
    (entries) => { entries.find(({ kind }) => kind === 'lexicalRule').type = 'DIFFERENT'; },
    (entries) => { entries.find(({ kind }) => kind === 'lexicalRule').action.push({ operation: 'pushMode', mode: 'default' }); },
    (entries) => { entries.find(({ kind }) => kind === 'lexicalObligation').source = 'different();'; },
  ]) assert.notEqual(renderGrammarLexicalOwnerComparison(change(mutate)), expected);
  for (const mutate of [
    (entries) => { entries.push(structuredClone(entries.find(({ kind }) => kind === 'lexicalOwner'))); },
    (entries) => { entries.push({ kind: 'lexicalOwner', name: 'other', item: G.ref('term') }); },
  ]) assert.throws(() => renderGrammarLexicalOwnerComparison(change(mutate)), /nonbijective original parser owners/u);
});
