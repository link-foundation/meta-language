import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {DecoratorSet} from '../src/decorators.js';
import {tokenize} from '../src/translation/lexer.js';
import {readPortableNumberSourceStyles} from '../scripts/portable-number-source-style.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';

function replay(source, result) {
  const decorators = new DecoratorSet(result.actions.length ? [{id:'numeric-style-control', level:'emitter', order:0,
    when:[['format','Rust']], actions:result.actions}] : []);
  assert.equal(DecoratorSet.fromLino(decorators.toLino()).decorate('emitter', {format:'Rust', source}).source, result.source);
  assert.deepEqual(readPortableNumberSourceStyles(result.source), {source:result.source, actions:[]});
}

test('portable Number styles preserve exact equality and negated unordered comparisons with original operand occurrences', () => {
  const source = `pub fn fact(values: &[f64], index: f64) -> f64 { values[index as usize] }
pub fn count(accepted: f64, total: f64) -> bool { accepted == total }
pub fn local(values: &[f64], index: f64) -> bool {
  let comparison = self::fact(values, index);
  if !(comparison < 0f64) && !(comparison > 0f64) { true } else { false }
}
pub fn calls(first: &[f64], middle: f64, index: f64) -> bool {
  let (other, spare) = (1i32, 2i32);
  !(self::fact(first, middle) < index) && !(self::fact(first, middle) > index)
}`;
  const result = readPortableNumberSourceStyles(source);
  assert.match(result.source, /\(accepted\)\.partial_cmp\(&\(total\)\) == Some\(std::cmp::Ordering::Equal\)/u);
  assert.match(result.source, /\(comparison\)\.partial_cmp\(&\(0f64\)\) != Some\(std::cmp::Ordering::Less\)/u);
  assert.match(result.source, /\(comparison\)\.partial_cmp\(&\(0f64\)\) != Some\(std::cmp::Ordering::Greater\)/u);
  assert.match(result.source, /self::fact\(first, middle\)\)\.partial_cmp\(&\(index\)\) != Some\(std::cmp::Ordering::Less\)/u);
  assert.equal(result.source.split('self::fact(first, middle)').length, source.split('self::fact(first, middle)').length);
  assert.ok(result.source.indexOf('Ordering::Less') < result.source.indexOf('Ordering::Greater'));
  assert.doesNotMatch(result.source, /#\[(?:allow|expect)/u);
  replay(source, result);
  // Independently specified IEEE partial-order outcomes, including values for
  // which replacing a negated comparison by >=, <= or == would be incorrect.
  const cases = [
    [NaN, 0, null], [0, NaN, null], [NaN, NaN, null],
    [-0, 0, 'Equal'], [0, -0, 'Equal'], [1, 1 + Number.EPSILON, 'Less'],
    [Number.MIN_VALUE, 0, 'Greater'], [Number.MAX_VALUE, Infinity, 'Less'],
    [-Infinity, -Number.MAX_VALUE, 'Less'], [Infinity, Infinity, 'Equal'],
  ];
  for (const [left, right, ordering] of cases) {
    assert.equal(left === right, ordering === 'Equal');
    assert.equal(!(left < right), ordering !== 'Less');
    assert.equal(!(left > right), ordering !== 'Greater');
  }
});

test('portable Number styles distinguish actual scalar code from macros, strings, overloaded types and shadowing', () => {
  const source = `// !(left < right) and accepted == total stay in this comment.
macro_rules! preserved { () => { pub fn macro_body(left: f64, right: f64) -> bool { !(left < right) } } }
#[control(pub fn attribute_body(left: f64, right: f64) -> bool { !(left < right) })]
pub fn quoted(left: f64, right: f64) -> bool {
  let text = r#"accepted == total; !(left < right)"#;
  left == right
}
pub fn unknown(left: &str, right: &str) -> bool { !(left < right) }
pub fn shadow(left: f64, right: f64) -> bool { let left = 1i32; !(left < right) }
pub fn tuple_shadow(left: f64, right: f64) -> bool { let (left, unused) = (1i32, 2i32); !(left < right) }
pub fn pattern(left: f64, right: f64) -> bool { match 1i32 { left => !(left < right) } }
pub fn member(left: f64, right: f64) -> bool { object.left == right }
pub fn larger(left: f64, right: f64) -> bool { left == right + 1f64 }`;
  const result = readPortableNumberSourceStyles(source);
  assert.match(result.source, /\(left\)\.partial_cmp\(&\(right\)\) == Some\(std::cmp::Ordering::Equal\)/u);
  for (const name of ['unknown', 'shadow', 'tuple_shadow', 'pattern', 'member', 'larger']) {
    assert.equal(readSourceTextRustBlock(result.source, 'fn', name), readSourceTextRustBlock(source, 'fn', name));
  }
  const before = tokenize(source, 'Rust'), after = tokenize(result.source, 'Rust');
  assert.deepEqual(after.comments, before.comments);
  assert.deepEqual(after.tokens.filter(token => token.kind === 'string').map(token => token.raw),
    before.tokens.filter(token => token.kind === 'string').map(token => token.raw));
  assert.ok(result.source.includes('pub fn macro_body(left: f64, right: f64) -> bool { !(left < right) }'));
  assert.ok(result.source.includes('#[control(pub fn attribute_body(left: f64, right: f64) -> bool { !(left < right) })]'));
  replay(source, result);
  const alias = 'type f64 = Custom; pub fn aliased(left: f64, right: f64) -> bool { left == right }';
  assert.deepEqual(readPortableNumberSourceStyles(alias), {source:alias, actions:[]});
});

