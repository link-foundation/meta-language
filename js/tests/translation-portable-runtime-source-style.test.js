import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { STRING_PRELUDE } from '../src/translation/string-runtime.js';
import { readPortableRuntimeSourceStyles } from '../scripts/portable-runtime-source-style.mjs';

function readArrayRuntime() {
  const source = readFileSync(new URL('../src/translation/emit-rust.js', import.meta.url), 'utf8');
  const literal = /const ARRAY_PRELUDE = (`[\s\S]*?`);/u.exec(source);
  assert.ok(literal, 'the actual complete array runtime remains the source of the test');
  return vm.runInNewContext(literal[1], {}, { timeout: 1000 });
}

function readIntegerRuntime(name = 'PRELUDE') {
  const source = readFileSync(new URL('../src/translation/emit-rust.js', import.meta.url), 'utf8');
  const literal = new RegExp('const ' + name + ' = (`[\\s\\S]*?`);', 'u').exec(source);
  assert.ok(literal);
  return vm.runInNewContext(literal[1], {}, { timeout: 1000 });
}

function readRuntimeNames(source) {
  const { tokens } = tokenize(source, 'Rust');
  return tokens.flatMap((token, index) => token.kind === 'identifier' && token.value === 'fn' ? [tokens[index + 1].value] : []);
}

