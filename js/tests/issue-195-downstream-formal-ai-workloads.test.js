// The link-assistant/formal-ai workloads run against the clean installed
// meta-language artifacts (I195-DOWNSTREAM-FORMAL-AI-WORKLOADS).
// js/scripts/run-formal-ai-workloads.mjs writes the report in CI; this suite
// checks the validator on synthetic reports and, when
// ISSUE_195_FORMAL_AI_WORKLOAD_REPORT names the CI report, validates its npm
// part and records the JavaScript observations. The Rust suite
// (rust/tests/unit/issue_195_downstream_formal_ai_workloads.rs) reads the crate part.
import assert from 'node:assert/strict';
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { tmpdir } from 'node:os';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';

import {
  FORMAL_AI_PROBE_ASSERTIONS,
  FORMAL_AI_WORKLOAD_ASSERTIONS,
  FORMAL_AI_WORKLOAD_FIXTURE,
  FORMAL_AI_WORKLOAD_FIXTURE_ID,
  FORMAL_AI_WORKLOAD_REPORT_VARIABLE,
  FORMAL_AI_WORKLOAD_REQUIREMENT,
  FORMAL_AI_WORKLOAD_SCHEMA_VERSION,
  compareSelfAstCensus,
  countOutcomes,
  loadFormalAiWorkloadFixture,
  validateFormalAiWorkloadReport,
  workloadFiles,
} from '../scripts/issue-195-formal-ai-workloads.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = loadFormalAiWorkloadFixture();
const reportPath = process.env[FORMAL_AI_WORKLOAD_REPORT_VARIABLE];

// A report on which every assertion of both runtimes holds.
function passingReport() {
  const digest = 'a'.repeat(64);
  const probe = Object.fromEntries(FORMAL_AI_PROBE_ASSERTIONS.map((assertion) => [
    assertion,
    { holds: true, checks: [{ name: `${assertion} check`, holds: true, detail: {} }] },
  ]));
  const part = (runtime, resolution, extra = {}) => {
    const tests = workloadFiles(fixture, runtime).map((file) => ({ file, name: 'a workload', outcome: 'passed' }));
    return {
      skipped: false,
      version: '1.2.3',
      checksum: { sha256: digest, expectedSha256: digest, ...extra },
      resolution: { installedVersion: '1.2.3', ...resolution },
      tests,
      counts: countOutcomes(tests),
      exit: 0,
      probe: structuredClone(probe),
    };
  };
  return {
    schemaVersion: FORMAL_AI_WORKLOAD_SCHEMA_VERSION,
    requirementId: FORMAL_AI_WORKLOAD_REQUIREMENT,
    formalAi: {
      repository: fixture.repository,
      revision: fixture.revision,
      checkedOutRevision: fixture.revision,
      verifiedBlobs: fixture.workloads.map(({ file, blob }) => ({ file, expected: blob, actual: blob, matches: true })),
    },
    npm: part('javascript', {
      lockResolved: 'file:../../meta-language-1.2.3.tgz',
      freshCache: true,
      consumerHadNoModules: true,
    }, { installedIntegrity: 'sha512-same', expectedIntegrity: 'sha512-same' }),
    crate: part('rust', {
      unpackedDirectory: '/work/crate-source/meta-language-1.2.3/',
      manifestPath: '/work/crate-source/meta-language-1.2.3/Cargo.toml',
      metadataSource: null,
      lockSource: null,
      metaLanguagePackages: 1,
    }),
  };
}

const failing = (result) => FORMAL_AI_WORKLOAD_ASSERTIONS.filter((assertion) => !result.observed.includes(assertion));

