// Builds parity/grammars/native/racket.lino from the readable expressions
// below and writes it in canonical Links Notation (parse, then render).
//   node experiments/native-racket-build.mjs
//
// The rules port tree-sitter-racket 0.25.0 (the oracle that backs the default
// Racket parse), which follows The Racket Reference section 1.3 (The Reader).
// tree-sitter reads the input with a longest-match lexer; an ordered choice
// here commits to its first alternative, so every leading token that is a
// prefix of a longer token is guarded by a negative lookahead. The here string
// body, which the oracle reads with an external C scanner, is a rule action:
// the terminator line is stored in a variable and the body ends at the first
// line equal to it.
import { writeFileSync } from 'node:fs';

import { parseGrammarLinks, renderGrammarLinks } from '../src/index.js';

const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const chars = (text) => [...text].map((c) => `(char ${enc(c)})`);
const range = (from, to) => `(range ${enc(from)} ${enc(to)})`;
const cls = (negated, items) => `(class ${negated ? 'negated' : 'plain'} ${items.join(' ')})`;
const seq = (...items) => `(seq ${items.join(' ')})`;
const ord = (...items) => `(choice ordered ${items.join(' ')})`;
const any = (...items) => `(choice unordered ${items.join(' ')})`;
const imm = (item) => `(immediateToken ${item})`;
const ref = (name) => `(ref ${name})`;
const not = (item) => `(not ${item})`;
const opt = (item) => `(optional ${item})`;
const rep0 = (item) => `(repeat0 ${item})`;
const rep1 = (item) => `(repeat1 ${item})`;
const repeat = (min, max, item) => `(repeat ${min} ${max} ${item})`;
const alias = (name, item) => `(alias ${name} ${item})`;
const longest = (...items) => `(longest ${items.join(' ')})`;
const lexical = (level, item) => `(lexicalPrecedence ${level} ${item})`;
// A case-insensitive letter, as the oracle's [hH] classes.
const ci = (text) => seq(...[...text].map((c) => (c.toLowerCase() === c.toUpperCase() ? lit(c) : cls(false, chars(`${c.toLowerCase()}${c.toUpperCase()}`)))));

// LEAF.whitespace, LEAF.newline and LEAF.delimiter of the oracle.
const whitespaceItems = [...chars(' \t\n\v\f\r\u0085  '), range(' ', ' '), ...chars('    　﻿')];
const newlineChars = '\r\n\u0085  ';
const nonNewline = cls(true, chars(newlineChars));
const delimiterItems = [...whitespaceItems, ...chars('(){}",\'`;[]')];

// LEAF.symbol_start and LEAF.symbol_remain. The oracle's `.` takes any
// character but a line feed; the merged grammar follows Racket Reference
// section 1.3.1, where a backslash quotes the next character, line feed
// included.
const bars = seq(lit('|'), rep0(cls(true, chars('|'))), lit('|'));
const anyChar = any(cls(true, chars('\n')), lit('\n'));
const escaped = seq(lit('\\'), anyChar);
const symbolStart = any(cls(true, [...delimiterItems, ...chars('#|\\')]), lit('#%'), bars, escaped);
const symbolRemain = any(cls(true, [...delimiterItems, ...chars('|\\')]), bars, escaped);

const hex = cls(false, [range('0', '9'), range('a', 'f'), range('A', 'F')]);
const digits = {
  2: cls(false, chars('01')),
  8: range('0', '7'),
  10: range('0', '9'),
  16: hex,
};
const decimalDigit = digits[10];
const sign = cls(false, chars('+-'));
const prefixes = {
  2: seq(lit('#'), cls(false, chars('bB'))),
  8: seq(lit('#'), cls(false, chars('oO'))),
  10: opt(seq(lit('#'), cls(false, chars('dD')))),
  16: seq(lit('#'), cls(false, chars('xX'))),
};
const infnan = (marks) => any(
  seq(ci('inf'), lit('.'), cls(false, chars(marks))),
  seq(ci('nan'), lit('.'), cls(false, chars(marks))),
);

function inexactSimple(n) {
  const digit = digits[n];
  const digitsHash = seq(rep1(digit), rep0(lit('#')));
  return any(
    seq(digitsHash, opt(lit('.')), rep0(lit('#'))),
    seq(opt(rep1(digit)), lit('.'), digitsHash),
    seq(digitsHash, lit('/'), digitsHash),
  );
}

