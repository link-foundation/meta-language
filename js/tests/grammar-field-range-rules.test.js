import assert from 'node:assert/strict';
import test from 'node:test';
import { renderGrammarSelectionNativeForms } from '../scripts/generate-grammar-selection-rules.mjs';
import { readGrammarSelectionFieldRangeLibraryStyles } from '../scripts/generate-grammar-selection-field-range-rules.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { GRAMMAR_SELECTION_FORMS } from '../src/grammar-selection-forms.js';
import {
  acceptGrammarSelectionFieldRangeMapping,
  acceptGrammarSelectionFieldRangeCandidate,
  acceptGrammarSelectionFieldRangeSelector,
  readGrammarSelectionFieldRangeContext,
  readGrammarSelectionFieldRangeFieldOrder,
  readGrammarSelectionFieldRangeChildren,
  acceptGrammarSelectionFieldRangeMatch,
  readGrammarSelectionFieldRangePlan,
} from '../src/translation/grammar-selection-field-range-rules.js';

test('native schema generation retains the closed selector choice and every ordered scalar field', () => {
  const native = renderGrammarSelectionNativeForms(GRAMMAR_SELECTION_FORMS);
  assert.ok(native.includes('("source-field-selector", &[("label", FieldType::Name), ("mode", FieldType::Choice(&["include", "exclude"])), ("first", FieldType::Integer), ("count", FieldType::Integer)]),'));
  assert.ok(native.includes('("grammar-field-candidate", &[("name", FieldType::Name), ("mapping", FieldType::Name), ("selector", FieldType::Expression)]),'));
  assert.ok(native.includes('("source-range", &[("start", FieldType::Integer), ("end", FieldType::Integer)]),'));
  assert.ok(native.includes('("grammar-range-candidate", &[("name", FieldType::Name), ("mapping", FieldType::Name), ("ranges", FieldType::Expressions)]),'));
  for (const choice of [[], ['include', 'include'], [''], ['include', 1]]) {
    assert.throws(() => renderGrammarSelectionNativeForms({ selector: [['mode', choice]] }), TypeError);
  }
  assert.throws(() => renderGrammarSelectionNativeForms({ selector: [['mode', 'unknown']] }), TypeError);
});
import { readGrammarSelectionIncludedRangePlan } from '../src/translation/grammar-selection-included-range-rules.js';

