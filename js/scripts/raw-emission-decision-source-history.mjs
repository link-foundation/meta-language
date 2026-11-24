import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

export const rawDecisionSourceRevision = 'f73a10b22f8d0bcfbf4707621bcc9edd6989533d';
export const rawDecisionSourceHistory = new URL('../../parity/self-translation/raw-emission-decision-source-history.lino', import.meta.url);
export const rawDecisionSourceFiles = Object.freeze([
  'js/src/translation/javascript.js',
  'js/src/translation/javascript-infer.js',
  'js/src/typed-program-link-rules.js',
  'js/src/typed-program-links.js',
  'js/src/typed-program-checked-projection.js',
  'experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs',
  'js/scripts/portable-source-library-style.mjs',
  'js/src/translation/lexer.js',
]);
export const rawDecisionSourceIdentity = bytes => ({ bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') });
const packetSha256 = '3aa1419999cc3f54b2395701f27f407c3ff0d2cdb8db7f1f53c1ab6286e808fe';
const root = new URL('../../', import.meta.url);
let sources;

export function restoreRawDecisionSource(bytes, path) {
  if (!rawDecisionSourceFiles.includes(path)) return bytes;
  if (!sources) {
    const data = readFileSync(rawDecisionSourceHistory);
    assert.equal(rawDecisionSourceIdentity(data).sha256, packetSha256, 'the complete raw decision source history stays exact');
    const packet = readBootstrapStructuredDataLino(data.toString('utf8'));
    assert.equal(packet.revision, rawDecisionSourceRevision);
    assert.deepEqual(packet.sources.map(row => row.file), rawDecisionSourceFiles);
    sources = new Map(packet.sources.map(row => [row.file, row]));
  }
  const row = sources.get(path);
  assert.deepEqual(rawDecisionSourceIdentity(readFileSync(new URL(path, root))), row.current,
    'the physical raw emission decision source changed: ' + path);
  assert.deepEqual(rawDecisionSourceIdentity(bytes), row.current,
    'project data is stale: raw emission decision source ' + path);
  const historical = Buffer.from(row.source);
  return verifyGitArtifactCertificate(row.certificate, historical, { revision: rawDecisionSourceRevision, path });
}
