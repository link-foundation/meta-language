#!/usr/bin/env node
// Runs the link-assistant/formal-ai workloads against the exact meta-language
// candidate artifacts (requirement I195-DOWNSTREAM-FORMAL-AI-WORKLOADS).
//
// The runner checks out the pinned revision of parity/fixtures/formal-ai-workloads.json
// and verifies the git blob of every inventoried file. formal-ai's Rust crate
// depends on meta-language, so the runner unpacks the .crate, patches formal-ai's
// rust/ to it with `cargo --config 'patch.crates-io.meta-language.path="…"'`,
// refreshes formal-ai's Cargo.lock for meta-language alone (the candidate may need
// newer dependencies than the lock pins), records that the build resolves
// meta-language to the artifact, and runs only
// formal-ai's inventoried meta-language test groups (libtest filters of the
// `unit` and `source` targets) with the probe of
// js/scripts/issue-195-formal-ai-rust-probe.mjs. formal-ai's npm package declares
// no meta-language dependency, so the JavaScript side installs the npm tarball
// into a clean consumer (empty node_modules, fresh cache) and runs formal-ai's
// data through the public API (js/scripts/issue-195-formal-ai-consumer.mjs).
// Both runtimes write the same outputs from one inputs file; their comparison
// joins both sharedConceptsReused observations. The JSON report is validated by
// js/scripts/issue-195-formal-ai-workloads.mjs; the exit status is non-zero when
// a run runtime has a problem.
//
// Usage:
//   node js/scripts/run-formal-ai-workloads.mjs --npm-tarball <meta-language-x.y.z.tgz>
//     --crate <meta-language-x.y.z.crate> --work-dir <directory> --out <report.json>
//     [--formal-ai-checkout <git checkout holding the pinned revision>] [--skip-rust]
// The tarball and the crate are each expected next to a `<artifact>.sha256`
// file ("<digest>  <name>"), as the delivery candidates are published.
// --skip-rust is for local runs only: the report then has no Rust run, the Rust
// assertions stay unobserved and the comparison fails for want of Rust outputs.
// CI runs both runtimes.
import { createHash } from 'node:crypto';
import { gunzipSync } from 'node:zlib';
import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { gitBlob } from './issue-195-rml-pr184.mjs';
import {
  JAVASCRIPT_INPUTS_VARIABLE,
  JAVASCRIPT_PROBE,
  JAVASCRIPT_WORKLOAD_TESTS,
  TEST_NAME_SEPARATOR,
  consumerHelpersModule,
} from './issue-195-formal-ai-consumer.mjs';
import {
  RUST_PROBE,
  RUST_PROBE_ENVIRONMENT,
  RUST_PROBE_TARGET,
  RUST_PROBE_TESTS,
} from './issue-195-formal-ai-rust-probe.mjs';
import {
  FORMAL_AI_DATA_FILES,
  TEST_EVENT_REPORTER,
  buildFormalAiInputs,
  compareRuntimeOutputs,
} from './issue-195-formal-ai-workload-probes.mjs';
import {
  FORMAL_AI_PROBE_ASSERTIONS,
  FORMAL_AI_WORKLOAD_FIXTURE,
  FORMAL_AI_WORKLOAD_REQUIREMENT,
  FORMAL_AI_WORKLOAD_SCHEMA_VERSION,
  SELF_AST_CENSUS_DIRECTORY,
  SELF_AST_CENSUS_EXAMPLE,
  compareSelfAstCensus,
  countOutcomes,
  loadFormalAiWorkloadFixture,
  validateFormalAiWorkloadReport,
  workloadFiles,
} from './issue-195-formal-ai-workloads.mjs';
import { satisfies } from './semver-range.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const skipRust = process.argv.includes('--skip-rust');
const tarball = path.resolve(required('--npm-tarball'));
const crate = skipRust ? null : path.resolve(required('--crate'));
const workDirectory = path.resolve(required('--work-dir'));
const output = path.resolve(required('--out'));
const formalAiCheckout = option('--formal-ai-checkout', null);
const fixture = loadFormalAiWorkloadFixture(root);
// The cargo test targets of formal-ai's workload tests, by the file cargo announces each under.
const RUST_TARGETS = Object.freeze({ 'rust/tests/unit/mod.rs': 'unit', 'rust/tests/source/mod.rs': 'source' });
const PROBE_FILE = `rust/tests/${RUST_PROBE_TARGET}.rs`;

