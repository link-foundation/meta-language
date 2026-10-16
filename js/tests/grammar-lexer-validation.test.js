import assert from 'node:assert/strict';
import test from 'node:test';
import { Grammar, GrammarBuilder as G, compileGrammar } from '../src/grammar.js';
import { displayGrammarExpression, validateGrammar } from '../src/grammar-validate.js';

test('grammar validation treats lexical scope as a wrapper and elected tokens as consuming terminals', () => {
  const elected = { kind: 'lexicalToken', name: 'source::lexer', token: 'WORD' };
  const scoped = { kind: 'lexicalScope', name: 'source::lexer', owner: 'start', item: G.repeat1(elected) };
  const grammar = new Grammar('start', new Map([['start', { kind: 'normal', expression: scoped }]]));
  assert.deepEqual(validateGrammar(grammar), []);
  assert.equal(displayGrammarExpression(scoped), 'lexicalScope("source::lexer", "start", (lexicalToken("source::lexer", "WORD"))+)');
  const bad = new Grammar('start', new Map([['start', { kind: 'normal', expression: { ...scoped, item: G.repeat1(G.empty()) } }]]));
  const diagnostics = validateGrammar(bad);
  assert.equal(diagnostics.filter(({ kind }) => kind === 'nullable-repetition').length, 2);
  assert.equal(diagnostics.some(({ message }) => message.includes('make the repeated expression consume input')), true);
  const unbound = new Grammar('start', new Map([['start', { kind: 'normal', expression: { ...elected, name: '' } }]]));
  assert.throws(() => compileGrammar(unbound), /undefined lexer scope/u);
});
