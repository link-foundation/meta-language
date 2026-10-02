#!/usr/bin/env node
// Runs the relative-meta-logic pull request 184 workloads against the exact
// meta-language candidate artifacts (requirement I195-DOWNSTREAM-RML-WORKLOADS).
//
// The runner checks out the pinned head of parity/fixtures/rml-pr184-workloads.json,
// verifies the git blob of every inventoried workload file, installs the npm
// tarball into RML's js/ from an empty node_modules and a fresh cache, unpacks
// the .crate and patches RML's rust/ to it with
// `cargo --config 'patch.crates-io.meta-language.path="…"'`, and records that
// both consumers resolve meta-language to the artifact. It then runs RML's own
// workload tests (node --test with a JSON-lines reporter, and cargo test of the
// two workload targets) together with the consumer probes of
// js/scripts/issue-195-rml-workload-probes.mjs, and writes a JSON report that
// js/scripts/issue-195-rml-workloads.mjs validates. The exit status is non-zero
// when a run runtime has a problem.
//
// Usage:
//   node js/scripts/run-rml-pr184-workloads.mjs --npm-tarball <meta-language-x.y.z.tgz>
//     --crate <meta-language-x.y.z.crate> --work-dir <directory> --out <report.json>
//     [--rml-checkout <git checkout holding the pinned head>] [--skip-rust]
// The tarball and the crate are each expected next to a `<artifact>.sha256`
// file ("<digest>  <name>"), as the delivery candidates are published.
// --skip-rust is for local runs only: the report then has no Rust run and the
// Rust assertions stay unobserved. CI runs both runtimes.
import { createHash } from 'node:crypto';
import { gunzipSync } from 'node:zlib';
import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { gitBlob } from './issue-195-rml-pr184.mjs';
import {
  JAVASCRIPT_PROBE,
  RUST_PROBE,
  RUST_PROBE_TARGET,
  RUST_PROBE_TESTS,
  TEST_EVENT_REPORTER,
} from './issue-195-rml-workload-probes.mjs';
import {
  RML_PROBE_ASSERTIONS,
  RML_WORKLOAD_FIXTURE,
  RML_WORKLOAD_REQUIREMENT,
  RML_WORKLOAD_SCHEMA_VERSION,
  countOutcomes,
  loadRmlWorkloadFixture,
  validateRmlWorkloadReport,
  workloadTestFiles,
} from './issue-195-rml-workloads.mjs';
import { satisfies } from './semver-range.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const skipRust = process.argv.includes('--skip-rust');
const tarball = path.resolve(required('--npm-tarball'));
const crate = skipRust ? null : path.resolve(required('--crate'));
const workDirectory = path.resolve(required('--work-dir'));
const output = path.resolve(required('--out'));
const rmlCheckout = option('--rml-checkout', null);
const fixture = loadRmlWorkloadFixture(root);

const logs = [];

async function main() {
  await rm(workDirectory, { recursive: true, force: true });
  await mkdir(workDirectory, { recursive: true });
  const rml = await checkoutRml();
  const mismatched = rml.verifiedBlobs.filter(({ matches }) => !matches);
  const skipped = (reason) => ({ skipped: true, reason });
  const report = {
    schemaVersion: RML_WORKLOAD_SCHEMA_VERSION,
    requirementId: RML_WORKLOAD_REQUIREMENT,
    fixture: RML_WORKLOAD_FIXTURE,
    fixtureSha256: sha256(await readFile(path.join(root, RML_WORKLOAD_FIXTURE))),
    platform: `${process.platform}-${process.arch}`,
    node: process.version,
    rml: rml.report,
  };
  if (mismatched.length > 0) {
    const reason = `the checkout differs from the pinned workloads: ${mismatched.map(({ file }) => file).join(', ')}`;
    report.npm = skipped(reason);
    report.crate = skipped(reason);
  } else {
    report.npm = await guarded(() => javascriptWorkloads(rml.directory));
    report.crate = skipRust
      ? skipped('--skip-rust: the Rust workloads were not run')
      : await guarded(() => rustWorkloads(rml.directory));
  }
  report.logs = logs;
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
  console.log(`rml pr184 workloads: wrote ${output}`);

  let failed = mismatched.length > 0;
  for (const runtime of ['javascript', 'rust']) {
    if (runtime === 'rust' && skipRust) {
      console.log('rust: not run (--skip-rust)');
      continue;
    }
    const { problems, observed } = validateRmlWorkloadReport(report, fixture, runtime);
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

async function checkoutRml() {
  const directory = path.join(workDirectory, 'rml');
  const source = rmlCheckout ? path.resolve(rmlCheckout) : `https://github.com/${fixture.repository}.git`;
  await run('rml-init', 'git', ['init', '--quiet', directory]);
  await run('rml-fetch', 'git', ['-C', directory, 'fetch', '--quiet', '--depth', '1', source, fixture.headRevision]);
  await run('rml-checkout', 'git', [
    '-C', directory, '-c', 'advice.detachedHead=false', 'checkout', '--quiet', fixture.headRevision,
  ]);
  const checkedOutRevision = (await run('rml-revision', 'git', ['-C', directory, 'rev-parse', 'HEAD'])).stdout.trim();
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
      pullRequest: fixture.pullRequest,
      headRevision: fixture.headRevision,
      checkedOutRevision,
      source: rmlCheckout ? 'a local checkout (--rml-checkout)' : source,
      verifiedBlobs,
    },
  };
}