const logs = [];

async function main() {
  await rm(workDirectory, { recursive: true, force: true });
  await mkdir(workDirectory, { recursive: true });
  const formalAi = await checkoutFormalAi();
  const mismatched = formalAi.verifiedBlobs.filter(({ matches }) => !matches);
  const skipped = (reason) => ({ skipped: true, reason });
  const report = {
    schemaVersion: FORMAL_AI_WORKLOAD_SCHEMA_VERSION,
    requirementId: FORMAL_AI_WORKLOAD_REQUIREMENT,
    fixture: FORMAL_AI_WORKLOAD_FIXTURE,
    fixtureSha256: sha256(await readFile(path.join(root, FORMAL_AI_WORKLOAD_FIXTURE))),
    platform: `${process.platform}-${process.arch}`,
    node: process.version,
    formalAi: formalAi.report,
  };
  const outputs = {};
  if (mismatched.length > 0) {
    const reason = `the checkout differs from the pinned workloads: ${mismatched.map(({ file }) => file).join(', ')}`;
    report.npm = skipped(reason);
    report.crate = skipped(reason);
  } else {
    const inputsPath = path.join(workDirectory, 'formal-ai-inputs.json');
    const inputs = await buildFormalAiInputs(formalAi.directory);
    await writeFile(inputsPath, `${JSON.stringify(inputs)}\n`);
    report.inputs = {
      file: path.basename(inputsPath),
      sha256: sha256(await readFile(inputsPath)),
      documents: inputs.documents.length,
      labels: inputs.labels,
      rules: inputs.rules.length,
      linkEdits: inputs.linkEdits.length,
      projections: inputs.projections.length,
    };
    report.npm = await guarded(() => javascriptWorkloads(formalAi.directory, inputsPath, outputs));
    report.crate = skipRust
      ? skipped('--skip-rust: the Rust workloads were not run')
      : await guarded(() => rustWorkloads(formalAi.directory, inputsPath, outputs));
    // The two runtimes reuse the same concepts only when they produced the same links and projections.
    const comparison = compareRuntimeOutputs(outputs.javascript, outputs.rust);
    report.comparison = comparison;
    for (const part of [report.npm, report.crate]) {
      const shared = part.probe?.sharedConceptsReused;
      if (!shared) continue;
      shared.checks.push(...comparison);
      shared.holds = shared.checks.every(({ holds }) => holds === true);
    }
  }
  report.logs = logs;
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
  console.log(`formal-ai workloads: wrote ${output}`);

  let failed = mismatched.length > 0;
  for (const runtime of ['javascript', 'rust']) {
    if (runtime === 'rust' && skipRust) {
      console.log('rust: not run (--skip-rust)');
      continue;
    }
    const { problems, observed } = validateFormalAiWorkloadReport(report, fixture, runtime);
    const part = report[runtime === 'javascript' ? 'npm' : 'crate'];
    console.log(`${runtime}: ${part.counts ? `${part.counts.passed}/${part.counts.total} workload tests passed; ` : ''}observed ${observed.join(', ') || 'nothing'}`);
    for (const [assertion, messages] of Object.entries(problems)) {
      for (const message of messages) console.error(`  ${runtime} ${assertion}: ${message}`);
    }
    failed ||= observed.length !== Object.keys(problems).length;
  }
  if (failed) process.exitCode = 1;
}

