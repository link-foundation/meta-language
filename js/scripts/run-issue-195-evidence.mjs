#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { spawn, execFileSync } from 'node:child_process';
import {
  appendFile,
  copyFile,
  mkdir,
  open,
  readFile,
  readdir,
  rm,
  writeFile,
} from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { acquireLease } from '../../scripts/lib/cache-cleanup.mjs';
import { buildIssue195Manifest } from './issue-195-acceptance-lib.mjs';
import { measureCacheCleanup } from './issue-195-cache-evidence.mjs';
import {
  DELIVERY_CONSUMER_ASSERTIONS,
  deliveryAssertions,
  supportedPlatformCoverage,
} from './issue-195-delivery.mjs';
import { buildEvidencePlan, observedEvidenceForCell } from './issue-195-evidence-plan.mjs';
import {
  RUNTIMES,
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
} from './issue-195-evidence-stages.mjs';
import { verifyEvaluatedCheckout } from './issue-195-merge-enforcement.mjs';
import { PARITY_ARTIFACT_FILES } from './issue-195-parity-evidence.mjs';
import { TEST_GROUPS } from './test-groups.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const commit = option('--commit', process.env.GITHUB_SHA ?? git(['rev-parse', 'HEAD']));
verifyEvaluatedCheckout(root, commit);
const checkpoint = option('--checkpoint', 'pre-merge');
const resultsDirectory = safeResultsDirectory(option('--results-dir', 'issue-195-results'));
const logsDirectory = path.join(resultsDirectory, 'logs');
const workDirectory = path.join(resultsDirectory, 'work');
const observationsPath = path.join(workDirectory, 'execution-records.jsonl');
const artifactsDirectory = path.join(resultsDirectory, 'artifacts');
const parityDirectory = path.join(resultsDirectory, 'runtime-parity');
const manifest = await buildIssue195Manifest(root);
const plan = buildEvidencePlan(manifest, checkpoint);
const packageMetadata = JSON.parse(await readFile(path.join(root, 'js/package.json'), 'utf8'));
const grammarLock = JSON.parse(
  await readFile(path.join(root, 'js/src/vendor/grammars/grammar-lock.json'), 'utf8'),
);
const corpusBytes = await readFile(path.join(root, 'parity/fixtures/issue-195-evidence.json'));
const corpus = JSON.parse(corpusBytes.toString('utf8'));
const corpusSha256 = createHash('sha256').update(corpusBytes).digest('hex');

const consumerScript = path.join(root, 'js/scripts/run-issue-195-consumer.mjs');
const candidatesDirectory = option('--candidates', null);
const platformReportsDirectory = option('--platform-reports', null);
const cellsByTestId = new Map(
  plan.flatMap(({ cells }) => cells.map(({ cell }) => [cell.testId, cell])),
);
const runnerObservations = [];
const deliveryContext = { corpus, corpusSha256, version: packageMetadata.version };

const prepareCandidatesDirectory = option('--prepare-candidates', null);
if (prepareCandidatesDirectory) {
  await mkdir(logsDirectory, { recursive: true });
  const prepared = await prepareCandidates(path.resolve(prepareCandidatesDirectory));
  verifyEvaluatedCheckout(root, commit);
  console.log(
    `issue-195 evidence: prepared ${path.basename(prepared.tarball)} and ${path.basename(prepared.crate)}`,
  );
  process.exit(0);
}

const stageName = option('--stage', null);
const aggregateOnly = process.argv.includes('--aggregate');
const stages = evidenceStages(checkpoint);
const selectedStage = stageName ? evidenceStage(checkpoint, stageName) : null;
if (selectedStage && aggregateOnly) throw new Error('--stage and --aggregate cannot be combined');

await prepareDirectories();
// While the run builds, a concurrent cleanup (a pre-commit hook, another
// wrapper) keeps the outputs it uses; the run's own final cleanup ignores it.
const lease = acquireLease({ cwd: root, label: `issue-195 ${checkpoint} evidence` });
process.on('exit', () => lease.release());
process.env.ISSUE_195_COMMIT = commit;

if (selectedStage) {
  // CI runs every stage as its own job; the aggregate job merges their outputs.
  const succeeded = await runStage(selectedStage);
  verifyEvaluatedCheckout(root, commit);
  process.exitCode = succeeded ? 0 : 1;
} else {
  if (!aggregateOnly) {
    // One stage after another: the suites and builds never share the machine.
    console.log(
      `issue-195 evidence: executing ${stages.length} ${checkpoint} evidence stages for ${commit}`,
    );
    for (const stage of stages) await runStage(stage);
  }
  await aggregateStages();
  verifyEvaluatedCheckout(root, commit);
}

/**
 * Runs one stage and writes its record and execution records. A failing
 * command or check does not stop the stage's other evidence; the record keeps
 * the failure, which the aggregate reports as one gate error naming the stage.
 */
