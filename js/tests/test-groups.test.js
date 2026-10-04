// CI runs the JavaScript tests as one job per group of scripts/test-groups.mjs.
// These checks keep every test file in exactly one group and the workflow
// running every group before the JavaScript Package check passes.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { TEST_GROUPS, partitionProblems, partitionTests, testFiles } from '../scripts/test-groups.mjs';

test('every JavaScript test file is in exactly one test group', () => {
  const files = testFiles();
  const partition = partitionTests(files);
  assert.deepEqual(partitionProblems(partition), []);
  assert.equal(Object.values(partition.groups).flat().length, files.length);
});

test('the test group guard reports unassigned, ambiguous and empty groups', () => {
  const problems = partitionProblems(partitionTests(['grammar-a.test.js', 'unknown.test.js']));
  assert.ok(problems.includes('tests/unknown.test.js is in no test group of scripts/test-groups.mjs'));
  for (const group of Object.keys(TEST_GROUPS).filter((group) => group !== 'grammar')) {
    assert.ok(problems.includes(`test group ${group} has no test files`), group);
  }
  const overlapping = partitionTests(['x.test.js']);
  overlapping.ambiguous.push('x.test.js (grammar, tooling)');
  assert.ok(partitionProblems(overlapping).includes('tests/x.test.js (grammar, tooling) is in more than one test group'));
});

test('the JavaScript workflow runs every test group and gates the package check on them', async () => {
  const workflow = await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const tests = workflow.slice(workflow.indexOf('\n  tests:\n'), workflow.indexOf('\n  test:\n'));
  const groups = tests.match(/group: \[([^\]]+)\]/u)[1].split(',').map((group) => group.trim());
  assert.deepEqual(groups, Object.keys(TEST_GROUPS));
  assert.match(tests, /timeout-minutes: 15\n/u);
  assert.match(tests, /fail-fast: false\n/u);
  assert.match(
    tests,
    /run: node \.\.\/scripts\/with-cache-cleanup\.mjs --event test -- node scripts\/test-groups\.mjs --run "\$TEST_GROUP"\n/u,
  );
  const packageJob = workflow.slice(workflow.indexOf('\n  test:\n'), workflow.indexOf('\n  publish:\n'));
  assert.match(packageJob, /name: JavaScript Package\n    needs: tests\n    if: \$\{\{ !cancelled\(\) \}\}\n/u);
  assert.match(packageJob, /TESTS_RESULT: \$\{\{ needs\.tests\.result \}\}\n/u);
  assert.match(packageJob, /if \[ "\$TESTS_RESULT" != "success" \]; then/u);
  assert.match(packageJob, /run: node scripts\/test-groups\.mjs --check\n/u);
  assert.doesNotMatch(packageJob, /-- npm test\n/u);
});