async function javascriptWorkloads(rmlDirectory) {
  const directory = path.join(rmlDirectory, 'js');
  const cache = path.join(workDirectory, 'npm-cache');
  const bytes = await readFile(tarball);
  const unpacked = path.join(workDirectory, 'npm-artifact');
  await extractArchive(bytes, unpacked);
  const version = JSON.parse(await readFile(path.join(unpacked, 'package', 'package.json'), 'utf8')).version;
  const declaredRequirement = JSON.parse(await readFile(path.join(directory, 'package.json'), 'utf8'))
    .dependencies?.['meta-language'] ?? null;
  const consumerHadNoModules = !existsSync(path.join(directory, 'node_modules'));
  const freshCache = !existsSync(cache);
  const environment = {
    ...childEnvironment(),
    npm_config_cache: cache,
    npm_config_update_notifier: 'false',
    NODE_OPTIONS: '',
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
    declaredRequirement,
    declaredRequirementSatisfied: declaredRequirement ? satisfies(version, declaredRequirement, 'npm') : null,
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

  const files = workloadTestFiles(fixture, 'javascript');
  const events = path.join(workDirectory, 'javascript-test-events.jsonl');
  await writeFile(path.join(directory, 'issue-195-test-events.mjs'), TEST_EVENT_REPORTER);
  // One test file at a time: theory-network.test.mjs reads the whole repository and is the
  // memory peak, so the files do not compete for memory.
  const testRun = await run('javascript-workloads', process.execPath, [
    '--test', '--test-concurrency=1',
    '--test-reporter=spec', '--test-reporter-destination=stdout',
    '--test-reporter=./issue-195-test-events.mjs', `--test-reporter-destination=${events}`,
    ...files.map((file) => path.posix.relative('js', file)),
  ], { cwd: directory, env: environment, allowFailure: true });
  const tests = existsSync(events)
    ? (await readFile(events, 'utf8')).split('\n').filter(Boolean).map((line) => JSON.parse(line))
      .filter(({ kind }) => kind === 'test')
      .map(({ file, name, outcome, error, process: child }) => ({
        file: file ? path.relative(rmlDirectory, file).split(path.sep).join('/') : null,
        name,
        outcome,
        ...(error ? { error } : {}),
        ...(child ? { process: child } : {}),
      }))
    : [];

  const probePath = path.join(workDirectory, 'javascript-probe.json');
  await writeFile(path.join(directory, 'issue-195-meta-language-probe.mjs'), JAVASCRIPT_PROBE);
  const probeRun = await run('javascript-probe', process.execPath, ['issue-195-meta-language-probe.mjs', probePath], {
    cwd: directory,
    env: environment,
    allowFailure: true,
  });
  const probe = existsSync(probePath) ? JSON.parse(await readFile(probePath, 'utf8')) : {};
  return {
    skipped: false,
    version,
    checksum,
    resolution,
    workloadFiles: files,
    tests,
    counts: countOutcomes(tests),
    exit: testRun.code,
    probe: probeAssertions(probe, probeRun),
  };
}

async function rustWorkloads(rmlDirectory) {
  const directory = path.join(rmlDirectory, 'rust');
  const extracted = path.join(workDirectory, 'crate-source');
  const target = path.join(workDirectory, 'rml-target');
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
  // outside RML's requirement is pinned exactly, and the report says so.
  const manifestPath = path.join(directory, 'Cargo.toml');
  const manifest = await readFile(manifestPath, 'utf8');
  const declaredRequirement = /^meta-language = "([^"]+)"$/mu.exec(manifest)?.[1] ?? null;
  const declaredRequirementSatisfied = Boolean(declaredRequirement) && satisfies(version, declaredRequirement, 'cargo');
  const effectiveRequirement = declaredRequirementSatisfied ? declaredRequirement : `=${version}`;
  if (!declaredRequirementSatisfied) {
    await writeFile(manifestPath, manifest.replace(/^meta-language = "[^"]+"$/mu, `meta-language = "${effectiveRequirement}"`));
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
    ISSUE_195_RML_PROBE_DIRECTORY: probeDirectory,
  };

  const metadata = JSON.parse((await run('rust-metadata', 'cargo', [
    ...configuration, 'metadata', '--format-version', '1',
  ], { cwd: directory, env: environment })).stdout);
  const metaLanguagePackages = metadata.packages.filter(({ name }) => name === 'meta-language');
  const metaLanguage = metaLanguagePackages[0] ?? {};
  const files = workloadTestFiles(fixture, 'rust');
  const targets = files.map((file) => path.posix.basename(file, '.rs'));
  const testRun = await run('rust-workloads', 'cargo', [
    ...configuration, 'test', '--no-fail-fast',
    ...[...targets, RUST_PROBE_TARGET].flatMap((name) => ['--test', name]),
  ], { cwd: directory, env: environment, allowFailure: true });
  const executed = libtestOutcomes(testRun.combined);
  const lock = await readFile(path.join(directory, 'Cargo.lock'), 'utf8');
  const lockEntry = /\[\[package\]\]\nname = "meta-language"\nversion = "([^"]+)"\n(?:source = "([^"]+)"\n)?/u.exec(lock);
  const resolution = {
    declaredRequirement,
    declaredRequirementSatisfied,
    effectiveRequirement,
    patch: configuration.join(' '),
    unpackedDirectory: `${unpackedDirectory}/`,
    manifestPath: metaLanguage.manifest_path?.split(path.sep).join('/') ?? null,
    metadataSource: metaLanguage.source ?? null,
    metaLanguagePackages: metaLanguagePackages.length,
    lockVersion: lockEntry?.[1] ?? null,
    lockSource: lockEntry ? lockEntry[2] ?? null : 'no meta-language entry',
    installedVersion: metaLanguage.version ?? null,
  };

  const tests = executed.filter(({ file }) => file !== `rust/tests/${RUST_PROBE_TARGET}.rs`);
  const probeTests = executed.filter(({ file }) => file === `rust/tests/${RUST_PROBE_TARGET}.rs`);
  const probe = {};
  for (const assertion of RML_PROBE_ASSERTIONS) {
    const name = RUST_PROBE_TESTS[assertion];
    const outcome = probeTests.find((entry) => entry.name === name)?.outcome ?? 'not run';
    const evidencePath = path.join(probeDirectory, `${assertion}.json`);
    const evidence = outcome === 'passed' && existsSync(evidencePath)
      ? JSON.parse(await readFile(evidencePath, 'utf8'))
      : { checks: [{ name: `probe test ${name}`, holds: false, detail: `the probe test ${outcome}; see ${testRun.log}` }] };
    probe[assertion] = { test: name, outcome, checks: evidence.checks };
  }
  probe.sharedConceptsReused.checks.push(check('the RML build links one meta-language, the unpacked crate', {
    holds: metaLanguagePackages.length === 1 && resolution.metadataSource === null &&
      Boolean(resolution.manifestPath?.startsWith(resolution.unpackedDirectory)),
    detail: { packages: metaLanguagePackages.length, manifestPath: resolution.manifestPath },
  }));
  probe.foundationAuthorityStaysInRml.checks.push(
    dependencyDirection(metadata, metaLanguage.id),
    await crateNamesNoRmlItem(crateSource),
    await onlyBridgesNameMetaLanguage(rmlDirectory),
  );
  for (const assertion of RML_PROBE_ASSERTIONS) {
    probe[assertion].holds = probe[assertion].checks.every(({ holds }) => holds === true);
  }
  return {
    skipped: false,
    version,
    checksum,
    resolution,
    workloadFiles: files,
    tests,
    counts: countOutcomes(tests),
    exit: testRun.code,
    probeTests,
    probe,
  };
}

