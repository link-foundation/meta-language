import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar, Grammar, GrammarBuilder as G } from '../src/grammar.js';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { parseNativeGrammar, renderNativeGrammar } from '../src/grammar-interchange.js';
import { renameGrammarRule } from '../src/grammar-rename.js';
import { mergeGrammars } from '../src/grammar-merge.js';
import { readLexicalWinnerIndex } from '../src/translation/frontend-rules.js';
import { selectAntlrReferenceAssertion } from '../scripts/antlr-version-assertions.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const parser = (source) => compileGrammar(importAntlr(source));
const leaves = (node) => node.type === 'token' ? [node] : (node.children ?? []).flatMap(leaves);
const token = (kind, text, start, end, rule, channel = 'default', trivia = false, lexer = 'Election') => ({
  type: 'token', kind, ...(trivia ? { trivia: true } : {}), start, end,
  lexical: { lexer, rule, type: rule, channel, disposition: 'emit' }, text,
});

test('ANTLR lexical election uses full match length and original order before channel filtering', () => {
  const grammar = `grammar Election; channels { ERROR }
    start: WORD '=' WORD EOF;
    WORD: [a-z]+; EQUAL: '='; WS: [ ]+ -> channel(HIDDEN);
    Unexpected: . -> channel(ERROR);`;
  const result = parser(grammar).parseTree('left = right');
  assert.equal(result.ok, true);
  assert.deepEqual(result.tree, { type: 'node', kind: 'start', start: 0, end: 12, children: [
    token('WORD', 'left', 0, 4, 'WORD'), token('WS', ' ', 4, 5, 'WS', 'HIDDEN', true),
    token('=', '=', 5, 6, 'EQUAL'), token('WS', ' ', 6, 7, 'WS', 'HIDDEN', true),
    token('WORD', 'right', 7, 12, 'WORD'),
  ] });
  assert.equal(parser(grammar).parseTree('left right').ok, false);
});

test('ANTLR a catchall on a custom channel remains a retained token rather than a name-based error', () => {
  const grammar = `grammar Election; channels { ERROR }
    start: WORD EOF; WORD: [a-z]+; Unexpected: . -> channel(ERROR);`;
  const result = parser(grammar).parseTree('word@');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree), [token('WORD', 'word', 0, 4, 'WORD'), token('Unexpected', '@', 4, 5, 'Unexpected', 'ERROR', true)]);
  assert.equal(parser(grammar).parseTree('word@tail').ok, false);
});

test('ANTLR lexical priority is independent of which parser branch could succeed', () => {
  const firstDefault = parser(`grammar Election; start: A EOF; A: 'x'; H: 'x' -> channel(HIDDEN);`);
  assert.equal(firstDefault.parseTree('x').ok, true);
  const firstHidden = parser(`grammar Election; start: A EOF; H: 'x' -> channel(HIDDEN); A: 'x';`);
  assert.equal(firstHidden.parseTree('x').ok, false);
  const longerHidden = parser(`grammar Election; start: A A EOF; A: 'x'; H: 'xx' -> channel(HIDDEN);`);
  assert.equal(longerHidden.parseTree('xx').ok, false);
});

test('ANTLR implicit literal tokens precede explicit rules and fragments never enter election', () => {
  const grammar = parser(`grammar Election; start: ID EOF; other: 'if';
    fragment LETTER: [a-z]; ID: LETTER+;`);
  assert.equal(grammar.parseTree('if').ok, false);
  assert.equal(grammar.parseTree('iffy').ok, true);
  assert.deepEqual(leaves(grammar.parseTree('a').tree).map(({ kind, text }) => [kind, text]), [['ID', 'a']]);
});

test('ANTLR winning mode commands apply once and retyped tokens keep source rule identity', () => {
  const source = `parser grammar Parser; options { tokenVocab = Election; }
    start: OPEN TEXT CLOSE EOF;
    lexer grammar Election; tokens { CLOSE }
    OPEN: '<' -> pushMode(STRING); OUTSIDE: [a-z]+;
    mode STRING;
    TEXT: [a-z]+; END: '>' -> type(CLOSE), popMode;`;
  const result = parser(source).parseTree('<abc>');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree).map(({ kind, text, lexical }) => [kind, text, lexical.rule, lexical.type]), [
    ['OPEN', '<', 'OPEN', 'OPEN'], ['TEXT', 'abc', 'TEXT', 'TEXT'], ['CLOSE', '>', 'END', 'CLOSE'],
  ]);
  assert.equal(parser(source).parseTree('abc').ok, false);
  assert.equal(parser(source).parseTree('<abc').ok, false);
});

