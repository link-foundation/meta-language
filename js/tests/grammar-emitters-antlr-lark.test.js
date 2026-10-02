// ANTLR and Lark emitter tests. These port rust/tests/unit/grammar_emit_antlr_lark.rs
// (exact emitted text and lossy-note needles), and pin the emitted text for every
// case of parity/fixtures/grammar-importers.json to the output of the Rust
// `meta-language import-grammar --format <fmt> --to antlr|lark` command, so both
// runtimes produce byte-identical ANTLR and Lark text for the shared corpus.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

import { Grammar, parseWithGrammar } from '../src/grammar.js';
import {
  GrammarEmitError,
  emitAntlr,
  emitLark,
  emit_antlr as emitAntlrAlias,
  emit_lark as emitLarkAlias,
} from '../src/grammar-emitters.js';
import {
  importAbnf,
  importAntlr,
  importBnf,
  importEbnf,
  importLark,
  importPest,
  importTreeSitterJson,
} from '../src/grammar-importers.js';

const corpus = JSON.parse(await readFile(new URL('../../parity/fixtures/grammar-importers.json', import.meta.url)));
const fixture = (relative) => readFile(new URL(`../../rust/tests/fixtures/grammar/${relative}`, import.meta.url), 'utf8');

const TARGETS = [
  ['antlr', emitAntlr, importAntlr],
  ['lark', emitLark, importLark],
];
const IMPORTERS = {
  abnf: importAbnf,
  bnf: importBnf,
  ebnf: importEbnf,
  pest: importPest,
  'tree-sitter-json': importTreeSitterJson,
};

const t = (value) => ({ kind: 'literal', value });
const nt = (name) => ({ kind: 'ref', name });
const seq = (...items) => ({ kind: 'seq', items });
const choice = (ordered, ...items) => ({ kind: 'choice', items, ordered });
const cls = (negated, ...items) => ({ kind: 'charClass', items, negated });
const ch = (value) => ({ kind: 'char', value });
const range = (start, end) => ({ kind: 'range', start, end });
const charRange = (start, end) => ({ kind: 'charRange', start, end });
const any = { kind: 'any' };
const empty = { kind: 'empty' };
const insensitive = (value) => ({ kind: 'literalInsensitive', value });
const optional = (item) => ({ kind: 'optional', item });
const star = (item) => ({ kind: 'repeat0', item });
const plus = (item) => ({ kind: 'repeat1', item });
const repeat = (item, min, max) => ({ kind: 'repeat', item, min, max });
const and = (item) => ({ kind: 'and', item });
const not = (item) => ({ kind: 'not', item });
const capture = (label, item) => ({ kind: 'capture', label, item });

/** Builds a grammar from `[name, kind, expression, doc?]` rows. */
function grammar(start, rows, sourceFormat = null) {
  const built = new Grammar(start, rows.map(([name, kind, expression]) => [name, { kind, expression }]), sourceFormat);
  built.ruleDocs = new Map(rows.filter((row) => row[3] !== undefined).map(([name, , , doc]) => [name, doc]));
  return built;
}

const docOf = (built, name) => built.rule(name)?.doc ?? built.ruleDocs?.get(name) ?? null;
const kinds = (built) => [...built.rules.values()].map((rule) => [rule.name, rule.kind]);

/** Emits, re-imports, and checks that emission is a fixpoint from the first re-import. */
function roundTrip(built, emit, importGrammar) {
  const { source, report } = emit(built);
  const reimported = importGrammar(source);
  const again = emit(reimported).source;
  const stable = importGrammar(again);
  assert.equal(emit(stable).source, again, `emission is a fixpoint:\n${source}`);
  return { source, report, reimported };
}

function assertNotes(report, needles) {
  for (const needle of needles) {
    assert.ok(report.lossy.some((note) => note.includes(needle)),
      `expected a note containing ${JSON.stringify(needle)} in ${JSON.stringify(report.lossy, null, 1)}`);
  }
}

test('snake_case aliases are exported', () => {
  assert.equal(emitAntlrAlias, emitAntlr);
  assert.equal(emitLarkAlias, emitLark);
});

