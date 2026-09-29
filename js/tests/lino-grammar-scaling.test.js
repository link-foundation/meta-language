// Stack-safety and complexity guard for the built-in LiNo grammar, the
// counterpart of rust/tests/unit/lino_grammar_scaling.rs. Each input below made
// an earlier Links Notation parser overflow the stack or read the document in
// superlinear time (the regressions relative-meta-logic tracks against
// links-notation): deep parentheses, long runs of unclosed groups or quotes,
// deep indentation, long delimiter runs and many failing lines. Every input is
// parsed at two sizes and the cost per byte compared: a linear parse keeps it
// flat and a quadratic one multiplies it by the size ratio.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';
import { LINO_MAX_DEPTH, parseLinoCst } from '../src/lino-grammar.js';

// The large input holds SIZE_RATIO times as many units as the small one.
const SIZE_RATIO = 8;
// Linear parsing lands near 1, quadratic near SIZE_RATIO.
const MAX_PER_BYTE_GROWTH = 3;
const SAMPLES = 3;
const ATTEMPTS = 3;

const LINO_SCALING_CASES = [
  { name: 'deep parentheses', units: 500, source: (n) => `${'('.repeat(n)}a${')'.repeat(n)}\n` },
  { name: 'unclosed groups', units: 500, source: (n) => `${'(a '.repeat(n)}\n` },
  {
    name: 'deep indentation',
    units: 500,
    source: (n) => `${Array.from({ length: n }, (_, depth) => `${' '.repeat(depth % (LINO_MAX_DEPTH - 4))}a`).join('\n')}\n`,
  },
  { name: 'indented identifiers', units: 250, source: (n) => 'id:\n  a\n  b\n    c\n'.repeat(n) },
  { name: 'long delimiter run', units: 2000, source: (n) => `${'"'.repeat(n)}x${'"'.repeat(n)}\n` },
  { name: 'unclosed quotes', units: 500, source: (n) => `${'"a '.repeat(n)}\n` },
  { name: 'comments and groups', units: 250, source: (n) => 'a: b c # note\n  child (x y)\n'.repeat(n) },
  { name: 'failing indented lines', units: 100, source: (n) => `x\n${'  (broken\n'.repeat(n)}` },
];

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

for (const { name, units, source } of LINO_SCALING_CASES) {
  test(`the LiNo grammar reads ${name} without deep recursion and in linear time`, () => {
    const small = source(units);
    const large = source(units * SIZE_RATIO);
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
  });
}

test('links nested deeper than the official limit become an ERROR, not a stack overflow', () => {
  const tooDeep = `${'('.repeat(100_000)}a${')'.repeat(100_000)}\n`;
  const tree = parseLinoCst(tooDeep);
  assert.ok(tree.children.some((child) => child.term === 'ERROR'));
  const deepest = `${'('.repeat(LINO_MAX_DEPTH)}a${')'.repeat(LINO_MAX_DEPTH)}\n`;
  assert.ok(!parseLinoCst(deepest).children.some((child) => child.term === 'ERROR'));
});
