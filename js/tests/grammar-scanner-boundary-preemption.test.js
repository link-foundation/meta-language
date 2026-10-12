import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { acceptScannerPreemptionBoundary } from '../src/translation/frontend-rules.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

test('an already shifted scanner boundary stays compatible while other continuations retain preemption', () => {
  for (const virtualToken of [false, true]) for (const atBoundary of [false, true]) {
    assert.equal(acceptScannerPreemptionBoundary(virtualToken, atBoundary), !(virtualToken && atBoundary));
  }
  const parser = compileGrammar(parseGrammarLinks(`
    (grammar (start document) (matching longest))
    (extra (class plain (char %20) (char %0A)))
    (scanner layout (tokens boundary) (operations (if (not (valid boundary)) (then fail)) (if (next (literal %0A)) (then (emit boundary))) fail))
    (rule document normal (seq (choice unordered (ref single) (ref continued)) (optional (immediateToken (ref boundary)))))
    (rule single normal (literal a))
    (rule continued normal (seq (ref single) (literal b)))
  `));
  for (const source of ['a', 'a\n', 'a b']) assert.equal(parser.parseTree(source).ok, true, source);
  assert.equal(parser.parseTree('a\nb').ok, false, 'the absent boundary still preempts the competing lexer continuation');
});

test('Ruby shifted boundaries preserve three root statements and the original CI declaration cases', () => {
  const listing = readFileSync(new URL('../../parity/grammars/native/ruby.lino', import.meta.url), 'utf8');
  const parser = compileGrammar(parseGrammarLinks(listing));
  const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/native-grammars/ruby-profile.json', import.meta.url), 'utf8'));
  const corpus = JSON.parse(readFileSync(new URL('../../parity/fixtures/native-grammars/ruby-profile-corpus.json', import.meta.url), 'utf8'));
  const projection = { ...fixture, oracleKinds: nativeOracleKinds(listing) };
  const sources = ['case expr\n in 5\nend\n'.repeat(3),
    ...['empty class', 'for', 'rescue modifier'].map(title => {
      const pinned = corpus.cases.find(test => test.title === title);
      assert.ok(pinned, 'each original CI case remains pinned');
      return pinned.source;
    })];
  const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
  for (const source of sources) {
    const oracle = oracleSnapshot(source, 'Ruby');
    assert.equal(oracle.recovers, false);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, projection), oracle.rows, source);
    assert.equal(text(native.tree), source);
  }
});