function check(name, { holds, detail }) {
  return { name, holds, detail };
}

// The JavaScript probe's observations, or a failed check carrying why it wrote none.
function probeAssertions(probe, probeRun) {
  return Object.fromEntries(RML_PROBE_ASSERTIONS.map((assertion) => [
    assertion,
    probe[assertion] ?? {
      holds: false,
      checks: [{ name: 'probe run', holds: false, detail: `the probe exited with ${probeRun.code}; see ${probeRun.log}` }],
    },
  ]));
}

// meta-language must not depend, even transitively, on an RML crate: the dependency points from RML to the package.
function dependencyDirection(metadata, metaLanguageId) {
  const nodes = new Map((metadata.resolve?.nodes ?? []).map((node) => [node.id, node]));
  const names = new Map(metadata.packages.map(({ id, name }) => [id, name]));
  const reached = new Set([metaLanguageId]);
  for (let grown = true; grown;) {
    grown = false;
    for (const id of [...reached]) {
      for (const dependency of nodes.get(id)?.dependencies ?? []) {
        if (!reached.has(dependency)) {
          reached.add(dependency);
          grown = true;
        }
      }
    }
  }
  const rmlCrates = [...reached].map((id) => names.get(id)).filter((name) => name === 'relative-meta-logic');
  const rmlDependsOnPackage = (metadata.packages.find(({ name }) => name === 'relative-meta-logic')?.dependencies ?? [])
    .some(({ name }) => name === 'meta-language');
  return check('meta-language depends on no RML crate, RML depends on meta-language', {
    holds: Boolean(metaLanguageId) && nodes.has(metaLanguageId) && rmlCrates.length === 0 && rmlDependsOnPackage,
    detail: { metaLanguageDependencies: reached.size - 1, rmlCrates },
  });
}

