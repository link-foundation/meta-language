import assert from 'node:assert/strict';
import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { readEvidenceCommandFailureTail, redactEvidenceCommandLog } from '../scripts/issue-195-evidence-command-logs.mjs';
import { readEvidenceStageExecutionRecords } from '../scripts/issue-195-evidence-execution-records.mjs';
import { evidenceStages, mergeStageRecords, stageFiles, translationGroups } from '../scripts/issue-195-evidence-stages.mjs';
import { javascriptSuiteCommandArguments, javascriptSuiteGroupEntries, javascriptSuitePartition,
  javascriptSuiteSelection, javascriptSuiteTestGroups } from '../scripts/issue-195-javascript-suite-partitions.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { TEST_GROUPS } from '../scripts/test-groups.mjs';
import { structuredDataRecordToLino } from '../src/structured-data-records.js';

const commit = 'a'.repeat(40);
const groups = javascriptSuiteTestGroups();
const counts = group => group === 'grammar-native' ? 3 : group === 'conformance' ? 2 : 1;
const partitionSelections = () => Object.keys(TEST_GROUPS).flatMap(group =>
  Array.from({ length: counts(group) }, (_, index) => javascriptSuitePartition(group, `${index + 1}/${counts(group)}`, groups)));

function partitionRecord(selection) {
  return { schemaVersion: 1, issue: 195, stage: selection.name, commit, checkpoint: 'pre-merge',
    javascriptPartition: selection, toolchainVersions: { node: 'v24.0.0', platform: 'linux-x64' },
    groups: { 'suite:javascript': { commands: [['node', ...javascriptSuiteCommandArguments(selection.group, selection.shard)].join(' ')],
      artifacts: [`logs/suite-${selection.name}.log`], failureLogs: [] } }, error: null };
}

function records() {
  return new Map([
    ...evidenceStages('pre-merge').filter(stage => stage.name !== 'javascript-suite').map(stage => [stage.name, {
      schemaVersion: 1, issue: 195, stage: stage.name, commit, checkpoint: 'pre-merge', toolchainVersions: {},
      groups: Object.fromEntries(stage.groups.map(group => [group, { commands: [`run ${group}`], artifacts: [`logs/${stage.name}.log`], failureLogs: [] }])), error: null,
    }]),
    ...partitionSelections().map(selection => [selection.name, partitionRecord(selection)]),
  ]);
}

function merged(input) { return mergeStageRecords({ checkpoint: 'pre-merge', commit, records: input, javascriptTestGroups: groups }); }

test('the default suite still executes every existing group; selected scope executes exactly one existing file shard', () => {
  assert.deepEqual(javascriptSuiteGroupEntries(null).map(entry => entry.group), Object.keys(TEST_GROUPS));
  assert.ok(javascriptSuiteGroupEntries(null).every(entry => entry.shard === null));
  assert.deepEqual(javascriptSuiteCommandArguments('grammar', null), ['scripts/test-groups.mjs', '--run', 'grammar']);
  for (const selection of partitionSelections()) {
    const parsed = javascriptSuiteSelection(['--javascript-group', selection.group, '--javascript-shard', selection.shard], 'javascript-suite', false, groups);
    assert.deepEqual(parsed, selection);
    assert.deepEqual(javascriptSuiteGroupEntries(parsed), [{ group: selection.group, shard: selection.shard, label: `suite-${selection.name}` }]);
    assert.deepEqual(javascriptSuiteCommandArguments(parsed.group, parsed.shard), ['scripts/test-groups.mjs', '--run', selection.group, '--shard', selection.shard]);
  }
  assert.equal(javascriptSuiteSelection([], null, false, groups), null);
  assert.deepEqual(javascriptSuiteSelection(['--javascript-group', 'translation'], 'javascript-suite', false, groups), javascriptSuitePartition('translation', '1/1', groups));
});

