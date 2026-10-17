// Validate CI workload partitions and merge them into one complete report.
// Only the bundle I/O adapter may populate `files`, from bounded regular files
// rooted in one producer's own directory. No partial is a public report.
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import path from 'node:path';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { readStructuredDataRecords } from '../src/structured-data-records.js';
import {
  FORMAL_AI_WORKLOAD_ASSERTIONS,
  FORMAL_AI_WORKLOAD_FIXTURE,
  compareSelfAstCensus,
  countOutcomes,
  formalAiWorkloadPartitionGroundingProblems,
  validateFormalAiWorkloadReport,
  workloadFiles,
} from './issue-195-formal-ai-workloads.mjs';
import { compareRuntimeOutputs } from './issue-195-formal-ai-workload-probes.mjs';
import { TEST_NAME_SEPARATOR } from './issue-195-formal-ai-consumer.mjs';
import { RUST_PROBE_TARGET, RUST_PROBE_TESTS } from './issue-195-formal-ai-rust-probe.mjs';

export const FORMAL_AI_WORKLOAD_PARTITIONS = Object.freeze(['corpus-ratchet', 'remaining']);
const TARGETS = Object.freeze(['unit', 'source', RUST_PROBE_TARGET]);
const RATCHET = Object.freeze({ target: 'unit', name: 'issue_1138_rust_projection::grammar_projection_corpus_ratchet' });
const PROBES = Object.freeze(Object.values(RUST_PROBE_TESTS));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const key = ({ target, name }) => JSON.stringify([target, name]);
const fail = (message) => { throw new Error(message); };
const equal = (actual, expected, message) => { if (!isDeepStrictEqual(actual, expected)) fail(message); };

/** The original full logical scope remains independent of execution subsets. */
export function formalAiWorkloadLogicalFilters(fixture) {
  return ['--include-ignored', ...new Set([
    ...fixture.workloads.filter(({ kind, file }) => kind === 'test' && file.startsWith('rust/')).map(({ filter }) => filter),
    'issue_195_probe_',
  ])];
}

/** Retain effective compiler/profile overrides, excluding host cache paths. */
export function formalAiRustProfileEnvironment(environment) {
  if (!environment || typeof environment !== 'object' || Array.isArray(environment)) fail('missing actual Rust profile environment');
  const selected = Object.entries(environment).filter(([name]) => /^CARGO_PROFILE_/u.test(name) ||
    /^CARGO_TARGET_.+_(?:RUSTFLAGS|LINKER|RUNNER)$/u.test(name) ||
    ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTDOCFLAGS',
      'CARGO_BUILD_RUSTFLAGS', 'RUSTC', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'RUSTUP_TOOLCHAIN'].includes(name));
  if (selected.some(([, value]) => typeof value !== 'string')) fail('invalid actual Rust profile environment value');
  return Object.fromEntries(selected.sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0));
}

function workloadOwner(target, name, fixture) {
  if (target === RUST_PROBE_TARGET && PROBES.includes(name)) {
    return { target, name, file: `rust/tests/${RUST_PROBE_TARGET}.rs`, probe: true };
  }
  const owners = fixture.workloads.filter((entry) => entry.kind === 'test' && entry.target === target &&
    typeof entry.filter === 'string' && name.startsWith(entry.filter));
  if (owners.length !== 1) fail(`native identity ${key({ target, name })} has ${owners.length} fixture owners`);
  return { target, name, file: owners[0].file, probe: false };
}

