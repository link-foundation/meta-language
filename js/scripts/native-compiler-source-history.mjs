import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

const receipt = new URL('../../parity/self-translation/native-compiler-source-history.lino', import.meta.url);
const paths = [
  'js/scripts/record-rule-library.mjs',
  'parity/self-translation/ascii-text-predicate-rules-native.lino',
  'rust/src/formal_concept_definition.rs',
  'rust/src/shared_numeric_function_model.rs',
  'rust/src/translation/javascript/declarations.rs',
];
const pin = source => ({ bytes: Buffer.byteLength(source), sha256: createHash('sha256').update(source).digest('hex') });
let sources;

/** Replay the exact source bytes still consumed by these historical readers. */
export function restoreNativeCompilerSource(source, path) {
  if (!paths.includes(path)) return source;
  if (!sources) {
    const bytes = readFileSync(receipt);
    assert.equal(pin(bytes).sha256, 'a5a62f5d05206bf5792d9cdbc40337f1738bedd61554d6d634bd50d26528e138');
    const archive = readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(archive.schemaVersion, 1);
    assert.equal(archive.revision, 'dca4a094b12da818904519940f24766c0b0b4011');
    assert.equal(archive.coordinateUnit, 'UTF-16 code units');
    assert.deepEqual(archive.sources.map(row => row.file), paths);
    sources = new Map(archive.sources.map(row => [row.file, row]));
  }
  const { current, previous, edits } = sources.get(path);
  const actual = pin(source);
  if (actual.sha256 !== current.sha256) return source;
  assert.deepEqual(actual, current);
  let boundary = source.length;
  for (const { start, removed, inserted } of edits.toReversed()) {
    assert.ok(Number.isSafeInteger(start) && start >= 0 && start + inserted.length <= boundary);
    assert.equal(source.slice(start, start + inserted.length), inserted);
    source = source.slice(0, start) + removed + source.slice(start + inserted.length);
    boundary = start;
  }
  assert.deepEqual(pin(source), previous);
  return source;
}