async function runStage(stage) {
  const files = stageFiles(stage.name);
  const recordPath = path.join(resultsDirectory, files.record);
  const stageObservationsPath = path.join(resultsDirectory, files.observations);
  await rm(recordPath, { force: true });
  await rm(stageObservationsPath, { force: true });
  process.env.ISSUE_195_OBSERVATION_FILE = stageObservationsPath;
  runnerObservations.length = 0;
  console.log(`issue-195 evidence: stage ${stage.name}`);
  const failures = [];
  let toolchainVersions = {};
  let groups = new Map();
  try {
    toolchainVersions = readToolchainVersions(stage.tools);
    groups = await produceStage(stage, failures);
  } catch (error) {
    failures.push(error.message);
  }
  const error = failures.length > 0 ? failures.join('; ') : null;
  if (error) console.error(`issue-195 evidence: stage ${stage.name} failed: ${error}`);
  if (runnerObservations.length > 0) {
    // One write keeps runner records whole next to the suites' own callbacks.
    await appendFile(
      stageObservationsPath,
      runnerObservations.map((record) => `${JSON.stringify(record)}\n`).join(''),
    );
  }
  const record = {
    schemaVersion: 1,
    issue: 195,
    stage: stage.name,
    commit,
    checkpoint,
    toolchainVersions,
    groups: Object.fromEntries(groups),
    error,
  };
  await writeFile(recordPath, stableJson(record));
  // The stage log keeps logs/ next to work/ in every stage upload, so each
  // artifact unpacks at the results directory root.
  await writeFile(
    path.join(logsDirectory, `stage-${stage.name}.log`),
    `stage: ${stage.name}\ncommit: ${commit}\ncheckpoint: ${checkpoint}\ngroups: ${[...groups.keys()].join(', ')}\nerror: ${error ?? 'none'}\n`,
  );
  return error === null;
}

async function produceStage(stage, failures) {
  if (stage.name === 'javascript-suite') {
    return new Map([['suite:javascript', await runJavaScriptSuite(failures)]]);
  }
  if (stage.name === 'rust-suite') return new Map([['suite:rust', await runRustSuite(failures)]]);
  if (stage.name === 'runtime-parity') {
    const runtimeParity = await runCommand('runtime-parity', 'node', [
      path.join(root, 'js/scripts/check-issue-195-runtime-parity.mjs'),
      '--artifacts-dir',
      parityDirectory,
    ]);
    return new Map([['runtime-parity', bundle(runtimeParity, parityArtifacts())]]);
  }
  if (stage.target) return validateNativeTranslations(stage.target);
  if (stage.name === 'delivery') {
    return new Map(checkpoint === 'release-delivery'
      ? await producePublishedEvidence()
      : await produceDeliveryEvidence('candidate'));
  }
  if (stage.name === 'merge-enforcement') {
    return new Map([['merge-enforcement', await produceMergeEnforcementEvidence()]]);
  }
  throw new Error(`evidence stage ${stage.name} has no producer`);
}

/** Runs each JavaScript test group in its own process, one group after another. */
async function runJavaScriptSuite(failures) {
  const records = [];
  for (const group of Object.keys(TEST_GROUPS)) {
    const record = await runCommand(
      `suite-javascript-${group}`,
      'node',
      ['scripts/test-groups.mjs', '--run', group],
      { cwd: path.join(root, 'js'), allowFailure: true },
    );
    if (!record.ok) failures.push(`JavaScript test group ${group} failed; see ${record.artifacts[0]}`);
    records.push(record);
  }
  return bundle(...records);
}

/**
 * Runs the Rust tests with the default features: the execution records come
 * from the test binaries, and no recording test needs another feature.
 */
async function runRustSuite(failures) {
  const record = await runCommand('suite-rust', 'cargo', [
    'test', '--no-fail-fast', '--locked', '--manifest-path', path.join(root, 'rust/Cargo.toml'), '--tests',
  ], { allowFailure: true });
  if (!record.ok) failures.push(`Rust tests failed; see ${record.artifacts[0]}`);
  return bundle(record);
}

function parityArtifacts() {
  return {
    commands: [],
    artifacts: [
      PARITY_ARTIFACT_FILES.digests,
      PARITY_ARTIFACT_FILES.differences,
      ...RUNTIMES.map(PARITY_ARTIFACT_FILES.translations),
    ].map((file) => relative(path.join(parityDirectory, file))),
  };
}

/**
 * Merges the stage outputs in the results directory, derives the translation
 * and cache-cleanup groups, and writes every verification cell. A stage that
 * failed or did not report writes one gate error naming it.
 */
