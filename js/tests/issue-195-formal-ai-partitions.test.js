// PRIVATE REVIEW DRAFT. Synthetic protocol/evidence controls only; no native
// process, external checkout, install or workload execution occurs here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtemp, mkdir, readFile, rm, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { structuredDataToLino } from '../src/structured-data-links.js';
import { structuredDataRecordToLino } from '../src/structured-data-records.js';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import {
  formalAiPartitionExpectations,
  mergeFormalAiPartitionBundles,
  readFormalAiPartitionBundle,
  runFormalAiNativePartition,
} from '../scripts/formal-ai-partition-bundles.mjs';
import {
  FORMAL_AI_WORKLOAD_ASSERTIONS,
  FORMAL_AI_WORKLOAD_FIXTURE,
  countOutcomes,
  loadFormalAiWorkloadFixture,
  validateFormalAiWorkloadReport,
  workloadFiles,
} from '../scripts/issue-195-formal-ai-workloads.mjs';
import { TEST_NAME_SEPARATOR } from '../scripts/issue-195-formal-ai-consumer.mjs';
import { compareRuntimeOutputs } from '../scripts/issue-195-formal-ai-workload-probes.mjs';
import {
  formalAiWorkloadLogicalFilters,
  formalAiWorkloadPartitionCommands,
  mergeFormalAiWorkloadPartitions,
  readFormalAiNativeExecution,
  readFormalAiNativeInventory,
  selectFormalAiWorkloadPartition,
  validateFormalAiWorkloadPartition,
} from '../scripts/formal-ai-workload-partitions.mjs';

const fixture = loadFormalAiWorkloadFixture();
const digest = (value) => createHash('sha256').update(value).digest('hex');
const targets = ['unit', 'source', 'issue_195_meta_language_probe'];
const ratchet = 'issue_1138_rust_projection::grammar_projection_corpus_ratchet';
const probes = ['issue_195_probe_outputs', 'issue_195_probe_distinctions'];
const nativeFiles = fixture.workloads.filter(({ kind, file }) => kind === 'test' && file.startsWith('rust/'));
const dataFiles = workloadFiles(fixture, 'javascript');

function inventoryRuns() {
  // The mock oracle is authored without the production partition selector.
  const identities = [{ target: 'unit', name: ratchet }];
  for (let index = 0; index < 133; index += 1) {
    const owner = nativeFiles[index % nativeFiles.length];
    identities.push({ target: owner.target, name: `${owner.filter}synthetic_${index}` });
  }
  identities.push(...probes.map((name) => ({ target: 'issue_195_meta_language_probe', name })));
  return targets.map((target) => {
    const names = identities.filter((entry) => entry.target === target).map(({ name }) => name);
    return { target, exit: 0, harnessArguments: [...formalAiWorkloadLogicalFilters(fixture), '--list'],
      transcript: `${names.map((name) => `${name}: test`).join('\n')}\n${names.length} tests, 0 benchmarks\n` };
  });
}

function executionRuns(inventory, partition) {
  const assigned = inventory.filter((entry) => entry.probe ||
    (partition === 'corpus-ratchet' ? entry.target === 'unit' && entry.name === ratchet : !(entry.target === 'unit' && entry.name === ratchet)));
  return targets.flatMap((target) => {
    const names = assigned.filter((entry) => entry.target === target).map(({ name }) => name);
    return names.length ? [{ target, exit: 0,
      harnessArguments: ['--test-threads=1', '--include-ignored', '--exact', ...names],
      transcript: names.map((name) => `test ${name} ... ok`).join('\n') + '\n' }] : [];
  });
}

