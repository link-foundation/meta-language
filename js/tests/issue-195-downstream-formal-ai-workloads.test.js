// The link-assistant/formal-ai workloads run against the clean installed
// meta-language artifacts (I195-DOWNSTREAM-FORMAL-AI-WORKLOADS).
// js/scripts/run-formal-ai-workloads.mjs writes the report in CI; this suite
// checks the validator on synthetic reports and, when
// ISSUE_195_FORMAL_AI_WORKLOAD_REPORT names the CI report, validates its npm
// part and records the JavaScript observations. The Rust suite
// (rust/tests/unit/issue_195_downstream_formal_ai_workloads.rs) reads the crate part.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';

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
  const report = JSON.parse(readFileSync(reportPath, 'utf8'));
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
