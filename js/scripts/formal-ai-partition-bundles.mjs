// Filesystem/process transport for the Formal AI partition proof. The pure
// validator owns selection/equality. Native calls are injected from the existing
// runner; the merge entry point has no child-process implementation or calls.
import { createHash } from 'node:crypto';
import { constants } from 'node:fs';
import { lstat, mkdir, open, opendir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { readProjectFileBytes, writeProjectData } from './project-data.mjs';
import {
  FORMAL_AI_WORKLOAD_PARTITIONS,
  formalAiWorkloadLogicalFilters,
  formalAiWorkloadPartitionCommands,
  mergeFormalAiWorkloadPartitions,
  readFormalAiNativeExecution,
  readFormalAiNativeInventory,
} from './formal-ai-workload-partitions.mjs';
import { FORMAL_AI_WORKLOAD_FIXTURE, FORMAL_AI_WORKLOAD_REQUIREMENT } from './issue-195-formal-ai-workloads.mjs';

export const FORMAL_AI_PARTITION_PROFILE = Object.freeze({
  CARGO_BUILD_JOBS: '2', CARGO_INCREMENTAL: '0', CARGO_PROFILE_DEV_DEBUG: '0',
  CARGO_PROFILE_TEST_DEBUG: '0', RUST_TEST_THREADS: '1', patchedCrateOptLevel: 3,
});
export const FORMAL_AI_PARTITION_BUNDLE_LIMITS = Object.freeze({ files: 64, fileBytes: 128 * 2 ** 20, totalBytes: 256 * 2 ** 20, depth: 4 });
const targets = ['unit', 'source', 'issue_195_meta_language_probe'];
const sourcePaths = [
  'js/scripts/run-formal-ai-workloads.mjs', 'js/scripts/formal-ai-workload-partitions.mjs',
  'js/scripts/formal-ai-partition-bundles.mjs', 'js/scripts/issue-195-formal-ai-workloads.mjs',
  'js/scripts/issue-195-formal-ai-consumer.mjs', 'js/scripts/issue-195-formal-ai-rust-probe.mjs',
  'js/scripts/issue-195-formal-ai-workload-probes.mjs', 'js/scripts/issue-195-rml-workload-probes.mjs',
  'js/scripts/formal-ai-semantic-projections.mjs', 'js/scripts/workload-wire-data.mjs',
  'js/scripts/issue-195-rml-pr184.mjs', 'js/scripts/semver-range.mjs',
];
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const requireValue = (condition, message) => { if (!condition) throw new Error(`Formal AI partitions: ${message}`); };

function relativeFile(file) {
  return typeof file === 'string' && file.length > 0 && !/[\\\0:*?\[\]{}]/u.test(file) &&
    !path.posix.isAbsolute(file) && path.posix.normalize(file) === file &&
    !file.split('/').some((part) => part === '.' || part === '..' || part === '');
}

async function regularBytes(file, limit) {
  const status = await lstat(file);
  requireValue(status.isFile() && !status.isSymbolicLink() && status.size <= limit, `unsafe or oversized regular file ${file}`);
  const handle = await open(file, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const opened = await handle.stat();
    requireValue(opened.isFile() && opened.size === status.size && opened.size <= limit &&
      opened.dev === status.dev && opened.ino === status.ino, `file identity changed during read ${file}`);
    const bytes = await handle.readFile();
    requireValue(bytes.length === opened.size && bytes.length <= limit, `file size changed during read ${file}`);
    return bytes;
  } finally { await handle.close(); }
}

/** Actual local candidate bytes, checksum sidecars and own checkout/context. */
export async function formalAiPartitionExpectations({ root, tarball, crate, fixture, environment = process.env, producerResult = 'success' }) {
  requireValue(environment.CI === 'true' && environment.GITHUB_ACTIONS === 'true', 'partitions require the actual GitHub Actions context');
  requireValue(/^[0-9a-f]{40}$/u.test(environment.GITHUB_SHA ?? '') && /^\d+$/u.test(environment.GITHUB_RUN_ID ?? '') &&
    /^[1-9]\d*$/u.test(environment.GITHUB_RUN_ATTEMPT ?? '') && environment.GITHUB_REPOSITORY === 'link-foundation/meta-language', 'invalid same-run repository/commit/attempt identity');
  const candidate = async (file, npm) => {
    const bytes = await regularBytes(file, FORMAL_AI_PARTITION_BUNDLE_LIMITS.fileBytes);
    const checksum = (await regularBytes(`${file}.sha256`, 1024)).toString('utf8').trim().split(/\s+/u);
    requireValue(checksum.length === 2 && checksum[0] === digest(bytes) && checksum[1] === path.basename(file), 'candidate checksum or exact filename differs');
    return { file: path.basename(file), sha256: digest(bytes), ...(npm ? { integrity: `sha512-${createHash('sha512').update(bytes).digest('base64')}` } : {}) };
  };
  const identity = {
    repository: environment.GITHUB_REPOSITORY, commit: environment.GITHUB_SHA,
    run: environment.GITHUB_RUN_ID, attempt: environment.GITHUB_RUN_ATTEMPT,
    formalAiRepository: fixture.repository, formalAiRevision: fixture.revision,
    fixtureSha256: digest(readProjectFileBytes(path.join(root, FORMAL_AI_WORKLOAD_FIXTURE))),
    semanticProjectionFixtureSha256: digest(readProjectFileBytes(path.join(root, 'parity/fixtures/formal-ai-semantic-projections.json'))),
    sourceSha256: Object.fromEntries(await Promise.all(sourcePaths.map(async (file) =>
      [file, digest(await regularBytes(path.join(root, file), FORMAL_AI_PARTITION_BUNDLE_LIMITS.fileBytes))]))),
  };
  return {
    identity,
    artifacts: Object.fromEntries(FORMAL_AI_WORKLOAD_PARTITIONS.map((partition) => [partition,
      `issue-195-formal-ai-partition-${partition}-${identity.commit}-${identity.run}-${identity.attempt}`])),
    candidates: { npm: await candidate(tarball, true), crate: await candidate(crate, false) },
    platform: `${process.platform}-${process.arch}`, profile: FORMAL_AI_PARTITION_PROFILE,
    nativeWorkloads: 134, javascriptWorkloads: 148, producerResult,
  };
}

/** Complete original scope is listed first. Every assigned target still runs. */
export async function runFormalAiNativePartition({ partition, fixture, configuration, directory, environment, run, workDirectory, state }) {
  const filters = formalAiWorkloadLogicalFilters(fixture);
  state.nativeInventory = [];
  state.nativeExecution = [];
  for (const target of targets) {
    const harnessArguments = [...filters, '--list'];
    const result = await run(`rust-list-${target}`, 'cargo', [
      ...configuration, 'test', '--no-fail-fast', '--test', target, '--', ...harnessArguments,
    ], { cwd: directory, env: { ...environment, RUST_TEST_THREADS: '1' }, allowFailure: true });
    state.nativeInventory.push({ target, harnessArguments, exit: result.code, transcript: result.combined, log: result.log });
  }
  state.inventory = readFormalAiNativeInventory(state.nativeInventory, fixture);
  const completed = [];
  for (const { target, harnessArguments } of formalAiWorkloadPartitionCommands(state.inventory, partition)) {
    const result = await run(`rust-workloads-${target}`, 'cargo', [
      ...configuration, 'test', '--no-fail-fast', '--test', target, '--', ...harnessArguments,
    ], { cwd: directory, env: { ...environment, RUST_TEST_THREADS: '1' }, allowFailure: true, traceTests: true });
    state.nativeExecution.push({ target, harnessArguments, exit: result.code, transcript: result.combined, log: result.log });
    completed.push(result);
  }
  const combined = completed.map(({ combined }) => combined).join('\n');
  const log = path.join(workDirectory, 'rust-workloads.log');
  // Keep each unchanged run() command/exit log as well as this complete aggregate.
  await writeFile(log, completed.map((result) => `log: ${result.log}\n${result.combined}\nexit: ${result.code}\n`).join('\n'));
  let outcomes = null;
  try { outcomes = readFormalAiNativeExecution(state.nativeExecution, state.inventory, partition); }
  catch (error) { state.executionProblems = [error.message]; }
  return { code: completed.every(({ code }) => code === 0) ? 0 : completed.find(({ code }) => code !== 0)?.code ?? 1, combined, log, outcomes };
}

/** Create a separate envelope/evidence directory; never publish a partial as report.lino. */
export async function writeFormalAiPartitionBundle({ partition, expected, report, state, workDirectory, output }) {
  requireValue(FORMAL_AI_WORKLOAD_PARTITIONS.includes(partition), 'unknown producer partition');
  requireValue(path.basename(output) === 'partition.lino', 'a partial must use its distinct partition.lino identity');
  const destination = path.dirname(output);
  await mkdir(destination, { recursive: true });
  const retained = new Map();
  let totalBytes = 0;
  const retain = async (file, source) => {
    requireValue(relativeFile(file) && file !== path.basename(output), 'unsafe or conflicting evidence filename');
    const bytes = await regularBytes(source, FORMAL_AI_PARTITION_BUNDLE_LIMITS.fileBytes);
    if (retained.has(file)) {
      requireValue(retained.get(file).sha256 === digest(bytes), 'two evidence sources collide');
      return retained.get(file);
    }
    totalBytes += bytes.length;
    requireValue(retained.size < FORMAL_AI_PARTITION_BUNDLE_LIMITS.files - 1 && totalBytes <= FORMAL_AI_PARTITION_BUNDLE_LIMITS.totalBytes, 'producer evidence bundle exceeds its bounded transport');
    const descriptor = { file, bytes: bytes.length, sha256: digest(bytes) };
    await writeFile(path.join(destination, file), bytes, { flag: 'wx' });
    retained.set(file, descriptor);
    return descriptor;
  };
  const evidence = {};
  const errors = [...(state.executionProblems ?? [])];
  const attempt = async (name, body) => { try { evidence[name] = await body(); } catch (error) { errors.push(`${name}: ${error.message}`); } };
  await attempt('nativeInventory', async () => Promise.all((state.nativeInventory ?? []).map(async (entry) => ({
    target: entry.target, harnessArguments: entry.harnessArguments, exit: entry.exit,
    // Original run() log includes its command, cwd and complete output. The
    // independent parser tolerates those non-outcome lines, never using them as outcomes.
    transcript: await retain(path.basename(entry.log), entry.log),
  }))));
  await attempt('nativeExecution', async () => Promise.all((state.nativeExecution ?? []).map(async (entry) => ({
    target: entry.target, harnessArguments: entry.harnessArguments, exit: entry.exit,
    transcript: await retain(path.basename(entry.log), entry.log),
  }))));
  for (const [name, file] of Object.entries({ javascriptEvents: 'javascript-test-events.lino', javascriptOutputs: 'javascript-outputs.lino',
    rustOutputs: 'rust-outputs.lino', inputs: 'formal-ai-inputs.lino', selfAstBefore: 'self-ast-before.lino', selfAstAfter: 'self-ast-after.lino', cargoLock: 'post-update-Cargo.lock' })) {
    await attempt(name, () => retain(file, path.join(workDirectory, file)));
  }
  await attempt('compiler', () => retain('rust-compiler-identity.txt', path.join(workDirectory, 'rust-compiler-identity.txt')));
  await attempt('compilerProfile', () => retain('compiler-profile.lino', path.join(workDirectory, 'compiler-profile.lino')));
  await attempt('javascriptProbe', async () => ({ file: await retain('javascript-probe.lino', path.join(workDirectory, 'javascript-probe.lino')), exit: state.javascriptProbeExit ?? null }));
  await attempt('rustDistinctions', () => retain('distinctionsPreserved.lino', path.join(workDirectory, 'rust-probe', 'distinctionsPreserved.lino')));
  const envelope = {
    kind: 'formal-ai-workload-partition', partitionSchemaVersion: 1,
    identity: { ...expected.identity, partition, artifact: expected.artifacts[partition] },
    build: { platform: report.platform, profile: state.profile ?? null,
      profileEnvironment: state.profileEnvironment ?? null, compiler: state.compiler ?? null,
      cargoLockSha256: evidence.cargoLock?.sha256 ?? null },
    inventory: state.inventory ?? [], evidence, result: report, captureErrors: errors,
  };
  writeProjectData(output, envelope, { format: 'lino' });
  return envelope;
}

/** No source archive, logs directory or sibling producer can enter this map. */
export async function readFormalAiPartitionBundle(directory, limits = FORMAL_AI_PARTITION_BUNDLE_LIMITS) {
  const root = path.resolve(directory);
  const rootStatus = await lstat(root);
  requireValue(rootStatus.isDirectory() && !rootStatus.isSymbolicLink(), 'bundle root is not an ordinary directory');
  const files = new Map();
  const directories = [];
  let entries = 0;
  let totalBytes = 0;
  const pending = [{ relative: '', depth: 0 }];
  while (pending.length) {
    const current = pending.pop();
    requireValue(current.depth <= limits.depth, 'bundle directory nesting exceeds the bound');
    for await (const entry of await opendir(path.join(root, current.relative))) {
      entries += 1;
      requireValue(entries <= limits.files * 2, 'bundle directory entry count exceeds the bound');
      const file = current.relative ? `${current.relative}/${entry.name}` : entry.name;
      requireValue(relativeFile(file), 'unsafe bundle path');
      const status = await lstat(path.join(root, file));
      requireValue(!status.isSymbolicLink(), 'symbolic links are not evidence files');
      if (status.isDirectory()) { directories.push(file); pending.push({ relative: file, depth: current.depth + 1 }); continue; }
      requireValue(status.isFile() && !files.has(file) && files.size < limits.files, 'unknown, duplicate or excessive bundle file');
      const bytes = await regularBytes(path.join(root, file), limits.fileBytes);
      totalBytes += bytes.length;
      requireValue(totalBytes <= limits.totalBytes, 'bundle byte limit exceeded');
      files.set(file, bytes);
    }
  }
  for (const directory of directories) requireValue([...files.keys()].some((file) => file.startsWith(`${directory}/`)), 'unknown empty evidence directory');
  const bytes = files.get('partition.lino');
  requireValue(bytes, 'missing authentic partial envelope');
  files.delete('partition.lino');
  return { envelope: readStructuredDataLino(new TextDecoder('utf-8', { fatal: true }).decode(bytes)), files,
    receipt: { file: 'partition.lino', bytes: bytes.length, sha256: digest(bytes) } };
}

/** Source-only merger: no clone, Cargo, consumer setup or child process. */
export async function mergeFormalAiPartitionBundles({ directories, root, tarball, crate, fixture, workDirectory, output, producerResult, environment = process.env }) {
  await mkdir(workDirectory, { recursive: true });
  await mkdir(path.dirname(output), { recursive: true });
  let expected;
  let parts = [];
  let result;
  try {
    expected = await formalAiPartitionExpectations({ root, tarball, crate, fixture, environment, producerResult });
    requireValue(directories.length === 2, 'two disjoint producer directories are required');
    for (const [parent, child] of [[directories[0], directories[1]], [directories[1], directories[0]]]) {
      const relative = path.relative(path.resolve(parent), path.resolve(child));
      requireValue(relative === '..' || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative), 'two disjoint producer directories are required');
    }
    // Read both even when one refuses, preserving all per-producer diagnostics.
    const reads = await Promise.allSettled(directories.map((directory) => readFormalAiPartitionBundle(directory)));
    const errors = reads.flatMap((entry, index) => entry.status === 'rejected' ? [`${directories[index]}: ${entry.reason.message}`] : []);
    parts = reads.filter((entry) => entry.status === 'fulfilled').map(({ value }) => value);
    if (errors.length) result = { report: null, provenance: null, problems: errors };
    else result = mergeFormalAiWorkloadPartitions(parts, fixture, expected);
    if (result.report) {
      // Keep the old public diagnostic/input/output paths. Equal validated bytes
      // are copied only after the complete two-producer proof succeeds.
      const selected = parts.find(({ envelope }) => envelope.identity.partition === 'remaining');
      for (const field of ['inputs', 'javascriptOutputs', 'rustOutputs']) {
        const descriptor = selected.envelope.evidence[field];
        await writeFile(path.join(workDirectory, descriptor.file), selected.files.get(descriptor.file));
      }
      result.provenance.producers = parts.map(({ envelope, receipt }) => ({
        partition: envelope.identity.partition, artifact: envelope.identity.artifact, envelope: receipt,
      }));
      writeProjectData(path.join(workDirectory, 'partition-provenance.lino'), result.provenance, { format: 'lino' });
    }
  } catch (error) { result = { report: null, provenance: null, problems: [error.message] }; }
  const report = result.report ?? {
    schemaVersion: 1, requirementId: FORMAL_AI_WORKLOAD_REQUIREMENT, fixture: FORMAL_AI_WORKLOAD_FIXTURE,
    fixtureSha256: expected?.identity.fixtureSha256 ?? null,
    partitionFailure: { problems: result.problems, producerResult, identity: expected?.identity ?? null },
  };
  writeProjectData(output, report, { format: 'lino' });
  return { ...result, output };
}
