// The relative-meta-logic pull request 184 workloads run against the clean
// installed meta-language artifacts (I195-DOWNSTREAM-RML-WORKLOADS).
// js/scripts/run-rml-pr184-workloads.mjs writes the report in CI; this suite
// checks the validator on synthetic reports and, when
// ISSUE_195_RML_WORKLOAD_REPORT names the CI report, validates its npm part
// and records the JavaScript observations. The Rust suite
// (rust/tests/unit/issue_195_downstream_rml_workloads.rs) reads the crate part.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';

import {
  RML_PROBE_ASSERTIONS,
  RML_WORKLOAD_ASSERTIONS,
  RML_WORKLOAD_FIXTURE,
  RML_WORKLOAD_FIXTURE_ID,
  RML_WORKLOAD_REPORT_VARIABLE,
  RML_WORKLOAD_REQUIREMENT,
  RML_WORKLOAD_SCHEMA_VERSION,
  countOutcomes,
  loadRmlWorkloadFixture,
  validateRmlWorkloadReport,
  workloadTestFiles,
} from '../scripts/issue-195-rml-workloads.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixture = loadRmlWorkloadFixture();
const reportPath = process.env[RML_WORKLOAD_REPORT_VARIABLE];

// A report on which every assertion of both runtimes holds.
function passingReport() {
  const digest = 'a'.repeat(64);
  const probe = Object.fromEntries(RML_PROBE_ASSERTIONS.map((assertion) => [
    assertion,
    { holds: true, checks: [{ name: `${assertion} check`, holds: true, detail: {} }] },
  ]));
  const part = (runtime, resolution, extra = {}) => {
    const tests = workloadTestFiles(fixture, runtime).map((file) => ({ file, name: 'a workload', outcome: 'passed' }));
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
    schemaVersion: RML_WORKLOAD_SCHEMA_VERSION,
    requirementId: RML_WORKLOAD_REQUIREMENT,
    rml: {
      repository: fixture.repository,
      headRevision: fixture.headRevision,
      checkedOutRevision: fixture.headRevision,
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

const failing = (result) => RML_WORKLOAD_ASSERTIONS.filter((assertion) => !result.observed.includes(assertion));

test('the fixture inventories the workload test files of both runtimes', () => {
  assert.deepEqual(workloadTestFiles(fixture, 'javascript'), [
    'js/tests/meta-language-support.test.mjs',
    'js/tests/theory-network-linked.test.mjs',
    'js/tests/theory-network.test.mjs',
  ]);
  assert.deepEqual(workloadTestFiles(fixture, 'rust'), [
    'rust/tests/meta_language_support_tests.rs',
    'rust/tests/theory_network_tests.rs',
  ]);
});

test('a report on which everything holds observes every assertion of both runtimes', () => {
  for (const runtime of ['javascript', 'rust']) {
    const result = validateRmlWorkloadReport(passingReport(), fixture, runtime);
    assert.deepEqual(result.observed, [...RML_WORKLOAD_ASSERTIONS], runtime);
  }
});

test('a failing workload test leaves the workloads unobserved', () => {
  const report = passingReport();
  report.npm.tests.push({ file: 'js/tests/theory-network.test.mjs', name: 'loads the network', outcome: 'failed' });
  report.npm.counts = countOutcomes(report.npm.tests);
  report.npm.exit = 1;
  const result = validateRmlWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(failing(result), ['workloadsRunOnInstalledArtifacts']);
  assert.ok(result.problems.workloadsRunOnInstalledArtifacts.some((problem) => /loads the network failed/u.test(problem)));

  const unrun = passingReport();
  unrun.crate.tests = unrun.crate.tests.filter(({ file }) => file !== 'rust/tests/theory_network_tests.rs');
  unrun.crate.counts = countOutcomes(unrun.crate.tests);
  assert.deepEqual(failing(validateRmlWorkloadReport(unrun, fixture, 'rust')), ['workloadsRunOnInstalledArtifacts'],
    'a workload file without a passed test');

  const miscounted = passingReport();
  miscounted.npm.counts.passed += 1;
  assert.deepEqual(failing(validateRmlWorkloadReport(miscounted, fixture, 'javascript')), ['workloadsRunOnInstalledArtifacts'],
    'counts that differ from the executed tests');
});

test('a consumer that resolved meta-language from the registry observes nothing', () => {
  const report = passingReport();
  report.npm.resolution.lockResolved = 'https://registry.npmjs.org/meta-language/-/meta-language-0.46.0.tgz';
  const result = validateRmlWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.sharedConceptsReused.some((problem) => /registry\.npmjs\.org/u.test(problem)));

  const crate = passingReport();
  crate.crate.resolution.metadataSource = 'registry+https://github.com/rust-lang/crates.io-index';
  crate.crate.resolution.lockSource = 'registry+https://github.com/rust-lang/crates.io-index';
  crate.crate.resolution.manifestPath = '/cargo/registry/src/meta-language-1.2.3/Cargo.toml';
  assert.deepEqual(validateRmlWorkloadReport(crate, fixture, 'rust').observed, []);

  const integrity = passingReport();
  integrity.npm.checksum.installedIntegrity = 'sha512-other';
  assert.deepEqual(validateRmlWorkloadReport(integrity, fixture, 'javascript').observed, [], 'another tarball was installed');

  const reused = passingReport();
  reused.npm.resolution.freshCache = false;
  assert.deepEqual(validateRmlWorkloadReport(reused, fixture, 'javascript').observed, [], 'a reused npm cache');

  const checksum = passingReport();
  checksum.crate.checksum.expectedSha256 = 'b'.repeat(64);
  assert.deepEqual(validateRmlWorkloadReport(checksum, fixture, 'rust').observed, [], 'a crate that is not the candidate');
});

test('a checkout whose workload blob differs from the pin observes nothing', () => {
  const report = passingReport();
  report.rml.verifiedBlobs[0].actual = '0'.repeat(40);
  const result = validateRmlWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.foundationAuthorityStaysInRml.some((problem) => problem.includes(fixture.workloads[0].file)));

  const missing = passingReport();
  missing.rml.verifiedBlobs.pop();
  assert.deepEqual(validateRmlWorkloadReport(missing, fixture, 'rust').observed, [], 'an unverified blob');

  const moved = passingReport();
  moved.rml.checkedOutRevision = 'e'.repeat(40);
  assert.deepEqual(validateRmlWorkloadReport(moved, fixture, 'rust').observed, [], 'another head');
});

test('a missing or skipped runtime observes nothing for that runtime only', () => {
  const report = passingReport();
  delete report.crate;
  assert.deepEqual(validateRmlWorkloadReport(report, fixture, 'rust').observed, []);
  assert.deepEqual(validateRmlWorkloadReport(report, fixture, 'javascript').observed, [...RML_WORKLOAD_ASSERTIONS]);

  const skipped = passingReport();
  skipped.npm = { skipped: true, reason: 'the npm install failed' };
  const result = validateRmlWorkloadReport(skipped, fixture, 'javascript');
  assert.deepEqual(result.observed, []);
  assert.ok(result.problems.distinctionsPreserved.some((problem) => /the npm install failed/u.test(problem)));

  const stale = passingReport();
  stale.schemaVersion = RML_WORKLOAD_SCHEMA_VERSION + 1;
  assert.deepEqual(failing(validateRmlWorkloadReport(stale, fixture, 'rust')), ['workloadsRunOnInstalledArtifacts']);
  assert.throws(() => validateRmlWorkloadReport(passingReport(), fixture, 'python'), /unknown runtime python/u);
});

test('a failed or empty probe check leaves only its assertion unobserved', () => {
  for (const assertion of RML_PROBE_ASSERTIONS) {
    const report = passingReport();
    report.crate.probe[assertion].checks.push({ name: 'the evaluator closure', holds: false, detail: 'meta-language reached' });
    const result = validateRmlWorkloadReport(report, fixture, 'rust');
    assert.deepEqual(failing(result), [assertion]);
    assert.match(result.problems[assertion][0], /the evaluator closure" failed: meta-language reached/u);
  }
  const empty = passingReport();
  empty.npm.probe.distinctionsPreserved.checks = [];
  assert.deepEqual(failing(validateRmlWorkloadReport(empty, fixture, 'javascript')), ['distinctionsPreserved']);

  const contradicted = passingReport();
  contradicted.npm.probe.sharedConceptsReused.holds = false;
  assert.deepEqual(failing(validateRmlWorkloadReport(contradicted, fixture, 'javascript')), ['sharedConceptsReused']);
});

const liveSkip = !reportPath
  ? `${RML_WORKLOAD_REPORT_VARIABLE} is not set; the workload report is produced by the CI job "Relative Meta Logic Workloads"`
  : !existsSync(reportPath)
    ? `${RML_WORKLOAD_REPORT_VARIABLE} names ${reportPath}, which does not exist; the workloads stay unobserved`
    : false;

test('the CI workload report holds every assertion for the installed npm package', { skip: liveSkip }, () => {
  const report = JSON.parse(readFileSync(reportPath, 'utf8'));
  assert.equal(report.requirementId, RML_WORKLOAD_REQUIREMENT);
  assert.equal(report.fixture, RML_WORKLOAD_FIXTURE);
  const { problems, observed } = validateRmlWorkloadReport(report, fixture, 'javascript');
  assert.deepEqual(problems, Object.fromEntries(RML_WORKLOAD_ASSERTIONS.map((assertion) => [assertion, []])));
  assert.deepEqual(observed, [...RML_WORKLOAD_ASSERTIONS]);
  recordIssue195Observations({
    requirementId: RML_WORKLOAD_REQUIREMENT,
    suffix: 'behavior',
    fixtureId: RML_WORKLOAD_FIXTURE_ID,
    fixtureFile: RML_WORKLOAD_FIXTURE,
    assertions: observed,
    testName: 'the CI workload report holds every assertion for the installed npm package',
    runtime: 'javascript',
  });
});