test('ANTLR losing commands cannot change the winning lexer mode', () => {
  const source = `parser grammar Parser; options { tokenVocab = Election; } start: A A EOF;
    lexer grammar Election; A: 'x'; LOSER: 'x' -> pushMode(OTHER);
    mode OTHER; OTHER_A: 'y';`;
  assert.equal(parser(source).parseTree('xx').ok, true);
  assert.equal(parser(source).parseTree('xy').ok, false);
});

test('ANTLR warning-bearing skip, type and channel combinations preserve actual command ordering', (context) => {
  const versionApplicability = [selectAntlrReferenceAssertion()];
  const source = `grammar Election; tokens { WORD } start: WORD EOF;
    RAW: 'a' -> skip, type(WORD);
    DROP: 'x' -> channel(HIDDEN), skip, channel(DEFAULT_TOKEN_CHANNEL);
    WS: ' ' -> skip;`;
  const compiled = parser(source);
  const result = compiled.parseTree('x a ');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree).map(({ kind, text, lexical, trivia = false }) => [kind, text, lexical.rule, lexical.type, lexical.channel, lexical.disposition, trivia]), [
    ['DROP', 'x', 'DROP', 'SKIP', 'default', 'skip', true],
    ['WS', ' ', 'WS', 'SKIP', 'default', 'skip', true],
    ['WORD', 'a', 'RAW', 'WORD', 'default', 'emit', false],
    ['WS', ' ', 'WS', 'SKIP', 'default', 'skip', true],
  ]);
  assert.equal(compiled.parseTree('x').ok, false);
  assert.equal(parser(`grammar Election; tokens { WORD } start: WORD EOF; RAW: 'a' -> type(WORD), skip;`).parseTree('a').ok, false);
  recordIssue195Observations({
    requirementId: 'I195-ASSERTION-EXPLICIT-VERSION-RANGES', suffix: 'antlr-command-order', runtime: 'tooling',
    fixtureId: 'assertion-version-antlr-command-order-source', fixtureFile: 'js/tests/grammar-antlr-lexer-election.test.js',
    assertions: ['antlrCommandOrderHasApplicableVersionContext'], testName: context.name, versionApplicability,
  });
});

test('ANTLR final type commands add bindings while superseded historical aliases remain inspectable', () => {
  const source = `grammar Election; tokens { TEXT, WORD, UNUSED } start: WORD EOF;
    RAW: 'a' -> type(TEXT), type(WORD);`;
  const grammar = importAntlr(source);
  assert.deepEqual(grammar.rule('RAW').expression, G.literal('a'));
  assert.equal(grammar.rule('RAW').doc, '-> type(TEXT), type(WORD)');
  assert.deepEqual(grammar.rule('TEXT').expression, G.ref('RAW'));
  assert.equal(grammar.rule('TEXT').doc, 'also RAW, which -> type(TEXT) retypes');
  assert.deepEqual(grammar.rule('WORD').expression, G.ref('RAW'));
  assert.equal(grammar.ruleNames().includes('UNUSED'), false);
  const result = compileGrammar(grammar).parseTree('a');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree).map(({ kind, lexical }) => [kind, lexical.rule, lexical.type]), [['WORD', 'RAW', 'WORD']]);
  assert.equal(parser(source.replace('start: WORD', 'start: TEXT')).parseTree('a').ok, false);
  const hidden = parser(`grammar Election; tokens { WORD } start: WORD EOF;
    RAW: 'a' -> channel(HIDDEN), type(WORD);`);
  assert.equal(hidden.parseTree('a').ok, false);
});

test('ANTLR decimal channel zero normalizes operationally while original source attributes remain intact', () => {
  const grammar = importAntlr(`grammar Election; start: A EOF; A: 'a' -> channel(000); NOTE: 'b' -> channel(HIDDEN);`);
  assert.equal(grammar.rule('A').channel, undefined);
  assert.match(grammar.rule('A').doc, /-> channel\(000\)/u);
  assert.deepEqual(grammar.rule('A').expression, G.literal('a'));
  const compiled = compileGrammar(grammar);
  assert.equal(compiled.parseTree('a').ok, true);
  const result = compiled.parseTree('ab');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree).map(({ lexical }) => [lexical.rule, lexical.channel, lexical.disposition]), [
    ['A', 'default', 'emit'], ['NOTE', 'HIDDEN', 'emit'],
  ]);
  assert.equal(compiled.parseTree('b').ok, false);
});

test('ANTLR literal-delimited nongreedy comments stop at their first delimiter', () => {
  const result = parser(`grammar Election; start: A A EOF;
    A: 'a'; COMMENT: '/*' .*? '*/' -> channel(HIDDEN);`).parseTree('a/*one*/a/*two*/');
  assert.equal(result.ok, true);
  assert.deepEqual(leaves(result.tree).map(({ kind, text }) => [kind, text]), [
    ['A', 'a'], ['COMMENT', '/*one*/'], ['A', 'a'], ['COMMENT', '/*two*/'],
  ]);
});