function passingPair() {
  const expected = {
    identity: { repository: 'link-foundation/meta-language', commit: 'b'.repeat(40), run: '123', attempt: '2',
      fixtureSha256: 'c'.repeat(64), semanticProjectionFixtureSha256: 'd'.repeat(64),
      sourceSha256: { runner: 'e'.repeat(64), partition: 'f'.repeat(64), probe: 'a'.repeat(64) } },
    artifacts: { 'corpus-ratchet': 'part-corpus-ratchet-bbb-run123-attempt2', remaining: 'part-remaining-bbb-run123-attempt2' },
    candidates: { npm: { file: 'meta-language-1.2.3.tgz', sha256: '1'.repeat(64), integrity: 'sha512-exact' },
      crate: { file: 'meta-language-1.2.3.crate', sha256: '2'.repeat(64) } },
    profile: { CARGO_BUILD_JOBS: '2', CARGO_INCREMENTAL: '0', CARGO_PROFILE_DEV_DEBUG: '0',
      CARGO_PROFILE_TEST_DEBUG: '0', RUST_TEST_THREADS: '1', patchedCrateOptLevel: 3 },
    platform: 'linux-x64', nativeWorkloads: 134, javascriptWorkloads: 148, producerResult: 'success',
  };
  const listed = inventoryRuns();
  const inventory = readFormalAiNativeInventory(listed, fixture);
  const javascriptTests = Array.from({ length: 148 }, (_, index) => ({
    file: dataFiles[index % dataFiles.length], name: `synthetic ${index}`, outcome: 'passed',
  }));
  const outputs = { documents: {}, queries: {}, linkEdits: {}, projections: {}, ruleSet: {}, seedNetwork: {} };
  const comparison = compareRuntimeOutputs(outputs, outputs);
  const checked = { name: 'actual synthetic process check', holds: true, detail: {} };
  const assertion = () => ({ holds: true, checks: [structuredClone(checked)] });
  const inputs = { documents: [], labels: ['lino'], rules: [], linkEdits: [], projections: [], semanticProjections: [] };
  const pair = ['corpus-ratchet', 'remaining'].map((partition) => {
    const files = new Map();
    const retain = (file, text) => {
      const bytes = Buffer.from(text);
      files.set(file, bytes);
      return { file, bytes: bytes.length, sha256: digest(bytes) };
    };
    const data = (file, value) => retain(file, structuredDataToLino(value));
    const nativeExecution = executionRuns(inventory, partition);
    const selected = inventory.filter((entry) => entry.probe ||
      (partition === 'corpus-ratchet' ? entry.target === 'unit' && entry.name === ratchet : !(entry.target === 'unit' && entry.name === ratchet)));
    const rows = (isProbe) => selected.filter(({ probe }) => probe === isProbe).map(({ file, name }) => ({ file, name, outcome: 'passed' }));
    const javascriptProbe = { outputs, sharedConceptsReused: assertion(), distinctionsPreserved: assertion(), semanticProjectionObservations: [] };
    const compiler = 'rustc 1.99.0\nrelease: 1.99.0\nhost: x86_64-unknown-linux-gnu\n';
    const profileEnvironment = { CARGO_PROFILE_DEV_DEBUG: '0', CARGO_PROFILE_TEST_DEBUG: '0' };
    const configuration = ['--config', `patch.crates-io.meta-language.path=${JSON.stringify(`/different-${partition}/crate`)}`,
      '--config', 'profile.dev.package.meta-language.opt-level=3'];
    const evidence = {
      nativeInventory: listed.map((run) => ({ ...run, transcript: retain(`list-${run.target}.log`, run.transcript) })),
      nativeExecution: nativeExecution.map((run) => ({ ...run, transcript: retain(`execute-${run.target}.log`, run.transcript) })),
      javascriptEvents: retain('javascript-test-events.lino', javascriptTests.map(({ file, name, outcome }) => ({
        kind: 'test', name: `${file}${TEST_NAME_SEPARATOR}${name}`, outcome,
      })).map((event) => structuredDataRecordToLino(event)).join('')),
      javascriptProbe: { file: data('javascript-probe.lino', javascriptProbe), exit: 0 },
      javascriptOutputs: data('javascript-outputs.lino', outputs),
      rustOutputs: data('rust-outputs.lino', outputs),
      rustDistinctions: data('distinctionsPreserved.lino', { checks: [checked] }),
      inputs: data('formal-ai-inputs.lino', inputs),
      cargoLock: retain('post-update-Cargo.lock', 'pinned actual Cargo.lock bytes\n'),
      compiler: retain('rust-compiler-identity.txt', compiler),
      compilerProfile: data('compiler-profile.lino', { profile: expected.profile, environment: profileEnvironment, configuration }),
      selfAstBefore: data('self-ast-before.lino', [['a.lino', 'source exact\n    total_link_count 1\n']]),
      selfAstAfter: data('self-ast-after.lino', [['a.lino', 'source exact\n    total_link_count 2\n']]),
    };
    const result = {
      schemaVersion: 1, fixture: FORMAL_AI_WORKLOAD_FIXTURE, fixtureSha256: expected.identity.fixtureSha256,
      platform: expected.platform, node: 'v24.0.0',
      formalAi: { repository: fixture.repository, revision: fixture.revision, checkedOutRevision: fixture.revision,
        verifiedBlobs: fixture.workloads.map(({ file, blob }) => ({ file, actual: blob, expected: blob, matches: true })) },
      comparison,
      inputs: { file: evidence.inputs.file, sha256: evidence.inputs.sha256, documents: 0, labels: ['lino'], rules: 0,
        linkEdits: 0, projections: 0, semanticProjections: 0, semanticProjectionFixtureSha256: expected.identity.semanticProjectionFixtureSha256 },
      npm: { skipped: false, version: '1.2.3', checksum: { artifact: expected.candidates.npm.file,
        sha256: expected.candidates.npm.sha256, expectedSha256: expected.candidates.npm.sha256,
        expectedIntegrity: expected.candidates.npm.integrity, installedIntegrity: expected.candidates.npm.integrity },
        resolution: { installedVersion: '1.2.3', lockResolved: `file:/consumer/${partition}/artifact.tgz`, freshCache: true, consumerHadNoModules: true },
        tests: structuredClone(javascriptTests), counts: countOutcomes(javascriptTests), exit: 0,
        workloadFiles: dataFiles, outputs: { file: evidence.javascriptOutputs.file, sha256: evidence.javascriptOutputs.sha256 },
        semanticProjectionObservations: [], probe: {
          sharedConceptsReused: { holds: true, checks: [checked, ...comparison] }, distinctionsPreserved: assertion(),
        } },
      crate: { skipped: false, version: '1.2.3', checksum: { artifact: expected.candidates.crate.file,
        sha256: expected.candidates.crate.sha256, expectedSha256: expected.candidates.crate.sha256 },
        resolution: { installedVersion: '1.2.3', unpackedDirectory: `/different-${partition}/crate/`,
          manifestPath: `/different-${partition}/crate/Cargo.toml`, metadataSource: null, lockSource: null,
          metaLanguagePackages: 1, lockRefresh: { exit: 0 }, patch: configuration.join(' ') },
        filters: formalAiWorkloadLogicalFilters(fixture), workloadFiles: workloadFiles(fixture, 'rust'),
        tests: rows(false), counts: countOutcomes(rows(false)), probeTests: rows(true), exit: 0,
        outputs: { file: evidence.rustOutputs.file, sha256: evidence.rustOutputs.sha256 },
        selfAstCensus: { exit: 0, documents: 1, linkTotals: [{ file: 'a.lino', before: 1, after: 2 }], otherChanges: [] },
        probe: {
          sharedConceptsReused: { holds: true, test: probes[0], outcome: 'passed', checks: [checked, ...comparison] },
          distinctionsPreserved: { holds: true, test: probes[1], outcome: 'passed', checks: [checked] },
        } },
    };
    return { envelope: { kind: 'formal-ai-workload-partition', partitionSchemaVersion: 1,
      identity: { ...expected.identity, partition, artifact: expected.artifacts[partition] },
      captureErrors: [], build: { platform: expected.platform, profile: expected.profile,
        profileEnvironment, compiler,
        cargoLockSha256: evidence.cargoLock.sha256 }, inventory, evidence, result }, files };
  });
  return { expected, pair };
}