test('unknown, ambiguous, missing or invalid group/shard CLI scope cannot silently execute another scope', () => {
  const reject = (args, stage = 'javascript-suite', aggregate = false) => assert.throws(() => javascriptSuiteSelection(args, stage, aggregate, groups));
  reject(['--javascript-shard', '1/3']);
  reject(['--javascript-group']);
  reject(['--javascript-group', '--javascript-shard', '1/3']);
  reject(['--javascript-group', 'unknown']);
  reject(['--javascript-group', '__proto__']);
  reject(['--javascript-group', 'grammar', '--javascript-group', 'translation']);
  reject(['--javascript-group', 'grammar', '--javascript-shard', '1/1', '--javascript-shard', '1/1']);
  for (const shard of ['0/3', '4/3', '1/0', '1/999999', '1.5/3', '1/2x']) reject(['--javascript-group', 'grammar-native', '--javascript-shard', shard]);
  reject(['--javascript-group', 'grammar'], null);
  reject(['--javascript-group', 'grammar'], 'rust-suite');
  reject(['--javascript-group', 'grammar'], 'javascript-suite', true);
});

test('physical records and observation files are disjoint and their union retains every current test file once', () => {
  const selections = partitionSelections();
  assert.equal(selections.length, Object.keys(TEST_GROUPS).length + 3);
  const paths = selections.flatMap(selection => Object.values(stageFiles(selection.name)));
  assert.equal(new Set(paths).size, selections.length * 2);
  const files = selections.flatMap(selection => selection.files);
  assert.equal(new Set(files).size, files.length);
  assert.deepEqual([...files].sort(), Object.values(groups).flat().sort());
});

