// Apply only the exact redundant-clone removals reviewed in the hosted Rust diagnostic.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { DecoratorSet } from '../src/decorators.js';

const reviewFile = new URL('../../parity/self-translation/binary-network-ci-clone-review-138183525811.lino', import.meta.url);
const capture = {
  run: 45251703034,
  job: 138183525811,
  commit: 'c29d2b626cd05c209c814cc78cf654668ed8d29c',
  logSha256: 'de624fb6a63e2e6f4c5fa0761c8f47c12a12a5e50ee3439502978f220124e422',
  rustVersion: '1.100.0',
  compilerRange: '>=1.100.0 <1.101.0',
};
const counts = {
  link_network_binary_rules: 59,
  link_network_binary_record_rules: 60,
};
const sha256 = source => createHash('sha256').update(source).digest('hex');

export function readBinaryNetworkCompilerCloneStyles(styled, family) {
  const bytes = readFileSync(reviewFile);
  assert.equal(sha256(bytes), '59faf399f98103a9511616edc720d3eb0830f2ae4729c79d2fd6dfad48986a0a');
  const review = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.ok(Object.hasOwn(counts, family), 'a known generated binary module family');
  assert.equal(review.schemaVersion, 1);
  assert.equal(review.reviewKind, 'Rendered Rust compiler diagnostics');
  assert.deepEqual(review.capture, capture);
  const paths = review.sources.map(({ path, bytes: length, sha256: hash }) => {
    assert.ok(Number.isSafeInteger(length) && length > 0);
    assert.match(hash, /^[a-f0-9]{64}$/u);
    return path;
  });
  assert.deepEqual(paths, Object.keys(counts).toSorted().flatMap(name =>
    [1, 2, 3].map(partition => `rust/src/${name}/part_${partition}.rs`)));
  const locations = new Set();
  const names = new Set();
  const corrections = [];
  let memberCount = 0;
  const prefix = `rust/src/${family}/`;
  for (const { path, name, source, bytes, sha256: sourceHash, startLine, edits } of review.functions) {
    if (!path.startsWith(prefix)) continue;
    assert.ok(paths.includes(path));
    assert.match(name, /^[a-z_][a-z_0-9]*$/u);
    assert.ok(!names.has(name), 'one complete reviewed declaration for each generated function');
    names.add(name);
    assert.ok(source.startsWith(`pub fn ${name}(`) && source.endsWith('\n}'));
    assert.equal(Buffer.byteLength(source), bytes);
    assert.equal(sha256(source), sourceHash);
    assert.equal(styled.filter(item => item.includes(source)).length, 1, 'each authentic generated declaration remains unique');
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
    id: 'binary-network-compiler-clone-review',
    level: 'emitter',
    order: 970,
    when: [['format', 'Rust'], ['scope', 'formatted-item']],
    actions: corrections,
  };
}

export function applyReviewedBinaryFramingStyles(initial, format) {
  const layer = new DecoratorSet([readBinaryNetworkCompilerCloneStyles(initial, 'link_network_binary_rules')]);
  return initial.map(item => {
    const corrected = layer.decorate('emitter', { format: 'Rust', scope: 'formatted-item', source: item }).source;
    return corrected === item ? item : format(corrected);
  });
}

/** Recover the complete producer used by the frozen candidate and record adapters. */
export function restoreBinaryNetworkFramingGeneratorSource(source) {
  if (sha256(source) !== 'ff30619739176e0e4f7012dcc4a8ab5820d09de4770a89f41c47990e9a9e94dc') return source;
  const changes = [
    ["import { applyReviewedBinaryFramingStyles, restoreBinaryNetworkFramingGeneratorSource } from './binary-network-compiler-clone-styles.mjs';\n", ''],
    ["    generatorSha256: digest(restoreBinaryNetworkFramingGeneratorSource(readFileSync(new URL(import.meta.url), 'utf8'))),", '    generatorSha256: digest(readFileSync(new URL(import.meta.url))),'],
    ["const initialStyles = styleBinaryItems(items, decorators);\nconst styled = candidateMode ? initialStyles : applyReviewedBinaryFramingStyles(initialStyles, format);\n", 'const styled = styleBinaryItems(items, decorators);\n'],
  ];
  let previous = source;
  for (const [current, original] of changes) {
    assert.equal(previous.split(current).length, 2);
    previous = previous.replace(current, original);
  }
  assert.equal(sha256(previous), 'b69fe844f08f933f7cca47b3fc34139e606b7bd72cefb3fd3c6380d4301745c8');
  return previous;
}