test('the fixture inventories the data files and the test files each runtime runs', () => {
  assert.equal(fixture.dependencies.rust, '0.58.2');
  assert.equal(fixture.dependencies.javascript, null, "formal-ai's npm package declares no meta-language dependency");
  assert.deepEqual(workloadFiles(fixture, 'javascript'), [
    'data/seed/grammar-projection-rules.lino',
    'data/seed/program-cst-grammars.lino',
    'data/seed/hello-world-programs.lino',
    'data/meta/link-edit-rules.lino',
    'data/benchmarks/coding-modification-suite.lino',
  ]);
  const rust = fixture.workloads.filter(({ kind, file }) => kind === 'test' && file.startsWith('rust/'));
  assert.deepEqual(rust.map(({ file }) => file), workloadFiles(fixture, 'rust'));
  assert.equal(rust.length, 11);
  for (const { file, target, filter } of rust) {
    assert.ok(['unit', 'source'].includes(target), `${file} runs in the unit or source target`);
    assert.match(filter, /::$/u, `${file} is filtered to its module`);
  }
});

test('a report on which everything holds observes every assertion of both runtimes', () => {
  for (const runtime of ['javascript', 'rust']) {
    const result = validateFormalAiWorkloadReport(passingReport(), fixture, runtime);
    assert.deepEqual(result.observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS], runtime);
  }
});

test('a failing workload test leaves the workloads unobserved', () => {
  const report = passingReport();
  report.npm.tests.push({
    file: 'data/seed/grammar-projection-rules.lino',
    name: 'loads as a translation rule set with every seed rule',
    outcome: 'failed',
  });
  report.npm.counts = countOutcomes(report.npm.tests);
  report.npm.exit = 1;
  const result = validateFormalAiWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(failing(result), ['workloadsRunOnInstalledArtifacts']);
  assert.ok(result.problems.workloadsRunOnInstalledArtifacts.some((problem) => /every seed rule failed/u.test(problem)));

  const unrun = passingReport();
  unrun.crate.tests = unrun.crate.tests.filter(({ file }) => file !== 'rust/tests/unit/issue_1085_link_edit_rules.rs');
  unrun.crate.counts = countOutcomes(unrun.crate.tests);
  const unrunResult = validateFormalAiWorkloadReport(unrun, fixture, 'rust');
  assert.deepEqual(failing(unrunResult), ['workloadsRunOnInstalledArtifacts'], 'a workload file without a passed test');
  assert.deepEqual(unrunResult.problems.workloadsRunOnInstalledArtifacts,
    ['no rust test of rust/tests/unit/issue_1085_link_edit_rules.rs passed']);

  const miscounted = passingReport();
  miscounted.npm.counts.passed += 1;
  assert.deepEqual(failing(validateFormalAiWorkloadReport(miscounted, fixture, 'javascript')), ['workloadsRunOnInstalledArtifacts'],
    'counts that differ from the executed tests');
});

test('a consumer that resolved meta-language elsewhere observes nothing', () => {
  const report = passingReport();
  report.npm.resolution.lockResolved = 'https://registry.npmjs.org/meta-language/-/meta-language-0.58.2.tgz';
  const result = validateFormalAiWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.sharedConceptsReused.some((problem) => /registry\.npmjs\.org/u.test(problem)));

  const crate = passingReport();
  crate.crate.resolution.metadataSource = 'registry+https://github.com/rust-lang/crates.io-index';
  crate.crate.resolution.lockSource = 'registry+https://github.com/rust-lang/crates.io-index';
  crate.crate.resolution.manifestPath = '/cargo/registry/src/meta-language-0.58.2/Cargo.toml';
  assert.deepEqual(validateFormalAiWorkloadReport(crate, fixture, 'rust').observed, []);

  const twice = passingReport();
  twice.crate.resolution.metaLanguagePackages = 2;
  const twiceResult = validateFormalAiWorkloadReport(twice, fixture, 'rust');
  assert.deepEqual(twiceResult.observed, [], 'a build that links the registry crate beside the candidate');
  assert.ok(twiceResult.problems.distinctionsPreserved.some((problem) => /resolves 2 meta-language packages/u.test(problem)));

  const integrity = passingReport();
  integrity.npm.checksum.installedIntegrity = 'sha512-other';
  assert.deepEqual(validateFormalAiWorkloadReport(integrity, fixture, 'javascript').observed, [], 'another tarball was installed');

  const reused = passingReport();
  reused.npm.resolution.consumerHadNoModules = false;
  assert.deepEqual(validateFormalAiWorkloadReport(reused, fixture, 'javascript').observed, [], 'a consumer with modules');

  const checksum = passingReport();
  checksum.crate.checksum.expectedSha256 = 'b'.repeat(64);
  assert.deepEqual(validateFormalAiWorkloadReport(checksum, fixture, 'rust').observed, [], 'a crate that is not the candidate');

  const version = passingReport();
  version.crate.resolution.installedVersion = '0.58.1';
  assert.deepEqual(validateFormalAiWorkloadReport(version, fixture, 'rust').observed, [], 'another meta-language version');
});

