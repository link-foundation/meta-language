// Builds parity/grammars/scanners/rust.lino, the native port of
// tree-sitter-rust v0.24.2's src/scanner.c (sha256 9609a2f9...283, MIT): one
// (scanner NAME (tokens ...) (operations ...)) line per group of external
// tokens scanner.c scans together. Each operation list follows the C
// function of the same tokens: strings: process_string and the closing
// quote; raw_strings: scan_raw_string_start, _content and _end (the hash
// count is the depth of the `hashes` stack); floats: process_float_literal;
// block_comments: process_block_comment (the nesting depth is the `nesting`
// stack); line_documentation: process_line_documentation_content; error_sentinel
// never matches, as in C, where it marks error recovery.
//
//   node js/experiments/build-rust-scanner.mjs > parity/grammars/scanners/rust.lino
import { contentScanner } from '../scripts/scanner-families.mjs';

const DQ = '(literal %22)';
const WS = '(class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))';
const DIGIT = '(class plain (range 0 9))';
const NUM = '(class plain (range 0 9) (char _))';
const ALPHA = '(class plain (category Lu) (category Ll) (category Lt) (category Lm) (category Lo))';
const HASH = '(literal %23)';
const lit = (t) => `(literal ${t})`;
const clear = (s) => `(while (greater (depth ${s}) (integer 0)) (do (pop ${s})))`;
const skipWs = `(while (next ${WS}) (do (skip ${WS})))`;
const scanners = [
  ['raw_strings', ['raw_string_literal_start', 'raw_string_literal_content', 'raw_string_literal_end'], [
    `(if (valid raw_string_literal_start) (then ${skipWs} (if (some (next ${lit('b')}) (next ${lit('c')})) (then advance)) (consume ${lit('r')}) (while (next ${HASH}) (do advance (push hashes (integer 1)))) (consume ${DQ}) (emit raw_string_literal_start)))`,
    `(if (valid raw_string_literal_content) (then ${skipWs} (while (not atEnd) (do (if (next ${DQ}) (then mark advance (while (all (next ${HASH}) (less (depth seen) (depth hashes))) (do advance (push seen (integer 1)))) (if (equal (depth seen) (depth hashes)) (then ${clear('seen')} (emit raw_string_literal_content))) ${clear('seen')}) (else advance)))) fail))`,
    `(if (valid raw_string_literal_end) (then ${skipWs} (consume ${DQ}) (while (greater (depth hashes) (integer 0)) (do advance (pop hashes))) (emit raw_string_literal_end)))`,
    'fail',
  ]],
  ['floats', ['float_literal'], [
    skipWs,
    `(if (not (next ${DIGIT})) (then fail))`,
    'advance',
    `(while (next ${NUM}) (do advance))`,
    `(if (next ${lit('.')}) (then advance (if (some (next ${ALPHA}) (next ${lit('.')})) (then fail)) (while (next ${NUM}) (do advance)) (push float (integer 1))))`,
    'mark',
    `(if (some (next ${lit('e')}) (next ${lit('E')})) (then advance (if (some (next ${lit('%2B')}) (next ${lit('-')})) (then advance)) (if (not (next ${NUM})) (then ${clear('float')} (emit float_literal))) advance (while (next ${NUM}) (do advance)) mark (push float (integer 1))))`,
    '(if (equal (depth float) (integer 0)) (then fail))',
    clear('float'),
    `(if (not (some (next ${lit('u')}) (next ${lit('i')}) (next ${lit('f')}))) (then (emit float_literal)))`,
    'advance',
    `(if (not (next ${DIGIT})) (then (emit float_literal)))`,
    `(while (next ${DIGIT}) (do advance))`,
    'mark',
    '(emit float_literal)',
  ]],
  ['block_comments', ['outer_block_documentation_comment_marker', 'inner_block_documentation_comment_marker', 'block_comment_content'], [
    `(if (valid inner_block_documentation_comment_marker) (then (consume ${lit('%21')}) (if (not (next ${lit('%2A%2F')})) (then (push documentation (integer 1)))) (emit inner_block_documentation_comment_marker)))`,
    `(if (valid outer_block_documentation_comment_marker) (then (consume ${lit('%2A')}) (if (some (next ${lit('%2F')}) (next ${lit('%2A')})) (then fail)) (push documentation (integer 1)) (emit outer_block_documentation_comment_marker)))`,
    `(if (valid block_comment_content) (then (if (greater (depth documentation) (integer 0)) (then (pop documentation)) (else (if (some (next ${lit('%21')}) (all (next ${lit('%2A')}) (not (next ${lit('%2A%2A')})))) (then fail)))) (if (next ${lit('%2A%2F')}) (then fail)) (push nesting (integer 1)) (while (greater (depth nesting) (integer 0)) (do (if atEnd (then ${clear('nesting')}) (else (if (next ${lit('%2A%2F')}) (then (pop nesting) (if (greater (depth nesting) (integer 0)) (then (consume ${lit('%2A%2F')})))) (else (if (next ${lit('%2F%2A')}) (then (consume ${lit('%2F%2A')}) (push nesting (integer 1))) (else advance)))))))) (emit block_comment_content)))`,
    'fail',
  ]],
  ['line_documentation', ['line_documentation_content'], [
    `(while (not (some atEnd (next ${lit('%0A')}))) (do advance))`,
    `(if (next ${lit('%0A')}) (then advance))`,
    '(emit line_documentation_content)',
  ]],
  ['error_sentinel', ['error_sentinel'], ['fail']],
];
const strings = contentScanner({ name: 'strings', token: 'string_content', closeToken: 'string_close', closing: '"', stops: ['\\'], allowEnd: false });
const text = strings + scanners.map(([name, tokens, ops]) => `(scanner ${name} (tokens ${tokens.join(' ')}) (operations ${ops.join(' ')}))`).join('\n') + '\n';
process.stdout.write(text);
