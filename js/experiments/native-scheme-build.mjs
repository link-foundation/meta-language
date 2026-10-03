// Builds parity/grammars/native/scheme.lino from the readable expressions
// below and writes it in canonical Links Notation (parse, then render).
//   node experiments/native-scheme-build.mjs
//
// The lexical rules port the R5RS, R6RS and R7RS rules of tree-sitter-scheme
// 0.24.7 (the oracle that backs the default Scheme parse) and its extensions;
// R7RS small adds the `#u8(` bytevector (sections 6.9 and 7.1.2) and datum
// labels (section 2.4), which the oracle recovers from.
import { writeFileSync } from 'node:fs';

import { parseGrammarLinks, renderGrammarLinks } from '../src/index.js';

const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const ci = (text) => `(literalInsensitive ${enc(text)})`;
const chars = (text) => [...text].map((c) => `(char ${enc(c)})`);
const cls = (negated, items) => `(class ${negated ? 'negated' : 'plain'} ${items.join(' ')})`;
const category = (name) => `(category ${name})`;
const seq = (...items) => `(seq ${items.join(' ')})`;
const ord = (...items) => `(choice ordered ${items.join(' ')})`;
const any = (...items) => `(choice unordered ${items.join(' ')})`;
const imm = (item) => `(immediateToken ${item})`;
const ref = (name) => `(ref ${name})`;
const not = (item) => `(not ${item})`;
const opt = (item) => `(optional ${item})`;
const rep0 = (item) => `(repeat0 ${item})`;
const rep1 = (item) => `(repeat1 ${item})`;
const alias = (name, item) => `(alias ${name} ${item})`;
const longest = (...items) => `(longest ${items.join(' ')})`;
const lexical = (level, item) => `(lexicalPrecedence ${level} ${item})`;
const empty = 'empty';

const hex = cls(false, ['(range 0 9)', '(range a f)', '(range A F)']);
const digits = {
  2: cls(false, chars('01')),
  8: '(range 0 7)',
  10: '(range 0 9)',
  16: hex,
};
const sign = opt(cls(false, chars('+-')));
const signed = cls(false, chars('+-'));

// common.whitespace, common.intra_whitespace, common.line_ending and
// common.symbol_element.
const whitespaceChars = [...chars(' \r\n\t\f\v'), category('Zs'), category('Zl'), category('Zp')];
const intraWhitespace = cls(false, [...chars('\t'), category('Zs')]);
const lineEnding = any(cls(false, chars('\n\r \u0085')), lit('\r\n'), lit('\r\u0085'));
const symbolElement = cls(true, [...whitespaceChars, ...chars('#;"\'`,()[]{}\\|')]);

// hidden_node.symbol: a run of symbol elements or one R7RS `|...|` symbol.
const symbolBody = any(
  rep1(symbolElement),
  seq(lit('|'), rep0(any(
    rep1(cls(true, chars('|\\'))),
    seq(lit('\\'), cls(false, chars('xX')), rep1(hex), lit(';')),
    seq(lit('\\'), cls(false, chars('abtnr'))),
    lit('\\|'),
  )), lit('|')),
);

// The number prefix: a radix and an exactness in either order.
const radix = {
  2: cls(false, chars('bB')),
  8: cls(false, chars('oO')),
  10: cls(false, chars('dD')),
  16: cls(false, chars('xX')),
};
function prefix(n) {
  const radixPart = n === 10 ? opt(seq(lit('#'), radix[n])) : seq(lit('#'), radix[n]);
  const exactness = opt(seq(lit('#'), cls(false, chars('ieIE'))));
  return any(seq(radixPart, exactness), seq(exactness, radixPart));
}

