import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { arch, platform, release } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../js/scripts/project-data.mjs';
import { parseGrammarLinks } from '../js/src/grammar-links.js';
import { readStructuredDataLino } from '../js/src/structured-data-links.js';
import { parseCargoLock, parseCargoManifest } from '../js/scripts/dependency-inventory.mjs';
import { linkCliArchiveReconstructionFixture, linkCliReconstruction,
  selectLinkCliArchiveReconstructionAssertion } from '../js/scripts/link-cli-archive-reconstruction-assertions.mjs';
import { recordIssue195Observations } from '../js/tests/support/issue-195-observations.js';

const root = fileURLToPath(new URL('..', import.meta.url));
const [work, report] = process.argv.slice(2).map(file => path.resolve(file));
assert.ok(work && report, 'pass a CI work directory and a report file');
const fixturePath = linkCliReconstruction.fixtureFile;
const fixture = readProjectData(path.join(root, fixturePath));
const grammarSources = new Map(fixture.files.map(file => [file.path, file.text]));
const manifest = path.join(root, 'rust/Cargo.toml');
const { linkCliVersion, notationVersion } = linkCliReconstruction;
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const observations = [];

function invoke(command, args, { cwd = root, status = 0, inherit = false } = {}) {
  const result = spawnSync(command, args, {
    cwd, maxBuffer: 128 * 1024 * 1024,
    stdio: inherit ? ['ignore', 'inherit', 'inherit'] : ['ignore', 'pipe', 'pipe'],
  });
  if (result.error) throw result.error;
  assert.equal(result.status, status,
    [command, ...args].join(' ') + '\n' + String(result.stderr ?? result.signal ?? ''));
  return result;
}

mkdirSync(work, { recursive: true });
const installation = mkdtempSync(path.join(work, 'clink-'));
const [nodeMajor, nodeMinor] = process.versions.node.split('.').map(Number);
assert.ok(nodeMajor === 22 && nodeMinor >= 15 || nodeMajor === 24,
  `uncovered Node ${process.versions.node}; ${fixture.versions.JavaScript}`);
const rustc = String(invoke('rustc', ['--version']).stdout).trim();
assert.match(rustc, /^rustc 1\.9[0-9]\.\d+(?:\s|$)/u, `uncovered ${rustc}; ${fixture.versions.Rust}`);
const rustDependencies = parseCargoManifest(readFileSync(manifest, 'utf8'));
const cargoLock = parseCargoLock(readFileSync(path.join(root, 'rust/Cargo.lock'), 'utf8'));
assert.ok(rustDependencies.dependencies.some(dependency => dependency.name === 'links-notation'
  && dependency.requirement === '=' + notationVersion), 'the native Links Notation version pin differs');
assert.ok(cargoLock.some(entry => entry.name === rustDependencies.settings.name
  && entry.dependencies.includes('links-notation ' + notationVersion)), 'the locked native Links Notation version differs');
invoke('cargo', ['install', 'link-cli', '--locked', '--version', '=' + linkCliVersion,
  '--bin', 'clink', '--root', installation], { inherit: true });
const clink = path.join(installation, 'bin', 'clink');
const versionOutput = String(invoke(clink, ['--version']).stdout);
const actualLinkCliVersion = versionOutput.match(/\b\d+\.\d+\.\d+(?:[-+][^\s]+)?\b/u)?.[0];
assert.ok(actualLinkCliVersion, 'unrecognized Link CLI version output ' + versionOutput);
const versionApplicability = [selectLinkCliArchiveReconstructionAssertion({ linkCliVersion: actualLinkCliVersion })];
invoke('cargo', ['build', '--locked', '--manifest-path', manifest, '--bin', 'meta-language'], { inherit: true });
const metadata = JSON.parse(invoke('cargo', [
  'metadata', '--no-deps', '--locked', '--format-version', '1', '--manifest-path', manifest,
]).stdout);
const native = path.join(metadata.target_directory, 'debug', 'meta-language');
const commands = [
  { runtime: 'javascript', command: process.execPath, prefix: [path.join(root, 'js/src/cli.js')] },
  { runtime: 'rust', command: native, prefix: [] },
];

function loadThroughStore(directory, bytes, step) {
  const stage = mkdtempSync(path.join(directory, 'store-'));
  try {
    const source = path.join(stage, 'input.store');
    const database = path.join(stage, 'database');
    const output = path.join(stage, 'output.store');
    writeFileSync(source, bytes);
    invoke(clink, ['--db', database, '--import-binary', source], { cwd: stage });
    rmSync(source);
    invoke(clink, ['--db', database, '--export-binary', output], { cwd: stage });
    const reconstructed = readFileSync(output);
    assert.ok(reconstructed.length > 0, step);
    return reconstructed;
  } finally {
    rmSync(stage, { recursive: true });
  }
}

function convert(runtime, directory, filename, format, fixtureStep) {
  const args = ['archive', 'convert', filename, '--binary'];
  if (format === 'binary') args.push('--input-binary');
  if (format === 'link-cli') args.push('--input-link-cli-store');
  const measure = path.join(directory, 'reconstruction-time.txt');
  const before = process.hrtime.bigint();
  const { stdout } = invoke('/usr/bin/time', [
    '-f', '%M', '-o', measure, runtime.command, ...runtime.prefix, ...args,
  ], { cwd: directory });
  const reconstructionMilliseconds = Number(process.hrtime.bigint() - before) / 1e6;
  const peakResidentKibibytes = Number(readFileSync(measure, 'utf8').trim());
  assert.ok(Number.isSafeInteger(peakResidentKibibytes) && peakResidentKibibytes > 0);
  const input = readFileSync(path.join(directory, filename));
  const measurement = {
    runtime: runtime.runtime, fixtureStep, format,
    inputBytes: input.length, inputSha256: hash(input), outputBytes: stdout.length,
    outputSha256: hash(stdout), reconstructionMilliseconds, peakResidentKibibytes,
  };
  return { stdout, measurement };
}

