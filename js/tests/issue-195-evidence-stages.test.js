// The issue #195 evidence run is split into stages that CI runs as separate
// jobs. Running every suite and native toolchain in one process tree pushed
// the aggregate job past its memory; these tests pin the split, the merge of
// the stage records, and the gate error a failed stage becomes.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest, evaluateIssue195Acceptance } from '../scripts/issue-195-acceptance-lib.mjs';
import { buildEvidencePlan } from '../scripts/issue-195-evidence-plan.mjs';
import {
  BOUNDED_BUILD_ENVIRONMENT,
  NATIVE_TARGETS,
  RUNTIMES,
  CHECKPOINTS,
  aggregateGroups,
  boundedEnvironment,
  evidenceStage,
  evidenceStages,
  mergeStageRecords,
  stageFiles,
  stageGateError,
  toolchainProblems,
  translationGroups,
  uncitedFiles,
} from '../scripts/issue-195-evidence-stages.mjs';
import { recordIssue195DirectiveObservation as observe } from './support/issue-195-observations.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (relative) => readFileSync(path.join(root, relative), 'utf8');
const commit = 'a'.repeat(40);

function stageRecord(stage, overrides = {}) {
  return {
    schemaVersion: 1,
    issue: 195,
    stage: stage.name,
    commit,
    checkpoint: 'pre-merge',
    toolchainVersions: { node: 'v24.0.0', stage: stage.name },
    groups: Object.fromEntries(stage.groups.map((group) => [group, {
      commands: [`run ${group}`], artifacts: [`logs/${group}.log`], failureLogs: [],
    }])),
    error: null,
    ...overrides,
  };
}

const allRecords = () => new Map(evidenceStages('pre-merge').map((stage) => [stage.name, stageRecord(stage)]));

test('the stages and the aggregate together produce exactly the evidence plan groups of each checkpoint', async () => {
  const manifest = await buildIssue195Manifest(root);
  for (const checkpoint of CHECKPOINTS) {
    const planned = buildEvidencePlan(manifest, checkpoint).map(({ group }) => group).sort();
    const staged = evidenceStages(checkpoint).flatMap((stage) => stage.groups);
    const produced = [...staged, ...aggregateGroups(checkpoint)];
    assert.equal(new Set(produced).size, produced.length, `${checkpoint}: a group has two producers`);
    assert.deepEqual(produced.sort(), planned, checkpoint);
  }
  assert.deepEqual(
    evidenceStages('pre-merge').map(({ name }) => name),
    ['javascript-suite', 'rust-suite', 'runtime-parity', 'native-javascript', 'native-rust', 'native-lean',
      'native-rocq', 'delivery'],
  );
  // Live default-branch rules exist only after merge: no pull request stage inspects them.
  assert.deepEqual(evidenceStages('post-merge').map(({ name }) => name), ['merge-enforcement']);
  assert.deepEqual(evidenceStages('release-delivery').map(({ name }) => name), ['delivery']);
  assert.deepEqual(CHECKPOINTS, ['pre-merge', 'post-merge', 'release-delivery']);
  assert.throws(() => evidenceStages('pull-request'), /unknown issue-195 evidence checkpoint/);
  assert.throws(() => evidenceStage('pre-merge', 'everything'), /unknown pre-merge evidence stage everything/);
  assert.deepEqual(stageFiles('rust-suite'), {
    record: 'work/stage-rust-suite.json', observations: 'work/stage-rust-suite.jsonl',
  });
});

test('each native stage records only its own toolchain', () => {
  const tools = Object.fromEntries(NATIVE_TARGETS.map((target) =>
    [target, evidenceStage('pre-merge', `native-${target.toLowerCase()}`).tools]));
  assert.deepEqual(tools, {
    JavaScript: ['node'], Rust: ['node', 'rustc'], Lean: ['node', 'lean'], Rocq: ['node', 'rocq'],
  });
});

test('merged stage records keep each group with the toolchain of the stage that produced it', () => {
  const { groups, stageErrors } = mergeStageRecords({ checkpoint: 'pre-merge', commit, records: allRecords() });
  assert.deepEqual(stageErrors, []);
  assert.deepEqual(groups.get('suite:rust'), {
    commands: ['run suite:rust'], artifacts: ['logs/suite:rust.log'], failureLogs: [],
    toolchainVersions: { node: 'v24.0.0', stage: 'rust-suite' },
  });
  assert.equal(groups.get('native:Lean:javascript').toolchainVersions.stage, 'native-lean');
});