async function aggregateStages() {
  const records = new Map();
  for (const name of (await readdir(workDirectory)).filter((file) => /^stage-.+\.json$/u.test(file)).sort()) {
    const stage = name.slice('stage-'.length, -'.json'.length);
    try {
      records.set(stage, JSON.parse(await readFile(path.join(workDirectory, name), 'utf8')));
    } catch (error) {
      records.set(stage, error);
    }
  }
  const { groups, stageErrors } = mergeStageRecords({ checkpoint, commit, records });
  const executionRecords = await readStageExecutionRecords(records, stageErrors);
  process.env.ISSUE_195_OBSERVATION_FILE = observationsPath;
  for (const [group, record] of translationGroups(groups)) groups.set(group, record);
  runnerObservations.length = 0;
  if (aggregateGroups(checkpoint).includes('cache-cleanup:measured')) {
    groups.set('cache-cleanup:measured', {
      ...produceCacheCleanupEvidence(),
      toolchainVersions: readToolchainVersions(['node']),
    });
  }
  executionRecords.push(...runnerObservations);
  await writeFile(
    observationsPath,
    executionRecords.map((record) => `${JSON.stringify(record)}\n`).join(''),
  );
  const grammarVersions = readGrammarVersions();

  let written = 0;
  let complete = 0;
  const absentGroups = [];
  for (const { group, cells } of plan) {
    const record = groups.get(group);
    if (!record) {
      absentGroups.push(group);
      continue;
    }
    for (const { cell } of cells) {
      const observed = observedEvidenceForCell(cell, executionRecords);
      const output = path.join(root, cell.evidenceArtifact.split('#')[0]);
      const fixtureDigests = Object.fromEntries(
        cell.fixtureIds.map((fixtureId) => [fixtureId, manifest.fixtureCatalog[fixtureId].sha256]),
      );
      await writeFile(
        output,
        stableJson({
          schemaVersion: 1,
          issue: 195,
          commit,
          producer: 'js/scripts/run-issue-195-evidence.mjs',
          generatedAt: new Date().toISOString(),
          results: [
            {
              testId: cell.testId,
              outcome: observed.complete ? 'passed' : 'missing',
              kind: cell.kind,
              positiveEvidence: observed.complete,
              command: record.commands.join(' && '),
              toolchainVersions: record.toolchainVersions,
              grammarVersions,
              evidenceArtifacts: [relative(output), ...record.artifacts],
              failureLogs: record.failureLogs ?? [],
              assertionsPassed: observed.assertionsPassed,
              executionRecords: observed.executionRecords,
              fixtureDigests,
            },
          ],
        }),
      );
      written += 1;
      if (observed.complete) complete += 1;
    }
  }
  await writeFile(
    path.join(resultsDirectory, 'evidence-stages.json'),
    stableJson({
      schemaVersion: 1,
      issue: 195,
      commit,
      producer: 'js/scripts/run-issue-195-evidence.mjs',
      generatedAt: new Date().toISOString(),
      results: [],
      stages: [...records.keys()],
      absentGroups,
      stageErrors,
    }),
  );
  for (const stageError of stageErrors) console.error(`issue-195 evidence: ${stageGateError(stageError)}`);
  if (absentGroups.length > 0) {
    console.error(`issue-195 evidence: no record for ${absentGroups.join(', ')}; their cells are missing`);
  }
  console.log(`issue-195 evidence: ${complete}/${written} cells have observed assertion callbacks`);
}

/**
 * Reads the execution records of every stage. A record that is malformed,
 * names an unknown cell or repeats another is dropped and reported as an
 * error of the stage that wrote it.
 */
async function readStageExecutionRecords(records, stageErrors) {
  const known = new Set(plan.flatMap(({ cells }) => cells.map(({ cell }) => cell.testId)));
  const seen = new Set();
  const accepted = [];
  for (const stage of stages) {
    if (!records.has(stage.name)) continue;
    let contents;
    try {
      contents = await readFile(path.join(resultsDirectory, stageFiles(stage.name).observations), 'utf8');
    } catch (error) {
      if (error.code === 'ENOENT') continue;
      throw error;
    }
    const problems = [];
    contents.split(/\r?\n/u).forEach((line, index) => {
      if (!line) return;
      let record;
      try {
        record = JSON.parse(line);
      } catch (error) {
        problems.push(`invalid execution record at line ${index + 1}: ${error.message}`);
        return;
      }
      if (!record || typeof record !== 'object') {
        problems.push(`execution record at line ${index + 1} is not an object`);
        return;
      }
      if (!known.has(record.testId)) {
        problems.push(`execution record has unknown testId ${record.testId}`);
        return;
      }
      const key = `${record.testId}\u0000${record.assertionId}\u0000${record.fixtureId}`;
      if (seen.has(key)) {
        problems.push(`duplicate execution record ${key.replaceAll('\u0000', ' ')}`);
        return;
      }
      seen.add(key);
      accepted.push(record);
    });
    if (problems.length > 0) stageErrors.push({ stage: stage.name, error: problems.join('; ') });
  }
  return accepted;
}

function option(name, fallback) {
  const index = process.argv.indexOf(name);
  return index === -1 ? fallback : process.argv[index + 1];
}