// A runtime whose setup throws is still reported, with the error, so the report always exists.
async function guarded(body) {
  try {
    return await body();
  } catch (error) {
    console.error(error);
    return { skipped: false, error: String(error?.message ?? error), exit: null };
  }
}

async function checkoutFormalAi() {
  const directory = path.join(workDirectory, 'formal-ai');
  const source = formalAiCheckout ? path.resolve(formalAiCheckout) : `https://github.com/${fixture.repository}.git`;
  await run('formal-ai-init', 'git', ['init', '--quiet', directory]);
  await run('formal-ai-fetch', 'git', ['-C', directory, 'fetch', '--quiet', '--depth', '1', source, fixture.revision]);
  await run('formal-ai-checkout', 'git', [
    '-C', directory, '-c', 'advice.detachedHead=false', 'checkout', '--quiet', fixture.revision,
  ]);
  const checkedOutRevision = (await run('formal-ai-revision', 'git', ['-C', directory, 'rev-parse', 'HEAD'])).stdout.trim();
  const verifiedBlobs = [];
  for (const { file, kind, blob } of fixture.workloads) {
    const target = path.join(directory, file);
    const actual = existsSync(target) ? gitBlob(await readFile(target)) : null;
    verifiedBlobs.push({ file, kind, expected: blob, actual, matches: actual === blob });
  }
  return {
    directory,
    verifiedBlobs,
    report: {
      repository: fixture.repository,
      revision: fixture.revision,
      checkedOutRevision,
      source: formalAiCheckout ? 'a local checkout (--formal-ai-checkout)' : source,
      verifiedBlobs,
    },
  };
}

