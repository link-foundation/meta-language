// GBNF importer and emitter tests. These port the GBNF cases of the Rust
// tests rust/tests/unit/grammar_import_gbnf.rs,
// rust/tests/integration/grammar_import_gbnf.rs and
// rust/tests/unit/grammar_emit.rs; experiments/gbnf-parity/ compares both
// runtimes case by case.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import {
  Grammar,
  GrammarBuilder as E,
  deserializeGrammar,
  serializeGrammar,
} from '../src/grammar.js';
import { GrammarEmitError } from '../src/grammar-emitters/common.js';
import { emitGbnf } from '../src/grammar-emitters/gbnf.js';
import { GrammarImportError } from '../src/grammar-importers/common.js';
import { importGbnf } from '../src/grammar-importers/gbnf.js';
import { renderGrammarExpression } from './support/render-grammar-expression.js';

const fixture = async (relative) => (await readFile(
  new URL(`../../rust/tests/fixtures/grammar/${relative}`, import.meta.url),
  'utf8',
)).replaceAll('\r\n', '\n');

const ARITHMETIC_GBNF = await fixture('gbnf/arithmetic.gbnf');
const COVERING_GBNF = await fixture('emit/covering.gbnf');
const JSON_OBJECT_GBNF = await fixture('emit/json-object.gbnf');

// Raw IR nodes: unlike the flattening builders, these keep nested sequences,
// empty items and counted repeats exactly as the Rust `ExprBuilder` does.
const seq = (...items) => ({ kind: 'seq', items });
const alt = (...items) => ({ kind: 'choice', items, ordered: false });
const ordered = (...items) => ({ kind: 'choice', items, ordered: true });
const repeat = (item, min, max) => ({ kind: 'repeat', item, min, max });
const cls = (negated, ...items) => E.charClass(items, negated);
const ch = (value) => ({ kind: 'char', value });
const range = (start, end) => ({ kind: 'range', start, end });

function grammar(start, ...rules) {
  return new Grammar(start, new Map(rules.map(([name, expression]) => [name, { expression }])));
}

function render(imported, name) {
  return renderGrammarExpression(imported.rule(name).expression);
}

function assertUnsupported(action, construct) {
  assert.throws(action, (error) => {
    assert.ok(error instanceof GrammarEmitError);
    assert.equal(error.format, 'gbnf');
    assert.equal(error.kind, 'unsupported');
    assert.ok(error.construct.includes(construct), error.construct);
    assert.ok(error.message.includes('gbnf'));
    assert.ok(error.message.includes(construct));
    return true;
  });
}

function assertParseError(source, message) {
  assert.throws(() => importGbnf(source), (error) => {
    assert.ok(error instanceof GrammarImportError);
    assert.equal(error.format, 'gbnf');
    assert.equal(error.kind, 'parse');
    assert.equal(error.message, `gbnf import parse error: ${message}`);
    return true;
  });
}

test('imports the arithmetic GBNF fixture', () => {
  const imported = importGbnf(ARITHMETIC_GBNF);

  assert.equal(imported.sourceFormat, 'gbnf');
  assert.equal(imported.startRule().name, 'root');
  assert.deepEqual(imported.ruleNames(), [
    'root', 'expr', 'term', 'factor', 'number', 'identifier', 'not_quote',
  ]);
  assert.equal(imported.rule('root').kind, 'normal');
  assert.equal(
    imported.rule('root').doc,
    '# arithmetic grammar with counted repetition and character classes',
  );
  assert.equal(imported.rule('expr').doc, undefined);
  assert.deepEqual(imported.rule('root').expression, E.ref('expr'));
  assert.deepEqual(imported.rule('expr').expression, seq(
    E.ref('term'),
    E.repeat0(seq(alt(E.literal('+'), E.literal('-')), E.ref('term'))),
  ));
  assert.deepEqual(imported.rule('number').expression, seq(
    repeat(cls(false, range('0', '9')), 1, null),
    E.optional(seq(E.literal('.'), repeat(cls(false, range('0', '9')), 1, 3))),
  ));
  assert.deepEqual(imported.rule('identifier').expression, seq(
    cls(false, range('a', 'z'), range('A', 'Z'), ch('_')),
    repeat(cls(false, range('a', 'z'), range('A', 'Z'), range('0', '9'), ch('_')), 0, null),
  ));
  assert.deepEqual(imported.rule('not_quote').expression, cls(true, ch('"'), ch('\n')));
  assert.deepEqual(imported.undefinedNonterminals(), []);
});

test('lowers exact GBNF repetition', () => {
  assert.deepEqual(importGbnf('root ::= "a"{3}').rule('root').expression,
    repeat(E.literal('a'), 3, 3));
});