test('a checkout whose workload blob differs from the pin observes nothing', () => {
  const report = passingReport();
  report.formalAi.verifiedBlobs[0].actual = '0'.repeat(40);
  const result = validateFormalAiWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.distinctionsPreserved.some((problem) => problem.includes(fixture.workloads[0].file)));

  const missing = passingReport();
  missing.formalAi.verifiedBlobs.pop();
  assert.deepEqual(validateFormalAiWorkloadReport(missing, fixture, 'rust').observed, [], 'an unverified blob');

  const moved = passingReport();
  moved.formalAi.checkedOutRevision = 'e'.repeat(40);
  assert.deepEqual(validateFormalAiWorkloadReport(moved, fixture, 'rust').observed, [], 'another revision');

  const repository = passingReport();
  repository.formalAi.repository = 'someone/formal-ai';
  assert.deepEqual(validateFormalAiWorkloadReport(repository, fixture, 'javascript').observed, [], 'another repository');
});

test('a missing or skipped runtime observes nothing for that runtime only', () => {
  const report = passingReport();
  delete report.crate;
  assert.deepEqual(validateFormalAiWorkloadReport(report, fixture, 'rust').observed, []);
  assert.deepEqual(validateFormalAiWorkloadReport(report, fixture, 'javascript').observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS]);

  const skipped = passingReport();
  skipped.crate = { skipped: true, reason: '--skip-rust: the Rust workloads were not run' };
  const result = validateFormalAiWorkloadReport(skipped, fixture, 'rust');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.distinctionsPreserved.some((problem) => /--skip-rust/u.test(problem)));

  const stale = passingReport();
  stale.schemaVersion = FORMAL_AI_WORKLOAD_SCHEMA_VERSION + 1;
  assert.deepEqual(failing(validateFormalAiWorkloadReport(stale, fixture, 'rust')), ['workloadsRunOnInstalledArtifacts']);
  assert.throws(() => validateFormalAiWorkloadReport(passingReport(), fixture, 'python'), /unknown runtime python/u);
});