test('ANTLR unresolved viable predicates and winning actions are explicit failures', () => {
  const predicate = parser(`grammar Election; start: A EOF; A: {guard()}? 'a'; B: 'b';`);
  assert.throws(() => predicate.parseTree('a'), /unresolved predicate/u);
  const action = parser(`grammar Election; start: A EOF; A: 'a' {effect();};`);
  assert.throws(() => action.parseTree('a'), /unresolved action/u);
  const losing = parser(`grammar Election; start: B EOF; B: 'b'; A: {guard()}? 'a';`);
  assert.equal(losing.parseTree('b').ok, true);
});

test('ANTLR default parser owners retain unresolved obligations without changing imported bodies', () => {
  const source = `grammar Election; start: first EOF; first: A {effect();}; second: A; A: 'a';`;
  const grammar = importAntlr(source);
  assert.deepEqual(grammar.rule('first').expression, G.ref('A'));
  assert.equal(grammar.rule('first').doc, 'dropped action');
  for (const candidate of [grammar, parseGrammarLinks(renderGrammarLinks(grammar)), parseNativeGrammar(renderNativeGrammar(grammar))]) {
    const compiled = compileGrammar(candidate);
    assert.throws(() => compiled.parseTree('a'), /unresolved action in Election:first: effect\(\);/u);
    assert.equal(compiled.parseTree('a', { startRule: 'second' }).ok, true);
  }
  assert.throws(() => parser(`grammar Election; start: {guard()}? A EOF; A: 'a';`).parseTree('a'), /unresolved predicate in Election:start/u);
});

test('ANTLR unsupported grammar options retain their original bytes as executable context obligations', () => {
  const options = ' superClass = FirstBase; /* retained π */ language = Java;\n';
  const grammar = importAntlr(`grammar Election; options {${options}} start: A EOF; A: 'a';`);
  const other = importAntlr(`grammar Election; options {${options.replace('FirstBase', 'SecondBase')}} start: A EOF; A: 'a';`);
  assert.deepEqual(grammar.rule('A').expression, other.rule('A').expression);
  assert.equal(grammar.rule('A').doc, other.rule('A').doc);
  assert.notDeepEqual(grammar.declarations.lexers, other.declarations.lexers);
  for (const candidate of [grammar, parseGrammarLinks(renderGrammarLinks(grammar)), parseNativeGrammar(renderNativeGrammar(grammar))]) {
    const obligations = candidate.declarations.lexers[0].expression.items.filter(({ kind }) => kind === 'lexicalObligation');
    assert.deepEqual(obligations, [{ kind: 'lexicalObligation', name: '', category: 'option', source: options }]);
    assert.throws(() => compileGrammar(candidate), /unresolved option:.*superClass = FirstBase/u);
  }
  const parserOptions = ' tokenVocab = Election; superClass = ParserBase; ';
  const separate = importAntlr(`parser grammar Parser; options {${parserOptions}} start: A EOF;
    lexer grammar Election; A: 'a';`);
  assert.deepEqual(separate.declarations.lexers[0].expression.items.filter(({ kind }) => kind === 'lexicalObligation'), [
    { kind: 'lexicalObligation', name: 'start', category: 'option', source: parserOptions },
  ]);
  assert.throws(() => compileGrammar(separate).parseTree('a'), /unresolved option in Election:start/u);
});

test('ANTLR empty lexical families retain EOF acceptance and reject unrecognized source bytes', () => {
  const grammar = importAntlr(`grammar Empty; start: EOF;`);
  assert.deepEqual(grammar.rule('start').expression, G.not(G.any()));
  for (const candidate of [grammar, parseGrammarLinks(renderGrammarLinks(grammar)), parseNativeGrammar(renderNativeGrammar(grammar))]) {
    const compiled = compileGrammar(candidate);
    const accepted = compiled.parseTree('');
    assert.equal(accepted.ok, true);
    assert.deepEqual(accepted.tree, { type: 'node', kind: 'start', start: 0, end: 0, children: [] });
    assert.equal(compiled.parseTree('a').ok, false);
  }
});

test('ANTLR parser recovery retains errors without retrying inside a longer token', () => {
  const source = `grammar Election; start: A EOF; A: 'a'; B: [a-z]+;`;
  const compiled = parser(source);
  assert.equal(compiled.parseTree('ba').ok, false);
  const result = compiled.parseTree('ba', { errorRecovery: true, maxRepairs: 2 });
  assert.equal(result.ok, false);
  assert.equal(result.rejection.reason, 'recovered');
  assert.equal(leaves(result.tree).some(({ kind }) => kind === 'A'), false);
  const sourceBytes = (node) => node.type === 'token' || node.type === 'error' ? node.text : (node.children ?? []).map(sourceBytes).join('');
  assert.equal(sourceBytes(result.tree), 'ba');
});

