import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { checkAssociativeStackSources } from '../scripts/associative-stack-sources.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

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