async function javascriptWorkloads(formalAiDirectory, inputsPath, outputs) {
  const directory = path.join(workDirectory, 'javascript-consumer');
  const cache = path.join(workDirectory, 'npm-cache');
  const bytes = await readFile(tarball);
  const unpacked = path.join(workDirectory, 'npm-artifact');
  await extractArchive(bytes, unpacked);
  const version = JSON.parse(await readFile(path.join(unpacked, 'package', 'package.json'), 'utf8')).version;
  // formal-ai's JavaScript package is its root package.json; it declares no meta-language dependency.
  const formalAiPackage = JSON.parse(await readFile(path.join(formalAiDirectory, 'package.json'), 'utf8'));
  const formalAiDeclaration = Object.entries({
    ...formalAiPackage.dependencies, ...formalAiPackage.devDependencies, ...formalAiPackage.peerDependencies,
  }).find(([name]) => name === 'meta-language')?.[1] ?? null;
  await mkdir(directory, { recursive: true });
  await writeFile(path.join(directory, 'package.json'), `${JSON.stringify({
    name: 'issue-195-formal-ai-consumer', version: '0.0.0', private: true, type: 'module',
  }, null, 2)}\n`);
  const consumerHadNoModules = !existsSync(path.join(directory, 'node_modules'));
  const freshCache = !existsSync(cache);
  const environment = {
    ...childEnvironment(),
    npm_config_cache: cache,
    npm_config_update_notifier: 'false',
    NODE_OPTIONS: '',
    [JAVASCRIPT_INPUTS_VARIABLE]: inputsPath,
  };
  await run('npm-install', 'npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', tarball], {
    cwd: directory,
    env: environment,
  });
  const lock = JSON.parse(await readFile(path.join(directory, 'package-lock.json'), 'utf8'));
  const installed = lock.packages?.['node_modules/meta-language'] ?? {};
  const installedManifest = JSON.parse(
    await readFile(path.join(directory, 'node_modules', 'meta-language', 'package.json'), 'utf8'),
  );
  const resolution = {
    formalAiDeclaration,
    installedRequirement: JSON.parse(await readFile(path.join(directory, 'package.json'), 'utf8'))
      .dependencies?.['meta-language'] ?? null,
    lockResolved: installed.resolved ?? null,
    installedVersion: installedManifest.version,
    consumerHadNoModules,
    freshCache,
  };
  const checksum = {
    artifact: path.basename(tarball),
    sha256: sha256(bytes),
    expectedSha256: await expectedChecksum(tarball),
    installedIntegrity: installed.integrity ?? null,
    expectedIntegrity: `sha512-${createHash('sha512').update(bytes).digest('base64')}`,
  };

  const dataFiles = new Set(Object.values(FORMAL_AI_DATA_FILES));
  const events = path.join(workDirectory, 'javascript-test-events.jsonl');
  await writeFile(path.join(directory, 'formal-ai-helpers.mjs'), consumerHelpersModule());
  await writeFile(path.join(directory, 'formal-ai-workloads.test.mjs'), JAVASCRIPT_WORKLOAD_TESTS);
  await writeFile(path.join(directory, 'issue-195-test-events.mjs'), TEST_EVENT_REPORTER);
  const testRun = await run('javascript-workloads', process.execPath, [
    '--test', '--test-concurrency=1',
    '--test-reporter=spec', '--test-reporter-destination=stdout',
    '--test-reporter=./issue-195-test-events.mjs', `--test-reporter-destination=${events}`,
    'formal-ai-workloads.test.mjs',
  ], { cwd: directory, env: environment, allowFailure: true });
  // Each test name starts with the formal-ai data file it ran; a test without one is kept unattributed.
  const tests = existsSync(events)
    ? (await readFile(events, 'utf8')).split('\n').filter(Boolean).map((line) => JSON.parse(line))
      .filter(({ kind }) => kind === 'test')
      .map(({ name, outcome, error, process: child }) => {
        const [prefix, ...rest] = name.split(TEST_NAME_SEPARATOR);
        const attributed = dataFiles.has(prefix);
        return {
          file: attributed ? prefix : null,
          name: attributed ? rest.join(TEST_NAME_SEPARATOR) : name,
          outcome,
          ...(error ? { error } : {}),
          ...(child ? { process: child } : {}),
        };
      })
    : [];

  const probePath = path.join(workDirectory, 'javascript-probe.json');
  await writeFile(path.join(directory, 'issue-195-meta-language-probe.mjs'), JAVASCRIPT_PROBE);
  const probeRun = await run('javascript-probe', process.execPath, ['issue-195-meta-language-probe.mjs', probePath, inputsPath], {
    cwd: directory,
    env: environment,
    allowFailure: true,
  });
  const probe = existsSync(probePath) ? JSON.parse(await readFile(probePath, 'utf8')) : {};
  outputs.javascript = probe.outputs;
  return {
    skipped: false,
    version,
    checksum,
    resolution,
    workloadFiles: workloadFiles(fixture, 'javascript'),
    tests,
    counts: countOutcomes(tests),
    exit: testRun.code,
    outputs: await outputsFile('javascript', probe.outputs),
    probe: probeAssertions(probe, probeRun),
  };
}