test('ANTLR lexer declarations survive both codecs and public renaming without rewriting local closure', () => {
  const grammar = importAntlr(`grammar Election; start: ID EOF; fragment LETTER: [a-z]; ID: LETTER+;`);
  const renamed = renameGrammarRule(grammar, 'ID', 'name').grammar;
  const links = parseGrammarLinks(renderGrammarLinks(renamed));
  const native = parseNativeGrammar(renderNativeGrammar(renamed));
  assert.deepEqual(links.normalized(), renamed.normalized());
  assert.deepEqual(native.normalized(), renamed.normalized());
  for (const value of [renamed, links, native]) {
    assert.deepEqual(value.undefinedNonterminals(), []);
    assert.equal(compileGrammar(value).parseTree('abc').ok, true);
    assert.equal(leaves(compileGrammar(value).parseTree('abc').tree)[0].kind, 'name');
    assert.equal(value.declarations.lexers[0].expression.items.find((entry) => entry.kind === 'lexicalRule').item.item.name, 'LETTER');
  }
  for (const kind of ['lexicalOwner', 'lexicalBinding']) {
    const invalid = importAntlr(`grammar Election; start: A EOF; A: 'a';`);
    invalid.declarations.lexers[0].expression.items.find((entry) => entry.kind === kind).item.name = 'missing';
    assert.equal(invalid.undefinedNonterminals().includes('missing'), true);
    assert.equal(invalid.undefinedNonterminals().includes('LETTER'), false);
    assert.throws(() => compileGrammar(invalid), /undefined public lexical reference missing/u);
  }
});

test('ANTLR source-local election remains separate from native alternatives after merge', () => {
  const antlr = importAntlr(`grammar Election; channels { ERROR } start: ID EOF; ID: 'aa'; Unexpected: . -> channel(ERROR);`);
  const native = new Grammar('start', new Map([['start', { kind: 'normal', expression: G.literal('native') }]]));
  const merged = mergeGrammars([{ id: 'native', language: 'Toy', grammar: native }, { id: 'antlr', language: 'Toy', grammar: antlr }], { reconcile: true }).groups[0].grammar;
  const compiled = compileGrammar(parseGrammarLinks(renderGrammarLinks(merged)));
  assert.equal(compiled.parseTree('aa').ok, true);
  assert.equal(compiled.parseTree('native').ok, true);
  assert.equal(compiled.parseTree('a').ok, false);
  assert.equal(antlr.rule('Unexpected').channel, 'ERROR');
});

test('ANTLR reconciled parser rules preserve each original owner and unresolved action', () => {
  const grammar = importAntlr(`grammar Election; start: first EOF;
    first: A {effect();}; second: A; A: 'a';`);
  const group = mergeGrammars([{ id: 'source', language: 'Toy', grammar }]).groups[0];
  const canonical = group.identities['source:first'];
  assert.equal(canonical, group.identities['source:second']);
  const expression = group.grammar.rule(canonical).expression;
  assert.equal(expression.kind, 'choice');
  assert.deepEqual(expression.items.map(({ name, owner }) => [name, owner]), [
    ['source::Election', 'first'], ['source::Election', 'second'],
  ]);
  for (const restored of [group.grammar, parseGrammarLinks(renderGrammarLinks(group.grammar)), parseNativeGrammar(renderNativeGrammar(group.grammar))]) {
    assert.throws(() => compileGrammar(restored).parseTree('a'), /unresolved action in source::Election:first/u);
  }
  expression.items[0].owner = 'unregistered';
  assert.throws(() => compileGrammar(group.grammar), /undefined original parser owner unregistered/u);
});

test('ANTLR canonical token sharing retains the original parser token expectation', () => {
  for (const expected of ['A', 'B']) {
    const grammar = importAntlr(`grammar Election; start: ${expected} EOF; A: 'x'; B: 'x';`);
    const group = mergeGrammars([{ id: 'source', language: 'Toy', grammar }]).groups[0];
    assert.equal(group.identities['source:A'], group.identities['source:B']);
    assert.equal(compileGrammar(grammar).parseTree('x').ok, expected === 'A');
    for (const candidate of [group.grammar, parseGrammarLinks(renderGrammarLinks(group.grammar)), parseNativeGrammar(renderNativeGrammar(group.grammar))]) {
      assert.equal(compileGrammar(candidate).parseTree('x').ok, expected === 'A');
    }
  }
});

test('shared lexical winner decision never promotes ineligible candidates or parser viability', () => {
  assert.equal(readLexicalWinnerIndex([1, 3, 3, 8], [true, true, true, false]), 1);
  assert.equal(readLexicalWinnerIndex([1, 1], [true, true]), 0);
  assert.equal(readLexicalWinnerIndex([8], [false]), -1);
  assert.equal(readLexicalWinnerIndex([], []), -1);
});
