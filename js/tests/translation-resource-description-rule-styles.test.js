import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import * as decisions from '../src/resource-description-rules.js';
import { DecoratorSet } from '../src/decorators.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';
import { parseRust } from '../src/translation/rust.js';
import { reviewResourceDescriptionStyles } from '../scripts/resource-description-reviewed-styles.mjs';

const source = readFileSync(new URL('../src/resource-description-rules.js', import.meta.url), 'utf8');
const units = text => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
const scalarNames = ['resourceDescriptionLetter', 'resourceDescriptionDigit',
  'resourceDescriptionHexadecimal', 'acceptResourceDescriptionIdentifierCharacter'];
const rustName = name => name.replace(/[A-Z]/gu, character => `_${character.toLowerCase()}`);

test('every shared resource description decision survives checked semantic emission', () => {
  const replay = {};
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  runInNewContext(emitJavaScript(checked).text, replay, { timeout: 1_000 });
  const operations = new Set();
  const check = (operation, ...args) => {
    assert.equal(replay[operation](...args), decisions[operation](...args), `${operation}: ${JSON.stringify(args)}`);
    operations.add(operation);
  };
  for (const index of [-1, 0, 1, 2, 3, 0.5, NaN, Infinity, 9007199254740992]) {
    check('resourceDescriptionUnit', [10, 20, 30], index);
  }
  for (const code of [-1, 0, 33, 47, 48, 57, 58, 64, 65, 70, 71, 90, 91, 96, 97, 102, 103,
    122, 123, 159, 160, 55295, 55296, 57343, 57344, 63743, 63744, 64975, 64976, 65007,
    65008, 65519, 65520, 65533, 65534, 65535, 65536, 983037, 983038, 983039, 983040,
    1114109, 1114110, 1114111, 1114112]) {
    for (const operation of scalarNames) for (const query of [false, true]) check(operation, code, query);
  }
  for (const value of [[], [0], [65535], [55296, 56320], [56319, 57343], [55296], [56320],
    [55296, 65], [65536], [-1], [0.5], [NaN], units('空🦀')]) check('acceptResourceDescriptionUnicode', value);
  for (const value of ['urn:a', 'scheme:', 'https://example.test/空?x=🦀#part', 'urn:a%20b',
    'relative', 'urn:a b', 'urn:a%GG', 'https://user@other@example.test/a', 'http://[::1]/',
    'urn:a#b#c', 'http://example.test:abc/', 'urn:a\ud800', 'urn:a\uE000', 'urn:a?\uE000']) {
    check('acceptResourceDescriptionIdentifier', units(value));
  }
  for (const value of ['en', 'en-US', 'zh-Hant-TW', 'de-CH-1901', 'en-a-example-x-private',
    'x-private', '', 'e', 'en-', 'en-a', 'en-a-one-a-two', 'sl-rozaj-rozaj', 'en-💬']) {
    for (const grandfathered of [false, true]) check('acceptResourceDescriptionLanguage', units(value), grandfathered);
  }
  for (const leftKind of ['identifier', 'blank', 'literal', 'unknown']) {
    for (const rightKind of ['identifier', 'blank', 'literal', 'unknown']) {
      for (const left of [[], ['a'], ['a', 'b'], ['a', 'b', 'c']]) {
        for (const right of [left, ['different'], ['a', 'changed'], ['a', 'b', 'changed']]) {
          check('sameResourceDescriptionTerm', leftKind, left, rightKind, right);
        }
      }
    }
  }
  assert.deepEqual([...operations].sort(), Object.keys(decisions).sort());
});

