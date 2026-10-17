import assert from 'node:assert/strict';
import test from 'node:test';
import { sameGrammarOutput, sameGrammarTree } from '../src/grammar-runtime/tree-equality.js';
import { readGrammarSelectionEqualityFields, readGrammarTreeEqualityFields,
  readGrammarTreeRecursionPlan } from '../src/translation/grammar-selection-rules.js';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { authoredGrammarSelectionListing,
  grammarSelectionExpression, grammarSelectionLiteral } from './support/grammar-selection-cases.js';

function grammarSelectionEqualityTree() {
  const provenance = language => ({ source: language, language, edition: 'authored',
    rules: [{ rule: 'document', origin: `${language}:document`, concept: `shared:${language}`, sourceNames: [] }] });
  const root = language => ({ type: 'node', kind: `${language}_root`, start: 0, end: 3, children: [
    { type: 'token', kind: null, start: 0, end: 1, trivia: true },
    { type: 'token', kind: 'word', field: 'value', start: 1, end: 3, attributes: { role: 'word' },
      lexical: { lexer: language, rule: 'word', type: 'word', channel: 'default', disposition: 'emit', future: { ordered: ['a', 'b'] } } },
  ] });
  return { type: 'embed', language: 'first', start: 0, end: 3, root: root('first'), program: { private: 1 },
    selection: { mode: 'all-grammar', mapping: 'identity', start: 0, end: 3,
      candidates: ['first', 'second', 'third'].map(language => ({ language, relation: 'accepted' })),
      provenance: provenance('first'), ambiguous: false,
      alternatives: ['second', 'third'].map(language => ({ language, root: root(language), program: { private: 2 }, provenance: provenance(language) })) } };
}

test('full embedded CSTs and every parallel witness remain distinct under both comparisons', () => {
  const reference = grammarSelectionEqualityTree();
  const equal = structuredClone(reference);
  equal.program = { differentTransport: true };
  equal.selection.alternatives[0].program = { differentTransport: true };
  assert.equal(sameGrammarOutput([reference], [equal]), true);
  assert.equal(sameGrammarTree(reference, equal), true);
  const changes = [
    tree => { tree.language = 'other'; },
    tree => { tree.root.kind = 'other_root'; },
    tree => { tree.root.children[0].trivia = false; },
    tree => { tree.root.children[1].field = 'other'; },
    tree => { tree.root.children[1].end = 2; },
    tree => { tree.root.children[1].projectionName = 'other'; },
    tree => { tree.root.children[1].projectionAnonymous = true; },
    tree => { tree.root.children[1].attributes.role = 'other'; },
    tree => { tree.root.children[1].lexical.future.ordered.reverse(); },
    tree => { tree.root.children[1].lexical.disposition = 'skip'; },
    tree => { delete tree.root.children[1].lexical.future; },
    tree => { tree.selection.mode = 'any-grammar'; },
    tree => { tree.selection.mapping = 'quoted-escape'; },
    tree => { tree.selection.start = 1; },
    tree => { tree.selection.end = 2; },
    tree => { tree.selection.ambiguous = true; },
    tree => { tree.selection.candidates.reverse(); },
    tree => { tree.selection.candidates[0].relation = 'unknown'; },
    tree => { tree.selection.provenance.rules[0].concept = null; },
    tree => { tree.selection.alternatives[0].root.children[0].kind = 'space'; },
    tree => { tree.selection.alternatives[0].language = 'other'; },
    tree => { tree.selection.alternatives[0].provenance.edition = 'other'; },
    tree => { tree.selection.alternatives.reverse(); },
    tree => { tree.selection.alternatives.pop(); },
    tree => { delete tree.selection; },
    tree => { tree.selection.future = { explicit: true }; },
  ];
  for (const change of changes) {
    const candidate = structuredClone(reference);
    change(candidate);
    assert.equal(sameGrammarOutput([reference], [candidate]), false, change.toString());
    assert.equal(sameGrammarTree(reference, candidate), false, change.toString());
  }
});

