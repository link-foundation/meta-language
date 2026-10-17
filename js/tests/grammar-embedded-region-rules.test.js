import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { ParseConfiguration, TriviaAttachmentPolicy } from '../src/primitives.js';
import {
  acceptEmbeddedRegionBounds, acceptEmbeddedRegionLimit, acceptEmbeddedRegionSourceCoverage,
  readEmbeddedRegionAncestor, readEmbeddedRegionCharge, readEmbeddedRegionDefaultLimit,
  readEmbeddedRegionDiagnosticLimit, readEmbeddedRegionDiagnosticObserved, readEmbeddedRegionDisposition,
} from '../src/translation/embedded-region-rules.js';
import { embeddedRegionRulesPolicy, renderEmbeddedRegionNativeDefaults } from '../scripts/generate-embedded-region-rules.mjs';

test('default embedded region limits remain shared constant data and preserve immutable configuration', () => {
  const configuration = ParseConfiguration.default();
  assert.deepEqual([configuration.embeddedRegionDepthLimit, configuration.embeddedRegionCountLimit,
    configuration.embeddedRegionByteLimit], [128, 4096, 16 * 1024 * 1024]);
  assert.equal(readEmbeddedRegionDefaultLimit('unknown'), -1);
  const limited = configuration.withEmbeddedRegionLimits({ depthLimit: 3, countLimit: 4, byteLimit: 20 });
  assert.equal(configuration.embeddedRegionDepthLimit, 128);
  assert.deepEqual([limited.embeddedRegionDepthLimit, limited.embeddedRegionCountLimit,
    limited.embeddedRegionByteLimit], [3, 4, 20]);
  const changed = limited.withRegionDetectionPolicy('NameDriven')
    .withTriviaAttachmentPolicy(TriviaAttachmentPolicy.TokenLink).withAccessMode('read-only');
  assert.deepEqual([changed.embeddedRegionDepthLimit, changed.embeddedRegionCountLimit,
    changed.embeddedRegionByteLimit], [3, 4, 20]);
  assert.equal(changed.withEmbeddedRegionLimits({ byteLimit: 0 }).embeddedRegionCountLimit, 4);
  assert.equal(new ParseConfiguration({ embeddedRegionDepthLimit: 0 }).embeddedRegionDepthLimit, 0);
  const policy = embeddedRegionRulesPolicy();
  assert.deepEqual(policy.defaults, { depthLimit: 128, countLimit: 4096, byteLimit: 16777216 });
  assert.equal(policy.source, readFileSync(new URL('../src/translation/embedded-region-rules.js', import.meta.url), 'utf8'));
  assert.equal(renderEmbeddedRegionNativeDefaults(policy.defaults),
    '// Generated DATA from the shared embedded-region-rules.js defaults. Do not edit.\n'
    + 'pub const DEFAULT_EMBEDDED_REGION_DEPTH_LIMIT: usize = 128;\n'
    + 'pub const DEFAULT_EMBEDDED_REGION_COUNT_LIMIT: usize = 4096;\n'
    + 'pub const DEFAULT_EMBEDDED_REGION_BYTE_LIMIT: usize = 16777216;\n');
});

test('embedded region configuration refuses fractional negative nonnumeric and unbounded limits', () => {
  for (const value of [0, 1, Number.MAX_SAFE_INTEGER]) assert.equal(acceptEmbeddedRegionLimit(value), true);
  for (const value of [-1, 0.5, Infinity, -Infinity, NaN, Number.MAX_SAFE_INTEGER + 1]) {
    assert.equal(acceptEmbeddedRegionLimit(value), false);
    for (const field of ['embeddedRegionDepthLimit', 'embeddedRegionCountLimit', 'embeddedRegionByteLimit']) {
      assert.throws(() => new ParseConfiguration({ [field]: value }), /nonnegative safe integers/u);
    }
  }
  for (const value of ['1', null, false]) assert.throws(() =>
    ParseConfiguration.default().withEmbeddedRegionLimits({ byteLimit: value }), /nonnegative safe integers/u);
  assert.throws(() => renderEmbeddedRegionNativeDefaults({ depthLimit: 1, countLimit: -1, byteLimit: 1 }), /nonnegative safe integers/u);
});

