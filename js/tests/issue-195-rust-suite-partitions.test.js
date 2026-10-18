import assert from 'node:assert/strict';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { readFileSync, readdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { readEvidenceStageExecutionRecords } from '../scripts/issue-195-evidence-execution-records.mjs';
import { evidenceStages, mergeStageRecords, stageFiles } from '../scripts/issue-195-evidence-stages.mjs';
import { assertRustSuitePartitionEnvironment, mergeRustSuitePartitions, rustSuiteCommandArguments,
  rustSuitePartition, rustSuiteSelection } from '../scripts/issue-195-rust-suite-partitions.mjs';
import { rustEvidencePartitionPolicy } from '../scripts/generate-rust-evidence-partition-policy.mjs';
import { executePhysicalHarness, matchesLogicalFilters, physicalShardCount, readHarnessInventory,
  readPhysicalShardPolicy, readShardAdmission, selectPhysicalTests, verifyHarnessExecution, verifyPhysicalShardUnion } from '../scripts/rust-test-shards.mjs';
import { structuredDataRecordToLino } from '../src/structured-data-records.js';
import { readStructuredDataLino } from '../src/structured-data-links.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const policy = readPhysicalShardPolicy();
const commit = 'a'.repeat(40), tree = 'b'.repeat(40), run = '123', attempt = '4';
const scopes = ['alpha', 'beta', 'gamma', ...policy.evidence.sharedScopes.flat()];
const inventory = readHarnessInventory(scopes.flatMap(scope => [1, 2, 3].map(index => `${scope}::case_${index}: test`)).join('\n'));
const selections = () => Array.from({ length: policy.evidence.shards }, (_, index) => rustSuitePartition(`${index + 1}/${policy.evidence.shards}`));

function fakeEvidenceNative(failure, calls = []) {
  return async (binary, args, listing) => {
    calls.push({ binary, args, listing });
    if (listing) return { stdout: inventory.filter(row => matchesLogicalFilters(row.name, listing === 'all' ? [] : args.slice(0, -3)))
      .map(row => `${row.name}: ${row.kind}\n`).join(''), stderr: '', exitCode: 0, signal: null };
    const selected = inventory.filter(row => args.includes(row.name));
    return { stdout: selected.map(row => `test ${row.name} ... ${row.name === failure ? 'FAILED' : 'ok'}\n`).join(''),
      stderr: 'complete native diagnostics\n', exitCode: selected.some(row => row.name === failure) ? 101 : 0, signal: null };
  };
}

async function nativeEvidenceGroups(failure) {
  const result = [];
  for (const selection of selections()) {
    const identity = { commit, tree, run, attempt, profile: 'evidence', platform: 'ubuntu-latest', suite: 'all' };
    const execution = await executePhysicalHarness({ binary: '/project/rust/target/debug/deps/unit-abcdef01',
      arguments: ['--nocapture'], identity, shard: selection.index, count: selection.count, invoke: fakeEvidenceNative(failure) });
    result.push({ seal: { identity, shard: selection.index, count: selection.count, result: 'success' },
      executions: [execution], expectedBinaries: ['unit'] });
  }
  return result;
}

function partitionRecords() {
  return new Map(selections().map(selection => [selection.name, {
    schemaVersion: 1, issue: 195, stage: selection.name, commit, checkpoint: 'pre-merge', rustPartition: selection,
    toolchainVersions: { node: 'v24.0.0', rustc: 'rustc 1.99.0', platform: 'linux-x64' },
    groups: { 'suite:rust': { commands: [['cargo', ...rustSuiteCommandArguments(root)].join(' ')],
      artifacts: [`issue-195-results/logs/suite-${selection.name}.log`], failureLogs: [] } }, error: null,
  }]));
}

test('Rust evidence partition policy preserves every existing native scope and co-locates observation owners', () => {
  assert.deepEqual(policy.evidence, rustEvidencePartitionPolicy());
  assert.deepEqual(policy.suites.map(row => [row.name, row.shards]), [['grammar', 4], ['inference', 1], ['generative', 1], ['translation', 4], ['remaining', 4]]);
  assert.equal(policy.freshMergeShards, 8);
  assert.equal(physicalShardCount(policy, 'evidence', 'all'), 8);
  assert.throws(() => physicalShardCount(policy, 'evidence', 'translation'));
  const owners = new Map();
  for (const selection of selections()) {
    for (const row of selectPhysicalTests(inventory, selection.index, selection.count, policy.evidence.sharedScopes)) {
      assert.equal(owners.has(row.name), false); owners.set(row.name, selection.index);
    }
  }
  assert.equal(owners.size, inventory.length);
  for (const scope of scopes) assert.equal(new Set([1, 2, 3].map(index => owners.get(`${scope}::case_${index}`))).size, 1);
  for (const shared of policy.evidence.sharedScopes) assert.equal(new Set(shared.map(scope => owners.get(`${scope}::case_1`))).size, 1);
  assert.throws(() => selectPhysicalTests(inventory, 1, 8, [['alpha', 'beta'], ['beta', 'gamma']]), /overlapping shared observation/u);
});

test('Windows translation expands only its physical cells and preserves every old test under the original deadline', () => {
  assert.deepEqual(policy.suites.find(row => row.name === 'translation').platformShards,
    [{platform:'windows-latest',profile:'coverage',shards:8},
      {platform:'windows-latest',profile:'test',shards:16},{platform:'ubuntu-latest',shards:8}]);
  for (const platform of policy.platforms) for (const suite of policy.suites) {
    const expanded = platform === 'windows-latest' && ['translation','remaining'].includes(suite.name)
      || platform === 'ubuntu-latest' && ['grammar','translation'].includes(suite.name)
      || platform === 'macos-latest' && suite.name === 'grammar';
    const expected = platform === 'windows-latest' && suite.name === 'translation' ? 16 : expanded ? 8 : suite.shards;
    assert.equal(physicalShardCount(policy, 'test', suite.name, platform), expected);
  }
  for (const suite of policy.suites) assert.equal(physicalShardCount(policy, 'coverage', suite.name, 'ubuntu-latest'),
    ['grammar','translation'].includes(suite.name) ? 8 : suite.shards);
  assert.equal(physicalShardCount(policy,'coverage','translation','windows-latest'),8);
  for (let previous = 1; previous <= 4; previous++) {
    const before = selectPhysicalTests(inventory, previous, 4).map(row => row.name).sort();
    const after = [...selectPhysicalTests(inventory, previous, 8), ...selectPhysicalTests(inventory, previous + 4, 8)].map(row => row.name).sort();
    assert.deepEqual(after, before, 'each old hash cell is exactly subdivided without losing or duplicating a test');
  }
  const workflow = readFileSync(path.join(root, '.github/workflows/rust.yml'), 'utf8');
  for (const shard of [5, 6, 7, 8]) assert.ok(workflow.includes('            os: windows-latest\n            logical_suite: translation\n            physical_shard: '+shard+'\n'));
  for(let previous=1;previous<=8;previous++)assert.deepEqual(
    [...selectPhysicalTests(inventory,previous,16),...selectPhysicalTests(inventory,previous+8,16)].map(row=>row.name).sort(),
    selectPhysicalTests(inventory,previous,8).map(row=>row.name).sort());
  for(const shard of [9,10,11,12,13,14,15,16])assert.ok(workflow.includes('            os: windows-latest\n            logical_suite: translation\n            physical_shard: '+shard+'\n'));
  const deadline=readStructuredDataLino(readFileSync(path.join(root,'parity/fixtures/native-translation-deadline-2026-10-18.lino'),'utf8'));
  assert.equal(deadline.run,39113660901);assert.equal(deadline.job,117640079261);
  assert.equal(deadline.oldCount,8);assert.equal(deadline.newCount,16);
  assert.equal(deadline.annotations[0].message,'The job has exceeded the maximum execution time of 15m0s');
  assert.ok(workflow.includes("    timeout-minutes: ${{ matrix.os == 'windows-latest' && (matrix.logical_suite || matrix.suite) == 'grammar' && 30 || 15 }}\n"));
  for (const override of [{platform: 'unknown', shards: 8}, {platform: 'windows-latest', shards: 3}, {platform: 'windows-latest', shards: 8.5}]) {
    const altered = structuredClone(policy);altered.suites.find(row => row.name === 'translation').platformShards = [override];
    assert.throws(() => physicalShardCount(altered, 'test', 'translation', 'windows-latest'), /invalid platform shard expansion/u);
  }
});

test('fresh merge physical expansion preserves the complete original eight-cell inventory and deadline',()=>{
  assert.equal(policy.freshMergeShards,8);assert.equal(policy.freshMergeExpansion,16);
  assert.equal(physicalShardCount(policy,'fresh-merge','all'),16);
  for(let previous=1;previous<=8;previous++) {
    const before=selectPhysicalTests(inventory,previous,8).map(row=>row.name).sort();
    const after=[...selectPhysicalTests(inventory,previous,16),...selectPhysicalTests(inventory,previous+8,16)].map(row=>row.name).sort();
    assert.deepEqual(after,before);
  }
  for(const freshMergeExpansion of [7,16.5,NaN])assert.throws(()=>physicalShardCount({...policy,freshMergeExpansion},'fresh-merge','all'),/invalid fresh merge shard expansion/u);
  const workflow=readFileSync(path.join(root,'.github/workflows/rust.yml'),'utf8');
  assert.ok(workflow.includes('        physical_shard: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]\n'));
  assert.match(workflow,/name: Fresh Merge Simulation\n[\s\S]*?timeout-minutes: 20\n/u);
});

test('instrumented grammar expansion subdivides every original coverage cell without changing native test scope',()=>{
  assert.deepEqual(policy.suites.find(row=>row.name==='grammar').platformShards,
    [{platform:'ubuntu-latest',profile:'coverage',shards:8},{platform:'ubuntu-latest',profile:'test',shards:8},
      {platform:'macos-latest',profile:'test',shards:8}]);
  assert.equal(physicalShardCount(policy,'coverage','grammar','ubuntu-latest'),8);
  assert.equal(physicalShardCount(policy,'test','grammar','ubuntu-latest'),8);
  for(const [platform,count] of [['macos-latest',8],['windows-latest',4]])assert.equal(physicalShardCount(policy,'test','grammar',platform),count);
  for(let previous=1;previous<=4;previous++)assert.deepEqual(
    [...selectPhysicalTests(inventory,previous,8),...selectPhysicalTests(inventory,previous+4,8)].map(row=>row.name).sort(),
    selectPhysicalTests(inventory,previous,4).map(row=>row.name).sort());
});

test('Rust evidence CLI scope and configured Cargo runner must agree exactly', () => {
  assert.equal(rustSuiteSelection([], 'rust-suite', false), null);
  for (const selection of selections()) {
    assert.deepEqual(rustSuiteSelection(['--rust-shard', selection.shard], 'rust-suite', false), selection);
    const environment = { META_LANGUAGE_SHARD_PROFILE: 'evidence', META_LANGUAGE_SHARD_PLATFORM: 'ubuntu-latest',
      META_LANGUAGE_SHARD_SUITE: 'all', META_LANGUAGE_SHARD_INDEX: String(selection.index) };
    assertRustSuitePartitionEnvironment(selection, environment);
    for (const key of Object.keys(environment)) assert.throws(() => assertRustSuitePartitionEnvironment(selection, { ...environment, [key]: 'changed' }));
  }
  for (const shard of [undefined, '', '0/8', '9/8', '1/4', '1/999999', '1.5/8', '01/8', '--stage']) {
    assert.throws(() => rustSuiteSelection(['--rust-shard', shard], 'rust-suite', false));
  }
  assert.throws(() => rustSuiteSelection(['--rust-shard', '1/8'], 'javascript-suite', false));
  assert.throws(() => rustSuiteSelection(['--rust-shard', '1/8'], 'rust-suite', true));
  assert.throws(() => rustSuiteSelection(['--rust-shard', '1/8', '--rust-shard', '2/8'], 'rust-suite', false));
});

test('actual native inventories execute every evidence test once with unchanged diagnostic controls', async () => {
  const groups = await nativeEvidenceGroups();
  assert.deepEqual(verifyPhysicalShardUnion(groups, policy, { commit, run, attempt, profiles: ['evidence'] }), { cells: 8, inventories: 1 });
  for (const group of groups) {
    const execution = group.executions[0];
    assert.deepEqual(execution.arguments, ['--nocapture']); verifyHarnessExecution(execution);
  }
  const calls = [];
  const identity = groups[0].seal.identity;
  await executePhysicalHarness({ binary: '/project/unit-abcdef01', arguments: ['--nocapture'], identity,
    shard: 1, count: 8, invoke: fakeEvidenceNative(null, calls) });
  assert.deepEqual(calls[0].args, ['--nocapture', '--list', '--format', 'terse']);
  assert.deepEqual(calls[1].args, ['--list', '--format', 'terse']);
  assert.deepEqual(calls[2].args.slice(0, 2), ['--nocapture', '--exact']);
  assert.throws(() => matchesLogicalFilters('alpha::case', ['--ignored']), /unsupported logical filter option/u);
  const failed = await nativeEvidenceGroups(inventory[0].name);
  assert.throws(() => verifyPhysicalShardUnion(failed, policy, { commit, run, attempt, profiles: ['evidence'] }), /native test execution failed/u);
});

test('complete Rust evidence rejects missing, duplicated, changed or foreign native executions', async () => {
  const valid = await nativeEvidenceGroups();
  const mutations = [
    groups => groups.pop(), groups => groups.push(structuredClone(groups[0])),
    groups => { groups[0].seal.identity.commit = 'c'.repeat(40); },
    groups => { groups[0].seal.identity.run = 'other'; },
    groups => { groups[0].seal.count = 4; },
    groups => { groups[0].seal.result = 'failure'; },
    groups => { groups[0].executions[0].arguments.push('--skip', 'alpha'); },
    groups => { groups[0].executions[0].outcomes.push({ name: 'invented', outcome: 'ok' }); },
    groups => { groups[0].expectedBinaries.push('omitted_binary'); },
  ];
  for (const mutate of mutations) {
    const groups = structuredClone(valid); mutate(groups);
    assert.throws(() => verifyPhysicalShardUnion(groups, policy, { commit, run, attempt, profiles: ['evidence'] }), String(mutate));
  }
  assert.throws(() => verifyPhysicalShardUnion([], policy, { commit, run, attempt, profiles: ['invented'] }), /unknown or duplicate admitted/u);
});

test('complete physical Rust records reconstruct only the original logical suite after native union verification', async () => {
  const groups = await nativeEvidenceGroups();
  let verified = 0;
  const verifyPhysicalUnion = () => { verified += 1; return verifyPhysicalShardUnion(groups, policy, { commit, run, attempt, profiles: ['evidence'] }); };
  const records = partitionRecords();
  const merged = mergeRustSuitePartitions({ checkpoint: 'pre-merge', commit, records, verifyPhysicalUnion });
  assert.equal(verified, 1);
  assert.deepEqual([...merged.keys()], ['rust-suite']);
  const result = merged.get('rust-suite');
  assert.equal(result.error, null);
  assert.equal(result.groups['suite:rust'].commands.length, 1);
  assert.equal(result.groups['suite:rust'].artifacts.length, 8);
  for (const stage of evidenceStages('pre-merge').filter(stage => stage.name !== 'rust-suite')) records.set(stage.name, {
    schemaVersion: 1, issue: 195, stage: stage.name, commit, checkpoint: 'pre-merge', toolchainVersions: {}, error: null,
    groups: Object.fromEntries(stage.groups.map(group => [group, { commands: ['actual fixture producer'], artifacts: ['logs/fixture.log'], failureLogs: [] }])),
  });
  const complete = mergeStageRecords({ checkpoint: 'pre-merge', commit, records, verifyRustPhysicalUnion: verifyPhysicalUnion });
  assert.deepEqual(complete.stageErrors, []);
  assert.equal(complete.groups.has('suite:rust'), true);
});

test('missing or counterfeit Rust stage scopes cannot declare successful suite evidence', async () => {
  const groups = await nativeEvidenceGroups();
  const verifyPhysicalUnion = () => verifyPhysicalShardUnion(groups, policy, { commit, run, attempt, profiles: ['evidence'] });
  const mutations = [
    records => records.delete('rust-suite-2-of-8'),
    records => { records.set('rust-suite-1-of-8-duplicate', structuredClone(records.get('rust-suite-1-of-8'))); },
    records => { records.get('rust-suite-1-of-8').commit = 'c'.repeat(40); },
    records => { records.get('rust-suite-1-of-8').rustPartition.count = 4; },
    records => { records.get('rust-suite-1-of-8').groups['suite:javascript'] = {}; },
    records => { records.get('rust-suite-1-of-8').groups['suite:rust'].commands[0] += ' --skip expensive'; },
    records => { records.get('rust-suite-1-of-8').groups['suite:rust'].artifacts = ['issue-195-results/logs/suite-rust.log']; },
    records => { records.get('rust-suite-1-of-8').toolchainVersions.node = 'v22.0.0'; },
    records => { records.get('rust-suite-1-of-8').error = 'actual native failure'; },
    records => records.set('rust-suite-1-of-8', new Error('malformed Links record')),
    records => { records.set('rust-suite', { ...records.get('rust-suite-1-of-8'), stage: 'rust-suite' }); },
  ];
  for (const mutate of mutations) {
    const records = partitionRecords(); mutate(records);
    const result = mergeRustSuitePartitions({ checkpoint: 'pre-merge', commit, records, verifyPhysicalUnion }).get('rust-suite');
    assert.equal(Object.hasOwn(result.groups, 'suite:rust'), false, String(mutate));
    assert.equal(typeof result.error, 'string');
  }
  const result = mergeRustSuitePartitions({ checkpoint: 'pre-merge', commit, records: partitionRecords(),
    verifyPhysicalUnion: () => { throw new Error('missing authenticated native receipt'); } }).get('rust-suite');
  assert.match(result.error, /authenticated complete native test union failed/u);
});

test('Rust partition observation files remain disjoint and the original duplicate gate stays active', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'meta-language-rust-evidence-observations-'));
  try {
    await mkdir(path.join(directory, 'work'));
    const records = partitionRecords();
    const names = [...records.keys()];
    const files = names.flatMap(name => Object.values(stageFiles(name)));
    assert.equal(new Set(files).size, 16);
    for (let index = 0; index < names.length; index += 1) {
      await writeFile(path.join(directory, stageFiles(names[index]).observations), structuredDataRecordToLino({
        testId: 'cell', assertionId: `assertion-${index}`, fixtureId: 'fixture', fixtureDigest: 'd'.repeat(64),
        runtime: 'rust', commit, outcome: 'passed', testName: `actual_test_${index}`,
      }));
    }
    const stageErrors = [];
    const accepted = await readEvidenceStageExecutionRecords({ records, stageNames: ['rust-suite'], knownTestIds: ['cell'], resultsDirectory: directory, stageErrors });
    assert.equal(accepted.length, 8); assert.deepEqual(stageErrors, []);
    await writeFile(path.join(directory, stageFiles(names[1]).observations), await readFile(path.join(directory, stageFiles(names[0]).observations)));
    const duplicated = [];
    await readEvidenceStageExecutionRecords({ records, stageNames: ['rust-suite'], knownTestIds: ['cell'], resultsDirectory: directory, stageErrors: duplicated });
    assert.match(duplicated[0].error, /duplicate execution record/u);
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test('Rust evidence scheduling retains shared source requirement scopes and exact admitted job identities', () => {
  const owners = new Map();
  for (const file of readdirSync(path.join(root, 'rust/tests/unit')).filter(file => file.endsWith('.rs'))) {
    const source = readFileSync(path.join(root, 'rust/tests/unit', file), 'utf8');
    if (!/\b(?:observations::)?record\s*\(|\bobserve\s*\(/u.test(source)) continue;
    for (const [, requirement] of source.matchAll(/"(I195-[A-Z0-9{}_-]+)"/gu)) {
      const modules = owners.get(requirement) ?? new Set(); modules.add(file.slice(0, -3)); owners.set(requirement, modules);
    }
  }
  for (const [requirement, modules] of owners) {
    if (modules.size < 2) continue;
    assert.equal(policy.evidence.sharedScopes.some(group => [...modules].every(module => group.includes(module))), true, `${requirement}: shared source observation scope remains together`);
  }
  const jobs = selections().map(selection => ({ name: `Evidence Stage (rust-suite - ${selection.shard})`, status: 'completed', conclusion: 'success' }));
  assert.deepEqual(readShardAdmission(jobs, policy), ['evidence']);
  for (const changed of [jobs.slice(1), [...jobs, jobs[0]], jobs.map((job, index) => index ? job : { ...job, name: 'Evidence Stage (rust-suite)' }),
    jobs.map((job, index) => index ? job : { ...job, status: 'in_progress' })]) assert.throws(() => readShardAdmission(changed, policy));
});

test('the Rust evidence workflow preserves its tools, producer gates and original sixty-minute budget', () => {
  const jobs = new Map(parseWorkflow(readFileSync(path.join(root, '.github/workflows/ci.yml'), 'utf8')).map(job => [job.id, job]));
  const job = jobs.get('rust-suite');
  assert.match(job.header, /name: Evidence Stage \(rust-suite - \$\{\{ matrix.physical_shard \}\}\/8\)/u);
  assert.match(job.header, /timeout-minutes: 60\n/u);
  assert.match(job.header, /fail-fast: false\n/u);
  assert.match(job.header, /physical_shard: \[1, 2, 3, 4, 5, 6, 7, 8\]/u);
  assert.match(job.header, /group: issue-195-stage-\$\{\{ github.ref \}\}-rust-suite-\$\{\{ matrix.physical_shard \}\}/u);
  for (const name of ['Setup Node.js', 'Setup Rust', 'Setup Lean', 'Setup OCaml', 'Install Rocq 9.3', 'Install JavaScript dependencies',
    'Provision the pinned external grammar test inputs', 'Download the relative-meta-logic workload report', 'Download the formal-ai workload report',
    'Download the complete self-translation reports']) assert.equal(job.steps.some(step => step.name === name), true, name);
  const producer = job.steps.find(step => step.name === 'Produce the stage evidence');
  assert.match(producer.run, /--rust-shard "\$\{\{ matrix.physical_shard \}\}\/8"/u);
  assert.equal(job.steps.findIndex(step => /rust-test-shards.mjs --configure/u.test(step.run)) < job.steps.indexOf(producer), true);
  assert.equal(job.steps.findIndex(step => /rust-test-shards.mjs --finish/u.test(step.run)) > job.steps.indexOf(producer), true);
  assert.match(job.steps.find(step => step.name === 'Upload the stage outputs').text, /issue-195-stage-rust-suite-\$\{\{ matrix.physical_shard \}\}-of-8-/u);
  const aggregate = jobs.get('acceptance');
  assert.match(aggregate.steps.find(step => step.name === 'Require the complete authenticated Rust test union').run, /RUST_EVIDENCE_RESULT/u);
});
