import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

test('a composite extra owns its scanner priority independently of the following syntax token', () => {
  const parser = compileGrammar(parseGrammarLinks(`
    (grammar (start document) (matching longest))
    (extra (ref body))
    (scanner context (tokens start content close) (operations (if (valid start) (then (consume (literal %0A)) (emit start))) (if (valid content) (then (consume (literal c)) (emit content))) (if (valid close) (then (consume (literal %21)) (emit close))) fail))
    (rule body normal (seq (immediateToken (ref start)) (immediateToken (ref content)) (immediateToken (ref close))))
    (rule document normal (seq (literal a) (literal %2C) (literal b)))
  `));
  const actual = parser.parseTree('a,\nc!b');
  assert.equal(actual.ok, true);
  assert.deepEqual(actual.ambiguities, []);
  assert.equal(renderSyntaxTree(actual.tree), '(document "a" "," ~(body "\\n" "c" "!") "b")');
  for (const source of ['a,\nc!d', 'a,\nc']) assert.equal(parser.parseTree(source).ok, false, source);
});

test('heredoc extras preserve the complete pinned argument, mapping and array CI case', () => {
  const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = file => readProjectData(new URL(`../../${file}`, import.meta.url));
  const parser = compileGrammar(parseGrammarLinks(read('parity/grammars/native/ruby.lino')));
  const fixture = readData('parity/fixtures/native-grammars/ruby-profile.json');
  const corpus = readData('parity/fixtures/native-grammars/ruby-profile-corpus.json');
  const entry = corpus.cases.find(value => value.file === 'literals.txt' && value.title === 'heredocs with in args, pairs, and arrays');
  assert.ok(entry);
  const actual = parser.parseTree(entry.source);
  const oracle = oracleSnapshot(entry.source, 'ruby');
  assert.equal(oracle.recovers, false);
  assert.equal(actual.ok, true, JSON.stringify(actual.rejection));
  assert.deepEqual(actual.ambiguities, []);
  assert.deepEqual(nativeRows(actual.tree, entry.source, fixture), oracle.rows);
});
