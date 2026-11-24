import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { captureGitArtifactCertificates } from '../../js/scripts/capture-git-artifact-certificate.mjs';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { rawEmissionHistoryFile, rawEmissionProducerFiles, rawEmissionProducerRevision, rawEmissionSourceIdentity } from './raw-emission-source-history.mjs';

const root = new URL('../../', import.meta.url);
const trees = new Map();
const selections = rawEmissionProducerFiles.map(path => ({ path, revision: rawEmissionProducerRevision }));
const sources = [...captureGitArtifactCertificates(selections)].map(({ blob, certificate }) => {
  const file = certificate.path;
  certificate.trees = certificate.trees.map(tree => {
    if (!trees.has(tree.objectId)) trees.set(tree.objectId, tree);
    return trees.get(tree.objectId);
  });
  const source = blob.toString('utf8');
  assert.deepEqual(Buffer.from(source, 'utf8'), blob);
  return { file, current: rawEmissionSourceIdentity(readFileSync(new URL(file, root))),
    previous: rawEmissionSourceIdentity(blob), source, certificate };
});
writeProjectData(rawEmissionHistoryFile, { version: 1, sources });
const reader = new URL('./raw-emission-source-history.mjs', import.meta.url);
const input = readFileSync(reader, 'utf8');
const pattern = /^const historyDigest = '(?:PENDING|[a-f0-9]{64})';$/mu;
assert.equal(input.split(pattern).length, 2);
writeFileSync(reader, input.replace(pattern, `const historyDigest = '${rawEmissionSourceIdentity(readFileSync(rawEmissionHistoryFile)).sha256}';`));