test('antlr emits native constructs verbatim', () => {
  const g = grammar('entry', [
    ['entry', 'normal', seq(
      capture('name', nt('ID')),
      star(choice(false, nt('item'), t("it's"))),
      optional(seq(t(','), nt('item'))),
      plus(optional(nt('item'))),
      capture('non_greedy', star(nt('item'))),
      empty,
    )],
    ['item', 'normal', choice(false, not(t('x')), star(not(nt('ID'))), not(star(nt('ID'))), empty)],
    ['ID', 'token', seq(
      cls(false, range('a', 'z'), ch('-'), ch(']'), ch('\\'), ch('\n')),
      cls(true, ch('^'), ch('"')),
      charRange('0', '9'),
      any,
      t('tab\there\u0001'),
    )],
    ['DIGIT', 'silent', cls(false, range('0', '9')), '-> skip'],
  ], 'antlr');
  const { source, report, reimported } = roundTrip(g, emitAntlr, importAntlr);
  assert.equal(source, 'grammar Entry;\n\n' +
    "entry : name=ID (item | 'it\\'s')* (',' item)? (item?)+ item*? ;\n" +
    "item : ~'x' | ~ID* | ~(ID*) | ;\n" +
    "ID : [a-z\\-\\]\\\\\\n] ~[\\^\"] '0'..'9' . 'tab\\there\\u0001' ;\n" +
    'fragment DIGIT : [0-9] -> skip ;\n');
  assert.deepEqual(report.lossy, []);
  assert.equal(reimported.start, 'entry');
  assert.equal(docOf(reimported, 'DIGIT'), '-> skip');
});

test('antlr records every lowering', () => {
  const g = grammar('Main', [
    ['lexy', 'atomic', t('a')],
    ['Main', 'normal', choice(true,
      seq(and(nt('lexy')), insensitive('if1')),
      repeat(nt('lexy'), 2, 4),
      repeat(nt('lexy'), 2, null),
      capture(null, nt('lexy')),
      capture('fragment', nt('lexy')),
      seq(not(t('"')), any),
      not(cls(false, ch('q'))),
      cls(true),
      t(''),
    )],
    ['quiet', 'silent', nt('Main'), 'hidden\nhelper'],
  ]);
  const { source, report, reimported } = roundTrip(g, emitAntlr, importAntlr);
  assert.equal(source, 'grammar Main;\n\n' +
    "main : [iI] [fF] '1' | Lexy Lexy (Lexy Lexy?)? | Lexy Lexy+ | Lexy | fragment_=Lexy " +
    "| ~[\"] | ~[q] | . | '' ;\n" +
    "Lexy : 'a' ;\n" +
    '// hidden\n' +
    '// helper\n' +
    'fragment Quiet : main ;\n');
  assertNotes(report, [
    'ANTLR renamed rule "lexy" to "Lexy"',
    'ANTLR renamed rule "Main" to "main"',
    'ANTLR renamed rule "quiet" to "Quiet"',
    'ANTLR emitted atomic rule "lexy" as lexer rule "Lexy"',
    'ANTLR emitted silent rule "quiet" as lexer fragment "Quiet"',
    'ANTLR treats ordered choice as unordered choice in rule "main"',
    'ANTLR dropped positive lookahead in rule "main"',
    'ANTLR expanded case-insensitive literal "if1"',
    'ANTLR expanded counted repetition {2,4} in rule "main"',
    'ANTLR expanded counted repetition {2,} in rule "main"',
    'ANTLR dropped anonymous capture in rule "main"',
    'ANTLR renamed capture label "fragment" to "fragment_"',
    'ANTLR lowered negative lookahead followed by any character to a complemented set',
    'ANTLR emitted negative lookahead over a character set as a complemented set',
    'ANTLR emitted an empty negated character class as `.`',
    'ANTLR keeps an empty literal',
    'ANTLR lexer rule "Quiet" references parser rule "main"',
    'ANTLR parser rule "main" uses character-level constructs',
    'ANTLR re-imports the documentation of rule "Quiet" as Some("// hidden; // helper")',
  ]);
  assert.equal(reimported.start, 'main');
  assert.equal(reimported.rule('Lexy').kind, 'token');
  assert.equal(reimported.rule('Quiet').kind, 'silent');
});

