import { access, readFile } from 'node:fs/promises';
import path from 'node:path';

import {
  ASSERTION_PROFILES,
  ISSUE_195_MANIFEST_SCHEMA_VERSION,
  ISSUE_195_SOURCES,
  buildIssue195Manifest,
} from './issue-195-requirements.mjs';
import { buildLanguageCatalog, formatLanguageCatalog } from './build-language-catalog.mjs';

export {
  ASSERTION_PROFILES,
  ISSUE_195_MANIFEST_SCHEMA_VERSION,
  ISSUE_195_SOURCES,
  buildIssue195Manifest,
};

function allKeys(value, prefix = '') {
  if (!value || typeof value !== 'object') return [];
  const keys = [];
  for (const [key, child] of Object.entries(value)) {
    const full = prefix ? `${prefix}.${key}` : key;
    keys.push(full, ...allKeys(child, full));
  }
  return keys;
}

export async function validateIssue195Manifest(manifest, root) {
  const errors = [];
  if (manifest.schemaVersion !== ISSUE_195_MANIFEST_SCHEMA_VERSION) {
    errors.push(`schemaVersion must be ${ISSUE_195_MANIFEST_SCHEMA_VERSION}`);
  }
  if (manifest.issue !== 195) errors.push('issue must be 195');
  const requirements = manifest.atomicRequirements;
  if (!Array.isArray(requirements) || requirements.length === 0) {
    errors.push('atomicRequirements must be a non-empty array');
    return errors;
  }

  const ids = new Set();
  const testIds = new Set();
  for (const entry of requirements) {
    if (typeof entry.id !== 'string' || !/^I195-[A-Za-z0-9-]+$/.test(entry.id)) {
      errors.push(`invalid stable requirement id: ${JSON.stringify(entry.id)}`);
      continue;
    }
    if (ids.has(entry.id)) errors.push(`duplicate requirement id: ${entry.id}`);
    ids.add(entry.id);
    if (!Object.values(ISSUE_195_SOURCES).includes(entry.source)) {
      errors.push(`${entry.id} has a non-authoritative source permalink`);
    }
    for (const field of ['area', 'scope', 'expectedBehavior', 'requiredRuntimes', 'implementationEntryPoints', 'verifications']) {
      if (entry[field] === undefined || entry[field] === null) {
        errors.push(`${entry.id} is missing ${field}`);
      }
    }
    for (const field of ['language', 'version', 'edition', 'construct', 'aliases', 'extensions']) {
      if (entry.scope?.[field] === undefined || entry.scope?.[field] === null) {
        errors.push(`${entry.id} scope is missing ${field}`);
      }
    }
    if (allKeys(entry).some((key) => /(^|\.)(status|complete|completed)$/i.test(key))) {
      errors.push(`${entry.id} must not contain hand-edited completion/status fields`);
    }
    if (!Array.isArray(entry.requiredRuntimes) || entry.requiredRuntimes.length === 0) {
      errors.push(`${entry.id} must name required runtimes`);
    }
    if (!Array.isArray(entry.verifications) || entry.verifications.length === 0) {
      errors.push(`${entry.id} must name stable verification cells`);
      continue;
    }
    for (const runtime of entry.requiredRuntimes ?? []) {
      if (!(runtime in (entry.implementationEntryPoints ?? {}))) {
        errors.push(`${entry.id} is missing the ${runtime} implementation entry-point cell`);
      }
      if (!entry.verifications.some((cell) => cell.runtime === runtime)) {
        errors.push(`${entry.id} has no verification for required runtime ${runtime}`);
      }
    }
    for (const cell of entry.verifications) {
      if (typeof cell.testId !== 'string' || cell.testId.length === 0) {
        errors.push(`${entry.id} has a verification without a stable testId`);
        continue;
      }
      if (testIds.has(cell.testId)) errors.push(`duplicate testId: ${cell.testId}`);
      testIds.add(cell.testId);
      if (!['positive', 'negative', 'fault-injection'].includes(cell.kind)) {
        errors.push(`${cell.testId} has invalid kind ${cell.kind}`);
      }
      if (!['pre-merge', 'release-delivery'].includes(cell.checkpoint)) {
        errors.push(`${cell.testId} has invalid checkpoint ${cell.checkpoint}`);
      }
      if (!Array.isArray(cell.fixtureIds) || cell.fixtureIds.length === 0) {
        errors.push(`${cell.testId} must name fixtures`);
      }
      for (const fixtureId of cell.fixtureIds ?? []) {
        if (!manifest.fixtureCatalog?.[fixtureId]) {
          errors.push(`${cell.testId} references unknown fixture ${fixtureId}`);
        }
      }
      if (!Array.isArray(cell.assertions) || cell.assertions.length === 0) {
        errors.push(`${cell.testId} must name observable assertions`);
      }
      if (
        typeof cell.evidenceArtifact !== 'string' ||
        !/^issue-195-results\/[a-z0-9._-]+\.json(?:#[a-z0-9-]+)?$/.test(cell.evidenceArtifact)
      ) {
        errors.push(`${cell.testId} must name a stable issue-195-results JSON evidence artifact`);
      }
    }
  }

  const expected = await buildIssue195Manifest(root);
  const expectedIds = new Set(expected.atomicRequirements.map((entry) => entry.id));
  for (const id of expectedIds) {
    if (!ids.has(id)) errors.push(`manifest is missing generated scope requirement ${id}`);
  }
  for (const id of ids) {
    if (!expectedIds.has(id)) errors.push(`manifest has dangling requirement ${id}`);
  }

  for (const entry of requirements) {
    for (const runtime of entry.requiredRuntimes ?? []) {
      const paths = entry.implementationEntryPoints?.[runtime];
      if (paths === null) continue;
      if (!Array.isArray(paths) || paths.length === 0) {
        errors.push(`${entry.id} ${runtime} entry points must be a non-empty path array or null`);
        continue;
      }
      for (const relativePath of paths) {
        try {
          await access(path.join(root, relativePath));
        } catch {
          errors.push(`${entry.id} ${runtime} entry point does not exist: ${relativePath}`);
        }
      }
    }
  }

  const inventory = JSON.parse(
    await readFile(path.join(root, 'parity', 'language-grammar-inventory.json'), 'utf8'),
  );
  const languageNames = new Set(inventory.languages.map(({ name }) => name));
  const extensionNames = new Set(Object.keys(inventory.extensionDispatch ?? {}));
  for (const name of languageNames) {
    if (!extensionNames.has(name)) errors.push(`extension dispatch metadata is missing ${name}`);
  }
  for (const name of extensionNames) {
    if (!languageNames.has(name)) errors.push(`extension dispatch metadata dangles: ${name}`);
  }
  const inventoryAliasOwners = new Map();
  for (const language of inventory.languages) {
    if (!Array.isArray(language.aliases) || language.aliases.length === 0) {
      errors.push(`inventory language ${language.name} has no aliases`);
    }
    for (const alias of language.aliases ?? []) {
      const normalized = alias.toLowerCase();
      const owner = inventoryAliasOwners.get(normalized);
      if (owner && owner !== language.name) {
        errors.push(`inventory alias ${alias} is shared by ${owner} and ${language.name}`);
      }
      inventoryAliasOwners.set(normalized, language.name);
    }
  }

  // Both runtimes dispatch names, aliases, extensions, and default grammars
  // from the same generated catalog, so it must equal the inventory exactly.
  const lock = JSON.parse(
    await readFile(path.join(root, 'js', 'src', 'vendor', 'grammars', 'grammar-lock.json'), 'utf8'),
  );
  let expectedCatalog;
  try {
    expectedCatalog = formatLanguageCatalog(buildLanguageCatalog(inventory, lock));
  } catch (error) {
    errors.push(`language catalog cannot be generated: ${error.message}`);
  }
  if (expectedCatalog) {
    for (const runtime of ['js', 'rust']) {
      const catalogPath = path.join(root, runtime, 'src', 'data', 'language-catalog.json');
      const shipped = await readFile(catalogPath, 'utf8').catch(() => undefined);
      if (shipped !== expectedCatalog) {
        errors.push(`${runtime} language catalog does not match the inventory`);
      }
    }
  }
  const catalogConsumers = [
    ['js/src/programming-language-parser.js', /from '\.\/language-catalog\.js'/],
    ['rust/src/tree_sitter_adapter.rs', /language_catalog::language_entry/],
    ['rust/src/language_parser.rs', /language_catalog::language_entry/],
  ];
  for (const [file, pattern] of catalogConsumers) {
    const source = await readFile(path.join(root, file), 'utf8');
    if (!pattern.test(source)) errors.push(`${file} does not dispatch through the language catalog`);
  }
  return errors;
}

function resultIndex(resultDocuments) {
  const byTestId = new Map();
  const errors = [];
  for (const document of resultDocuments) {
    if (!document || typeof document !== 'object') {
      errors.push('result document must be an object');
      continue;
    }
    if (document.schemaVersion !== 1) errors.push('result document schemaVersion must be 1');
    if (document.issue !== 195) errors.push('result document issue must be 195');
    if (!Array.isArray(document.results)) {
      errors.push('result document results must be an array');
      continue;
    }
    for (const result of document.results) {
      if (byTestId.has(result.testId)) errors.push(`duplicate result for ${result.testId}`);
      byTestId.set(result.testId, { ...result, document });
    }
  }
  return { byTestId, errors };
}

export function evaluateIssue195Acceptance(
  manifest,
  resultDocuments,
  { checkpoint = 'all', commit = 'WORKTREE' } = {},
) {
  const { byTestId, errors } = resultIndex(resultDocuments);
  const knownTestIds = new Set(
    manifest.atomicRequirements.flatMap((entry) => entry.verifications.map((cell) => cell.testId)),
  );
  for (const resultId of byTestId.keys()) {
    if (!knownTestIds.has(resultId)) errors.push(`unknown or dangling result testId ${resultId}`);
  }

  const requirementResults = [];
  for (const entry of manifest.atomicRequirements) {
    const cells = [];
    for (const cell of entry.verifications) {
      if (checkpoint !== 'all' && cell.checkpoint !== checkpoint) continue;
      const reasons = [];
      const implementation = entry.implementationEntryPoints[cell.runtime];
      if (implementation === null) reasons.push('implementation entry point is not supplied');
      for (const fixtureId of cell.fixtureIds) {
        const fixture = manifest.fixtureCatalog[fixtureId];
        if (!fixture?.path || !fixture?.sha256) {
          reasons.push(`fixture ${fixtureId} is planned but has no pinned corpus artifact`);
        }
      }
      const observed = byTestId.get(cell.testId);
      if (!observed) {
        reasons.push('required result is missing');
      } else {
        if (!observed.document.producer) reasons.push('result producer is missing');
        if (!observed.document.generatedAt) reasons.push('result generation time is missing');
        if (observed.document.commit !== commit) {
          reasons.push(`result commit ${observed.document.commit ?? '<missing>'} does not match ${commit}`);
        }
        if (observed.outcome !== 'passed') reasons.push(`outcome is ${observed.outcome ?? '<missing>'}`);
        if (observed.kind !== cell.kind) {
          reasons.push(`result kind ${observed.kind ?? '<missing>'} does not match ${cell.kind}`);
        }
        if (observed.positiveEvidence !== true) reasons.push('positiveEvidence is not true');
        if (!observed.command) reasons.push('reproducible command is missing');
        if (!observed.toolchainVersions || Object.keys(observed.toolchainVersions).length === 0) {
          reasons.push('toolchain versions are missing');
        }
        if (!observed.grammarVersions || Object.keys(observed.grammarVersions).length === 0) {
          reasons.push('grammar/component versions are missing');
        }
        if (!Array.isArray(observed.evidenceArtifacts) || observed.evidenceArtifacts.length === 0) {
          reasons.push('evidence artifacts are missing');
        }
        if (
          cell.kind === 'fault-injection' &&
          (!Array.isArray(observed.failureLogs) || observed.failureLogs.length === 0)
        ) {
          reasons.push('fault-injection failure log is missing');
        }
        const records = observed.executionRecords;
        const executed = new Set();
        const seenRecords = new Set();
        if (!Array.isArray(records) || records.length === 0) {
          reasons.push('execution record is missing');
        } else {
          for (const record of records) {
            if (!record || typeof record !== 'object') {
              reasons.push('execution record is not an object');
              continue;
            }
            const key = `${record.assertionId}\u0000${record.fixtureId}`;
            if (seenRecords.has(key)) reasons.push(`duplicate execution record: ${key}`);
            seenRecords.add(key);
            if (record.testId !== cell.testId) reasons.push(`execution record testId mismatch: ${record.testId}`);
            if (record.runtime !== cell.runtime) reasons.push(`execution record runtime mismatch: ${record.runtime}`);
            if (record.commit !== commit) reasons.push(`execution record commit mismatch: ${record.commit}`);
            if (record.outcome !== 'passed') reasons.push(`execution record outcome is ${record.outcome ?? '<missing>'}`);
            if (!record.testName) reasons.push('execution record test callback name is missing');
            if (!cell.assertions.includes(record.assertionId)) reasons.push(`unexpected execution assertion: ${record.assertionId}`);
            if (!cell.fixtureIds.includes(record.fixtureId)) reasons.push(`unexpected execution fixture: ${record.fixtureId}`);
            if (record.fixtureDigest !== manifest.fixtureCatalog[record.fixtureId]?.sha256) {
              reasons.push(`execution record fixture digest mismatch for ${record.fixtureId}`);
            }
            if (record.outcome === 'passed') executed.add(key);
          }
        }
        for (const assertion of cell.assertions) {
          for (const fixtureId of cell.fixtureIds) {
            if (!executed.has(`${assertion}\u0000${fixtureId}`)) {
              reasons.push(`observable assertion did not execute: ${assertion} on ${fixtureId}`);
            }
          }
        }
        const recordedAssertions = cell.assertions.filter((assertion) =>
          cell.fixtureIds.every((fixtureId) => executed.has(`${assertion}\u0000${fixtureId}`))
        );
        if (
          JSON.stringify([...(observed.assertionsPassed ?? [])].sort()) !==
          JSON.stringify([...recordedAssertions].sort())
        ) {
          reasons.push('assertionsPassed does not match execution records');
        }
        for (const fixtureId of cell.fixtureIds) {
          const expectedDigest = manifest.fixtureCatalog[fixtureId]?.sha256;
          if (observed.fixtureDigests?.[fixtureId] !== expectedDigest) {
            reasons.push(`fixture digest mismatch for ${fixtureId}`);
          }
        }
      }
      cells.push({
        testId: cell.testId,
        runtime: cell.runtime,
        checkpoint: cell.checkpoint,
        passed: reasons.length === 0,
        reasons,
        evidence: observed
          ? {
              producer: observed.document.producer ?? null,
              generatedAt: observed.document.generatedAt ?? null,
              outcome: observed.outcome ?? null,
              kind: observed.kind ?? null,
              positiveEvidence: observed.positiveEvidence === true,
              command: observed.command ?? null,
              toolchainVersions: observed.toolchainVersions ?? null,
              grammarVersions: observed.grammarVersions ?? null,
              evidenceArtifacts: observed.evidenceArtifacts ?? null,
              failureLogs: observed.failureLogs ?? null,
              assertionsPassed: observed.assertionsPassed ?? null,
              executionRecords: observed.executionRecords ?? null,
              fixtureDigests: observed.fixtureDigests ?? null,
            }
          : null,
      });
    }
    if (cells.length > 0) {
      requirementResults.push({
        id: entry.id,
        area: entry.area,
        passed: cells.every((cell) => cell.passed),
        cells,
      });
    }
  }
  const passed = requirementResults.filter((entry) => entry.passed).length;
  return {
    schemaVersion: 1,
    issue: 195,
    commit,
    checkpoint,
    generatedAt: new Date().toISOString(),
    summary: {
      requirements: requirementResults.length,
      passed,
      failed: requirementResults.length - passed,
      verificationCells: requirementResults.reduce((sum, entry) => sum + entry.cells.length, 0),
      gateErrors: errors.length,
    },
    passed: errors.length === 0 && requirementResults.length > 0 && passed === requirementResults.length,
    gateErrors: errors,
    requirements: requirementResults,
  };
}

export function compareScopeBaseline(baseline, candidate) {
  const errors = [];
  const baselineRequirements = new Map(
    baseline.atomicRequirements.map((entry) => [entry.id, entry]),
  );
  const candidateRequirements = new Map(
    candidate.atomicRequirements.map((entry) => [entry.id, entry]),
  );
  for (const [id, oldEntry] of baselineRequirements) {
    const newEntry = candidateRequirements.get(id);
    if (!newEntry) {
      errors.push(`required scope row removed: ${id}`);
      continue;
    }
    for (const field of ['source', 'area', 'expectedBehavior']) {
      if (newEntry[field] !== oldEntry[field]) {
        errors.push(`protected requirement field changed for ${id}: ${field}`);
      }
    }
    for (const field of ['language', 'version', 'edition', 'construct']) {
      if (newEntry.scope?.[field] !== oldEntry.scope?.[field]) {
        errors.push(`protected scope field changed for ${id}: ${field}`);
      }
    }
    for (const field of ['aliases', 'extensions']) {
      const newValues = new Set(newEntry.scope?.[field] ?? []);
      for (const value of oldEntry.scope?.[field] ?? []) {
        if (!newValues.has(value)) {
          const label = field === 'aliases' ? 'alias' : 'extension';
          errors.push(`required ${label} removed from ${id}: ${value}`);
        }
      }
    }
    const newRuntimes = new Set(newEntry.requiredRuntimes ?? []);
    for (const runtime of oldEntry.requiredRuntimes ?? []) {
      if (!newRuntimes.has(runtime)) {
        errors.push(`required runtime removed from ${id}: ${runtime}`);
      }
    }
    const oldCells = new Map(oldEntry.verifications.map((cell) => [cell.testId, cell]));
    const newCells = new Map(newEntry.verifications.map((cell) => [cell.testId, cell]));
    for (const [testId, oldCell] of oldCells) {
      const newCell = newCells.get(testId);
      if (!newCell) {
        errors.push(`required verification removed: ${testId}`);
        continue;
      }
      for (const field of ['runtime', 'kind', 'checkpoint', 'evidenceArtifact']) {
        if (newCell[field] !== oldCell[field]) {
          errors.push(`protected verification field changed for ${testId}: ${field}`);
        }
      }
      for (const assertion of oldCell.assertions) {
        if (!newCell.assertions.includes(assertion)) {
          errors.push(`required assertion removed from ${testId}: ${assertion}`);
        }
      }
      for (const fixtureId of oldCell.fixtureIds) {
        if (!newCell.fixtureIds.includes(fixtureId)) {
          errors.push(`required fixture removed from ${testId}: ${fixtureId}`);
        }
      }
    }
  }
  return errors;
}

function syntheticFaultCase(assertions) {
  const manifest = {
    fixtureCatalog: {
      fixture: { path: 'fixture.json', selector: '$', sha256: 'fixture-digest' },
    },
    atomicRequirements: [
      {
        id: 'I195-FAULT-PROBE',
        area: 'fault-probe',
        scope: { aliases: ['required-alias'], extensions: ['.required'] },
        requiredRuntimes: ['aggregate'],
        implementationEntryPoints: { aggregate: ['gate.js'] },
        verifications: [
          {
            testId: 'i195-fault-probe',
            runtime: 'aggregate',
            kind: 'positive',
            checkpoint: 'pre-merge',
            fixtureIds: ['fixture'],
            assertions,
          },
        ],
      },
    ],
  };
  const document = {
    schemaVersion: 1,
    issue: 195,
    commit: 'fault-candidate',
    producer: 'issue-195-fault-probe',
    generatedAt: new Date(0).toISOString(),
    results: [
      {
        testId: 'i195-fault-probe',
        outcome: 'passed',
        kind: 'positive',
        positiveEvidence: true,
        command: 'fault probe',
        toolchainVersions: { node: process.version },
        grammarVersions: { acceptanceManifest: 'schema-1' },
        evidenceArtifacts: ['fault-result.json'],
        failureLogs: [],
        assertionsPassed: assertions,
        executionRecords: assertions.map((assertionId) => ({
          testId: 'i195-fault-probe', assertionId, fixtureId: 'fixture',
          fixtureDigest: 'fixture-digest', runtime: 'aggregate',
          commit: 'fault-candidate', outcome: 'passed', testName: 'fault probe',
        })),
        fixtureDigests: { fixture: 'fixture-digest' },
      },
    ],
  };
  return { manifest, document };
}

function faultIsRejected(fault) {
  if (fault === 'removed-inventory-row-or-alias') {
    const { manifest } = syntheticFaultCase(['scopePreserved']);
    const candidate = structuredClone(manifest);
    candidate.atomicRequirements[0].scope.aliases = [];
    const failureLogs = compareScopeBaseline(manifest, candidate);
    const assertionsPassed = [
      'faultActivated',
      ...(failureLogs.length > 0 ? ['expectedRequirementFailed', 'aggregateFailed', 'failureReasonRecorded'] : []),
    ];
    return { detected: assertionsPassed.length === 4, failureLogs, assertionsPassed };
  }

  const profile =
    fault === 'stubbed-binding-resolution' || fault === 'rename-capture-bug'
      ? ASSERTION_PROFILES.bindingRename
      : fault === 'stale-structure-after-edit'
        ? ASSERTION_PROFILES.transformPositive
        : fault === 'unsupported-translation-descriptor' ||
            fault === 'relabeled-source-translation'
          ? ASSERTION_PROFILES.translationPositive
          : ASSERTION_PROFILES.cstPositive;
  const { manifest, document } = syntheticFaultCase(profile);
  const baseline = evaluateIssue195Acceptance(manifest, [document], {
    checkpoint: 'pre-merge', commit: 'fault-candidate',
  });
  if (!baseline.passed) throw new Error(`fault probe baseline failed before mutation: ${fault}`);
  function omitAssertions(predicate) {
    document.results[0].assertionsPassed = profile.filter(predicate);
    document.results[0].executionRecords = document.results[0].executionRecords.filter(
      ({ assertionId }) => predicate(assertionId),
    );
  }
  if (fault === 'plain-text-parser-fallback') {
    omitAssertions(
      (assertion) => assertion !== 'realGrammarNodes',
    );
  } else if (fault === 'lexical-parser-fallback') {
    omitAssertions(
      (assertion) => assertion !== 'hierarchy',
    );
  } else if (fault === 'dropped-fields-trivia-or-spans') {
    omitAssertions(
      (assertion) => !['namedFields', 'commentsAndTrivia', 'exactUtf8Spans'].includes(assertion),
    );
  } else if (fault === 'stubbed-binding-resolution') {
    omitAssertions(
      (assertion) => assertion !== 'symbolIdentity',
    );
  } else if (fault === 'rename-capture-bug') {
    omitAssertions(
      (assertion) => assertion !== 'captureAvoidance',
    );
  } else if (fault === 'stale-structure-after-edit') {
    omitAssertions(
      (assertion) => assertion !== 'treeIntegrity',
    );
  } else if (fault === 'unsupported-translation-descriptor') {
    document.results[0].outcome = 'unsupported';
  } else if (fault === 'relabeled-source-translation') {
    omitAssertions(
      (assertion) => assertion !== 'noSourceRelabelling',
    );
  } else if (fault === 'skipped-test-or-job') {
    document.results = [];
  } else if (fault === 'falsified-capability-declaration') {
    manifest.atomicRequirements[0].status = 'complete';
    document.results = [];
  } else {
    throw new Error(`unknown issue 195 fault injection: ${fault}`);
  }
  const report = evaluateIssue195Acceptance(manifest, [document], {
    checkpoint: 'pre-merge',
    commit: 'fault-candidate',
  });
  const failureLogs = [
    ...report.gateErrors,
    ...report.requirements.flatMap(({ cells }) =>
      cells.flatMap(({ testId, reasons }) => reasons.map((reason) => `${testId}: ${reason}`)),
    ),
  ];
  const assertionsPassed = [
    'faultActivated',
    ...(!report.requirements[0]?.passed ? ['expectedRequirementFailed'] : []),
    ...(!report.passed ? ['aggregateFailed'] : []),
    ...(failureLogs.length > 0 ? ['failureReasonRecorded'] : []),
  ];
  return { detected: assertionsPassed.length === 4, failureLogs, assertionsPassed };
}

export function runIssue195GateFaultInjections(manifest, commit) {
  const results = manifest.atomicRequirements
    .filter(({ area }) => area === 'acceptance-gate-fault-injection')
    .map((entry) => {
      const fault = entry.scope.construct;
      const cell = entry.verifications[0];
      const { detected, failureLogs, assertionsPassed } = faultIsRejected(fault);
      return {
        testId: cell.testId,
        outcome: detected ? 'passed' : 'failed',
        kind: cell.kind,
        positiveEvidence: detected,
        command:
          'node js/scripts/check-issue-195-acceptance.mjs --produce-gate-results issue-195-results/gate-faults.json',
        toolchainVersions: { node: process.version },
        grammarVersions: { acceptanceManifest: `schema-${manifest.schemaVersion}` },
        evidenceArtifacts: [
          'js/tests/issue-195-acceptance.test.js',
          'js/scripts/issue-195-acceptance-lib.mjs',
        ],
        failureLogs,
        assertionsPassed,
        executionRecords: assertionsPassed.flatMap((assertionId) =>
          cell.fixtureIds.map((fixtureId) => ({
            testId: cell.testId, assertionId, fixtureId,
            fixtureDigest: manifest.fixtureCatalog[fixtureId].sha256,
            runtime: cell.runtime, commit, outcome: 'passed',
            testName: `fault mutation: ${fault}`,
          }))),
        fixtureDigests: Object.fromEntries(
          cell.fixtureIds.map((fixtureId) => [fixtureId, manifest.fixtureCatalog[fixtureId].sha256]),
        ),
      };
    });
  return {
    schemaVersion: 1,
    issue: 195,
    commit,
    producer: 'issue-195-acceptance-gate-fault-injections',
    generatedAt: new Date().toISOString(),
    results,
  };
}

export function renderIssue195Markdown(manifest, report) {
  const lines = [
    '<!-- Generated by js/scripts/check-issue-195-acceptance.mjs; do not edit by hand. -->',
    '# Issue 195 executable requirement ledger',
    '',
    `Commit: \`${report.commit}\``,
    '',
    `Checkpoint: \`${report.checkpoint}\``,
    '',
    `Generated: ${report.generatedAt}`,
    '',
    `Acceptance result: **${report.passed ? 'PASS' : 'FAIL'}** — ${report.summary.passed}/${report.summary.requirements} atomic requirements passed across ${report.summary.verificationCells} verification cells.`,
    '',
    'This report is computed from the atomic manifest and commit-bound test-result artifacts. Parser/capability declarations, unsupported descriptors, planned fixtures, skipped tests, and hand-edited labels are not execution evidence.',
    '',
    '| Requirement | Area | Result | Failed verification |',
    '| --- | --- | --- | --- |',
  ];
  for (const result of report.requirements) {
    const failedCells = result.cells.filter((cell) => !cell.passed);
    const reason = failedCells.length
      ? failedCells
          .map((cell) => `\`${cell.testId}\`: ${cell.reasons.join('; ')}`)
          .join('<br>')
      : 'All required evidence passed.';
    lines.push(`| \`${result.id}\` | ${result.area} | ${result.passed ? 'PASS' : 'FAIL'} | ${reason} |`);
  }
  lines.push(
    '',
    '## Authoritative sources',
    '',
    `- [Issue #195](${manifest.sources.issue})`,
    `- [Full-support clarification](${manifest.sources.clarification})`,
    `- [Executable acceptance-gate clarification](${manifest.sources.acceptanceGate})`,
    '',
    '## Enforcement status',
    '',
    'The dedicated workflow executes this evaluator and fails closed. Repository branch protection is a separate maintainer-controlled setting; the workflow does not claim to be a required check until protection names it.',
  );
  return lines.join('\n');
}
