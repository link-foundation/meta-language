// Builds parity/grammars/scanners/javascript.lino, the native port of
// tree-sitter-javascript v0.25.0's src/scanner.c (sha256 b3d3f642...42c7, MIT):
// one (scanner NAME (tokens ...) (operations ...)) line per external token.
// Each operation list follows the C function of the same token:
// automatic_semicolon: scan_automatic_semicolon with
// scan_whitespace_and_comments inlined (the token is empty: `mark` at its
// start, as mark_end there; `comment_condition`, whether `||` is not valid,
// is `(not (expected (literal ||)))`); template_characters:
// scan_template_chars; ternary_question_mark: scan_ternary_qmark;
// html_comment: scan_html_comment, scanned only where none of `||`,
// escape_sequence and regex_pattern is valid, as the C dispatch asks (the
// check comes last, where a comment was read, so a parse asks for it only
// there; the white space before it is the extras'); jsx_text: scan_jsx_text. Scanner temporaries
// are stacks the operations empty before they emit, so they leave no state.
// `iswspace`, `iswalpha` and `iswdigit` are those of the C locale the
// tree-sitter CLI runs the scanner in: ASCII.
//
//   node js/experiments/build-javascript-scanner.mjs > parity/grammars/scanners/javascript.lino
const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const next = (text) => `(next ${lit(text)})`;
const any = (...texts) => `(some ${texts.map(next).join(' ')})`;
const WS = '(class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))';
const DIGIT = '(class plain (range 0 9))';
const ALPHA = '(class plain (range a z) (range A Z))';
const LINE_END = any('\n', '\u2028', '\u2029');
const zero = '(integer 0)';
const one = '(integer 1)';
const isSet = (stack) => `(greater (depth ${stack}) ${zero})`;
const set = (stack) => `(if (equal (depth ${stack}) ${zero}) (then (push ${stack} ${one})))`;
const clear = (stack) => `(while ${isSet(stack)} (do (pop ${stack})))`;
const NOT_OR = `(not (expected ${lit('||')}))`;

// scan_whitespace_and_comments with `consume` false or true. REJECT fails the
// scan. With `consume` false the result is `accept` set for ACCEPT, unset for
// NO_NEWLINE; `block_newline` is saw_block_newline and `scanning` the outer
// loop.
function whitespaceAndComments(consume) {
  const blockEnd = consume
    ? `advance ${clear('block')}`
    : `advance ${clear('block')} (if (not ${next('/')}) (then (if ${isSet('block_newline')} (then (push accept ${one}))) ${clear('scanning')}))`;
  return [
    `(push scanning ${one})`,
    `(while ${isSet('scanning')} (do`,
    `(while (next ${WS}) (do advance))`,
    `(if ${next('/')} (then advance`,
    `(if ${next('/')} (then advance (while (not (some atEnd ${LINE_END})) (do advance)))`,
    `(else (if ${next('*')} (then advance (push block ${one})`,
    `(while (all ${isSet('block')} (not atEnd)) (do`,
    `(if ${next('*')} (then advance (if ${next('/')} (then ${blockEnd})))`,
    `(else (if ${LINE_END} (then ${set('block_newline')} advance) (else advance))))))`,
    clear('block'),
    ')',
    '(else fail)))))',
    `(else ${consume ? '' : `(push accept ${one}) `}${clear('scanning')}))))`,
    clear('block_newline'),
  ].join(' ');
}

const automaticSemicolon = [
  '(if atEnd (then (emit automatic_semicolon)))',
  'mark',
  `(push looking ${one})`,
  `(while ${isSet('looking')} (do`,
  `(if atEnd (then ${clear('looking')} (emit automatic_semicolon)))`,
  `(if ${next('/')} (then ${whitespaceAndComments(false)}`,
  `(if (all ${isSet('accept')} ${NOT_OR} (not ${any(',', '=')})) (then ${clear('accept')} ${clear('looking')} (emit automatic_semicolon)))`,
  `${clear('accept')}))`,
  `(if ${next('}')} (then ${clear('looking')} (emit automatic_semicolon)))`,
  `(if ${LINE_END} (then ${clear('looking')}) (else (if (not (next ${WS})) (then fail)) advance))))`,
  'advance',
  whitespaceAndComments(true),
  `(if ${any('`', ',', ':', ';', '*', '%', '>', '<', '=', '[', '(', '?', '^', '|', '&', '/')} (then fail))`,
  `(if ${next('.')} (then (if (next (seq ${lit('.')} ${DIGIT})) (then (emit automatic_semicolon)) (else fail))))`,
  `(if ${next('+')} (then (if ${next('++')} (then (emit automatic_semicolon)) (else fail))))`,
  `(if ${next('-')} (then (if ${next('--')} (then (emit automatic_semicolon)) (else fail))))`,
  `(if ${next('!')} (then (if ${next('!=')} (then fail) (else (emit automatic_semicolon)))))`,
  `(if ${next('in')} (then (if (not (next (seq ${lit('in')} ${ALPHA}))) (then fail)) (if ${next('instanceof')} (then (if (not (next (seq ${lit('instanceof')} ${ALPHA}))) (then fail))))))`,
  '(emit automatic_semicolon)',
];

const templateCharacters = [
  `(while (not atEnd) (do mark (if ${any('`', '\\', '${')} (then (if ${isSet('content')} (then ${clear('content')} (emit template_characters)) (else fail)))) advance ${set('content')}))`,
  'fail',
];

const ternaryQuestionMark = [
  `(while (next ${WS}) (do (skip ${WS})))`,
  `(consume ${lit('?')})`,
  `(if ${next('?')} (then fail))`,
  'mark',
  `(if ${next('.')} (then advance (if (not (next ${DIGIT})) (then fail))))`,
  '(emit ternary_question_mark)',
];

const htmlComment = [
  `(if ${next('<!--')} (then (consume ${lit('<!--')})) (else (consume ${lit('-->')})))`,
  `(while (not (some atEnd ${LINE_END})) (do advance))`,
  `(if (some (expected ${lit('||')}) (expected (ref escape_sequence)) (expected (ref regular_expression_pattern))) (then fail))`,
  'mark',
  '(emit html_comment)',
];

const jsxText = [
  `(while (not (some atEnd ${any('<', '>', '{', '}', '&')})) (do`,
  `(if ${next('\n')} (then ${set('at_newline')}) (else (if (not (next ${WS})) (then ${clear('at_newline')})) (if (not ${isSet('at_newline')}) (then ${set('text')}))))`,
  'advance))',
  clear('at_newline'),
  `(if ${isSet('text')} (then ${clear('text')} (emit jsx_text)))`,
  'fail',
];

const scanners = [
  ['automatic_semicolon', automaticSemicolon],
  ['template_characters', templateCharacters],
  ['ternary_question_mark', ternaryQuestionMark],
  ['html_comment', htmlComment],
  ['jsx_text', jsxText],
];
const text = scanners.map(([name, ops]) => `(scanner ${name} (tokens ${name}) (operations ${ops.join(' ')}))`).join('\n') + '\n';
process.stdout.write(text);
