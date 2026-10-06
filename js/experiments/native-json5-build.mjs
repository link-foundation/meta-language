// Builds parity/grammars/native/json5.lino from the readable expressions
// below and writes it in canonical Links Notation (parse, then render).
//   node experiments/native-json5-build.mjs
import { writeFileSync } from 'node:fs';

import { parseGrammarLinks, renderGrammarLinks } from '../src/index.js';

const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const chars = (text) => [...text].map((c) => `(char ${enc(c)})`);
const cls = (negated, items) => `(class ${negated ? 'negated' : 'plain'} ${items.join(' ')})`;
const category = (name) => `(category ${name})`;
const seq = (...items) => `(seq ${items.join(' ')})`;
const ord = (...items) => `(choice ordered ${items.join(' ')})`;
const imm = (item) => `(immediateToken ${item})`;
const ref = (name) => `(ref ${name})`;
const not = (item) => `(not ${item})`;
const opt = (item) => `(optional ${item})`;
const rep0 = (item) => `(repeat0 ${item})`;
const rep1 = (item) => `(repeat1 ${item})`;
const field = (label, item) => `(capture labeled ${label} ${item})`;

const digit = '(range 0 9)';
const hex = cls(false, ['(range 0 9)', '(range a f)', '(range A F)']);
const hex4 = seq(hex, hex, hex, hex);

// tree-sitter-json5-orchard skips /\s/, which its lexer reads as the ASCII
// spaces; JSON5 section 8 (White Space) adds NBSP, LS, PS, the byte order mark and the
// other Zs spaces.
const whitespace = cls(false, [...chars('\t\n\v\f\r    ﻿'), category('Zs')]);

// tree-sitter-json5-orchard: `//` up to a line feed, so a carriage return or
// LS stays in the comment, and `/* ... */`.
const comment = ord(
  seq(lit('//'), rep0(cls(true, chars('\n')))),
  seq(lit('/*'), rep0(ord(cls(true, chars('*')), seq(lit('*'), not(lit('/'))))), lit('*/')),
);

// A comma-separated list with an optional trailing comma (commaSep).
const commaSep = (item) => opt(seq(item, rep0(seq(lit(','), item)), opt(lit(','))));

// The oracle's identifier start is [$_\p{L}] and its part adds [0-9]; JSON5
// section 3 (ECMAScript 5.1 IdentifierName) adds Nl to the start, a \u escape
// anywhere, and Mn, Mc, Nd, Pc, ZWNJ and ZWJ to the part.
const unicodeEscape = seq(lit('\\u'), hex4);
const identifierStart = ord(cls(false, [...chars('$_'), category('L'), category('Nl')]), unicodeEscape);
const identifierPart = ord(
  cls(false, [...chars('$_‌‍'), category('L'), category('Nl'), category('Mn'), category('Mc'), category('Nd'), category('Pc')]),
  unicodeEscape,
);

// The oracle's escapes are ["'\\/bfnrtv], u and four hex digits, x and two
// hex digits, and a line feed or CR LF continuation. JSON5 section 5.1 (Escapes) adds
// \0 before a non-digit, any other character that is not a digit, x or u
// (NonEscapeCharacter), and a CR, LS or PS continuation.
const escape = seq(lit('\\'), ord(
  seq(lit('u'), hex4),
  seq(lit('x'), hex, hex),
  seq(lit('0'), not(digit)),
  cls(true, ['(range 0 9)', ...chars('xu')]),
));
// Strings may span lines, as in the oracle.
const quoted = (quote) => seq(lit(quote), rep0(ord(escape, rep1(cls(true, chars(`${quote}\\`))))), lit(quote));

const exponent = seq(cls(false, chars('eE')), opt(cls(false, chars('+-'))), rep1(digit));
const integer = ord(lit('0'), seq('(range 1 9)', rep0(digit)));
// The oracle's number, which reads `.` and `.e5` as numbers as well.
const number = seq(opt(cls(false, chars('+-'))), ord(
  seq(lit('0'), cls(false, chars('xX')), rep1(hex)),
  seq(integer, lit('.'), rep0(digit), opt(exponent)),
  seq(lit('.'), rep0(digit), opt(exponent)),
  seq(integer, opt(exponent)),
  lit('Infinity'),
  lit('NaN'),
));

const rules = [
  ['file', 'normal', ref('_value')],
  ['_value', 'silent', ord(ref('object'), ref('array'), ref('number'), ref('string'), ref('null'), ref('true'), ref('false'))],
  ['object', 'normal', seq(lit('{'), commaSep(ref('member')), lit('}'))],
  ['member', 'normal', seq(field('name', ord(ref('string'), ref('identifier'))), lit(':'), field('value', ref('_value')))],
  ['identifier', 'token', seq(identifierStart, rep0(identifierPart))],
  ['array', 'normal', seq(lit('['), commaSep(ref('_value')), lit(']'))],
  ['string', 'token', ord(quoted('"'), quoted("'"))],
  ['number', 'token', number],
  ['null', 'token', lit('null')],
  ['true', 'token', lit('true')],
  ['false', 'token', lit('false')],
  ['comment', 'token', comment],
];

const text = [
  '(grammar (start file))',
  `(extra ${whitespace})`,
  `(extra ${ref('comment')})`,
  ...rules.map(([name, kind, expression]) => `(rule ${name} ${kind} ${expression})`),
].join('\n');
const canonical = renderGrammarLinks(parseGrammarLinks(text));
writeFileSync(new URL('../../parity/grammars/native/json5.lino', import.meta.url), canonical);
console.log(canonical.split('\n').length, 'lines');
