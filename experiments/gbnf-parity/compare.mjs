// Compares the JavaScript GBNF importer and emitter with the Rust originals:
// every case below runs through the Rust crate (render.rs, via ../run-rust-experiment.mjs) and
// through js/src/grammar-{importers,emitters}/gbnf.js, and each observable
// result (rule renderings, docs, start rule, emitted text, lossy notes, error
// messages) must be identical. Usage: node experiments/gbnf-parity/compare.mjs
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { isDeepStrictEqual } from 'node:util';

import { Grammar } from '../../js/src/grammar.js';
import { importGbnf } from '../../js/src/grammar-importers/gbnf.js';
import { emitGbnf } from '../../js/src/grammar-emitters/gbnf.js';
import { renderGrammarRule } from '../../js/tests/support/render-grammar-expression.js';

const here = path.dirname(new URL(import.meta.url).pathname);
const fixture = (relative) => readFileSync(path.join(here, '../../rust/tests/fixtures/grammar', relative), 'utf8');

const e = {
  empty: () => ({ kind: 'empty' }),
  lit: (value) => ({ kind: 'literal', value }),
  ci: (value) => ({ kind: 'literalInsensitive', value }),
  ref: (name) => ({ kind: 'ref', name }),
  seq: (...items) => ({ kind: 'seq', items }),
  alt: (...items) => ({ kind: 'choice', items, ordered: false }),
  ord: (...items) => ({ kind: 'choice', items, ordered: true }),
  opt: (item) => ({ kind: 'optional', item }),
  star: (item) => ({ kind: 'repeat0', item }),
  plus: (item) => ({ kind: 'repeat1', item }),
  rep: (item, min, max) => ({ kind: 'repeat', item, min, max }),
  and: (item) => ({ kind: 'and', item }),
  not: (item) => ({ kind: 'not', item }),
  cap: (label, item) => ({ kind: 'capture', label, item }),
  range: (start, end) => ({ kind: 'charRange', start, end }),
  cls: (negated, ...items) => ({ kind: 'charClass', items, negated }),
  ch: (value) => ({ kind: 'char', value }),
  r: (start, end) => ({ kind: 'range', start, end }),
  any: () => ({ kind: 'any' }),
};
const grammar = (start, ...rules) => ({ start, rules: rules.map(([name, expression]) => ({ name, expression })) });

const importCases = [
  fixture('gbnf/arithmetic.gbnf'),
  fixture('emit/covering.gbnf'),
  fixture('emit/json-object.gbnf'),
  'root ::= "a"{3}',
  'root ::= "a"{2,} "b"{0,1} "c"{1,} "d"{0,}',
  'root ::= a\r\na ::= "x" # trailing\n# doc one\n#   doc two  \nb ::= a\n',
  'root ::= "a"\n  | "b"\n  # between\n  | "c"',
  'root ::= ("a" | ("b" | "c")) | "d"',
  'root ::= ( "a" "b" ) ( ) "c"',
  'root ::= | ',
  'root ::=',
  'root ::= "" | ""',
  'root ::= "x" | ""',
  'root ::= "\\n\\r\\t\\b\\f\\\\\\"\\x41\\u00e9\\U0001F600\\q\\[\\-"',
  'root ::= [\\x41-\\x5a\\]\\-^a-] [^^] [-a] [\\u00e9-\\U0001F600] [\\n\\t\\\\]',
  'root ::= . .* .+ .?',
  'root ::= x-y_z\nx-y_z ::= "é" "日本" "😀"',
  '\u0085root ::=　"a" ',
  'root ::= [unterminated',
  'root ::= "unterminated',
  'root ::= "bad\\x4g"',
  'root ::= "é\\u12"',
  'root ::= "\\uD800"',
  'root ::= "\\U00110000"',
  'root ::= "\\',
  'root ::= [\\x4g]',
  'root ::= [é\\xzz]',
  'root ::= [\\uD800]',
  'root ::= [\\',
  'root ::= [z-a]',
  'root ::= []',
  'root ::= [^]',
  'root ::= "a"{3,2}',
  'root ::= "a"{,2}',
  'root ::= "a"{2',
  'root ::= "a"{99999999999999999999}',
  'root ::= "a"{18446744073709551615}',
  'root ::= ("a"',
  'root ::= ("a"\n)',
  'root ::= )',
  'root ::= "a" )',
  'root "a"',
  'root ::= @',
  "root ::= '",
  'root ::= é',
  'root ::= ́',
  'root ::= ​',
  'root ::= \u0000',
  'root ::= ﻿',
  'é ::= "a"',
  'expr ::= "a"',
  '# only a comment',
  '',
  'root ::= a b ::= c',
];

