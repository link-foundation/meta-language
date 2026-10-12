import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { acceptPrecedenceOperandContext } from '../src/translation/frontend-rules.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

test('lower precedence comma operands remain valid where their list cannot replace the enclosing assignment', () => {
  const parser = compileGrammar(parseGrammarLinks(`
    (grammar (start assignment) (matching longest))
    (extra (class plain (char %20)))
    (rule assignment normal (precedence 15 right (seq (ref identifier) (literal %3D) (choice unordered (ref expression) (ref list)))))
    (rule list normal (precedence -1 none (seq (ref expression) (repeat1 (seq (literal %2C) (ref expression))))))
    (rule expression silent (choice unordered (ref number) (ref sum) (ref product) (ref assignment)))
    (rule sum normal (precedence 1 left (seq (ref expression) (literal %2B) (ref expression))))
    (rule product normal (precedence 2 left (seq (ref expression) (literal %2A) (ref expression))))
    (rule number token (repeat1 (class plain (range 0 9))))
    (rule identifier token (repeat1 (class plain (range a z))))
  `));
  const meaningful = tree => tree.children?.filter(child => !child.trivia) ?? [];
  for (const source of ['x=1,2', 'x=1+2,3', 'x=1,2+3*4', 'x=y=1,2']) {
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, source);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.equal(meaningful(actual.tree).at(-1).kind === 'list'
      || meaningful(meaningful(actual.tree).at(-1)).at(-1)?.kind === 'list', true, source);
  }
  const result = parser.parseTree('x=1+2*3,4');
  const list = meaningful(result.tree).at(-1);
  assert.equal(meaningful(list)[0].kind, 'sum');
  assert.equal(meaningful(meaningful(list)[0]).at(-1).kind, 'product');
  for (const source of ['x=,1', 'x=1,', 'x=1,,2']) assert.equal(parser.parseTree(source).ok, false, source);
});

test('shared source contexts retain regrouping only when both constructs fit a common complete rule', () => {
  assert.equal(acceptPrecedenceOperandContext([true, false], [false, true]), false);
  assert.equal(acceptPrecedenceOperandContext([false, true], [true, true]), true);
  assert.equal(acceptPrecedenceOperandContext([], []), false);
});

test('Ruby rescue assignment retains the complete pinned CI input and independent oracle', () => {
  const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
  const parser = compileGrammar(parseGrammarLinks(read('parity/grammars/native/ruby.lino')));
  const fixture = JSON.parse(read('parity/fixtures/native-grammars/ruby-profile.json'));
  const corpus = JSON.parse(read('parity/fixtures/native-grammars/ruby-profile-corpus.json'));
  const entry = corpus.cases.find(value => value.file === 'control-flow.txt' && value.title === 'rescue modifier in assignments');
  assert.ok(entry);
  const actual = parser.parseTree(entry.source);
  const oracle = oracleSnapshot(entry.source, 'ruby');
  assert.equal(oracle.recovers, false);
  assert.equal(actual.ok, true, JSON.stringify(actual.rejection));
  assert.deepEqual(actual.ambiguities, []);
  assert.deepEqual(nativeRows(actual.tree, entry.source, fixture), oracle.rows);
});