// number_base(n) of the oracle.
function numberBase(n) {
  const digit = digits[n];
  const expMark = n === 16 ? cls(false, chars('slSL')) : cls(false, chars('sldefSLDEF'));
  const exactness = seq(lit('#'), cls(false, chars('eiEI')));
  const unsignedInteger = rep1(digit);
  const exactInteger = seq(opt(sign), unsignedInteger);
  const unsignedRational = any(unsignedInteger, seq(unsignedInteger, lit('/'), unsignedInteger));
  const exactRational = seq(opt(sign), unsignedRational);
  const exactComplex = seq(opt(exactRational), sign, opt(unsignedRational), cls(false, chars('iI')));
  const exact = any(exactRational, exactComplex);
  const special = infnan('0fF');
  const normal = seq(inexactSimple(n), opt(seq(expMark, exactInteger)));
  const unsigned = any(normal, special);
  const real = any(seq(opt(sign), normal), seq(sign, special));
  const complex = any(
    seq(opt(real), sign, opt(unsigned), cls(false, chars('iI'))),
    seq(real, lit('@'), real),
  );
  const inexact = any(real, complex);
  return seq(any(seq(opt(exactness), prefixes[n]), seq(prefixes[n], opt(exactness))), any(exact, inexact));
}

// extflonum(n) of the oracle.
function extflonum(n) {
  const exactInteger = seq(opt(sign), rep1(digits[n]));
  const normal = seq(inexactSimple(n), opt(seq(cls(false, chars('tT')), exactInteger)));
  const real = any(seq(opt(sign), normal), seq(sign, infnan('0fFtT')));
  return seq(prefixes[n], real);
}

const bases = [2, 8, 10, 16];
const number = any(...bases.map(extflonum), ...bases.map(numberBase));

const character = seq(lit('#\\'), any(
  ...['nul', 'null', 'backspace', 'tab', 'newline', 'linefeed', 'vtab', 'page', 'return', 'space', 'rubout'].map(lit),
  repeat(3, 3, digits[8]),
  seq(lit('u'), repeat(1, 4, hex)),
  seq(lit('U'), repeat(1, 8, hex)),
  // Racket Reference section 1.3.14 reads #\ followed by any character,
  // line feed included; the oracle stops at a line feed.
  anyChar,
));

const escapeSequence = seq(lit('\\'), any(
  cls(false, chars('abtnvfre"\'\\')),
  repeat(1, 3, digits[8]),
  seq(lit('x'), repeat(1, 2, hex)),
  seq(lit('u'), repeat(1, 4, hex)),
  seq(lit('U'), repeat(1, 8, hex)),
  cls(false, chars('\r\n')),
  lit('\r\n'),
));

// _line_comment: `#! ` or `#!/` up to a line end, continued by a backslash
// before the line break.
const lineComment = seq(
  any(lit('#! '), lit('#!/')),
  rep0(seq(rep0(nonNewline), lit('\\'), cls(false, chars(newlineChars)))),
  rep0(nonNewline),
);

const langChar = cls(false, [range('a', 'z'), range('A', 'Z'), range('0', '9'), ...chars('+_-')]);
const langName = any(langChar, seq(langChar, rep0(any(langChar, lit('/'))), langChar));

// A prefixed datum: the prefix, white space and comments, the datum.
const skips = rep0(ref('_skip'));
const prefixed = (marker, ...guard) => seq(lit(marker), ...guard, skips, ref('_datum'));
const realString = seq(
  lit('"'),
  rep0(ord(ref('escape_sequence'), alias('string_text', imm(rep1(cls(true, chars('"\\'))))))),
  lit('"'),
);
const list = ord(
  seq(lit('('), rep0(ord(ref('dot'), ref('_token'))), lit(')')),
  seq(lit('['), rep0(ord(ref('dot'), ref('_token'))), lit(']')),
  seq(lit('{'), rep0(ord(ref('dot'), ref('_token'))), lit('}')),
);
const hereLine = rep0(cls(true, chars('\n')));

