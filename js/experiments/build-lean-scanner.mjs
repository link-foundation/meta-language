// Builds parity/grammars/scanners/lean.lino, the native port of
// tree-sitter-lean4 0.3.0's src/scanner.c (sha256 in
// parity/grammars/sources.json, MIT): one (scanner layout (tokens ...)
// (operations ...)) line whose operations follow
// tree_sitter_lean_external_scanner_scan for all five external tokens. Each
// `valid_symbols[X]` is `(some (valid X) (expected (ref X)))`, so the
// operations choose the token the C function returns, and the run fails
// where that is not the requested one. The indent stack is the stack
// `indents`, pushed while it is shallower than MAX_DEPTH; `queued_indent` is
// the stack `queued`, empty for NO_QUEUED. A `skip` before any mark_end is a
// `skip`; after mark_end it is an `advance`, since the token ends at the
// mark either way. The parenthesis, bracket and brace depths of the syntax
// quotation body are temporary stacks, emptied before the scanner emits.
//
//   node js/experiments/build-lean-scanner.mjs > parity/grammars/scanners/lean.lino
const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('');
const lit = (text) => `(literal ${enc(text)})`;
const next = (text) => `(next ${lit(text)})`;
const any = (...texts) => `(some ${texts.map(next).join(' ')})`;
const SPACE = '(class plain (char %20) (char %09))';
const NEWLINE = '(class plain (char %0A) (char %0D))';
const BLANK = '(class plain (char %20) (char %09) (char %0A) (char %0D))';
const zero = '(integer 0)';
const one = '(integer 1)';
const valid = (token) => `(some (valid ${token}) (expected (ref ${token})))`;
const START = valid('layout_start');
const SEMICOLON = valid('layout_semicolon');
const END = valid('layout_end');
const isSet = (stack) => `(greater (depth ${stack}) ${zero})`;
const clear = (stack) => `(while ${isSet(stack)} (do (pop ${stack})))`;
const pop = (stack) => `(if ${isSet(stack)} (then (pop ${stack})))`;
// `lexer->advance(lexer, true)` before and after mark_end.
const step = (skip, item) => (skip ? `(skip ${item})` : 'advance');
const spaces = (skip) => `(while (next ${SPACE}) (do ${step(skip, SPACE)}))`;
const blank = (skip) => `(while (next ${BLANK}) (do ${step(skip, BLANK)}))`;
// measure_indent, pushing the column onto `stack`.
const measure = (skip, stack) => [
  `(while (next ${NEWLINE}) (do ${step(skip, NEWLINE)}))`,
  spaces(skip),
  `(if atEnd (then (push ${stack} ${zero})) (else (push ${stack} column)))`,
].join(' ');
const pushIndent = (value) => `(if (less (depth indents) (integer 64)) (then (push indents ${value})))`;
const DEEP = isSet('indents');
const PIPE = next('|');

const pushLayoutIndent = [
  spaces(true),
  `(if (next ${NEWLINE}) (then ${measure(true, 'measured')} ${pushIndent('(top measured)')} (pop measured)) (else ${pushIndent('column')}))`,
  pop('queued'),
].join(' ');

const BRACKETS = [['(', ')', 'parens'], ['[', ']', 'brackets'], ['{', '}', 'braces']];
const syntaxQuotationBody = [
  `(while (not (some atEnd (all ${next(')')} ${BRACKETS.map(([, , stack]) => `(not ${isSet(stack)})`).join(' ')}))) (do`,
  BRACKETS.map(([open, close, stack]) => `(if ${next(open)} (then (push ${stack} ${one}))) (if ${next(close)} (then ${pop(stack)}))`).join(' '),
  `advance (if (not ${isSet('consumed')}) (then (push consumed ${one})))))`,
  BRACKETS.map(([, , stack]) => clear(stack)).join(' '),
  `(if ${isSet('consumed')} (then (pop consumed) mark (emit syntax_quotation_body)))`,
  'fail',
].join(' ');

const queuedIndent = [
  `(if (all ${isSet('queued')} ${DEEP}) (then`,
  `(if (all (less (top queued) (top indents)) ${END}) (then`,
  `(if (equal (depth indents) ${one}) (then mark ${blank(false)} (if ${PIPE} (then fail))))`,
  '(pop indents) (emit layout_end)))',
  `(if (all (equal (top queued) (top indents)) ${SEMICOLON}) (then`,
  `${blank(true)} (if ${PIPE} (then fail)) (pop queued) (emit layout_semicolon)))`,
  '(pop queued)))',
].join(' ');

const newline = [
  `(if (all (next ${NEWLINE}) ${DEEP}) (then mark ${measure(false, 'queued')}`,
  `(if (all (less (top queued) (top indents)) ${END}) (then`,
  `(if (all (equal (depth indents) ${one}) ${PIPE}) (then fail))`,
  '(pop indents) (emit layout_end)))',
  `(if (all (equal (top queued) (top indents)) ${SEMICOLON}) (then`,
  `(if ${PIPE} (then fail)) (pop queued) (emit layout_semicolon)))`,
  'fail))',
].join(' ');

const operations = [
  `(if (all ${START} ${SEMICOLON} ${END}) (then fail))`,
  `(if ${valid('syntax_quotation_body')} (then ${syntaxQuotationBody}))`,
  `(if ${START} (then ${pushLayoutIndent} (emit layout_start)))`,
  `(if ${valid('match_body_start')} (then ${pushLayoutIndent} (emit match_body_start)))`,
  queuedIndent,
  spaces(true),
  newline,
  `(if (all atEnd ${DEEP} ${END}) (then (pop indents) (emit layout_end)))`,
  `(if (all ${END} ${DEEP} ${any(')', ']', '}')}) (then (pop indents) (emit layout_end)))`,
  'fail',
];
const tokens = ['layout_start', 'layout_semicolon', 'layout_end', 'match_body_start', 'syntax_quotation_body'];
process.stdout.write(`(scanner layout (tokens ${tokens.join(' ')}) (operations ${operations.join(' ')}))\n`);