test('a failed or empty probe check leaves only its assertion unobserved', () => {
  for (const assertion of FORMAL_AI_PROBE_ASSERTIONS) {
    const report = passingReport();
    report.npm.probe[assertion].checks.push({
      name: 'both runtimes load the same translation rule set',
      holds: false,
      detail: '1 of 1 differ',
    });
    const result = validateFormalAiWorkloadReport(report, fixture, 'javascript');
    assert.deepEqual(failing(result), [assertion]);
    assert.match(result.problems[assertion][0], /the same translation rule set" failed: 1 of 1 differ/u);
  }
  const empty = passingReport();
  empty.crate.probe.distinctionsPreserved.checks = [];
  assert.deepEqual(failing(validateFormalAiWorkloadReport(empty, fixture, 'rust')), ['distinctionsPreserved']);

  const contradicted = passingReport();
  contradicted.crate.probe.sharedConceptsReused.holds = false;
  assert.deepEqual(failing(validateFormalAiWorkloadReport(contradicted, fixture, 'rust')), ['sharedConceptsReused']);
});

const liveSkip = !reportPath
  ? `${FORMAL_AI_WORKLOAD_REPORT_VARIABLE} is not set; the workload report is produced by the CI job "Formal AI Workloads"`
  : !existsSync(reportPath)
    ? `${FORMAL_AI_WORKLOAD_REPORT_VARIABLE} names ${reportPath}, which does not exist; the workloads stay unobserved`
    : false;

test('the CI workload report holds every assertion for the installed npm package', { skip: liveSkip }, () => {
  const report = readProjectData(reportPath);
  assert.equal(report.requirementId, FORMAL_AI_WORKLOAD_REQUIREMENT);
  assert.equal(report.fixture, FORMAL_AI_WORKLOAD_FIXTURE);
  const { problems, observed } = validateFormalAiWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(problems, Object.fromEntries(FORMAL_AI_WORKLOAD_ASSERTIONS.map((assertion) => [assertion, []])));
  assert.deepEqual(observed, [...FORMAL_AI_WORKLOAD_ASSERTIONS]);
  recordIssue195Observations({
    requirementId: FORMAL_AI_WORKLOAD_REQUIREMENT,
    suffix: 'behavior',
    fixtureId: FORMAL_AI_WORKLOAD_FIXTURE_ID,
    fixtureFile: FORMAL_AI_WORKLOAD_FIXTURE,
    assertions: observed,
    testName: 'the CI workload report holds every assertion for the installed npm package',
    runtime: 'javascript',
  });
});

// formal-ai's committed self-AST census (data/meta/self-ast/src/agentic_coding/
// code_task.lino at d209aac) as meta-language 0.58.2 rendered it, trimmed to
// its ast section. This checkout adds the grammar provenance link and splits
// the text of 12 plain line comments into whitespace and hidden-text tokens,
// so the lossless total grows by 13 while the abstract syntax is unchanged.
const CENSUS = [
  '  ast',
  '    engine meta_language',
  '    projection abstract_syntax',
  '    text_preserved true',
  '    clean true',
  '    total_link_count 10494',
  '    named_node_count 2088',
  '    distinct_node_kinds 81',
  '    node_kinds',
  '      abstract_type 1',
  '',
].join('\n');

test('the self-AST census refresh may change only the lossless link totals', () => {
  const file = 'src/agentic_coding/code_task.lino';
  const refreshed = CENSUS.replace('total_link_count 10494', 'total_link_count 10507');
  assert.deepEqual(compareSelfAstCensus(new Map([[file, CENSUS]]), new Map([[file, refreshed]])), {
    documents: 1,
    linkTotals: [{ file, before: 10494, after: 10507 }],
    otherChanges: [],
  });
  assert.deepEqual(compareSelfAstCensus(new Map([[file, CENSUS]]), new Map([[file, CENSUS]])).linkTotals, []);

  const regressed = refreshed.replace('named_node_count 2088', 'named_node_count 2087').replace('abstract_type 1', 'abstract_type 0');
  assert.deepEqual(compareSelfAstCensus(new Map([[file, CENSUS]]), new Map([[file, regressed]])).otherChanges, [
    { file, line: 7, before: '    named_node_count 2088', after: '    named_node_count 2087' },
    { file, line: 10, before: '      abstract_type 1', after: '      abstract_type 0' },
  ]);
  assert.deepEqual(compareSelfAstCensus(new Map([[file, CENSUS]]), new Map()).otherChanges, [
    { file, line: null, before: 'present', after: 'absent' },
  ]);
});

// The CI job was shut down (exit 143) without output while cargo built formal-ai
// with one job per core and full debuginfo; the runner now bounds the build and
// reports its progress and resources on every command.
test('the workload runner bounds the formal-ai cargo build and reports its progress', () => {
  const runner = readFileSync(new URL('../scripts/run-formal-ai-workloads.mjs', import.meta.url), 'utf8');
  const defaults = /const RUST_BUILD_DEFAULTS = Object\.freeze\(\{([^}]*)\}\)/u.exec(runner)?.[1] ?? '';
  for (const [name, value] of [
    ['CARGO_BUILD_JOBS', '2'],
    ['CARGO_INCREMENTAL', '0'],
    ['CARGO_PROFILE_DEV_DEBUG', '0'],
    ['CARGO_PROFILE_TEST_DEBUG', '0'],
    ['RUST_TEST_THREADS', '2'],
  ]) {
    assert.match(defaults, new RegExp(`${name}: '${value}'`, 'u'), `RUST_BUILD_DEFAULTS sets ${name}=${value}`);
  }
  assert.match(runner, /\.\.\.RUST_BUILD_DEFAULTS,\s*\.\.\.childEnvironment\(\),\s*CARGO_TARGET_DIR/u);
  // Unoptimized, the corpus ratchet parsed for more than 23 minutes: the patched crate is optimized.
  assert.match(runner, /const PATCHED_CRATE_OPT_LEVEL = 3;/u);
  assert.match(runner, /'--config', `profile\.dev\.package\.meta-language\.opt-level=\$\{PATCHED_CRATE_OPT_LEVEL\}`/u);
  assert.match(runner, /\] start: .*resources\(\)/u);
  assert.match(runner, /\] exit .*resources\(\)/u);
  // It was still shut down, silently, inside the patched `cargo test`: a running command
  // prints a heartbeat, and is killed as a failure before memory runs out.
  assert.match(runner, /\] running for .*resources\(\).*last output/u);
  assert.match(runner, /os\.freemem\(\) >= MEMORY_FLOOR_BYTES/u);
  assert.match(runner, /process\.kill\(-child\.pid, 'SIGKILL'\)/u);

  const selectorSource = /\nfunction formalAiCargoTargetDirectory\([^]*?\n\}\n/u.exec(runner)?.[0];
  assert.ok(selectorSource, 'the explicit target selector is inspectable without starting workloads');
  const selectTarget = new Function('path', 'lstatSync', 'realpathSync', `${selectorSource}\nreturn formalAiCargoTargetDirectory;`)(path, lstatSync, realpathSync);
  const scratch = realpathSync(mkdtempSync(path.join(tmpdir(), 'meta-language-formal-ai-cache-control-')));
  try {
    const temporary = path.join(scratch, 'runner-temp'); mkdirSync(temporary);
    const work = path.join(temporary, 'issue-195-formal-ai-workloads');
    const owner = path.join(temporary, 'issue-195-formal-ai-build');
    const target = path.join(owner, 'target');
    const environment = { CI: 'true', GITHUB_ACTIONS: 'true', RUNNER_TEMP: temporary, CARGO_TARGET_DIR: '/unrelated', CARGO_BUILD_TARGET_DIR: '/also-unrelated' };
    assert.equal(selectTarget(work, null, {}), path.join(work, 'formal-ai-target'));
    assert.equal(selectTarget(work, null, environment), path.join(work, 'formal-ai-target'), 'ambient Cargo storage never selects a cache');
    assert.equal(selectTarget(work, target, environment), target);
    for (const changed of [ { CI: 'false' }, { GITHUB_ACTIONS: 'false' }, { RUNNER_TEMP: '' }, { RUNNER_TEMP: 'relative' }, { RUNNER_TEMP: temporary + path.sep + '.' } ]) {
      assert.throws(() => selectTarget(work, target, { ...environment, ...changed }), /RUNNER_TEMP/u);
    }
    for (const requested of [undefined, '', 'relative', temporary, owner, target + path.sep + '..', path.join(scratch, 'other-target')]) {
      assert.throws(() => selectTarget(work, requested, environment), /exact task-owned/u);
    }
    for (const overlapping of [temporary, owner, target, path.join(target, 'work')]) {
      assert.throws(() => selectTarget(overlapping, target, environment), /must not contain/u);
    }
    const outside = path.join(scratch, 'outside'); mkdirSync(outside);
    symlinkSync(outside, owner, process.platform === 'win32' ? 'junction' : 'dir');
    assert.throws(() => selectTarget(work, target, environment), /symlink/u);
    rmSync(owner); mkdirSync(owner);
    symlinkSync(outside, target, process.platform === 'win32' ? 'junction' : 'dir');
    assert.throws(() => selectTarget(work, target, environment), /symlink/u);
    rmSync(target); writeFileSync(target, 'not a target directory');
    assert.throws(() => selectTarget(work, target, environment), /non-directory/u);
    rmSync(target); mkdirSync(target);
    assert.equal(selectTarget(work, target, environment), target);
  } finally { rmSync(scratch, { recursive: true, force: true }); }
  assert.match(runner, /const cargoTargetDirectory = formalAiCargoTargetDirectory\(workDirectory, option\('--cargo-target-dir', null\)\);/u);
  assert.match(runner, /const target = cargoTargetDirectory;/u);
  assert.match(runner, /await rm\(workDirectory, \{ recursive: true, force: true \}\);/u);
  assert.match(runner, /const cache = path\.join\(workDirectory, 'npm-cache'\);/u);
  assert.match(runner, /const freshCache = !existsSync\(cache\);/u);
  assert.match(runner, /'update', '-p', 'meta-language'/u);
  assert.match(runner, /'metadata', '--format-version', '1'/u);
  assert.match(runner, /await refreshSelfAstCensus\(formalAiDirectory, directory, configuration, environment\)/u);
  assert.match(runner, /'test', '--no-fail-fast'/u);
  assert.match(runner, /metaLanguagePackages\.length === 1/u);
  assert.match(runner, /resolution\.manifestPath\?\.startsWith\(resolution\.unpackedDirectory\)/u);

  const job = parseWorkflow(readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8'))
    .find(entry => entry.id === 'formal-ai-workload-parts');
  const names = job.steps.map(step => step.name);
  const identify = job.steps.find(step => step.name === 'Identify the exact Formal AI build cache');
  const restore = job.steps.find(step => step.name === 'Restore Formal AI build outputs');
  const execute = job.steps.find(step => step.name === 'Run the link-assistant/formal-ai workloads on the exact artifacts');
  const save = job.steps.find(step => step.name === 'Save Formal AI build outputs');
  const cleanup = job.steps.find(step => step.name === 'Clean regenerable caches');
  for (const step of [identify, restore, execute, save, cleanup]) assert.ok(step);
  assert.ok(names.indexOf(identify.name) < names.indexOf(restore.name));
  assert.ok(names.indexOf(restore.name) < names.indexOf(execute.name));
  assert.ok(names.indexOf(execute.name) < names.indexOf(save.name));
  assert.ok(names.indexOf(save.name) < names.indexOf(cleanup.name));
  assert.match(identify.text, /runner\.os.*runner\.arch.*rust1\.99\.0-opt3-debug0-incremental0/u);
  assert.match(identify.text, /hashFiles\('parity\/fixtures\/formal-ai-workloads\.lino'/u);
  assert.match(identify.run, /sha256sum --check/u);
  assert.match(identify.run, /compiler_identity="\$\(rustc -vV\)"/u);
  assert.ok(identify.run.includes('$dependency_key-$candidate_digest'));
  for (const step of [restore, save]) {
    assert.match(step.text, /path: \|\n\s+\$\{\{ runner\.temp \}\}\/issue-195-formal-ai-build\/target\/debug\n/u);
    assert.match(step.text, /target\/\.rustc_info\.json/u);
    assert.match(step.text, /target\/CACHEDIR\.TAG/u);
    assert.match(step.text, /key: \$\{\{ steps\.formal-ai-cache-key\.outputs\.key \}\}/u);
    assert.doesNotMatch(step.text, /npm-cache|node_modules|crate-source|report\.lino|outputs\.lino|probe/u);
  }
  assert.match(restore.text, /restore-keys: \|\n\s+\$\{\{ steps\.formal-ai-cache-key\.outputs\.dependency_key \}\}-/u);
  assert.doesNotMatch(execute.text, /cache-hit|\n\s+if:/u, 'cache hits can never skip exact candidate workloads');
  assert.ok(execute.run.includes('--cargo-target-dir "$RUNNER_TEMP/issue-195-formal-ai-build/target"'));
  assert.ok(cleanup.run.includes('--target-dir "$RUNNER_TEMP/issue-195-formal-ai-build/target"'));
  assert.ok(cleanup.run.startsWith('env -u CARGO_TARGET_DIR -u CARGO_BUILD_TARGET_DIR node scripts/clean-caches.mjs --event ci-teardown '));
});

