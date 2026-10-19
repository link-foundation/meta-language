import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { javascriptSuitePartition, javascriptSuiteTestGroups } from '../scripts/issue-195-javascript-suite-partitions.mjs';
import { partitionTests, testFiles } from '../scripts/test-groups.mjs';

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const workflow = restoreVerificationConcurrencyHistory(read('.github/workflows/ci.yml'), 'ci.yml');
const matrixEntries = job => [...job.header.matchAll(/- group: ([a-z-]+)\n\s+shard: ([0-9]+\/[0-9]+)\n\s+partition: ([0-9]+-of-[0-9]+)/gu)]
  .map(([, group, shard, partition]) => ({ group, shard, partition }));
const needs = job => /^ {4}needs: (.+)$/mu.exec(job.header)?.[1]
  .replace(/^\[|\]$/gu, '').split(',').map(value => value.trim()) ?? [];
const condition = job => /^ {4}if: (.+)$/mu.exec(job.header)?.[1];
const expected = Object.keys(javascriptSuiteTestGroups()).flatMap(group => {
  const count = group === 'grammar-native' ? 3 : group === 'conformance' ? 2 : 1;
  return Array.from({ length: count }, (_, index) => ({
    group, shard: `${index + 1}/${count}`, partition: `${index + 1}-of-${count}`,
  }));
});

function assertEvidenceDagConservation(text) {
  const jobs = new Map(parseWorkflow(text).map(job => [job.id, job]));
  const early = jobs.get('javascript-suite-independent');
  const downstream = jobs.get('javascript-suite');
  const rust = jobs.get('rust-suite');
  const aggregate = jobs.get('acceptance');
  assert.ok(early && downstream && rust && aggregate, 'both JavaScript collections, Rust and aggregate exist');
  assert.equal(expected.length, 15, 'the original fifteen physical scopes remain');
  assert.deepEqual(matrixEntries(early), expected.filter(({ group }) => group !== 'downstream'));
  assert.deepEqual(matrixEntries(downstream), expected.filter(({ group }) => group === 'downstream'));

  const scopes = [...matrixEntries(early), ...matrixEntries(downstream)];
  const groups = javascriptSuiteTestGroups();
  const selections = scopes.map(({ group, shard }) => javascriptSuitePartition(group, shard, groups));
  const files = selections.flatMap(selection => selection.files);
  assert.equal(new Set(selections.map(selection => selection.name)).size, expected.length);
  assert.equal(new Set(files).size, files.length, 'every test file executes once');
  assert.deepEqual([...files].sort(), Object.values(groups).flat().sort());

  assert.deepEqual(needs(early), ['candidates', 'self-translation-report']);
  assert.deepEqual(needs(downstream), ['candidates', 'consumers', 'rml-workloads', 'formal-ai-workloads', 'self-translation-report']);
  const normalize = step => step.text.replace(/^ {10}ISSUE_195_(?:RML|FORMAL_AI)_WORKLOAD_REPORT:.*\n/gmu, '');
  const reportSteps = new Set(['Download the relative-meta-logic workload report', 'Download the formal-ai workload report']);
  assert.deepEqual(early.steps.map(normalize), downstream.steps.filter(step => !reportSteps.has(step.name)).map(normalize),
    'only workload-report downloads and variables differ between collection steps');
  const earlyText = early.steps.map(step => step.text).join('\n');
  assert.doesNotMatch(earlyText, /ISSUE_195_(?:RML|FORMAL_AI)_WORKLOAD_REPORT|name: issue-195-(?:rml|formal-ai)-workloads-/u);

  for (const suite of [early, downstream]) {
    assert.match(suite.header, /name: Evidence Stage \(javascript-suite - \$\{\{ matrix.group \}\} \$\{\{ matrix.shard \}\}\)/u);
    assert.match(suite.header, /timeout-minutes: 60\n/u);
    assert.match(suite.header, /fail-fast: false\n/u);
    assert.match(suite.header, /group: issue-195-stage-\$\{\{ github.ref \}\}-javascript-suite-\$\{\{ matrix.group \}\}-\$\{\{ matrix.partition \}\}/u);
    assert.match(suite.header, /cancel-in-progress: true/u);
    assert.equal(condition(suite), "${{ !cancelled() && github.event_name != 'release' && needs.candidates.result == 'success' }}");
    for (const name of ['Setup Node.js', 'Setup Rust', 'Setup Lean', 'Setup OCaml', 'Install Rocq 9.3',
      'Install JavaScript dependencies', 'Provision the pinned external grammar test inputs',
      'Build the independently corrected Agda layout oracle']) {
      assert.ok(suite.steps.some(step => step.name === name), `${suite.id} retains ${name}`);
    }
    const execute = suite.steps.find(step => step.name === 'Produce the stage evidence');
    assert.match(execute.text, /STAGE: javascript-suite\n/u);
    assert.match(execute.text, /META_LANGUAGE_TOOLING_FULL_SCOPE: '1'/u);
    assert.match(execute.run, /--commit "\$GITHUB_SHA"/u);
    assert.match(execute.run, /--javascript-group "\$JAVASCRIPT_GROUP"/u);
    assert.match(execute.run, /--javascript-shard "\$JAVASCRIPT_SHARD"/u);
    assert.match(execute.text, /ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY:/u);
    const reports = suite.steps.findIndex(step => step.name === 'Download the complete self-translation reports');
    assert.ok(reports >= 0 && reports < suite.steps.indexOf(execute));
    assert.match(suite.steps[reports].text, /pattern: self-translation-report-\$\{\{ github.sha \}\}-\*/u);
    const upload = suite.steps.find(step => step.name === 'Upload the stage outputs');
    assert.ok(upload.afterFailure);
    assert.match(upload.text, /name: issue-195-stage-javascript-suite-\$\{\{ matrix.group \}\}-\$\{\{ matrix.partition \}\}-\$\{\{ github.sha \}\}/u);
    for (const output of ['work/stage-*', 'work/native/', 'logs/', 'artifacts/', 'runtime-parity/']) {
      assert.ok(upload.text.includes(`issue-195-results/${output}`), `${suite.id} retains ${output}`);
    }
    assert.match(upload.text, /if-no-files-found: error/u);
    assert.match(upload.text, /retention-days: 30/u);
    assert.match(suite.steps.at(-1).run, /clean-caches\.mjs --event ci-teardown/u);
    assert.ok(suite.steps.at(-1).afterFailure);
  }
  for (const [name, artifact] of [
    ['Download the relative-meta-logic workload report', 'rml'],
    ['Download the formal-ai workload report', 'formal-ai'],
  ]) {
    const report = downstream.steps.findIndex(step => step.name === name);
    assert.ok(report >= 0 && report < downstream.steps.findIndex(step => step.name === 'Produce the stage evidence'));
    assert.ok(downstream.steps[report].text.includes(`name: issue-195-${artifact}-workloads-`));
  }
  assert.deepEqual(needs(rust), ['candidates', 'consumers', 'rml-workloads', 'formal-ai-workloads', 'javascript-suite-independent', 'javascript-suite']);
  assert.equal(condition(rust), "${{ !cancelled() && github.event_name != 'release' && needs.candidates.result == 'success' && needs.javascript-suite.result == 'success' && needs.javascript-suite-independent.result == 'success' }}");
  const originalAggregateNeeds = ['js', 'self-translation-report-validation', 'candidates', 'consumers', 'rml-workloads',
    'formal-ai-workloads', 'javascript-suite-independent', 'runtime-parity', 'native-javascript', 'native-rust',
    'native-translations', 'javascript-suite', 'rust-suite', 'delivery'];
  // 1c7a4ac7 adds the native workflow producer so the aggregate can authenticate
  // its complete physical test union. Retain every original edge and its order,
  // and require exactly that one added edge before all the original tail edges.
  assert.deepEqual(needs(aggregate).filter(name => name !== 'rust'), originalAggregateNeeds);
  assert.deepEqual(needs(aggregate), [originalAggregateNeeds[0], 'rust', ...originalAggregateNeeds.slice(1)]);
  assert.equal(condition(aggregate), '${{ always() }}');
  const download = aggregate.steps.find(step => /pattern: issue-195-stage-/u.test(step.text));
  assert.match(download.text, /pattern: issue-195-stage-\*-\$\{\{ github.sha \}\}/u);
  assert.match(download.text, /merge-multiple: true/u);
}

