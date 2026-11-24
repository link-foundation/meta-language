import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { asciiHostPin, replayAsciiHostEdits } from './ascii-text-predicate-host-history.mjs';

const history = new URL('../../parity/self-translation/registered-native-test-source-history.lino', import.meta.url);
const historicalRevision = '7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3';
const historySha256 = 'c1def0fbbbaa6425f2759c068bf4e62a813da7ae4763b0bab3227ba38256a3a7';
let sources;

/** Recover the exact registered predecessor for a historical source check. */
export function restoreRegisteredNativeTestSource(source, path) {
  if (!sources) {
    const bytes = readFileSync(history);
    assert.equal(asciiHostPin(bytes).sha256, historySha256);
    const packet = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(packet.schemaVersion, 1);
    assert.equal(packet.revision, historicalRevision);
    sources = new Map(packet.sources.map(row => [row.path, row]));
    assert.equal(sources.size, packet.sources.length);
  }
  const record = sources.get(path);
  if (!record) return source;
  const current = asciiHostPin(source);
  if (current.sha256 !== record.current.sha256) return source;
  assert.deepEqual(current, record.current);

  const replay = replayAsciiHostEdits(source, record.changes);
  assert.deepEqual(asciiHostPin(replay.source), record.previous);
  return replay.source;
}