function safeResultsDirectory(value) {
  const resolved = path.resolve(root, value);
  if (resolved === root || !resolved.startsWith(`${root}${path.sep}`)) {
    throw new Error(`results directory must be inside the repository: ${value}`);
  }
  return resolved;
}

/**
 * A full run starts from empty outputs. A stage keeps the other stages'
 * outputs (a native stage reads the runtime-parity stage's), and the
 * aggregate keeps every downloaded stage output and replaces only the cells.
 */
async function prepareDirectories() {
  await mkdir(resultsDirectory, { recursive: true });
  if (!selectedStage) {
    for (const name of await readdir(resultsDirectory)) {
      if (name.endsWith('.json')) await rm(path.join(resultsDirectory, name));
    }
    await rm(observationsPath, { force: true });
  }
  if (!selectedStage && !aggregateOnly) {
    await rm(workDirectory, { recursive: true, force: true });
    await rm(logsDirectory, { recursive: true, force: true });
    await rm(artifactsDirectory, { recursive: true, force: true });
    await rm(path.join(resultsDirectory, 'platform-reports'), { recursive: true, force: true });
  }
  await Promise.all([
    mkdir(workDirectory, { recursive: true }),
    mkdir(logsDirectory, { recursive: true }),
    mkdir(artifactsDirectory, { recursive: true }),
    mkdir(parityDirectory, { recursive: true }),
  ]);
}

/** The versions of the stage's tools; a declared tool must have its declared version. */
function readToolchainVersions(tools) {
  const versions = Object.fromEntries(tools.map((tool) => [tool, commandOutput(tool, ['--version'])]));
  versions.platform = `${process.platform}-${process.arch}`;
  const problems = toolchainProblems(versions);
  if (problems.length > 0) throw new Error(problems.join('; '));
  return versions;
}

function readGrammarVersions() {
  return {
    metaLanguage: packageMetadata.version,
    webTreeSitter: packageMetadata.dependencies['web-tree-sitter'],
    grammars: Object.fromEntries(
      Object.entries(grammarLock.grammars).map(([id, grammar]) => [id, grammar.version]),
    ),
    acceptanceManifest: `schema-${manifest.schemaVersion}`,
  };
}

async function produceMergeEnforcementEvidence() {
  const reportPath = path.join(artifactsDirectory, 'merge-enforcement.json');
  const execution = await runCommand('merge-enforcement', 'node', [
    path.join(root, 'js/scripts/check-issue-195-merge-enforcement.mjs'),
    '--commit', commit, '--report', reportPath,
  ], { allowFailure: true });
  const report = JSON.parse(await readFile(reportPath, 'utf8'));
  for (const [assertionId, holds] of Object.entries(report.checks)) {
    if (holds === true) observe(
      'i195-acceptance-required-merge-check-tooling-behavior', assertionId,
      'live GitHub merge enforcement and separate release checkpoint inspection',
    );
  }
  return { ...bundle(execution), artifacts: [relative(reportPath)] };
}

/**
 * Cleans the worktree after every other group ran, keeping the records, logs,
 * artifacts and native evidence the cells cite, and records the measurement.
 */
function produceCacheCleanupEvidence() {
  const reportPath = path.join(artifactsDirectory, 'cache-cleanup.json');
  const testId = 'i195-cache-cleanup-measured-tooling-measured';
  console.log('issue-195 evidence: cache-cleanup');
  const { checks, report } = measureCacheCleanup({ root, resultsDirectory, reportPath });
  console.log(
    `issue-195 evidence: cache cleanup reclaimed ${report.reclaimedBytes} of ${report.beforeBytes} bytes, ${report.afterBytes} remain`,
  );
  for (const [assertionId, holds] of Object.entries(checks)) {
    if (holds) observe(testId, assertionId, 'evidence runner cache cleanup of the worktree');
    else console.log(`issue-195 evidence: cache cleanup check ${assertionId} failed`);
  }
  return {
    commands: [renderCommand('node', [
      'scripts/clean-caches.mjs', '--event', 'acceptance', '--results-dir', relative(resultsDirectory),
    ])],
    artifacts: [relative(reportPath)],
    failureLogs: Object.values(checks).every(Boolean) ? [] : [relative(reportPath)],
  };
}

async function producePublishedEvidence() {
  const groups = new Map(await produceDeliveryEvidence('published'));
  // No formal-ai release consumes the published packages yet, so its cells
  // run nothing and stay missing until a clean formal-ai consumer is added.
  groups.set('delivery:formal-ai', { commands: [], artifacts: [], failureLogs: [] });
  return groups;
}

/**
 * Validates both runtimes' emitted translations to `target` with the declared
 * native toolchain. Each runtime records the toolchain identity, validates the
 * artifacts, re-executes the recorded reproduction script, and confirms that
 * the same command rejects a deliberately invalid artifact whose log is kept
 * as the failure log. Native acceptance checks emitted-code validity only; it
 * is not proof authority.
 */