test('the real LiNo partition records produce the complete original logical suite and translation groups', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-evidence-partitions-'));
  try {
    const input = records();
    for (const [name, record] of input) {
      const filename = path.join(directory, `${name}.lino`);
      writeProjectData(filename, record, { format: 'lino' });
      input.set(name, readProjectData(filename, { format: 'lino' }));
    }
    const result = merged(input);
    assert.deepEqual(result.stageErrors, []);
    const suite = result.groups.get('suite:javascript');
    assert.equal(suite.commands.length, partitionSelections().length);
    assert.equal(suite.artifacts.length, partitionSelections().length);
    assert.equal(translationGroups(result.groups).size, 8);
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test('missing, mixed, duplicate, foreign or stale physical LiNo scope never supplies a complete logical suite', async () => {
  const first = 'javascript-suite-grammar-native-1-of-3';
  const mutations = [
    input => input.delete('javascript-suite-grammar-native-2-of-3'),
    input => { const selection = javascriptSuitePartition('grammar-native', '1/2', groups); input.set(selection.name, partitionRecord(selection)); },
    input => input.set(`${first}-duplicate`, { ...input.get(first), stage: `${first}-duplicate` }),
    input => { input.get(first).javascriptPartition.files = [...input.get(first).javascriptPartition.files, 'foreign.test.js']; },
    input => { input.get(first).javascriptPartition.files = input.get(first).javascriptPartition.files.slice(1); },
    input => { delete input.get(first).javascriptPartition.shard; },
    input => { input.get(first).javascriptPartition.group = 'translation'; },
    input => { input.get(first).commit = 'b'.repeat(40); },
    input => { input.get(first).checkpoint = 'post-merge'; },
    input => { input.get(first).stage = 'javascript-suite'; },
    input => { input.get(first).schemaVersion = 2; },
    input => { input.get(first).issue = 199; },
    input => { input.get(first).groups['suite:rust'] = input.get(first).groups['suite:javascript']; },
    input => { input.get(first).groups['suite:javascript'].commands = ['node scripts/test-groups.mjs --run translation']; },
    input => { input.get(first).toolchainVersions.node = 'v22.0.0'; },
    input => input.set(first, new SyntaxError('damaged partition record')),
    input => { input.get(first).error = 'actual JavaScript group command exited 1; see its log'; },
    input => input.set('javascript-suite', { ...input.get(first), stage: 'javascript-suite' }),
  ];
  const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-evidence-invalid-partitions-'));
  try {
    for (const mutate of mutations) {
      const input = records();
      mutate(input);
      for (const [name, record] of input) {
        const filename = path.join(directory, `${name}.lino`);
        if (record instanceof Error) {
          await writeFile(filename, 'invalid LiNo stage output');
          assert.throws(() => readProjectData(filename, { format: 'lino' }));
        } else {
          writeProjectData(filename, record, { format: 'lino' });
          input.set(name, readProjectData(filename, { format: 'lino' }));
        }
      }
      const result = merged(input);
      assert.equal(result.groups.has('suite:javascript'), false, String(mutate));
      assert.equal(result.stageErrors.length, 1, String(mutate));
      assert.equal(result.stageErrors[0].stage, 'javascript-suite');
      assert.ok(result.stageErrors[0].error.length > 0);
      assert.equal([...translationGroups(result.groups).keys()].some(key => key.endsWith(':javascript')), false);
    }
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test('a missing Rust stage still joins its incomplete whole JavaScript matrix gate error', () => {
  const input = records();
  input.delete('javascript-suite-conformance-2-of-2');
  input.delete('rust-suite');
  const result = merged(input);
  assert.equal(result.stageErrors.length, 1);
  assert.match(result.stageErrors[0].error, /rust-suite did not run, as it runs after javascript-suite/u);
});

test('the real observation reader retains all physical LiNo scopes and rejects repeated or unknown callbacks', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-evidence-observations-'));
  try {
    await mkdir(path.join(directory, 'work'));
    const selections = partitionSelections();
    const input = new Map(selections.map(selection => [selection.name, partitionRecord(selection)]));
    const observations = selections.map((selection, index) => ({ testId: `existing-test-${index}`, assertionId: 'existing-assertion', fixtureId: 'existing-fixture',
      fixtureDigest: 'b'.repeat(64), runtime: 'javascript', commit, outcome: 'passed', testName: selection.name }));
    for (const [index, selection] of selections.entries()) await writeFile(path.join(directory, stageFiles(selection.name).observations), structuredDataRecordToLino(observations[index]));
    const stageErrors = [];
    const read = () => readEvidenceStageExecutionRecords({ records: input, stageNames: evidenceStages('pre-merge').map(stage => stage.name),
      knownTestIds: observations.map(record => record.testId), resultsDirectory: directory, stageErrors });
    assert.deepEqual(await read(), observations);
    assert.deepEqual(stageErrors, []);
    await writeFile(path.join(directory, stageFiles(selections[1].name).observations), structuredDataRecordToLino(observations[0]));
    assert.equal((await read()).length, selections.length - 1);
    assert.match(stageErrors[0].error, /duplicate execution record/u);
    stageErrors.length = 0;
    await writeFile(path.join(directory, stageFiles(selections[1].name).observations), structuredDataRecordToLino({ ...observations[1], testId: 'foreign-cell' }));
    assert.equal((await read()).length, selections.length - 1);
    assert.match(stageErrors[0].error, /unknown testId foreign-cell/u);
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test('actual failure log tails are bounded, preserve diagnostics and redact credentials and signed URLs', async () => {
  const environment = { GH_TOKEN: 'sensitive-github-value', AWS_SECRET_ACCESS_KEY: 'sensitive-cloud-value' };
  const source = 'sensitive-github-value sensitive-cloud-value ghp_token npm_token Bearer credential https://user:password@example.test/log?X-Amz-Credential=abc&X-Amz-Signature=secret#token';
  const redacted = redactEvidenceCommandLog(source, environment);
  for (const secret of ['sensitive-github-value', 'sensitive-cloud-value', 'ghp_token', 'npm_token', 'Bearer credential', 'password', 'X-Amz-Signature', 'secret#token']) assert.equal(redacted.includes(secret), false, secret);
  const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-evidence-log-'));
  try {
    const filename = path.join(directory, 'actual-command.log');
    await writeFile(filename, `${'previous output\n'.repeat(6000)}${source}\nnot ok 17 - actual existing test\nexit: 1\n`);
    const tail = await readEvidenceCommandFailureTail(filename, environment);
    assert.ok(tail.length <= 8000);
    assert.ok(tail.split('\n').length <= 40);
    assert.match(tail, /not ok 17 - actual existing test\nexit: 1/u);
    assert.equal(tail.includes('sensitive-github-value'), false);
    assert.ok((await readFile(filename, 'utf8')).includes('sensitive-github-value'), 'original artifact remains untouched');
    await writeFile(filename, 'sensitive-github-value'.repeat(4000));
    assert.equal(await readEvidenceCommandFailureTail(filename, environment), '');
  } finally { await rm(directory, { recursive: true, force: true }); }
});