test('early evidence preserves every exact JavaScript scope, artifact, tool and gate', () => {
  assertEvidenceDagConservation(workflow);
});

test('every actual JavaScript workload-report reader remains in the report-dependent collection', () => {
  const partition = partitionTests(testFiles());
  const readers = testFiles().filter(file => /process\.env(?:\[(?:FORMAL_AI|RML)_WORKLOAD_REPORT_VARIABLE\]|\.ISSUE_195_(?:FORMAL_AI|RML)_WORKLOAD_REPORT\b)/u
    .test(read(`js/tests/${file}`)));
  assert.deepEqual(readers, ['issue-195-downstream-formal-ai-workloads.test.js', 'issue-195-downstream-rml-workloads.test.js']);
  assert.ok(readers.every(file => partition.groups.downstream.includes(file)));
  const runner = read('js/scripts/run-issue-195-evidence.mjs');
  assert.doesNotMatch(runner, /ISSUE_195_(?:FORMAL_AI|RML)_WORKLOAD_REPORT/u,
    'stage startup does not depend on workload reports');
});

test('scope loss, duplicate scopes, missing producer edges and premature Rust fail the DAG guard', () => {
  const row = '          - group: grammar\n            shard: 1/1\n            partition: 1-of-1\n';
  for (const [name, mutate] of [
    ['removed physical scope', text => text.replace(row, '')],
    ['duplicate physical scope', text => text.replace(row, row + row)],
    ['lost workload producer', text => text.replace('needs: [candidates, consumers, rml-workloads, formal-ai-workloads, self-translation-report]',
      'needs: [candidates, consumers, rml-workloads, self-translation-report]')],
    ['premature Rust needs', text => text.replace('formal-ai-workloads, javascript-suite-independent, javascript-suite]', 'formal-ai-workloads, javascript-suite]')],
    ['premature Rust condition', text => text.replace(" && needs.javascript-suite-independent.result == 'success'", '')],
    ['either JavaScript collection is insufficient', text => text.replace(" && needs.javascript-suite-independent.result == 'success'", " || needs.javascript-suite-independent.result == 'success'")],
    ['aggregate misses early jobs', text => text.replace('formal-ai-workloads, javascript-suite-independent, runtime-parity', 'formal-ai-workloads, runtime-parity')],
    ['lost original deadline', text => text.replace('timeout-minutes: 60\n    # The evidence groups', 'timeout-minutes: 61\n    # The evidence groups')],
  ]) {
    const changed = mutate(workflow);
    assert.notEqual(changed, workflow, name);
    assert.throws(() => assertEvidenceDagConservation(changed), { name: 'AssertionError' }, name);
  }
});

import { restoreVerificationConcurrencyHistory } from './support/verification-concurrency-history.js';