test('formal AI partitions preserve 134 target-qualified workloads, both probes and one 148-test JavaScript meaning', () => {
  const { expected, pair } = passingPair();
  for (const part of pair) {
    const result = validateFormalAiWorkloadPartition(part.envelope, fixture, expected, part.files);
    assert.deepEqual(result.problems, []);
    for (const runtime of ['javascript', 'rust']) assert.deepEqual(validateFormalAiWorkloadReport(part.envelope, fixture, runtime).observed, []);
  }
  const result = mergeFormalAiWorkloadPartitions(pair, fixture, expected);
  assert.deepEqual(result.problems, []);
  assert.equal(result.report.crate.tests.length, 134);
  assert.equal(result.report.npm.tests.length, 148);
  assert.equal(result.report.crate.probeTests.length, 2);
  for (const runtime of ['javascript', 'rust']) assert.deepEqual(validateFormalAiWorkloadReport(result.report, fixture, runtime).observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS]);
  assert.deepEqual(pair.map(({ envelope }) => envelope.result.crate.tests.length), [1, 133]);
});

test('formal AI exact-target execution refuses missing repeated unknown failed ignored and changed argv rows', () => {
  const inventory = readFormalAiNativeInventory(inventoryRuns(), fixture);
  const runs = executionRuns(inventory, 'remaining');
  assert.equal(readFormalAiNativeExecution(runs, inventory, 'remaining').length, 135);
  for (const mutate of [
    (copy) => { copy[0].transcript = copy[0].transcript.split('\n').slice(1).join('\n'); },
    (copy) => { copy[0].transcript += copy[0].transcript.split('\n')[0] + '\n'; },
    (copy) => { copy[0].transcript += 'test unrelated::invented ... ok\n'; },
    (copy) => { copy[0].transcript = copy[0].transcript.replace(' ... ok', ' ... FAILED'); },
    (copy) => { copy[0].transcript = copy[0].transcript.replace(' ... ok', ' ... ignored'); },
    (copy) => { copy[0].exit = 1; },
    (copy) => { copy[0].harnessArguments.pop(); },
    (copy) => { copy.push(structuredClone(copy[0])); },
  ]) {
    const copy = structuredClone(runs); mutate(copy);
    assert.throws(() => readFormalAiNativeExecution(copy, inventory, 'remaining'));
  }
  assert.ok(formalAiWorkloadPartitionCommands(inventory, 'corpus-ratchet').every(({ harnessArguments }) => harnessArguments.length > 3));
  assert.equal(selectFormalAiWorkloadPartition(inventory, 'corpus-ratchet').filter(({ probe }) => !probe).length, 1);
});