async function validateNativeTranslations(target) {
  const observations = {};
  for (const runtime of RUNTIMES) {
    const file = path.join(parityDirectory, PARITY_ARTIFACT_FILES.translations(runtime));
    try {
      observations[runtime] = JSON.parse(await readFile(file, 'utf8'));
    } catch (error) {
      throw new Error(`the runtime-parity stage output ${relative(file)} cannot be read: ${error.message}`);
    }
  }
  const groups = new Map();
  for (const [runtime, observation] of Object.entries(observations)) {
    const declaration = corpus.nativeValidation[target];
    const directory = path.join(workDirectory, 'native', runtime, target.toLowerCase());
    await mkdir(directory, { recursive: true });
    const translations = observation.translations.filter(
      ({ targetLanguage }) => targetLanguage === target,
    );
    if (translations.length !== 3) {
      throw new Error(`${runtime} emitted ${translations.length} ${target} translations, expected 3`);
    }
    const label = `native-${runtime}-${target.toLowerCase()}`;
    const testId = `i195-native-${target.toLowerCase()}-${runtime}-positive`;
    const version = commandOutput(declaration.tool, ['--version']);
    const toolchainPath = path.join(directory, 'toolchain.json');
    await writeFile(toolchainPath, stableJson({
      tool: declaration.tool,
      declaredVersion: declaration.declaredVersion,
      version,
      platform: `${process.platform}-${process.arch}`,
    }));
    const artifacts = [];
    for (const translation of translations) {
      const sourceName = translation.sourceLanguage.toLowerCase();
      const artifact = path.join(directory, `${sourceName}_to_${target.toLowerCase()}${declaration.extension}`);
      await writeFile(artifact, translation.code);
      artifacts.push(artifact);
    }
    const validations = [];
    for (const artifact of artifacts) {
      validations.push(await runCommand(
        `${label}-${path.basename(artifact, declaration.extension)}`,
        ...nativeCommand(target, artifact),
        { allowFailure: true },
      ));
    }
    const reproducePath = path.join(directory, 'reproduce.sh');
    await writeFile(reproducePath, `#!/bin/sh\nset -eu\n${artifacts
      .map((artifact) => renderCommand(...nativeCommand(target, artifact)))
      .join('\n')}\n`);
    const reproduction = await runCommand(`${label}-reproduce`, 'sh', [reproducePath], {
      allowFailure: true,
    });
    const rejected = path.join(directory, `rejected${declaration.extension}`);
    await writeFile(rejected, `${translations[0].code}${declaration.rejectedSuffix}`);
    const negative = await runCommand(`${label}-rejected`, ...nativeCommand(target, rejected), {
      allowFailure: true,
    });
    const record = bundle(...validations, reproduction, negative);
    record.artifacts.push(
      relative(toolchainPath),
      relative(reproducePath),
      ...artifacts.map(relative),
      relative(rejected),
    );
    const toolchain = JSON.parse(await readFile(toolchainPath, 'utf8'));
    if (version.includes(declaration.declaredVersion)) {
      observe(testId, 'declaredToolchainPresent', `${declaration.tool} ${declaration.declaredVersion} is installed`);
    }
    if (validations.every(({ ok }) => ok)) {
      observe(testId, 'artifactValidated', `${declaration.tool} accepts all ${runtime} ${target} translations`);
    }
    if (toolchain.version === version && toolchain.version.includes(declaration.declaredVersion)) {
      observe(testId, 'toolchainVersionRecorded', `${target} toolchain identity recorded`);
    }
    if (reproduction.ok && validations.every(({ ok }) => ok)) {
      observe(testId, 'reproducibleCommandRecorded', `${target} recorded validation script re-executes`);
    }
    if (!negative.ok) {
      record.failureLogs = negative.artifacts;
      observe(testId, 'failureLogRecorded', `${declaration.tool} rejects an invalid ${target} artifact with a log`);
    }
    groups.set(`native:${target}:${runtime}`, record);
    // The cell is recorded: compiler outputs (.rmeta, .vo, .glob) are not
    // evidence, so they leave before the next runtime builds its own.
    const cited = new Set([...record.artifacts, ...record.failureLogs]);
    const names = await readdir(directory);
    const kept = names.filter((name) => cited.has(relative(path.join(directory, name))));
    for (const name of uncitedFiles(names, kept)) {
      await rm(path.join(directory, name), { recursive: true, force: true });
    }
  }
  return groups;
}

function nativeCommand(language, artifact) {
  if (language === 'JavaScript') return ['node', ['--check', artifact]];
  if (language === 'Rust') {
    const out = `${artifact.slice(0, -'.rs'.length)}.rmeta`;
    return ['rustc', [
      '--crate-type', 'lib', '--edition', '2024', '--emit', 'metadata', '-o', out, artifact,
    ]];
  }
  if (language === 'Lean') return ['lean', [artifact]];
  return ['rocq', ['compile', '-q', artifact]];
}