test('antlr renames collisions, reserved words and references', () => {
  const g = grammar('Foo', [
    ['Foo', 'normal', seq(nt('foo'), nt('grammar'), nt('x-y'))],
    ['foo', 'normal', t('f')],
    ['grammar', 'normal', t('g')],
    ['EOF', 'token', t('e')],
  ]);
  const { source, report, reimported } = roundTrip(g, emitAntlr, importAntlr);
  assert.equal(source, "grammar Foo2;\n\nfoo_2 : foo grammar_ x_y ;\nfoo : 'f' ;\ngrammar_ : 'g' ;\nEOF_ : 'e' ;\n");
  assert.deepEqual(report.lossy, [
    'ANTLR renamed rule "Foo" to "foo_2"',
    'ANTLR renamed rule "grammar" to "grammar_"',
    'ANTLR renamed rule "EOF" to "EOF_"',
    'ANTLR renamed non-terminal reference "x-y" to "x_y"',
  ]);
  assert.equal(reimported.start, 'foo_2');
  assert.ok(reimported.undefinedNonterminals().includes('x_y'));
});

test('antlr orders the start rule first and reports lexer-only grammars', () => {
  const ordered = roundTrip(grammar('b', [['a', 'normal', t('a')], ['b', 'normal', nt('a')]]), emitAntlr, importAntlr);
  assert.ok(ordered.source.startsWith("grammar B;\n\nb : a ;\na : 'a' ;\n"), ordered.source);
  assert.equal(ordered.reimported.start, 'b');

  const lexerOnly = roundTrip(grammar('B', [['A', 'token', t('a')], ['B', 'token', nt('A')]]), emitAntlr, importAntlr);
  assert.equal(lexerOnly.source, "lexer grammar B;\n\nB : A ;\nA : 'a' ;\n");
  assert.deepEqual(lexerOnly.report.lossy, []);
  assert.equal(lexerOnly.reimported.start, 'B');

  const tokenStart = roundTrip(grammar('A', [['A', 'token', t('a')], ['b', 'normal', nt('A')]]), emitAntlr, importAntlr);
  assert.deepEqual(tokenStart.report.lossy, [
    'ANTLR re-imports start rule as "b" instead of "A", because ANTLR starts at the first parser rule',
  ]);
});

test('both emitters reject constructs without a faithful form with the Rust messages', () => {
  const cases = [
    [cls(false), 'antlr emit unsupported construct: empty character class',
      'lark emit unsupported construct: empty character class'],
    [charRange('z', 'a'), "antlr emit unsupported construct: descending character range 'z'..'a'",
      "lark emit unsupported construct: descending character range 'z'..'a'"],
    [cls(false, range('z', 'a')), "antlr emit unsupported construct: descending character class range 'z'-'a'",
      "lark emit unsupported construct: descending character class range 'z'-'a'"],
    [repeat(t('a'), 3, 2), 'antlr emit unsupported construct: repeat maximum 2 is below minimum 3',
      'lark emit unsupported construct: repeat maximum 2 is below minimum 3'],
    [repeat(t('a'), 0, 300), 'antlr emit unsupported construct: counted repetition {0,300} expands to 300 copies', null],
    [capture('regex', t('a\\')), "antlr emit unsupported construct: descending character range 'z'..'a'",
      'lark emit unsupported construct: regex /a\\/ ends with a lone backslash'],
  ];
  for (const [expression, antlrMessage, larkMessage] of cases) {
    const g = grammar('a', [['a', 'token', expression]]);
    if (expression.kind !== 'capture') {
      assert.throws(() => emitAntlr(g), (error) => error instanceof GrammarEmitError &&
        error.message === antlrMessage && error.format === 'antlr', antlrMessage);
    }
    if (larkMessage === null) {
      assert.equal(emitLark(g).source, 'A: "a" ~ 0..300\n');
    } else {
      assert.throws(() => emitLark(g), (error) => error instanceof GrammarEmitError &&
        error.message === larkMessage && error.format === 'lark', larkMessage);
    }
  }
  const ignored = grammar('a', [['a', 'normal', t('a')], ['_ignore', 'silent', empty, '%ignore']]);
  assert.throws(() => emitLark(ignored), { message: 'lark emit unsupported construct: empty %ignore expression' });
  const raw = grammar('a', [['a', 'token', { kind: 'charClass', value: 'a-z', negated: false }]]);
  assert.throws(() => emitAntlr(raw), { message: 'antlr emit unsupported construct: raw CharClass pattern' });
  assert.throws(() => emitLark(raw), { message: 'lark emit unsupported construct: raw CharClass pattern' });
});