test('the workload runner retains raw command output before exit and after spawn failure', async () => {
  // Exercise each actual command runner with tiny JavaScript children. No downstream
  // checkout, dependency installation, native compilation or workload runs here.
  const fileSystem = await import('node:fs');
  const promises = await import('node:fs/promises');
  const childProcess = await import('node:child_process');
  const operatingSystem = await import('node:os');
  const timers = await import('node:timers/promises');
  for (const script of ['run-formal-ai-workloads.mjs', 'run-rml-pr184-workloads.mjs']) {
    const source = readFileSync(new URL(`../scripts/${script}`, import.meta.url), 'utf8');
    const definition = /\nasync function run\([^]*?\n\}\n/u.exec(source)?.[0];
    assert.ok(definition, 'the actual command runner is available without starting workloads');
    const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-command-transcript-'));
    const records = [];
    const run = new Function('path', 'workDirectory', 'spawn', 'writeFile', 'appendFileSync', 'logs', 'childEnvironment', 'resources', 'os', 'HEARTBEAT_MS', 'MEMORY_FLOOR_BYTES', `${definition}\nreturn run;`)(
      path, directory, childProcess.spawn, promises.writeFile, fileSystem.appendFileSync, records,
      new Function('return process.env;'), new Function('return Promise.resolve("authored command control");'),
      operatingSystem, 60_000, 0,
    );
    const released = path.join(directory, 'release');
    const childSource = `const fs = require('node:fs');
process.stdout.write(Buffer.from([0xcf]));
const interval = setInterval(() => {
  if (!fs.existsSync(process.argv[1])) return;
  process.stdout.write(Buffer.from([0x80, 0xf0, 0x9f, 0x98, 0x80, 10]));
  clearInterval(interval);
}, 10);
setTimeout(() => { clearInterval(interval); process.exitCode = 24; }, 5000).unref();`;
    const pending = run('stream', process.execPath, ['-e', childSource, released]);
    try {
      try {
        const transcript = path.join(directory, 'stream.log');
        const deadline = Date.now() + 4_000;
        while ((!existsSync(transcript) || readFileSync(transcript).at(-1) !== 0xcf) && Date.now() < deadline) {
          await timers.setTimeout(10);
        }
        const partial = readFileSync(transcript);
        assert.equal(partial.at(-1), 0xcf, 'the live log retains an incomplete UTF-8 scalar as its original byte');
        assert.doesNotMatch(partial.toString('utf8'), /\nexit:/u, 'a running child has no invented exit');
      } finally {
        writeFileSync(released, 'finish');
      }
      const streamed = await pending;
      assert.equal(streamed.code, 0);
      assert.equal(streamed.stdout, 'π😀\n');
      assert.equal(streamed.stderr, '');
      assert.equal(streamed.combined, streamed.stdout);
      assert.equal(readFileSync(streamed.log, 'utf8'), `command: ${process.execPath} -e ${childSource} ${released}\ncwd: ${directory}\n\n${streamed.combined}\nexit: 0\n`);
      const failed = await run('failed', process.execPath, ['-e', 'process.stdout.write("output\\n"); process.stderr.write("problem\\n"); process.exitCode = 23;'], { allowFailure: true });
      assert.equal(failed.code, 23);
      assert.equal(failed.stdout, 'output\n');
      assert.equal(failed.stderr, 'problem\n');
      assert.equal(readFileSync(failed.log, 'utf8'), `command: ${records.at(-1).command}\ncwd: ${directory}\n\n${failed.combined}\nexit: 23\n`);
      await assert.rejects(run('spawn-failure', path.join(directory, 'absent-command'), []), { code: 'ENOENT' });
      assert.match(readFileSync(path.join(directory, 'spawn-failure.log'), 'utf8'), /command: [^]*\nspawn error: [^]*ENOENT/u);
      assert.deepEqual(records.map(({ label, exit }) => [label, exit]), [['stream', 0], ['failed', 23]]);
    } finally {
      await Promise.allSettled([pending]);
      rmSync(directory, { recursive: true, force: true });
    }
  }
});