async function rustWorkloads(formalAiDirectory, inputsPath, outputs) {
  const directory = path.join(formalAiDirectory, 'rust');
  const extracted = path.join(workDirectory, 'crate-source');
  const target = path.join(workDirectory, 'formal-ai-target');
  const probeDirectory = path.join(workDirectory, 'rust-probe');
  const bytes = await readFile(crate);
  const checksum = {
    artifact: path.basename(crate),
    sha256: sha256(bytes),
    expectedSha256: await expectedChecksum(crate),
  };
  await extractArchive(bytes, extracted);
  const crateSource = path.join(extracted, path.basename(crate).replace(/\.crate$/u, ''));
  const version = /^version = "([^"]+)"/mu.exec(await readFile(path.join(crateSource, 'Cargo.toml'), 'utf8'))?.[1] ?? null;

  // A path patch applies only to a version the dependency requirement accepts; a candidate
  // outside formal-ai's requirement is pinned exactly, and the report says so.
  const manifestPath = path.join(directory, 'Cargo.toml');
  const manifest = await readFile(manifestPath, 'utf8');
  const requirementLine = /^meta-language = \{ version = "([^"]+)"(.*)\}$/mu;
  const declaredRequirement = requirementLine.exec(manifest)?.[1] ?? null;
  const declaredRequirementSatisfied = Boolean(declaredRequirement) && satisfies(version, declaredRequirement, 'cargo');
  const effectiveRequirement = declaredRequirementSatisfied ? declaredRequirement : `=${version}`;
  if (!declaredRequirementSatisfied) {
    await writeFile(manifestPath, manifest.replace(requirementLine,
      (line, requirement, rest) => `meta-language = { version = "${effectiveRequirement}"${rest}}`));
  }
  const unpackedDirectory = crateSource.split(path.sep).join('/');
  const configuration = ['--config', `patch.crates-io.meta-language.path=${JSON.stringify(unpackedDirectory)}`];
  await writeFile(path.join(directory, 'tests', `${RUST_PROBE_TARGET}.rs`), RUST_PROBE);
  await mkdir(probeDirectory, { recursive: true });
  const environment = {
    ...childEnvironment(),
    CARGO_TARGET_DIR: target,
    // CI exports CARGO_TERM_COLOR=always; plain output keeps the libtest transcript parseable.
    CARGO_TERM_COLOR: 'never',
    [RUST_PROBE_ENVIRONMENT.inputs]: inputsPath,
    [RUST_PROBE_ENVIRONMENT.directory]: probeDirectory,
  };

  // formal-ai's Cargo.lock pins the dependencies of the published meta-language; the candidate
  // may need newer ones (for instance tree-sitter-language ^0.1.8 where the lock holds 0.1.7), so
  // the lock is refreshed for meta-language alone before cargo resolves the patched build.
  const lockPath = path.join(directory, 'Cargo.lock');
  const lockBefore = await readFile(lockPath, 'utf8');
  const lockRefreshArguments = [...configuration, 'update', '-p', 'meta-language'];
  const lockRefreshRun = await run('rust-lock-refresh', 'cargo', lockRefreshArguments, { cwd: directory, env: environment });
  const lockRefresh = {
    command: ['cargo', ...lockRefreshArguments].join(' '),
    exit: lockRefreshRun.code,
    lockChanged: (await readFile(lockPath, 'utf8')) !== lockBefore,
  };
  const metadata = JSON.parse((await run('rust-metadata', 'cargo', [
    ...configuration, 'metadata', '--format-version', '1',
  ], { cwd: directory, env: environment })).stdout);
  const metaLanguagePackages = metadata.packages.filter(({ name }) => name === 'meta-language');
  const metaLanguage = metaLanguagePackages[0] ?? {};
  const selfAstCensus = await refreshSelfAstCensus(formalAiDirectory, directory, configuration, environment);
  const workloads = fixture.workloads.filter(({ kind, file }) => kind === 'test' && file.startsWith('rust/'));
  const filters = [...new Set([...workloads.map(({ filter }) => filter), 'issue_195_probe_'])];
  // formal-ai's own test groups, filtered to the inventoried modules, and the probe, in one patched build.
  const testRun = await run('rust-workloads', 'cargo', [
    ...configuration, 'test', '--no-fail-fast',
    ...[...new Set(workloads.map((entry) => entry.target)), RUST_PROBE_TARGET].flatMap((name) => ['--test', name]),
    '--', ...filters,
  ], { cwd: directory, env: environment, allowFailure: true });
  const executed = libtestOutcomes(testRun.combined).map((entry) => {
    const targetName = RUST_TARGETS[entry.file];
    const workload = workloads.find((candidate) => candidate.target === targetName && entry.name.startsWith(candidate.filter));
    return workload ? { ...entry, file: workload.file } : entry;
  });
  const lock = await readFile(lockPath, 'utf8');
  const lockEntry = /\[\[package\]\]\nname = "meta-language"\nversion = "([^"]+)"\n(?:source = "([^"]+)"\n)?/u.exec(lock);
  const resolution = {
    declaredRequirement,
    declaredRequirementSatisfied,
    effectiveRequirement,
    patch: configuration.join(' '),
    lockRefresh,
    unpackedDirectory: `${unpackedDirectory}/`,
    manifestPath: metaLanguage.manifest_path?.split(path.sep).join('/') ?? null,
    metadataSource: metaLanguage.source ?? null,
    metaLanguagePackages: metaLanguagePackages.length,
    lockVersion: lockEntry?.[1] ?? null,
    lockSource: lockEntry ? lockEntry[2] ?? null : 'no meta-language entry',
    installedVersion: metaLanguage.version ?? null,
  };

  const tests = executed.filter(({ file }) => file !== PROBE_FILE);
  const probeTests = executed.filter(({ file }) => file === PROBE_FILE);
  const outcome = (name) => probeTests.find((entry) => entry.name === name)?.outcome ?? 'not run';
  const evidence = async (name) => {
    const file = path.join(probeDirectory, `${name}.json`);
    return existsSync(file) ? JSON.parse(await readFile(file, 'utf8')) : null;
  };
  const probeFailure = (name) => check(`probe test ${name}`, {
    holds: false,
    detail: `the probe test ${outcome(name)}; see ${testRun.log}`,
  });
  outputs.rust = outcome(RUST_PROBE_TESTS.outputs) === 'passed' ? await evidence('rust-outputs') ?? undefined : undefined;
  const distinctions = await evidence('distinctionsPreserved');
  const distinctionChecks = distinctions?.checks ?? [];
  if (distinctionChecks.length === 0 ||
      (outcome(RUST_PROBE_TESTS.distinctionsPreserved) !== 'passed' && distinctionChecks.every(({ holds }) => holds === true))) {
    distinctionChecks.push(probeFailure(RUST_PROBE_TESTS.distinctionsPreserved));
  }
  const probe = {
    sharedConceptsReused: {
      test: RUST_PROBE_TESTS.outputs,
      outcome: outcome(RUST_PROBE_TESTS.outputs),
      checks: [
        outputs.rust
          ? check("formal-ai's Rust functions wrote the shared outputs", { holds: true, detail: { test: RUST_PROBE_TESTS.outputs } })
          : probeFailure(RUST_PROBE_TESTS.outputs),
        check('the formal-ai build links one meta-language, the unpacked crate', {
          holds: metaLanguagePackages.length === 1 && resolution.metadataSource === null &&
            Boolean(resolution.manifestPath?.startsWith(resolution.unpackedDirectory)),
          detail: { packages: metaLanguagePackages.length, manifestPath: resolution.manifestPath },
        }),
      ],
    },
    distinctionsPreserved: {
      test: RUST_PROBE_TESTS.distinctionsPreserved,
      outcome: outcome(RUST_PROBE_TESTS.distinctionsPreserved),
      checks: [
        ...distinctionChecks,
        check("formal-ai's self-AST census keeps every named node, node kind and symbol with the candidate", {
          holds: selfAstCensus.exit === 0 && selfAstCensus.otherChanges.length === 0,
          detail: { exit: selfAstCensus.exit, log: selfAstCensus.log, otherChanges: selfAstCensus.otherChanges },
        }),
      ],
    },
  };
  for (const assertion of FORMAL_AI_PROBE_ASSERTIONS) {
    probe[assertion].holds = probe[assertion].checks.every(({ holds }) => holds === true);
  }
  return {
    skipped: false,
    version,
    checksum,
    resolution,
    selfAstCensus,
    filters,
    workloadFiles: workloadFiles(fixture, 'rust'),
    tests,
    counts: countOutcomes(tests),
    exit: testRun.code,
    outputs: await outputsFile('rust', outputs.rust),
    probeTests,
    probe,
  };
}