test('field range selectors require their closed kind, exact label, finite occurrence window and mode', () => {
  assert.equal(acceptGrammarSelectionFieldRangeMapping('field-ranges'), true);
  for (const mapping of ['identity', 'json-string', 'included-ranges', '', null]) {
    assert.equal(acceptGrammarSelectionFieldRangeMapping(mapping), false);
  }
  assert.equal(acceptGrammarSelectionFieldRangeCandidate('grammar-field-candidate', 'field-ranges'), true);
  for (const [kind, mapping] of [['grammar-candidate', 'field-ranges'], ['grammar-range-candidate', 'field-ranges'],
    ['grammar-field-candidate', 'included-ranges']]) {
    assert.equal(acceptGrammarSelectionFieldRangeCandidate(kind, mapping), false);
  }
  for (const mode of ['include', 'exclude']) {
    assert.equal(acceptGrammarSelectionFieldRangeSelector('source-field-selector', 'value', mode, 0, 1), true);
    assert.equal(acceptGrammarSelectionFieldRangeSelector('source-field-selector', 'é', mode, 4294967295, 4294967295), true);
  }
  for (const args of [
    ['selector', 'value', 'include', 0, 1], ['source-field-selector', '', 'include', 0, 1],
    ['source-field-selector', null, 'include', 0, 1], ['source-field-selector', 1, 'include', 0, 1],
    ['source-field-selector', new String('value'), 'include', 0, 1],
    ['source-field-selector', 'value', 'all', 0, 1], ['source-field-selector', 'value', 'include', '0', 1],
    ['source-field-selector', 'value', 'include', 0, '1'],
  ]) assert.equal(acceptGrammarSelectionFieldRangeSelector(...args), false);
  for (const bad of [-1, 0.5, NaN, Infinity, -Infinity, 4294967296]) {
    assert.equal(acceptGrammarSelectionFieldRangeSelector('source-field-selector', 'value', 'include', bad, 1), false);
    assert.equal(acceptGrammarSelectionFieldRangeSelector('source-field-selector', 'value', 'include', 0, bad), false);
  }
  assert.equal(acceptGrammarSelectionFieldRangeSelector('source-field-selector', 'value', 'include', 0, 0), false);
  const owners = 'pub fn accept_grammar_selection_field_range_selector(kind: &str, label: &str, mode: &str, first: f64, count: f64) -> bool { '
    + '((kind == "source-field-selector") && ("string" == "string")) && !label.is_empty() && mode == "include" && first >= 0f64 && count > 0f64 }\n'
    + 'pub fn accept_grammar_selection_field_range_match(actual_field: &str, label: &str, trivia: bool) -> bool { '
    + '((!trivia && ("string" == "string")) && ("string" == "string")) && !label.is_empty() && actual_field == label }';
  const expected = owners.replaceAll('&& ("string" == "string")', '');
  const styled = readGrammarSelectionFieldRangeLibraryStyles(owners);
  assert.equal(styled.actions.length, 2, 'all three pure right-hand conjunctions belong to exactly two complete borrowed owners');
  assert.equal(styled.source, expected, 'every other operand, order and source byte is retained');
  assert.equal(readGrammarSelectionFieldRangeLibraryStyles(expected).source, expected);
  assert.equal(readGrammarSelectionFieldRangeLibraryStyles(expected).actions.length, 0);
  const replay = new DecoratorSet([{ id: 'field-type-guard-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: styled.actions }]);
  assert.equal(DecoratorSet.fromLino(replay.toLino()).decorate('emitter', { format: 'Rust', source: owners }).source, expected);
  for (const refused of [
    owners.replace('(kind == "source-field-selector")', '{ assert!(true); kind == "source-field-selector" }'),
    owners.replace('&& ("string" == "string")', '&& ("string" == "other")'),
    owners.replace('&& ("string" == "string")', '&& ("string" /* retain */ == "string")'),
    owners.replace('&& ("string" == "string")', '&& ("string" == "string").then_some(true).is_some()'),
    owners.replace('&& ("string" == "string")', '&& unknown!("string" == "string")'),
    owners.replace('label: &str', 'label: String'),
    owners.replace('&& ("string" == "string")', '&& ("string" == "string") && ("string" == "string")'),
    'type str = Custom;\n' + owners,
    '#[opaque]\n' + owners,
    owners + '\n' + owners,
  ]) assert.equal(readGrammarSelectionFieldRangeLibraryStyles(refused).source, refused,
    'changed ABI, comparison, count, scope, macros, comments and opaque declarations never inherit a type guard proof');
});

test('field range continuation styles retain complete scopes and refuse unproved transfers', () => {
  const enumeration = 'MlReadGrammarSelectionFieldRangePlanJoin5';
  const variant = enumeration + 'Next';
  const branch = `if (occurrence >= first) && (occurrence < following) {
      let selected = if cursor < start {
        { self::${enumeration}::${variant}(ranges, width) }
      } else { self::${enumeration}::${variant}(ranges, width) };
      match selected {
        self::${enumeration}::${variant}(selected_ranges, selected_width) => {
          { (ranges, width, index) = (selected_ranges, selected_width, index + 2f64); continue; }
        }
      }
    } else { { index = index + 2f64; continue; } }`;
  const prefix = `pub enum ${enumeration} { ${variant}(Vec<f64>, f64), }
/// Keep this complete source owner and its leading documentation.
pub fn ml_read_grammar_selection_field_range_plan_loop3(
  spans: &[f64], mut start: f64, mut end: f64, mut first: f64, mode: &str,
  mut following: f64, mut ranges: Vec<f64>, mut previous: f64, mut cursor: f64, mut width: f64, mut index: f64,
) -> self::MlReadGrammarSelectionFieldRangePlanLoop3Result {
  loop { let occurrence = index / 2f64;
    `;
  const source = prefix + branch + '\n  }\n}';
  const offset = source.lastIndexOf('else');
  const expected = source.slice(0, offset) + '    ' + source.slice(offset + 4);
  const styled = readGrammarSelectionFieldRangeLibraryStyles(source);
  assert.equal(styled.actions.length, 1, 'one complete actual continuation owner supplies the proof');
  assert.equal(styled.source, expected, 'only the redundant keyword becomes equal-width whitespace');
  assert.equal(styled.source.length, source.length, 'all body offsets, blocks and original bytes remain fixed');
  assert.ok(styled.actions[0].from.includes('pub fn ml_read_grammar_selection_field_range_plan_loop3('));
  assert.equal(styled.actions[0].from.length, styled.actions[0].to.length);
  const replay = new DecoratorSet([{ id: 'field-continuation-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: styled.actions }]);
  assert.equal(DecoratorSet.fromLino(replay.toLino()).decorate('emitter', { format: 'Rust', source }).source, expected);
  assert.deepEqual(readGrammarSelectionFieldRangeLibraryStyles(expected), { source: expected, actions: [] },
    'the scoped recorded correction is idempotent');
  for (const refused of [
    source.replace('ml_read_grammar_selection_field_range_plan_loop3(', 'unreviewed_continuation('),
    source.replace('mut first: f64', 'mut first: Custom'),
    source.replace(`${variant}(Vec<f64>, f64),`, `${variant}(Vec<f64>, f64), Other,`),
    source.replace('pub enum', '#[derive(Custom)]\npub enum'),
    source.replace(`${variant}(Vec<f64>, f64),`, `${variant}(/* retain */ Vec<f64>, f64),`),
    source.replace(`${variant}(ranges, width)`, 'Other(ranges, width)'),
    source.replace('let selected = if', 'let other = if'),
    source.replace('let selected = if', `let selected = self::${enumeration}::${variant}(ranges, width); let selected = if`),
    source.replace('selected_width) => {', 'selected_width) if first > 0f64 => {'),
    source.replace('        }\n      }', '        }, _ => { continue; }\n      }'),
    source.replace('continue;', 'width = 1f64;'),
    source.replace('continue;', 'continue; observe();'),
    source.replace('let selected = if', '/* retain */ let selected = if'),
    source.replace('let selected = if', 'unknown!(); let selected = if'),
    prefix + 'let value = ' + branch + ';\n  }\n}',
    prefix + 'return ' + branch + ';\n  }\n}',
    prefix + 'consume(' + branch + ');\n  }\n}',
    prefix + 'let value = { ' + branch + ' };\n  }\n}',
    prefix + 'let value = vec![{ ' + branch + ' }];\n  }\n}',
    source.replace('continue;', "continue 'outer;"),
    source.replace('continue;', 'loop { continue; }'),
    'type f64 = Custom;\n' + source,
    'struct Vec<T>(T);\n' + source,
    source + '\n' + source,
  ]) assert.deepEqual(readGrammarSelectionFieldRangeLibraryStyles(refused), { source: refused, actions: [] },
    'changed owners, constructors, arms, control transfers, comments, macros and value contexts retain every byte');
});

test('field ranges retain exact operation-specific field orders and match only non-trivia capture labels', () => {
  for (const [role, expected] of [
    ['candidate', 'kind|name|mapping|selector'], ['selector', 'kind|label|mode|first|count'],
    ['occurrence', 'path|start|end'], ['derivation', 'operation|input|start|end|selector|occurrences|ranges|witness'],
    ['mapping', 'kind|operation|input|output|selector|occurrences|ranges|intervals'],
  ]) assert.equal(readGrammarSelectionFieldRangeFieldOrder(role), expected);
  for (const role of ['', 'range', 'source', 'json-string']) assert.equal(readGrammarSelectionFieldRangeFieldOrder(role), '');
  assert.equal(acceptGrammarSelectionFieldRangeMatch('value', 'value', false), true);
  assert.equal(acceptGrammarSelectionFieldRangeMatch('é', 'é', false), true);
  for (const args of [['value', 'key', false], ['value', 'value', true], ['', '', false],
    [null, 'value', false], ['value', null, false], [1, 1, false]]) {
    assert.equal(acceptGrammarSelectionFieldRangeMatch(...args), false);
  }
  for (const [type, sourceChildren, expected] of [
    ['node', false, 'children'], ['node', true, 'children'],
    ['embed', true, 'sourceChildren'], ['embed', false, 'none'],
    ['token', false, 'none'], ['token', true, 'none'],
    ['error', false, 'none'], ['error', true, 'none'],
    ['missing', false, 'none'], ['missing', true, 'none'],
    ['', false, ''], ['root', true, ''], ['constructor', false, ''],
  ]) assert.equal(readGrammarSelectionFieldRangeChildren(type, sourceChildren), expected);
});

test('field range context delegates legacy mappings and refuses mixed or lexical profiles', () => {
  for (const [mappings, inToken, expected] of [
    [[], false, 0], [['identity'], false, 0], [['json-string'], true, 0], [['included-ranges'], false, 0],
    [['future-view'], false, 0], [['field-ranges'], false, 1], [['field-ranges', 'field-ranges'], false, 1],
    [['field-ranges'], true, 2], [['field-ranges', 'identity'], false, 2],
    [['json-string', 'field-ranges'], false, 2], [['field-ranges', 'included-ranges'], false, 2],
    [['field-ranges', 'future-view'], false, 2],
  ]) assert.equal(readGrammarSelectionFieldRangeContext(mappings, inToken), expected);
});

test('field range plans select finite ordered windows or exact complements without changing input spans', () => {
  for (const [spans, start, end, first, count, mode, expected, width] of [
    [[21, 23, 26, 28, 29, 30], 20, 32, 0, 2, 'include', [1, 3, 6, 8], 4],
    [[21, 23, 26, 28, 29, 30], 20, 32, 1, 1, 'exclude', [0, 6, 8, 12], 10],
    [[21, 23, 26, 28, 29, 30], 20, 32, 2, 1, 'include', [9, 10], 1],
    [[21, 23, 26, 28, 29, 30], 20, 32, 0, 3, 'exclude', [0, 1, 3, 6, 8, 9, 10, 12], 7],
    [[10, 12, 12, 15], 10, 16, 0, 2, 'include', [0, 2, 2, 5], 5],
    [[10, 12, 12, 15], 10, 16, 0, 2, 'exclude', [5, 6], 1],
    [[0, 5, 5, 10], 0, 10, 0, 1, 'include', [0, 5], 5],
    [[0, 5, 5, 10], 0, 10, 0, 1, 'exclude', [5, 10], 5],
    [[4294967292, 4294967294], 4294967291, 4294967295, 0, 1, 'include', [1, 3], 2],
  ]) {
    const original = Object.freeze([...spans]);
    const actual = readGrammarSelectionFieldRangePlan(original, start, end, first, count, mode);
    assert.deepEqual(actual, expected);
    assert.deepEqual(original, spans);
    assert.notEqual(actual, original);
    const includedPlan = readGrammarSelectionIncludedRangePlan(actual, end - start);
    assert.notDeepEqual(includedPlan, [], 'the existing mapper independently accepts every explicit expected view');
    assert.equal(includedPlan.at(-1), width);
  }
});

test('field range plans validate every occurrence, bounds and window before admitting a proper contraction', () => {
  for (const args of [
    [[], 0, 10, 0, 1, 'include'], [[1], 0, 10, 0, 1, 'include'],
    [[1, 1], 0, 10, 0, 1, 'include'], [[2, 1], 0, 10, 0, 1, 'include'],
    [[-1, 2], 0, 10, 0, 1, 'include'], [[1, 11], 0, 10, 0, 1, 'include'],
    [[1, 4, 3, 5], 0, 10, 0, 1, 'include'], [[6, 7, 1, 2], 0, 10, 0, 1, 'include'],
    [[1, 2, 5, 4], 0, 10, 0, 1, 'include'], [[5, 4, 6, 7], 0, 10, 1, 1, 'exclude'],
    [[1, 2], 2, 10, 0, 1, 'include'], [[1, 2], 0, 10, 1, 1, 'include'],
    [[1, 2], 0, 10, 0, 2, 'include'], [[1, 2], 0, 10, 0, 0, 'include'],
    [[0, 10], 0, 10, 0, 1, 'include'], [[0, 10], 0, 10, 0, 1, 'exclude'],
    [[0, 5, 5, 10], 0, 10, 0, 2, 'include'], [[0, 5, 5, 10], 0, 10, 0, 2, 'exclude'],
    [[1, 2], 0, 10, 0, 1, 'all'], [[1, 2], 10, 0, 0, 1, 'include'],
    [[1, 2], 0, 0, 0, 1, 'include'], [[1, 2], -1, 10, 0, 1, 'include'],
    [[1, 2], 0, 4294967296, 0, 1, 'include'],
  ]) assert.deepEqual(readGrammarSelectionFieldRangePlan(...args), [], String(args));
  for (const bad of ['1', NaN, Infinity, -Infinity, 0.5, -1, 4294967296, null, undefined]) {
    assert.deepEqual(readGrammarSelectionFieldRangePlan([bad, 2], 0, 10, 0, 1, 'include'), []);
    assert.deepEqual(readGrammarSelectionFieldRangePlan([1, bad], 0, 10, 0, 1, 'exclude'), []);
  }
  for (const bad of ['0', NaN, Infinity, -Infinity, 0.5, -1, 4294967296]) {
    assert.deepEqual(readGrammarSelectionFieldRangePlan([1, 2], bad, 10, 0, 1, 'include'), []);
    assert.deepEqual(readGrammarSelectionFieldRangePlan([1, 2], 0, bad, 0, 1, 'include'), []);
    assert.deepEqual(readGrammarSelectionFieldRangePlan([1, 2], 0, 10, bad, 1, 'include'), []);
    assert.deepEqual(readGrammarSelectionFieldRangePlan([1, 2], 0, 10, 0, bad, 'include'), []);
  }
  assert.deepEqual(readGrammarSelectionFieldRangePlan([, 2], 0, 10, 0, 1, 'include'), []);
});
