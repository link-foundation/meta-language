import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { runInNewContext } from 'node:vm';

import test from 'node:test';
import { checkProgram, coerce } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { renderPortableTypeIdentity, acceptOptionalAbsence, mergeOptionalAbsence, readRustOptionalPrelude, renderRustCompoundType, renderRustReferenceType, wrapPortableReferenceIdentity } from '../src/translation/structured-type-rules.js';
import { BOOL, option, sameType, typeKey } from '../src/translation/types.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { parseRust } from '../src/translation/rust.js';
import { tokenize } from '../src/translation/lexer.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import * as decisions from '../src/translation/structured-type-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/translation-structured-types.json', import.meta.url));

test('structural identities preserve named fields, nesting and UTF-16 field order', () => {
  for (const entry of fixture.cases) assert.equal(typeKey(entry.type), entry.identity, entry.name);
  const record = fixture.cases.find(({ name }) => name === 'sorted record').type;
  assert.equal(sameType(record, { ...record, fields: [...record.fields].reverse() }), true);
  assert.equal(sameType(record, { ...record, open: true }), false);
  assert.deepEqual(option(option(BOOL, 'null'), 'undefined'), { kind: 'option', element: BOOL, absence: 'both' });
  assert.deepEqual(option(option(BOOL, 'null'), 'null'), { kind: 'option', element: BOOL, absence: 'null' });
  const sharedArray = fixture.cases.find(({ name }) => name === 'reference array').type;
  assert.equal(sameType(sharedArray, { ...sharedArray, reference: false }), false);
  assert.equal(wrapPortableReferenceIdentity('array:bool', true), 'reference:array:bool');
  assert.equal(wrapPortableReferenceIdentity('array:bool', false), 'array:bool');
});

test('absence admission and normalization retain null and undefined separately', () => {
  for (const expected of ['null', 'undefined', 'both']) for (const actual of ['null', 'undefined', 'both']) {
    assert.equal(acceptOptionalAbsence(expected, actual), expected === 'both' || expected === actual);
    assert.equal(mergeOptionalAbsence(expected, actual), expected === actual ? expected : 'both');
  }
  const none = { k: 'none', absence: 'undefined', type: option(BOOL, 'undefined') };
  assert.equal(coerce(none, option(BOOL, 'both'), 'JavaScript').type.absence, 'both');
  assert.throws(() => coerce(none, option(BOOL, 'null'), 'JavaScript'), /expected null absence/);
});

test('compound signatures are emitted through shared rules with exact target source', () => {
  for (const entry of fixture.emissionCases) assert.equal(emitRust(checkProgram(entry.surface)).text, entry.rust, entry.name);
  assert.equal(readRustOptionalPrelude(), fixture.optionalPrelude);
  assert.equal(renderRustCompoundType('tuple', '', []), '()');
  assert.equal(renderRustCompoundType('record', '', ['bool']), '(bool,)');
  assert.equal(renderRustCompoundType('function', 'bool', []), 'std::rc::Rc<dyn Fn() -> bool>');
  assert.equal(renderRustCompoundType('cell', '', ['bool']), 'std::rc::Rc<std::cell::RefCell<bool>>');
  assert.equal(renderRustReferenceType('Vec<bool>', true), 'std::rc::Rc<std::cell::RefCell<Vec<bool>>>');
  assert.equal(renderRustReferenceType('Vec<bool>', false), 'Vec<bool>');
});

test('data traits inspect every nested compound and reject callable structural fields', () => {
  for (const entry of fixture.traitCases) {
    assert.equal(emitRust(checkProgram(entry.surface)).text, entry.rust, entry.name);
    const derive = entry.totalEquality ? '#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Box' : '#[derive(Clone, Debug, PartialEq)]\npub enum Box';
    assert.ok(entry.rust.includes(derive), entry.name);
  }
  assert.throws(() => emitRust(checkProgram(fixture.callableData)), /callable data field/);
});

test('pure targets refuse shared reference signatures until their state model is represented', () => {
  for (const entry of fixture.emissionCases.filter(({ name }) => name.startsWith('reference ') || name.includes('cell'))) {
    for (const emit of [emitLean, emitRocq]) {
      assert.throws(() => emit(checkProgram(entry.surface)), /mutable structural value: shared reference state needs an explicit target state model/, entry.name);
    }
  }
});