function r5rsNumber(n) {
  const digit = digits[n];
  const suffix = opt(seq(cls(false, chars('eEsSfFdDlL')), sign, rep1(digits[10])));
  const uinteger = seq(rep1(digit), rep0(lit('#')));
  const decimal = n === 10
    ? any(
      seq(uinteger, suffix),
      seq(lit('.'), rep1(digit), rep0(lit('#')), suffix),
      seq(rep1(digit), lit('.'), rep0(digit), rep0(lit('#')), suffix),
      seq(rep1(digit), rep1(lit('#')), lit('.'), rep0(lit('#')), suffix),
    )
    : empty;
  const ureal = any(uinteger, seq(uinteger, lit('/'), uinteger), decimal);
  const real = seq(sign, ureal);
  const complex = any(real, seq(real, lit('@'), real), seq(opt(real), signed, opt(ureal), lit('i')));
  return seq(prefix(n), complex);
}

function r6rsNumber(n) {
  const digit = digits[n];
  const suffix = opt(seq(cls(false, chars('eEsSfFdDlL')), sign, rep1(digits[10])));
  const uinteger = rep1(digit);
  const decimal = n === 10
    ? any(
      seq(uinteger, suffix),
      seq(lit('.'), rep1(digit), suffix),
      seq(rep1(digit), lit('.'), rep0(digit), suffix),
      seq(rep1(digit), lit('.'), suffix),
    )
    : empty;
  const mantissaWidth = opt(seq(lit('|'), rep1(digits[10])));
  const naninf = any(lit('nan.0'), lit('inf.0'));
  const ureal = any(uinteger, seq(uinteger, lit('/'), uinteger), seq(decimal, mantissaWidth));
  const real = any(seq(sign, ureal), seq(signed, naninf));
  const complex = any(
    real,
    seq(real, lit('@'), real),
    seq(opt(real), signed, opt(any(ureal, naninf)), lit('i')),
  );
  return seq(prefix(n), complex);
}

function r7rsNumber(n) {
  const digit = digits[n];
  const infnan = seq(signed, any(ci('inf.0'), ci('nan.0')));
  const suffix = opt(seq(cls(false, chars('eE')), sign, rep1(digit)));
  const uinteger = rep1(digit);
  const decimal = n === 10
    ? any(
      seq(uinteger, suffix),
      seq(lit('.'), rep1(digit), suffix),
      seq(rep1(digit), lit('.'), rep0(digit), suffix),
    )
    : empty;
  const ureal = any(uinteger, seq(uinteger, lit('/'), uinteger), decimal);
  const real = any(seq(sign, ureal), infnan);
  const complex = any(
    real,
    seq(real, lit('@'), real),
    seq(real, signed, ureal, lit('i')),
    seq(real, signed, lit('i')),
    seq(real, infnan, lit('i')),
    seq(signed, ureal, lit('i')),
    seq(infnan, lit('i')),
    seq(signed, lit('i')),
  );
  return seq(prefix(n), complex);
}

const bases = [2, 8, 10, 16];
const number = any(...bases.map(r5rsNumber), ...bases.map(r6rsNumber), ...bases.map(r7rsNumber));

const boolean = seq(lit('#'), any(cls(false, chars('tTfF')), ci('true'), ci('false')));

const character = seq(lit('#\\'), any(
  // R5RS: case-insensitive names and any character.
  ci('space'), ci('newline'), 'any',
  // R6RS.
  ...['nul', 'alarm', 'backspace', 'tab', 'linefeed', 'newline', 'vtab', 'page', 'return', 'esc', 'space', 'delete'].map(lit),
  seq(lit('x'), rep1(hex)),
  seq(lit('u'), rep1(hex)),
  // R7RS.
  ...['alarm', 'backspace', 'delete', 'escape', 'newline', 'null', 'return', 'space', 'tab'].map(lit),
  seq(cls(false, chars('xX')), rep1(hex)),
  // tree-sitter-scheme extensions.
  ...['bel', 'ls', 'nel', 'rubout', 'vt'].map(lit),
));

const escapeSequence = seq(lit('\\'), any(
  // R5RS.
  lit('"'), lit('\\'),
  // R6RS.
  cls(false, chars('abtnvfr"\\')),
  seq(lit('x'), rep1(hex), lit(';')),
  seq(intraWhitespace, lineEnding, intraWhitespace),
  // R7RS.
  cls(false, chars('abtnr"\\')),
  seq(rep0(intraWhitespace), lineEnding, rep0(intraWhitespace)),
  seq(cls(false, chars('xX')), rep1(hex), lit(';')),
  // The extension `\\.`: any character but a line feed.
  cls(true, chars('\n')),
));