function replayRuntimeStyles(source, actions) {
  return new DecoratorSet([{ id: 'runtime-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source }).source;
}

test('portable runtime styles preserve complete original helpers and replay as recorded idempotent actions', () => {
  const before = readArrayRuntime() + '\n\n' + STRING_PRELUDE;
  const { source, actions } = readPortableRuntimeSourceStyles(before);
  assert.equal(actions.length, 2);
  assert.equal(replayRuntimeStyles(before, actions), source);
  assert.deepEqual(readRuntimeNames(source), readRuntimeNames(before));
  assert.deepEqual(readPortableRuntimeSourceStyles(source), { source, actions: [] });
  assert.match(source, /map_or_else\(\|\| panic!\("array index out of range"\), Clone::clone\)/u);
  assert.match(source, /const fn integer\(value: f64\)/u);
  assert.match(source, /pub const fn integer\(value: f64\)/u, 'the ml_string integer helper is covered separately from the private array helper');
  assert.match(source, /pub const fn from_units\(units: Vec<u16>\)/u);
  assert.match(source, /pub const fn length\(&self\)/u);
  assert.match(source, /let Some\(separator\) = separator else \{ return vec!\[self\.clone\(\)\]; \};/u);
  assert.match(source, /\.is_some_and\(\|second\|/u);
  assert.match(source, /char::from_u32\(u32::from\(unit\)\)\.is_some_and\(\|c\|/u);
  assert.doesNotMatch(source, /#\[(?:allow|expect)\(/u);
  // Numeric index domains, casts and array equality callbacks are untouched.
  for (const body of [
    'index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0',
    '(start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)',
    'if !all || (search.0.is_empty() && index == self.0.len()) { break; }',
    'out.push(error.unpaired_surrogate());',
    'value.trunc().rem_euclid(4294967296.0) as u32',
  ]) assert.ok(source.includes(body), body);
});

test('runtime source qualification refuses changed storage dependencies and leaves translated declarations untouched', () => {
  for (const changed of [
    STRING_PRELUDE.replace('pub struct Text(pub Vec<u16>);', 'pub struct Text(pub Vec<u32>);'),
    STRING_PRELUDE.replace('if value.is_nan() { 0.0 } else { value.trunc() }', 'if value.is_nan() { value } else { value.trunc() }'),
    STRING_PRELUDE.replace('let count = integer(count);', 'let count = count;'),
    STRING_PRELUDE.replace('let length = integer(length).max(0.0);', 'let length = length;'),
    STRING_PRELUDE.replace('536870888.0', '536870889.0'),
    STRING_PRELUDE.replace('pub mod ml_string', 'pub mod unrelated'),
  ]) assert.deepEqual(readPortableRuntimeSourceStyles(changed), { source: changed, actions: [] });
  const external = 'pub fn integer(value:f64)->f64{if value.is_nan(){0.0}else{value.trunc()}}\n'
    + 'pub mod unrelated { pub fn from_units(units:Vec<u16>)->Self{Self(units)} }';
  assert.deepEqual(readPortableRuntimeSourceStyles(external), { source: external, actions: [] });
  const array = readArrayRuntime().replace('panic!("array index out of range")', 'panic!("different panic")');
  const result = readPortableRuntimeSourceStyles(array).source;
  assert.match(result, /match index\.and_then/u, 'a different panic is never replaced by the known array failure');
  assert.match(result, /panic!\("different panic"\)/u);
  const shadowed = readArrayRuntime().replace('pub mod ml_array {', 'pub mod ml_array { type f64 = Custom;');
  assert.deepEqual(readPortableRuntimeSourceStyles(shadowed), { source: shadowed, actions: [] });
});

test('surrogate styles retain ordered guards and exact arithmetic while using infallible conversions', () => {
  const { source } = readPortableRuntimeSourceStyles(STRING_PRELUDE);
  const first = source.indexOf('if (0xd800..=0xdbff).contains(&first)');
  const second = source.indexOf('&& let Some(&second) = self.0.get(index + 1)', first);
  const third = source.indexOf('&& (0xdc00..=0xdfff).contains(&second)', second);
  assert.ok(first >= 0 && second > first && third > second, 'the second unit is read only after the high-surrogate guard');
  assert.ok(source.includes('return Some(f64::from((u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000));'));
  assert.ok(source.includes('Some(f64::from(first))'));
  assert.doesNotMatch(source, /(?:first|second|unit) as u32|Some\(first as f64\)/u);
  for (const first of [0, 0xd7ff, 0xd800, 0xdbff, 0xdc00, 0xffff]) {
    for (const second of [0, 0xdbff, 0xdc00, 0xdfff, 0xe000]) {
      const value = String.fromCharCode(first, second).codePointAt(0);
      const paired = first >= 0xd800 && first <= 0xdbff && second >= 0xdc00 && second <= 0xdfff;
      assert.equal(value, paired ? (first - 0xd800) * 1024 + second - 0xdc00 + 0x10000 : first);
    }
  }
});

test('runtime assertions retain panic payloads and the exact normalized Number failure domains', () => {
  const { source } = readPortableRuntimeSourceStyles(STRING_PRELUDE);
  assert.ok(source.includes('assert!(count >= 0.0 && count.is_finite(), "Invalid count value: {count}");'));
  assert.ok(source.includes('assert!(size <= 536870888.0, "Invalid string length");'));
  assert.ok(source.includes('assert!(length <= 536870888.0, "Invalid string length");'));
  assert.ok(source.includes('let count = integer(count);'));
  assert.ok(source.includes('let length = integer(length).max(0.0);'));
  for (const raw of [-Infinity, -1, -0, 0, 0.5, 1, 536870888, Number.MAX_VALUE, Infinity, NaN]) {
    const count = Number.isNaN(raw) ? 0 : Math.trunc(raw);
    assert.equal(count < 0 || !Number.isFinite(count), !(count >= 0 && Number.isFinite(count)));
    if (count >= 0 && Number.isFinite(count)) for (const length of [0, 1, 2, 65535]) {
      const size = count * length;
      assert.equal(size > 536870888, !(size <= 536870888));
    }
    const length = Math.max(count, 0);
    assert.equal(length > 536870888, !(length <= 536870888));
  }
});

test('runtime styling preserves opaque literals, unrelated modules and exact documentation context', () => {
  const carried = 'const CARRIED:&str=r####"' + STRING_PRELUDE.slice(STRING_PRELUDE.indexOf('pub mod')) + '"####;\n';
  const before = carried + STRING_PRELUDE;
  const result = readPortableRuntimeSourceStyles(before);
  assert.equal(result.actions.length, 1, 'a textual module collision retains the entire source context');
  assert.equal(result.actions[0].from, before);
  assert.ok(result.source.startsWith(carried));
  assert.equal(replayRuntimeStyles(before, result.actions), result.source);
  const doc = '/// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.';
  const math = 'pub mod ml_math {\n' + doc + '\npub fn sign(value:f64)->f64{value}\n}';
  const styled = readPortableRuntimeSourceStyles(math).source;
  assert.equal(styled, math.replace('where f64::signum', 'where `f64::signum`'));
  assert.deepEqual(readPortableRuntimeSourceStyles(doc + '\n' + 'pub fn sign(value:f64)->f64{value}'),
    { source: doc + '\n' + 'pub fn sign(value:f64)->f64{value}', actions: [] });
  for (const opaque of [
    'custom!{' + STRING_PRELUDE + '}',
    'custom ! [' + STRING_PRELUDE + ']',
    'outer!(inner!{' + STRING_PRELUDE + '})',
    'macro_rules! carried { () => {' + STRING_PRELUDE + '}; }',
    'macro_rules ! carried { () => {' + STRING_PRELUDE + '}; }',
    '#[carried(' + STRING_PRELUDE + ')] struct Holder;',
    '#![carried(' + STRING_PRELUDE + ')]',
  ]) {
    assert.deepEqual(readPortableRuntimeSourceStyles(opaque), { source: opaque, actions: [] });
    const mixed = readPortableRuntimeSourceStyles(opaque + '\n' + STRING_PRELUDE);
    assert.ok(mixed.source.startsWith(opaque + '\n'));
    assert.notEqual(mixed.source, opaque + '\n' + STRING_PRELUDE, 'the real prelude after opaque tokens is still styled');
    assert.equal(replayRuntimeStyles(opaque + '\n' + STRING_PRELUDE, mixed.actions), mixed.source);
  }
  const hiddenDoc = 'pub mod ml_math { unknown!{\n' + doc + '\n} }';
  assert.deepEqual(readPortableRuntimeSourceStyles(hiddenDoc), { source: hiddenDoc, actions: [] });
  const numeric = readIntegerRuntime('NUMBER_PRELUDE'), documented = readPortableRuntimeSourceStyles(numeric).source;
  assert.equal(documented, numeric.replace('ECMAScript Number::toString:', 'ECMAScript `Number::toString`:')
    .replace('/// SameValue, as Object.is and assert.strictEqual compare:', '/// `SameValue`, as `Object.is` and `assert.strictEqual` compare:')
    .replace('pub fn same_value', 'pub const fn same_value'));
  assert.deepEqual(readPortableRuntimeSourceStyles(documented), { source: documented, actions: [] });
  const changedSameValue = numeric.replace('a.to_bits() == b.to_bits()', 'a == b');
  const changedNumeric = readPortableRuntimeSourceStyles(changedSameValue).source;
  assert.ok(changedNumeric.includes('pub fn same_value'));
  assert.ok(changedNumeric.includes('a == b'));
  const mathRuntime = readIntegerRuntime('MATH_PRELUDE'), ranged = readPortableRuntimeSourceStyles(mathRuntime).source;
  assert.equal(ranged, mathRuntime.replace('if x < 0.0 && x >= -0.5 {', 'if (-0.5..0.0).contains(&x) {')
    .replace('where f64::signum', 'where `f64::signum`'));
  assert.deepEqual(readPortableRuntimeSourceStyles(ranged), { source: ranged, actions: [] });
  const changedRound = mathRuntime.replace('if x < 0.0 && x >= -0.5 {', 'if x < 0.0 && x > -0.5 {');
  assert.equal(readPortableRuntimeSourceStyles(changedRound).source, changedRound.replace('where f64::signum', 'where `f64::signum`'));
  for (const value of [-Infinity, -1, -0.5, -0.25, -0, 0, 0.25, Infinity, NaN]) {
    assert.equal(value < 0 && value >= -0.5, value >= -0.5 && value < 0);
  }
});

test('integer runtime styles retain complete magnitude arithmetic, decimal layout and source panic messages', () => {
  const before = readIntegerRuntime(), result = readPortableRuntimeSourceStyles(before);
  assert.equal(result.actions.length, 1);
  assert.equal(replayRuntimeStyles(before, result.actions), result.source);
  assert.deepEqual(readRuntimeNames(result.source), readRuntimeNames(before));
  assert.deepEqual(readPortableRuntimeSourceStyles(result.source), { source: result.source, actions: [] });
  assert.ok(result.source.includes('for limb in &mut remainder'));
  assert.ok(result.source.includes("text.strip_prefix('-').map_or((false, text), |rest| (true, rest))"));
  assert.ok(result.source.includes('pub const fn is_zero'));
  assert.ok(result.source.includes('pub const fn is_negative'));
  assert.ok(result.source.includes('assert!(!value.is_negative(), "{message}");'));
  assert.ok(result.source.includes('std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}")).expect("writing to a String cannot fail");'));
  assert.ok(result.source.includes('for chunk in chunks.iter().rev()'));
  assert.ok(result.source.includes('text.push_str(&chunks.pop().unwrap_or(0).to_string());'));
  for (const expression of ['*limb = (*limb << 1) | carry;', 'let product = u64::from(out[i + j]) + u64::from(*x) * u64::from(*y) + carry;', 'quotient[bit / 32] |= 1 << (bit % 32);']) {
    assert.ok(result.source.includes(expression), expression);
  }
  for (const changed of [before.replace('"{chunk:09}"', '"{chunk:08}"'), before.replace('pub struct Big', 'pub struct Other'), before.replace('panic!("{message}")', 'panic!("different message")')]) {
    assert.deepEqual(readPortableRuntimeSourceStyles(changed), { source: changed, actions: [] });
  }
});
