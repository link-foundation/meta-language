// CI runs the JavaScript tests as one job per group of scripts/test-groups.mjs.
// These checks keep every test file in exactly one group and the workflow
// running every group before the JavaScript Package check passes.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';

import { TEST_GROUPS, partitionGroupFiles, partitionProblems, partitionTests, testFiles } from '../scripts/test-groups.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

test('CI file partitions preserve every original native, conformance and translation file exactly once', () => {
  const groups = partitionTests(testFiles()).groups;
  for (const [name, count] of [['grammar', 3], ['grammar-native', 3], ['grammar-native', 6], ['conformance', 2], ['translation', 3]]) {
    const shards = Array.from({ length: count }, (_, index) => partitionGroupFiles(groups[name], `${index + 1}/${count}`));
    assert.ok(shards.every(files => files.length > 0));
    assert.equal(new Set(shards.flat()).size, groups[name].length);
    assert.deepEqual(shards.flat().sort(), groups[name]);
  }
  for (const shard of ['0/3', '4/3', '1/0', '1/999', '1.5/3', '1/2x']) assert.throws(() => partitionGroupFiles(['a', 'b', 'c'], shard), /test group shard/u);
});

test('six JavaScript native grammar partitions conserve each historical partition after the recorded automatic deadline', async () => {
  const path = 'parity/fixtures/javascript-native-grammar-deadline-2026-10-18.lino';
  const bytes = await readFile(new URL('../../' + path, import.meta.url));
  assert.equal(bytes.length, 188134);
  assert.equal(createHash('sha256').update(bytes).digest('hex'), '3b2db46a8d37f7a0f81f1095e9a0c377571ac06e68526d55345aa9726d4d5e8c');
  const capture = readProjectData(new URL('../../' + path, import.meta.url));
  assert.equal(capture.head, 'd41b44f26a9e5224a7c061d14cdc735ba4f1885f');
  assert.equal(capture.runId, 39125289901);
  assert.equal(capture.jobId, 117669022340);
  for (const response of Object.values(capture.responses)) {
    assert.equal(Buffer.byteLength(response.text), response.bytes);
    assert.equal(createHash('sha256').update(response.text).digest('hex'), response.sha256);
    assert.deepEqual(JSON.parse(response.text), response.data);
  }
  const job = capture.responses.job.data;
  assert.equal(job.id, capture.jobId);
  assert.equal(job.run_id, capture.runId);
  assert.equal(job.head_sha, capture.head);
  assert.equal(job.conclusion, 'cancelled');
  assert.ok(Date.parse(job.completed_at) - Date.parse(job.started_at) >= 900000);
  const deadline = capture.responses.annotations.data.filter(row => row.message.includes('maximum execution time'));
  assert.equal(deadline.length, 1);
  assert.equal(deadline[0].message, 'The job has exceeded the maximum execution time of 15m0s');
  assert.equal(deadline[0].annotation_level, 'failure');
  const original = execFileSync('git', ['show', capture.head + ':' + capture.workflow.path], {
    cwd: fileURLToPath(new URL('../../', import.meta.url)), maxBuffer: 128 * 1024,
  });
  assert.equal(original.length, capture.workflow.bytes);
  assert.equal(createHash('sha256').update(original).digest('hex'), capture.workflow.sha256);
  assert.equal(original.toString('utf8'), capture.workflow.text);
  const historical = original.toString('utf8').slice(original.indexOf('\n  tests:\n'), original.indexOf('\n  package-checks:\n'));
  assert.match(historical, /timeout-minutes: 15\n/u);
  assert.deepEqual([...historical.matchAll(/\{group: grammar-native, partition: (\d+), shards: (\d+)\}/gu)].map(([, index, count]) => [Number(index), Number(count)]), [[1, 3], [2, 3], [3, 3]]);
  const files = partitionTests(testFiles()).groups['grammar-native'];
  for (let index = 1; index <= 3; index += 1) {
    assert.deepEqual([...partitionGroupFiles(files, `${index}/6`), ...partitionGroupFiles(files, `${index + 3}/6`)].sort(),
      partitionGroupFiles(files, `${index}/3`));
  }
});

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

