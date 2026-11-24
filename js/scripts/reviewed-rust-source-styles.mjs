import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData } from './project-data.mjs';
import { partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const reviewFile = new URL('../../parity/self-translation/rust-clippy-style-review.lino', import.meta.url);
const digest = source => createHash('sha256').update(source).digest('hex');
let reviewedModules;

const originalBinaryProducerDigest = 'b69fe844f08f933f7cca47b3fc34139e606b7bd72cefb3fd3c6380d4301745c8';
const binaryProducerChanges = [
  ["import { applyReviewedRustSourceStyles, readOriginalBinaryRuleProducer } from './reviewed-rust-source-styles.mjs';\n", ''],
  ['generatorSha256: digest(readOriginalBinaryRuleProducer(readFileSync(new URL(import.meta.url), \'utf8\'))),',
    'generatorSha256: digest(readFileSync(new URL(import.meta.url))),'],
  ["if (!candidateMode) {\n  const target = destination.pathname.split('/').at(-2);\n  for (const output of outputs) {\n    output[1] = applyReviewedRustSourceStyles(`rust/src/${target}/${output[0]}`, output[1], format);\n  }\n}\n", ''],
];

/** Retain the byte-exact producer used by the immutable candidate experiment. */
export function readOriginalBinaryRuleProducer(source) {
  if (digest(source) === originalBinaryProducerDigest) return source;
  let original = source;
  for (const [current, previous] of binaryProducerChanges) {
    assert.equal(original.split(current).length, 2, 'one exact reviewed binary producer change');
    original = original.replace(current, () => previous);
  }
  assert.equal(digest(original), originalBinaryProducerDigest, 'the complete original producer is recoverable');
  return original;
}

function readReviewedRustSourceStyles() {
  const review = readProjectData(reviewFile);
  assert.equal(review.schemaVersion, 2);
  assert.equal(review.run, 45283381518);
  assert.equal(review.job, 138157819576);
  assert.equal(review.commit, '83dff2e53f63bcfae83698c2531696e90417164f');
  assert.equal(review.rustVersion, '1.100.0');
  assert.equal(review.logSha256, 'e7d5adc330536606d573ff10f0c0ecb8a588f2c98508d17b90e875b34010d537');
  const modules = new Map(review.modules.map(module => [module.path, module]));
  assert.equal(modules.size, review.modules.length);
  const cloneLines = review.modules.flatMap(module => module.diagnostics.map(diagnostic => diagnostic.logLine));
  const styleLines = review.modules.flatMap(module => module.styles.flatMap(style => style.logLines));
  assert.equal(cloneLines.length, 150);
  assert.equal(styleLines.length, 19);
  assert.equal(new Set([...cloneLines, ...styleLines]).size, 169);
  return modules;
}

/** Apply reviewed source corrections after partitioning a generated library. */
export function partitionReviewedRustLibrary(source, options) {
  return partitionPortableRustLibrary(source, options).map(part => {
    if (part.path === null) return part;
    const correctedSource = applyReviewedRustSourceStyles(`rust/src/translation/${part.path}`, part.source, formatProductRuleSource);
    assert.ok(correctedSource.split('\n').length < 1000, 'reviewed corrections retain a bounded child module');
    return { ...part, source: correctedSource };
  });
}

/** Replay corrections tied to rendered diagnostics on the exact emitted module. */
export function applyReviewedRustSourceStyles(path, source, format) {
  reviewedModules ??= readReviewedRustSourceStyles();
  const module = reviewedModules.get(path);
  if (!module) return source;
  assert.equal(digest(source), module.sourceSha256, `reviewed Rust module changed: ${path}`);
  const edits = [
    ...module.diagnostics.map(({ start }) => ({ start, from: '.clone()', to: '' })),
    ...module.styles,
  ].sort((left, right) => left.start - right.start);
  let corrected = '';
  let previousEnd = 0;
  for (const { start, from, to } of edits) {
    const end = start + from.length;
    assert.ok(Number.isSafeInteger(start) && start >= previousEnd && end <= source.length, path);
    assert.equal(source.slice(start, end), from, path);
    corrected += source.slice(previousEnd, start) + to;
    previousEnd = end;
  }
  corrected += source.slice(previousEnd);
  const styles = new DecoratorSet([{
    id: 'reviewed-rust-source-corrections', level: 'emitter', order: 0, when: [['format', 'Rust']],
    actions: [{ op: 'replace', field: 'source', from: source, to: corrected }],
  }]);
  return format(styles.decorate('emitter', { format: 'Rust', source }).source);
}
