import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { checkAssociativeStackSources, checkAssociativeStackProviderRoles } from '../scripts/associative-stack-sources.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const file = path => new URL(`../../${path}`, import.meta.url);
const audit = readProjectData(file('parity/associative-stack-audit.json'));
const snapshots = new Map(audit.guides.flatMap(guide => [guide.commitVerification.metadataFile, ...guide.files.map(source => source.sourceFile)])
  .map(path => [path, readFileSync(file(path))]));

test('both foundation guides retain exact pinned source and commit-response bytes', () => {
  assert.deepEqual(audit.guides.map(({ repository }) => repository), ['link-assistant/formal-ai', 'link-foundation/meta-theory']);
  assert.equal(snapshots.size, 6);
  assert.equal(audit.guides.flatMap(({ files }) => files).length, 4);
  assert.deepEqual(checkAssociativeStackSources(audit, snapshots), []);
  const guide = audit.guides[0];
  assert.equal(guide.files[0].path, 'docs/associative-tech-stack.md');
  const theory = audit.guides[1];
  assert.deepEqual(theory.files.map(({ path }) => path), [
    'archive/0.0.2/article.md',
    'drafts/0.0.3/article/04-mathematical-introduction.md',
    'drafts/0.0.3/article/07-meta-theory.md',
  ]);
  assert.equal(audit.executionEvidence, false, 'captured source is not upstream execution or adoption evidence');
});

test('foundation provenance rejects altered contents, hashes, Git objects and revision metadata', () => {
  for (const guide of audit.guides) for (const source of guide.files) {
    const changed = new Map(snapshots);
    changed.set(source.sourceFile, Buffer.concat([snapshots.get(source.sourceFile), Buffer.from('\n')]));
    assert.match(checkAssociativeStackSources(audit, changed).join('\n'), /byte length, content hash or Git blob/u);
    changed.delete(source.sourceFile);
    assert.match(checkAssociativeStackSources(audit, changed).join('\n'), /source is missing/u);
  }
  const changes = [
    value => { value.guides[0].files[0].bytes += 1; },
    value => { value.guides[0].files[0].sha256 = '0'.repeat(64); },
    value => { value.guides[0].files[0].blobSha1 = '0'.repeat(40); },
    value => { value.guides[0].files[0].sourceFile = '../outside.md'; },
    value => { value.guides[0].files[0].url = 'https://github.com/link-assistant/formal-ai/blob/main/docs/associative-tech-stack.md'; },
    value => { value.guides[0].revision = 'main'; },
    value => { value.guides[0].commitVerification.resolvedRevision = '0'.repeat(40); },
    value => { value.guides[0].commitVerification.treeSha1 = '0'.repeat(40); },
    value => { value.guides[0].commitVerification.sha256 = '0'.repeat(64); },
    value => { value.guides[1].files.pop(); },
    value => { value.guides[1].files[1] = value.guides[1].files[0]; },
    value => { value.guides[1] = value.guides[0]; },
    value => { value.executionEvidence = true; },
  ];
  for (const change of changes) {
    const changed = structuredClone(audit);
    change(changed);
    assert.ok(checkAssociativeStackSources(changed, snapshots).length > 0, 'incoherent provenance cannot pass validation');
  }
  for (const guide of audit.guides) {
    const changed = new Map(snapshots);
    const response = JSON.parse(snapshots.get(guide.commitVerification.metadataFile));
    response.sha = '0'.repeat(40);
    const bytes = Buffer.from(JSON.stringify(response));
    changed.set(guide.commitVerification.metadataFile, bytes);
    const matchingHash = structuredClone(audit);
    const verification = matchingHash.guides.find(value => value.repository === guide.repository).commitVerification;
    verification.bytes = bytes.length;
    verification.sha256 = createHash('sha256').update(bytes).digest('hex');
    assert.match(checkAssociativeStackSources(matchingHash, changed).join('\n'), /captured GitHub response/u,
      'a matching response hash cannot resolve a different commit as the recorded revision');
  }
});

