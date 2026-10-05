// Each parse has a hard memory budget (requirement I195-RESOURCE-PARSE-MEMORY-BUDGET):
// the memo cells it keeps, across its runs, repair rounds and embedded
// grammars, are counted against `memoryLimit`, and a parse that needs more
// stops with a `memoryBudget` rejection instead of growing until the process
// runs out of heap, as the formal-ai workloads did parsing 0.4 to 0.9 MB
// TypeScript files through the native default. The Rust twin is
// rust/tests/unit/issue_195_parse_memory_budget.rs.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { memoryBudget } from '../src/grammar-runtime/executor.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-RESOURCE-PARSE-MEMORY-BUDGET';
const here = path.dirname(fileURLToPath(import.meta.url));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: 'docs/vision.md',
    assertions,
    testName,
  });
}

const nativeParser = (id) => compileGrammar(parseGrammarLinks(nativeGrammarText(id)));

test('a parse that needs more memo cells than its budget is a memoryBudget rejection', () => {
  assert.equal(memoryBudget({}).limit, 2_000_000);
  const json = nativeParser('native-json');
  const source = `[${Array.from({ length: 200 }, (_, i) => `{"key${i}": [${i}, true, null]}`).join(', ')}]`;
  assert.equal(json.parseTree(source).ok, true);
  for (const options of [{}, { errorRecovery: true, recovery: 'accept' }]) {
    const outcome = json.parseTree(source, { ...options, memoryLimit: 500 });
    assert.deepEqual(outcome, { ok: false, tree: null, ambiguities: [], rejection: { reason: 'memoryBudget', limit: 500 } });
  }
  // Repair rounds share the one budget: invalid input does not get a new budget per round.
  const broken = source.replace('"key7": [7,', '"key7": [7,,');
  assert.equal(json.parseTree(broken, { errorRecovery: true, recovery: 'accept', memoryLimit: 500 }).rejection.reason, 'memoryBudget');
  assert.throws(() => json.parse(source, { memoryLimit: 500 }), {
    name: 'GrammarParseError', reason: 'memoryBudget', message: 'the parse needed more than 500 memo cells',
  });
  observe(['budgetExceededIsRejection', 'budgetSharedAcrossRepairRounds', 'rejectionNamesLimit'],
    'a parse that needs more memo cells than its budget is a memoryBudget rejection');
});

test('under a capped heap a large TypeScript parse ends with a diagnostic instead of exhausting the heap', () => {
  // About 33 kB of TypeScript keeps some 360000 memo cells, more than a
  // 128 MiB heap holds; the budget of 100000 cells stops the parse first.
  const child = `
    import { compileGrammar } from ${JSON.stringify(new URL('../src/grammar.js', import.meta.url).href)};
    import { parseGrammarLinks } from ${JSON.stringify(new URL('../src/grammar-links.js', import.meta.url).href)};
    import { nativeGrammarText } from ${JSON.stringify(new URL('../src/native-grammar-parser.js', import.meta.url).href)};
    const unit = (i) => \`export const table\${i}: Map<string, Array<number>> = new Map([["a", [1, 2, \${i}]]]);\\n\`;
    const source = Array.from({ length: 400 }, (_, i) => unit(i)).join('');
    const parser = compileGrammar(parseGrammarLinks(nativeGrammarText('native-typescript')));
    const outcome = parser.parseTree(source, { errorRecovery: true, recovery: 'accept', memoryLimit: 100000 });
    console.log(JSON.stringify({ rejection: outcome.rejection, heapMiB: process.memoryUsage().heapUsed / 2 ** 20 }));
  `;
  const run = spawnSync(process.execPath, ['--max-old-space-size=128', '--input-type=module', '-e', child], {
    cwd: here, encoding: 'utf8', timeout: 120_000,
  });
  assert.equal(run.status, 0, run.stderr);
  const { rejection, heapMiB } = JSON.parse(run.stdout);
  assert.deepEqual(rejection, { reason: 'memoryBudget', limit: 100000 });
  assert.ok(heapMiB < 128, `heap ${heapMiB} MiB`);
  observe(['cappedHeapParseEndsWithDiagnostic'],
    'under a capped heap a large TypeScript parse ends with a diagnostic instead of exhausting the heap');
});
