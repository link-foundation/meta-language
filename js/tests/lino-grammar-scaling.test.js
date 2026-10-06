// Stack-safety and complexity guard for the built-in LiNo grammar, the
// counterpart of rust/tests/unit/lino_grammar_scaling.rs. Each regression of
// parity/fixtures/lino-compatibility-matrix.json made an earlier Links Notation
// parser overflow the stack or read the document in superlinear time (the
// findings relative-meta-logic reported against links-notation): lone carriage
// returns, deep parentheses, long runs of unclosed groups or quotes, deep
// indentation, long delimiter runs and many failing lines. Every input is
// parsed at two sizes and the cost per byte compared: a linear parse keeps it
// flat and a quadratic one multiplies it by the size ratio.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';
import { LINO_MAX_DEPTH, parseLinoCst } from '../src/lino-grammar.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const MATRIX_FILE = 'parity/fixtures/lino-compatibility-matrix.json';
const matrix = JSON.parse(readFileSync(new URL(`../../${MATRIX_FILE}`, import.meta.url), 'utf8'));

// The large input holds SIZE_RATIO times as many units as the small one.
const SIZE_RATIO = 8;
// Linear parsing lands near 1, quadratic near SIZE_RATIO.
const MAX_PER_BYTE_GROWTH = 3;
const SAMPLES = 3;
const ATTEMPTS = 3;

// The source of a matrix input shape holding `units` units.
function shapeSource(shape, units) {
  switch (shape.kind) {
    case 'repeat': return `${shape.head}${shape.unit.repeat(units)}${shape.tail}`;
    case 'nest': return `${shape.open.repeat(units)}${shape.middle}${shape.close.repeat(units)}${shape.tail}`;
    case 'staircase':
      return `${Array.from({ length: units }, (_, depth) => `${' '.repeat(depth % shape.modulo)}${shape.text}`).join('\n')}\n`;
    default: throw new Error(`unknown input shape ${shape.kind}`);
  }
}

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-LINO-UPSTREAM-REGRESSIONS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-lino-upstream-regressions',
    fixtureFile: MATRIX_FILE,
    assertions,
    testName,
  });
}

// The whole LiNo parse: the grammar CST and the links-notation semantics.
function nanosPerByte(source) {
  let best = Infinity;
  for (let sample = 0; sample < SAMPLES; sample += 1) {
    const started = process.hrtime.bigint();
    const network = LinkNetwork.parse(source, 'LiNo');
    best = Math.min(best, Number(process.hrtime.bigint() - started));
    assert.equal(network.reconstructText(), source, 'the parse stays lossless');
  }
  return best / source.length;
}

// The regressions whose lossless and linear checks held.
const passedRegressions = new Set();

for (const { id, name, units, shape } of matrix.regressions) {
  test(`the LiNo grammar reads ${name} without deep recursion and in linear time`, () => {
    const small = shapeSource(shape, units);
    const large = shapeSource(shape, units * SIZE_RATIO);
    nanosPerByte(small); // warm up the JIT
    const reports = [];
    const passed = Array.from({ length: ATTEMPTS }).some(() => {
      const smallCost = nanosPerByte(small);
      const largeCost = nanosPerByte(large);
      const growth = largeCost / smallCost;
      reports.push(`${smallCost.toFixed(0)} ns/char at ${small.length} chars, ${largeCost.toFixed(0)} ns/char at ${large.length} chars, growth ${growth.toFixed(2)}x`);
      return growth <= MAX_PER_BYTE_GROWTH;
    });
    assert.ok(passed, `${name}: per-character parse cost grew more than ${MAX_PER_BYTE_GROWTH}x:\n  ${reports.join('\n  ')}`);
    passedRegressions.add(id);
  });
}

test('every upstream regression input is read losslessly and in linear time', () => {
  // The tests of a file run in order, so every regression test has finished here.
  assert.deepEqual([...passedRegressions], matrix.regressions.map(({ id }) => id));
  observe(['everyRegressionInputLossless', 'everyRegressionInputLinear'],
    'every upstream regression input is read losslessly and in linear time');
});

test('links nested deeper than the official limit become an ERROR, not a stack overflow', () => {
  assert.equal(matrix.maximumDepth, LINO_MAX_DEPTH);
  const { shape, units } = matrix.excessiveNesting;
  const tooDeep = shapeSource(shape, units);
  assert.ok(parseLinoCst(tooDeep).children.some((child) => child.term === 'ERROR'));
  assert.equal(LinkNetwork.parse(tooDeep, 'LiNo').reconstructText(), tooDeep);
  const deepest = shapeSource(shape, LINO_MAX_DEPTH);
  assert.ok(!parseLinoCst(deepest).children.some((child) => child.term === 'ERROR'));
  observe(['excessiveNestingIsErrorNode'], 'links nested deeper than the official limit become an ERROR, not a stack overflow');
});
