import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { assertionVersionPolicy } from '../scripts/generate-assertion-version-policy.mjs';
import { ANTLR_REFERENCE_POLICY_FILE, ASSERTION_VERSION_POLICY_FILE,
  readAssertionVersionPolicy, selectAntlrReferenceAssertion } from '../scripts/antlr-version-assertions.mjs';
import { createObservationVersionValidator } from '../scripts/observation-version-assertions.mjs';
import { selectPeggyOptimizerAssertion } from '../scripts/peggy-version-assertions.mjs';
import { evaluateIssue195Acceptance } from '../scripts/issue-195-acceptance-lib.mjs';
import { VISION_REQUIREMENTS, buildVisionRequirements } from '../scripts/issue-195-vision-requirements.mjs';
import { writeProjectData } from '../scripts/project-data.mjs';
import { readStructuredDataRecords } from '../src/structured-data-records.js';

const nativePolicyFile = 'rust/src/data/grammar-lexer-rules.lino';
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const requirementId = 'I195-ASSERTION-EXPLICIT-VERSION-RANGES';

function withVersionFixture(run) {
  // Node resolves package manifests through real paths. macOS tmpdir aliases
  // /var to /private/var, so use the same real root for fixture containment.
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'meta-language-assertion-context-')));
  const policy = assertionVersionPolicy();
  const expectation = policy.assertions.find(({ id }) => id === 'antlr-lexer-reference-semantics').expectations[0];
  const source = { upstreamSemantics: { version: expectation.expected, ...expectation.sourceReferences } };
  const writeData = (file, value) => {
    mkdirSync(path.dirname(path.join(root, file)), { recursive: true });
    writeProjectData(path.join(root, file), value);
  };
  const writeSource = value => {
    writeData(ANTLR_REFERENCE_POLICY_FILE, value);
    writeData(nativePolicyFile, value);
  };
  // These are external npm manifest interfaces, whose required syntax is JSON.
  const writeManifest = (file, value) => {
    mkdirSync(path.dirname(path.join(root, file)), { recursive: true });
    writeFileSync(path.join(root, file), JSON.stringify(value));
  };
  try {
    writeData(ASSERTION_VERSION_POLICY_FILE, policy);
    writeSource(source);
    writeManifest('js/package.json', { devDependencies: { peggy: '5.1.0' } });
    writeManifest('js/package-lock.json', { packages: { 'node_modules/peggy': { version: '5.1.0' } } });
    writeManifest('js/node_modules/peggy/package.json', { name: 'peggy', version: '5.1.0' });
    for (const binding of policy.observationBindings) {
      const file = path.join(root, binding.fixtureFile);
      mkdirSync(path.dirname(file), { recursive: true });
      writeFileSync(file, `// Independent fixture for ${binding.assertionId}\n`);
    }
    return run({ root, policy, source, writeData, writeSource, writeManifest });
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

const selectContext = (root, binding) => binding.assertionPolicy === 'antlr-lexer-reference-semantics'
  ? selectAntlrReferenceAssertion({ root }) : selectPeggyOptimizerAssertion({ root });

function observation(root, binding) {
  return {
    testId: binding.testId, assertionId: binding.assertionId, fixtureId: binding.fixtureId,
    fixtureDigest: sha256(readFileSync(path.join(root, binding.fixtureFile))),
    runtime: 'tooling', commit: 'candidate-sha', outcome: 'passed', testName: 'version assertion fixture',
    versionApplicability: [structuredClone(selectContext(root, binding))],
  };
}

function acceptanceCase(root, binding) {
  const record = observation(root, binding);
  const manifest = {
    fixtureCatalog: { [binding.fixtureId]: { path: binding.fixtureFile, selector: '$', sha256: record.fixtureDigest } },
    atomicRequirements: [{
      id: requirementId, area: 'assertion-version-ranges', requiredRuntimes: ['tooling'],
      implementationEntryPoints: { tooling: ['js/scripts/antlr-version-assertions.mjs'] },
      verifications: [{ testId: binding.testId, runtime: 'tooling', kind: 'positive', checkpoint: 'pre-merge',
        fixtureIds: [binding.fixtureId], assertions: [binding.assertionId] }],
    }],
  };
  const document = {
    schemaVersion: 1, issue: 195, commit: 'candidate-sha', producer: 'version-context-fixture',
    generatedAt: new Date(0).toISOString(),
    results: [{ testId: binding.testId, outcome: 'passed', kind: 'positive', positiveEvidence: true,
      command: 'node bounded-version-context-fixture.js', toolchainVersions: { node: process.version },
      grammarVersions: { [record.versionApplicability[0].assertionPolicy]: record.versionApplicability[0].activeVersion },
      evidenceArtifacts: ['private-unit-fixture.lino'],
      assertionsPassed: [binding.assertionId], fixtureDigests: { [binding.fixtureId]: record.fixtureDigest },
      executionRecords: [record] }],
  };
  return { manifest, document };
}

test('ANTLR context authenticates actual paired source bytes, primary URLs and the fixed selected range', () => {
  const actual = selectAntlrReferenceAssertion();
  assert.equal(actual.activeVersion, '4.13.2');
  assert.deepEqual([actual.minimum, actual.maximum, actual.includeMinimum, actual.includeMaximum],
    ['4.13.2', '4.13.2', true, true]);
  assert.equal(actual.applicabilityPolicySha256, readAssertionVersionPolicy().sha256);
  assert.equal(actual.sourcePolicySha256, sha256(readFileSync(new URL('../../' + ANTLR_REFERENCE_POLICY_FILE, import.meta.url))));
  for (const version of ['ANTLR 4.13.1', 'ANTLR 4.13.3', 'ANTLR 4.14.0', 'ANTLR latest', '4.13.2', 'ANTLR 04.13.2']) {
    withVersionFixture(({ root, source, writeSource }) => {
      writeSource({ upstreamSemantics: { ...source.upstreamSemantics, version } });
      assert.throws(() => selectAntlrReferenceAssertion({ root }), /uncovered active version|explicit ANTLR X.Y.Z/u);
    });
  }
  withVersionFixture(({ root, source, writeSource, writeData }) => {
    writeSource({ upstreamSemantics: { ...source.upstreamSemantics, runtime: 'https://example.invalid/unverified' } });
    assert.throws(() => selectAntlrReferenceAssertion({ root }), /primary source URLs differ/u);
    writeSource(source);
    writeData(nativePolicyFile, { ...source, unrelatedDifference: true });
    assert.throws(() => selectAntlrReferenceAssertion({ root }), /differ between runtime packages/u);
  });
});

test('Peggy optimizer context uses the installed manifest and refuses runtime, dependency and lock drift', () => {
  const actual = selectPeggyOptimizerAssertion({ runtimeVersion: '5.1.0' });
  assert.deepEqual([actual.minimum, actual.maximum, actual.includeMinimum, actual.includeMaximum],
    ['5.1.0', '5.1.0', true, true]);
  assert.equal(actual.sourcePolicy, 'js/node_modules/peggy/package.json');
  assert.equal(actual.sourcePolicySha256, sha256(readFileSync(new URL('../../' + actual.sourcePolicy, import.meta.url))));
  assert.throws(() => selectPeggyOptimizerAssertion({ runtimeVersion: '5.1.1' }), /executing Peggy version differs/u);
  for (const version of ['5.0.9', '5.1.1', '5.2.0', 'latest', '05.1.0']) {
    withVersionFixture(({ root, writeManifest }) => {
      writeManifest('js/node_modules/peggy/package.json', { name: 'peggy', version });
      assert.throws(() => selectPeggyOptimizerAssertion({ root }), /uncovered active version|invalid 3-coordinate release/u);
    });
  }
  for (const file of ['js/package.json', 'js/package-lock.json']) {
    withVersionFixture(({ root, writeManifest }) => {
      writeManifest(file, file.endsWith('package-lock.json')
        ? { packages: { 'node_modules/peggy': { version: '5.1.1' } } }
        : { devDependencies: { peggy: '^5.1.0' } });
      assert.throws(() => selectPeggyOptimizerAssertion({ root }), /direct dependency pin and lock version must agree/u);
    });
  }
});

test('declared observation scopes require complete exact version context independently of incoming test names', () => {
  withVersionFixture(({ root, policy }) => {
    const validate = createObservationVersionValidator({ root });
    for (const binding of policy.observationBindings) {
      const record = observation(root, binding);
      assert.deepEqual(validate(record, { fixtureFile: binding.fixtureFile }), record.versionApplicability);
      assert.deepEqual(validate({ ...record, testName: 'a caller cannot unregister this binding' },
        { fixtureFile: binding.fixtureFile }), record.versionApplicability);
      const missing = structuredClone(record);
      delete missing.versionApplicability;
      assert.throws(() => validate(missing, { fixtureFile: binding.fixtureFile }), /no version applicability/u);
      for (const field of Object.keys(record.versionApplicability[0])) {
        const forged = structuredClone(record);
        delete forged.versionApplicability[0][field];
        assert.throws(() => validate(forged, { fixtureFile: binding.fixtureFile }), /differs from its actual selected source policy/u, field);
      }
      for (const [field, value] of Object.entries({ activeVersion: '4.13.3', minimum: '4.0.0', maximum: '5.0.0',
        includeMaximum: false, applicabilityPolicySha256: 'forged', sourcePolicySha256: 'forged',
        sourceReferences: {}, inventedContext: true })) {
        const forged = structuredClone(record);
        forged.versionApplicability[0][field] = value;
        assert.throws(() => validate(forged, { fixtureFile: binding.fixtureFile }), /differs from its actual selected source policy/u, field);
      }
      assert.throws(() => validate(record, { fixtureFile: 'wrong-source.js' }), /fixture path differs/u);
      assert.throws(() => validate({ ...record, fixtureDigest: 'forged' }, { fixtureFile: binding.fixtureFile }), /fixture digest differs/u);
    }
    const ordinary = { testId: 'ordinary-tooling-behavior', assertionId: 'ordinary', fixtureId: 'ordinary' };
    assert.equal(validate(ordinary), undefined);
    assert.throws(() => validate({ ...ordinary, versionApplicability: [selectAntlrReferenceAssertion({ root })] }), /unregistered observation scope/u);
  });
});

test('registered binding removal and range widening cannot silently unversion evidence', () => {
  for (const mutate of [
    policy => { policy.observationBindings = []; },
    policy => { policy.observationBindings[0].testId = 'unbound-test'; },
    policy => { policy.assertions[1].expectations[0].maximum = '5.0.0'; },
    policy => { policy.assertions[1].expectations.push({ ...policy.assertions[1].expectations[0], id: 'overlap' }); },
  ]) {
    withVersionFixture(({ root, policy, writeData }) => {
      mutate(policy);
      writeData(ASSERTION_VERSION_POLICY_FILE, policy);
      assert.throws(() => createObservationVersionValidator({ root }), /differ from their registered authoring source/u);
    });
  }
});

test('acceptance retains original exact-commit and duplicate gates and rejects missing or forged version context', () => {
  withVersionFixture(({ root, policy }) => {
    const evaluate = (manifest, document) => evaluateIssue195Acceptance(manifest, [document],
      { checkpoint: 'pre-merge', commit: 'candidate-sha', versionPolicyRoot: root });
    for (const binding of policy.observationBindings) {
    const valid = acceptanceCase(root, binding);
    assert.equal(evaluate(valid.manifest, valid.document).passed, true);
    for (const [mutate, expected] of [
      [record => { delete record.versionApplicability; }, /no version applicability/u],
      [record => { record.versionApplicability[0].activeVersion = '4.13.3'; }, /version applicability differs/u],
      [record => { record.versionApplicability[0].maximum = '5.0.0'; }, /version applicability differs/u],
      [record => { record.versionApplicability[0].sourcePolicySha256 = 'forged'; }, /version applicability differs/u],
      [record => { record.fixtureDigest = 'forged'; }, /fixture digest mismatch/u],
      [record => { record.commit = 'old-sha'; }, /execution record commit mismatch/u],
    ]) {
      const { manifest, document } = acceptanceCase(root, binding);
      mutate(document.results[0].executionRecords[0]);
      const report = evaluate(manifest, document);
      assert.equal(report.passed, false);
      assert.match(JSON.stringify(report), expected);
    }
    const duplicate = acceptanceCase(root, binding);
    duplicate.document.results[0].executionRecords.push(structuredClone(duplicate.document.results[0].executionRecords[0]));
    const report = evaluate(duplicate.manifest, duplicate.document);
    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /duplicate execution record/u);
    const mismatchedPath = acceptanceCase(root, binding);
    mismatchedPath.manifest.fixtureCatalog[binding.fixtureId].path = 'different-fixture.js';
    assert.equal(evaluate(mismatchedPath.manifest, mismatchedPath.document).passed, false);
    }
  });
});

