import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData } from './project-data.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

const reviewFile = new URL('../../parity/self-translation/binary-last-use-review-138157819576.lino', import.meta.url);
const digest = value => createHash('sha256').update(value).digest('hex');
const reviewSha256 = '30fc979cd7ef1f084586d3a5eb6209c0775bfe04515ba8c3345de1f2d1bbf95d';
const previousGeneratorSha256 = 'b69fe844f08f933f7cca47b3fc34139e606b7bd72cefb3fd3c6380d4301745c8';

/** Apply the rendered compiler's reviewed last-use edits to exact formatted declarations. */
export function applyBinaryLastUseStyles(styled, family, format) {
  assert.equal(digest(readFileSync(reviewFile)), reviewSha256);
  const review = readProjectData(reviewFile);
  const record = review.families.find(row => row.family === family);
  assert.ok(record, 'the native family is covered by the reviewed Clippy run');
  const expectedCount = family === 'link_network_binary_rules' ? 59 : 60;
  assert.equal(record.diagnostics.length, expectedCount);
  assert.equal(record.functions.reduce((count, row) => count + row.offsets.length, 0), expectedCount);
  const actions = record.functions.map(({ name, sha256, offsets }) => {
    const owners = styled.filter(item => item.startsWith(`pub fn ${name}(`) || item.startsWith(`pub const fn ${name}(`) ||
      item.includes(`\npub fn ${name}(`) || item.includes(`\npub const fn ${name}(`));
    assert.equal(owners.length, 1, `one exact generated declaration owns ${name}`);
    const from = readSourceTextRustBlock(owners[0], 'fn', name);
    assert.equal(digest(from), sha256, `reviewed declaration ${name} must retain its exact source`);
    let to = from;
    let boundary = from.length;
    for (const offset of offsets.slice().reverse()) {
      assert.ok(Number.isSafeInteger(offset) && offset >= 0 && offset + 8 <= boundary);
      assert.equal(from.slice(offset, offset + 8), '.clone()');
      to = to.slice(0, offset) + to.slice(offset + 8);
      boundary = offset;
    }
    return { op: 'replace', field: 'source', from, to };
  });
  const style = new DecoratorSet([{ id: `${family}-reviewed-last-use`, level: 'emitter', order: 0,
    when: [['format', 'Rust'], ['scope', 'formatted-item']], actions }]);
  return styled.map(source => {
    const corrected = style.decorate('emitter', { format: 'Rust', scope: 'formatted-item', source }).source;
    return corrected === source ? source : format(corrected);
  });
}

/** Retain the complete frozen producer for the historical candidate and record adapter. */
export function restoreBinaryPrimitiveGeneratorSource(source) {
  if (digest(source) !== 'a0fc3ed3d8bd3dcf58bf293e6524a5828a5310d10014fe3586ddcd8460de20f4') return source;
  const edits = [
    ["import { applyBinaryLastUseStyles, restoreBinaryPrimitiveGeneratorSource } from './binary-last-use-styles.mjs';\n", ''],
    ['digest(restoreBinaryPrimitiveGeneratorSource(readFileSync(new URL(import.meta.url), \'utf8\')))', 'digest(readFileSync(new URL(import.meta.url)))'],
    [`const formatted = styleBinaryItems(items, decorators);\nconst styled = candidateMode ? formatted : applyBinaryLastUseStyles(formatted, 'link_network_binary_rules', format);`,
      'const styled = styleBinaryItems(items, decorators);'],
  ];
  for (const [before, after] of edits) {
    assert.equal(source.split(before).length, 2);
    source = source.replace(before, () => after);
  }
  assert.equal(digest(source), previousGeneratorSha256);
  return source;
}