test('lark emits native constructs verbatim', () => {
  const g = grammar('start', [
    ['start', 'normal', seq(
      nt('item'),
      star(seq(t(','), nt('item'))),
      optional(choice(false, nt('trailer'), t('say "hi"\n'))),
      plus(star(nt('WORD'))),
      empty,
    )],
    ['item', 'silent', choice(false,
      nt('WORD'),
      repeat(nt('NUMBER'), 2, 2),
      repeat(nt('NUMBER'), 1, 3),
      star(repeat(nt('WORD'), 0, 1)),
      empty,
    ), '// items; inline; priority 2'],
    ['trailer', 'normal', capture('regex', t('[a-z]+'))],
    ['WORD', 'token', cls(true, range('A', 'Z'), ch('/'), ch(']'), ch('^'), ch('\u0007'), ch('\u{1F600}'))],
    ['NUMBER', 'token', cls(false, range('0', '9'))],
    ['_SEP', 'normal', t(';')],
    ['_ignore', 'silent', nt('WS'), '%ignore'],
    ['WS', 'token', cls(false, ch(' '), ch('\t'))],
  ], 'lark');
  const { source, report, reimported } = roundTrip(g, emitLark, importLark);
  assert.equal(source,
    'start: item ("," item)* [trailer | "say \\"hi\\"\\n"] (WORD*)+\n' +
    '// items\n' +
    '?item.2: WORD | NUMBER ~ 2 | NUMBER ~ 1..3 | (WORD ~ 0..1)* |\n' +
    'trailer: /[a-z]+/\n' +
    'WORD: /[^A-Z\\/\\]\\^\\x07\\U0001F600]/\n' +
    'NUMBER: /[0-9]/\n' +
    '_SEP: ";"\n' +
    'WS: /[ \\t]/\n' +
    '%ignore WS\n');
  assert.deepEqual(report.lossy, []);
  assert.equal(docOf(reimported, 'item'), '// items; inline; priority 2');
  assert.equal(reimported.start, 'start');
});