test('the pinned stack source classifies every provider and keeps its distinct dependency boundaries', () => {
  assert.deepEqual(checkAssociativeStackSources(audit, snapshots), []);
  assert.deepEqual(checkAssociativeStackProviderRoles(audit, snapshots), []);
  assert.deepEqual(audit.sourceProviders.map(({name}) => name), [
    'links-notation', 'link-cli', 'doublets-rs', 'platform-mem',
    'lino-objects-codec', 'meta-language', 'link-calculator', 'lino-arguments',
    'lino-i18n', 'doublets-web', 'relative-meta-logic', 'meta-theory',
    'transformer', 'Agent CLI', 'Hive Mind',
  ]);
  assert.equal(audit.sourceProviders.flatMap(provider => provider.locations).length, 16);
  assert.deepEqual(audit.sourceProviders.map(provider => provider.repository), [
    'link-foundation/links-notation', 'link-foundation/link-cli', 'linksplatform/doublets-rs', 'linksplatform/mem-rs',
    'link-foundation/lino-objects-codec', 'link-foundation/meta-language', 'link-assistant/calculator',
    'link-foundation/lino-arguments', 'link-foundation/lino-i18n', 'linksplatform/doublets-web',
    'link-foundation/relative-meta-logic', 'link-foundation/meta-theory', 'link-foundation/transformer',
    'link-assistant/agent', 'link-assistant/hive-mind',
  ]);
  assert.deepEqual(audit.sourceProviders.map(provider => provider.locations.map(location => location.role)), [
    ['direct runtime dependency'], ['direct runtime dependency', 'protocol compatibility'],
    ['transitive storage dependency'], ['transitive storage dependency'],
    ['direct runtime dependency'], ['direct runtime dependency'], ['direct runtime dependency'],
    ['direct runtime dependency'], ['direct runtime dependency'], ['conditional browser compatibility'],
    ['conceptual reference'], ['conceptual reference'], ['conceptual reference'], ['development tool'], ['development tool'],
  ]);
  assert.deepEqual(audit.sourceProviders.filter(provider => provider.locations.some(location => location.role === 'transitive storage dependency')).map(({name}) => name), ['doublets-rs', 'platform-mem']);
  assert.deepEqual(audit.sourceProviders.filter(provider => provider.locations.some(location => location.role === 'development tool')).map(({name}) => name), ['Agent CLI', 'Hive Mind']);
  assert.deepEqual(audit.sourceProviders.find(provider => provider.name === 'link-cli').locations.map(({role}) => role), ['direct runtime dependency', 'protocol compatibility']);
  assert.equal(audit.sourceProviders.find(provider => provider.name === 'doublets-web').locations[0].role, 'conditional browser compatibility');
  assert.equal(audit.sourceProviders.find(provider => provider.name === 'meta-theory').locations[0].role, 'conceptual reference');
  assert.equal(audit.components.find(provider => provider.name === 'link-cli').role, 'protocol compatibility', 'the guide runtime and this project compatibility adapter remain separate roles');
  assert.equal(audit.executionEvidence, false);
  recordIssue195Observations({
    requirementId: 'I195-ASSOCIATIVE-STACK-PINNED-SOURCE-AUDIT', runtime: 'tooling', suffix: 'behavior',
    fixtureId: 'conversation-contract:associativeFoundationRequest',
    fixtureFile: 'parity/sources/associative-stack-and-theory-foundation-2026-10-12.txt',
    assertions: ['associativeStackSourceReadAndPinned', 'stackContentHashRecorded', 'providerRolesClassified'],
    testName: 'the pinned stack source classifies every provider and keeps its distinct dependency boundaries',
  });
});

test('provider source classifications reject missing sections, wrong roles, identities and source anchors', () => {
  const changes = [
    value => { delete value.sourceProviders; },
    value => { value.sourceProviders[0] = null; },
    value => { value.sourceProviders[0].locations[0] = null; },
    value => { value.sourceProviders[0].locations[0].heading = 7; },
    value => { value.sourceProviders.pop(); },
    value => { value.sourceProviders.splice(3, 1); },
    value => { value.sourceProviders.push(value.sourceProviders[0]); },
    value => { value.sourceProviders[0].name = 'notation'; },
    value => { value.sourceProviders[0].repository = 'link-assistant/formal-ai'; },
    value => { value.sourceProviders[2].repository = 'linksplatform/mem-rs'; },
    value => { value.sourceProviders[0].locations = []; },
    value => { value.sourceProviders[0].locations.push(value.sourceProviders[0].locations[0]); },
    value => { value.sourceProviders[0].locations[0].heading = 'invented provider'; },
    value => { value.sourceProviders[0].locations[0].sourceFile = 'moving/main.md'; },
    value => { value.sourceProviders[0].locations[0].anchor = 'unrecorded runtime capability'; },
    value => { value.sourceProviders[0].locations[0].role = 'development tool'; },
    value => { value.sourceProviders[2].locations[0].role = 'direct runtime dependency'; },
    value => { value.sourceProviders[1].locations[1].role = 'conceptual reference'; },
    value => { value.sourceProviders[9].locations[0].role = 'conceptual reference'; },
    value => { value.sourceProviders[3].locations[0].anchor = 'physical doublet network'; },
    value => { value.sourceProviders.at(-1).locations[0].role = 'direct runtime dependency'; },
  ];
  for (const change of changes) {
    const changed = structuredClone(audit); change(changed);
    assert.ok(checkAssociativeStackProviderRoles(changed, snapshots).length > 0);
  }
});