test('formal AI target-qualified inventories retain identical test names in different binaries and reject ambiguous owners', () => {
  const smallFixture = { workloads: [
    { kind: 'test', file: 'rust/tests/unit/example.rs', target: 'unit', filter: 'shared::' },
    { kind: 'test', file: 'rust/tests/unit/ratchet.rs', target: 'unit', filter: 'issue_1138_rust_projection::' },
    { kind: 'test', file: 'rust/tests/source/example.rs', target: 'source', filter: 'shared::' },
  ] };
  const names = [[ratchet, 'shared::same'], ['shared::same'], probes];
  const runs = targets.map((target, index) => ({ target, exit: 0,
    harnessArguments: [...formalAiWorkloadLogicalFilters(smallFixture), '--list'],
    transcript: names[index].map((name) => `${name}: test`).join('\n') + `\n${names[index].length} tests, 0 benchmarks\n` }));
  const inventory = readFormalAiNativeInventory(runs, smallFixture);
  assert.equal(inventory.filter(({ name }) => name === 'shared::same').length, 2);
  const duplicate = structuredClone(runs);
  duplicate[0].transcript = duplicate[0].transcript.replace('2 tests, 0 benchmarks', 'shared::same: test\n3 tests, 0 benchmarks');
  assert.throws(() => readFormalAiNativeInventory(duplicate, smallFixture), /duplicate/u);
  const ambiguous = structuredClone(smallFixture);
  ambiguous.workloads.push({ ...ambiguous.workloads[0], file: 'rust/tests/unit/other.rs' });
  const changed = structuredClone(runs);
  for (const run of changed) run.harnessArguments = [...formalAiWorkloadLogicalFilters(ambiguous), '--list'];
  assert.throws(() => readFormalAiNativeInventory(changed, ambiguous), /2 fixture owners/u);
});

