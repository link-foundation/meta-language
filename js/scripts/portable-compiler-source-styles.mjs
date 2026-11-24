import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { DecoratorSet } from '../src/decorators.js';

const root = new URL('../../', import.meta.url);
const reviewFile = 'parity/self-translation/portable-ci-source-style-review-138183525811.lino';
const reviewDigest = '4fd3d3f2b6e97e5822e8008f6fe6c5fd5e7bba0f2c75c5e60657cc8763ce63b9';
const capture = {
  run: 45251703034,
  job: 138183525811,
  commit: 'c29d2b626cd05c209c814cc78cf654668ed8d29c',
  logSha256: 'de624fb6a63e2e6f4c5fa0761c8f47c12a12a5e50ee3439502978f220124e422',
  rustVersion: '1.100.0',
  compilerRange: '>=1.100.0 <1.101.0',
};
const diagnosticCounts = {
  declaration_dependency_rules: 8,
  javascript_record_parameter_rules: 1,
  ascii_text_predicate_rules: 4,
  external_emission_rules: 4,
  reference_identity_rules: 1,
  rust_literal_match_rules: 1,
};
const digest = source => createHash('sha256').update(source).digest('hex');

/** Replay reviewed corrections against exact formatted translated declarations. */
export function applyReviewedPortableSourceStyles(source, family, format) {
  if (!Object.hasOwn(diagnosticCounts, family)) return source;
  const bytes = readFileSync(new URL(reviewFile, root));
  assert.equal(digest(bytes), reviewDigest);
  const review = readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.equal(review.schemaVersion, 1);
  assert.equal(review.reviewKind, 'Rendered Rust compiler diagnostics');
  assert.deepEqual(review.capture, capture);
  const paths = new Set();
  for (const { path, bytes, sha256 } of review.sources) {
    assert.match(path, /^rust\/src\/translation\/[a-z_]+\/part_[1-5]\.rs$/u);
    assert.ok(Number.isSafeInteger(bytes) && bytes > 0 && !paths.has(path));
    assert.match(sha256, /^[a-f0-9]{64}$/u);
    paths.add(path);
  }
  const actions = [];
  const diagnostics = new Set();
  const names = new Set();
  for (const { path, name, source: from, bytes, sha256, edits } of review.functions) {
    if (path.split('/').at(-2) !== family) continue;
    assert.ok(paths.has(path) && !names.has(name));
    names.add(name);
    assert.match(name, /^[a-z_][a-z_0-9]*$/u);
    assert.ok(from.includes(`pub fn ${name}(`) && from.endsWith('\n}') && edits.length > 0);
    assert.equal(Buffer.byteLength(from), bytes);
    assert.equal(digest(from), sha256);
    assert.equal(source.split(from).length, 2, 'the reviewed translated function remains unique: ' + name);
    let cursor = 0;
    let corrected = '';
    for (const { start, from: original, to, diagnostics: evidence } of edits) {
      assert.ok(Number.isSafeInteger(start) && start >= cursor && original && original !== to);
      const end = start + original.length;
      assert.equal(from.slice(start, end), original);
      corrected += from.slice(cursor, start) + to;
      cursor = end;
      for (const row of evidence) {
        assert.ok(row.path === path && Number.isSafeInteger(row.logLine) && row.logLine > 0 && !diagnostics.has(row.logLine));
        assert.ok(Number.isSafeInteger(row.line) && row.line > 0 && Number.isSafeInteger(row.column) && row.column > 0);
        assert.match(row.lint, /^[a-z_]+$/u);
        assert.match(row.diagnosticSha256, /^[a-f0-9]{64}$/u);
        diagnostics.add(row.logLine);
      }
    }
    actions.push({ op: 'replace', field: 'source', from, to: corrected + from.slice(cursor) });
  }
  assert.equal(diagnostics.size, diagnosticCounts[family], 'every archived Rust diagnostic keeps its approved correction');
  const layer = new DecoratorSet([{
    id: 'portable-compiler-source-review', level: 'emitter', order: 980,
    when: [['format', 'Rust'], ['scope', 'formatted-source']], actions,
  }]);
  return format(layer.decorate('emitter', { format: 'Rust', scope: 'formatted-source', source }).source);
}