// An RML item is reached through a Rust path or an extern crate; the upstream name in provenance strings
// and comments (the parity corpus credits relative-meta-logic examples) names no item.
async function crateNamesNoRmlItem(crateSource) {
  const files = (await rustSources(path.join(crateSource, 'src')))
    .filter(({ text }) => /\b(?:rml|relative_meta_logic)\s*::|\bextern\s+crate\s+(?:rml|relative_meta_logic)\b/u.test(text))
    .map(({ file }) => path.relative(crateSource, file).split(path.sep).join('/'));
  return check('the unpacked crate names no RML item', { holds: files.length === 0, detail: { files } });
}

// RML's evaluator and checker live in rust/src/lib.rs; only the inventoried bridge sources may name meta_language.
async function onlyBridgesNameMetaLanguage(rmlDirectory) {
  const bridges = new Set(fixture.workloads
    .filter(({ file, kind }) => kind === 'source' && file.startsWith('rust/src/')).map(({ file }) => file));
  const naming = (await rustSources(path.join(rmlDirectory, 'rust', 'src')))
    .filter(({ text }) => /\bmeta_language::/u.test(text))
    .map(({ file }) => path.relative(rmlDirectory, file).split(path.sep).join('/'));
  const outside = naming.filter((file) => !bridges.has(file));
  return check('only the inventoried RML bridge sources name meta_language', {
    holds: naming.length > 0 && outside.length === 0 && !naming.includes('rust/src/lib.rs'),
    detail: { naming, outside },
  });
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

async function rustSources(directory) {
  const files = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await rustSources(file));
    else if (entry.name.endsWith('.rs')) files.push({ file, text: await readFile(file, 'utf8') });
  }
  return files;
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