test('formal AI merger refuses absent duplicate stale foreign compiler profile lock and source facts', () => {
  const { expected, pair } = passingPair();
  for (const parts of [[], [pair[0]], [pair[0], pair[0]], [...pair, pair[1]]]) {
    const result = mergeFormalAiWorkloadPartitions(parts, fixture, expected);
    assert.ok(result.problems.length); assert.equal(result.report, null);
  }
  for (const mutate of [
    (copy) => { copy[0].envelope.identity.commit = '0'.repeat(40); },
    (copy) => { copy[0].envelope.identity.run = 'old-run'; },
    (copy) => { copy[0].envelope.identity.attempt = '1'; },
    (copy) => { copy[0].envelope.identity.artifact = copy[1].envelope.identity.artifact; },
    (copy) => { copy[0].envelope.identity.sourceSha256.runner = '0'.repeat(64); },
    (copy) => { copy[0].envelope.build.compiler += 'different compiler identity\n'; },
    (copy) => { copy[0].envelope.build.profile.RUST_TEST_THREADS = '2'; },
    (copy) => { copy[0].envelope.build.profileEnvironment.RUSTFLAGS = '--cfg another_profile'; },
    (copy) => { copy[0].envelope.build.cargoLockSha256 = '0'.repeat(64); },
    (copy) => { copy[0].envelope.result.crate.resolution.manifestPath += '-sibling'; },
  ]) {
    const copy = structuredClone(pair); mutate(copy);
    const result = mergeFormalAiWorkloadPartitions(copy, fixture, expected);
    assert.ok(result.problems.length); assert.equal(result.report, null);
  }
  assert.equal(mergeFormalAiWorkloadPartitions(pair, fixture, { ...expected, producerResult: 'failure' }).report, null);
});

test('formal AI independent bundle validation refuses tampered inputs outputs events and undeclared paths before publication', () => {
  for (const field of ['inputs', 'javascriptOutputs', 'rustOutputs', 'javascriptEvents', 'cargoLock', 'selfAstAfter']) {
    const { expected, pair } = passingPair();
    const descriptor = pair[0].envelope.evidence[field];
    pair[0].files.set(descriptor.file, Buffer.from('changed bytes'));
    const result = mergeFormalAiWorkloadPartitions(pair, fixture, expected);
    assert.ok(result.problems.some((problem) => problem.includes('tampered'))); assert.equal(result.report, null);
  }
  // Re-signing a changed descriptor cannot substitute a different dependency
  // graph, execution identity or semantic result for independently equal data.
  for (const field of ['cargoLock', 'javascriptEvents', 'nativeExecution', 'selfAstAfter']) {
    const { expected, pair } = passingPair();
    const first = pair[0];
    const descriptor = field === 'nativeExecution'
      ? first.envelope.evidence.nativeExecution[0].transcript : first.envelope.evidence[field];
    const text = Buffer.from(first.files.get(descriptor.file)).toString('utf8');
    const changed = field === 'cargoLock' ? text + '# another dependency graph\n'
      : field === 'nativeExecution' ? text.replace(ratchet, 'unknown::invented')
        : field === 'selfAstAfter' ? structuredDataToLino([['a.lino', 'source changed\n    total_link_count 2\n']])
          : structuredDataRecordToLino({ kind: 'test', name: `${dataFiles[0]}${TEST_NAME_SEPARATOR}invented`, outcome: 'passed' });
    const bytes = Buffer.from(changed);
    first.files.set(descriptor.file, bytes);
    descriptor.bytes = bytes.length; descriptor.sha256 = digest(bytes);
    if (field === 'cargoLock') first.envelope.build.cargoLockSha256 = descriptor.sha256;
    const result = mergeFormalAiWorkloadPartitions(pair, fixture, expected);
    assert.ok(result.problems.length, field); assert.equal(result.report, null, field);
  }
  for (const file of ['../escape', '/absolute', 'nested/../escape', 'windows\\escape', 'unused.log']) {
    const { expected, pair } = passingPair();
    pair[0].files.set(file, Buffer.from('undeclared'));
    assert.equal(mergeFormalAiWorkloadPartitions(pair, fixture, expected).report, null);
  }
});