test('a missing, failed, foreign or mismatched stage becomes one error naming the stage', () => {
  const stages = Object.fromEntries(evidenceStages('pre-merge').map((stage) => [stage.name, stage]));
  const records = allRecords();
  records.delete('native-rocq');
  records.set('rust-suite', stageRecord(stages['rust-suite'], { groups: {}, error: 'Rust tests failed; see logs/suite-rust.log' }));
  records.set('delivery', stageRecord(stages.delivery, { commit: 'b'.repeat(40) }));
  records.set('native-javascript', stageRecord(stages['native-javascript'], {
    groups: { 'native:JavaScript:javascript': {}, 'native:JavaScript:rust': {}, 'suite:javascript': {} },
  }));
  records.set('runtime-parity', stageRecord(stages['runtime-parity'], { groups: {} }));
  records.set('native-lean', new SyntaxError('Unexpected end of JSON input'));
  records.set('linting', stageRecord({ name: 'linting', groups: [] }));
  const { groups, stageErrors } = mergeStageRecords({ checkpoint: 'pre-merge', commit, records });
  assert.deepEqual(stageErrors.map(stageGateError), [
    'evidence stage rust-suite failed: Rust tests failed; see logs/suite-rust.log',
    'evidence stage runtime-parity failed: the stage reported no record for runtime-parity',
    'evidence stage native-javascript failed: the stage reported group suite:javascript, which belongs to another stage',
    'evidence stage native-lean failed: the stage record cannot be read: Unexpected end of JSON input',
    'evidence stage native-rocq failed: the stage produced no record; see its job log',
    `evidence stage delivery failed: the stage record has commit ${'b'.repeat(40)}, expected ${commit}`,
    'evidence stage linting failed: linting is not a pre-merge evidence stage',
  ]);
  for (const group of ['suite:rust', 'runtime-parity', 'native:Lean:rust', 'native:Rocq:rust', 'delivery:npm']) {
    assert.equal(groups.has(group), false, group);
  }
  assert.equal(groups.has('native:JavaScript:javascript'), true);
  // The foreign report does not replace the group of the stage that owns it.
  assert.equal(groups.get('suite:javascript').toolchainVersions.stage, 'javascript-suite');
  observe('I195-RESOURCE-CI-EVIDENCE-STAGES', ['failedStageIsOneGateError'],
    'a missing, failed, foreign or mismatched stage becomes one error naming the stage');
});

test('a translation group needs its runtime suite, the parity observation and its native validation', () => {
  const { groups } = mergeStageRecords({ checkpoint: 'pre-merge', commit, records: allRecords() });
  const complete = translationGroups(groups);
  assert.deepEqual(
    [...complete.keys()].sort(),
    RUNTIMES.flatMap((runtime) => NATIVE_TARGETS.map((target) => `translation:${target}:${runtime}`)).sort(),
  );
  assert.deepEqual(complete.get('translation:Lean:rust').commands, [
    'run suite:rust', 'run runtime-parity', 'run native:Lean:rust',
  ]);
  assert.deepEqual(complete.get('translation:Lean:rust').failureLogs, []);
  groups.delete('suite:rust');
  groups.delete('native:Rocq:javascript');
  assert.deepEqual([...translationGroups(groups).keys()].sort(), [
    'translation:JavaScript:javascript', 'translation:Lean:javascript', 'translation:Rust:javascript',
  ]);
});

test('the evaluator reports every stage error as a gate error', async () => {
  const manifest = await buildIssue195Manifest(root);
  const document = {
    schemaVersion: 1, issue: 195, commit, results: [],
    stageErrors: [{ stage: 'rust-suite', error: 'Rust tests failed; see logs/suite-rust.log' }],
  };
  const report = evaluateIssue195Acceptance(manifest, [document], { checkpoint: 'pre-merge', commit });
  assert.equal(report.passed, false);
  assert.ok(report.gateErrors.includes('evidence stage rust-suite failed: Rust tests failed; see logs/suite-rust.log'));
  const malformed = evaluateIssue195Acceptance(manifest, [{ ...document, stageErrors: 'none' }], {
    checkpoint: 'pre-merge', commit,
  });
  assert.ok(malformed.gateErrors.includes('result document stageErrors must be an array'));
});

