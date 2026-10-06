// The web-tree-sitter frontend reports the node kind the native runtime
// reports: tree-sitter-scala 0.26.2 maps the anonymous `(` of class parameters
// to the public symbol its `arguments` alias renamed, and the kind must stay
// `(` while the aliased `arguments` node keeps its alias.
import assert from 'node:assert/strict';
import test from 'node:test';

import { parseProgrammingLanguage } from '../src/programming-language-parser.js';

const terms = (node) => [node.term, ...(node.children ?? []).flatMap(terms)];

test('an anonymous token that shares a public symbol with an alias keeps its native kind', () => {
  const parsed = parseProgrammingLanguage('class A(x: Int)\n', 'scala');
  assert.deepEqual(
    parsed.tokens.map(({ kind }) => kind),
    ['class', 'whitespace', 'identifier', '(', 'identifier', ':', 'whitespace', 'type_identifier', ')', 'whitespace'],
  );
  assert.ok(!terms(parsed.tree).includes('arguments'), 'class parameters have no arguments node');
});

test('an aliased node keeps its alias', () => {
  const parsed = parseProgrammingLanguage('object O { f(1) }\n', 'scala');
  assert.ok(terms(parsed.tree).includes('arguments'), 'a call has an arguments node');
  assert.ok(parsed.tokens.some(({ kind, text }) => kind === '(' && text === '('));
});
