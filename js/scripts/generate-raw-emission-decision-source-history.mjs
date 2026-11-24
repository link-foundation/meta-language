import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { writeProjectData } from './project-data.mjs';
import {
  rawDecisionSourceFiles, rawDecisionSourceHistory, rawDecisionSourceIdentity, rawDecisionSourceRevision,
} from './raw-emission-decision-source-history.mjs';

const root = new URL('../../', import.meta.url);
const revision = rawDecisionSourceRevision;
const sources = rawDecisionSourceFiles.map(file => {
  const { blob, certificate } = captureGitArtifactCertificate(revision, file);
  const source = blob.toString('utf8');
  assert.deepEqual(Buffer.from(source), blob);
  return { file, source, certificate, current: rawDecisionSourceIdentity(readFileSync(new URL(file, root))) };
});
writeProjectData(rawDecisionSourceHistory, { revision, sources });
const reader = new URL('./raw-emission-decision-source-history.mjs', import.meta.url);
const source = readFileSync(reader, 'utf8');
const pin = /^const packetSha256 = '(?:PENDING|[a-f0-9]{64})';$/mu;
assert.equal(source.split(pin).length, 2);
writeFileSync(reader, source.replace(pin, `const packetSha256 = '${rawDecisionSourceIdentity(readFileSync(rawDecisionSourceHistory)).sha256}';`));