test('the real Links observation writer preserves selected context and validates before deduplication', () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-version-writer-'));
  const file = path.join(directory, 'observations.lino');
  const writer = new URL('./support/issue-195-observations.js', import.meta.url).href;
  const adapter = new URL('../scripts/antlr-version-assertions.mjs', import.meta.url).href;
  const peggyAdapter = new URL('../scripts/peggy-version-assertions.mjs', import.meta.url).href;
  try {
    const child = spawnSync(process.execPath, ['--input-type=module', '-e', `
      import assert from 'node:assert/strict';
      import { recordIssue195Observations } from ${JSON.stringify(writer)};
      import { selectAntlrReferenceAssertion } from ${JSON.stringify(adapter)};
      import { selectPeggyOptimizerAssertion } from ${JSON.stringify(peggyAdapter)};
      const observation = {
        requirementId: ${JSON.stringify(requirementId)}, suffix: 'antlr-command-order', runtime: 'tooling',
        fixtureId: 'assertion-version-antlr-command-order-source', fixtureFile: 'js/tests/grammar-antlr-lexer-election.test.js',
        assertions: ['antlrCommandOrderHasApplicableVersionContext'], testName: 'private writer contract fixture',
        versionApplicability: [selectAntlrReferenceAssertion()],
      };
      recordIssue195Observations(observation);
      recordIssue195Observations(observation);
      const forged = structuredClone(observation);
      forged.versionApplicability[0].activeVersion = '4.13.3';
      assert.throws(() => recordIssue195Observations(forged), /version applicability differs/u);
      const missing = structuredClone(observation);
      delete missing.versionApplicability;
      assert.throws(() => recordIssue195Observations(missing), /no version applicability/u);
      recordIssue195Observations({
        requirementId: ${JSON.stringify(requirementId)}, suffix: 'peggy-character-set-shape', runtime: 'tooling',
        fixtureId: 'assertion-version-peggy-character-set-shape-source', fixtureFile: 'js/tests/grammar-interchange-tools.test.js',
        assertions: ['peggyOptimizedCharacterSetsHaveApplicableVersionContext', 'peggyCharacterSetRefusalsHaveApplicableVersionContext'],
        testName: 'private Peggy writer contract fixture', versionApplicability: [selectPeggyOptimizerAssertion()],
      });
    `], { encoding: 'utf8', env: { ...process.env, ISSUE_195_OBSERVATION_FILE: file, ISSUE_195_COMMIT: 'private-writer-commit' } });
    assert.equal(child.status, 0, child.stderr);
    const records = readStructuredDataRecords(readFileSync(file, 'utf8'));
    assert.equal(records.length, 3);
    assert.deepEqual(records[0].versionApplicability, [selectAntlrReferenceAssertion()]);
    assert.equal(records[0].commit, 'private-writer-commit');
    assert.equal(records[0].testId, assertionVersionPolicy().observationBindings[0].testId);
    for (const record of records.slice(1)) assert.deepEqual(record.versionApplicability, [selectPeggyOptimizerAssertion()]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('concrete version verification cells append without changing the original behavior scope', () => {
  const fixtures = {};
  const rows = buildVisionRequirements(fixtures, {
    requirement: value => value, verification: value => value,
    pinnedFixture: (file, selector) => ({ path: file, selector, sha256: 'bounded-builder-fixture' }),
    source: 'unchanged-source-fixture',
  });
  const row = rows.find(({ id }) => id === requirementId);
  const authoring = VISION_REQUIREMENTS.find(({ id }) => id === requirementId);
  const fixtureIds = ['conversation-contract:versionBoundedAssertionsRequest', 'assertion-version-ranges', 'unicode-identifier-properties'];
  const assertions = ['versionDependentExpectationsHaveExplicitBounds', 'activeVersionSelectsApplicableExpectation', 'historicalInputsAndExpectationsRetained'];
  assert.deepEqual(row.requiredRuntimes, ['tooling']);
  assert.deepEqual(row.verifications[0], { requirementId, runtime: 'tooling', suffix: 'behavior', fixtureIds, assertions, checkpoint: 'pre-merge' });
  assert.deepEqual(authoring.assertions, assertions);
  assert.equal(row.verifications.length, 4);
  assert.deepEqual(row.verifications.slice(1).map(({ suffix }) => suffix), ['antlr-command-order', 'antlr-inherited-options', 'peggy-character-set-shape']);
  const bindingGroups = Map.groupBy(assertionVersionPolicy().observationBindings, ({ testId }) => testId);
  for (const [index, bindings] of [...bindingGroups.values()].entries()) {
    const [binding] = bindings;
    const cell = row.verifications[index + 1];
    assert.deepEqual(cell.fixtureIds, [binding.fixtureId]);
    assert.deepEqual(cell.assertions, bindings.map(({ assertionId }) => assertionId));
    assert.equal(fixtures[binding.fixtureId].path, binding.fixtureFile);
    assert.equal(fixtures[binding.fixtureId].selector, authoring.additionalVerifications[index].fixtures[0].selector);
    assert.equal(row.verifications[0].fixtureIds.includes(binding.fixtureId), false);
  }
  for (const id of ['I195-ASSERTION-VERSION-COVERAGE', 'I195-ASSERTION-LANGUAGE-EDITION-ORDER']) {
    const unchanged = rows.find(row => row.id === id);
    assert.equal(unchanged.verifications.length, 1);
    assert.deepEqual(unchanged.verifications[0].fixtureIds, fixtureIds);
    assert.equal(unchanged.implementationEntryPoints.tooling, null);
  }
});
