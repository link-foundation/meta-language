import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { acceptArgumentCount, decodeUnicodeEscape, findDocumentationParameterRange, findDefaultParameterReference } from '../src/translation/frontend-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';

const units = (text) => Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));

test('Unicode escape decisions include fixed, braced, surrogate-pair and rejected forms', () => {
  for (const [text, allowFixed, expected] of [
    ['\\u00e9', true, { $: 'scalar', code: 233, end: 6 }],
    ['\\u{1F600}', false, { $: 'scalar', code: 128512, end: 9 }],
    ['\\ud83d\\uDE00', true, { $: 'scalar', code: 128512, end: 12 }],
    ['\\xE9', true, { $: 'scalar', code: 233, end: 4 }],
    ['\\x00', true, { $: 'scalar', code: 0, end: 4 }],
    ['\\xE9', false, { $: 'malformed' }],
    ['\\xA', true, { $: 'malformed' }],
    ['\\x{A}', true, { $: 'malformed' }],
    ['\\xGG', true, { $: 'malformed' }],
    ['xx00e9', true, { $: 'malformed' }],
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
  for (const name of ['decode_unicode_escape', 'accept_argument_count', 'find_default_parameter_reference', 'start_regular_expression', 'regular_expression_end', 'find_binding_run_end', 'accept_binding_scope', 'accept_literal_binding', 'constant_binding_form', 'accept_constant_emission', 'render_constant_binding', 'accept_module_binding_scope', 'find_documentation_parameter_range', 'accept_type_query_operand', 'read_type_query_result', 'read_array_method_form', 'accept_root_syntax_item', 'accept_source_prefix_restoration', 'accept_declaration_signature', 'read_string_test_operation', 'render_string_test_expression', 'read_string_test_helper', 'read_string_test_support', 'read_string_map_operation', 'render_string_map_expression', 'render_string_length_expression', 'read_string_map_refusal', 'accept_checked_type_query_operand', 'read_scanner_continuation_action', 'accept_source_envelope_restoration', 'accept_array_method_arguments']) {
    assert.ok(emitted.text.includes(`pub fn ${name}(`), name);
  }
});

test('frontend Rust decorators retain checked nullable arithmetic and loop result branches', () => {
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  const raw = emitRust(checkProgram(parseJavaScript(source))).text;
  const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino', import.meta.url), 'utf8'));
  const adapted = decorateEmitted('Rust', { source: raw }, decorators).source;
  const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: adapted }).source;
  for (const text of [raw, styled]) {
    // The Unicode digit value is already computed with undefined -> NaN.
    // A style rule must retain that value instead of substituting old source arithmetic.
    assert.doesNotMatch(text, /\(unit - (?:48|55|87)f64\)/u);
    for (const name of ['digit_2', 'digit_3', 'digit_4']) assert.equal((text.match(new RegExp(`^\\s+${name}$`, 'gmu')) ?? []).length, 2, name);
    const loops = text.split(/(?=^pub fn )/gmu).filter((body) => /^pub fn ml_regular_expression_end_loop\d+\([^\n]+\) -> f64 \{/u.test(body) && body.includes('let flag ='));
    assert.equal(loops.length, 1);
    assert.equal((loops[0].match(/^\s+index$/gmu) ?? []).length, 2, 'each loop exit supplies its Number result without an extra tail expression');
  }
});


test('documentation names support optional brackets and metadata defaults', () => {
  for (const [text, expected] of [
    [' value description', [1, 6]], ['\t[value]', [2, 7]],
    ['[value = 99]', [1, 6]], ['[$value2=anything]', [1, 8]],
    ['[9value]', []], ['[]', []], ['[value', []], ['[value description]', []],
  ]) assert.deepEqual(findDocumentationParameterRange(units(text)), expected, text);
});
