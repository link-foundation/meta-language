// Builds parity/grammars/scanners/typescript.lino, the native port of
// tree-sitter-typescript v0.23.2's common/scanner.h (sha256 in
// parity/grammars/sources.json, MIT), which typescript/src/scanner.c and
// tsx/src/scanner.c both include: one (scanner NAME (tokens ...) (operations
// ...)) line per external token, shared by the TypeScript and TSX grammars.
// Each operation list follows the C function of the same token:
// automatic_semicolon: scan_automatic_semicolon with
// scan_whitespace_and_comments inlined (the token is empty: `mark` at its
// start, as mark_end there; `valid_symbols[LOGICAL_OR]`, which tells a type
// from an expression, is `(expected (literal ||))` and
// `valid_symbols[FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON]` is `(expected (ref
// function_signature_automatic_semicolon))`); template_characters:
// scan_template_chars; ternary_question_mark: scan_ternary_qmark;
// html_comment: scan_closing_comment, scanned only where none of `||`,
// escape_sequence and regex_pattern is valid, as the C dispatch asks;
// jsx_text: scan_jsx_text. The C scanner never returns
// FUNCTION_SIGNATURE_AUTOMATIC_SEMICOLON (it scans AUTOMATIC_SEMICOLON where
// either is valid, and function_signature accepts both) nor ERROR_RECOVERY,
// which only tree-sitter's error recovery asks about, so their scanners fail.
// Scanner temporaries are stacks the operations empty before they emit, so
// they leave no state. `iswspace`, `iswalpha` and `iswdigit` are those of the
// C locale the tree-sitter CLI runs the scanner in: ASCII.
//
//   node js/experiments/build-typescript-scanner.mjs > parity/grammars/scanners/typescript.lino
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
const OR = `(expected ${lit('||')})`;
const SIGNATURE = '(expected (ref function_signature_automatic_semicolon))';
const EMIT = '(emit automatic_semicolon)';

// scan_whitespace_and_comments: fails where a `/` starts no comment.
const whitespaceAndComments = [
  `(push scanning ${one})`,
  `(while ${isSet('scanning')} (do`,
  `(while (next ${WS}) (do advance))`,
  `(if ${next('/')} (then advance`,
  `(if ${next('/')} (then advance (while (not (some atEnd ${next('\n')})) (do advance)))`,
  `(else (if ${next('*')} (then advance (push block ${one})`,
  `(while (all ${isSet('block')} (not atEnd)) (do`,
  `(if ${next('*')} (then advance (if ${next('/')} (then advance ${clear('block')}))) (else advance))))`,
  `${clear('block')})`,
  '(else fail)))))',
  `(else ${clear('scanning')}))))`,
].join(' ');

const automaticSemicolon = [
  `(if atEnd (then ${EMIT}))`,
  'mark',
  `(push looking ${one})`,
  `(while ${isSet('looking')} (do`,
  `(if atEnd (then ${clear('looking')} ${EMIT}))`,
  `(if ${next('}')} (then ${clear('looking')} advance (while (next ${WS}) (do advance))`,
  `(if (all ${next(':')} (not ${OR})) (then fail)) ${EMIT}))`,
  `(if (not (next ${WS})) (then fail))`,
  `(if ${next('\n')} (then ${clear('looking')}) (else advance))))`,
  'advance',
  whitespaceAndComments,
  `(if ${any('`', ',', '.', ';', '*', '%', '>', '<', '=', '?', '^', '|', '&', '/', ':')} (then fail))`,
  `(if (all ${next('{')} ${SIGNATURE}) (then fail))`,
  `(if (all ${any('(', '[')} ${OR}) (then fail))`,
  `(if ${next('+')} (then (if ${next('++')} (then ${EMIT}) (else fail))))`,
  `(if ${next('-')} (then (if ${next('--')} (then ${EMIT}) (else fail))))`,
  `(if ${next('!')} (then (if ${next('!=')} (then fail) (else ${EMIT}))))`,
  `(if ${next('in')} (then (if (not (next (seq ${lit('in')} ${ALPHA}))) (then fail)) (if ${next('instanceof')} (then (if (not (next (seq ${lit('instanceof')} ${ALPHA}))) (then fail))))))`,
  EMIT,
];

const templateCharacters = [
  `(while (not atEnd) (do mark (if ${any('`', '\\', '${')} (then (if ${isSet('content')} (then ${clear('content')} (emit template_characters)) (else fail)))) advance ${set('content')}))`,
  'fail',
];

const ternaryQuestionMark = [
  `(while (next ${WS}) (do (skip ${WS})))`,
  `(consume ${lit('?')})`,
  `(if ${any('?', '.')} (then fail))`,
  'mark',
  `(while (next ${WS}) (do advance))`,
  `(if ${any(':', ')', ',')} (then fail))`,
  `(if ${next('.')} (then advance (if (not (next ${DIGIT})) (then fail))))`,
  '(emit ternary_question_mark)',
];

const htmlComment = [
  `(if ${next('<!--')} (then (consume ${lit('<!--')})) (else (consume ${lit('-->')})))`,
  `(while (not (some atEnd ${LINE_END})) (do advance))`,
  `(if (some ${OR} (expected (ref escape_sequence)) (expected (ref regular_expression_pattern))) (then fail))`,
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
  ['function_signature_automatic_semicolon', ['fail']],
  ['error_recovery', ['fail']],
];
const text = scanners.map(([name, ops]) => `(scanner ${name} (tokens ${name}) (operations ${ops.join(' ')}))`).join('\n') + '\n';
process.stdout.write(text);
