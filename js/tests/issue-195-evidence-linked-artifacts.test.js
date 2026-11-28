import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const workflowJobs = (file) => new Map(parseWorkflow(read(`.github/workflows/${file}`)).map((job) => [job.id, job]));

test('consumer and workload artifact paths match their linked report outputs and every acceptance reader', () => {
  const ci = workflowJobs('ci.yml');
  const reportPaths = new Map();
  for (const [id, producer] of [
    ['consumers', 'run-issue-195-consumer.mjs'],
    ['rml-workloads', 'run-rml-pr184-workloads.mjs'],
    ['formal-ai-workloads', 'run-formal-ai-workloads.mjs'],
  ]) {
    const job = ci.get(id);
    const run = job.steps.find((step) => step.run.includes(producer));
    const output = /--out\s+([^\s]+)/u.exec(run?.run ?? '')?.[1];
    assert.ok(output?.endsWith('/report.lino'), `${id} writes a linked report`);
    const dependencies = job.steps.findIndex((step) => /npm ci --ignore-scripts/u.test(step.run));
    assert.ok(dependencies >= 0 && dependencies < job.steps.indexOf(run), `${id} installs the bootstrap-safe linked report reader`);
    assert.match(job.steps[dependencies].text, /working-directory: js/u);
    assert.ok(job.steps.some((step) => /actions\/upload-artifact@/u.test(step.text)
      && step.text.includes(`path: ${output}\n`)), `${id} uploads the exact report it produced`);
    reportPaths.set(id, output);
  }
  for (const id of ['javascript-suite', 'rust-suite', 'delivery']) {
    const text = ci.get(id).steps.map((step) => step.text).join('\n');
    assert.ok(text.includes(`ISSUE_195_RML_WORKLOAD_REPORT: \${{ github.workspace }}/${reportPaths.get('rml-workloads')}`), `${id} receives the RML report`);
    assert.ok(text.includes(`ISSUE_195_FORMAL_AI_WORKLOAD_REPORT: \${{ github.workspace }}/${reportPaths.get('formal-ai-workloads')}`), `${id} receives the formal-ai report`);
  }
  const retained = ci.get('formal-ai-workloads').steps.map((step) => step.text).join('\n');
  assert.match(retained, /issue-195-formal-ai-workloads\/\*-outputs\.lino/u);
  assert.match(retained, /issue-195-formal-ai-workloads\/formal-ai-inputs\.lino/u);
});

test('linked report storage keeps every self-translation shard and exact-commit verification gate', () => {
  for (const [file, producer, consumer] of [
    ['ci.yml', 'self-translation-report', 'self-translation-report-validation'],
    ['js.yml', 'reports', 'report-assertions'],
  ]) {
    const jobs = workflowJobs(file);
    assert.match(jobs.get(producer).header, /shard: \[1, 2, 3, 4, 5, 6, 7, 8\]/u);
    assert.ok(jobs.get(producer).steps.some((step) => /--shard \$\{\{ matrix\.shard \}\}\/8/u.test(step.run)));
    const validator = jobs.get(consumer);
    assert.match(validator.header, new RegExp(`needs: ${producer}`, 'u'));
    assert.ok(validator.steps.some((step) => /pattern: self-translation-report-\$\{\{ github\.sha \}\}-\*/u.test(step.text)));
    assert.ok(validator.steps.some((step) => /test-groups\.mjs --run report-artifacts/u.test(step.run)));
    assert.doesNotMatch(validator.header, /continue-on-error: true/u);
  }
  const producer = read('js/scripts/generate-self-translation-report.mjs');
  assert.match(producer, /self-translation-report\.lino/u);
  for (const assertion of ['row.sourceSha256', 'row.rustSha256', 'report.commit']) {
    assert.ok(producer.includes(assertion), `the report verifier retains ${assertion}`);
  }
  const coverage = read('js/tests/self-translation-report-artifacts.test.js');
  assert.match(coverage, /assert\.equal\(row\.coverage\.unrepresentedBytes, 0/u);
  assert.match(coverage, /row\.items\.translated > 0/u);
});

test('grammar and final evidence artifacts use linked storage while standard package locks remain JSON', () => {
  const ci = workflowJobs('ci.yml');
  assert.ok(ci.get('grammar-bulk-pipeline').steps.some((step) => /hashFiles\('grammar-bulk\/grammar-bulk-matrix\.lino'\)/u.test(step.text)));
  const final = ci.get('acceptance').steps.map((step) => step.run).join('\n');
  assert.match(final, /--produce-gate-results issue-195-results\/gate-faults\.lino/u);
  assert.match(final, /--json-report issue-195-artifacts\/report\.lino/u);
  assert.match(ci.get('post-merge').steps.map((step) => step.run).join('\n'), /--json-report issue-195-artifacts\/post-merge-report\.lino/u);
  assert.match(ci.get('consumers').steps.map((step) => step.run).join('\n'), /--consumer-lock "\$RUNNER_TEMP\/issue-195-consumer\/npm-consumer\/package-lock\.json"/u);
  assert.match(read('.github/workflows/rust.yml'), /cache-dependency-path: js\/package-lock\.json/u);
});
