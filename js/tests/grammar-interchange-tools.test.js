import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import peggy from 'peggy';
import { checkIndependentGrammarCycle, importIndependentPeggyAst } from '../scripts/grammar-interchange-tools.mjs';
import { importPest } from '../src/grammar-importers.js';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { compileGrammar, emitPeggy, Grammar } from '../src/grammar.js';
import { GrammarParseError } from '../src/grammar-runtime.js';
import { grammarToolOperationFailure, isGrammarToolSyntaxRejection, nativeGrammarToolOutcome, targetGrammarToolOutcome } from '../scripts/grammar-tool-outcomes.mjs';
import { selectPeggyOptimizerAssertion } from '../scripts/peggy-version-assertions.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = readProjectData(new URL('../../parity/fixtures/grammar-interchange-tools.json', import.meta.url));

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

test('independent optimized Peggy character sets retain exact source lexer bindings and byte diagnostics', (context) => {
  const versionApplicability = [selectPeggyOptimizerAssertion({ runtimeVersion: peggy.VERSION })];
  for (const [alternatives, first, second, rejectedOffset] of [
    ["'是' | '否'", '是', '否', 4],
    ["'a' | 'b' | 'c'", 'a', 'c', 2],
    ["'😀' | '😁'", '😀', '😁', 5],
  ]) {
    const original = importAntlr(`grammar Message; message: word ' ' word; word: ${alternatives};`);
    const ast = peggy.generate(emitPeggy(original), { output: 'ast' });
    const word = ast.rules[original.ruleNames().indexOf('word') + 1].expression;
    assert.equal(word.type, 'class', 'the independent optimizer actually merged the original literal alternatives');
    const replay = importIndependentPeggyAst(ast, original);
    assert.strictEqual(replay.declarations.lexers, original.declarations.lexers, 'complete original lexical clauses are retained');
    const parser = compileGrammar(replay);
    const text = `${first} ${second}`;
    const accepted = parser.parseTree(text);
    assert.equal(accepted.ok, true, text);
    assert.equal(accepted.tree.end, Buffer.byteLength(text));
    assert.deepEqual(accepted.tree.children.filter(({ type }) => type === 'node').map(({ kind, start, end }) => [kind, start, end]), [
      ['word', 0, Buffer.byteLength(first)], ['word', rejectedOffset, Buffer.byteLength(text)],
    ]);
    const rejected = parser.parseTree(`${first} x`);
    assert.equal(rejected.ok, false);
    assert.equal(rejected.rejection.offset, rejectedOffset);
  }
  recordIssue195Observations({
    requirementId: 'I195-ASSERTION-EXPLICIT-VERSION-RANGES', suffix: 'peggy-character-set-shape', runtime: 'tooling',
    fixtureId: 'assertion-version-peggy-character-set-shape-source', fixtureFile: 'js/tests/grammar-interchange-tools.test.js',
    assertions: ['peggyOptimizedCharacterSetsHaveApplicableVersionContext'], testName: context.name, versionApplicability,
  });
});

