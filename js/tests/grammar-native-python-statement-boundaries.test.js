import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { Executor, stepBudget } from '../src/grammar-runtime/executor.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

const listing = nativeGrammarText('native-python');
const parser = compileGrammar(parseGrammarLinks(listing));
const projection = { ...readProjectData(new URL('../../parity/fixtures/native-grammars/python.lino', import.meta.url)),
  oracleKinds: nativeOracleKinds(listing) };
const options = { errorRecovery: true, recovery: 'accept' };

// Keep the exact synthetic unit used by the original native scaling guard.
function pythonUnit(index) {
  return `def item_${index}(value):\n    """Item ${index}."""\n    total = value + ${index}\n    if total > ${index}:\n        return total\n    return ${index}\n\n`;
}

function reconstructPythonSource(tree) {
  if (tree.type === 'node') return tree.children.map(reconstructPythonSource).join('');
  assert.equal(tree.type, 'token', 'valid source contains neither ERROR nor MISSING leaves');
  return tree.text;
}

function assertPythonStatementBoundaries(rows, source, expected) {
  const functions = rows.filter(row => row[2] === 'function_definition');
  assert.equal(functions.length, expected.length, 'each authored function remains a distinct root statement');
  const scope = [];
  const actual = new Map();
  let previousEnd = 0;
  for (const row of rows) {
    scope.length = row[0];
    if (row[2] === 'function_definition') {
      assert.equal(row[0], 1, 'a later top-level function is never swallowed by an earlier block');
      const name = source.slice(row[4]).match(/^def ([A-Za-z_][A-Za-z_0-9]*)\(/u)?.[1];
      const next = expected[actual.size];
      assert.equal(name, next.name);
      assert.equal(row[4], source.indexOf(`def ${name}(`));
      assert.ok(row[4] >= previousEnd, 'top-level function spans are ordered and disjoint');
      previousEnd = row[5];
      actual.set(name, { blocks: 0, conditionals: 0 });
    }
    if (row[2] === 'block' || row[2] === 'if_statement') {
      const owner = scope.findLast(parent => parent[2] === 'function_definition');
      assert.ok(owner, 'every authored block and conditional belongs to its own function');
      assert.ok(row[4] >= owner[4] && row[5] <= owner[5], 'nested spans stay within their function');
      const enclosingBlock = scope.findLast(parent => parent[2] === 'block');
      if (enclosingBlock) assert.ok(row[4] >= enclosingBlock[4] && row[5] <= enclosingBlock[5],
        'an inner block closes before its enclosing block');
      const name = source.slice(owner[4]).match(/^def ([A-Za-z_][A-Za-z_0-9]*)\(/u)[1];
      actual.get(name)[row[2] === 'block' ? 'blocks' : 'conditionals']++;
    }
    scope[row[0]] = row;
  }
  assert.deepEqual([...actual].map(([name, counts]) => ({ name, ...counts })), expected,
    'the complete authored nested block and conditional structure remains explicit');
}

const pythonStatementCases = [
  { name: 'two original nested function units', source: pythonUnit(0) + pythonUnit(1),
    functions: [{ name: 'item_0', blocks: 2, conditionals: 1 }, { name: 'item_1', blocks: 2, conditionals: 1 }] },
  { name: 'two nested blocks then a top-level function',
    source: 'def outer(value):\n    if value:\n        if value > 1:\n            return value\n        return 1\n    return 0\n\ndef after(value):\n    return value\n',
    functions: [{ name: 'outer', blocks: 3, conditionals: 2 }, { name: 'after', blocks: 1, conditionals: 0 }] },
  { name: 'comments at nested function and module indentation',
    source: 'def first(value):\n    if value:\n        return value\n        # nested tail\n    # function body\n    return 0\n    # function tail\n\n# module boundary\ndef second(value):\n    return value\n',
    functions: [{ name: 'first', blocks: 2, conditionals: 1 }, { name: 'second', blocks: 1, conditionals: 0 }] },
  { name: 'blank lines and two dedents at EOF without a final newline',
    source: 'def first():\n    if True:\n        return 1\n\n    return 2',
    functions: [{ name: 'first', blocks: 2, conditionals: 1 }] },
  { name: 'parenthesized continuation keeps its internal indentation',
    source: 'def first():\n    return (\n      1 +\n      2\n    )\ndef second():\n    return 3\n',
    functions: [{ name: 'first', blocks: 1, conditionals: 0 }, { name: 'second', blocks: 1, conditionals: 0 }] },
  { name: 'triple-quoted content may contain dedented physical lines',
    source: 'def first():\n    """first\nunindented content\n    final"""\n    return 1\ndef second():\n    return 2\n',
    functions: [{ name: 'first', blocks: 1, conditionals: 0 }, { name: 'second', blocks: 1, conditionals: 0 }] },
];

for (const { name, source, functions } of pythonStatementCases) test(`Python statement boundaries preserve ${name}`, () => {
  const oracle = oracleSnapshot(source, 'Python');
  assert.equal(oracle.recovers, false, 'the pinned independent oracle accepts without recovery');
  const outcome = parser.parseTree(source, options);
  assert.equal(outcome.ok, true);
  assert.equal(outcome.rejection, null);
  assert.deepEqual(outcome.ambiguities, []);
  assert.equal(reconstructPythonSource(outcome.tree), source);
  const rows = nativeRows(outcome.tree, source, projection);
  assert.deepEqual(rows, oracle.rows, 'every concrete row, flag, field and byte span matches the independent oracle');
  assertPythonStatementBoundaries(rows, source, functions);
});

test('Python statement repetition retains bounded candidates and proportional work for sixteen and thirty-two original units', () => {
  const originalBody = Executor.prototype.ruleBody;
  const originalRun = Executor.prototype.run;
  const measured = new Set(['function_definition', 'suite', 'block']);
  const samples = [];
  let current = null;
  try {
    Executor.prototype.ruleBody = function(rule, position, ...args) {
      const results = originalBody.call(this, rule, position, ...args);
      if (current && measured.has(rule.name)) {
        const entry = current.rules[rule.name] ??= { calls: 0, maxResults: 0, maxSpan: 0 };
        entry.calls++;
        entry.maxResults = Math.max(entry.maxResults, results.length);
        for (const result of results) entry.maxSpan = Math.max(entry.maxSpan, result.end - position);
      }
      return results;
    };
    Executor.prototype.run = function(...args) {
      try { return originalRun.apply(this, args); }
      finally {
        if (current) current.rounds.push({ steps: this.budget.steps, limit: this.budget.limit,
          repairPoints: this.repairPoints?.size ?? null });
      }
    };
    for (const units of [16, 32]) {
      const pieces = Array.from({ length: units }, (_, index) => pythonUnit(index));
      const source = pieces.join('');
      assert.equal(Buffer.byteLength(source), units === 16 ? 1822 : 3694, 'the original synthetic input is unchanged');
      current = { units, rules: {}, rounds: [] };
      const outcome = parser.parseTree(source, options);
      assert.equal(outcome.ok, true);
      assert.equal(outcome.rejection, null);
      assert.deepEqual(outcome.ambiguities, []);
      assert.equal(reconstructPythonSource(outcome.tree), source);
      const oracle = oracleSnapshot(source, 'Python');
      assert.equal(oracle.recovers, false);
      const rows = nativeRows(outcome.tree, source, projection);
      assert.deepEqual(rows, oracle.rows);
      assertPythonStatementBoundaries(rows, source, pieces.map((_, index) => ({ name: `item_${index}`, blocks: 2, conditionals: 1 })));
      assert.ok(current.rounds.length > 0, 'the actual native grammar executor ran');
      for (const round of current.rounds) {
        assert.equal(round.limit, stepBudget(options, Buffer.byteLength(source)).limit, 'the original step budget is unchanged');
        assert.equal(round.repairPoints, 0, 'valid input never enters a repair round');
        assert.ok(round.steps > 0 && round.steps <= round.limit);
      }
      for (const name of measured) {
        const entry = current.rules[name];
        assert.ok(entry?.calls > 0, `${name}: actual rule executions are observed`);
        assert.equal(entry.maxResults, 1, `${name}: a block boundary produces one candidate, not every following function suffix`);
        assert.ok(entry.maxSpan <= Math.max(...pieces.map(piece => Buffer.byteLength(piece))),
          `${name}: no candidate spans a later function`);
      }
      samples.push({ ...current, steps: current.rounds.reduce((sum, round) => sum + round.steps, 0) });
      current = null;
    }
  } finally {
    Executor.prototype.ruleBody = originalBody;
    Executor.prototype.run = originalRun;
  }
  const [small, large] = samples;
  assert.ok(large.steps <= small.steps * 2.5, 'doubling original units preserves proportional charged work instead of quadratic suffix exploration');
  for (const name of measured) assert.ok(large.rules[name].calls <= small.rules[name].calls * 2 + 1,
    `${name}: doubling units retains proportional rule-body executions`);
});
