// Structural controls for the connected partition producer; no external
// process, dependency installation or native workload runs in these tests.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const needs = (job) => (/^    needs: (.+)$/mu.exec(job?.header ?? '')?.[1] ?? '').replace(/[\[\]]/gu, '').split(',').map((value) => value.trim()).filter(Boolean);
const runStep = (job, fragment) => job?.steps.find((step) => step.run.includes(fragment));

function partitionWorkflowProblems(workflow, runner, transport) {
  const jobs = new Map(parseWorkflow(workflow).map((job) => [job.id, job]));
  const producer = jobs.get('formal-ai-workload-parts');
  const merger = jobs.get('formal-ai-workloads');
  const problems = [];
  const requireValue = (condition, message) => { if (!condition) problems.push(message); };
  requireValue(Boolean(producer && merger), 'both execution matrix and canonical merger are required');
  if (!producer || !merger) return problems;
  requireValue(JSON.stringify(needs(producer)) === JSON.stringify(['candidates']), 'both consumers retain the original upstream JavaScript barrier');
  requireValue(JSON.stringify(needs(merger)) === JSON.stringify(['candidates', 'formal-ai-workload-parts']), 'the canonical merger must wait for both real matrix cells');
  requireValue(/fail-fast: false/u.test(producer.header) && /partition: \[corpus-ratchet, remaining\]/u.test(producer.header), 'exactly two isolated execution cells collect all failures');
  requireValue(/timeout-minutes: 120/u.test(producer.header), 'the original native deadline is unchanged');
  requireValue(/group: issue-195-formal-ai-workloads-\$\{\{ github\.ref \}\}-\$\{\{ matrix\.partition \}\}/u.test(producer.header), 'producer concurrency cannot cancel its sibling');
  requireValue(/!cancelled\(\) && needs\.candidates\.result == 'success'/u.test(merger.header), 'failed upstream JavaScript cannot start a source-only merger');
  const execute = runStep(producer, 'run-formal-ai-workloads.mjs');
  requireValue(Boolean(execute) && execute.run.includes('--partition "${{ matrix.partition }}"') &&
    execute.run.includes('--cargo-target-dir "$RUNNER_TEMP/issue-195-formal-ai-build/target"') &&
    execute.run.includes('--out ".issue-195-work/formal-ai-partitions/${{ matrix.partition }}/partition.lino"'), 'native cells use genuine partial outputs and the original owned target');
  requireValue(execute && !/cache-hit|\n\s+if:/u.test(execute.text), 'cache hits cannot skip either real consumer');
  const save = producer.steps.find((step) => step.name === 'Save Formal AI build outputs');
  requireValue(save?.text.includes("matrix.partition == 'remaining'"), 'only the remaining cell saves the shared build cache');
  const partialUpload = producer.steps.find((step) => step.text.includes('name: issue-195-formal-ai-partition-${{ matrix.partition }}-'));
  requireValue(partialUpload?.text.includes('${{ github.sha }}-${{ github.run_id }}-${{ github.run_attempt }}') &&
    partialUpload.text.includes('path: .issue-195-work/formal-ai-partitions/${{ matrix.partition }}/'), 'partials retain exact commit/run/attempt and producer-scoped directories');
  const merged = runStep(merger, 'run-formal-ai-workloads.mjs');
  requireValue(Boolean(merged) && merged.run.includes('--merge-partitions .issue-195-work/formal-ai-partitions/corpus-ratchet,.issue-195-work/formal-ai-partitions/remaining') &&
    merged.run.includes('--producer-result "${{ needs.formal-ai-workload-parts.result }}"') &&
    merged.run.includes('--out issue-195-artifacts/formal-ai-workloads/report.lino'), 'only the proven union writes the original canonical report');
  requireValue(!merger.steps.some((step) => /rust-toolchain|\brustc\b|\bcargo\b|--partition "/u.test(step.text)), 'the merge job does no native build or workload work');
  const npm = merger.steps.findIndex((step) => /npm ci --ignore-scripts/u.test(step.run));
  requireValue(npm >= 0 && npm < merger.steps.indexOf(merged), 'the canonical linked reader installs before merge');
  for (const partition of ['corpus-ratchet', 'remaining']) {
    const exact = `name: issue-195-formal-ai-partition-${partition}-${'${{ github.sha }}'}-${'${{ github.run_id }}'}-${'${{ github.run_attempt }}'}`;
    const download = merger.steps.find((step) => step.text.includes(exact));
    requireValue(download?.text.includes(`path: .issue-195-work/formal-ai-partitions/${partition}`), 'each exact partial downloads into a disjoint directory');
  }
  requireValue(merger.steps.some((step) => step.text.includes('name: issue-195-formal-ai-workloads-${{ github.sha }}') &&
    step.text.includes('path: issue-195-artifacts/formal-ai-workloads/report.lino')), 'the public report artifact identity stays unchanged');
  for (const job of ['javascript-suite', 'rust-suite', 'delivery', 'acceptance']) {
    requireValue(needs(jobs.get(job)).includes('formal-ai-workloads'), `${job} still requires the canonical full-report job`);
  }
  const javascript = runner.indexOf('report.npm = await guarded');
  const native = runner.indexOf('report.crate = skipRust');
  requireValue(javascript >= 0 && native > javascript, 'every local complete JavaScript consumer runs before native work');
  requireValue(/if \(mergePartitions\)[\s\S]*?mergeFormalAiPartitionBundles[\s\S]*?else await main\(\)/u.test(runner), 'merge mode never enters the workload main');
  requireValue(runner.includes("const cargoTargetDirectory = formalAiCargoTargetDirectory(workDirectory, option('--cargo-target-dir', null));") &&
    runner.indexOf('async function rustWorkloads') < runner.indexOf('const cargoTargetDirectory ='), 'only actual native workloads initialize Cargo targets');
  requireValue(transport.includes('readFormalAiNativeInventory(state.nativeInventory, fixture)') &&
    transport.includes('formalAiWorkloadPartitionCommands(state.inventory, partition)'), 'actual full lists control exact execution');
  requireValue(transport.includes('mergeFormalAiWorkloadPartitions(parts, fixture, expected)') &&
    transport.includes('formalAiPartitionExpectations({ root, tarball, crate, fixture, environment, producerResult })'), 'the merger builds its own expected context rather than trusting envelopes');
  requireValue(!/spawn|execFile|node:child_process/u.test(transport), 'bundle transport/merge cannot run native processes');
  return problems;
}

test('Formal AI execution partitions preserve source-only canonical merging all JavaScript barriers and exact public artifacts', () => {
  assert.deepEqual(partitionWorkflowProblems(restoreVerificationConcurrencyHistory(read('.github/workflows/ci.yml'), 'ci.yml'), read('js/scripts/run-formal-ai-workloads.mjs'), read('js/scripts/formal-ai-partition-bundles.mjs')), []);
});

test('Formal AI workflow refuses lost producers mixed artifacts overwritten scopes cache skipping and early native execution', () => {
  const workflow = restoreVerificationConcurrencyHistory(read('.github/workflows/ci.yml'), 'ci.yml');
  const runner = read('js/scripts/run-formal-ai-workloads.mjs');
  const transport = read('js/scripts/formal-ai-partition-bundles.mjs');
  for (const [name, change] of [
    ['lost matrix barrier', (value) => value.replace('needs: [candidates, formal-ai-workload-parts]', 'needs: candidates')],
    ['early producer', (value) => value.replace('    name: Formal AI Workloads (${{ matrix.partition }})\n    needs: candidates', '    name: Formal AI Workloads (${{ matrix.partition }})\n    needs: changes')],
    ['sibling cancellation', (value) => value.replace('group: issue-195-formal-ai-workloads-${{ github.ref }}-${{ matrix.partition }}', 'group: issue-195-formal-ai-workloads-${{ github.ref }}')],
    ['weakened deadline', (value) => value.replace('    timeout-minutes: 120\n', '    timeout-minutes: 180\n')],
    ['stale producer identity', (value) => value.replace('name: issue-195-formal-ai-partition-${{ matrix.partition }}-${{ github.sha }}-${{ github.run_id }}-${{ github.run_attempt }}', 'name: issue-195-formal-ai-partition-${{ matrix.partition }}-${{ github.sha }}')],
    ['overwritten evidence', (value) => value.replace('path: .issue-195-work/formal-ai-partitions/remaining', 'path: .issue-195-work/formal-ai-partitions/corpus-ratchet')],
    ['fabricated producer success', (value) => value.replace('--producer-result "${{ needs.formal-ai-workload-parts.result }}"', '--producer-result success')],
    ['partial public report', (value) => value.replace('--out issue-195-artifacts/formal-ai-workloads/report.lino', '--out .issue-195-work/remaining/partition.lino')],
    ['parallel cache reservation', (value) => value.replace("matrix.partition == 'remaining' && ", '')],
  ]) assert.ok(partitionWorkflowProblems(change(workflow), runner, transport).length, name);
  assert.ok(partitionWorkflowProblems(workflow, runner.replace('report.npm = await guarded', 'removedNpm = await guarded'), transport).length);
  assert.ok(partitionWorkflowProblems(workflow, runner, transport + "\nimport { spawn } from 'node:child_process';\n").length);
});

test('Formal AI partition adoption preserves every byte of the existing complete report validator', () => {
  const source = readFileSync(new URL('../scripts/issue-195-formal-ai-workloads.mjs', import.meta.url));
  // Exact original source bytes read before this additive export was drafted.
  // This also protects all original private grounding/coverage/probe checks.
  assert.equal(createHash('sha256').update(source.subarray(0, 12317)).digest('hex'),
    '0a6d06b129255286be21e30be4d95a9f6bebe4cfbaca9fbc34f5bae4dadb3162');
  assert.match(source.subarray(12317).toString('utf8'), /export function formalAiWorkloadPartitionGroundingProblems/u);
});

import { restoreVerificationConcurrencyHistory } from './support/verification-concurrency-history.js';
