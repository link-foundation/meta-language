// Apply only the exact redundant-clone removals reviewed in the hosted Rust diagnostic.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { readReviewedCompilerCloneStyles } from './reviewed-compiler-clone-styles.mjs';

const file = new URL('../../parity/self-translation/binary-network-ci-clone-review-138183525811.lino', import.meta.url);
const hash = '59faf399f98103a9511616edc720d3eb0830f2ae4729c79d2fd6dfad48986a0a';
const counts = {
  link_network_binary_rules: 59,
  link_network_binary_record_rules: 60,
};
const paths = Object.keys(counts).toSorted().flatMap(name =>
  [1, 2, 3].map(partition => `rust/src/${name}/part_${partition}.rs`));
const id = 'binary-network-compiler-clone-review';
const sha256 = source => createHash('sha256').update(source).digest('hex');

export function readBinaryNetworkCompilerCloneStyles(styled, family) {
  return readReviewedCompilerCloneStyles(styled, family, { file, hash, counts, paths, id });
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