// The memory guard killed the Rust workloads in grammar_projection_corpus_ratchet
// with two tests running at once: the tests now run one at a time and each one's
// line gives its result, seconds and the lowest free memory while it ran.
test('the workload runner logs each Rust test with the lowest free memory while it ran', () => {
  const runner = readFileSync(new URL('../scripts/run-formal-ai-workloads.mjs', import.meta.url), 'utf8');
  assert.match(runner, /'--', '--test-threads=1', \.\.\.filters,\s*\], \{[^}]*RUST_TEST_THREADS: '1'[^}]*\}, allowFailure: true, traceTests: true \}\)/u);
  const source = /\nfunction testTrace\(label\) \{[\s\S]*?\n\}\n/u.exec(runner)?.[0];
  assert.ok(source, 'testTrace is defined');
  let free = 8 * 2 ** 30;
  const lines = [];
  const testTrace = new Function('os', 'console', `${source}\nreturn testTrace;`)(
    { freemem: () => free },
    { log: (line) => lines.push(line) },
  );
  const trace = testTrace('rust-workloads');
  trace.take(Buffer.from('running 2 tests\ntest a::first ... '));
  free = 3 * 2 ** 30;
  trace.sample();
  free = 6 * 2 ** 30;
  trace.take(Buffer.from('ok\ntest a::second has been running for over 60 seconds\ntest a::sec'));
  trace.take(Buffer.from('ond ... FAILED\n'));
  assert.equal(lines.length, 2);
  assert.match(lines[0], /^\[rust-workloads\] test a::first: ok after \d+ s, lowest free memory 3\.0 GiB$/u);
  assert.match(lines[1], /^\[rust-workloads\] test a::second: FAILED after \d+ s, lowest free memory 6\.0 GiB$/u);
});

