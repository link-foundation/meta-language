// docs/vision.md is the one authoritative specification: every directive
// section is specified, the entry documents link to it, the subsystem
// documents defer to it, and every ledger row traces back to one of its
// sections and forward to its inventories, sources, CI and packages.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { test } from 'node:test';

import { DOWNSTREAM_CONSUMERS } from '../scripts/issue-195-downstream.mjs';
import { evidenceGroupFor } from '../scripts/issue-195-evidence-plan.mjs';
import {
  AREA_SPECIFICATIONS,
  VISION_REQUIREMENTS,
  VISION_SPECIFICATION,
} from '../scripts/issue-195-vision-requirements.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const repositoryFile = (relative) => new URL(`../../${relative}`, import.meta.url);
const read = (relative) => readFileSync(repositoryFile(relative), 'utf8');
const vision = read(VISION_SPECIFICATION);
const manifest = JSON.parse(read('parity/issue-195-requirements.json'));
const register = JSON.parse(read('parity/issue-195-sources.json'));

// GitHub's heading anchors: lowercase, punctuation other than hyphens dropped,
// spaces turned into hyphens.
function headingAnchors(markdown) {
  return new Set(markdown.split('\n')
    .filter((line) => /^#{1,6} /u.test(line))
    .map((line) => line.replace(/^#+ /u, '').trim().toLowerCase()
      .replace(/[^\p{L}\p{N} -]/gu, '').replace(/ /gu, '-')));
}
const anchors = headingAnchors(vision);

// The sections the repository-wide directive (comment 5885245090) requires.
const DIRECTIVE_SECTIONS = [
  'sources-of-truth',
  'one-common-language-of-links',
  'links-notation-fitness',
  'native-merged-grammars',
  'grammar-feature-union',
  'concrete-and-abstract-syntax-trees',
  'language-catalog-and-default-parsing',
  'grammar-import-conversion-and-reverse-conversion',
  'automatic-merging-concept-recognition-deduplication-and-renaming',
  'readable-english-names',
  'four-language-semantics-and-translation',
  'structured-transformation-and-binding-safety',
  'foundation-neutrality',
  'downstream-consumers',
  'dependencies',
  'acceptance-and-evidence',
  'what-finite-tests-establish',
  'cache-cleanup',
  'packages-and-publication',
  'documentation-consistency',
  'current-state',
];

function observe(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: VISION_SPECIFICATION,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

test('docs/vision.md specifies every section of the repository-wide directive', () => {
  assert.match(vision, /^# Meta-language vision and architecture$/mu);
  assert.match(vision, /This document describes the \*\*target\*\*/u, 'the vision separates the target from the current state');
  for (const section of DIRECTIVE_SECTIONS) assert.ok(anchors.has(section), `docs/vision.md has a ${section} section`);
  for (const row of VISION_REQUIREMENTS) assert.ok(anchors.has(row.specification), `${row.id} cites an existing section`);
  for (const [area, section] of Object.entries(AREA_SPECIFICATIONS)) {
    assert.ok(anchors.has(section), `the ${area} area cites an existing section`);
  }
  // Every relative link of the vision resolves.
  for (const [, target] of vision.matchAll(/\]\((?!https?:|#)([^)#]+)(?:#[^)]*)?\)/gu)) {
    assert.ok(existsSync(new URL(target, repositoryFile(VISION_SPECIFICATION))), `docs/vision.md links to existing ${target}`);
  }
  observe('I195-VISION-SPECIFICATION', ['specificationCommitted', 'everyDirectiveSectionSpecified'],
    'docs/vision.md specifies every section of the repository-wide directive');
});

test('the entry documents link to the vision and the subsystem documents defer to it', () => {
  assert.match(read('README.md'), /\[`docs\/vision\.md`\]\(docs\/vision\.md\) is the authoritative specification/u);
  assert.match(read('CONTRIBUTING.md'), /\[`docs\/vision\.md`\]\(docs\/vision\.md\) is the authoritative vision/u);
  assert.match(read('AGENTS.md'), /\[`docs\/vision\.md`\]\(docs\/vision\.md\) is the authoritative vision/u);
  for (const [document, link] of [
    ['docs/grammar/architecture.md', '../vision.md'],
    ['docs/grammar/README.md', '../vision.md'],
    ['docs/parity-roadmap.md', 'vision.md'],
  ]) {
    assert.ok(read(document).includes(`> This document is subordinate to the authoritative\n> [vision and architecture specification](${link})`),
      `${document} defers to the vision`);
  }
  observe('I195-VISION-SPECIFICATION', [
    'linkedFromReadme',
    'linkedFromContributorInstructions',
    'linkedFromAgentInstructions',
    'subordinateDocumentsDeferToSpecification',
  ], 'the entry documents link to the vision and the subsystem documents defer to it');
});

test('docs/vision.md states what finite tests establish', () => {
  const section = vision.split('## What finite tests establish')[1].split('\n## ')[0];
  assert.match(section, /need proofs or\s+decision procedures/u);
  assert.match(section, /never present a finite sample as a universal result/u);
  observe('I195-ACCEPTANCE-FINITE-CLAIMS-DOCUMENTED', ['finiteEvidenceLimitsDocumented'],
    'docs/vision.md states what finite tests establish');
});

test('every ledger row traces to its specification, inventories, sources, CI and packages', () => {
  const sourceKeyByUrl = new Map(register.requirementSources.map(({ key, url }) => [url, key]));
  const registeredKeys = new Set(sourceKeyByUrl.values());
  const packageFor = { javascript: 'npm:meta-language', rust: 'crates.io:meta-language', tooling: 'repository tooling, not packaged', aggregate: 'acceptance gate, not packaged' };
  for (const row of manifest.atomicRequirements) {
    const { traceability } = row;
    assert.ok(traceability, `${row.id} has a traceability record`);
    const [file, anchor] = traceability.specification.split('#');
    assert.equal(file, VISION_SPECIFICATION);
    assert.ok(anchors.has(anchor), `${row.id} traces to the existing section ${anchor}`);
    assert.ok(traceability.inventories.length > 0, `${row.id} names its inventories`);
    for (const inventory of traceability.inventories) assert.ok(existsSync(repositoryFile(inventory)), `${row.id} inventory ${inventory} exists`);
    assert.ok(traceability.sources.includes(sourceKeyByUrl.get(row.source)), `${row.id} is covered by its own source`);
    for (const key of traceability.sources) assert.ok(registeredKeys.has(key), `${row.id} source ${key} is registered`);
    assert.ok(existsSync(repositoryFile(traceability.workflow)), `${row.id} workflow exists`);
    assert.deepEqual(traceability.evidenceGroups,
      [...new Set(row.verifications.map((cell) => evidenceGroupFor(row, cell)))].sort(), `${row.id} evidence groups`);
    assert.deepEqual(traceability.packages, row.requiredRuntimes.map((runtime) => packageFor[runtime]), `${row.id} packages`);
  }
  observe('I195-VISION-REQUIREMENT-TRACEABILITY', [
    'everyRowHasTraceability',
    'specificationAnchorsResolve',
    'inventoriesExist',
    'coveringSourcesRegistered',
    'workflowsAndEvidenceGroupsResolve',
    'packagesMatchRuntimes',
  ], 'every ledger row traces to its specification, inventories, sources, CI and packages');
  // The consumer matrix (I195-DOWNSTREAM-CONSUMER-MATRIX) names the consumers
  // whose usage each row covers; a row no consumer uses needs no downstream
  // evidence.
  const consumers = Object.values(DOWNSTREAM_CONSUMERS);
  for (const { id, traceability } of manifest.atomicRequirements) {
    assert.ok(Array.isArray(traceability.downstream), `${id} is mapped by the consumer matrix`);
    for (const consumer of traceability.downstream) assert.ok(consumers.includes(consumer), `${id}: ${consumer}`);
  }
  observe('I195-VISION-REQUIREMENT-TRACEABILITY', ['downstreamEvidenceLinked'],
    'every ledger row traces to its downstream evidence');
});