// A prefixed datum: the prefix, inter-token space and comments, the datum.
const prefixed = (marker, ...guard) => seq(lit(marker), ...guard, rep0(ref('_intertoken')), ref('_datum'));

const rules = [
  ['program', 'normal', rep0(ref('_token'))],
  ['_token', 'silent', ord(ref('_intertoken'), ref('_datum'))],
  ['_intertoken', 'silent', ord(
    alias('whitespace', imm(rep1(cls(false, whitespaceChars)))),
    ref('directive'),
    ref('comment'),
    ref('block_comment'),
  )],
  ['comment', 'normal', ord(
    alias('comment_text', imm(seq(lit(';'), rep0(cls(true, chars('\n')))))),
    prefixed('#;'),
  )],
  ['directive', 'normal', seq(lit('#!'), rep0(ref('_intertoken')), alias('directive_name', imm(symbolBody)))],
  ['block_comment', 'normal', seq(
    lit('#|'),
    rep0(ord(ref('block_comment'), alias('comment_text', imm(rep1(seq(not(lit('#|')), not(lit('|#')), 'any')))))),
    lit('|#'),
  )],
  ['_datum', 'silent', ord(
    ref('boolean'),
    ref('character'),
    ref('keyword'),
    // The longer of a number and a symbol; for a match of the same length
    // the number's lexical precedence wins, as tree-sitter-scheme declares
    // it first.
    longest(ref('number'), ref('symbol')),
    ref('string'),
    ref('vector'),
    ref('byte_vector'),
    ref('datum_label'),
    ref('datum_reference'),
    ref('list'),
    ref('quote'),
    ref('quasiquote'),
    ref('syntax'),
    ref('quasisyntax'),
    ref('unsyntax_splicing'),
    ref('unsyntax'),
    ref('unquote_splicing'),
    ref('unquote'),
  )],
  ['boolean', 'token', boolean],
  ['number', 'token', lexical(1, number)],
  ['character', 'token', character],
  ['string', 'normal', seq(
    lit('"'),
    rep0(ord(ref('escape_sequence'), alias('string_text', imm(rep1(cls(true, chars('"\\'))))))),
    lit('"'),
  )],
  ['escape_sequence', 'token', escapeSequence],
  ['symbol', 'token', symbolBody],
  ['keyword', 'token', seq(lit('#:'), symbolBody)],
  ['list', 'normal', ord(
    seq(lit('('), rep0(ref('_token')), lit(')')),
    seq(lit('['), rep0(ref('_token')), lit(']')),
    seq(lit('{'), rep0(ref('_token')), lit('}')),
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
  ['vector', 'normal', seq(lit('#('), rep0(ref('_token')), lit(')'))],
  // The R6RS `#vu8(` of tree-sitter-scheme and the R7RS small `#u8(`
  // (sections 6.9 and 7.1.2), which reads its bytes as the oracle reads `#vu8(`.
  ['byte_vector', 'normal', ord(
    seq(lit('#vu8('), rep0(ref('_token')), lit(')')),
    seq(lit('#u8('), rep0(ref('_token')), lit(')')),
  )],
  // R7RS small section 2.4 (Datum labels): `#<n>=<datum>` and `#<n>#`.
  ['datum_label', 'normal', seq(ref('label'), rep0(ref('_intertoken')), ref('_datum'))],
  ['label', 'token', seq(lit('#'), rep1(digits[10]), lit('='))],
  ['datum_reference', 'token', seq(lit('#'), rep1(digits[10]), lit('#'))],
];

const text = [
  '(grammar (start program))',
  ...rules.map(([name, kind, expression]) => `(rule ${name} ${kind} ${expression})`),
].join('\n');
const canonical = renderGrammarLinks(parseGrammarLinks(text));
writeFileSync(new URL('../../parity/grammars/native/scheme.lino', import.meta.url), canonical);
console.log(canonical.split('\n').length, 'lines');