test('metadata preserves absent keys, nulls, array lengths and object order while attributes keep their existing rule', () => {
  const reference = grammarSelectionEqualityTree(), other = structuredClone(reference);
  const leaf = tree => tree.root.children[1];
  leaf(reference).lexical = { lexer: 'first', extra: null };
  leaf(other).lexical = { extra: null, lexer: 'first' };
  assert.equal(sameGrammarOutput([reference], [other]), false, 'linked metadata key order is explicit');
  leaf(other).lexical = { lexer: 'first' };
  assert.equal(sameGrammarTree(reference, other), false, 'absent and explicit null metadata differ');
  delete leaf(reference).lexical;
  leaf(other).lexical = null;
  assert.equal(sameGrammarOutput([reference], [other]), false, 'absent and explicit null lexical carriers differ');
  delete leaf(other).lexical;
  leaf(reference).attributes = { first: 1, second: 2 };
  leaf(other).attributes = { second: 2, first: 1 };
  assert.equal(sameGrammarOutput([reference], [other]), true, 'ordinary attributes retain order-independent equality');
  reference.selection.provenance.future = [];
  other.selection.provenance.future = new Array(1);
  assert.equal(sameGrammarOutput([reference], [other]), false, 'array length survives even with no enumerable elements');
  other.selection.provenance.future = [];
  reference.selection.provenance.absent = undefined;
  assert.equal(sameGrammarOutput([reference], [other]), false, 'own-key presence survives undefined values');
  const missing = { type: 'missing', kind: 'word', start: 1, end: 1 };
  assert.equal(sameGrammarOutput([missing], [{ ...missing, literal: true }]), false, 'missing literal and missing rule remain distinct');
});

test('structural whitespace ownership stays unchanged outside embedded concrete roots', () => {
  const token = { type: 'token', kind: 'word', start: 1, end: 2 };
  const first = { type: 'node', kind: 'document', start: 0, end: 2, children: [
    { type: 'token', kind: null, start: 0, end: 1, trivia: true }, token,
  ] };
  const second = { ...first, start: 1, children: [{ ...token }] };
  assert.equal(sameGrammarTree(first, second), true);
  assert.equal(sameGrammarOutput([first], [second]), false);
});

test('actual grammar alternatives with equal primary embeddings retain their different all-policy witnesses', () => {
  const literal = grammarSelectionLiteral('π');
  const select = names => grammarSelectionExpression('all-grammar', names, literal);
  const outer = parseGrammarLinks(authoredGrammarSelectionListing(`(choice unordered ${select(['first', 'second'])} ${select(['first', 'third'])})`));
  const grammars = new Map(['first', 'second', 'third'].map(name => [name,
    parseGrammarLinks(authoredGrammarSelectionListing(literal, `${name}_root`))]));
  const outcome = compileGrammar(outer, { resolveGrammar: name => grammars.get(name) }).parseTree('π');
  assert.equal(outcome.ok, true);
  assert.deepEqual(outcome.ambiguities, [{ rule: 'document', start: 0, end: 2 }]);
  assert.deepEqual(outcome.tree.children[0].selection.alternatives.map(item => item.language), ['second']);
});

test('portable equality decisions declare concrete recursion and every selection header field', () => {
  assert.equal(readGrammarTreeRecursionPlan('embed', false), 'full-embedding');
  assert.equal(readGrammarTreeRecursionPlan('embed', true), 'full-embedding');
  assert.equal(readGrammarTreeRecursionPlan('node', false), 'meaningful-children');
  assert.equal(readGrammarTreeRecursionPlan('node', true), 'full-children');
  assert.equal(readGrammarTreeRecursionPlan('token', true), 'leaf');
  assert.ok(readGrammarTreeEqualityFields('token', true).split('|').includes('lexical'));
  assert.deepEqual(readGrammarSelectionEqualityFields(false).split('|'), ['mode', 'mapping', 'start', 'end', 'candidates', 'provenance', 'ambiguous']);
  assert.deepEqual(readGrammarSelectionEqualityFields(true).split('|'), ['language', 'provenance']);
});
