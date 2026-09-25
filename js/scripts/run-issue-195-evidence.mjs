#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { spawn, execFileSync } from 'node:child_process';
import {
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

import { buildIssue195Manifest } from './issue-195-acceptance-lib.mjs';
import { buildEvidencePlan, observedEvidenceForCell } from './issue-195-evidence-plan.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const commit = option('--commit', process.env.GITHUB_SHA ?? git(['rev-parse', 'HEAD']));
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
const corpus = JSON.parse(
  await readFile(path.join(root, 'parity/fixtures/issue-195-evidence.json'), 'utf8'),
);

await prepareDirectories();
process.env.ISSUE_195_OBSERVATION_FILE = observationsPath;
process.env.ISSUE_195_COMMIT = commit;
const toolchainVersions = readToolchainVersions();
const grammarVersions = readGrammarVersions();

console.log(
  `issue-195 evidence: executing ${plan.length} ${checkpoint} evidence groups for ${commit}`,
);
const evidenceGroups = checkpoint === 'release-delivery'
  ? await producePublishedEvidence()
  : await producePreMergeEvidence();
const executionRecords = await readExecutionRecords();

let written = 0;
let complete = 0;
for (const { group, cells } of plan) {
  const record = evidenceGroups.get(group);
  if (!record) throw new Error(`evidence group completed without a record: ${group}`);
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
            toolchainVersions,
            grammarVersions,
            evidenceArtifacts: [relative(output), ...record.artifacts],
            failureLogs: [],
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
console.log(`issue-195 evidence: ${complete}/${written} cells have observed assertion callbacks`);

async function readExecutionRecords() {
  let contents;
  try {
    contents = await readFile(observationsPath, 'utf8');
  } catch (error) {
    if (error.code === 'ENOENT') return [];
    throw error;
  }
  const records = contents.split(/\r?\n/u).filter(Boolean).map((line, index) => {
    try {
      return JSON.parse(line);
    } catch (error) {
      throw new Error(`invalid execution record at line ${index + 1}: ${error.message}`);
    }
  });
  const known = new Set(plan.flatMap(({ cells }) => cells.map(({ cell }) => cell.testId)));
  const seen = new Set();
  for (const record of records) {
    if (!record || typeof record !== 'object') {
      throw new Error('execution record is not an object');
    }
    if (!known.has(record.testId)) {
      throw new Error(`execution record has unknown testId ${record.testId}`);
    }
    const key = `${record.testId}\u0000${record.assertionId}\u0000${record.fixtureId}`;
    if (seen.has(key)) throw new Error(`duplicate execution record ${key}`);
    seen.add(key);
  }
  return records;
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

async function prepareDirectories() {
  await mkdir(resultsDirectory, { recursive: true });
  for (const name of await readdir(resultsDirectory)) {
    if (name.endsWith('.json')) await rm(path.join(resultsDirectory, name));
  }
  await rm(workDirectory, { recursive: true, force: true });
  await rm(logsDirectory, { recursive: true, force: true });
  await rm(artifactsDirectory, { recursive: true, force: true });
  await Promise.all([
    mkdir(workDirectory, { recursive: true }),
    mkdir(logsDirectory, { recursive: true }),
    mkdir(artifactsDirectory, { recursive: true }),
    mkdir(parityDirectory, { recursive: true }),
  ]);
}

function readToolchainVersions() {
  const versions = {
    node: commandOutput('node', ['--version']),
    npm: commandOutput('npm', ['--version']),
    rustc: commandOutput('rustc', ['--version']),
    cargo: commandOutput('cargo', ['--version']),
    lean: commandOutput('lean', ['--version']),
    rocq: commandOutput('rocq', ['--version']),
    platform: `${process.platform}-${process.arch}`,
  };
  const expected = [
    ['rustc', versions.rustc, '1.98.1'],
    ['lean', versions.lean, '4.33.1'],
    ['rocq', versions.rocq, '9.2'],
  ];
  for (const [tool, actual, wanted] of expected) {
    if (!actual.includes(wanted)) {
      throw new Error(`${tool} version does not match declared toolchain ${wanted}: ${actual}`);
    }
  }
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

async function producePreMergeEvidence() {
  const [javascriptSuite, rustSuite] = await Promise.all([
    runCommand('suite-javascript', 'npm', ['test'], { cwd: path.join(root, 'js') }),
    runCommand(
      'suite-rust',
      'cargo',
      ['test', '--manifest-path', path.join(root, 'rust/Cargo.toml'), '--all-features'],
    ),
  ]);
  const runtimeParity = await runCommand(
    'runtime-parity',
    'node',
    [
      path.join(root, 'js/scripts/check-issue-195-runtime-parity.mjs'),
      '--artifacts-dir',
      parityDirectory,
    ],
  );
  const nativeGroups = await validateNativeTranslations();
  const [npmCandidate, crateCandidate] = await Promise.all([
    buildNpmCandidate(),
    buildCrateCandidate(),
  ]);
  const rmlCandidate = await validateDownstreamRml(npmCandidate, crateCandidate);
  const groups = new Map([
    ['suite:javascript', bundle(javascriptSuite)],
    ['suite:rust', bundle(rustSuite)],
    ['runtime-parity', bundle(runtimeParity)],
    ['delivery:npm', bundle(npmCandidate, rmlCandidate.javascript)],
    ['delivery:crate', bundle(crateCandidate, rmlCandidate.rust)],
    [
      'delivery:rml',
      bundle(npmCandidate, crateCandidate, rmlCandidate.javascript, rmlCandidate.rust),
    ],
  ]);
  for (const [group, record] of nativeGroups) groups.set(group, record);
  for (const runtime of ['javascript', 'rust']) {
    for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
      groups.set(
        `translation:${target}:${runtime}`,
        bundle(
          runtime === 'javascript' ? javascriptSuite : rustSuite,
          runtimeParity,
          nativeGroups.get(`native:${target}:${runtime}`),
        ),
      );
    }
  }
  return groups;
}

async function producePublishedEvidence() {
  const [npmPublished, cratePublished] = await Promise.all([
    buildPublishedNpmPackage(),
    buildPublishedCratePackage(),
  ]);
  const rmlPublished = await validateDownstreamRml(npmPublished, cratePublished, 'published');
  return new Map([
    ['delivery:npm', bundle(npmPublished, rmlPublished.javascript)],
    ['delivery:crate', bundle(cratePublished, rmlPublished.rust)],
    [
      'delivery:rml',
      bundle(npmPublished, cratePublished, rmlPublished.javascript, rmlPublished.rust),
    ],
  ]);
}

async function validateNativeTranslations() {
  const observations = {
    javascript: JSON.parse(await readFile(path.join(parityDirectory, 'javascript.json'), 'utf8')),
    rust: JSON.parse(await readFile(path.join(parityDirectory, 'rust.json'), 'utf8')),
  };
  const groups = new Map();
  for (const [runtime, observation] of Object.entries(observations)) {
    for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
      const directory = path.join(workDirectory, 'native', runtime, target.toLowerCase());
      await mkdir(directory, { recursive: true });
      const records = [];
      const translations = observation.translations.filter(
        ({ targetLanguage }) => targetLanguage === target,
      );
      if (translations.length !== 3) {
        throw new Error(`${runtime} emitted ${translations.length} ${target} translations, expected 3`);
      }
      for (const translation of translations) {
        const extension = corpus.nativeValidation[target].extension;
        const sourceName = translation.sourceLanguage.toLowerCase();
        const artifact = path.join(directory, `${sourceName}_to_${target.toLowerCase()}${extension}`);
        await writeFile(artifact, translation.code);
        records.push(await validateNativeArtifact(target, artifact, runtime, sourceName));
      }
      groups.set(`native:${target}:${runtime}`, bundle(...records));
    }
  }
  return groups;
}

function validateNativeArtifact(language, artifact, runtime, sourceName) {
  const label = `native-${runtime}-${sourceName}-to-${language.toLowerCase()}`;
  if (language === 'JavaScript') return runCommand(label, 'node', ['--check', artifact]);
  if (language === 'Rust') {
    const out = path.join(path.dirname(artifact), `${sourceName}.rmeta`);
    return runCommand(label, 'rustc', [
      '--crate-type', 'lib', '--edition', '2024', '--emit', 'metadata', '-o', out, artifact,
    ]);
  }
  if (language === 'Lean') return runCommand(label, 'lean', [artifact]);
  return runCommand(label, 'rocq', ['compile', '-q', artifact]);
}

async function buildNpmCandidate() {
  const directory = path.join(workDirectory, 'npm-candidate');
  const packageDirectory = path.join(directory, 'package');
  const consumerDirectory = path.join(directory, 'consumer');
  await Promise.all([
    mkdir(packageDirectory, { recursive: true }),
    mkdir(consumerDirectory, { recursive: true }),
  ]);
  const pack = await runCommand('delivery-npm-pack', 'npm', [
    'pack', '--json', '--pack-destination', packageDirectory,
  ], { cwd: path.join(root, 'js') });
  const tarballName = (await readdir(packageDirectory)).find((name) => name.endsWith('.tgz'));
  if (!tarballName) throw new Error('npm pack did not produce a tarball');
  const tarball = path.join(packageDirectory, tarballName);
  const stableTarball = path.join(artifactsDirectory, tarballName);
  await copyFile(tarball, stableTarball);
  await writeFile(path.join(consumerDirectory, 'package.json'), stableJson({
    private: true,
    type: 'module',
    dependencies: { 'meta-language': `file:${stableTarball}` },
  }));
  await writeFile(
    path.join(consumerDirectory, 'smoke.mjs'),
    "import { LinkNetwork } from 'meta-language';\nfor (const [language, source, requiredTerm] of [['JavaScript', 'const α = 1;\\n', 'lexical_declaration'], ['Rocq', 'Definition identity (x : nat) := x.\\n', 'definition_command']]) {\n  const network = LinkNetwork.parse(source, language);\n  if (!network.verifyFullMatch().isClean() || network.reconstructText() !== source || !network.links().some((link) => link.metadata().term === requiredTerm)) process.exit(1);\n}\n",
  );
  const install = await runCommand('delivery-npm-install', 'npm', [
    'install', '--ignore-scripts', '--no-audit', '--no-fund',
  ], { cwd: consumerDirectory });
  const smoke = await runCommand('delivery-npm-offline-smoke', 'node', ['smoke.mjs'], {
    cwd: consumerDirectory,
    env: { ...process.env, NO_PROXY: '*', no_proxy: '*' },
  });
  const checksumPath = path.join(artifactsDirectory, `${tarballName}.sha256`);
  await writeFile(checksumPath, `${await sha256File(stableTarball)}  ${tarballName}\n`);
  const record = bundle(pack, install, smoke);
  record.artifacts.push(relative(stableTarball), relative(checksumPath));
  return { ...record, tarball: stableTarball, checksumPath };
}

async function buildCrateCandidate() {
  const directory = path.join(workDirectory, 'crate-candidate');
  const packageDirectory = path.join(directory, 'package');
  const consumerDirectory = path.join(directory, 'consumer');
  await Promise.all([
    mkdir(packageDirectory, { recursive: true }),
    mkdir(path.join(consumerDirectory, 'src'), { recursive: true }),
  ]);
  const pack = await runCommand('delivery-crate-pack', 'cargo', [
    'package', '--manifest-path', path.join(root, 'rust/Cargo.toml'),
    '--allow-dirty', '--target-dir', packageDirectory,
  ]);
  const packageOutput = path.join(packageDirectory, 'package');
  const crateName = (await readdir(packageOutput)).find((name) => name.endsWith('.crate'));
  if (!crateName) throw new Error('cargo package did not produce a crate archive');
  const crate = path.join(packageOutput, crateName);
  const stableCrate = path.join(artifactsDirectory, crateName);
  await copyFile(crate, stableCrate);
  const extracted = path.join(directory, 'extracted');
  await mkdir(extracted, { recursive: true });
  const unpack = await runCommand('delivery-crate-unpack', 'tar', ['-xzf', stableCrate, '-C', extracted]);
  const crateSource = path.join(extracted, crateName.slice(0, -'.crate'.length));
  await writeFile(path.join(consumerDirectory, 'Cargo.toml'), `[package]\nname = "issue-195-consumer"\nversion = "0.0.0"\nedition = "2024"\n\n[dependencies]\nmeta-language = { path = ${JSON.stringify(crateSource)}, default-features = false }\n`);
  await writeFile(
    path.join(consumerDirectory, 'src/main.rs'),
    'use meta_language::{LinkNetwork, ParseConfiguration};\nfn main() { let source = "const α = 1;\\n"; let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default()); assert!(network.verify_full_match(None).is_clean()); assert_eq!(network.reconstruct_text(), source); }\n',
  );
  const fetch = await runCommand('delivery-crate-fetch', 'cargo', ['fetch'], { cwd: consumerDirectory });
  const smoke = await runCommand('delivery-crate-offline-smoke', 'cargo', ['run', '--offline'], {
    cwd: consumerDirectory,
  });
  const checksumPath = path.join(artifactsDirectory, `${crateName}.sha256`);
  await writeFile(checksumPath, `${await sha256File(stableCrate)}  ${crateName}\n`);
  const record = bundle(pack, unpack, fetch, smoke);
  record.artifacts.push(relative(stableCrate), relative(checksumPath));
  return {
    ...record,
    crate: stableCrate,
    crateSource,
    checksumPath,
  };
}

async function buildPublishedNpmPackage() {
  const directory = path.join(workDirectory, 'npm-published');
  const packageDirectory = path.join(directory, 'package');
  const consumerDirectory = path.join(directory, 'consumer');
  await Promise.all([
    mkdir(packageDirectory, { recursive: true }),
    mkdir(consumerDirectory, { recursive: true }),
  ]);
  const pack = await runCommandWithRetry('delivery-npm-published-pack', 'npm', [
    'pack', `meta-language@${packageMetadata.version}`, '--json',
    '--pack-destination', packageDirectory,
  ]);
  const tarballName = (await readdir(packageDirectory)).find((name) => name.endsWith('.tgz'));
  if (!tarballName) throw new Error('npm registry pack did not produce a tarball');
  const tarball = path.join(packageDirectory, tarballName);
  const stableTarball = path.join(artifactsDirectory, tarballName);
  await copyFile(tarball, stableTarball);
  await writeFile(path.join(consumerDirectory, 'package.json'), stableJson({
    private: true,
    type: 'module',
    dependencies: { 'meta-language': `file:${stableTarball}` },
  }));
  await writeFile(
    path.join(consumerDirectory, 'smoke.mjs'),
    "import { LinkNetwork } from 'meta-language';\nfor (const [language, source, requiredTerm] of [['JavaScript', 'const α = 1;\\n', 'lexical_declaration'], ['Rocq', 'Definition identity (x : nat) := x.\\n', 'definition_command']]) {\n  const network = LinkNetwork.parse(source, language);\n  if (!network.verifyFullMatch().isClean() || network.reconstructText() !== source || !network.links().some((link) => link.metadata().term === requiredTerm)) process.exit(1);\n}\n",
  );
  const install = await runCommand('delivery-npm-published-install', 'npm', [
    'install', '--ignore-scripts', '--no-audit', '--no-fund',
  ], { cwd: consumerDirectory });
  const smoke = await runCommand(
    'delivery-npm-published-offline-smoke',
    'node',
    ['smoke.mjs'],
    { cwd: consumerDirectory, env: { ...process.env, NO_PROXY: '*', no_proxy: '*' } },
  );
  const checksumPath = path.join(artifactsDirectory, `${tarballName}.sha256`);
  await writeFile(checksumPath, `${await sha256File(stableTarball)}  ${tarballName}\n`);
  const record = bundle(pack, install, smoke);
  record.artifacts.push(relative(stableTarball), relative(checksumPath));
  return { ...record, tarball: stableTarball, checksumPath };
}

async function buildPublishedCratePackage() {
  const directory = path.join(workDirectory, 'crate-published');
  const consumerDirectory = path.join(directory, 'consumer');
  const extracted = path.join(directory, 'extracted');
  await Promise.all([
    mkdir(path.join(consumerDirectory, 'src'), { recursive: true }),
    mkdir(extracted, { recursive: true }),
  ]);
  const crateName = `meta-language-${packageMetadata.version}.crate`;
  const stableCrate = path.join(artifactsDirectory, crateName);
  const download = await runCommandWithRetry('delivery-crate-published-download', 'curl', [
    '--fail', '--location', '--silent', '--show-error',
    '--output', stableCrate,
    `https://crates.io/api/v1/crates/meta-language/${packageMetadata.version}/download`,
  ]);
  const unpack = await runCommand(
    'delivery-crate-published-unpack',
    'tar',
    ['-xzf', stableCrate, '-C', extracted],
  );
  const crateSource = path.join(extracted, crateName.slice(0, -'.crate'.length));
  await writeFile(path.join(consumerDirectory, 'Cargo.toml'), `[package]\nname = "issue-195-published-consumer"\nversion = "0.0.0"\nedition = "2024"\n\n[dependencies]\nmeta-language = { path = ${JSON.stringify(crateSource)}, default-features = false }\n`);
  await writeFile(
    path.join(consumerDirectory, 'src/main.rs'),
    'use meta_language::{LinkNetwork, ParseConfiguration};\nfn main() { let source = "const α = 1;\\n"; let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default()); assert!(network.verify_full_match(None).is_clean()); assert_eq!(network.reconstruct_text(), source); }\n',
  );
  const fetch = await runCommand('delivery-crate-published-fetch', 'cargo', ['fetch'], {
    cwd: consumerDirectory,
  });
  const smoke = await runCommand(
    'delivery-crate-published-offline-smoke',
    'cargo',
    ['run', '--offline'],
    { cwd: consumerDirectory },
  );
  const checksumPath = path.join(artifactsDirectory, `${crateName}.sha256`);
  await writeFile(checksumPath, `${await sha256File(stableCrate)}  ${crateName}\n`);
  const record = bundle(download, unpack, fetch, smoke);
  record.artifacts.push(relative(stableCrate), relative(checksumPath));
  return { ...record, crate: stableCrate, crateSource, checksumPath };
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
    env: { ...process.env, NO_PROXY: '*', no_proxy: '*' },
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
    'test', '--offline', '--test', 'meta_language_support_tests',
  ], { cwd: rustDirectory });
  return {
    javascript: bundle(clone, checkout, javascriptInstall, javascriptTest),
    rust: bundle(clone, checkout, rustFetch, rustTest),
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

async function runCommand(label, command, args, { cwd = root, env = process.env } = {}) {
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
  if (code !== 0) throw new Error(`${label} failed with exit ${code}; see ${relative(logPath)}`);
  return { commands: [rendered], artifacts: [relative(logPath)] };
}

function bundle(...records) {
  return {
    commands: [...new Set(records.flatMap((record) => record.commands))],
    artifacts: [...new Set(records.flatMap((record) => record.artifacts))],
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