test('malformed GBNF reports parse errors with Rust byte offsets', () => {
  assertParseError('root ::= [unterminated', 'unterminated character class at byte 9');
  assertParseError('expr ::= "a"', 'GBNF grammar does not contain root rule');
  assertParseError('', 'GBNF grammar does not contain root rule');
  assertParseError('root ::= "é\\u12"', 'hexadecimal escape requires hexadecimal digits at byte 16');
  assertParseError('root ::= [é\\xzz]', 'hexadecimal escape requires hexadecimal digits at byte 13');
  assertParseError('root ::= "\\uD800"', 'invalid hexadecimal escape at byte 9');
  assertParseError('root ::= [z-a]', 'character class range start exceeds end at byte 9');
  assertParseError('root ::= [^]', 'character class must not be empty at byte 9');
  assertParseError('root ::= "a"{3,2}', 'repeat minimum exceeds maximum at byte 0');
  assertParseError('root ::= "a"{99999999999999999999}', 'number exceeds usize at byte 13');
  assertParseError('root ::= ("a"', 'expected \')\' at byte 0');
  assertParseError('root ::= "a" )', 'expected rule name at byte 13');
  assertParseError('root ::= a b ::= c', 'expected expression element at byte 13');
  assertParseError("root ::= '", "unexpected character '\\'' at byte 9");
  assertParseError('root ::= ́', "unexpected character '\\u{301}' at byte 9");
  assertParseError('é ::= "a"', "unexpected character 'é' at byte 0");
});

test('imports GBNF alternation, grouping, escapes and comments like Rust', () => {
  const imported = importGbnf([
    '# first',
    '#   second  ',
    'root ::= ("a" | ("b" | "c")) | "d" # trailing',
    'items ::= "x"',
    '  | "y" # between',
    '  | ',
    'text ::= "\\n\\x41\\u00e9\\U0001F600\\q" [\\x41-\\x5a\\]\\-^a-] [-a] . ""',
    'empty ::= ( ) | ""',
  ].join('\r\n'));

  assert.equal(imported.rule('root').doc, '# first\n#   second');
  assert.equal(render(imported, 'root'),
    'choice(literal("a"), literal("b"), literal("c"), literal("d"))');
  assert.equal(render(imported, 'items'), 'choice(literal("x"), literal("y"), empty)');
  assert.equal(render(imported, 'text'), 'seq(literal("\\nAé😀q"), class(range("A", "Z"), ' +
    'char("]"), char("-"), char("^"), char("a"), char("-")), class(char("-"), char("a")), any, ' +
    'literal(""))');
  assert.equal(render(imported, 'empty'), 'choice(empty, literal(""))');
});

test('imported GBNF grammar survives a serialization round trip', () => {
  const imported = importGbnf(ARITHMETIC_GBNF);
  const restored = deserializeGrammar(serializeGrammar(imported));

  assert.deepEqual(restored.normalized(), imported.normalized());
});

test('emits the GBNF golden with root mapping, native operators and lossy report', () => {
  const covering = grammar(
    'entry',
    ['root', E.literal('shadow')],
    ['entry', seq(
      E.literal('start'),
      E.ref('root'),
      E.charRange('A', 'Z'),
      cls(false, range('a', 'c'), ch('_'), ch('?')),
      cls(true, ch('"'), ch('\n')),
      E.any(),
      E.literalInsensitive('Case'),
      ordered(E.literal('a'), E.literal('b')),
      E.optional(E.ref('item')),
      E.repeat0(E.ref('item')),
      E.repeat1(E.ref('item')),
      repeat(E.ref('item'), 2, 4),
      repeat(E.ref('item'), 3, null),
      repeat(E.ref('item'), 2, 2),
      seq(E.not(cls(false, ch('x'), range('0', '9'))), E.any()),
      E.capture('label', E.literal('cap')),
      E.empty(),
    )],
    ['item', E.literal('x')],
    ['bad name', E.ref('entry')],
  );

  const { source, report } = emitGbnf(covering);

  assert.equal(source, COVERING_GBNF);
  assert.deepEqual(report.lossy, [
    'GBNF renamed rule "root" to "root-1"',
    'GBNF renamed rule "bad name" to "bad-name"',
    'GBNF expands case-insensitive terminal "Case" to character classes',
    'GBNF treats ordered choice as unordered choice',
    'GBNF dropped capture label "label"',
  ]);

  const reparsed = importGbnf(source);
  assert.equal(reparsed.sourceFormat, 'gbnf');
  assert.deepEqual(reparsed.undefinedNonterminals(), []);
});

