// Reviewed spellings of existing generated runtimes, not translated algorithms.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { tokenize } from '../src/translation/lexer.js';
import { STRING_PRELUDE } from '../src/translation/string-runtime.js';

function readPortableRuntimeSyntax(source) {
  const { tokens, comments } = tokenize(source, 'Rust'), matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index];
    if (token.kind !== 'punct') continue;
    if (['(', '[', '{'].includes(token.value)) stack.push(index);
    else if ([')', ']', '}'].includes(token.value)) {
      const open = stack.pop();
      assert.notEqual(open, undefined, 'a generated runtime delimiter retains its opening');
      assert.equal(tokens[open].value, ({ ')': '(', ']': '[', '}': '{' })[token.value]);
      matching.set(open, index);
    }
  }
  assert.equal(stack.length, 0, 'a generated runtime retains balanced delimiters');
  const opaque = new Map();
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index];
    let open;
    if (token.kind === 'macro') {
      open = index + 1;
      if (token.value === 'macro_rules' && tokens[open]?.kind === 'identifier') open++;
    } else if (token.kind === 'identifier' && tokens[index + 1]?.value === '!') {
      open = index + 2;
      if (token.value === 'macro_rules' && tokens[open]?.kind === 'identifier') open++;
    } else if (token.value === '#') {
      open = tokens[index + 1]?.value === '!' ? index + 2 : index + 1;
      if (tokens[open]?.value !== '[') continue;
    }
    if (open !== undefined && tokens[open]?.kind === 'punct'
      && ['(', '[', '{'].includes(tokens[open].value) && matching.has(open)) {
      const close = matching.get(open);
      opaque.set(index, close);
      index = close;
    }
  }
  return { tokens, comments, matching, opaque };
}

function readPortableRuntimeTokenKey(source) {
  const tokens = tokenize(source, 'Rust').tokens, selected = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index].value === '#' && tokens[index + 1]?.value === '['
      && tokens[index + 2]?.value === 'must_use' && tokens[index + 3]?.value === ']') index += 3;
    else if (tokens[index].kind !== 'eof') selected.push(tokens[index]);
  }
  return JSON.stringify(selected
    .map(token => [token.kind, token.kind === 'number' ? token.raw.replaceAll('_', '') : token.raw]));
}

function replacePortableRuntimeTokens(source, before, after) {
  const { tokens } = readPortableRuntimeSyntax(source);
  const expected = tokenize(before, 'Rust').tokens.filter(token => token.kind !== 'eof');
  const key = token => JSON.stringify([token.kind, token.kind === 'number' ? token.raw.replaceAll('_', '') : token.raw]);
  const edits = [];
  for (let index = 0; index + expected.length <= tokens.length; index++) {
    if (!expected.every((token, offset) => key(token) === key(tokens[index + offset]))) continue;
    edits.push([tokens[index].start, tokens[index + expected.length - 1].end]);
    index += expected.length - 1;
  }
  assert.notEqual(edits.length, 0, 'a qualified runtime style retains its exact token context');
  for (const [start, end] of edits.reverse()) source = source.slice(0, start) + after + source.slice(end);
  return source;
}

function readPortableRuntimeFunctions(source) {
  const { tokens, matching } = readPortableRuntimeSyntax(source), functions = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'fn') continue;
    let open = index + 2;
    while (open < tokens.length && tokens[open].value !== '{') {
      if (matching.has(open)) open = matching.get(open) + 1;
      else open++;
    }
    const close = matching.get(open);
    assert.notEqual(close, undefined, 'a generated runtime function retains its body');
    let start = index;
    if (tokens[start - 1]?.value === 'const') start--;
    if (tokens[start - 1]?.value === 'pub') start--;
    functions.push({ name: tokens[index + 1].value, start: tokens[start].start, end: tokens[close].end });
    index = close;
  }
  return functions;
}