function unstyled(text) {
  if (typeof text !== 'string') fail('a retained native transcript must be text');
  return text.replace(/\u001b\[[0-9;]*m/gu, '');
}

/** `runs` are three actual single-target --list invocations, not declarations. */
export function readFormalAiNativeInventory(runs, fixture) {
  if (!Array.isArray(runs) || runs.length !== TARGETS.length) fail('the complete inventory requires every original target once');
  const identities = [];
  const seen = new Set();
  for (const target of TARGETS) {
    const matches = runs.filter((entry) => entry.target === target);
    if (matches.length !== 1) fail(`inventory target ${target} is missing or repeated`);
    const run = matches[0];
    if (run.exit !== 0) fail(`inventory target ${target} did not finish successfully`);
    equal(run.harnessArguments, [...formalAiWorkloadLogicalFilters(fixture), '--list'], `inventory scope changed for ${target}`);
    const lines = unstyled(run.transcript).split(/\r?\n/u);
    let listed = 0;
    for (const line of lines) {
      const entry = /^(.+): (test|benchmark)$/u.exec(line);
      if (!entry) continue;
      if (entry[2] !== 'test') fail('benchmarks are outside the original native workload scope');
      const identity = workloadOwner(target, entry[1], fixture);
      if (seen.has(key(identity))) fail(`duplicate listed native identity ${key(identity)}`);
      seen.add(key(identity));
      identities.push(identity);
      listed += 1;
    }
    const summaries = lines.map((line) => /^(\d+) tests?, (\d+) benchmarks?$/u.exec(line)).filter(Boolean);
    if (summaries.length !== 1 || Number(summaries[0][1]) !== listed || Number(summaries[0][2]) !== 0) {
      fail(`inventory summary does not match actual listed identities for ${target}`);
    }
  }
  for (const name of PROBES) {
    if (!identities.some((entry) => entry.target === RUST_PROBE_TARGET && entry.name === name)) fail(`missing native probe ${name}`);
  }
  if (identities.filter((entry) => entry.target === RATCHET.target && entry.name === RATCHET.name).length !== 1) {
    fail('the corpus ratchet must select exactly one target-qualified original identity');
  }
  for (const file of workloadFiles(fixture, 'rust')) {
    if (!identities.some((entry) => !entry.probe && entry.file === file)) fail(`no listed native workload owns ${file}`);
  }
  return identities;
}

/** No substring Cargo selector participates in partitioning. */
export function selectFormalAiWorkloadPartition(inventory, partition) {
  if (!FORMAL_AI_WORKLOAD_PARTITIONS.includes(partition)) fail(`unknown native partition ${partition}`);
  return inventory.filter((entry) => entry.probe ||
    ((entry.target === RATCHET.target && entry.name === RATCHET.name) === (partition === 'corpus-ratchet')));
}

/** Empty target subsets never launch a harness with an empty --exact filter. */
export function formalAiWorkloadPartitionCommands(inventory, partition) {
  const selected = selectFormalAiWorkloadPartition(inventory, partition);
  return TARGETS.flatMap((target) => {
    const names = selected.filter((entry) => entry.target === target).map(({ name }) => name);
    return names.length === 0 ? [] : [{
      target,
      harnessArguments: ['--test-threads=1', '--include-ignored', '--exact', ...names],
    }];
  });
}

/** Terminal rows must come from each selected actual single-target process. */
export function readFormalAiNativeExecution(runs, inventory, partition) {
  const commands = formalAiWorkloadPartitionCommands(inventory, partition);
  if (!Array.isArray(runs) || runs.length !== commands.length) fail('native execution target set is incomplete or repeated');
  const selected = selectFormalAiWorkloadPartition(inventory, partition);
  const rows = [];
  const seen = new Set();
  for (const command of commands) {
    const matches = runs.filter((entry) => entry.target === command.target);
    if (matches.length !== 1) fail(`execution target ${command.target} is missing or repeated`);
    const run = matches[0];
    equal(run.harnessArguments, command.harnessArguments, `native execution argv changed for ${command.target}`);
    if (run.exit !== 0) fail(`native target ${command.target} exited with ${run.exit}`);
    for (const line of unstyled(run.transcript).split(/\r?\n/u)) {
      const match = /^test (.+?) \.\.\. (ok|FAILED|ignored)(?:\s.*)?$/u.exec(line);
      if (!match) continue;
      // libtest's annotation is removed only after the exact selected identity
      // lookup fails; a genuine name ending in that text remains an exact name.
      let name = match[1];
      let entry = selected.find((item) => item.target === command.target && item.name === name);
      if (!entry && name.endsWith(' - should panic')) {
        name = name.slice(0, -' - should panic'.length);
        entry = selected.find((item) => item.target === command.target && item.name === name);
      }
      if (!entry) fail(`unknown executed native identity ${key({ target: command.target, name })}`);
      if (seen.has(key(entry))) fail(`duplicate executed native identity ${key(entry)}`);
      seen.add(key(entry));
      if (match[2] !== 'ok') fail(`native identity ${key(entry)} did not pass`);
      rows.push({ ...entry, outcome: 'passed' });
    }
  }
  for (const entry of selected) if (!seen.has(key(entry))) fail(`missing executed native identity ${key(entry)}`);
  return rows;
}

function relativeFile(file) {
  return typeof file === 'string' && file.length > 0 && !file.includes('\\') && !file.includes('\0') &&
    !path.posix.isAbsolute(file) && path.posix.normalize(file) === file &&
    !file.split('/').some((part) => part === '.' || part === '..' || part === '');
}

function evidenceReader(files) {
  if (!(files instanceof Map)) fail('a producer bundle requires independently read regular-file bytes');
  const used = new Set();
  return {
    bytes(descriptor) {
      if (!descriptor || !relativeFile(descriptor.file) || !Number.isSafeInteger(descriptor.bytes) || descriptor.bytes < 0 ||
          !/^[0-9a-f]{64}$/u.test(descriptor.sha256 ?? '')) fail('invalid producer-scoped evidence descriptor');
      const bytes = files.get(descriptor.file);
      if (!(bytes instanceof Uint8Array) || bytes.length !== descriptor.bytes || sha256(bytes) !== descriptor.sha256) {
        fail(`missing or tampered producer evidence ${descriptor.file}`);
      }
      used.add(descriptor.file);
      return bytes;
    },
    text(descriptor) { return new TextDecoder('utf-8', { fatal: true }).decode(this.bytes(descriptor)); },
    data(descriptor) { return readStructuredDataLino(this.text(descriptor)); },
    complete() {
      if (used.size !== files.size) fail('the producer bundle contains undeclared or unvalidated files');
    },
  };
}

function reportRows(rows) {
  return rows.map(({ file, name, outcome }) => ({ file, name, outcome }));
}

function nativeRuns(entries, reader) {
  if (!Array.isArray(entries)) fail('missing native process receipts');
  return entries.map((entry) => ({ ...entry, transcript: reader.text(entry.transcript) }));
}

function probeHolds(probe, label) {
  for (const assertion of ['sharedConceptsReused', 'distinctionsPreserved']) {
    const checks = probe?.[assertion]?.checks;
    if (probe?.[assertion]?.holds !== true || !Array.isArray(checks) || checks.length === 0 || checks.some((check) => check.holds !== true)) {
      fail(`${label} did not execute all original ${assertion} checks successfully`);
    }
  }
}

function exactFileDescriptor(actual, retained, label) {
  if (actual?.file !== retained.file || actual?.sha256 !== retained.sha256) fail(`${label} report names different retained bytes`);
}

/**
 * `expected` comes from the merger's own checkout, same-run context and actual
 * candidate files. Never build it from a received envelope. `files` contains
 * bounded regular bytes read below a single authenticated artifact directory.
 */
export function validateFormalAiWorkloadPartition(envelope, fixture, expected, files) {
  try {
    if (envelope?.kind !== 'formal-ai-workload-partition' || envelope.partitionSchemaVersion !== 1 ||
        'schemaVersion' in envelope || 'npm' in envelope || 'crate' in envelope) fail('not a genuine partial envelope');
    if (!Array.isArray(envelope.captureErrors) || envelope.captureErrors.length) fail('the producer could not retain all original evidence');
    const partition = envelope.identity?.partition;
    if (!FORMAL_AI_WORKLOAD_PARTITIONS.includes(partition)) fail('unknown producer partition');
    equal(envelope.identity, { ...expected.identity, partition, artifact: expected.artifacts[partition] }, 'foreign commit/run/attempt/source/candidate/producer identity');
    if (!expected.identity || !expected.artifacts || expected.nativeWorkloads !== 134 || expected.javascriptWorkloads !== 148) {
      fail('missing independently pinned full workload expectations');
    }
    const report = envelope.result;
    const grounding = formalAiWorkloadPartitionGroundingProblems(report, fixture);
    if (grounding.length) fail(grounding.join('\n'));
    equal(report.fixtureSha256, expected.identity.fixtureSha256, 'report fixture differs from the actual pinned bytes');
    equal(report.fixture, FORMAL_AI_WORKLOAD_FIXTURE, 'report logical fixture identity changed');
    equal(report.platform, expected.platform, 'producer platform changed');
    equal(envelope.build.platform, expected.platform, 'build platform changed');
    equal(envelope.build.profile, expected.profile, 'actual compiler profile differs from the original settings');
    equal(envelope.build.profileEnvironment, formalAiRustProfileEnvironment(envelope.build.profileEnvironment), 'unknown or malformed actual compiler profile environment');
    for (const name of ['CARGO_PROFILE_DEV_DEBUG', 'CARGO_PROFILE_TEST_DEBUG']) {
      equal(envelope.build.profileEnvironment[name], expected.profile[name] ?? '0', `actual ${name} differs from the original bounded profile`);
    }
    if (typeof envelope.build.compiler !== 'string' ||
        envelope.build.compiler.split(/\r?\n/u).filter((line) => line === 'release: 1.99.0').length !== 1) fail('missing actual pinned rustc -vV identity');
    for (const [runtime, part] of [['npm', report.npm], ['crate', report.crate]]) {
      const candidate = expected.candidates[runtime];
      equal({ file: part.checksum.artifact, sha256: part.checksum.sha256 },
        { file: candidate.file, sha256: candidate.sha256 }, `${runtime} is not the exact independently downloaded candidate`);
      if (runtime === 'npm') equal(part.checksum.expectedIntegrity, candidate.integrity, 'npm candidate SHA512 differs');
    }
    const root = report.crate.resolution.unpackedDirectory;
    const manifest = report.crate.resolution.manifestPath;
    if (typeof root !== 'string' || !root.endsWith('/') || manifest !== `${root}Cargo.toml`) fail('native package manifest is outside its own unpacked root');
    const reader = evidenceReader(files);
    const evidence = envelope.evidence;
    equal(reader.text(evidence.compiler), envelope.build.compiler, 'actual compiler stdout differs from the build identity');
    const compilerProfile = reader.data(evidence.compilerProfile);
    equal(compilerProfile.profile, envelope.build.profile, 'actual recorded profile differs from the build identity');
    equal(compilerProfile.environment, envelope.build.profileEnvironment, 'actual compiler environment differs from the build identity');
    equal(compilerProfile.configuration, ['--config', `patch.crates-io.meta-language.path=${JSON.stringify(root.slice(0, -1))}`,
      '--config', 'profile.dev.package.meta-language.opt-level=3'], 'actual Cargo configuration changed the candidate/profile');
    equal(report.crate.resolution.patch, compilerProfile.configuration.join(' '), 'reported Cargo patch differs from actual invocations');
    const inventory = readFormalAiNativeInventory(nativeRuns(evidence.nativeInventory, reader), fixture);
    if (inventory.filter((entry) => !entry.probe).length !== expected.nativeWorkloads) fail('the complete native identity count changed');
    equal(envelope.inventory, inventory, 'declared inventory differs from the actual retained --list');
    const executed = readFormalAiNativeExecution(nativeRuns(evidence.nativeExecution, reader), inventory, partition);
    equal(report.crate.filters, formalAiWorkloadLogicalFilters(fixture), 'original native logical scope changed');
    equal(report.crate.workloadFiles, workloadFiles(fixture, 'rust'), 'original native workload files changed');
    const workloads = reportRows(executed.filter((entry) => !entry.probe));
    const probes = reportRows(executed.filter((entry) => entry.probe));
    equal(report.crate.tests, workloads, 'native report differs from actual selected terminal outcomes');
    equal(report.crate.probeTests, probes, 'native probe report differs from actual terminal outcomes');
    equal(report.crate.counts, countOutcomes(workloads), 'native partial counts disagree');
    equal(report.crate.exit, 0, 'native partial process failed');
    // The existing reporter retains independently framed LiNo records, rather
    // than one array document. Preserve its actual storage/wire contract.
    const events = readStructuredDataRecords(reader.text(evidence.javascriptEvents));
    if (!Array.isArray(events)) fail('missing actual JavaScript terminal event stream');
    const dataFiles = new Set(workloadFiles(fixture, 'javascript'));
    const javascriptTests = events.filter(({ kind }) => kind === 'test').map(({ name, outcome, error, process: child }) => {
      if (typeof name !== 'string') fail('invalid JavaScript event identity');
      const [prefix, ...rest] = name.split(TEST_NAME_SEPARATOR);
      return {
        file: dataFiles.has(prefix) ? prefix : null,
        name: dataFiles.has(prefix) ? rest.join(TEST_NAME_SEPARATOR) : name,
        outcome, ...(error ? { error } : {}), ...(child ? { process: child } : {}),
      };
    });
    equal(javascriptTests, report.npm.tests, 'JavaScript report differs from actual terminal events');
    if (javascriptTests.length !== expected.javascriptWorkloads || javascriptTests.some((entry) => entry.file === null || entry.outcome !== 'passed') ||
        new Set(javascriptTests.map(({ file, name }) => JSON.stringify([file, name]))).size !== javascriptTests.length) fail('JavaScript workload identities are missing, duplicate or unsuccessful');
    equal(report.npm.workloadFiles, workloadFiles(fixture, 'javascript'), 'JavaScript workload scope changed');
    const javascriptProbe = reader.data(evidence.javascriptProbe.file);
    if (evidence.javascriptProbe.exit !== 0) fail('the JavaScript probe process failed');
    probeHolds(javascriptProbe, 'the retained JavaScript probe');
    for (const assertion of ['sharedConceptsReused', 'distinctionsPreserved']) {
      const original = javascriptProbe[assertion];
      equal(report.npm.probe[assertion].checks.slice(0, original.checks.length), original.checks,
        `report omitted actual JavaScript ${assertion} checks`);
    }
    equal(report.npm.semanticProjectionObservations, javascriptProbe.semanticProjectionObservations ?? [],
      'report changed the actual semantic projection observations');
    const javascriptOutputs = reader.data(evidence.javascriptOutputs);
    const rustOutputs = reader.data(evidence.rustOutputs);
    equal(javascriptProbe.outputs, javascriptOutputs, 'JavaScript probe output differs from the retained output');
    exactFileDescriptor(report.npm.outputs, evidence.javascriptOutputs, 'JavaScript output');
    exactFileDescriptor(report.crate.outputs, evidence.rustOutputs, 'Rust output');
    const runtimeComparison = compareRuntimeOutputs(javascriptOutputs, rustOutputs);
    if (runtimeComparison.some(({ holds }) => holds !== true)) fail('actual JavaScript/Rust output comparison failed');
    equal(report.comparison, runtimeComparison, 'the report changed the actual runtime comparison');
    for (const part of [report.npm, report.crate]) {
      const comparisons = part.probe.sharedConceptsReused.checks.slice(-runtimeComparison.length);
      equal(comparisons, runtimeComparison, 'report omitted or changed the actual runtime comparison');
    }
    const rustDistinctions = reader.data(evidence.rustDistinctions);
    if (!Array.isArray(rustDistinctions?.checks) || rustDistinctions.checks.length === 0 || rustDistinctions.checks.some(({ holds }) => holds !== true)) fail('retained Rust distinction checks did not hold');
    equal(report.crate.probe.distinctionsPreserved.checks.slice(0, rustDistinctions.checks.length), rustDistinctions.checks,
      'report omitted actual Rust distinction checks');
    for (const [assertion, name] of Object.entries(RUST_PROBE_TESTS)) {
      const observed = report.crate.probe[assertion === 'outputs' ? 'sharedConceptsReused' : assertion];
      if (observed?.test !== name || observed.outcome !== 'passed') fail('native probe assertion has no exact passing identity');
    }
    const inputs = reader.data(evidence.inputs);
    exactFileDescriptor(report.inputs, evidence.inputs, 'logical input');
    equal({ documents: report.inputs.documents, labels: report.inputs.labels, rules: report.inputs.rules,
      linkEdits: report.inputs.linkEdits, projections: report.inputs.projections,
      semanticProjections: report.inputs.semanticProjections,
      semanticProjectionFixtureSha256: report.inputs.semanticProjectionFixtureSha256 }, {
      documents: inputs.documents.length, labels: inputs.labels, rules: inputs.rules.length,
      linkEdits: inputs.linkEdits.length, projections: inputs.projections.length,
      semanticProjections: inputs.semanticProjections.length,
      semanticProjectionFixtureSha256: expected.identity.semanticProjectionFixtureSha256,
    }, 'the logical input summary differs from retained bytes and the pinned semantic projection fixture');
    const lock = reader.bytes(evidence.cargoLock);
    equal(envelope.build.cargoLockSha256, sha256(lock), 'post-update Cargo.lock bytes differ from the build identity');
    if (report.crate.resolution.lockRefresh?.exit !== 0) fail('candidate dependency lock update failed');
    const before = reader.data(evidence.selfAstBefore);
    const after = reader.data(evidence.selfAstAfter);
    if (!Array.isArray(before) || !Array.isArray(after) || before.some((entry) => !Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== 'string' || typeof entry[1] !== 'string') ||
        after.some((entry) => !Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== 'string' || typeof entry[1] !== 'string') ||
        new Set(before.map(([file]) => file)).size !== before.length || new Set(after.map(([file]) => file)).size !== after.length) fail('invalid retained census document maps');
    const census = compareSelfAstCensus(new Map(before), new Map(after));
    if (census.documents === 0 || census.otherChanges.length !== 0 || report.crate.selfAstCensus.exit !== 0) fail('actual self-AST census changed outside link totals');
    equal({ documents: report.crate.selfAstCensus.documents, linkTotals: report.crate.selfAstCensus.linkTotals,
      otherChanges: report.crate.selfAstCensus.otherChanges }, census, 'self-AST census report differs from actual before/after maps');
    reader.complete();
    return { problems: [], value: { partition, envelope, report, inventory, executed, javascriptTests,
      build: envelope.build, inputs, javascriptOutputs, rustOutputs, before, after } };
  } catch (error) {
    return { problems: [error instanceof Error ? error.message : String(error)], value: null };
  }
}

/** No canonical report is returned until both independent partials validate. */
export function mergeFormalAiWorkloadPartitions(parts, fixture, expected) {
  const problems = [];
  if (expected.producerResult !== 'success') problems.push('the actual matrix producer did not succeed');
  if (!Array.isArray(parts) || parts.length !== 2) problems.push('exactly two independently scoped producer bundles are required');
  const validated = [];
  for (const partition of FORMAL_AI_WORKLOAD_PARTITIONS) {
    const matches = (Array.isArray(parts) ? parts : []).filter(({ envelope }) => envelope?.identity?.partition === partition);
    if (matches.length !== 1) { problems.push(`missing or duplicate producer ${partition}`); continue; }
    const result = validateFormalAiWorkloadPartition(matches[0].envelope, fixture, expected, matches[0].files);
    problems.push(...result.problems.map((problem) => `${partition}: ${problem}`));
    if (result.value) validated.push(result.value);
  }
  if (problems.length) return { problems, report: null, provenance: null };
  try {
    const [first, second] = validated;
    for (const field of ['inventory', 'build', 'inputs', 'javascriptTests', 'javascriptOutputs', 'rustOutputs', 'before', 'after']) {
      equal(first[field], second[field], `producer ${field} values differ`);
    }
    equal(first.report.node, second.report.node, 'the actual JavaScript runtime versions differ');
    const all = first.inventory.filter((entry) => !entry.probe);
    const union = validated.flatMap(({ executed }) => executed.filter((entry) => !entry.probe));
    if (new Set(union.map(key)).size !== union.length) fail('workload partitions overlap');
    equal([...union.map(key)].sort(), [...all.map(key)].sort(), 'workload partitions omit or add actual listed identities');
    const byIdentity = new Map(union.map((entry) => [key(entry), entry]));
    const report = structuredClone(second.report);
    report.crate.tests = reportRows(all.map((entry) => byIdentity.get(key(entry))));
    report.crate.counts = countOutcomes(report.crate.tests);
    report.crate.exit = 0;
    for (const runtime of ['javascript', 'rust']) {
      const result = validateFormalAiWorkloadReport(report, fixture, runtime);
      equal(result.observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS], `unchanged complete ${runtime} validator refused the merged report`);
    }
    return { problems: [], report, provenance: {
      kind: 'formal-ai-workload-partition-union', schemaVersion: 1,
      identity: expected.identity, candidates: expected.candidates, build: first.build,
      producers: validated.map(({ partition, envelope }) => ({ partition, artifact: envelope.identity.artifact })),
      nativeInventory: first.inventory, workloadCount: report.crate.tests.length,
      javascriptCount: report.npm.tests.length, sharedProbeIdentities: PROBES.map((name) => ({ target: RUST_PROBE_TARGET, name })),
    } };
  } catch (error) {
    return { problems: [error instanceof Error ? error.message : String(error)], report: null, provenance: null };
  }
}
