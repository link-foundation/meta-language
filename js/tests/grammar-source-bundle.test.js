import assert from 'node:assert/strict';
import { test } from 'node:test';
import { parseWithGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { importGrammarSourceBundle } from '../scripts/grammar-source-bundle.mjs';

const sources = {
  document: { format: 'native', source: 'import words\nstart document\nrule document = normal seq(ref(word), literal(" "), ref(word))\n' },
  words: { format: 'native', source: 'import alphabet\nstart word\nrule word = normal repeat1(ref(letter))\n' },
  alphabet: { format: 'ebnf', source: 'letter = "a" | "b" ;\n' },
};

test('included grammar definitions become one source-free executable linked grammar', () => {
  const bundle = importGrammarSourceBundle('document', sources);
  assert.equal(bundle.provenance.length, 3);
  assert.deepEqual(bundle.provenance.find(({ id }) => id === 'words').imports, ['alphabet']);
  assert.deepEqual(bundle.grammar.declarations.imports, undefined);
  const replay = parseGrammarLinks(bundle.links);
  assert.equal(parseWithGrammar(replay, 'ab ba').kind, 'document');
  assert.throws(() => parseWithGrammar(replay, 'ac ba'));
  assert.deepEqual(replay.rule('letter').sourceNames, [{ source: 'alphabet', name: 'letter' }]);
});

test('changing an included native definition changes ordinary parsing without a resolver', () => {
  const changed = { ...sources, alphabet: { format: 'ebnf', source: 'letter = "a" | "c" ;\n' } };
  const before = importGrammarSourceBundle('document', sources);
  const after = importGrammarSourceBundle('document', changed);
  assert.notEqual(before.links, after.links);
  assert.throws(() => parseWithGrammar(before.grammar, 'ac ca'));
  assert.equal(parseWithGrammar(after.grammar, 'ac ca').kind, 'document');
  assert.throws(() => parseWithGrammar(after.grammar, 'ab ba'));
});

test('included scanner and action definitions retain their native operational meaning', () => {
  const bundle = importGrammarSourceBundle('document', {
    document: { format: 'native', source: 'import scanner\nstart document\nrule document = normal ref(wrapper)\n' },
    scanner: { format: 'native', source: 'start wrapper\nscanner letters tokens(word) operations(consume(literal("a")), while(next(literal("a")), do(advance)), emit(word))\nrule wrapper = normal ref(word) action(buildNode(parsed_word))\n' },
  });
  assert.equal(parseWithGrammar(bundle.grammar, 'aaa').kind, 'document');
  assert.equal(parseWithGrammar(bundle.grammar, 'aaa').children[0].kind, 'parsed_word');
  assert.throws(() => parseWithGrammar(bundle.grammar, 'aab'));
});

test('missing dependencies and dependency cycles retain precise import failures', () => {
  assert.throws(() => importGrammarSourceBundle('document', { document: sources.document }), /supporting grammar words is not registered/u);
  const cyclic = { ...sources, alphabet: { format: 'native', source: 'import document\nrule letter = normal literal("a")\n' } };
  assert.throws(() => importGrammarSourceBundle('document', cyclic), /document -> words -> alphabet -> document/u);
});