test('generated scalar Rust replays Unicode boundaries and query-only ranges through JavaScript', () => {
  const generated = readFileSync(new URL('../../rust/src/resource_description_rules/part_0.rs', import.meta.url), 'utf8');
  const tokens = tokenize(generated, 'Rust').tokens;
  const selected = new Set(scalarNames.map(rustName));
  const functions = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].value !== 'fn' || !selected.has(tokens[index + 1]?.value)) continue;
    const start = index;
    while (tokens[index].value !== '{') index += 1;
    let depth = 1;
    while (depth) {
      index += 1;
      if (tokens[index].value === '{') depth += 1;
      if (tokens[index].value === '}') depth -= 1;
    }
    // Flatten only the generated partition's parent-module qualification. The
    // portable Rust frontend currently cannot reparse the array/helper library.
    functions.push(generated.slice(tokens[start].start, tokens[index].end).replaceAll('super::', 'crate::'));
  }
  assert.equal(functions.length, scalarNames.length);
  const replay = {};
  runInNewContext(emitJavaScript(checkProgram(parseRust(`${functions.join('\n')}\nfn main() {}\n`))).text,
    replay, { timeout: 1_000 });
  // Every ASCII code, every BMP code and the supplementary-plane range edges
  // exercise both query policies and the original interior Boolean precedence.
  for (const code of [...Array.from({ length: 65538 }, (_, index) => index - 1),
    983036, 983037, 983038, 983039, 983040, 983041, 1048573, 1048574, 1048575,
    1048576, 1114108, 1114109, 1114110, 1114111, 1114112]) {
    for (const operation of scalarNames) for (const query of [false, true]) {
      assert.equal(replay[rustName(operation)](code, query), decisions[operation](code, query), `${operation}(${code}, ${query})`);
    }
  }
});

test('numeric style records preserve emitted values, suffixes and the existing declaration inventory', () => {
  const translated = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })));
  const raw = [...translated.preludes, ...translated.definitions].join('\n\n');
  const styles = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/resource-description-rules-decorators.lino', import.meta.url), 'utf8'));
  const numericActions = styles.decorators.flatMap(decorator => decorator.actions)
    .filter(action => /^[0-9]{6,}/u.test(action.from));
  assert.equal(numericActions.length, 5);
  for (const action of numericActions) {
    assert.equal(action.to.replaceAll('_', ''), action.from);
    const before = tokenize(raw, 'Rust').tokens;
    const after = tokenize(raw.replaceAll(action.from, action.to), 'Rust').tokens;
    assert.deepEqual(after.map(({ kind, value, suffix }) => ({ kind, value, suffix })),
      before.map(({ kind, value, suffix }) => ({ kind, value, suffix })));
  }
  const declarations = [...raw.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
  const generated = [0, 1, 2].map(index => readFileSync(new URL(`../../rust/src/resource_description_rules/part_${index}.rs`, import.meta.url), 'utf8'));
  assert.deepEqual(generated.flatMap(text => [...text.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1])), declarations);
  for (const text of generated) assert.ok(text.split('\n').length <= 900);
});

test('owned read-only arrays remain available across repeated continuation calls', () => {
  // One syntactic use still repeats after a continue or an implicit back edge.
  // Exercise all three repetition forms without invoking a native compiler.
  for (const body of [
    'loop { let result = consume(starts.clone()); if result { return true; } continue; }',
    'while pending() { if consume(starts.clone()) { return true; } } false',
    'for _ in positions() { if consume(starts.clone()) { return true; } } false',
  ]) {
    const original = 'pub fn compare(starts: Vec<f64>) -> bool { ' + body + ' }';
    const reviewed = reviewResourceDescriptionStyles(original, value => value).source;
    assert.ok(reviewed.includes('consume(starts.clone())'), body);
  }
  const terminal = 'pub fn finish(starts: Vec<f64>) -> Vec<f64> { starts.clone() }';
  assert.equal(reviewResourceDescriptionStyles(terminal, value => value).source,
    'pub fn finish(starts: Vec<f64>) -> Vec<f64> { starts }');

  const generated = readFileSync(new URL('../../rust/src/resource_description_rules/part_2.rs', import.meta.url), 'utf8');
  assert.match(generated, /ml_accept_resource_description_language_loop22\(\s*units,\s*starts\.clone\(\),/u);
  // Its exhausted continuation returns its two arrays; this path cannot loop.
  assert.match(generated, /MlAcceptResourceDescriptionLanguageLoop11Done\(starts, lengths\)/u);

  const replay = {};
  runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source, { referenceSemantics: false }))).text,
    replay, { timeout: 1_000 });
  for (const [value, expected] of [
    ['en-US', true],
    ['en-419', true],
    ['en-4A9', false],
    ['en-U1', false],
    ['sl-rozaj-biske-nedis', true],
    ['sl-rozaj-biske-biske', false],
    ['sl-rozaj-biske-ROZAJ', false],
    ['sl-rozaj-biske-NEDIS', true],
    ['sl-rozaj-biske-1994', true],
    ['sl-rozaj-biske-1994-1994', false],
  ]) {
    const input = units(value), original = [...input];
    assert.equal(decisions.acceptResourceDescriptionLanguage(input, false), expected, value);
    assert.equal(replay.acceptResourceDescriptionLanguage(input, false), expected, value);
    assert.deepEqual(input, original, value + ': ordered read-only input');
  }
});