test('formal AI connected partition transport lists every original target and collects all exact execution failures without native processes', async () => {
  const workDirectory = await mkdtemp(path.join(tmpdir(), 'meta-language-formal-ai-partition-'));
  try {
    for (const partition of ['corpus-ratchet', 'remaining']) {
      const state = {};
      const calls = [];
      const listed = inventoryRuns();
      const inventory = readFormalAiNativeInventory(listed, fixture);
      const executions = executionRuns(inventory, partition);
      const result = await runFormalAiNativePartition({
        partition, fixture, configuration: ['--config', 'patch=exact'], directory: workDirectory,
        environment: { RUST_TEST_THREADS: '2', CARGO_BUILD_JOBS: '2' }, workDirectory, state,
        run: async (label, command, args, options) => {
          calls.push({ command, args, options });
          const target = args[args.indexOf('--test') + 1];
          const listing = args.includes('--list');
          const run = (listing ? listed : executions).find((entry) => entry.target === target);
          const log = path.join(workDirectory, `${label}.log`);
          await writeFile(log, run.transcript);
          // Fail the first workload target; the selected probe target must still
          // execute and all full logs/actual exit statuses must be retained.
          return { code: listing || target === 'issue_195_meta_language_probe' ? 0 : 7, combined: run.transcript, log };
        },
      });
      assert.equal(result.code, 7);
      assert.equal(calls.filter(({ args }) => args.includes('--list')).length, 3);
      assert.deepEqual(calls.slice(0, 3).map(({ args }) => args[args.indexOf('--test') + 1]), targets);
      assert.ok(calls.at(-1).args.includes('issue_195_meta_language_probe'));
      for (const call of calls) {
        assert.equal(call.command, 'cargo');
        assert.equal(call.args[call.args.indexOf('test') + 1], '--no-fail-fast');
        assert.equal(call.options.env.RUST_TEST_THREADS, '1');
        assert.equal(call.options.allowFailure, true);
      }
      assert.equal(state.nativeExecution.length, executions.length);
      assert.ok(state.executionProblems.length);
      assert.match(await readFile(result.log, 'utf8'), /exit: 7/u);
      assert.equal(result.outcomes, null);
    }
  } finally { await rm(workDirectory, { recursive: true, force: true }); }
});

test('formal AI bundle reader refuses symlinks unknown paths and bounded file byte depth and count exhaustion', async () => {
  const temporary = await mkdtemp(path.join(tmpdir(), 'meta-language-formal-ai-bundle-'));
  try {
    const bundle = path.join(temporary, 'bundle');
    const outside = path.join(temporary, 'outside');
    await mkdir(bundle); await mkdir(outside);
    await writeFile(path.join(outside, 'private'), 'unrelated bytes');
    await symlink(outside, path.join(bundle, 'escape'), process.platform === 'win32' ? 'junction' : 'dir');
    await assert.rejects(readFormalAiPartitionBundle(bundle), /symbolic links/u);
    await rm(path.join(bundle, 'escape'));
    await writeFile(path.join(bundle, 'too-large'), '123456789');
    await assert.rejects(readFormalAiPartitionBundle(bundle, { files: 4, fileBytes: 8, totalBytes: 32, depth: 1 }), /oversized/u);
    await assert.rejects(readFormalAiPartitionBundle(bundle, { files: 4, fileBytes: 16, totalBytes: 8, depth: 1 }), /byte limit/u);
    await writeFile(path.join(bundle, 'second'), 'one');
    await assert.rejects(readFormalAiPartitionBundle(bundle, { files: 1, fileBytes: 16, totalBytes: 32, depth: 1 }), /excessive/u);
    await rm(path.join(bundle, 'too-large')); await rm(path.join(bundle, 'second'));
    await mkdir(path.join(bundle, 'deep', 'deeper'), { recursive: true });
    await assert.rejects(readFormalAiPartitionBundle(bundle, { files: 4, fileBytes: 16, totalBytes: 32, depth: 1 }), /nesting/u);
  } finally { await rm(temporary, { recursive: true, force: true }); }
});