function stylePortableStringRuntimeFunction(name, source) {
  const replace = (before, after) => { source = replacePortableRuntimeTokens(source, before, after); };
  if (['from_units', 'length', 'absolute'].includes(name)) replace('pub fn', 'pub const fn');
  else if (name === 'integer') replace('fn integer', 'const fn integer');
  else if (name === 'split') replace(
    'let separator = match separator { Some(value) => value, None => return vec![self.clone()] };',
    'let Some(separator) = separator else { return vec![self.clone()]; };');
  else if (name === 'code_point_at') {
    replace(`if (0xd800..=0xdbff).contains(&first) {
      if let Some(&second) = self.0.get(index + 1) {
        if (0xdc00..=0xdfff).contains(&second) {
          return Some(((first as u32 - 0xd800) * 1024 + second as u32 - 0xdc00 + 0x10000) as f64);
        }
      }
    }`, `if (0xd800..=0xdbff).contains(&first)
      && let Some(&second) = self.0.get(index + 1)
      && (0xdc00..=0xdfff).contains(&second) {
        return Some(f64::from((u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000));
      }`);
    replace('Some(first as f64)', 'Some(f64::from(first))');
  } else if (name === 'code_points') replace('.map_or(false, |second|', '.is_some_and(|second|');
  else if (name === 'trim') {
    replace('unit as u32', 'u32::from(unit)');
    replace('.map_or(false, |c|', '.is_some_and(|c|');
  } else if (name === 'repeat') {
    replace('if count < 0.0 || !count.is_finite() { panic!("Invalid count value: {}", count); }',
      'assert!(count >= 0.0 && count.is_finite(), "Invalid count value: {count}");');
    // The exact complete body normalizes NaN before this check and only
    // multiplies a finite nonnegative count by a finite vector length.
    replace('if size > 536870888.0 { panic!("Invalid string length"); }',
      'assert!(size <= 536870888.0, "Invalid string length");');
  } else if (name === 'pad') {
    // integer(length).max(0.0) in the authenticated body excludes NaN here.
    replace('if length > 536870888.0 { panic!("Invalid string length"); }',
      'assert!(length <= 536870888.0, "Invalid string length");');
  }
  return source;
}