const rules = [
  // The oracle's runtime skips a byte order mark at the start of the input.
  ['program', 'normal', ord(
    seq(alias('byte_order_mark', imm(lit('\ufeff'))), rep0(ref('_token'))),
    rep0(ref('_token')),
  )],
  ['_token', 'silent', ord(ref('_skip'), ref('extension'), ref('_datum'))],
  ['_skip', 'silent', ord(
    alias('whitespace', imm(rep1(cls(false, whitespaceItems)))),
    ref('comment'),
    ref('sexp_comment'),
    ref('block_comment'),
  )],
  // `.` is a symbol too; the oracle's lexer prefers the string token on a
  // tie, and a longer symbol or number otherwise.
  ['dot', 'token', seq(lit('.'), not(symbolRemain))],
  ['comment', 'token', any(seq(lit(';'), rep0(nonNewline)), lineComment)],
  ['block_comment', 'normal', seq(
    lit('#|'),
    rep0(ord(ref('block_comment'), alias('comment_text', imm(rep1(seq(not(lit('#|')), not(lit('|#')), 'any')))))),
    lit('|#'),
  )],
  ['sexp_comment', 'normal', prefixed('#;')],
  ['_datum', 'silent', ord(
    ref('boolean'),
    ref('string'),
    ref('here_string'),
    ref('byte_string'),
    ref('character'),
    // The longer of a number and a symbol; for a match of the same length
    // the number's lexical precedence wins, as the oracle declares it first.
    longest(ref('number'), ref('symbol')),
    ref('keyword'),
    ref('regex'),
    ref('box'),
    ref('graph'),
    ref('structure'),
    ref('hash'),
    ref('quote'),
    ref('quasiquote'),
    ref('syntax'),
    ref('quasisyntax'),
    ref('unquote_splicing'),
    ref('unquote'),
    ref('unsyntax_splicing'),
    ref('unsyntax'),
    ref('list'),
    ref('vector'),
  )],
  // `#fl` and `#fx` are the longer vector prefixes.
  ['boolean', 'token', any(
    lit('#true'), lit('#t'), lit('#T'), lit('#false'), seq(lit('#f'), not(cls(false, chars('lx')))), lit('#F'),
  )],
  ['string', 'normal', realString],
  ['byte_string', 'normal', seq(lit('#'), realString)],
  ['here_string', 'normal', seq(
    lit('#<<'),
    ref('here_terminator'),
    rep0(seq(ref('here_newline'), ref('here_line'))),
    ref('here_newline'),
    ref('here_end'),
  )],
  // The terminator line, stored for the lines below.
  ['here_terminator', 'token', hereLine, 'action', '(action (set here matched))'],
  ['here_newline', 'token', lit('\n')],
  // A body line differs from the terminator; the line equal to it ends the
  // here string.
  ['here_line', 'token', hereLine, 'action', '(action (if (equal matched (variable here)) (then fail)))'],
  ['here_end', 'token', hereLine, 'action', '(action (if (not (equal matched (variable here))) (then fail)))'],
  ['regex', 'normal', seq(alias('regex_prefix', imm(any(lit('#rx'), lit('#px'), lit('#rx#'), lit('#px#')))), realString)],
  ['escape_sequence', 'token', escapeSequence],
  ['number', 'token', lexical(1, number)],
  ['decimal', 'token', rep1(decimalDigit)],
  ['character', 'token', character],
  ['symbol', 'token', any(seq(lit('#'), cls(false, chars('cC')), cls(false, chars('iIsS'))), seq(symbolStart, rep0(symbolRemain)))],
  ['keyword', 'token', seq(lit('#:'), rep0(symbolRemain))],
  ['box', 'normal', prefixed('#&')],
  ['list', 'normal', list],
  ['vector', 'normal', seq(ord(lit('#fl'), lit('#fx'), lit('#')), opt(ref('decimal')), ref('list'))],
  ['structure', 'normal', seq(lit('#s'), ref('list'))],
  ['hash', 'normal', seq(alias('hash_prefix', imm(seq(ci('#hash'), opt(any(ci('alw'), ci('eqv'), ci('eq')))))), ref('list'))],
  // `#` and 1 to 8 digits, then `=` or `#` (section 1.3.17).
  ['graph', 'normal', ord(
    alias('graph_mark', imm(seq(lit('#'), repeat(1, 8, decimalDigit), lit('#')))),
    seq(alias('graph_mark', imm(seq(lit('#'), repeat(1, 8, decimalDigit), lit('=')))), skips, ref('_datum')),
  )],
  ['quote', 'normal', prefixed('\'')],
  ['quasiquote', 'normal', prefixed('`')],
  ['syntax', 'normal', prefixed('#\'')],
  ['quasisyntax', 'normal', prefixed('#`')],
  // The lexer reads `,@` and `#,@` whole, so `,` and `#,` never precede `@`.
  ['unquote', 'normal', prefixed(',', not(lit('@')))],
  ['unquote_splicing', 'normal', prefixed(',@')],
  ['unsyntax', 'normal', prefixed('#,', not(lit('@')))],
  ['unsyntax_splicing', 'normal', prefixed('#,@')],
  ['extension', 'normal', ord(
    seq(lit('#reader'), skips, ref('_datum')),
    seq(ord(lit('#lang '), lit('#!')), ref('lang_name')),
  )],
  // It must not start or end with `/` (section 1.3.18).
  ['lang_name', 'token', langName],
];

const text = [
  '(grammar (start program))',
  ...rules.map(([name, kind, expression, , action]) => `(rule ${name} ${kind} ${expression}${action ? ` ${action}` : ''})`),
].join('\n');
const canonical = renderGrammarLinks(parseGrammarLinks(text));
writeFileSync(new URL('../../parity/grammars/native/racket.lino', import.meta.url), canonical);
console.log(canonical.split('\n').length, 'lines');
