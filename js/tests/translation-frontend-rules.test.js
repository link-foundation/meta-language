import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { acceptArgumentCount, decodeUnicodeEscape, findDefaultParameterReference } from '../src/translation/frontend-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const units = (text) => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));

test('Unicode escape decisions include fixed, braced, surrogate-pair and rejected forms', () => {
  for (const [text, allowFixed, expected] of [
    ['\\u00e9', true, { $: 'scalar', code: 233, end: 6 }],
    ['\\u{1F600}', false, { $: 'scalar', code: 128512, end: 9 }],
    ['\\ud83d\\uDE00', true, { $: 'scalar', code: 128512, end: 12 }],
    ['\\u0000', true, { $: 'scalar', code: 0, end: 6 }],
    ['\\u{10ffff}', false, { $: 'scalar', code: 1114111, end: 10 }],
    ['\\ud83d', true, { $: 'unsupported', end: 6, escapeLength: 6 }],
    ['\\udc00', true, { $: 'unsupported', end: 6, escapeLength: 6 }],
    ['\\u{110000}', true, { $: 'unsupported', end: 10, escapeLength: 10 }],
    ['\\u{d83d}', true, { $: 'unsupported', end: 8, escapeLength: 8 }],
    ['\\u00e9', false, { $: 'malformed' }],
    ['\\u12', true, { $: 'malformed' }],
    ['\\u{1234567}', true, { $: 'malformed' }],
    ['\\u{}', true, { $: 'malformed' }],
    ['\\u{no}', true, { $: 'malformed' }],
  ]) assert.deepEqual(decodeUnicodeEscape(units(text), allowFixed), expected, text);
});

test('omitted arguments require a default for every omitted parameter', () => {
  assert.equal(acceptArgumentCount([false, true, true], 1), true);
  assert.equal(acceptArgumentCount([false, true, true], 3), true);
  assert.equal(acceptArgumentCount([false, true, true], 0), false);
  assert.equal(acceptArgumentCount([false, true, true], 4), false);
  assert.equal(acceptArgumentCount([true, false], 0), false);
  assert.equal(acceptArgumentCount([], 0), true);
});

test('default references distinguish member names and nested delimiters', () => {
  assert.equal(findDefaultParameterReference(['identifier', 'punct', 'identifier', 'punct'], ['object', '.', 'earlier', ','], ['earlier']), -1);
  assert.equal(findDefaultParameterReference(['punct', 'identifier', 'punct', 'punct', 'identifier', 'punct'], ['(', 'other', ',', ')', 'earlier', ','], ['earlier']), 4);
  assert.equal(findDefaultParameterReference(['identifier', 'punct', 'identifier', 'punct'], ['value', ',', 'earlier', ')'], ['earlier']), -1);
});

test('the complete frontend decision module translates through meta-language', () => {
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  const emitted = emitRust(checkProgram(parseJavaScript(source)));
  for (const name of ['decode_unicode_escape', 'accept_argument_count', 'find_default_parameter_reference', 'start_regular_expression', 'regular_expression_end', 'find_binding_run_end', 'accept_binding_scope', 'accept_literal_binding', 'constant_binding_form', 'accept_constant_emission', 'render_constant_binding', 'accept_module_binding_scope']) {
    assert.ok(emitted.text.includes(`pub fn ${name}(`), name);
  }
});