test('native corpora, recovery and scaling checks run independently with every assertion retained', () => {
  const partition = partitionTests([
    'grammar-native-field-expression-context.test.js',
    'issue-195-grammar-native-agda.test.js',
    'issue-195-grammar-native-zig.test.js',
    'issue-195-grammar-native-recovery.test.js',
    'error-recovery-parity.test.js',
    'parse-scaling.test.js',
    'lino-grammar-scaling.test.js',
    'issue-195-grammar-profile-ruby.test.js',
    'typed-program-constructor-references.test.js',
    'typed-program-pattern-bindings.test.js',
  ]);
  assert.deepEqual(partition.unassigned, []);
  assert.deepEqual(partition.ambiguous, []);
  assert.deepEqual(partition.groups.grammar, ['grammar-native-field-expression-context.test.js']);
  assert.deepEqual(partition.groups['grammar-native'], [
    'issue-195-grammar-native-agda.test.js', 'issue-195-grammar-native-zig.test.js',
  ]);
  assert.deepEqual(partition.groups['grammar-recovery'], [
    'issue-195-grammar-native-recovery.test.js', 'error-recovery-parity.test.js',
  ]);
  assert.deepEqual(partition.groups['grammar-scaling'], ['parse-scaling.test.js', 'lino-grammar-scaling.test.js']);
  assert.deepEqual(partition.groups['grammar-ruby'], ['issue-195-grammar-profile-ruby.test.js']);
  assert.deepEqual(partition.groups.translation, [
    'typed-program-constructor-references.test.js', 'typed-program-pattern-bindings.test.js',
  ]);
});

test('the JavaScript workflow runs every test group and gates the package check on them', async () => {
  const workflow = await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const tests = workflow.slice(workflow.indexOf('\n  tests:\n'), workflow.indexOf('\n  package-checks:\n'));
  const groups = tests.match(/group: \[([^\]]+)\]/u)[1].split(',').map((group) => group.trim());
  assert.deepEqual(groups, Object.keys(TEST_GROUPS).filter(group => group !== 'report-artifacts'));
  const reports = workflow.slice(workflow.indexOf('\n  report-assertions:\n'), workflow.indexOf('\n  tests:\n'));
  assert.match(reports, /needs: reports\n/u);
  assert.match(reports, /test-groups\.mjs --run report-artifacts\n/u);
  assert.deepEqual([...groups, 'report-artifacts'].sort(), Object.keys(TEST_GROUPS).sort());
  assert.match(tests, /timeout-minutes: 15\n/u);
  assert.match(tests, /fail-fast: false\n/u);
  assert.match(tests, /partition: \[1\]\n/u);
  for (const [group, count] of [['grammar', 3], ['grammar-native', 6], ['conformance', 2], ['translation', 3]]) {
    for (let partition = 1; partition <= count; partition += 1) {
      assert.ok(tests.includes(`{group: ${group}, partition: ${partition}, shards: ${count}}`));
    }
  }
  assert.ok(tests.includes('TEST_GROUP_SHARD: ${{ matrix.partition }}/${{ matrix.shards || 1 }}'));
  assert.ok(tests.includes('javascript-tests-${{ matrix.group }}-${{ matrix.partition }}'));
  assert.match(
    tests,
    /run: node \.\.\/scripts\/with-cache-cleanup\.mjs --event test -- node scripts\/test-groups\.mjs --run "\$TEST_GROUP"\n/u,
  );
  const packageJob = workflow.slice(workflow.indexOf('\n  test:\n'), workflow.indexOf('\n  publish:\n'));
  assert.match(packageJob, /name: JavaScript Package\n    needs: \[tests, report-assertions, package-checks, translation-test-feedback\]\n    if: \$\{\{ !cancelled\(\) \}\}\n/u);
  assert.match(packageJob, /PACKAGE_CHECKS_RESULT: \$\{\{ needs\.package-checks\.result \}\}\n/u);
  assert.match(packageJob, /if \[ "\$PACKAGE_CHECKS_RESULT" != "success" \]; then/u);
  assert.match(packageJob, /TRANSLATION_FEEDBACK_RESULT: \$\{\{ needs.translation-test-feedback.result \}\}\n/u);
  assert.match(packageJob, /if \[ "\$TRANSLATION_FEEDBACK_RESULT" != "success" \]; then/u);
  assert.match(packageJob, /TESTS_RESULT: \$\{\{ needs\.tests\.result \}\}\n/u);
  assert.match(packageJob, /if \[ "\$TESTS_RESULT" != "success" \]; then/u);
  assert.match(packageJob, /REPORTS_RESULT: \$\{\{ needs\.report-assertions\.result \}\}\n/u);
  assert.match(packageJob, /if \[ "\$EXTERNAL_REPORTS" != "true" \] && \[ "\$REPORTS_RESULT" != "success" \]; then/u);
  assert.match(packageJob, /run: node scripts\/test-groups\.mjs --check\n/u);
  assert.doesNotMatch(packageJob, /-- npm test\n/u);
});