test('embedded region bounds retain zero width and refuse children outside their actual parent', () => {
  for (const [start, end] of [[10, 20], [10, 10], [15, 15], [20, 20], [11, 19]]) {
    assert.equal(acceptEmbeddedRegionBounds(10, 20, start, end), true);
  }
  for (const [start, end] of [[9, 20], [10, 21], [19, 11], [-1, 0], [10.5, 20], [NaN, 20]]) {
    assert.equal(acceptEmbeddedRegionBounds(10, 20, start, end), false);
  }
  assert.equal(acceptEmbeddedRegionSourceCoverage(10, 10), true);
  assert.equal(acceptEmbeddedRegionSourceCoverage(9, 10), false);
});

test('embedded region ancestry compares canonical language and both absolute bounds without conflating siblings', () => {
  const parents = [-1, 0, 1, 0], languages = [0, 1, 0, 1];
  const starts = [0, 10, 20, 10], ends = [100, 90, 80, 90];
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 2, 0, 0, 100), 0);
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 2, 1, 10, 90), 1);
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 2, 0, 20, 80), 2);
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 3, 0, 20, 80), -1);
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 2, 0, 10, 90), -1);
  assert.equal(readEmbeddedRegionAncestor(parents, languages, starts, ends, 2, 1, 10, 80), -1);
  assert.equal(readEmbeddedRegionAncestor([], [], [], [], -1, 0, 0, 0), -1);
  for (const malformed of [[-1, 2], [-1, 1], [-1, 0.5], [-1, -2]]) {
    assert.equal(readEmbeddedRegionAncestor(malformed, [0, 1], [0, 0], [2, 2], 1, 2, 0, 2), -2);
  }
  assert.equal(readEmbeddedRegionAncestor([-1], [], [0], [1], 0, 0, 0, 1), -2);
  assert.equal(readEmbeddedRegionAncestor([-1], [0], [0], [1], 5, 0, 0, 1), -2);
  assert.equal(readEmbeddedRegionAncestor([-1], [-1], [0], [1], 0, -1, 0, 1), -2);
  assert.equal(readEmbeddedRegionAncestor([-1], [0], [2], [1], 0, 0, 2, 1), -2);
  for (let column = 0; column < 4; column += 1) {
    const values = [[-1, 0], [0, 1], [0, 0], [2, 2]];
    delete values[column][1];
    assert.equal(readEmbeddedRegionAncestor(...values, 1, 2, 0, 2), -2,
      'a missing numeric fact becomes NaN and fails validation, never an invented zero');
  }
});