test('independent optimized character set replay refuses extra members and incompatible source contexts', (context) => {
  const versionApplicability = [selectPeggyOptimizerAssertion({ runtimeVersion: peggy.VERSION })];
  const original = importAntlr(`grammar Message; message: word ' ' word; word: '是' | '否';`);
  for (const change of [
    (node) => { node.parts.push('x'); },
    (node) => { node.parts = [['一', '龟']]; },
    (node) => { node.inverted = true; },
    (node) => { node.ignoreCase = true; },
    (node) => { node.parts = [['\ud800', '\udfff']]; },
  ]) {
    const ast = peggy.generate(emitPeggy(original), { output: 'ast' });
    const word = ast.rules[original.ruleNames().indexOf('word') + 1].expression;
    assert.equal(word.type, 'class');
    change(word);
    assert.throws(() => importIndependentPeggyAst(ast, original), /target character class|source lexical/u);
  }
  const ambiguous = importAntlr(`grammar Message; message: word ' ' word; word: '是' | '否';`);
  ambiguous.declarations.lexers.push({ ...ambiguous.declarations.lexers[0], name: 'Other' });
  const ast = peggy.generate(emitPeggy(ambiguous), { output: 'ast' });
  assert.throws(() => importIndependentPeggyAst(ast, ambiguous), /multiple source lexical owners/u);
  recordIssue195Observations({
    requirementId: 'I195-ASSERTION-EXPLICIT-VERSION-RANGES', suffix: 'peggy-character-set-shape', runtime: 'tooling',
    fixtureId: 'assertion-version-peggy-character-set-shape-source', fixtureFile: 'js/tests/grammar-interchange-tools.test.js',
    assertions: ['peggyCharacterSetRefusalsHaveApplicableVersionContext'], testName: context.name, versionApplicability,
  });
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

test('an edited independent literal extends the shared lexer pool and retains original source clauses', () => {
  const original = importAntlr("grammar Message; message: 'before';");
  const source = emitPeggy(original).replace('"before"', '"after"');
  const replay = importIndependentPeggyAst(peggy.generate(source, { output: 'ast' }), original);
  const lexer = replay.declarations.lexers[0];
  const originalLexer = original.declarations.lexers[0];
  assert.notStrictEqual(lexer, originalLexer);
  assert.strictEqual(lexer.expression.items.find(({ kind }) => kind === 'lexicalOwner'),
    originalLexer.expression.items.find(({ kind }) => kind === 'lexicalOwner'));
  assert.deepEqual(lexer.expression.items.filter(({ kind }) => kind === 'lexicalLiteral').map(({ value }) => value), ['after', 'before']);
  const parser = compileGrammar(replay);
  assert.equal(parser.parseTree('after').ok, true);
  const rejected = parser.parseTree('before');
  assert.equal(rejected.ok, false);
  assert.equal(rejected.rejection.reason, 'syntax');
  assert.equal(rejected.rejection.offset, 0);
  const independent = peggy.generate(source);
  assert.equal(targetGrammarToolOutcome(independent, 'after').accepted, true);
  assert.equal(isGrammarToolSyntaxRejection(targetGrammarToolOutcome(independent, 'before')), true);
  assert.equal(originalLexer.expression.items.some(({ value }) => value === 'after'), false, 'source provenance was not mutated');
});

test('independent tool outcomes preserve operation causes and refuse resource failures as syntax evidence', () => {
  const cause = new Error('original operation cause');
  const failure = new TypeError('target action failed', { cause });
  const operation = targetGrammarToolOutcome({ parse() { throw failure; } }, 'é');
  assert.equal(operation.accepted, null);
  assert.equal(operation.error.name, 'TypeError');
  assert.equal(operation.error.message, failure.message);
  assert.equal(operation.error.stack, failure.stack);
  assert.equal(operation.error.cause.message, cause.message);
  assert.equal(operation.error.cause.stack, cause.stack);
  assert.equal('offset' in operation, false);
  assert.equal('expected' in operation, false);
  assert.equal(isGrammarToolSyntaxRejection(operation), false);
  class SyntaxError extends Error {}
  const syntax = new SyntaxError('actual target syntax');
  syntax.location = { start: { offset: 1 } };
  const parser = { SyntaxError, parse() { throw syntax; } };
  assert.deepEqual(targetGrammarToolOutcome(parser, 'éx'), { accepted: false, offset: 2, reason: 'syntax' });
  syntax.location.start.offset = 9;
  assert.equal(targetGrammarToolOutcome(parser, 'éx').accepted, null, 'a missing actual source location is an operation refusal');
  const original = importPest('root = { "a" }');
  const resource = nativeGrammarToolOutcome(original, 'a', () => { throw new GrammarParseError({ reason: 'stepLimit', limit: 1 }); });
  assert.deepEqual(resource, { accepted: false, offset: null, reason: 'stepLimit', expected: [] });
  assert.equal(isGrammarToolSyntaxRejection(resource), false);
  const invalid = new Grammar('root', new Map([['root', { name: 'root', expression: { kind: 'ref', name: 'missing' } }]]));
  const refused = nativeGrammarToolOutcome(invalid, 'a', (tree) => tree);
  assert.equal(refused.accepted, null);
  assert.equal(refused.error.name, 'GrammarRuntimeError');
  assert.equal(refused.error.reason, 'reference');
  assert.match(refused.error.message, /missing/u);
  assert.equal(isGrammarToolSyntaxRejection(refused), false);
  assert.equal(grammarToolOperationFailure(undefined).error.thrownType, 'undefined');
});

test('a target operation failure cannot satisfy the independent mutation witness', () => {
  const failure = new TypeError('deliberate target operation refusal', { cause: new Error('underlying target cause') });
  const counterfeit = {
    VERSION: peggy.VERSION,
    generate(source, options) {
      const output = peggy.generate(source, options);
      if (options?.output !== 'ast' && source.includes('"independent-target-mutation"')) {
        return { ...output, parse() { throw failure; } };
      }
      return output;
    },
  };
  const result = checkIndependentGrammarCycle(fixture.cases[0], counterfeit);
  assert.equal(result.mutation.originalWitnessRejectedByTarget, false);
  assert.equal(result.mutation.target.accepted, null);
  assert.equal(result.mutation.target.error.message, failure.message);
  assert.equal(result.mutation.target.error.cause.message, failure.cause.message);
  assert.equal(result.mutation.originalWitnessRejectedByNativeReplay, true);
  assert.ok(result.failures.includes('the target mutation did not change both executions'));
});