/** Packs the candidates at this commit, or downloads and verifies the published release. */
async function prepareCandidates(directory) {
  await mkdir(directory, { recursive: true });
  const version = packageMetadata.version;
  const crateVersion = /^version = "([^"]+)"/mu.exec(
    await readFile(path.join(root, 'rust/Cargo.toml'), 'utf8'),
  )?.[1];
  if (crateVersion !== version) {
    throw new Error(`npm version ${version} does not match crate version ${crateVersion}`);
  }
  const tarball = path.join(directory, `meta-language-${version}.tgz`);
  const crate = path.join(directory, `meta-language-${version}.crate`);
  let record;
  if (checkpoint === 'release-delivery') {
    const npmMetadata = path.join(directory, 'npm-registry.json');
    const crateMetadata = path.join(directory, 'crates-io.json');
    const userAgent = 'meta-language-issue-195-evidence (https://github.com/link-foundation/meta-language)';
    const metadata = await runCommandWithRetry('delivery-npm-published-metadata', 'curl', [
      '--fail', '--location', '--silent', '--show-error', '--output', npmMetadata,
      `https://registry.npmjs.org/meta-language/${version}`,
    ]);
    const pack = await runCommandWithRetry('delivery-npm-published-pack', 'npm', [
      'pack', `meta-language@${version}`, '--json', '--pack-destination', directory,
    ]);
    const crateInfo = await runCommandWithRetry('delivery-crate-published-metadata', 'curl', [
      '--fail', '--location', '--silent', '--show-error', '--user-agent', userAgent,
      '--output', crateMetadata, `https://crates.io/api/v1/crates/meta-language/${version}`,
    ]);
    const download = await runCommandWithRetry('delivery-crate-published-download', 'curl', [
      '--fail', '--location', '--silent', '--show-error', '--user-agent', userAgent,
      '--output', crate, `https://crates.io/api/v1/crates/meta-language/${version}/download`,
    ]);
    const integrity = JSON.parse(await readFile(npmMetadata, 'utf8')).dist?.integrity;
    const computedIntegrity = `sha512-${createHash('sha512').update(await readFile(tarball)).digest('base64')}`;
    if (integrity !== computedIntegrity) {
      throw new Error(`npm registry integrity ${integrity} does not match ${computedIntegrity}`);
    }
    const checksum = JSON.parse(await readFile(crateMetadata, 'utf8')).version?.checksum;
    if (checksum !== await sha256File(crate)) {
      throw new Error(`crates.io checksum ${checksum} does not match the downloaded crate`);
    }
    record = bundle(metadata, pack, crateInfo, download);
  } else {
    const crateTarget = path.join(workDirectory, 'crate-package');
    const pack = await runCommand('delivery-npm-pack', 'npm', [
      'pack', '--json', '--pack-destination', directory,
    ], { cwd: path.join(root, 'js') });
    const cratePack = await runCommand('delivery-crate-pack', 'cargo', [
      'package', '--manifest-path', path.join(root, 'rust/Cargo.toml'),
      '--allow-dirty', '--target-dir', crateTarget,
    ]);
    await copyFile(path.join(crateTarget, 'package', path.basename(crate)), crate);
    record = bundle(pack, cratePack);
  }
  for (const artifact of [tarball, crate]) {
    await writeFile(`${artifact}.sha256`, `${await sha256File(artifact)}  ${path.basename(artifact)}\n`);
  }
  return { ...record, tarball, crate };
}

/**
 * Runs the clean consumer on this platform against the exact candidates, joins
 * the reports produced on the other supported platforms, runs downstream RML,
 * and records each delivery assertion only when it was observed.
 */
