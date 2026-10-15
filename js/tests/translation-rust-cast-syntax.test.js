import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { parseRust } from '../src/translation/rust.js';
import { acceptRustCastFollowingToken, readRustCastSyntaxMessage } from '../src/translation/rust-cast-syntax-rules.js';

const policy = readProjectData(new URL('../src/translation/data/rust-cast-syntax.lino', import.meta.url));
const fixture = readProjectData(new URL('../../parity/fixtures/rust-array-source.lino', import.meta.url));
const source = body => `fn below(value:f64,limit:f64)->bool { ${body} } fn main(){}`;
const sha256 = text => createHash('sha256').update(text).digest('hex');

test('the linked cast rule distinguishes a bare type path from an already grouped target', () => {
  assert.equal(policy.ambiguousFollowingToken, '<');
  assert.equal(policy.parenthesizedTypeStart, '(');
  assert.equal(readRustCastSyntaxMessage(), policy.reason);
  for (const token of ['<', '<=', '>', '>=', '==', '!=', ')', '}', 'as', '']) {
    assert.equal(acceptRustCastFollowingToken(token, false), token !== '<', token);
    assert.equal(acceptRustCastFollowingToken(token, true), true, token);
  }
});

test('both general and length casts refuse actual ungrouped less-than source at its original token', () => {
  for (const input of [
    source('value as f64 < limit'),
    source('value as f64 /*😀 < trivia */ < limit'),
  ]) {
    const at = input.lastIndexOf('<');
    const typeStart = input.indexOf('as f64') + 3;
    assert.throws(() => parseRust(input), error => error.kind === 'unsupported'
      && error.reason === 'type f64<…>: generic types are outside the portable core'
      && error.span.start === typeStart && error.span.end === at);
    assert.throws(() => parseRust(input, { primitiveOwnerModel: 'standard primitive Rust binary64 owner 1' }), error => error.kind === 'syntax'
      && error.reason === policy.reason && error.span.start === at && error.span.end === at + 1);
  }
  const input = policy.sourceCorrection.originalSource;
  const at = input.lastIndexOf('<');
  assert.throws(() => parseRust(input), error => error.kind === 'syntax'
    && error.reason === policy.reason && error.span.start === at && error.span.end === at + 1);
});

test('cast expression and target grouping retain executable source-free comparisons in both formats', () => {
  const cases = [
    ['(value as f64) < limit', (value, limit) => value < limit],
    ['value as (f64) < limit', (value, limit) => value < limit],
    ['value as ((f64)) < limit', (value, limit) => value < limit],
    ['value as f64 <= limit', (value, limit) => value <= limit],
    ['value as f64 >= limit', (value, limit) => value >= limit],
    ['value as f64 > limit', (value, limit) => value > limit],
  ];
  const inputs = [[0, 1], [7, 5], [4294967295, Infinity], [-0, -0], [7, NaN]];
  for (const [body, expected] of cases) {
    const packet = createTypedProgramLinks(source(body), 'Rust');
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const context = {};
      runInNewContext(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, context, { timeout: 1000 });
      for (const [value, limit] of inputs) assert.equal(context.below(value, limit), expected(value, limit), body);
    }
  }
});

test('the exact historical array source remains a negative and its original positive observations use only the recorded decorator', () => {
  const correction = policy.sourceCorrection;
  const original = correction.originalSource;
  assert.equal(sha256(original), correction.originalSourceSha256);
  assert.equal(correction.originalSourceSha256, '73a6ec2aba8875929a9d2a1f50e12b27eb22b75783df8a50a5feac8839b48ba2');
  const archived = fixture.invalid.find(entry => entry.id === correction.invalidId);
  assert.equal(archived.source, original);
  assert.equal(archived.kind, 'syntax');
  assert.equal(archived.reason, policy.reason);
  assert.deepEqual(archived.span, { start: original.lastIndexOf('<'), end: original.lastIndexOf('<') + 1 });
  const positive = fixture.cases.find(entry => entry.id === correction.caseId);
  const styles = DecoratorSet.fromLino(readFileSync(new URL('../../' + correction.decoratorFile, import.meta.url), 'utf8'));
  assert.equal(styles.ids().length, 1);
  assert.equal(styles.ids()[0], correction.decoratorId);
  assert.equal(positive.source, styles.decorate('transformation', { fixture: correction.fixtureId, case: correction.caseId, format: 'Rust', source: original }).source);
  assert.equal(positive.sourceCorrection.originalSource, original);
  assert.equal(positive.sourceCorrection.originalSourceSha256, sha256(original));
  assert.equal(positive.sourceCorrection.correctedSourceSha256, sha256(positive.source));
  assert.deepEqual(positive.inputs, [[], [1], [1, 2], [1, 2, 3]]);
  assert.deepEqual(positive.expected, [true, true, true, false]);
  assert.equal(positive.name, 'bounded');
  assert.equal(fixture.cases.length, 6);
  assert.equal(fixture.invalid.length, 22);
});