test('CI report assertions consume every exact-commit shard and gate Rust and delivery', async () => {
  const workflow = await readFile(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const parsed = new Map((await import('../../scripts/check-cache-policy.mjs')).parseWorkflow(workflow).map(job => [job.id, job]));
  const required = job => /^ {4}needs:\s*(.+)$/mu.exec(job.header)?.[1].replace(/^\[|\]$/gu, '').split(',').map(value => value.trim()) ?? [];
  const assertions = parsed.get('self-translation-report-validation');
  assert.deepEqual(required(assertions), ['self-translation-report']);
  assert.match(assertions.header, /timeout-minutes: 15/u);
  assert.match(assertions.header, /!cancelled\(\)/u);
  const download = assertions.steps.findIndex(step => step.name === 'Download the complete self-translation reports');
  const run = assertions.steps.findIndex(step => /test-groups\.mjs --run report-artifacts/u.test(step.run));
  assert.ok(download >= 0 && run > download);
  assert.match(assertions.steps[download].text, /pattern: self-translation-report-\$\{\{ github.sha \}\}-\*/u);
  assert.doesNotMatch(assertions.steps[download].text, /merge-multiple: true/u);
  assert.match(assertions.steps[run].text, /ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY:/u);
  for (const id of ['self-translation-report', 'self-translation-report-validation']) {
    const checkout = parsed.get(id).steps.find(step => /actions\/checkout@/u.test(step.text));
    assert.match(checkout.text, /ref: \$\{\{ github.sha \}\}/u);
    assert.ok(!parsed.get(id).steps.some(step => /simulate-fresh-merge/u.test(step.run)), id);
  }
  for (const name of ['rust', 'candidates', 'runtime-parity', 'acceptance']) {
    assert.ok(required(parsed.get(name)).includes('self-translation-report-validation'), name);
  }
  assert.match(parsed.get('js').header, /external_reports: true/u);
  assert.doesNotMatch(parsed.get('rust').header, /always\(\)|cancelled\(\)/u);
  const seen = new Set();
  const active = new Set();
  const visit = id => {
    assert.ok(!active.has(id), `workflow dependency cycle at ${id}`);
    if (seen.has(id)) return;
    active.add(id);
    required(parsed.get(id)).forEach(visit);
    active.delete(id);
    seen.add(id);
  };
  [...parsed.keys()].forEach(visit);
});

test('standalone JavaScript runs produce and validate the same report shards', async () => {
  const workflow = await readFile(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const parsed = new Map((await import('../../scripts/check-cache-policy.mjs')).parseWorkflow(workflow).map(job => [job.id, job]));
  const producers = parsed.get('reports');
  const assertions = parsed.get('report-assertions');
  assert.match(workflow, /external_reports:\n(?:[^\n]*\n)*? {8}default: false\n/u);
  assert.match(producers.header, /!inputs.external_reports/u);
  assert.match(producers.header, /fail-fast: false/u);
  assert.match(producers.header, /shard: \[1, 2, 3, 4, 5, 6, 7, 8\]/u);
  assert.match(producers.steps.map(step => step.text).join('\n'), /--shard \$\{\{ matrix.shard \}\}\/8/u);
  assert.match(producers.steps.map(step => step.text).join('\n'), /name: self-translation-report-\$\{\{ github.sha \}\}-\$\{\{ matrix.shard \}\}/u);
  assert.match(assertions.header, /needs: reports/u);
  assert.match(assertions.header, /!cancelled\(\) && !inputs.external_reports/u);
  for (const job of [producers, assertions]) {
    const checkout = job.steps.find(step => /actions\/checkout@/u.test(step.text));
    assert.match(checkout.text, /ref: \$\{\{ github.sha \}\}/u);
    assert.ok(!job.steps.some(step => /simulate-fresh-merge/u.test(step.run)));
  }
  const download = assertions.steps.find(step => /actions\/download-artifact@/u.test(step.text));
  assert.match(download.text, /pattern: self-translation-report-\$\{\{ github.sha \}\}-\*/u);
  assert.doesNotMatch(download.text, /merge-multiple: true/u);
});
