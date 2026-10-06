// Builds parity/grammars/native/diff.lino from the readable expressions
// below and writes it in canonical Links Notation (parse, then render).
//   node experiments/native-diff-build.mjs
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
const tok = (item) => `(token ${item})`;
const alias = (name, item) => `(alias ${name} ${item})`;
const ref = (name) => `(ref ${name})`;
const not = (item) => `(not ${item})`;
const opt = (item) => `(optional ${item})`;
const rep0 = (item) => `(repeat0 ${item})`;
const rep1 = (item) => `(repeat1 ${item})`;
const field = (name, item) => `(capture labeled ${name} ${item})`;
const maybe = (item) => ord(item, 'empty');

const SPACE = ' \t\v\f';
const space = cls(false, SPACE);
const lineChar = cls(true, '\r\n');
const wordChar = cls(true, `${SPACE}\r\n`);
const digit = `(range 0 9)`;
const digits = rep1(digit);
const hex = `(class plain (range 0 9) (range a f))`;
const wordCharacter = `(class plain (range A Z) (range a z) (range 0 9) (char _) (char -))`;

// A line break, after the spaces that end the line; a block line also ends
// at the end of the input.
const newline = alias('newline', tok(seq(opt(lit('\r')), lit('\n'))));
const lineEnd = ord(newline, seq(rep0(space), not(imm('any'))));
// The rest of a line, spaces included: tree-sitter-diff's ANYTHING.
// A line break with the spaces of a blank line before it, which the
// tree-sitter-diff root keeps.
const blankLine = alias('newline', imm(seq(rep0(space), opt(lit('\r')), lit('\n'))));
const anything = alias('anything', imm(rep1(lineChar)));
// The rest of a line with something that is not a space in it.
const visible = alias('anything', imm(seq(rep0(space), cls(true, `${SPACE}\r\n`), rep0(lineChar))));
const word = (except) => seq(
  ...(except ? [not(tok(seq(lit(except), not(wordChar))))] : []),
  alias('word', tok(rep1(wordChar))),
);
const keywords = ['diff', 'Binary', 'index', 'similarity', 'new', 'deleted', 'old', 'rename', '@@', '+', '-', '#'];
const marker = (text) => imm(lit(text));

const rules = [
  ['source', 'normal', seq(
    rep0(ord(ref('block'), seq(ref('line'), newline), blankLine)),
    maybe(ref('line')),
  )],
  ['line', 'silent', ord(
    ref('file_change'), ref('binary_change'), ref('index'), ref('similarity'), ref('old_file'), ref('new_file'),
    ref('location'), ref('addition'), ref('deletion'), ref('context'), ref('comment'),
  )],
  ['header', 'silent', ord(ref('file_change'), ref('binary_change'), ref('index'), ref('similarity'))],
  ['block', 'normal', seq(
    ref('command'), lineEnd,
    rep0(seq(ref('header'), lineEnd)), not(seq(ref('header'), lineEnd)),
    maybe(seq(ref('old_file'), newline, ref('new_file'), newline, ref('hunks'))),
  )],
  ['hunks', 'normal', seq(rep1(ref('hunk')), not(seq(ref('location'), lineEnd)))],
  ['hunk', 'normal', seq(field('location', ref('location')), lineEnd, maybe(field('changes', ref('changes'))))],
  ['changes', 'normal', seq(
    rep1(seq(ref('change'), ord(seq(rep1(newline), not(newline)), seq(rep0(space), not(imm('any')))))),
    not(seq(ref('change'), lineEnd)),
  )],
  ['change', 'silent', ord(ref('addition'), ref('deletion'), ref('context'))],
  ['command', 'normal', seq(marker('diff'), alias('argument', tok(rep1(wordCharacter))), ref('filename'))],
  ['file_change', 'normal', ord(
    seq(ord(marker('new'), marker('deleted')), lit('file'), lit('mode'), ref('mode')),
    seq(ord(marker('new'), marker('old')), lit('mode'), ref('mode')),
    seq(marker('rename'), ord(lit('from'), lit('to')), ref('filename')),
  )],
  ['binary_change', 'normal', seq(
    marker('Binary'), lit('files'), alias('filename', rep1(word('and'))), lit('and'),
    alias('filename', rep1(word('differ'))), lit('differ'),
  )],
  ['index', 'normal', seq(marker('index'), ref('commit'), lit('..'), ref('commit'), maybe(ref('mode')))],
  ['similarity', 'normal', seq(marker('similarity'), lit('index'), alias('score', tok(digits)), lit('%'))],
  ['old_file', 'normal', seq(marker('---'), not(imm(lit('-'))), ref('filename'))],
  ['new_file', 'normal', seq(marker('+++'), not(imm(lit('+'))), ref('filename'))],
  ['location', 'normal', seq(marker('@@'), ref('linerange'), ref('linerange'), lit('@@'), maybe(anything))],
  ...['addition', 'deletion'].map((name) => {
    const sign = name === 'addition' ? '+' : '-';
    return [name, 'normal', ord(
      seq(marker(sign.repeat(4)), maybe(anything)),
      // Merged from GNU diff: a hunk line that starts with the sign is a
      // changed line, the sign followed by the line's text.
      seq(marker(sign.repeat(3)), visible),
      marker(sign.repeat(3)),
      seq(marker(sign.repeat(2)), maybe(anything)),
      seq(marker(sign), maybe(anything)),
    )];
  }),
  ['context', 'silent', seq(
    not(imm(ord(...keywords.map(lit)))),
    not(imm(seq(rep0(space), ord(lit('\n'), lit('\r\n'))))),
    alias('context', imm(rep1(lineChar))),
  )],
  ['comment', 'normal', seq(marker('#'), maybe(anything))],
  ['filename', 'normal', rep1(word(null))],
  ['commit', 'silent', alias('commit', tok(`(repeat 4 40 ${hex})`))],
  ['mode', 'silent', alias('mode', tok(digits))],
  ['linerange', 'silent', alias('linerange', tok(seq(cls(false, '+-'), digits, opt(seq(lit(','), digits)))))],
];

const text = [
  '(grammar (start source))',
  `(extra ${space})`,
  ...rules.map(([name, kind, expression]) => `(rule ${name} ${kind} ${expression})`),
].join('\n');
const canonical = renderGrammarLinks(parseGrammarLinks(text));
writeFileSync(new URL('../../parity/grammars/native/diff.lino', import.meta.url), canonical);
console.log(canonical.split('\n').length, 'lines');
