import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { buildMetaTheoryFoundationSources, checkMetaTheoryFoundationSources, observeMetaTheoryFoundationModels } from '../scripts/meta-theory-foundation-sources.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const file = path => new URL(`../../${path}`, import.meta.url);
const audit = readProjectData(file('parity/associative-stack-audit.lino'));
const snapshots = new Map(audit.guides.flatMap(guide => [guide.commitVerification.metadataFile, ...guide.files.map(source => source.sourceFile)])
  .map(path => [path, readFileSync(file(path))]));
const theory = audit.theorySourceAudit;
const paths = new Set([...theory.adoptedDefinitions, ...theory.premises].flatMap(value => value.implementations.map(({ path }) => path))
  .concat(theory.sharedModelSources.map(({ path }) => path)));
const projectSources = new Map([...paths].map(path => [path, readFileSync(file(path))]));

test('the pinned meta-theory audit distinguishes archived and draft definitions and traces actual structural models and premises', () => {
  assert.deepEqual(checkMetaTheoryFoundationSources(audit, snapshots, projectSources), []);
  assert.deepEqual(buildMetaTheoryFoundationSources(audit, snapshots, projectSources), theory);
  assert.equal(theory.revision, 'd5923a7d0996ab4abf70bf03e9f212e3d1a8e6cc');
  assert.deepEqual(theory.sourceVersions.map(({ path, version, stage }) => [path, version, stage]), [
    ['archive/0.0.2/article.md', '0.0.2', 'archived release'],
    ['drafts/0.0.3/article/04-mathematical-introduction.md', '0.0.3', 'draft'],
    ['drafts/0.0.3/article/07-meta-theory.md', '0.0.3', 'draft'],
  ]);
  assert.equal(new Set(theory.sourceVersions.map(({ source }) => source.revision)).size, 1, 'document stages differ at the same pinned repository revision');
  assert.deepEqual(theory.adoptedDefinitions.map(({ id }) => id), ['link reference', 'ordered recursive link tuple', 'reference to tuple network', 'self referring point']);
  assert.deepEqual(theory.premises.map(({ id }) => id), ['finite operational references', 'point arity is explicit', 'bounded structural evidence']);
  for (const definition of theory.adoptedDefinitions) {
    assert.ok(definition.sources.length > 0);
    assert.deepEqual(definition.implementations.map(({ path }) => path.split('/')[0]), ['js', 'rust']);
    for (const premise of definition.premises) assert.ok(theory.premises.some(({ id }) => id === premise));
  }
  assert.deepEqual(observeMetaTheoryFoundationModels(), {
    referenceRoundTrip: 13,
    arities: [0, 1, 2, 3],
    orderedRepeatedReferences: [10, 11, 10],
    recursiveReferences: [[21], [20]],
    sourceFreeReferences: true,
    distinctReferenceMappings: true,
    internedPointReferences: [true],
    sameTermReusesReference: true,
    explicitDoubletReferences: [30, 30],
  });
  assert.equal(theory.evidenceBoundary.nativeExecution, false);
  assert.equal(theory.evidenceBoundary.upstreamProofExecution, false);
  assert.equal(theory.evidenceBoundary.projectWideApplicationEvidence, false);
  assert.deepEqual(theory.retainedGaps.map(({ id }) => id), ['faithful doublet point projection', 'upstream proof certification', 'project wide formal application']);
  recordIssue195Observations({
    requirementId: 'I195-THEORY-FOUNDATION-PINNED-SOURCE-AUDIT', runtime: 'tooling', suffix: 'behavior',
    fixtureId: 'conversation-contract:associativeFoundationRequest',
    fixtureFile: 'parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt',
    assertions: ['metaTheorySourceReadAndPinned', 'archivesAndDraftsDistinguished', 'adoptedDefinitionsAndAssumptionsTraced'],
    testName: 'the pinned meta-theory audit distinguishes archived and draft definitions and traces actual structural models and premises',
  });
});