async function produceDeliveryEvidence(kind) {
  const candidates = await candidateArtifacts();
  const consumerDirectory = path.join(workDirectory, 'consumer');
  const localReport = path.join(
    resultsDirectory,
    'platform-reports',
    `${process.platform}-${process.arch}.json`,
  );
  const consumer = await runCommand('delivery-consumer', 'node', [
    consumerScript,
    '--npm-tarball', candidates.tarball,
    '--crate', candidates.crate,
    '--work-dir', consumerDirectory,
    '--out', localReport,
  ], { allowFailure: true });
  const reports = await readPlatformReports(consumer.ok ? localReport : null);
  const crateSource = path.join(
    consumerDirectory,
    'crate-source',
    path.basename(candidates.crate, '.crate'),
  );
  const rml = await validateDownstreamRml(candidates, { crateSource }, kind).catch((error) => {
    console.log(`issue-195 evidence: downstream RML failed: ${error.message}`);
    return { javascript: failedRecord(error), rust: failedRecord(error) };
  });
  const expected = {
    npm: (await readFile(`${candidates.tarball}.sha256`, 'utf8')).split(/\s+/u)[0],
    crate: (await readFile(`${candidates.crate}.sha256`, 'utf8')).split(/\s+/u)[0],
  };
  const reportArtifacts = { commands: [], artifacts: reports.map(({ artifact }) => artifact) };
  const local = reports.find(({ file }) => file === localReport)?.report ?? null;
  const groups = [];
  for (const deliveryPackage of ['npm', 'crate', 'rml']) {
    for (const runtime of ['javascript', 'rust']) {
      const testId = `i195-delivery-${deliveryPackage}-${kind}-${runtime}-${checkpoint}`;
      if (!cellsByTestId.has(testId)) continue;
      const primary = deliveryPackage === 'rml'
        ? (runtime === 'javascript' ? 'npm' : 'crate')
        : deliveryPackage;
      const crossRuntime = deliveryPackage !== 'rml' && (primary === 'npm') !== (runtime === 'javascript');
      const sides = crossRuntime ? ['npm', 'crate'] : [primary];
      const localChecks = deliveryAssertions(local, sides, expected, deliveryContext);
      for (const assertion of DELIVERY_CONSUMER_ASSERTIONS) {
        if (localChecks[assertion]) {
          observe(testId, assertion, `${kind} ${primary} consumer on ${local.platform}: ${assertion}`);
        }
      }
      const coverage = supportedPlatformCoverage(
        reports.map(({ report }) => report),
        sides,
        expected,
        deliveryContext,
      );
      if (coverage.observed) {
        observe(
          testId,
          'supportedPlatforms',
          `${kind} ${primary} consumer on ${corpus.delivery.supportedPlatforms.join(', ')}`,
        );
      } else {
        for (const { platform, reason } of coverage.failures) {
          console.log(`issue-195 evidence: ${testId} supportedPlatforms rejects ${platform}: ${reason}`);
        }
      }
      const rmlRuntimes = crossRuntime ? ['javascript', 'rust'] : [primary === 'npm' ? 'javascript' : 'rust'];
      if (rmlRuntimes.every((rmlRuntime) => rml[rmlRuntime].ok)) {
        observe(testId, 'downstreamRmlIntegration', `downstream RML ${rmlRuntimes.join(' and ')} tests`);
      }
    }
  }
  groups.push(
    ['delivery:npm', bundle(candidates, consumer, reportArtifacts, rml.javascript)],
    ['delivery:crate', bundle(candidates, consumer, reportArtifacts, rml.rust)],
    ['delivery:rml', bundle(candidates, consumer, reportArtifacts, rml.javascript, rml.rust)],
  );
  return groups;
}

async function candidateArtifacts() {
  const source = candidatesDirectory
    ? path.resolve(candidatesDirectory)
    : path.join(workDirectory, 'candidates');
  const record = candidatesDirectory
    ? { commands: [], artifacts: [] }
    : await prepareCandidates(source);
  const names = [
    `meta-language-${packageMetadata.version}.tgz`,
    `meta-language-${packageMetadata.version}.crate`,
  ];
  const artifacts = [];
  for (const name of names) {
    for (const file of [name, `${name}.sha256`]) {
      await copyFile(path.join(source, file), path.join(artifactsDirectory, file));
      artifacts.push(relative(path.join(artifactsDirectory, file)));
    }
  }
  return {
    commands: record.commands,
    artifacts: [...record.artifacts, ...artifacts],
    tarball: path.join(artifactsDirectory, names[0]),
    crate: path.join(artifactsDirectory, names[1]),
  };
}

async function readPlatformReports(localReport) {
  const files = platformReportsDirectory
    ? await jsonFiles(path.resolve(platformReportsDirectory))
    : [];
  if (localReport) files.push(localReport);
  const reports = [];
  for (const file of files) {
    const report = JSON.parse(await readFile(file, 'utf8'));
    const artifact = path.join(artifactsDirectory, 'platform-reports', path.basename(path.dirname(file)), path.basename(file));
    await mkdir(path.dirname(artifact), { recursive: true });
    await copyFile(file, artifact);
    reports.push({ file, report, artifact: relative(artifact) });
  }
  return reports;
}

async function jsonFiles(directory) {
  const files = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await jsonFiles(file));
    else if (entry.name.endsWith('.json')) files.push(file);
  }
  return files.sort();
}

/** Queues one passed execution record per fixture of the verification cell. */
function observe(testId, assertionId, testName) {
  const cell = cellsByTestId.get(testId);
  if (!cell) throw new Error(`observation for unknown testId ${testId}`);
  if (!cell.assertions.includes(assertionId)) {
    throw new Error(`observation for undeclared assertion ${testId}/${assertionId}`);
  }
  for (const fixtureId of cell.fixtureIds) {
    runnerObservations.push({
      testId,
      assertionId,
      fixtureId,
      fixtureDigest: manifest.fixtureCatalog[fixtureId].sha256,
      runtime: cell.runtime,
      commit,
      outcome: 'passed',
      testName,
    });
  }
}

function failedRecord(error) {
  return { commands: [], artifacts: [], ok: false, error: error.message };
}