test('lark records every lowering', () => {
  const g = grammar('Main', [
    ['Main', 'normal', choice(true,
      insensitive('a.b'),
      charRange('a', 'f'),
      any,
      repeat(nt('tok'), 0, null),
      repeat(nt('tok'), 1, null),
      repeat(nt('tok'), 3, null),
      seq(and(nt('tok')), not(nt('tok')), nt('tok')),
      seq(not(cls(false, ch('"'))), any),
      capture('name', nt('tok')),
      capture(null, nt('tok')),
      capture('regex', t('a/b\n')),
      capture('regex', t('[xy]')),
      capture('regex', t('')),
      cls(true),
      t(''),
    )],
    ['tok', 'atomic', nt('Main'), 'line one\nline two; priority 3'],
    ['_ignore', 'silent', t(' '), '%ignore'],
    ['Quiet', 'silent', t('q')],
  ]);
  const { source, report, reimported } = roundTrip(g, emitLark, importLark);
  assert.equal(source,
    'main: /(?i:a\\.b)/ | /[a-f]/ | /[\\x00-\\U0010FFFF]/ | TOK* | TOK+ | TOK ~ 3 TOK* | TOK ' +
    '| /[^"]/ | TOK | TOK | /a\\/b\\n/ | /[xy]/ | /(?:)/ | /[\\x00-\\U0010FFFF]/ | ""\n' +
    '// line one\n' +
    '// line two\n' +
    'TOK.3: main\n' +
    '?quiet: "q"\n' +
    '%ignore " "\n');
  assertNotes(report, [
    'Lark renamed rule "Main" to "main"',
    'Lark renamed rule "tok" to "TOK"',
    'Lark renamed rule "Quiet" to "quiet"',
    'Lark emitted atomic rule "tok" as terminal "TOK"',
    'Lark treats ordered choice as unordered choice in rule "main"',
    'Lark emitted case-insensitive literal "a.b" as a (?i:...) regex',
    'Lark emitted character range as a regex character class',
    'Lark emitted any character as a regex character class',
    'Lark emitted unbounded repetition {0,} as `*`',
    'Lark emitted unbounded repetition {1,} as `+`',
    'Lark emitted unbounded repetition {3,} as `~ 3` followed by `*`',
    'Lark dropped positive lookahead',
    'Lark dropped negative lookahead',
    'Lark lowered negative lookahead followed by any character',
    'Lark dropped capture label "name"',
    'Lark dropped anonymous capture',
    'Lark escaped regex /a/b\n/ as /a\\/b\\n/',
    'Lark regex /[xy]/ re-imports as a character class',
    'Lark emitted an empty regex as /(?:)/',
    'Lark emitted an empty negated character class as any character',
    'Lark keeps an empty literal',
    'Lark terminal "TOK" references rule "main"',
    'Lark re-imports the documentation of rule "TOK" as Some("// line one; // line two; priority 3")',
    'Lark re-imports the documentation of rule "quiet" as Some("inline")',
  ]);
  assert.ok(!report.lossy.some((note) => note.includes('re-imports start rule')), report.lossy.join('\n'));
  assert.equal(reimported.start, 'main');
  assert.deepEqual(reimported.rule('_ignore').expression, t(' '));
  assert.equal(reimported.rule('TOK').kind, 'token');
});

test('lark renames collisions and keeps %ignore names', () => {
  const g = grammar('Foo', [
    ['Foo', 'normal', seq(nt('foo'), nt('_ignore'), nt('Some-Ref'), nt('other'))],
    ['foo', 'normal', t('f')],
    ['_ignore', 'normal', t('i')],
    ['ws', 'silent', t(' '), '%ignore'],
    ['1st', 'token', t('1')],
    ['start', 'normal', t('s')],
  ]);
  const { source, report, reimported } = roundTrip(g, emitLark, importLark);
  assert.equal(source, 'foo_2: foo _ignore_2 SOME_REF other\nfoo: "f"\n_ignore_2: "i"\nT_1ST: "1"\nstart: "s"\n%ignore " "\n');
  assert.deepEqual(report.lossy, [
    'Lark renamed rule "Foo" to "foo_2"',
    'Lark renamed rule "_ignore" to "_ignore_2"',
    'Lark renamed rule "ws" to "_ignore"',
    'Lark renamed rule "1st" to "T_1ST"',
    'Lark renamed non-terminal reference "Some-Ref" to "SOME_REF"',
    'Lark re-imports start rule as "start" instead of "foo_2", because Lark starts at `start` or the first rule',
  ]);
  assert.equal(reimported.start, 'start');
  assert.equal(docOf(reimported, '_ignore'), '%ignore');
});

test('every rule kind round-trips to its target kind', () => {
  const g = grammar('normal', [
    ['normal', 'normal', seq(nt('TOKEN'), nt('atomic'))],
    ['TOKEN', 'token', t('t')],
    ['atomic', 'atomic', t('a')],
    ['silent', 'silent', t('s')],
  ]);
  assert.deepEqual(kinds(roundTrip(g, emitAntlr, importAntlr).reimported),
    [['normal', 'normal'], ['TOKEN', 'token'], ['Atomic', 'token'], ['Silent', 'silent']]);
  assert.deepEqual(kinds(roundTrip(g, emitLark, importLark).reimported),
    [['normal', 'normal'], ['TOKEN', 'token'], ['ATOMIC', 'token'], ['silent', 'silent']]);
});