function readPortableRuntimeLiteral(name) {
  const emitter = readFileSync(new URL('../src/translation/emit-rust.js', import.meta.url), 'utf8');
  const marker = 'const ' + name + ' = `';
  assert.equal(emitter.split(marker).length, 2, 'the styles retain one actual complete emitter runtime');
  const first = emitter.indexOf(marker) + marker.length, last = emitter.indexOf('`;', first);
  assert.ok(last >= first, 'the actual runtime retains its literal boundary');
  const literal = emitter.slice(first, last);
  assert.doesNotMatch(literal, /\\|\$\{|`/u, 'the actual runtime is a literal without template cooking');
  return literal;
}

function stylePortableIntegerRuntime(source) {
  const replace = (before, after) => { source = replacePortableRuntimeTokens(source, before, after); };
  replace('for limb in remainder.iter_mut()', 'for limb in &mut remainder');
  replace(`let (negative, digits) = match text.strip_prefix('-') {
    Some(rest) => (true, rest), None => (false, text),
  };`, "let (negative, digits) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));");
  replace('pub fn is_zero', 'pub const fn is_zero');
  replace('pub fn is_negative', 'pub const fn is_negative');
  replace('if value.is_negative() { panic!("{message}"); }', 'assert!(!value.is_negative(), "{message}");');
  // String's fmt::Write implementation always succeeds. The same u32 chunk
  // formatting is appended in the same order, without an intermediate String.
  replace('text.push_str(&format!("{chunk:09}"));',
    'std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}")).expect("writing to a String cannot fail");');
  return source;
}

/** Return recorded whole-runtime edits only for the original helper bodies. */
export function readPortableRuntimeSourceStyles(source) {
  const { tokens, matching, opaque } = readPortableRuntimeSyntax(source), edits = [];
  const stringFunctions = new Map(readPortableRuntimeFunctions(STRING_PRELUDE)
    .map(item => [item.name, STRING_PRELUDE.slice(item.start, item.end)]));
  const arrayFunctions = new Map([
    ['at', `pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
      match index.and_then(|index| values.get(index)) {
        Some(value) => value.clone(), None => panic!("array index out of range"),
      }
    }`],
    ['integer', 'fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }'],
  ]);
  for (let index = 0; index < tokens.length; index++) {
    if (opaque.has(index)) { index = opaque.get(index); continue; }
    if (tokens[index].value !== 'mod' || tokens[index + 1]?.kind !== 'identifier'
      || !['ml_string', 'ml_array', 'ml_math', 'ml_number', 'ml'].includes(tokens[index + 1].value)
      || tokens[index + 2]?.value !== '{') continue;
    const name = tokens[index + 1].value, close = matching.get(index + 2);
    const start = tokens[index - 1]?.value === 'pub' ? tokens[index - 1].start : tokens[index].start;
    const end = tokens[close].end, original = source.slice(start, end), replacements = [];
    // repeat/pad rely on integer()'s actual NaN normalization and Text's u16
    // storage. Qualify the complete runtime, including those dependencies.
    if (name === 'ml_string' && readPortableRuntimeTokenKey(original) !== readPortableRuntimeTokenKey(STRING_PRELUDE)) {
      index = close;
      continue;
    }
    if (name === 'ml_array' && !['ARRAY_PRELUDE', 'LEGACY_ARRAY_PRELUDE'].some(provider =>
      readPortableRuntimeTokenKey(original) === readPortableRuntimeTokenKey(readPortableRuntimeLiteral(provider)))) {
      index = close;
      continue;
    }
    if (name === 'ml') {
      if (readPortableRuntimeTokenKey(original) === readPortableRuntimeTokenKey(readPortableRuntimeLiteral('PRELUDE'))) {
        edits.push({ start, end, from: original, to: stylePortableIntegerRuntime(original) });
      }
      index = close;
      continue;
    }
    if (!['ml_math', 'ml_number'].includes(name)) for (const item of readPortableRuntimeFunctions(original)) {
      const from = original.slice(item.start, item.end);
      const expected = (name === 'ml_string' ? stringFunctions : arrayFunctions).get(item.name);
      if (!expected || readPortableRuntimeTokenKey(from) !== readPortableRuntimeTokenKey(expected)) continue;
      let to;
      if (name === 'ml_string') to = stylePortableStringRuntimeFunction(item.name, from);
      else if (item.name === 'integer') to = replacePortableRuntimeTokens(from, 'fn integer', 'const fn integer');
      else to = replacePortableRuntimeTokens(from,
        'match index.and_then(|index| values.get(index)) { Some(value) => value.clone(), None => panic!("array index out of range"), }',
        'index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)');
      if (to !== from) replacements.push([item.start, item.end, to]);
    }
    if (name === 'ml_number') {
      const provider = readPortableRuntimeLiteral('NUMBER_PRELUDE');
      const expected = readPortableRuntimeFunctions(provider).find(item => item.name === 'same_value');
      assert.ok(expected, 'the runtime retains its actual SameValue helper');
      for (const item of readPortableRuntimeFunctions(original)) {
        if (item.name !== 'same_value') continue;
        const from = original.slice(item.start, item.end);
        if (readPortableRuntimeTokenKey(from) !== readPortableRuntimeTokenKey(provider.slice(expected.start, expected.end))) continue;
        replacements.push([item.start, item.end, replacePortableRuntimeTokens(from, 'pub fn same_value', 'pub const fn same_value')]);
      }
    }
    let styled = original;
    for (const [first, last, text] of replacements.reverse()) styled = styled.slice(0, first) + text + styled.slice(last);
    if (name === 'ml_math' && readPortableRuntimeTokenKey(styled) === readPortableRuntimeTokenKey(readPortableRuntimeLiteral('MATH_PRELUDE'))) {
      styled = replacePortableRuntimeTokens(styled, 'if x < 0.0 && x >= -0.5 {', 'if (-0.5..0.0).contains(&x) {');
    }
    if (name === 'ml_math' || name === 'ml_number') {
      const syntax = readPortableRuntimeSyntax(styled), { comments } = syntax;
      for (const comment of comments.reverse()) {
        if ([...syntax.opaque].some(([first, last]) => comment.start >= syntax.tokens[first].start
          && comment.end <= syntax.tokens[last].end)) continue;
        let text = comment.text;
        if (name === 'ml_math' && text === '/// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.') {
          text = text.replace('f64::signum', '`f64::signum`');
        } else if (name === 'ml_number' && text === '/// ECMAScript Number::toString: the shortest decimal that reads back as') {
          text = text.replace('Number::toString', '`Number::toString`');
        } else if (name === 'ml_number' && text === '/// SameValue, as Object.is and assert.strictEqual compare: NaN equals') {
          text = text.replace('SameValue', '`SameValue`').replace('Object.is', '`Object.is`').replace('assert.strictEqual', '`assert.strictEqual`');
        }
        if (text !== comment.text) styled = styled.slice(0, comment.start) + text + styled.slice(comment.end);
      }
    }
    if (styled !== original) edits.push({ start, end, from: original, to: styled });
    index = close;
  }
  let result = source;
  for (const edit of [...edits].reverse()) result = result.slice(0, edit.start) + edit.to + result.slice(edit.end);
  // Decorator replacements are literal. If an opaque literal also carries an
  // entire helper module, retain the complete source context instead of
  // changing that literal, or applying duplicate module actions twice.
  const ambiguous = edits.some(edit => source.split(edit.from).length !== 2);
  const actions = ambiguous && result !== source ? [{ op: 'replace', field: 'source', from: source, to: result }]
    : edits.map(({ from, to }) => ({ op: 'replace', field: 'source', from, to }));
  return { source: result, actions };
}
