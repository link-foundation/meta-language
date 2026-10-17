import assert from 'node:assert/strict';
import test from 'node:test';
import { selectAntlrReferenceAssertion } from '../scripts/antlr-version-assertions.mjs';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { emitAntlr } from '../src/grammar-emitters/antlr.js';
import { compileGrammar, deserializeGrammar } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from '../src/structured-data-links.js';

assert.equal(selectAntlrReferenceAssertion().activeVersion, '4.13.2');

const source = `grammar Replay; start: (WORD | STOP | FIRST)+ EOF;
FIRST:'if'; RETYPE:'x' -> type(STOP); WORD:[a-z]+; STOP:'!';
WS:[ ]+ -> channel(HIDDEN); DROP:'#' -> skip;`;
const leaves = node => node.type === 'token' ? [node] : (node.children ?? []).flatMap(leaves);
const observations = result => result.ok ? leaves(result.tree).map(node => [node.kind, node.text,
  node.start, node.end, node.trivia === true, node.lexical.rule, node.lexical.type,
  node.lexical.channel, node.lexical.disposition]) : null;

test('source-free ANTLR command emission uses ordered actual lexer bodies and complete token plans', () => {
  const original = importAntlr(source);
  const packet = encodeStructuredDataLinks(original.normalized());
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(packet.network.toLino())));
  const restored = deserializeGrammar(decodeStructuredDataLinks(binary, packet.root));
  const parser = compileGrammar(original);
  // Public STOP contains the structural retype alias. Actual lexical STOP
  // still matches only '!'; emission must not elect RETYPE as STOP itself.
  const raw = emitAntlr(original);
  assert.equal(original.rule('STOP').expression.kind, 'choice');
  assert.match(raw.source, /STOP : '!' ;/u);
  for (const replay of [parseGrammarLinks(renderGrammarLinks(original)), restored]) {
    const emitted = emitAntlr(replay);
    assert.match(emitted.source, /RETYPE : 'x' -> type\(STOP\) ;/u);
    assert.match(emitted.source, /WS : \[ \]\+ -> channel\(HIDDEN\) ;/u);
    assert.match(emitted.source, /DROP : '#' -> skip ;/u);
    if (replay.rule('WS').doc === undefined) assert.match(emitted.source, /STOP : '!' ;/u);
    const order = ['FIRST','RETYPE','WORD','STOP','WS','DROP'].map(name => emitted.source.indexOf('\n' + name + ' :'));
    assert.ok(order.every((position, index) => position >= 0 && (index === 0 || order[index - 1] < position)));
    const reimported = compileGrammar(importAntlr(emitted.source));
    for (const input of ['if x word !', '#if# x ', 'x', 'if', 'word', '', ' ', '@']) {
      assert.deepEqual(observations(reimported.parseTree(input)), observations(parser.parseTree(input)), input);
    }
  }
  const prose = deserializeGrammar(original.normalized());
  prose.rules.set('WS', { ...prose.rule('WS'), doc: 'retained whitespace documentation' });
  const emitted = emitAntlr(prose);
  assert.match(emitted.source, /\/\/ retained whitespace documentation\nWS : \[ \]\+ -> channel\(HIDDEN\) ;/u);
  assert.deepEqual(observations(compileGrammar(importAntlr(emitted.source)).parseTree('if x')),
    observations(parser.parseTree('if x')));
});

test('source-free ANTLR commands refuse ambiguous owners, unresolved meanings and unsupported sections', () => {
  const fresh = () => deserializeGrammar(importAntlr(source).normalized());
  const refusals = [
    grammar => grammar.declarations.lexers.push({ ...grammar.declarations.lexers[0], name: 'Other' }),
    grammar => grammar.declarations.lexers[0].expression.items.push({ kind: 'lexicalMode', name: 'OTHER' }),
    grammar => grammar.declarations.lexers[0].expression.items.push({ kind: 'lexicalChannel', name: 'CUSTOM' }),
    grammar => grammar.declarations.lexers[0].expression.items.push({ kind: 'lexicalType', name: 'DECLARED' }),
    grammar => grammar.declarations.lexers[0].expression.items.push({ kind: 'lexicalObligation', name: '', category: 'option', source: 'unresolved' }),
    grammar => grammar.declarations.lexers[0].expression.items.push(grammar.declarations.lexers[0].expression.items.find(entry => entry.kind === 'lexicalRule')),
    grammar => { grammar.declarations.lexers[0].expression.items.find(entry => entry.kind === 'lexicalBinding' && entry.name === 'STOP').item.name = 'WORD'; },
    grammar => grammar.rules.delete('WS'),
  ];
  for (const mutate of refusals) {
    const grammar = fresh(); mutate(grammar);
    assert.throws(() => emitAntlr(grammar), error => error.name === 'GrammarEmitError'
      && /source-free lexical commands:/u.test(error.message));
  }
});