test('antlr sources round-trip with their rule structure', async () => {
  for (const source of [
    await fixture('antlr/arithmetic.g4'),
    await fixture('antlr/covering.g4'),
    await fixture('antlr/lexer-mode.g4'),
    "grammar Comments;\n start : 'a' // first branch\n | ('b' /* group end */) ;",
    'grammar Missing; start : missing ;',
  ]) {
    const g = importAntlr(source);
    const { source: text, report, reimported } = roundTrip(g, emitAntlr, importAntlr);
    assert.deepEqual(kinds(reimported), kinds(g), text);
    assert.equal(reimported.start, g.start, text);
    assert.ok(report.lossy.every((note) => note.includes('re-imports the documentation')), report.lossy.join('\n'));
  }
  const covering = roundTrip(importAntlr(await fixture('antlr/covering.g4')), emitAntlr, importAntlr);
  assert.ok(covering.source.includes("COMMENT : '//' ~[\\r\\n]* -> channel(HIDDEN) ;"), covering.source);
  assert.ok(covering.source.includes('entry : name=ID values=item*? literalRange item ;'), covering.source);
  assert.equal(docOf(covering.reimported, 'COMMENT'), '-> channel(HIDDEN)');
});

test('lark sources round-trip with their rule structure and docs', async () => {
  for (const source of [await fixture('lark/covering.lark'), 'other: "x"\nstart: other\n', 'a.3: "\\x41" /b+/i?\n']) {
    const g = importLark(source);
    const { source: text, report, reimported } = roundTrip(g, emitLark, importLark);
    assert.deepEqual(kinds(reimported), kinds(g), text);
    assert.equal(reimported.start, g.start, text);
    assert.deepEqual([...reimported.rules.keys()].map((name) => docOf(reimported, name)),
      [...g.rules.keys()].map((name) => docOf(g, name)), text);
    assert.deepEqual(report.lossy, []);
  }
});

