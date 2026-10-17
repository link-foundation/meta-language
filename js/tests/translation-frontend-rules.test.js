import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { acceptArgumentCount, acceptJavaScriptOptionalTupleContext, acceptPrecedenceOperandContext, acceptScannerPreemptionBoundary, decodeUnicodeEscape, findDocumentationParameterRange, findDefaultParameterReference, readTypeQueryResult } from '../src/translation/frontend-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { parseRust } from '../src/translation/rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { readSourceTextRustBlock } from '../scripts/source-text-byte-library-style.mjs';
import { readFrontendTypeQueryStyleCorrections } from '../scripts/frontend-rule-style-corrections.mjs';

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

test('generated Unicode decoding borrows its owned vector at every shared array accessor', () => {
  const generated = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
  const decoder = generated.split(/(?=^pub fn )/gmu).find(item => item.startsWith('pub fn decode_unicode_escape('));
  assert.ok(decoder);
  assert.match(decoder, /^pub fn decode_unicode_escape\(\n    units: Vec<f64>,/u);
  const accesses = [...decoder.matchAll(/ml_array::optional_at\(\s*(&?)units,/gu)];
  assert.ok(accesses.length > 0);
  assert.ok(accesses.every(access => access[1] === '&'), 'an owned Vec must be borrowed for the shared &[T] accessor');
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
    if (text === raw) {
      for (const name of ['digit_2', 'digit_3', 'digit_4']) assert.equal((text.match(new RegExp(`^\\s+${name}$`, 'gmu')) ?? []).length, 2, name);
    } else {
      // Clippy can inline a checked local value. Both Unicode decoders still
      // need each scalar subtraction, including its undefined -> NaN guard.
      const scalar = String.raw`(?:match\s+unit(?:\.clone\(\))?\s*\{\s*Some\(ml_present\)\s*=>\s*ml_present,\s*None\s*=>\s*f64::NAN,?\s*\}|unit(?:\.clone\(\))?\.unwrap_or\(f64::NAN\))`;
      for (const offset of [48, 55, 87]) assert.equal((text.match(new RegExp(`${scalar}\\s*-\\s*${offset}f64`, 'gu')) ?? []).length, 2, `checked digit subtraction ${offset}`);
    }
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

test('reviewed Boolean helper styles preserve boundary flags, bounded context overlap and nullable defaults', () => {
  for (const virtualToken of [false, true]) for (const atBoundary of [false, true]) {
    assert.equal(acceptScannerPreemptionBoundary(virtualToken, atBoundary), !(virtualToken && atBoundary));
  }
  for (const [owner, child, expected] of [
    [[], [], false], [[true], [], false], [[], [true], false],
    [[false, true], [true, false], false], [[false, true], [false, true], true],
    [[true, false], [true], true], [[false, true], [false], false],
  ]) assert.equal(acceptPrecedenceOperandContext(owner, child), expected);
  const strings = readFileSync(new URL('../../rust/src/translation/frontend_rules/strings.rs', import.meta.url), 'utf8');
  assert.match(strings, /pub const fn accept_scanner_preemption_boundary\(virtual_token: bool, at_boundary: bool\) -> bool/u);
  assert.match(strings, /pub const fn accept_lexical_cache_context\(in_token: bool, quiet: bool, repairing: bool\) -> bool/u);
  assert.match(strings, /pub const fn accept_lexical_cache_reservation\(cells: f64, count: f64, limit: f64\) -> bool/u);
  const entry = readSourceTextRustBlock(strings, 'fn', 'accept_precedence_operand_context');
  assert.doesNotMatch(entry, /(?:owner_contexts|child_contexts|ml_s17)\.clone\(\)/u);
  const continuations = readFileSync(new URL('../../rust/src/translation/frontend_rules/continuations_2.rs', import.meta.url), 'utf8');
  const loop = readSourceTextRustBlock(continuations, 'fn', 'ml_accept_precedence_operand_context_loop28');
  for (const input of ['owner_contexts', 'child_contexts']) assert.ok(loop.includes(`index < (${input}.len() as f64)`), `bounds for ${input}`);
  assert.doesNotMatch(loop, /ml_logical22\.clone\(\)/u);
  assert.equal((loop.match(/\.unwrap_or_default\(\)/gu) ?? []).length, 2, 'both nullable Boolean values default to false');
  const winner = readSourceTextRustBlock(continuations, 'fn', 'ml_read_lexical_winner_index_loop29');
  assert.equal((winner.match(/\.unwrap_or_default\(\)/gu) ?? []).length, 1, 'the independent lexical eligibility value defaults to false');
  assert.equal((winner.match(/\.unwrap_or\(f64::NAN\)/gu) ?? []).length, 2, 'both independent lexical match ends preserve missing-number NaN');
});

test('current frontend decisions derive borrowed type queries and optional tuple guards without changing their observations', () => {
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  const raw = emitRust(checkProgram(parseJavaScript(source))).text;
  const actions = readFrontendTypeQueryStyleCorrections(raw);
  const decorators = new DecoratorSet([{ id: 'current-frontend-decision-controls', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]);
  const styled = decorators.decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.deepEqual(readFrontendTypeQueryStyleCorrections(styled), [], 'every complete source-derived replacement is idempotent');
  const names = ['read_type_query_result', 'accept_java_script_optional_tuple_context'];
  const blocks = names.map(name => readSourceTextRustBlock(styled, 'fn', name));
  assert.match(blocks[0], /^pub fn read_type_query_result\(kind: &str\) -> String/u);
  assert.doesNotMatch(blocks[0], /else\s*\{\s*if\b/u, 'the complete lookup keeps its original ordered tests in one else-if chain');
  for (const name of ['language', 'source_kind', 'expected_kind', 'present_kind']) {
    assert.match(blocks[1], new RegExp(name + ': &str', 'u'));
  }
  for (const block of blocks) assert.doesNotMatch(block, /#\[(?:allow|expect)\(/u);
  const generated = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
  assert.match(readSourceTextRustBlock(generated, 'fn', names[0]), /^pub fn read_type_query_result\(kind: &str\) -> String/u);
  const generatedGuard = readSourceTextRustBlock(generated, 'fn', names[1]);
  for (const name of ['language', 'source_kind', 'expected_kind', 'present_kind']) {
    assert.match(generatedGuard, new RegExp(name + ': &str', 'u'));
  }
  const observations = [
    ['float', 'number'], ['nat', 'bigint'], ['int', 'bigint'], ['bool', 'boolean'],
    ['string', 'string'], ['function', 'function'], ['array', 'object'], ['data', 'object'],
    ['record', 'object'], ['tuple', 'object'], ['map', 'object'], ['set', 'object'],
    ['', ''], ['unknown', ''], ['option', ''], ['undefined', ''],
  ];
  for (const candidate of [names.map(name => readSourceTextRustBlock(raw, 'fn', name)), blocks]) {
    const context = {};
    runInNewContext(emitJavaScript(checkProgram(parseRust(candidate.join('\n\n') + '\nfn main() {}'))).text,
      context, { timeout: 1000 });
    for (const [kind, expected] of observations) {
      assert.equal(readTypeQueryResult(kind), expected, kind);
      assert.equal(context.read_type_query_result(kind), expected, kind);
    }
    for (const language of ['JavaScript', 'Rust', 'unknown']) for (const sourceKind of ['array', 'tuple', 'unknown']) {
      for (const expectedKind of ['option', 'tuple', 'unknown']) for (const presentKind of ['tuple', 'array', 'unknown']) {
        const expected = language === 'JavaScript' && sourceKind === 'array' && expectedKind === 'option' && presentKind === 'tuple';
        assert.equal(acceptJavaScriptOptionalTupleContext(language, sourceKind, expectedKind, presentKind), expected);
        assert.equal(context.accept_java_script_optional_tuple_context(language, sourceKind, expectedKind, presentKind), expected);
      }
    }
  }
});