const emitCases = [
  // Rust grammar_emit.rs: covering golden grammar.
  grammar(
    'entry',
    ['root', e.lit('shadow')],
    ['entry', e.seq(
      e.lit('start'), e.ref('root'), e.range('A', 'Z'),
      e.cls(false, e.r('a', 'c'), e.ch('_'), e.ch('?')),
      e.cls(true, e.ch('"'), e.ch('\n')),
      e.any(), e.ci('Case'), e.ord(e.lit('a'), e.lit('b')),
      e.opt(e.ref('item')), e.star(e.ref('item')), e.plus(e.ref('item')),
      e.rep(e.ref('item'), 2, 4), e.rep(e.ref('item'), 3, null), e.rep(e.ref('item'), 2, 2),
      e.seq(e.not(e.cls(false, e.ch('x'), e.r('0', '9'))), e.any()),
      e.cap('label', e.lit('cap')), e.empty(),
    )],
    ['item', e.lit('x')],
    ['bad name', e.ref('entry')],
  ),
  grammar(null, ['entry', e.seq(e.not(e.lit(',')), e.any())]),
  grammar(null, ['entry', e.seq(e.not(e.lit(',,')), e.any())]),
  grammar(null, ['entry', e.seq(e.not(e.cls(true, e.ch('a'))), e.any())]),
  grammar(null, ['entry', e.seq(e.not(e.cls(false, e.r('z', 'a'))), e.any())]),
  grammar(null, ['entry', e.seq(e.not(e.cls(false)), e.any())]),
  grammar(null, ['entry', e.seq(e.lit('a'), e.not(e.lit('b')))]),
  grammar(null, ['bad', e.and(e.lit('x'))]),
  grammar(null, ['bad', e.not(e.lit('xy'))]),
  grammar(null, ['bad', e.cls(false)]),
  grammar(null, ['bad', e.rep(e.lit('x'), 3, 2)]),
  grammar(null, ['bad', e.alt()]),
  grammar(null, ['bad', e.range('z', 'a')]),
  grammar(null, ['bad', e.cls(false, e.r('\u{1F600}', 'a'))]),
  grammar('missing', ['entry', e.lit('a')]),
  grammar(null),
  grammar('b', ['a', e.ref('b')], ['b', e.seq(e.ref('a'), e.ref('undefined name'))]),
  grammar(null,
    ['entry', e.seq(e.ref('9lives'), e.ref('--'), e.ref('a__b'), e.ref('root'), e.ref('root-1'), e.ref('é'), e.ref('\u{1F600}x'), e.ref('￿y'))],
    ['root', e.lit('r')],
    ['root-1', e.lit('r1')],
    ['a-b', e.lit('ab')],
  ),
  grammar(null, ['entry', e.seq(
    e.lit('"\\\n\r\t\b\f\u0001\u007f\u0085é'), e.range('\n', '\u0000'),
  )]),
  grammar(null, ['entry', e.seq(
    e.lit(''), e.range('\u0000', '\u009f'), e.cls(false, e.ch('['), e.ch(']'), e.ch('-'), e.ch('^'), e.ch('\\'), e.ch('\b'), e.r('\u0001', '\u001f')),
    e.ci(''), e.ci('a-B 1'), e.ci('é"'),
  )]),
  grammar(null, ['entry', e.alt(
    e.seq(e.lit('a'), e.alt(e.lit('b'), e.lit('c'))),
    e.ci('xy'),
    e.seq(e.ci('xy'), e.lit('z')),
    e.opt(e.alt(e.lit('p'), e.lit('q'))),
    e.cap(null, e.alt(e.lit('m'), e.lit('n'))),
    e.seq(e.cap('in "seq"\ń', e.alt(e.lit('m'), e.lit('n'))), e.lit('o')),
    e.star(e.seq()),
    e.seq(e.empty(), e.empty()),
    e.rep(e.ci('ab'), 0, 1),
  )]),
  grammar(null, ['entry', e.ord(e.ord(e.lit('a')), e.cap('x', e.ord(e.lit('b'), e.lit('c'))))]),
  grammar(null, ['entry', e.seq(e.not(e.lit('a')), e.not(e.lit('b')), e.any(), e.any())]),
];

function jsImport(source) {
  try {
    const imported = importGbnf(source);
    return {
      start: imported.start,
      sourceFormat: imported.sourceFormat,
      undefined: imported.undefinedNonterminals(),
      rules: [...imported.rules.values()].map((rule) => ({
        name: rule.name,
        doc: rule.doc ?? null,
        rendered: renderGrammarRule(rule),
      })),
    };
  } catch (error) {
    return { error: error.message };
  }
}

function jsEmit({ start, rules }) {
  const built = new Grammar(start, new Map(rules.map((rule) => [rule.name, rule])));
  try {
    const { source, report } = emitGbnf(built);
    return { source, lossy: report.lossy };
  } catch (error) {
    return { error: error.message };
  }
}

const directory = mkdtempSync(path.join(tmpdir(), 'gbnf-parity-'));
const casesFile = path.join(directory, 'cases.json');
writeFileSync(casesFile, JSON.stringify({ import: importCases, emit: emitCases }));
const rust = JSON.parse(execFileSync('node', [path.join(here, '..', 'run-rust-experiment.mjs'), path.join(here, 'render.rs'), casesFile], {
  cwd: here,
  encoding: 'utf8',
  maxBuffer: 64 * 1024 * 1024,
}));

// JavaScript numbers cannot hold every u64 repeat count exactly: the importer
// accepts the same bounds as Rust but counts above 2^53 - 1 lose precision.
const KNOWN_DIFFERENCES = new Set(['import #36']);

let mismatches = 0;
const compare = (label, input, rustResult, jsResult) => {
  if (isDeepStrictEqual(rustResult, jsResult)) return;
  if (KNOWN_DIFFERENCES.has(label)) {
    console.log(`known difference ${label}: ${JSON.stringify(input)}`);
    return;
  }
  mismatches += 1;
  console.log(`MISMATCH ${label}: ${JSON.stringify(input)}`);
  console.log(`  rust: ${JSON.stringify(rustResult)}`);
  console.log(`  js:   ${JSON.stringify(jsResult)}`);
};
importCases.forEach((source, index) => compare(`import #${index}`, source, rust.import[index], jsImport(source)));
emitCases.forEach((cases, index) => compare(`emit #${index}`, cases, rust.emit[index], jsEmit(cases)));

const total = importCases.length + emitCases.length;
console.log(`${total - mismatches - KNOWN_DIFFERENCES.size}/${total} GBNF parity cases identical, ${KNOWN_DIFFERENCES.size} known difference(s), ${mismatches} mismatch(es)`);
if (process.argv.includes('--verbose')) console.log(JSON.stringify(rust, null, 2));
process.exitCode = mismatches === 0 ? 0 : 1;