test('portable Number formatter conversion retains the complete shared algorithm and rejects counterfeit bounds', () => {
  const emitter = readFileSync(new URL('../src/translation/emit-rust.js', import.meta.url), 'utf8');
  const prelude = /const NUMBER_PRELUDE = `([\s\S]*?)`;\n/u.exec(emitter)[1];
  const formatter = readSourceTextRustBlock(prelude, 'fn', 'js_number');
  const source = `pub mod ml_number {\n${formatter}\n}`;
  const result = readPortableNumberSourceStyles(source);
  assert.equal(result.source, source.replace('let k = digits.len() as i32;',
    'let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");'));
  replay(source, result);
  const changed = source.replace('let k = digits.len() as i32;', "digits.push('0'); let k = digits.len() as i32;");
  assert.throws(() => readPortableNumberSourceStyles(changed), /complete shared binary64 algorithm/u);
  assert.throws(() => readPortableNumberSourceStyles(source.replace('n <= 21', 'n <= 21i32')), /complete shared binary64 algorithm/u);
  const unrelated = 'pub fn other(digits: &[u8]) -> i32 { digits.len() as i32 }';
  assert.deepEqual(readPortableNumberSourceStyles(unrelated), {source:unrelated, actions:[]});
});

test('portable Number styles narrowly attribute original midpoint rounding and the exact independent Boolean interface', () => {
  const source = `pub fn ml_read_record_field_insertion_order_loop14(lower: f64, upper: f64) -> f64 {
  let middle = ((lower + upper) / 2f64).floor();
  middle
}
pub fn require_optional_result_binding(declared: bool, additional: bool, actual: &str, result: &str, same: bool, recursive: bool) -> bool {
  ((!declared || additional) && actual != "var" && result == "var") && !same && !recursive
}
pub fn other(a: bool, b: bool, c: bool, d: bool) -> bool { a && b && c && d }`;
  const result = readPortableNumberSourceStyles(source);
  assert.match(result.source, /#\[expect\(clippy::manual_midpoint, reason = "checked JavaScript Number addition, division and floor retain their separate rounding and evaluation order"\)\]\nlet middle = \(\(lower \+ upper\) \/ 2f64\)\.floor\(\);/u);
  assert.match(result.source, /#\[expect\(clippy::fn_params_excessive_bools, reason = "the checked source interface retains independent Boolean and text inputs in their original evaluation order"\)\]\npub fn require_optional_result_binding/u);
  assert.ok(result.source.includes('let middle = ((lower + upper) / 2f64).floor();'));
  assert.equal(readSourceTextRustBlock(result.source, 'fn', 'other'), readSourceTextRustBlock(source, 'fn', 'other'));
  replay(source, result);
  const raw = source.replace('((lower + upper) / 2f64).floor()', '(((lower + upper) / 2f64)).floor()');
  const rawResult = readPortableNumberSourceStyles(raw);
  assert.ok(rawResult.source.includes('let middle = (((lower + upper) / 2f64)).floor();'),
    'the actual raw emitter group is recognized without changing a single arithmetic or grouping token');
  assert.equal((rawResult.source.match(/clippy::manual_midpoint/gu) ?? []).length, 1);
  replay(raw, rawResult);
  for (const changed of [source.replace('/ 2f64', '/ 3f64'), source.replace('(lower + upper)', '(upper + lower)'),
    source.replace('lower: f64', 'lower: f32'), source.replace('additional: bool', 'additional: i32'),
    raw.replace('(((lower + upper) / 2f64))', '(lower + (upper / 2f64))'),
    raw.replace('(((lower + upper) / 2f64))', '((lower / 2f64) + (upper / 2f64))')]) {
    assert.throws(() => readPortableNumberSourceStyles(changed), /midpoint|Number bounds|independent ordered source input/u);
  }
  assert.equal(((Number.MAX_VALUE + Number.MAX_VALUE) / 2), Infinity,
    'the source operation intentionally retains overflow instead of silently selecting a nonoverflowing midpoint');
});
