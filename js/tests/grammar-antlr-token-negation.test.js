// ANTLR 4.13.2 parser token exclusions retain complete elected tokens and CSTs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../scripts/project-data.mjs';
import {selectAntlrReferenceAssertion} from '../scripts/antlr-version-assertions.mjs';
import {importAntlr} from '../src/grammar-importers/antlr.js';
import {emitAntlr} from '../src/grammar-emitters/antlr.js';
import {acceptGrammarParserTokenNegation} from '../src/translation/grammar-lexer-rules.js';
import {compileGrammar, deserializeGrammar, GrammarBuilder as G} from '../src/grammar.js';
import {parseGrammarLinks, renderGrammarLinks} from '../src/grammar-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary, decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeStructuredDataLinks, decodeStructuredDataLinks} from '../src/structured-data-links.js';

assert.equal(selectAntlrReferenceAssertion().activeVersion, '4.13.2');
const controls = readProjectData(new URL('../../parity/fixtures/grammar-parser-token-negation-controls.lino', import.meta.url));
assert.equal(controls.baseline.revision, '63e37589e9d7f37c5f1099651c1cbc702b7403d3');
assert.equal(controls.baseline.importer.sha256, '52b58caf89799535192131987e018aed71654cff19942290c965d2482e19eb29');
assert.equal(createHash('sha256').update(controls.baseline.importer.source).digest('hex'), controls.baseline.importer.sha256);
assert.equal(Buffer.byteLength(controls.baseline.importer.source), controls.baseline.importer.bytes);
assert.equal(controls.cases.length, 4);
const baselinePath = new URL('../src/grammar-importers/antlr.js', import.meta.url);
const baselineSource = controls.baseline.importer.source
  .replace(/\bfrom (['"])(\.[^'"\n]+)\1/gu, (_, quote, path) => 'from ' + quote + new URL(path, baselinePath).href + quote)
  .replaceAll('import.meta.url', JSON.stringify(baselinePath.href));
const {importAntlr: importOriginalAntlr} = await import('data:text/javascript;base64,' + Buffer.from(baselineSource).toString('base64'));
function readTokenNegationBaseline(source) {
  const digest = createHash('sha256').update(source).digest('hex');
  const entry = controls.cases.find(row => row.sha256 === digest);
  assert.ok(entry, 'the unchanged authored input must have an original importer snapshot');
  assert.equal(entry.source, source);
  assert.equal(entry.bytes, Buffer.byteLength(source));
  const grammar = importOriginalAntlr(source);
  assert.deepEqual(grammar.normalized(), entry.normalized);
  return grammar;
}

const leaves = node => node.type === 'token' ? [node] : (node.children ?? []).flatMap(leaves);
const declaration = body => `grammar Exclusion; start: (${body})+ EOF;
  STOP: 'stop'; ALT: 'a'; LONG: 'alpha'; RETYPE: 'x' -> type(STOP);
  WORD: [a-z]+; TEXT: [é]+; WS: [ ]+ -> channel(HIDDEN);`;

// Independent authored-fixture oracle: whole words are elected before the
// exclusion. It does not read the imported AST or invoke meta-language.
// This is NOT an executed ANTLR oracle and must not be recorded as one.
function fixtureOracle(source) {
  const tokens = [];
  let byte = 0;
  for (const match of source.matchAll(/[a-z]+|é+|[ ]+|./gu)) {
    const text = match[0];
    const end = byte + Buffer.byteLength(text);
    const kind = /^ +$/u.test(text) ? 'WS' : text === 'stop' || text === 'x' ? 'STOP'
      : text === 'a' ? 'ALT' : text === 'alpha' ? 'LONG' : /^é+$/u.test(text) ? 'TEXT'
      : /^[a-z]+$/u.test(text) ? 'WORD' : null;
    if (!kind || kind === 'STOP' || kind === 'ALT') return null;
    tokens.push([kind, text, byte, end, kind === 'WS']);
    byte = end;
  }
  return tokens.some(row => !row[4]) ? tokens : null;
}

function replays(grammar) {
  const text = parseGrammarLinks(renderGrammarLinks(grammar));
  const packet = encodeStructuredDataLinks(grammar.normalized());
  const physical = LinkNetwork.fromLino(packet.network.toLino());
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(physical));
  const restored = deserializeGrammar(decodeStructuredDataLinks(binary, packet.root));
  assert.deepEqual(restored.normalized(), grammar.normalized());
  return [grammar, text, restored];
}

test('parser negation policy selects complete token sets without changing lexer complements', () => {
  assert.equal(acceptGrammarParserTokenNegation(true, ['ref'], ['STOP']), true);
  assert.equal(acceptGrammarParserTokenNegation(true, ['literal','ref'], ['stop','ALT']), true);
  assert.equal(acceptGrammarParserTokenNegation(true, ['not','ref'], ['any','ALT']), true);
  for (const args of [
    [false,['ref'],['STOP']], [true,[],[]], [true,['ref'],[]],
    [true,['ref'],['parserRule']], [true,['literal'],['']],
    [true,['any'],['']], [true,['not'],['']], [true,['charClass'],['']],
  ]) assert.equal(acceptGrammarParserTokenNegation(...args), false);
  const fixture = new URL('../../rust/tests/fixtures/grammar/antlr/set-complement.g4', import.meta.url);
  const original = readTokenNegationBaseline(readFileSync(fixture,'utf8'));
  const draft = importAntlr(readFileSync(fixture,'utf8'));
  assert.deepEqual(draft.normalized(), original.normalized(), 'all original lexer complement fields remain exact');
});

test('parser negated reference and literal sets consume elected tokens including complete trivia and UTF-8 spans', () => {
  const source = ' alpha beta é ';
  const expected = [
    ['WS',' ',0,1,true], ['LONG','alpha',1,6,false], ['WS',' ',6,7,true],
    ['WORD','beta',7,11,false], ['WS',' ',11,12,true], ['TEXT','é',12,14,false], ['WS',' ',14,15,true],
  ];
  assert.deepEqual(fixtureOracle(source), expected);
  for (const excluded of ['~(STOP | ALT)', "~('stop' | 'a')"]) {
    const grammar = importAntlr(declaration(excluded));
    for (const replay of replays(grammar)) {
      const parser = compileGrammar(replay);
      for (const input of [source, 'alpha', 'stopper', 'beta', 'éé', '', ' ', 'a', 'stop', 'x', 'alpha stop', '@']) {
        const oracle = fixtureOracle(input);
        const result = parser.parseTree(input);
        assert.equal(result.ok, oracle !== null, `${excluded}: ${JSON.stringify(input)}`);
        if (oracle) {
          assert.deepEqual(leaves(result.tree).map(node => [node.kind,node.text,node.start,node.end,node.trivia === true]), oracle);
          assert.equal(leaves(result.tree).map(node => node.text).join(''), input);
          assert.equal(result.tree.end, Buffer.byteLength(input));
        }
      }
    }
  }
  const single = compileGrammar(importAntlr("grammar One; start: ~'a' EOF; A:'a'; LONG:'alpha';"));
  assert.equal(single.parseTree('alpha').ok,true, 'excluding one token never excludes a longer token by its character prefix');
  assert.equal(single.parseTree('a').ok,false);
  assert.equal(single.parseTree('').ok,false, 'negation never consumes EOF');
});

test('parser EOF exclusions consume a complete non-EOF token and keep insensitive literals unsupported', () => {
  const source = "grammar End; start: ~EOF A EOF; A:'a';";
  const original = compileGrammar(importOriginalAntlr(source));
  assert.equal(original.parseTree('a').ok, true, 'the original zero-width EOF complement accepted only one token');
  for (const grammar of replays(importAntlr(source))) {
    const parser = compileGrammar(grammar);
    for (const input of ['', 'a', 'aa', 'aaa']) {
      const result = parser.parseTree(input);
      assert.equal(result.ok, input === 'aa', input);
      if (result.ok) assert.deepEqual(leaves(result.tree).map(node => [node.kind,node.text,node.start,node.end]),
        [['A','a',0,1],['A','a',1,2]]);
    }
  }
  for (const grammar of replays(importAntlr(declaration('~(EOF | STOP | ALT)')))) {
    const parser = compileGrammar(grammar);
    for (const input of ['alpha', ' alpha beta é ', '', ' ', 'stop', 'x', 'a']) {
      const expected = fixtureOracle(input);
      const result = parser.parseTree(input);
      assert.equal(result.ok, expected !== null, input);
      if (expected) assert.deepEqual(leaves(result.tree).map(node => [node.kind,node.text,node.start,node.end,node.trivia === true]), expected);
    }
  }
  const insensitive = "grammar Case; options {caseInsensitive=true;} start: ~'a' EOF; A:'a'; WORD:[a-z]+;";
  for (const grammar of [importOriginalAntlr(insensitive), importAntlr(insensitive)]) {
    assert.throws(() => compileGrammar(grammar).parseTree('b'), /unsupported parser terminal literalInsensitive/u);
  }
});

test('negation lookahead cannot apply mode commands twice or remove foreign obligations', () => {
  const mode = `parser grammar Parser; options {tokenVocab=Mode;} start: ~BLOCK TEXT CLOSE EOF;
    lexer grammar Mode; OPEN:'<' -> pushMode(INNER); BLOCK:'x';
    mode INNER; TEXT:[a-z]+; CLOSE:'>' -> popMode;`;
  for (const grammar of replays(importAntlr(mode))) {
    const result = compileGrammar(grammar).parseTree('<word>');
    assert.equal(result.ok,true);
    assert.deepEqual(leaves(result.tree).map(node=>[node.kind,node.text]), [['OPEN','<'],['TEXT','word'],['CLOSE','>']]);
  }
  // The final token is valid only in the original mode. A duplicated push
  // would leave INNER active after CLOSE and prevent TAIL from being elected.
  const following = `parser grammar Parser; options {tokenVocab=Mode;} start: ~BLOCK TEXT CLOSE TAIL EOF;
    lexer grammar Mode; OPEN:'<' -> pushMode(INNER); BLOCK:'x'; TAIL:'!';
    mode INNER; TEXT:[a-z]+; CLOSE:'>' -> popMode;`;
  for (const grammar of replays(importAntlr(following))) {
    const result = compileGrammar(grammar).parseTree('<word>!');
    assert.equal(result.ok, true);
    assert.deepEqual(leaves(result.tree).map(node => [node.kind,node.text]), [['OPEN','<'],['TEXT','word'],['CLOSE','>'],['TAIL','!']]);
  }
  for (const source of [
    "grammar Foreign; options {superClass=Missing;} start: ~BLOCK EOF; BLOCK:'x'; WORD:[a-z]+;",
    "grammar Foreign; start: {allow()}? ~BLOCK EOF; BLOCK:'x'; WORD:[a-z]+;",
  ]) {
    const before = readTokenNegationBaseline(source);
    const after = importAntlr(source);
    const obligations = grammar => grammar.declarations.lexers.flatMap(lexer=>lexer.expression.items.filter(entry=>entry.kind==='lexicalObligation'));
    assert.deepEqual(obligations(after),obligations(before));
    assert.throws(()=>compileGrammar(after).parseTree('word'), /unresolved (option|predicate)/u);
  }
  const covering = new URL('../../rust/tests/fixtures/grammar/antlr/covering.g4', import.meta.url);
  const before = readTokenNegationBaseline(readFileSync(covering,'utf8'));
  const after = importAntlr(readFileSync(covering,'utf8'));
  for (const name of before.ruleNames()) {
    if (name !== 'item') assert.deepEqual(after.rule(name),before.rule(name),name);
  }
  assert.deepEqual(after.rule('item').expression.items.slice(0,2), before.rule('item').expression.items.slice(0,2));
  assert.deepEqual(after.rule('item').expression.items[2],G.seq(G.not(G.literal('x')),G.any()));
});

test('ANTLR emission and reimport preserve whole-token exclusions, EOF and a separate following wildcard', () => {
  for (const excluded of ['~(STOP | ALT)', "~('stop' | 'a')", '~(EOF | STOP | ALT)']) {
    for (const grammar of replays(importAntlr(declaration(excluded)))) {
      const emitted = emitAntlr(grammar);
      const parser = compileGrammar(importAntlr(emitted.source));
      for (const input of ['alpha', ' alpha beta é ', '', ' ', 'stop', 'x', 'a', 'stopper', '@']) {
        const expected = fixtureOracle(input);
        const result = parser.parseTree(input);
        assert.equal(result.ok, expected !== null, `${excluded}: ${JSON.stringify(input)}`);
        if (expected) assert.deepEqual(leaves(result.tree).map(node => [node.kind,node.text,node.start,node.end,node.trivia === true]), expected);
      }
    }
  }
  for (const [source, accepted, refused] of [
    ["grammar End; start: ~EOF A EOF; A:'a';", ['aa'], ['', 'a', 'aaa']],
    ["grammar Pair; start: ~BLOCK . EOF; BLOCK:'x'; A:'a';", ['aa', 'ax'], ['', 'a', 'xa', 'aaa']],
  ]) {
    for (const grammar of replays(importAntlr(source))) {
      const emitted = emitAntlr(grammar);
      const parser = compileGrammar(importAntlr(emitted.source));
      for (const input of accepted) {
        const result = parser.parseTree(input);
        assert.equal(result.ok, true, input);
        assert.equal(leaves(result.tree).map(node => node.text).join(''), input);
        assert.equal(leaves(result.tree).length, 2, 'the exclusion and following terminal each consume once');
      }
      for (const input of refused) assert.equal(parser.parseTree(input).ok, false, input);
    }
  }
});