test('shared type identities retain distinct unresolved inference variables', () => {
  for (const entry of fixture.inferenceIdentities) assert.equal(renderPortableTypeIdentity(entry.kind, entry.detail, entry.children), entry.identity);
  assert.equal(typeKey({ kind: 'var', id: 0 }), 'var:0');
  assert.equal(typeKey({ kind: 'var', id: 1 }), 'var:1');
  assert.equal(sameType({ kind: 'array', element: { kind: 'var', id: 0 } }, { kind: 'array', element: { kind: 'var', id: 1 } }), false);
});

function rustFunctions(source) {
  const tokens = tokenize(source, 'Rust').tokens;
  const functions = new Map();
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'fn') continue;
    const start = index;
    const name = tokens[index + 1].value;
    while (!(tokens[index].kind === 'punct' && tokens[index].value === '{')) index += 1;
    const signatureEnd = index;
    let depth = 1;
    while (depth) {
      index += 1;
      if (tokens[index].kind !== 'punct') continue;
      if (tokens[index].value === '{') depth += 1;
      if (tokens[index].value === '}') depth -= 1;
    }
    functions.set(name, { source: source.slice(tokens[start].start, tokens[index].end),
      signature: tokens.slice(start, signatureEnd).map(({ kind, value, suffix }) => [kind, value, suffix]),
      body: tokens.slice(signatureEnd, index + 1).map(({ kind, value, suffix }) => [kind, value, suffix]) });
  }
  return functions;
}

test('reviewed library generation keeps every function and shares equivalent array helper interfaces', () => {
  execFileSync(process.execPath, [new URL('../scripts/generate-structured-type-rules.mjs', import.meta.url).pathname, '--check'], { timeout: 20_000 });
  const source = readFileSync(new URL('../src/translation/structured-type-rules.js', import.meta.url), 'utf8');
  const translated = emitRust(checkProgram(parseJavaScript(source)));
  const generated = readFileSync(new URL('../../rust/src/translation/structured_type_rules.rs', import.meta.url), 'utf8');
  assert.deepEqual([...rustFunctions(generated).keys()], [...rustFunctions(translated.definitions.join('\n')).keys()]);
  const originalPrelude = translated.preludes.find(prelude => prelude.startsWith('/// Reads of JavaScript arrays'));
  const original = rustFunctions(execFileSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
    input: originalPrelude, encoding: 'utf8', timeout: 10_000,
  }));
  const provider = rustFunctions(readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8'));
  for (const [name, helper] of original) {
    assert.deepEqual(provider.get(name)?.signature, helper.signature, name);
    // The existing at() style performs the same one clone or one panic.
    // All other complete helper bodies are identical after lexical reload.
    if (name !== 'at') assert.deepEqual(provider.get(name)?.body, helper.body, name);
  }
  assert.ok(generated.includes('pub use crate::translation::frontend_rules::ml_array;'));
});

test('borrowed scalar structural rules preserve actual generated Rust decisions after semantic replay', () => {
  const generated = readFileSync(new URL('../../rust/src/translation/structured_type_rules.rs', import.meta.url), 'utf8');
  const functions = rustFunctions(generated);
  const selected = ['mergeOptionalAbsence', 'acceptTypeIdentity', 'acceptOptionalAbsence',
    'readRustOptionalPrelude', 'wrapPortableReferenceIdentity', 'renderRustReferenceType'];
  const rustName = name => name.replace(/[A-Z]/gu, character => '_' + character.toLowerCase());
  const replay = {};
  const source = selected.map(name => functions.get(rustName(name)).source).join('\n') + '\nfn main() {}\n';
  runInNewContext(emitJavaScript(checkProgram(parseRust(source))).text, replay, { timeout: 1_000 });
  const values = ['', 'null', 'undefined', 'both', 'array:bool', 'Vec<bool>', '空🦀'];
  for (const left of values) for (const right of values) {
    for (const name of selected.slice(0, 3)) assert.equal(replay[rustName(name)](left, right), decisions[name](left, right), name);
  }
  for (const value of values) for (const reference of [false, true]) {
    for (const name of selected.slice(4)) assert.equal(replay[rustName(name)](value, reference), decisions[name](value, reference), name);
  }
  assert.equal(replay.read_rust_optional_prelude(), decisions.readRustOptionalPrelude());
});