test('an ignored workload cannot supply installed artifact acceptance evidence',()=>{
  // Run 38862173174, job 116854149833, head 61d3a766760306f3c0c2a93ef2d79450b465f242:
  // 133 of 134 Rust workloads passed; this exhaustive round trip was ignored.
  // Retained job log SHA-256: 167a73feb906fc8a0e921d049eccfccc7e5d8957e0761067a9866ca056464fc2.
  for(const runtime of ['javascript','rust']){
    const report=passingReport(),part=report[runtime==='javascript'?'npm':'crate'];
    const file=runtime==='rust'?'rust/tests/unit/issue_558_source_links.rs':workloadFiles(fixture,runtime)[0];
    assert.ok(workloadFiles(fixture,runtime).includes(file));
    part.tests.push({file,name:'issue_558_source_links::exhaustive_whole_repo_round_trip_is_lossless',outcome:'skipped'});
    part.counts=countOutcomes(part.tests);assert.equal(part.exit,0);
    const result=validateFormalAiWorkloadReport(report,fixture,runtime);
    assert.deepEqual(failing(result),['workloadsRunOnInstalledArtifacts']);
    assert.ok(result.problems.workloadsRunOnInstalledArtifacts.some(problem=>problem.includes('exhaustive_whole_repo_round_trip_is_lossless was skipped')));
  }
});

test('the workload runner includes ignored tests while retaining every original filter',()=>{
  const runner=readFileSync(new URL('../scripts/run-formal-ai-workloads.mjs',import.meta.url),'utf8');
  const declaration=/  const filters = ([^\n]+);/u.exec(runner)?.[1];assert.ok(declaration);
  const filters=new Function('workloads',`return ${declaration};`)(fixture.workloads.filter(({kind,file})=>kind==='test'&&file.startsWith('rust/')));
  assert.equal(filters.filter(value=>value==='--include-ignored').length,1);
  assert.deepEqual(filters.filter(value=>value!=='--include-ignored'),[...new Set([...fixture.workloads.filter(({kind,file})=>kind==='test'&&file.startsWith('rust/')).map(({filter})=>filter),'issue_195_probe_'])]);
  assert.ok(filters.every(value=>value==='--include-ignored'||!value.startsWith('--')));
});
