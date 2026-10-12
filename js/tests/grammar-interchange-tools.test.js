import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import peggy from 'peggy';
import { checkIndependentGrammarCycle, importIndependentPeggyAst } from '../scripts/grammar-interchange-tools.mjs';
import { importPest } from '../src/grammar-importers.js';
import { emitPeggy } from '../src/grammar.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-interchange-tools.json', import.meta.url), 'utf8'));

for (const entry of fixture.cases) {
  test(`${entry.id} executes the independent target and reimports its actual AST`, () => {
    const result = checkIndependentGrammarCycle(entry, peggy);
    assert.equal(result.targetVersion, '5.1.0');
    assert.deepEqual(result.failures, []);
    assert.ok(result.observations.some(({ target }) => target.accepted && target.tree.children.length === 2), 'target actions constructed the word nodes');
    assert.equal(result.mutation.originalWitnessRejectedByTarget, true);
    assert.equal(result.mutation.originalWitnessRejectedByNativeReplay, true);
  });
}

test('independent tool replay preserves precedence and ordered lexical decisions', () => {
  const result = checkIndependentGrammarCycle({
    id: 'ordered-lexical-precedence', format: 'pest',
    source: 'expression = { product ~ ("+" ~ product)* }\nproduct = { number ~ ("*" ~ number)* }\nnumber = { "12" | "1" | "2" | "3" }\n',
    accepts: [{ source: '1+2*3', tree: {
      kind: 'expression', start: 0, end: 5, text: '1+2*3', children: [
        { kind: 'product', start: 0, end: 1, text: '1', children: [{ kind: 'number', start: 0, end: 1, text: '1', children: [] }] },
        { kind: 'product', start: 2, end: 5, text: '2*3', children: [
          { kind: 'number', start: 2, end: 3, text: '2', children: [] },
          { kind: 'number', start: 4, end: 5, text: '3', children: [] },
        ] },
      ],
    } }, { source: '12+3' }],
    rejects: [{ source: '1+', offset: 2 }, { source: '1**2', offset: 2 }],
  }, peggy);
  assert.deepEqual(result.failures, []);
});

test('independent replay refuses actions instead of dropping their meaning', () => {
  const original = importPest('root = { "a" }');
  const ast = peggy.generate(emitPeggy(original), { output: 'ast' });
  ast.rules[1].expression = { type: 'action', expression: ast.rules[1].expression, code: 'return 42;' };
  assert.throws(() => importIndependentPeggyAst(ast, original), /construct action has no native lowering/u);
});

test('a target that misreports its AST cannot pass the independent cycle', () => {
  const counterfeit = {
    VERSION: peggy.VERSION,
    generate(source, options) {
      const output = peggy.generate(source, options);
      if (options?.output === 'ast') output.rules[1].expression = { type: 'literal', value: 'counterfeit', ignoreCase: false };
      return output;
    },
  };
  const result = checkIndependentGrammarCycle(fixture.cases[0], counterfeit);
  assert.ok(result.failures.some((failure) => failure.includes('replay acceptance differs')));
});