test('the evidence commands run with bounded build parallelism unless the caller set it', () => {
  assert.deepEqual(BOUNDED_BUILD_ENVIRONMENT, { CARGO_BUILD_JOBS: '2', RUST_TEST_THREADS: '2', CARGO_INCREMENTAL: '0' });
  const env = boundedEnvironment({ PATH: '/bin', CARGO_BUILD_JOBS: '4', RUST_TEST_THREADS: '' });
  assert.deepEqual(env, { PATH: '/bin', CARGO_BUILD_JOBS: '4', RUST_TEST_THREADS: '2', CARGO_INCREMENTAL: '0' });
  const runner = read('js/scripts/run-issue-195-evidence.mjs');
  assert.match(runner, /env = boundedEnvironment\(process\.env\)/u);
  // The stages run one after another; no suite shares the machine with another.
  assert.match(runner, /for \(const stage of stages\) await runStage\(stage\);/u);
  assert.doesNotMatch(runner, /Promise\.all\(\s*\[?\s*(?:run|produce)/u);
  // The Rust suite builds only the default features, and each JavaScript test group is its own process.
  assert.doesNotMatch(runner, /--all-features/u);
  assert.match(runner, /'scripts\/test-groups\.mjs', '--run', group\]/u);
  observe('I195-RESOURCE-BOUNDED-SEQUENTIAL-EVIDENCE',
    ['stagesRunSequentially', 'boundedBuildEnvironment', 'defaultFeaturesOnly', 'testGroupsRunSeparately'],
    'the evidence commands run with bounded build parallelism unless the caller set it');
});

test('declared toolchains must match exactly; tools a stage does not use are not required', () => {
  assert.deepEqual(toolchainProblems({ node: 'v24.0.0', rustc: 'rustc 1.99.0 (abc 2026-09-01)' }), []);
  assert.deepEqual(toolchainProblems({ lean: 'Lean (version 4.33.0)' }), [
    'lean version does not match declared toolchain 4.34.1: Lean (version 4.33.0)',
  ]);
});

test('a native stage deletes the compiler outputs no cell cites', () => {
  assert.deepEqual(
    uncitedFiles(['Main.vo', 'Main.v', 'Main.glob', 'reproduce.sh', 'rejected.log', 'lib.rmeta'],
      ['Main.v', 'reproduce.sh', 'rejected.log']),
    ['Main.glob', 'Main.vo', 'lib.rmeta'],
  );
  const runner = read('js/scripts/run-issue-195-evidence.mjs');
  assert.match(runner, /uncitedFiles\(/u);
  observe('I195-RESOURCE-NATIVE-OUTPUT-CLEANUP', ['uncitedOutputsDeleted', 'citedEvidenceKept'],
    'a native stage deletes the compiler outputs no cell cites');
});

test('CI runs each stage as its own job and the aggregate only merges and evaluates', () => {
  const workflow = read('.github/workflows/issue-195-acceptance.yml');
  const job = (id) => {
    const start = workflow.indexOf(`\n  ${id}:\n`);
    assert.ok(start >= 0, `job ${id}`);
    const end = workflow.slice(start + 1).search(/\n {2}[\w-]+:\n/u);
    return workflow.slice(start, end < 0 ? undefined : start + 1 + end);
  };
  assert.match(job('runtime-parity'), /--stage runtime-parity/u);
  // The JavaScript stages run before the Rust stages; Lean and Rocq share one matrix.
  assert.match(job('native-translations'), /target: \[lean, rocq\]/u);
  for (const target of ['javascript', 'rust']) {
    assert.match(job(`native-${target}`), /TARGET: native-/u);
  }
  assert.match(job('native-rust'), /needs: \[runtime-parity, native-javascript\]/u);
  assert.match(job('rust-suite'), /needs: \[[^\]]*javascript-suite\]/u);
  for (const id of ['native-javascript', 'native-rust', 'native-translations']) {
    assert.match(job(id), /--stage "\$TARGET"/u, id);
  }
  for (const stage of ['javascript-suite', 'rust-suite', 'delivery']) {
    assert.match(job(stage), new RegExp(`STAGE: ${stage}\n`, 'u'), stage);
    assert.doesNotMatch(job(stage), /merge-enforcement|RULESET_TOKEN|pull-requests: read/u, stage);
  }
  // The post-merge report runs on main only and never blocks.
  const postMerge = job('post-merge');
  assert.match(postMerge, /if: \$\{\{ github\.event_name == 'push' \}\}/u);
  assert.match(postMerge, /continue-on-error: true/u);
  assert.match(postMerge, /--checkpoint post-merge/u);
  assert.doesNotMatch(job('acceptance'), /post-merge/u);
  // A release reruns only the delivery stage.
  for (const stage of ['native-javascript', 'native-rust', 'native-translations', 'javascript-suite', 'rust-suite']) {
    assert.match(job(stage), /github\.event_name != 'release'/u, stage);
  }
  assert.doesNotMatch(job('delivery'), /github\.event_name != 'release'/u);
  for (const id of ['runtime-parity', 'native-javascript', 'native-rust', 'native-translations', 'javascript-suite', 'rust-suite', 'delivery']) {
    assert.match(job(id), /name: issue-195-stage-[^\n]*-\$\{\{ github\.sha \}\}/u, id);
    assert.match(job(id), /issue-195-results\/work\/stage-\*/u, id);
  }
  const aggregate = job('acceptance');
  assert.match(aggregate, /name: Full Requirements Aggregate/u);
  assert.match(aggregate, /needs: \[[^\]]*runtime-parity, native-javascript, native-rust, native-translations, javascript-suite, rust-suite, delivery\]/u);
  assert.match(aggregate, /pattern: issue-195-stage-\*-\$\{\{ github\.sha \}\}/u);
  assert.match(aggregate, /merge-multiple: true/u);
  assert.match(aggregate, /--aggregate/u);
  // The aggregate builds and tests nothing itself.
  assert.doesNotMatch(aggregate, /rust-toolchain|setup-ocaml|elan-init|--stage/u);
  observe('I195-RESOURCE-CI-EVIDENCE-STAGES',
    ['stagesRunAsSeparateJobs', 'nativeTranslationsMatrixByTarget', 'eachStageUploadsItsEvidence', 'aggregateOnlyMergesAndEvaluates'],
    'CI runs each stage as its own job and the aggregate only merges and evaluates');
});