test('reviewed Boolean factoring preserves scalar boundaries and adjacent branch evaluation order', () => {
  const original = `
    fn transition(state: f64, kind: f64) -> bool {
      ((state == 0f64) && (kind == 1f64)) || ((state == 0f64) && (kind == 2f64))
        || ((state == 2f64) && (kind == 1f64)) || ((state == 2f64) && (kind == 2f64))
        || ((state == 2f64) && (kind == 3f64))
    }
    fn first(code: f64) -> bool { code == 1f64 }
    fn second(code: f64) -> bool { code == 2f64 }
    fn adjacent(code: f64) -> f64 {
      if first(code) { 1f64 } else if second(code) { 1f64 } else { 0f64 }
    }
    fn main() {}
  `;
  const reviewed = reviewResourceDescriptionStyles(original, value => value).source;
  assert.notEqual(reviewed, original);
  assert.equal(reviewResourceDescriptionStyles(reviewed, value => value).source, reviewed);
  assert.equal(tokenize(reviewed, 'Rust').tokens.filter(token => token.raw === 'state').length, 3);
  const replay = value => {
    const context = {};
    runInNewContext(emitJavaScript(checkProgram(parseRust(value))).text, context, { timeout: 1_000 });
    return context;
  };
  const before = replay(original), after = replay(reviewed);
  const boundaries = [-Infinity, -1, -0, 0, 0.5, 1, 2, 3, 4, Infinity, NaN];
  for (const state of boundaries) for (const kind of boundaries) {
    assert.equal(after.transition(state, kind), before.transition(state, kind), `state=${state}, kind=${kind}`);
  }
  for (const first of [false, true]) for (const second of [false, true]) {
    const observations = [before, after].map(context => {
      const calls = [];
      context.first = code => { calls.push(['first', code]); return first; };
      context.second = code => { calls.push(['second', code]); return second; };
      return { result: context.adjacent(7), calls };
    });
    assert.deepEqual(observations[1], observations[0]);
    assert.deepEqual(observations[0].calls.map(([name]) => name), first ? ['first'] : ['first', 'second']);
  }
  const binding = 'pub fn binding(value: Option<f64>) -> f64 { if let Some(first) = value { first } else if let Some(first) = value { first } else { 0f64 } }';
  assert.equal(reviewResourceDescriptionStyles(binding, value => value).source, binding);
  const overloaded = 'pub fn other(state: Other, kind: Other) -> bool { (state == 0f64 && kind == 1f64) || (state == 0f64 && kind == 2f64) }';
  assert.equal(reviewResourceDescriptionStyles(overloaded, value => value).source, overloaded);
  const temporary = 'pub fn temporary() -> bool { if holder().check() { true } else if other().check() { true } else { false } }';
  assert.equal(reviewResourceDescriptionStyles(temporary, value => value).source, temporary);
});