// Emitted by the Rust binary: `meta-language import-grammar --format <fmt> --to <target>`.
const RUST_FIXTURE_TEXT = {
  'abnf:message': {
    antlr: "grammar Message;\n\nmessage : word sP number ;\nword : aLPHA+ ;\nnumber : dIGIT+ ;\nsP : ' ' ;\naLPHA : [A-Za-z] ;\ndIGIT : '0'..'9' ;\n",
    lark: 'message: word sp number\nword: alpha+\nnumber: digit+\nsp: " "\nalpha: /[A-Za-z]/\ndigit: /[0-9]/\n',
  },
  'bnf:message': {
    antlr: "grammar Message;\n\nmessage : word ' ' number ;\nword : letter word | letter ;\nnumber : digit number | digit ;\nletter : 'M' | 'e' | 't' | 'a' | 'A' ;\ndigit : '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;\n",
    lark: 'message: word " " number\nword: letter word | letter\nnumber: digit number | digit\nletter: "M" | "e" | "t" | "a" | "A"\ndigit: "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"\n',
  },
  'ebnf:message': {
    antlr: "grammar Message;\n\nmessage : word ' ' number ;\nword : letter letter* ;\nnumber : digit digit* ;\nletter : 'M' | 'e' | 't' | 'a' | 'A' ;\ndigit : '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;\n",
    lark: 'message: word " " number\nword: letter letter*\nnumber: digit digit*\nletter: "M" | "e" | "t" | "a" | "A"\ndigit: "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"\n',
  },
  'pest:message': {
    antlr: "grammar Message;\n\nmessage : Word ' ' Number ;\nWord : Letter+ ;\nNumber : Digit+ ;\nfragment Letter : 'A'..'Z' | 'a'..'z' ;\nfragment Digit : '0'..'9' ;\n",
    lark: 'message: WORD " " NUMBER\nWORD: letter+\nNUMBER: digit+\n?letter: /[A-Z]/ | /[a-z]/\n?digit: /[0-9]/\n',
  },
  'tree-sitter-json:message': {
    antlr: "grammar Document;\n\ndocument : word ' ' Number ;\nword : [A-Za-z]+ ;\nNumber : [0-9]+ ;\n",
    lark: 'document: word " " NUMBER\nword: /[A-Za-z]/+\nNUMBER: /[0-9]/+\n',
  },
  'abnf:constructs': {
    antlr: "grammar Assignment;\n\nassignment : [lL] [eE] [tT] sP+ name sP* '=' sP* value ';'? ;\nname : aLPHA ((aLPHA | dIGIT | '_') ((aLPHA | dIGIT | '_') ((aLPHA | dIGIT | '_') ((aLPHA | dIGIT | '_') ((aLPHA | dIGIT | '_') ((aLPHA | dIGIT | '_') (aLPHA | dIGIT | '_')?)?)?)?)?)?)? ;\nvalue : dIGIT (dIGIT dIGIT?)? | dQUOTE (' '..'!' | '#'..'~')* dQUOTE ;\nsP : ' ' ;\naLPHA : [A-Za-z] ;\ndIGIT : '0'..'9' ;\ndQUOTE : '\"' ;\n",
    lark: 'assignment: /(?i:let)/ sp+ name sp* "=" sp* value [";"]\nname: alpha (alpha | digit | "_") ~ 0..7\nvalue: digit ~ 1..3 | dquote (/[ -!]/ | /[#-~]/)* dquote\nsp: " "\nalpha: /[A-Za-z]/\ndigit: /[0-9]/\ndquote: "\\""\n',
  },
  'bnf:constructs': {
    antlr: "grammar Assignment;\n\nassignment : 'let ' name spaces '=' spaces value end ;\nname : letter name | letter ;\nspaces : ' ' spaces | ;\nvalue : digit value | digit | '\"' letters '\"' ;\nletters : letter letters | ;\nend : ';' | ;\nletter : 'a' | 'b' | 'x' | 'q' ;\ndigit : '1' | '2' | '7' ;\n",
    lark: 'assignment: "let " name spaces "=" spaces value end\nname: letter name | letter\nspaces: " " spaces |\nvalue: digit value | digit | "\\"" letters "\\""\nletters: letter letters |\nend: ";" |\nletter: "a" | "b" | "x" | "q"\ndigit: "1" | "2" | "7"\n',
  },
  'ebnf:constructs': {
    antlr: "grammar Assignment;\n\nassignment : 'let' ' ' name ' '* '=' ' '* value ';'? ;\nname : letter (letter | digit | '_')* ;\nvalue : digit (digit digit?)? | '\"' letter* '\"' ;\nletter : 'a' | 'b' | 'x' | 'q' ;\ndigit : '1' | '2' | '7' ;\n",
    lark: 'assignment: "let" " " name " "* "=" " "* value [";"]\nname: letter (letter | digit | "_")*\nvalue: digit [digit [digit]] | "\\"" letter* "\\""\nletter: "a" | "b" | "x" | "q"\ndigit: "1" | "2" | "7"\n',
  },
  'pest:constructs': {
    antlr: "grammar Assignment;\n\nassignment : [lL] [eE] [tT] ' '+ ~keyword Name ' '* '=' ' '* Value ';'? ;\nkeyword : 'let' ~Alnum ;\nName : Alpha ((Alnum | '_') ((Alnum | '_') ((Alnum | '_') ((Alnum | '_') ((Alnum | '_') ((Alnum | '_') (Alnum | '_')?)?)?)?)?)?)? ;\nValue : Digit (Digit Digit?)? ~Digit | '\"' ~[\"]* '\"' ;\nfragment Alnum : Alpha | Digit ;\nfragment Alpha : 'a'..'z' | 'A'..'Z' ;\nfragment Digit : '0'..'9' ;\n",
    lark: 'assignment: /(?i:let)/ " "+ NAME " "* "=" " "* VALUE [";"]\nkeyword: "let"\nNAME: alpha (alnum | "_") ~ 0..7\nVALUE: digit ~ 1..3 | "\\"" /[^"]/* "\\""\n?alnum: alpha | digit\n?alpha: /[a-z]/ | /[A-Z]/\n?digit: /[0-9]/\n',
  },
  'tree-sitter-json:constructs': {
    antlr: "grammar Assignment;\n\nassignment : 'let' ' '+ name=Identifier ' '* '=' ' '* value=value ';'? ;\nIdentifier : [a-z_] [a-z0-9_]* ;\nvalue : prec_1=(alias_integer=Number) | string ;\nNumber : [0-9]+ ;\nstring : '\"' ~[\"\\\\]* immediate_token='\"' ;\n",
    lark: 'assignment: "let" " "+ IDENTIFIER " "* "=" " "* value [";"]\nIDENTIFIER: /[a-z_]/ /[a-z0-9_]/*\nvalue: NUMBER | string\nNUMBER: /[0-9]/+\nstring: "\\"" /[^"\\\\]/* "\\""\n',
  },
};