test('formal AI source-only connected merger authenticates its own candidates and context before publishing the unchanged full report', async () => {
  const temporary = await mkdtemp(path.join(tmpdir(), 'meta-language-formal-ai-merge-'));
  try {
    const root = fileURLToPath(new URL('../../', import.meta.url));
    const tarball = path.join(temporary, 'meta-language-1.2.3.tgz');
    const crate = path.join(temporary, 'meta-language-1.2.3.crate');
    for (const file of [tarball, crate]) {
      await writeFile(file, `synthetic candidate ${path.basename(file)}`);
      await writeFile(`${file}.sha256`, `${digest(await readFile(file))}  ${path.basename(file)}\n`);
    }
    const environment = { CI: 'true', GITHUB_ACTIONS: 'true', GITHUB_REPOSITORY: 'link-foundation/meta-language',
      GITHUB_SHA: 'b'.repeat(40), GITHUB_RUN_ID: '123', GITHUB_RUN_ATTEMPT: '2' };
    const expected = await formalAiPartitionExpectations({ root, tarball, crate, fixture, environment });
    const { pair } = passingPair();
    const directories = [];
    for (const part of pair) {
      const { envelope } = part;
      const partition = envelope.identity.partition;
      envelope.identity = { ...expected.identity, partition, artifact: expected.artifacts[partition] };
      envelope.build.profile = structuredClone(expected.profile);
      envelope.build.platform = expected.platform;
      envelope.result.platform = expected.platform;
      envelope.result.fixtureSha256 = expected.identity.fixtureSha256;
      envelope.result.inputs.semanticProjectionFixtureSha256 = expected.identity.semanticProjectionFixtureSha256;
      for (const runtime of ['npm', 'crate']) {
        envelope.result[runtime].checksum.artifact = expected.candidates[runtime].file;
        envelope.result[runtime].checksum.sha256 = expected.candidates[runtime].sha256;
        envelope.result[runtime].checksum.expectedSha256 = expected.candidates[runtime].sha256;
      }
      envelope.result.npm.checksum.installedIntegrity = expected.candidates.npm.integrity;
      envelope.result.npm.checksum.expectedIntegrity = expected.candidates.npm.integrity;
      const directory = path.join(temporary, partition);
      await mkdir(directory); directories.push(directory);
      for (const [file, bytes] of part.files) await writeFile(path.join(directory, file), bytes);
      writeProjectData(path.join(directory, 'partition.lino'), envelope, { format: 'lino' });
    }
    const workDirectory = path.join(temporary, 'merged');
    const output = path.join(temporary, 'public', 'report.lino');
    const options = { directories, root, tarball, crate, fixture, workDirectory, output, producerResult: 'success', environment };
    const result = await mergeFormalAiPartitionBundles(options);
    assert.deepEqual(result.problems, []);
    const report = readProjectData(output);
    assert.equal(report.crate.counts.total, 134); assert.equal(report.npm.counts.total, 148);
    for (const runtime of ['javascript', 'rust']) assert.deepEqual(validateFormalAiWorkloadReport(report, fixture, runtime).observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS]);
    const provenance = readProjectData(path.join(workDirectory, 'partition-provenance.lino'));
    assert.equal(provenance.producers.length, 2);
    for (const producer of provenance.producers) {
      const directory = directories.find((value) => value.endsWith(producer.partition));
      assert.equal(producer.envelope.sha256, digest(await readFile(path.join(directory, 'partition.lino'))));
    }
    for (const failure of [
      { producerResult: 'failure' },
      { directories: [directories[0], directories[0]] },
      { directories: [directories[0], path.join(temporary, 'missing')] },
      { environment: { ...environment, GITHUB_RUN_ATTEMPT: '1' } },
    ]) {
      const refused = await mergeFormalAiPartitionBundles({ ...options, ...failure });
      assert.ok(refused.problems.length); assert.equal(refused.report, null);
      const failedReport = readProjectData(output);
      for (const runtime of ['javascript', 'rust']) assert.deepEqual(validateFormalAiWorkloadReport(failedReport, fixture, runtime).observed, []);
    }
    await writeFile(crate, 'changed candidate without changed checksum');
    const refused = await mergeFormalAiPartitionBundles(options);
    assert.ok(refused.problems.some((entry) => entry.includes('candidate checksum')));
    assert.equal(refused.report, null);
  } finally { await rm(temporary, { recursive: true, force: true }); }
});
