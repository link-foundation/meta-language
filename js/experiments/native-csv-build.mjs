// Builds parity/grammars/native/csv.lino from the readable expressions
// below and writes it in canonical Links Notation (parse, then render).
//   node experiments/native-csv-build.mjs
import { writeFileSync } from 'node:fs';

import { parseGrammarLinks, renderGrammarLinks } from '../src/index.js';

const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const cls = (negated, chars) => `(class ${negated ? 'negated' : 'plain'} ${[...chars].map((c) => `(char ${enc(c)})`).join(' ')})`;
const seq = (...items) => `(seq ${items.join(' ')})`;
const ord = (...items) => `(choice ordered ${items.join(' ')})`;
const imm = (item) => `(immediateToken ${item})`;
const alias = (name, item) => `(alias ${name} ${item})`;
const ref = (name) => `(ref ${name})`;
const not = (item) => `(not ${item})`;
const opt = (item) => `(optional ${item})`;
const rep0 = (item) => `(repeat0 ${item})`;
const rep1 = (item) => `(repeat1 ${item})`;

// The ASCII spaces tree-sitter-csv's default extras skip; a line break
// ends a row instead.
const space = cls(false, ' \t\v\f');
const digit = '(range 0 9)';
const hex = '(class plain (range 0 9) (range a f) (range A F))';

// tree-sitter-csv's line break, /\r|\r\n|\n/. After it the oracle skips the
// line breaks of blank lines as whitespace, so blank lines are not rows: one
// maximal run of line breaks ends a row, which keeps the parse unambiguous.
const newline = seq(alias('newline', imm(rep1(cls(false, '\r\n')))), not(imm(cls(false, '\r\n'))));
// A typed field ends the field: a comma, a line break or the end of the
// input follows. Otherwise the longer text token wins, as in the oracle's
// longest-match lexer (`1 `, `true1`, `0x`).
const fieldEnd = not(imm(cls(true, ',\r\n')));
// The spaces before a field are part of its token: the oracle lexes them
// with the token because a text field may start with a space.
const lead = rep0(space);

const rules = [
  ['document', 'normal', seq(
    rep0(seq(ref('row'), newline)),
    // RFC 4180: the last record may have no line break. A line break at the
    // end of the input ends the last row instead of opening an empty one.
    ord(not(imm('any')), ref('row')),
  )],
  ['row', 'normal', seq(ref('field'), rep0(seq(lit(','), ref('field'))))],
  ['field', 'normal', ord(ref('number'), ref('float'), ref('boolean'), ref('quoted'), ref('text'))],
  ['number', 'silent', seq(
    alias('number', imm(seq(lead, ord(seq(lit('0'), cls(false, 'xX'), rep1(hex)), rep1(digit))))),
    fieldEnd,
  )],
  ['float', 'silent', seq(
    alias('float', imm(seq(lead, ord(
      seq(rep0(digit), lit('.'), rep1(digit)),
      seq(rep1(digit), lit('.'), rep0(digit)),
    )))),
    fieldEnd,
  )],
  ['boolean', 'normal', seq(ord(imm(lit('true')), imm(lit('false'))), fieldEnd)],
  // RFC 4180 section 2.5-2.7: a quoted field may hold commas, line breaks
  // and doubled quotes. The oracle skips the spaces after the closing quote.
  ['quoted', 'silent', seq(
    alias('text', imm(seq(lead, lit('"'), rep0(ord(lit('""'), cls(true, '"'))), lit('"')))),
    opt(alias('blank_space', imm(rep1(space)))),
    fieldEnd,
  )],
  // RFC 4180 section 2.5: an unquoted field holds no quote; it may be empty.
  ['text', 'silent', alias('text', imm(rep0(cls(true, ',"\r\n'))))],
];

const text = [
  '(grammar (start document))',
  ...rules.map(([name, kind, expression]) => `(rule ${name} ${kind} ${expression})`),
].join('\n');
const canonical = renderGrammarLinks(parseGrammarLinks(text));
writeFileSync(new URL('../../parity/grammars/native/csv.lino', import.meta.url), canonical);
console.log(canonical.split('\n').length, 'lines');
