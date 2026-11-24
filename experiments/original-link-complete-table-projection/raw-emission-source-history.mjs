import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { verifyGitArtifactCertificate } from '../../js/scripts/git-object-certificate.mjs';

export const rawEmissionProducerRevision = '81107a5945651f17c962e4bb81fb560f96c65ccd';
export const rawEmissionProducerFiles = Object.freeze([
  'js/src/translation/javascript-infer.js',
  'js/src/translation/javascript-surface-rules.js',
  'js/src/translation/collection-construction.js',
  'js/src/translation/surface-reference-facts.js',
  'js/src/translation/types.js',
  'js/src/translation/optional-value-rules.js',
  'js/tests/translation-collection-construction.test.js',
]);
export const rawEmissionHistoryFile = new URL('./data/raw-emission-source-history.lino', import.meta.url);
const historyDigest = 'eb66543bdbea3e413447e057f25ff71f069fac498c1b3bee9077a94f318afaa3';
export const rawEmissionSourceIdentity = bytes => ({ bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') });
let history;

/** Present the owning extraction's source only for a complete reviewed successor. */
export function restoreRawEmissionProducerSource(file, bytes) {
  if (!Buffer.isBuffer(bytes) || !rawEmissionProducerFiles.includes(file)) return bytes;
  if (!history) {
    const data = readFileSync(rawEmissionHistoryFile);
    assert.equal(rawEmissionSourceIdentity(data).sha256, historyDigest);
    const packet = readBootstrapStructuredDataLino(data.toString('utf8'));
    assert.equal(packet.version, 1);
    assert.deepEqual(packet.sources.map(row => row.file), rawEmissionProducerFiles);
    history = new Map(packet.sources.map(row => [row.file, row]));
  }
  const row = history.get(file);
  const identity = rawEmissionSourceIdentity(bytes);
  if (identity.sha256 === row.previous.sha256 || identity.sha256 !== row.current.sha256) return bytes;
  assert.deepEqual(identity, row.current);
  const restored = Buffer.from(row.source, 'utf8');
  assert.deepEqual(rawEmissionSourceIdentity(restored), row.previous);
  verifyGitArtifactCertificate(row.certificate, restored, { revision: rawEmissionProducerRevision, path: file });
  return restored;
}
