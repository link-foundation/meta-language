import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import test from 'node:test';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { readRustLibraryItems } from '../scripts/rust-library-packaging.mjs';
import { readAsciiTextPredicateLibraryStyles } from '../scripts/ascii-text-predicate-library-style.mjs';

const interval = 'pub fn accept_rust_const_function_interval(start: f64, end: f64, nonempty: bool) -> bool {\n    (start <= end) && (!nonempty || (start < end))\n}';
const finite = 'pub fn accept_rust_const_function_finite_literal(value: f64) -> bool {\n    value.is_finite()\n}';
const replay = (source, actions) => new DecoratorSet([{ id: 'ascii-const-qualification-control', level: 'emitter', order: 0,
  when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source }).source;

test('ASCII family const styles retain both complete generated primitive declarations and every other source byte', () => {
  const source = '// complete original generated declarations\n#[must_use]\n' + interval
    + '\n\n// independent actual primitive owner\n#[must_use]\n' + finite + '\n';
  const result = readAsciiTextPredicateLibraryStyles(source);
  assert.equal(result.actions.length, 2);
  assert.equal(result.source, source.replace('pub fn accept_rust_const_function_interval', 'pub const fn accept_rust_const_function_interval')
    .replace('pub fn accept_rust_const_function_finite_literal', 'pub const fn accept_rust_const_function_finite_literal'));
  assert.equal(replay(source, result.actions), result.source);
  let inverse = result.source;
  for (const action of result.actions.toReversed()) {
    assert.equal(inverse.split(action.to).length, 2);
    inverse = inverse.replace(action.to, () => action.from);
  }
  assert.equal(inverse, source);
  assert.deepEqual(readAsciiTextPredicateLibraryStyles(result.source), { source: result.source, actions: [] });
  assert.equal(result.source.includes('#[allow('), false);
});

test('unknown owners signatures bodies duplicate declarations imports primitives and uncovered version profiles gain no qualification', () => {
  for (const source of [interval.replace('accept_rust_const_function_interval', 'unrelated'),
    interval.replace('start: f64', 'start: Custom'), interval.replace('end: f64', 'end: &f64'),
    interval.replace('-> bool', '-> Custom'), interval.replace('(start <= end)', 'other(start,end)'),
    interval.replace('!nonempty || (start < end)', 'nonempty || (start < end)'),
    interval.replace('pub fn', 'pub unsafe fn'), interval + '\n' + interval,
    finite.replace('value.is_finite()', 'value.is_nan()'), finite.replace('value: f64', 'value: Custom'),
    'pub type f64 = Custom;\n' + interval + '\n' + finite,
    'pub enum bool { Value }\n' + interval + '\n' + finite,
    'use crate::foreign::f64;\n' + interval + '\n' + finite,
    'use crate::foreign::*;\n' + interval + '\n' + finite,
  ]) assert.deepEqual(readAsciiTextPredicateLibraryStyles(source), { source, actions: [] }, source);
  const source = interval + '\n' + finite;
  for (const rustVersionRange of ['>=1.89.0 <1.100.0', '>=1.100.0', '*', '']) {
    assert.deepEqual(readAsciiTextPredicateLibraryStyles(source, { rustVersionRange }), { source, actions: [] });
  }
});

test('actual complete translated closure and both honest archive replays remain raw while normal library styles add only const', () => {
  const artifact = readProjectData(new URL('../../parity/self-translation/ascii-text-predicate-rules-native.lino', import.meta.url));
  assert.deepEqual(artifact.replays.map(row => row.format), ['text', 'binary']);
  assert.equal(artifact.librarySourceStyles.rustVersionRange, '>=1.90.0 <1.100.0');
  assert.deepEqual(artifact.librarySourceStyles.references, ['https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite']);
  for (const input of artifact.librarySourceStyles.inputs) {
    const source = readFileSync(new URL('../../' + input.path, import.meta.url), 'utf8');
    assert.equal(source, input.source);
    assert.equal(Buffer.byteLength(source), input.bytes);
    assert.equal(createHash('sha256').update(source).digest('hex'), input.sourceSha256);
  }
  const originalOwners = readRustLibraryItems(artifact.emitted.definitions.join('\n\n')).items;
  for (const name of ['accept_rust_const_function_interval', 'accept_rust_const_function_finite_literal']) {
    const owners = originalOwners.filter(owner => owner.name === name);
    assert.equal(owners.length, 1);
    assert.ok(owners[0].source.includes('pub fn ' + name + '('));
    assert.equal(owners[0].source.includes('pub const fn ' + name + '('), false, 'source qualification does not fabricate original const evidence');
  }
  const output = artifact.files.map(row => row.source).join('\n\n');
  for (const name of ['accept_rust_const_function_interval', 'accept_rust_const_function_finite_literal']) {
    assert.ok(output.includes('pub const fn ' + name + '('), 'the ordinary writer owns the styled native declaration');
    assert.ok(readFileSync(new URL('../../rust/src/translation/rust/const_function_rules.rs', import.meta.url), 'utf8').includes('pub const fn ' + name + '('),
      'the existing independent compiled policy retains its already-const primitive counterpart');
  }
  for (const replay of artifact.replays) assert.equal(replay.emittedTextSha256,
    artifact.replays[0].emittedTextSha256, 'both complete original archives retain the same raw emitted source');
});
