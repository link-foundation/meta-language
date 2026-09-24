import { access, readFile } from 'node:fs/promises';
import path from 'node:path';

import {
  ASSERTION_PROFILES,
  ISSUE_195_MANIFEST_SCHEMA_VERSION,
  ISSUE_195_SOURCES,
  buildIssue195Manifest,
} from './issue-195-requirements.mjs';

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

  const jsParserSource = await readFile(
    path.join(root, 'js', 'src', 'programming-language-parser.js'),
    'utf8',
  );
  const aliasBlock = jsParserSource.match(
    /const LANGUAGE_ALIASES = new Map\(\[([\s\S]*?)\]\);/,
  )?.[1];
  if (!aliasBlock) {
    errors.push('could not enumerate JavaScript grammar alias registry');
  } else {
    const actual = new Set(
      [...aliasBlock.matchAll(/\['([^']+)',\s*'[^']+'\]/g)].map((match) =>
        match[1].toLowerCase(),
      ),
    );
    const expectedAliases = new Set(
      inventory.languages
        .filter(({ javascript }) => javascript.status === 'grammar-cst')
        .flatMap(({ aliases }) => aliases.map((alias) => alias.toLowerCase())),
    );
    for (const alias of expectedAliases) {
      if (!actual.has(alias)) errors.push(`JavaScript grammar registry is missing alias ${alias}`);
    }
    for (const alias of actual) {
      if (!expectedAliases.has(alias)) {
        errors.push(`JavaScript grammar registry alias is absent from inventory: ${alias}`);
      }
    }
  }

  const rustParserSource = await readFile(
    path.join(root, 'rust', 'src', 'tree_sitter_adapter.rs'),
    'utf8',
  );
  const rustBuiltInParserSource = await readFile(
    path.join(root, 'rust', 'src', 'language_parser.rs'),
    'utf8',
  );
  const rustAliasBlock = rustParserSource.match(
    /fn grammar_for_language[\s\S]*?\n}\n\nfn convert_node/,
  )?.[0];
  if (!rustAliasBlock) {
    errors.push('could not enumerate Rust grammar alias registry');
  } else {
    const actual = new Set(
      [...rustAliasBlock.matchAll(/"([^"]+)"/g)].map((match) => match[1].toLowerCase()),
    );
    const builtInAliasBlock = rustBuiltInParserSource.match(
      /const BUILT_IN_GRAMMAR_ALIASES[^=]*=\s*&\[([\s\S]*?)\];/,
    )?.[1];
    if (!builtInAliasBlock) {
      errors.push('could not enumerate Rust built-in grammar alias registry');
    } else {
      for (const match of builtInAliasBlock.matchAll(/"([^"]+)"/g)) {
        actual.add(match[1].toLowerCase());
      }
    }
    const expectedAliases = new Set(
      inventory.languages
        .filter(({ rust }) => rust.status === 'grammar-cst')
        .flatMap(({ aliases }) => aliases.map((alias) => alias.toLowerCase())),
    );
    for (const alias of expectedAliases) {
      if (!actual.has(alias)) errors.push(`Rust grammar registry is missing alias ${alias}`);
    }
    for (const alias of actual) {
      if (!expectedAliases.has(alias)) {
        errors.push(`Rust grammar registry alias is absent from inventory: ${alias}`);
      }
    }
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
        const assertions = new Set(observed.assertionsPassed ?? []);
        for (const assertion of cell.assertions) {
          if (!assertions.has(assertion)) reasons.push(`observable assertion did not pass: ${assertion}`);
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
    return { detected: failureLogs.length > 0, failureLogs };
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
  if (fault === 'plain-text-parser-fallback') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'realGrammarNodes',
    );
  } else if (fault === 'lexical-parser-fallback') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'hierarchy',
    );
  } else if (fault === 'dropped-fields-trivia-or-spans') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => !['namedFields', 'commentsAndTrivia', 'exactUtf8Spans'].includes(assertion),
    );
  } else if (fault === 'stubbed-binding-resolution') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'symbolIdentity',
    );
  } else if (fault === 'rename-capture-bug') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'captureAvoidance',
    );
  } else if (fault === 'stale-structure-after-edit') {
    document.results[0].assertionsPassed = profile.filter(
      (assertion) => assertion !== 'treeIntegrity',
    );
  } else if (fault === 'unsupported-translation-descriptor') {
    document.results[0].outcome = 'unsupported';
  } else if (fault === 'relabeled-source-translation') {
    document.results[0].assertionsPassed = profile.filter(
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
  return { detected: !report.passed, failureLogs };
}

export function runIssue195GateFaultInjections(manifest, commit) {
  const results = manifest.atomicRequirements
    .filter(({ area }) => area === 'acceptance-gate-fault-injection')
    .map((entry) => {
      const fault = entry.scope.construct;
      const cell = entry.verifications[0];
      const { detected, failureLogs } = faultIsRejected(fault);
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
        assertionsPassed: detected ? cell.assertions : [],
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