test('shared importer fixtures emit the Rust text and re-import to grammars accepting the corpus', () => {
  assert.deepEqual(Object.keys(RUST_FIXTURE_TEXT).sort(), corpus.cases.map((fixtureCase) => fixtureCase.id).sort());
  for (const fixtureCase of corpus.cases) {
    const g = IMPORTERS[fixtureCase.format](fixtureCase.source);
    for (const [target, emit, importGrammar] of TARGETS) {
      const label = `${fixtureCase.id} -> ${target}`;
      const { source, report, reimported } = roundTrip(g, emit, importGrammar);
      assert.equal(source, RUST_FIXTURE_TEXT[fixtureCase.id][target], label);
      assert.equal(reimported.rules.size, g.rules.size, label);
      if (!report.lossy.some((note) => note.includes(' renamed '))) {
        assert.deepEqual([...reimported.rules.keys()].sort(), [...g.rules.keys()].sort(), label);
      }
      // Like the Rust test, only lossless (modulo renames and choice order)
      // emissions must accept the corpus after re-import.
      if (report.lossy.every((note) => note.includes(' renamed ') || note.includes('ordered choice'))) {
        for (const accepted of fixtureCase.accepts) {
          assert.doesNotThrow(() => parseWithGrammar(reimported, accepted), `${label} accepts ${JSON.stringify(accepted)}:\n${source}`);
        }
      }
    }
  }
});

test('unusual rule names reach a naming fixpoint', () => {
  const names = ['1st', 'x-y', '__', '_', 'Über', 'aB', 'AB_c', 'T_1ST', '_WS', 'r_', 'start', 'EOF', 'fragment', 'mode'];
  for (const kind of ['normal', 'token', 'atomic', 'silent']) {
    const g = grammar('root', [
      ...names.map((name) => [name, kind, t(name)]),
      ['root', 'normal', seq(...names.map(nt))],
    ]);
    for (const [target, emit, importGrammar] of TARGETS) {
      const { source, reimported } = roundTrip(g, emit, importGrammar);
      assert.equal(reimported.rules.size, g.rules.size, `${target}:\n${source}`);
      assert.deepEqual(reimported.undefinedNonterminals(), [], `${target}:\n${source}`);
    }
  }
});

test('notes and quoting use Rust Debug escaping', () => {
  const g = grammar('a', [['a', 'normal', seq(insensitive('ß\u0301"'), t('\u0007\u2028'))]]);
  const antlr = emitAntlr(g);
  // Matches the Rust binary: no letter here has a single-character case
  // variant, so the per-letter expansion degenerates to the literal itself.
  assert.equal(antlr.source, "grammar A;\n\na : 'ß\u0301\"' '\\u0007\u2028' ;\n");
  assertNotes(antlr.report, ['ANTLR expanded case-insensitive literal "ß\\u{301}\\"" to per-letter character sets in rule "a"']);
  const lark = emitLark(g);
  assert.equal(lark.source, 'a: /(?i:ß\u0301\\")/ "\\x07\\u2028"\n');
  assertNotes(lark.report, ['Lark emitted case-insensitive literal "ß\\u{301}\\"" as a (?i:...) regex in rule "a"']);
});