test('embedded region dispositions reuse the current host and diagnose true cycles and exact resource boundaries', () => {
  const disposition = (changes = {}) => {
    const values = { parentStart: 0, parentEnd: 100, parentLanguage: 0, language: 1,
      start: 10, end: 20, ancestor: -1, depth: 1, count: 0, bytes: 0,
      depthLimit: 2, countLimit: 2, byteLimit: 10, rootSpellingReuse: true, ...changes };
    return readEmbeddedRegionDisposition(...Object.values(values));
  };
  assert.equal(disposition(), 'parse');
  assert.equal(disposition({ depth: 2, count: 1 }), 'parse');
  assert.equal(disposition({ depth: 3 }), 'embeddedRegionDepthLimit');
  assert.equal(disposition({ count: 2 }), 'embeddedRegionCountLimit');
  assert.equal(disposition({ bytes: 1 }), 'embeddedRegionByteLimit');
  assert.equal(disposition({ bytes: 11, start: 20, end: 20 }), 'embeddedRegionByteLimit');
  assert.equal(disposition({ start: 20, end: 20, byteLimit: 0 }), 'parse');
  assert.equal(disposition({ ancestor: 0 }), 'nonConsumingEmbeddingCycle');
  assert.equal(disposition({ language: 0, start: 0, end: 100, ancestor: 0, depthLimit: 0 }), 'reuse');
  assert.equal(disposition({ language: 0, start: 0, end: 100, ancestor: 0, depth: 2 }), 'reuseAttached');
  assert.equal(disposition({ language: 0, start: 0, end: 100, ancestor: 0, rootSpellingReuse: false, byteLimit: 100 }), 'parse');
  assert.equal(disposition({ start: -1 }), 'invalidEmbeddedRegionBounds');
  assert.equal(disposition({ ancestor: -2 }), 'invalidEmbeddedRegionAncestry');
  assert.equal(disposition({ countLimit: 0.5 }), 'invalidEmbeddedRegionLimits');
  assert.equal(disposition({ byteLimit: Number.MAX_SAFE_INTEGER, bytes: Number.MAX_SAFE_INTEGER - 10 }), 'parse');
  assert.equal(disposition({ byteLimit: Number.MAX_SAFE_INTEGER, bytes: Number.MAX_SAFE_INTEGER - 9 }), 'embeddedRegionByteLimit');
});

test('embedded region diagnostics expose attempted counters while only admitted attempts charge the traversal', () => {
  assert.equal(readEmbeddedRegionCharge(3, 10, 20, false), 4);
  assert.equal(readEmbeddedRegionCharge(3, 10, 20, true), 13);
  assert.equal(readEmbeddedRegionCharge(3, 10, 10, true), 3);
  const reasons = ['embeddedRegionDepthLimit', 'embeddedRegionCountLimit', 'embeddedRegionByteLimit'];
  assert.deepEqual(reasons.map(reason => readEmbeddedRegionDiagnosticObserved(reason, 4, 5, 6, 10, 20)), [4, 6, 16]);
  assert.deepEqual(reasons.map(reason => readEmbeddedRegionDiagnosticLimit(reason, 3, 5, 15)), [3, 5, 15]);
  for (const reason of ['nonConsumingEmbeddingCycle', 'invalidEmbeddedRegionBounds']) {
    assert.equal(readEmbeddedRegionDiagnosticObserved(reason, 4, 5, 6, 10, 20), -1);
    assert.equal(readEmbeddedRegionDiagnosticLimit(reason, 3, 5, 15), -1);
  }
  assert.equal(readEmbeddedRegionDiagnosticObserved('embeddedRegionCountLimit', 1, Number.MAX_SAFE_INTEGER, 0, 0, 1), -1);
  assert.equal(readEmbeddedRegionDiagnosticObserved('embeddedRegionByteLimit', 1, 0, Number.MAX_SAFE_INTEGER, 0, 1), -1);
});

test('complete embedded region kernels translate from their actual shared JavaScript source', async () => {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const { emitJavaScript } = await import('../src/translation/emit-javascript.js');
  const checked = checkProgram(parseJavaScript(embeddedRegionRulesPolicy().source, { referenceSemantics: false }));
  const rust = emitRust(checked, { arrayConsumption: true });
  for (const name of ['read_embedded_region_ancestor', 'read_embedded_region_disposition',
    'read_embedded_region_charge', 'read_embedded_region_diagnostic_limit']) assert.equal(rust.text.includes(name), true);
  assert.equal(rust.assumptions?.some(value => /array reads are in bounds/u.test(value)), false);
  const javascript = emitJavaScript(checked).text;
  assert.equal(vm.runInNewContext(javascript + '\nreadEmbeddedRegionAncestor([-1,0],[0,1],[0,0],[5,5],1,0,0,5);', {}, { timeout: 1000 }), 0);
  assert.equal(vm.runInNewContext(javascript + '\nreadEmbeddedRegionDisposition(0,5,1,0,0,5,0,2,1,5,128,4096,16777216,true);', {}, { timeout: 1000 }), 'nonConsumingEmbeddingCycle');
});