function replay(runtime) {
  const ordinary = mkdtempSync(path.join(work, runtime.runtime + '-ordinary-'));
  const stored = mkdtempSync(path.join(work, runtime.runtime + '-store-'));
  const dirs = [ordinary, stored];
  try {
    for (const dir of dirs) for (const file of fixture.files) {
      writeFileSync(path.join(dir, file.path), file.text);
    }
    const checkpoints = new Set();
    for (const step of fixture.steps) {
      const ordinaryArgs = step.args;
      const fromArchive = checkpoints.has(ordinaryArgs[1]);
      const expectsArchive = step.archive || (step.exit === 0 && ['discover', 'refine'].includes(ordinaryArgs[0]));
      const mayRecoverArchive = step.exit === 1 && ordinaryArgs[0] === 'parse';
      const storeArgs = ordinaryArgs.filter(option =>
        option !== '--binary' && option !== '--input-binary');
      if (fromArchive) storeArgs.push('--input-link-cli-store');
      if (expectsArchive || mayRecoverArchive) storeArgs.push('--link-cli-store');
      console.log(runtime.runtime + ' ' + step.name);
      const baseline = invoke(runtime.command, [...runtime.prefix, 'language', ...ordinaryArgs],
        { cwd: ordinary, status: step.exit });
      const selected = invoke(runtime.command, [...runtime.prefix, 'language', ...storeArgs],
        { cwd: stored, status: step.exit });
      assert.ok(selected.stderr.equals(baseline.stderr), step.name + ' changed diagnostics');
      if (step.error) assert.ok(String(selected.stderr).includes(step.error), step.name);
      if (step.rejection) assert.ok(readStructuredDataLino(String(selected.stderr)).rejection, step.name);
      if (step.inventory) {
        assert.deepEqual(readStructuredDataLino(String(selected.stdout)).map(entry => entry.identifier), step.inventory);
      }
      if (step.grammar) {
        assert.deepEqual(parseGrammarLinks(String(selected.stdout)).normalized(),
          parseGrammarLinks(grammarSources.get(step.grammar)).normalized());
      }
      if (expectsArchive || mayRecoverArchive && baseline.stdout.length > 0) {
        assert.ok(baseline.stdout.length > 0 && selected.stdout.length > 0, step.name + ' requires a complete archive');
        const chosen = step.save ?? 'unsaved.archive';
        const intermediate = 'alternate-encoding.archive';
        const format = ordinaryArgs.includes('--binary') ? 'binary' : 'text';
        writeFileSync(path.join(ordinary, chosen), baseline.stdout);
        writeFileSync(path.join(stored, chosen), loadThroughStore(stored, selected.stdout, step.name));
        const canonical = convert(runtime, ordinary, chosen, format, step.name);
        const restored = convert(runtime, stored, chosen, 'link-cli', step.name);
        const outputEncoding = format === 'binary' ? [] : ['--binary'];
        const inputEncoding = format === 'binary' ? ['--input-binary'] : [];
        const alternate = invoke(runtime.command,
          [...runtime.prefix, 'archive', 'convert', chosen, ...inputEncoding, ...outputEncoding],
          { cwd: ordinary });
        writeFileSync(path.join(ordinary, intermediate), alternate.stdout);
        const roundTrip = convert(runtime, ordinary, intermediate,
          format === 'binary' ? 'text' : 'binary', step.name);
        assert.ok(canonical.stdout.equals(restored.stdout), step.name + ' changed the complete archive after link-cli');
        assert.ok(canonical.stdout.equals(roundTrip.stdout), step.name + ' changed the complete archive after conversion');
        observations.push(canonical.measurement, restored.measurement, roundTrip.measurement);
        rmSync(path.join(ordinary, intermediate));
        if (step.save) checkpoints.add(chosen);
      } else {
        assert.ok(selected.stdout.equals(baseline.stdout), step.name + ' changed observable output');
      }
      const obsolete = step.remove ?? [];
      for (const dir of dirs) for (const file of obsolete) rmSync(path.join(dir, file));
      for (const file of obsolete) checkpoints.delete(file);
      if (step.args[0] === 'prepare') {
        const inputs = step.args.flatMap((value, index) =>
          value === '--source' ? [step.args[index + 4]] : []);
        for (const dir of dirs) for (const input of inputs) rmSync(path.join(dir, input));
      }
    }
  } finally {
    for (const dir of dirs) rmSync(dir, { recursive: true });
  }
}

try {
  for (const { runtime, ...command } of commands) {
    replay({ runtime, ...command });
    recordIssue195Observations({
      requirementId: linkCliReconstruction.requirementId, runtime,
      suffix: linkCliReconstruction.suffix, fixtureId: linkCliArchiveReconstructionFixture(runtime),
      fixtureFile: fixturePath, assertions: linkCliReconstruction.assertions, versionApplicability,
      testName: 'complete dynamic language archive reconstruction using independent Link CLI import and export processes',
    });
  }
} finally {
  writeProjectData(report, {
    fixturePath, fixtureSha256: hash(readFileSync(path.join(root, fixturePath))),
    commit: String(invoke('git', ['rev-parse', 'HEAD']).stdout).trim(),
    toolchain: {
      node: process.version, rustc, linksNotation: notationVersion,
      linkCliVersion: actualLinkCliVersion, platform: platform(), release: release(), architecture: arch(),
    },
    algorithm: 'eager public archive conversion through the complete record graph',
    observations,
  });
  rmSync(installation, { recursive: true });
}