test('emits the GBNF JSON object fixture for LLM constraints', () => {
  const commaMember = seq(E.literal(','), E.ref('ws'), E.ref('member'));
  const commaValue = seq(E.literal(','), E.ref('ws'), E.ref('value'));
  const digit = E.charRange('0', '9');
  const json = grammar(
    'object',
    ['object', seq(
      E.literal('{'), E.ref('ws'),
      E.optional(seq(E.ref('member'), E.repeat0(commaMember))),
      E.literal('}'), E.ref('ws'),
    )],
    ['member', seq(E.ref('string'), E.literal(':'), E.ref('ws'), E.ref('value'))],
    ['value', alt(
      E.ref('string'), E.ref('number'), E.ref('object'), E.ref('array'),
      E.literal('true'), E.literal('false'), E.literal('null'),
    )],
    ['array', seq(
      E.literal('['), E.ref('ws'),
      E.optional(seq(E.ref('value'), E.repeat0(commaValue))),
      E.literal(']'), E.ref('ws'),
    )],
    ['string', seq(
      E.literal('"'),
      E.repeat0(alt(
        cls(true, ch('"'), ch('\\')),
        seq(E.literal('\\'), cls(false, ...[...'"\\/bfnrt'].map(ch))),
      )),
      E.literal('"'), E.ref('ws'),
    )],
    ['number', seq(
      E.optional(E.literal('-')),
      E.ref('int'),
      E.optional(seq(E.literal('.'), E.repeat1(digit))),
      E.optional(seq(
        cls(false, ch('e'), ch('E')),
        E.optional(alt(E.literal('-'), E.literal('+'))),
        E.repeat1(digit),
      )),
      E.ref('ws'),
    )],
    ['int', alt(E.literal('0'), seq(E.charRange('1', '9'), E.repeat0(E.charRange('0', '9'))))],
    ['ws', E.repeat0(cls(false, ch(' '), ch('\t'), ch('\n'), ch('\r')))],
  );

  const { source, report } = emitGbnf(json);

  assert.equal(source, JSON_OBJECT_GBNF);
  assert.deepEqual(report.lossy, []);
  assert.deepEqual(importGbnf(source).undefinedNonterminals(), []);
});

test('GBNF folds a single-char negative predicate before any char', () => {
  const { source, report } = emitGbnf(grammar(null, ['entry', seq(E.not(E.literal(',')), E.any())]));

  assert.equal(source, 'root ::= [^,]\n');
  assert.deepEqual(report.lossy, []);
});

test('emitted GBNF re-imports for the basic smoke case', () => {
  const smoke = grammar(
    null,
    ['entry', seq(E.ref('letter'), E.repeat0(E.ref('letter')))],
    ['letter', alt(E.literal('a'), E.literal('b'))],
  );

  const { source } = emitGbnf(smoke);

  assert.equal(source, 'root ::= letter (letter)*\nletter ::= "a" | "b"\n');
  assert.deepEqual(importGbnf(source).undefinedNonterminals(), []);
});

test('GBNF escapes literals and classes and sanitizes names like Rust', () => {
  const { source, report } = emitGbnf(grammar(
    null,
    ['entry', seq(
      E.literal('"\\\n\r\t\b\f\u0001é'),
      E.charRange('\u0000', '\u009f'),
      cls(false, ch('['), ch(']'), ch('-'), ch('^'), ch('\\')),
      E.literalInsensitive('a-B 1'),
      E.capture(null, alt(E.literal('m'), E.literal('n'))),
      E.ref('9lives'),
      E.ref('root'),
    )],
    ['root', E.literal('r')],
  ));

  assert.equal(source, 'root ::= "\\"\\\\\\n\\r\\t\\b\\f\\x01é" [\\x00-\\x9F] [\\[\\]\\-\\^\\\\] ' +
    '[Aa]"-"[Bb]" ""1" ("m" | "n") ml-9lives root-1\nroot-1 ::= "r"\n');
  assert.deepEqual(report.lossy, [
    'GBNF renamed rule "root" to "root-1"',
    'GBNF renamed non-terminal reference "9lives" to "ml-9lives"',
    'GBNF expands case-insensitive terminal "a-B 1" to character classes',
    'GBNF dropped anonymous capture',
  ]);
});

test('GBNF emission reports unsupported constructs', () => {
  assertUnsupported(() => emitGbnf(grammar(null, ['bad', E.and(E.literal('x'))])), 'positive-predicate');
  assertUnsupported(() => emitGbnf(grammar(null, ['bad', E.not(E.literal('xy'))])), 'negative-predicate');
  assertUnsupported(() => emitGbnf(grammar(null, ['bad', cls(false)])), 'empty CharClass');
  assertUnsupported(() => emitGbnf(grammar(null, ['bad', repeat(E.literal('x'), 3, 2)])), 'greater than max');
  assertUnsupported(() => emitGbnf(grammar(null, ['bad', alt()])), 'empty Choice');
  assertUnsupported(
    () => emitGbnf(grammar(null, ['bad', E.charRange('z', 'a')])),
    'CharRange has descending bounds U+007A..=U+0061',
  );
  assertUnsupported(
    () => emitGbnf(grammar('missing', ['entry', E.literal('a')])),
    'configured start rule is not present in the grammar',
  );
  assert.deepEqual(emitGbnf(grammar(null)), { source: '', report: { lossy: [] } });
});