async function validateDownstreamRml(npmCandidate, crateCandidate, kind = 'candidate') {
  const suffix = kind === 'candidate' ? '' : `-${kind}`;
  const label = `delivery-rml${suffix}`;
  const directory = path.join(workDirectory, `relative-meta-logic${suffix}`);
  const clone = await runCommand(`${label}-clone`, 'git', [
    'clone', '--quiet', corpus.downstreamRml.repository, directory,
  ]);
  const checkout = await runCommand(`${label}-checkout`, 'git', [
    'checkout', '--quiet', corpus.downstreamRml.revision,
  ], { cwd: directory });
  const javascriptDirectory = path.join(directory, 'js');
  const javascriptInstall = await runCommand(`${label}-javascript-install`, 'npm', [
    'install', '--ignore-scripts', '--no-audit', '--no-fund', npmCandidate.tarball,
  ], { cwd: javascriptDirectory });
  const javascriptTest = await runCommand(`${label}-javascript-test`, 'node', [
    '--test', corpus.downstreamRml.javascriptTest.replace(/^js\//u, ''),
  ], {
    cwd: javascriptDirectory,
    env: { ...boundedEnvironment(process.env), NO_PROXY: '*', no_proxy: '*' },
    allowFailure: true,
  });

  const rustDirectory = path.join(directory, 'rust');
  const cargoPath = path.join(rustDirectory, 'Cargo.toml');
  const cargoText = await readFile(cargoPath, 'utf8');
  const patchedCargo = cargoText.replace(
    /^meta-language\s*=.*$/mu,
    `meta-language = { path = ${JSON.stringify(crateCandidate.crateSource)} }`,
  );
  if (patchedCargo === cargoText) throw new Error('RML Cargo.toml has no meta-language dependency');
  await writeFile(cargoPath, patchedCargo);
  const rustFetch = await runCommand(`${label}-rust-fetch`, 'cargo', ['fetch'], {
    cwd: rustDirectory,
  });
  const rustTest = await runCommand(`${label}-rust-test`, 'cargo', [
    'test', '--no-fail-fast', '--offline', '--test', 'meta_language_support_tests',
  ], { cwd: rustDirectory, allowFailure: true });
  return {
    javascript: { ...bundle(clone, checkout, javascriptInstall, javascriptTest), ok: javascriptTest.ok },
    rust: { ...bundle(clone, checkout, rustFetch, rustTest), ok: rustTest.ok },
  };
}

async function runCommandWithRetry(label, command, args, options = {}) {
  const attempts = 30;
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    try {
      return await runCommand(`${label}-${attempt}`, command, args, options);
    } catch (error) {
      if (attempt === attempts) throw error;
      console.log(`issue-195 evidence: ${label} is not available (attempt ${attempt}/${attempts})`);
      await new Promise((resolve) => setTimeout(resolve, 20_000));
    }
  }
  throw new Error(`${label} exhausted all attempts`);
}

async function runCommand(
  label,
  command,
  args,
  { cwd = root, env = boundedEnvironment(process.env), allowFailure = false } = {},
) {
  const logPath = path.join(logsDirectory, `${label}.log`);
  const handle = await open(logPath, 'w');
  const rendered = renderCommand(command, args);
  await handle.write(`command: ${rendered}\ncwd: ${cwd}\n\n`);
  console.log(`issue-195 evidence: ${label}`);
  let code;
  try {
    code = await new Promise((resolve, reject) => {
      const child = spawn(command, args, { cwd, env, stdio: ['ignore', handle.fd, handle.fd] });
      child.once('error', reject);
      child.once('close', resolve);
    });
    await handle.write(`\nexit: ${code}\n`);
  } finally {
    await handle.close();
  }
  if (code !== 0 && !allowFailure) {
    throw new Error(`${label} failed with exit ${code}; see ${relative(logPath)}`);
  }
  if (code !== 0) console.log(`issue-195 evidence: ${label} failed with exit ${code}`);
  return { commands: [rendered], artifacts: [relative(logPath)], ok: code === 0 };
}

function bundle(...records) {
  return {
    commands: [...new Set(records.flatMap((record) => record.commands))],
    artifacts: [...new Set(records.flatMap((record) => record.artifacts))],
    failureLogs: [...new Set(records.flatMap((record) => record.failureLogs ?? []))],
  };
}

function renderCommand(command, args) {
  return [command, ...args].map((part) => {
    const value = String(part);
    return /^[A-Za-z0-9_./:=@+-]+$/u.test(value)
      ? value
      : `'${value.replaceAll("'", "'\\''")}'`;
  }).join(' ');
}

function commandOutput(command, args) {
  return execFileSync(command, args, { encoding: 'utf8' }).trim().replace(/\s+/gu, ' ');
}

function git(args) {
  return commandOutput('git', args);
}

function relative(filename) {
  return path.relative(root, filename).split(path.sep).join('/');
}

function stableJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

async function sha256File(filename) {
  return createHash('sha256').update(await readFile(filename)).digest('hex');
}