test('theory source audit refuses wrong stages, quotes, revisions, hashes and declaration-only adoption', () => {
  const edits = [
    value => { delete value.theorySourceAudit; },
    value => { value.theorySourceAudit.revision = 'main'; },
    value => { value.theorySourceAudit.sourceVersions[0].stage = 'draft'; },
    value => { value.theorySourceAudit.sourceVersions[1].stage = 'archived release'; },
    value => { value.theorySourceAudit.sourceVersions[1].version = '0.0.2'; },
    value => { value.theorySourceAudit.sourceVersions.pop(); },
    value => { value.theorySourceAudit.sourceVersions[0].source.revision = '0'.repeat(40); },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources[0].url = 'https://github.com/link-foundation/meta-theory/blob/main/archive/0.0.2/article.md'; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources[0].sha256 = '0'.repeat(64); },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources[0].startLine += 1; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources[0].excerpt = 'a repository citation is the entire foundation'; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources[0].excerptSha256 = '0'.repeat(64); },
    value => { value.theorySourceAudit.adoptedDefinitions[0].sources = []; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].implementations = []; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].implementations.pop(); },
    value => { value.theorySourceAudit.adoptedDefinitions[0].implementations[0].path = 'docs/vision.md'; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].observations = []; },
    value => { value.theorySourceAudit.adoptedDefinitions.pop(); },
    value => { value.theorySourceAudit.adoptedDefinitions[0].model = 'universal proof authority'; },
    value => { value.theorySourceAudit.adoptedDefinitions[0].premises = []; },
    value => { value.theorySourceAudit.premises.pop(); },
    value => { value.theorySourceAudit.premises[0].sources = []; },
    value => { value.theorySourceAudit.premises[0].sharedModels = ['universe-model.Type:Type']; },
    value => { value.theorySourceAudit.sharedModelSources[0].sha256 = '0'.repeat(64); },
    value => { value.theorySourceAudit.retainedGaps = []; },
    value => { value.theorySourceAudit.evidenceBoundary.nativeExecution = true; },
    value => { value.theorySourceAudit.evidenceBoundary.upstreamProofExecution = true; },
    value => { value.theorySourceAudit.evidenceBoundary.projectWideApplicationEvidence = true; },
  ];
  for (const edit of edits) {
    const changed = structuredClone(audit);
    edit(changed);
    assert.ok(checkMetaTheoryFoundationSources(changed, snapshots, projectSources).length > 0, edit.toString());
  }
});

test('theory traces require the exact captured upstream bytes and actual shared implementation and model sources', () => {
  const guide = audit.guides.find(guide => guide.repository === 'link-foundation/meta-theory');
  for (const source of guide.files) {
    const changed = new Map(snapshots);
    changed.set(source.sourceFile, Buffer.concat([snapshots.get(source.sourceFile), Buffer.from('\n')]));
    assert.match(checkMetaTheoryFoundationSources(audit, changed, projectSources).join('\n'), /hash|provenance/u);
    changed.delete(source.sourceFile);
    assert.match(checkMetaTheoryFoundationSources(audit, changed, projectSources).join('\n'), /missing/u);
  }
  for (const path of paths) {
    const changed = new Map(projectSources);
    changed.set(path, Buffer.concat([changed.get(path), Buffer.from('\n')]));
    assert.match(checkMetaTheoryFoundationSources(audit, snapshots, changed).join('\n'), /provenance/u);
    changed.delete(path);
    assert.match(checkMetaTheoryFoundationSources(audit, snapshots, changed).join('\n'), /missing/u);
  }
  const changed = new Map(projectSources);
  changed.set('rust/src/link_network.rs', Buffer.from('meta-theory is our philosophical and theoretical foundation'));
  assert.match(checkMetaTheoryFoundationSources(audit, snapshots, changed).join('\n'), /implementation anchor is missing/u,
    'a provider declaration cannot replace actual native reference and tuple representation');
});
