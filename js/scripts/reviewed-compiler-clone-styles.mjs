// Verify exact redundant-clone corrections against the hosted Rust diagnostic.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

const capture = {
  run: 45251703034,
  job: 138183525811,
  commit: 'c29d2b626cd05c209c814cc78cf654668ed8d29c',
  logSha256: 'de624fb6a63e2e6f4c5fa0761c8f47c12a12a5e50ee3439502978f220124e422',
  rustVersion: '1.100.0',
  compilerRange: '>=1.100.0 <1.101.0',
};
const sha256 = source => createHash('sha256').update(source).digest('hex');

export function readReviewedCompilerCloneStyles(styled, family, { file, hash, counts, paths: expectedPaths, id }) {
  const bytes = readFileSync(file);
  assert.equal(sha256(bytes), hash);
  const review = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.ok(Object.hasOwn(counts, family), 'a known generated module family');
  assert.equal(review.schemaVersion, 1);
  assert.equal(review.reviewKind, 'Rendered Rust compiler diagnostics');
  assert.deepEqual(review.capture, capture);
  const paths = review.sources.map(({ path, bytes: length, sha256: hash }) => {
    assert.ok(Number.isSafeInteger(length) && length > 0);
    assert.match(hash, /^[a-f0-9]{64}$/u);
    return path;
  });
  assert.deepEqual(paths, expectedPaths);
  const locations = new Set();
  const names = new Set();
  const corrections = [];
  let memberCount = 0;
  for (const { path, name, source, bytes, sha256: sourceHash, startLine, edits } of review.functions) {
    if (path.split('/').at(-2) !== family) continue;
    assert.ok(paths.includes(path));
    assert.match(name, /^[a-z_][a-z_0-9]*$/u);
    assert.ok(!names.has(name), 'one complete reviewed declaration for each generated function');
    names.add(name);
    assert.ok(source.startsWith(`pub fn ${name}(`) && source.endsWith('\n}'));
    assert.equal(Buffer.byteLength(source), bytes);
    assert.equal(sha256(source), sourceHash);
    assert.equal(styled.reduce((count, item) => count + item.split(source).length - 1, 0), 1, 'each authentic generated declaration remains unique: ' + name);
    assert.ok(Number.isSafeInteger(startLine) && startLine > 0 && edits.length > 0);
    const ordered = edits.toSorted((left, right) => left.start - right.start);
    let previousEnd = 0;
    for (const { start, line, column, logLine, diagnosticSha256 } of ordered) {
      assert.ok(Number.isSafeInteger(start) && start >= previousEnd && start + 8 <= source.length);
      assert.equal(source.slice(start, start + 8), '.clone()');
      const precedingLines = source.slice(0, start).split('\n');
      assert.equal(line, startLine + precedingLines.length - 1);
      assert.equal(column, precedingLines.at(-1).length + 1);
      assert.ok(Number.isSafeInteger(logLine) && logLine > 0 && !locations.has(logLine));
      assert.match(diagnosticSha256, /^[a-f0-9]{64}$/u);
      locations.add(logLine);
      previousEnd = start + 8;
      memberCount += 1;
    }
    let result = source;
    for (const { start } of ordered.toReversed()) result = result.slice(0, start) + result.slice(start + 8);
    corrections.push({ op: 'replace', field: 'source', from: source, to: result });
  }
  assert.equal(memberCount, counts[family], 'every diagnosed clone has exactly one correction');
  return {
    id,
    level: 'emitter',
    order: 970,
    when: [['format', 'Rust'], ['scope', 'formatted-item']],
    actions: corrections,
  };
}