// formal-ai commits a census of its own sources as meta-language parses them
// (data/meta/self-ast), and a test compares it with a fresh rendering. Its
// total_link_count counts every link of the lossless network, so a candidate
// that adds links (the grammar provenance link, hidden-text tokens) leaves the
// counts that the published crate rendered stale. formal-ai refreshes the
// documents on every meta-language upgrade with its own example; the runner
// does the same before the tests and reports every changed line. Only link
// totals may change: anything else is a regression of the candidate's parse.
async function refreshSelfAstCensus(formalAiDirectory, directory, configuration, environment) {
  const censusDirectory = path.join(formalAiDirectory, SELF_AST_CENSUS_DIRECTORY);
  const before = await readCensus(censusDirectory);
  const refresh = await run('rust-self-ast-census-refresh', 'cargo', [
    ...configuration, 'run', '--quiet', '--example', SELF_AST_CENSUS_EXAMPLE,
  ], { cwd: directory, env: environment, allowFailure: true });
  const comparison = compareSelfAstCensus(before, await readCensus(censusDirectory));
  return {
    command: ['cargo', 'run', '--example', SELF_AST_CENSUS_EXAMPLE].join(' '),
    exit: refresh.code,
    log: refresh.log,
    ...comparison,
  };
}

async function readCensus(directory) {
  const documents = new Map();
  if (!existsSync(directory)) return documents;
  for (const entry of await readdir(directory, { recursive: true, withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith('.lino')) continue;
    const file = path.join(entry.parentPath ?? entry.path, entry.name);
    documents.set(path.relative(directory, file).split(path.sep).join('/'), await readFile(file, 'utf8'));
  }
  return documents;
}

function check(name, { holds, detail }) {
  return { name, holds, detail };
}

// The outputs of one runtime are kept beside the logs; the report names the file and its digest.
async function outputsFile(runtime, value) {
  if (!value) return null;
  const file = path.join(workDirectory, `${runtime}-outputs.json`);
  await writeFile(file, `${JSON.stringify(value, null, 2)}\n`);
  return { file: path.basename(file), sha256: sha256(await readFile(file)) };
}

// The JavaScript probe's observations, or a failed check carrying why it wrote none.
function probeAssertions(probe, probeRun) {
  return Object.fromEntries(FORMAL_AI_PROBE_ASSERTIONS.map((assertion) => [
    assertion,
    probe[assertion] ?? {
      holds: false,
      checks: [{ name: 'probe run', holds: false, detail: `the probe exited with ${probeRun.code}; see ${probeRun.log}` }],
    },
  ]));
}

// The libtest outcome of every test, by the integration test file cargo announced it under.
function libtestOutcomes(text) {
  const outcomes = [];
  let file = null;
  // Strip ANSI styling in case a caller's environment forces coloured cargo output.
  for (const line of text.replace(/\u001b\[[0-9;]*m/gu, '').split(/\r?\n/u)) {
    const running = /^\s*Running (?:unittests )?(\S+\.rs)\b/u.exec(line);
    if (running) {
      file = `rust/${running[1].split('\\').join('/')}`;
      continue;
    }
    const result = /^test (\S+) \.\.\. (ok|FAILED|ignored)\b/u.exec(line);
    if (result && file) {
      outcomes.push({ file, name: result[1], outcome: { ok: 'passed', FAILED: 'failed', ignored: 'skipped' }[result[2]] });
    }
  }
  return outcomes;
}

// Extracts a gzip tar archive without an external `tar` (as in run-issue-195-consumer.mjs).
async function extractArchive(bytes, destination) {
  const archive = gunzipSync(bytes);
  let longName = null;
  for (let offset = 0; offset + 512 <= archive.length;) {
    const header = archive.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) break;
    const field = (start, length) => header.subarray(start, start + length).toString('utf8').replace(/\0.*$/su, '');
    const size = Number.parseInt(field(124, 12).trim() || '0', 8);
    const type = field(156, 1) || '0';
    const body = archive.subarray(offset + 512, offset + 512 + size);
    offset += 512 + Math.ceil(size / 512) * 512;
    if (type === 'L') {
      longName = body.toString('utf8').replace(/\0.*$/su, '');
      continue;
    }
    if (type === 'x') {
      longName = /\d+ path=([^\n]*)\n/u.exec(body.toString('utf8'))?.[1] ?? longName;
      continue;
    }
    const prefix = field(345, 155);
    const name = longName ?? (prefix ? `${prefix}/${field(0, 100)}` : field(0, 100));
    longName = null;
    const target = path.resolve(destination, name);
    if (!target.startsWith(path.resolve(destination) + path.sep)) {
      throw new Error(`archive entry escapes the extraction directory: ${name}`);
    }
    if (type === '5') {
      await mkdir(target, { recursive: true });
    } else if (type === '0' || type === '7') {
      await mkdir(path.dirname(target), { recursive: true });
      await writeFile(target, body);
    } else if (type !== 'g') {
      throw new Error(`unsupported archive entry type ${type} for ${name}`);
    }
  }
}

async function expectedChecksum(artifact) {
  const text = await readFile(`${artifact}.sha256`, 'utf8');
  const [digest, name] = text.trim().split(/\s+/u);
  if (name !== path.basename(artifact)) {
    throw new Error(`checksum file names ${name}, expected ${path.basename(artifact)}`);
  }
  return digest;
}

// The parent environment with a single PATH entry on Windows (as in run-issue-195-consumer.mjs).
function childEnvironment() {
  const environment = { ...process.env };
  if (process.platform !== 'win32') return environment;
  const names = Object.keys(environment).filter((name) => name.toUpperCase() === 'PATH');
  const entries = names.flatMap((name) => environment[name].split(path.delimiter)).filter(Boolean);
  for (const name of names) delete environment[name];
  environment.PATH = [...new Set(entries)].join(path.delimiter);
  return environment;
}

// Runs a command, logs it to <work-dir>/<label>.log and returns its output; the output of both
// streams is also kept in arrival order, since cargo announces test binaries on stderr.
async function run(label, command, args, { cwd = workDirectory, env = childEnvironment(), allowFailure = false } = {}) {
  const log = path.join(workDirectory, `${label}.log`);
  const rendered = [command, ...args].join(' ');
  const { code, stdout, stderr, combined } = await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      env,
      shell: process.platform === 'win32' && ['npm', 'cargo'].includes(command),
    });
    const out = [];
    const err = [];
    const both = [];
    child.stdout.on('data', (chunk) => { out.push(chunk); both.push(chunk); });
    child.stderr.on('data', (chunk) => { err.push(chunk); both.push(chunk); });
    child.once('error', reject);
    child.once('close', (exit) => resolve({
      code: exit,
      stdout: Buffer.concat(out).toString('utf8'),
      stderr: Buffer.concat(err).toString('utf8'),
      combined: Buffer.concat(both).toString('utf8'),
    }));
  });
  await writeFile(log, `command: ${rendered}\ncwd: ${cwd}\n\n${combined}\nexit: ${code}\n`);
  logs.push({ label, command: rendered, log, exit: code });
  if (code !== 0 && !allowFailure) {
    const tail = combined.trim().split('\n').slice(-40).join('\n');
    throw new Error(`${label} failed with exit ${code}; see ${log}\n${tail}`);
  }
  return { code, stdout, stderr, combined, log };
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function option(name, fallback) {
  const index = process.argv.indexOf(name);
  return index === -1 ? fallback : process.argv[index + 1];
}

function required(name) {
  const value = option(name, undefined);
  if (!value) throw new Error(`missing ${name}`);
  return value;
}

await main();
