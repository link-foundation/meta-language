// Generate the pinned Odin scanner as executable Links data. Both runtime
// interpreters execute the same operations, including the oracle's lexical
// edge cases; this generator is never called by the production parser.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';
import { delimitedScanner } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

const literal = (text) => `(literal ${[...Buffer.from(text)].map((byte) => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('')})`;
const pattern = (text) => renderTreeSitterPattern(parseTreeSitterPattern(text));
const branch = (condition, body, otherwise = null) => `(if ${condition} (then ${body})${otherwise === null ? '' : ` (else ${otherwise})`})`;
const next = (text) => `(next ${literal(text)})`;
const variable = (name) => `(variable odin_${name})`;
const set = (name, value) => `(set odin_${name} (integer ${value}))`;
const flag = (name) => `(equal ${variable(name)} (integer 1))`;
const space = pattern('[ \\t\\n\\r\\v\\f]');
const horizontal = pattern('[ \\t\\r\\v\\f]');
const digit = pattern('[0-9]');
const skipHorizontal = `(while (next ${horizontal}) (do (skip ${horizontal})))`;
const skipSpace = `(while (next ${space}) (do (skip ${space})))`;
const validNumber = `(all (some ${flag('decimal')} ${flag('exponent')}) (some ${flag('before_decimal')} ${flag('after_decimal')}))`;
const emitFloat = 'mark (emit floating_point_number)';
const abandonFloat = set('active', 0);

export function odinScanner() {
  // A second dot terminates a decimal, but a pair of initial dots abandons
  // it so the grammar can read the range operator. The source scanner accepts
  // incomplete exponents and repeated signs before the first exponent digit.
  const dot = branch(`(all (some ${flag('decimal')} ${flag('exponent')}) (some ${flag('after_decimal')} ${flag('before_decimal')}))`, emitFloat,
    `mark ${set('decimal', 1)} advance ${branch(next('.'), `advance ${abandonFloat}`, `mark ${branch(`(all (not (next ${digit})) (some ${flag('after_decimal')} ${flag('before_decimal')}))`, '(emit floating_point_number)')}`)}`);
  const imaginary = branch(`(not ${flag('after_decimal')})`, abandonFloat, branch(validNumber, `advance ${emitFloat}`, abandonFloat));
  const exponent = branch(`(all ${flag('exponent')} (some ${flag('after_decimal')} ${flag('before_decimal')}))`, emitFloat,
    branch(`(some ${flag('before_decimal')} ${flag('after_decimal')})`, `${set('exponent', 1)} advance`, abandonFloat));
  const sign = branch(`(some (equal ${variable('index')} (integer 0)) (all ${flag('exponent')} (not ${flag('after_exponent')})))`, 'advance', abandonFloat);
  const other = branch(`(next ${digit})`, `advance ${branch(flag('decimal'), set('after_decimal', 1), set('before_decimal', 1))} ${branch(flag('exponent'), set('after_exponent', 1))}`,
    branch(validNumber, emitFloat, branch(flag('before_decimal'), 'fail', abandonFloat)));
  const step = branch(next('.'), dot, branch(`(some ${next('i')} ${next('j')} ${next('k')})`, imaginary,
    branch(`(some ${next('e')} ${next('E')})`, exponent, branch(`(some ${next('+')} ${next('-')})`, sign, other))));
  const number = branch('(valid floating_point_number)', `${skipHorizontal} ${branch('(not (expected (ref newline)))', skipSpace)} ${['decimal', 'exponent', 'before_decimal', 'after_decimal', 'after_exponent', 'index'].map((name) => set(name, 0)).join(' ')} ${set('active', 1)} (while ${flag('active')} (do ${step} (set odin_index (add ${variable('index')} (integer 1)))))`);
  const commaLine = branch(next('\n'), `(while (next ${space}) (do advance)) ${branch(`(not ${next('}')})`, '(emit newline_comma)')}`);
  const comma = branch('(valid newline_comma)', `${skipHorizontal} ${branch(next(','), `advance mark (while (next ${horizontal}) (do advance)) ${commaLine}`)}`);
  // The first LF is the token; following spaces and up to five word bytes
  // are lookahead. "else" needs trailing whitespace, and a brace suppresses
  // that LF only when there is no blank line and its literal is expected.
  const suppressKeyword = `(next ${pattern('(?:where|else)[ \\t\\n\\r\\v\\f]')})`;
  const suppressBrace = `(all (equal ${variable('blank_lines')} (integer 0)) (expected ${literal('{')}) (next ${pattern('\\{[ \\t\\n\\r\\v\\f]')}))`;
  const newline = branch('(valid newline)', `${skipHorizontal} ${branch(next('\n'), `advance mark ${set('blank_lines', 0)} (while (next ${space}) (do ${branch(next('\n'), '(set odin_blank_lines (add (variable odin_blank_lines) (integer 1)))')} advance)) ${branch(suppressKeyword, 'fail')} ${branch(suppressBrace, 'fail')} (emit newline)`)}`);
  const continuation = branch(`(all (valid backslash) ${next('\\')})`, `advance (consume ${literal('\n')}) (while (next ${space}) (do advance)) (emit backslash)`);
  const comment = delimitedScanner({ name: 'odin_comment', token: 'block_comment', opening: '/*', closing: '*/', nested: true, openingLookahead: '"', rejectOpeningLookahead: true, rejected: ['\0'] });
  const commentOperations = comment.slice(comment.indexOf('(operations ') + 12, -3);
  const continuationContext = `(next ${pattern('[ \\t\\n\\r\\v\\f]*(?:where|else)[ \\t\\n\\r\\v\\f]')})`;
  const braceContext = `(all (expected ${literal('{')}) (next ${pattern('[ \\t\\r\\v\\f]*\\n[ \\t\\r\\v\\f]*\\{[ \\t\\n\\r\\v\\f]')}))`;
  return `(scanner odin_lexical_tokens (tokens newline backslash newline_comma floating_point_number block_comment) (operations ${number} ${comma} ${newline} ${continuation} ${skipSpace} ${commentOperations}))\n`
    + `(scanner odin_contextual_trivia (tokens contextual_whitespace) (operations (if (not (valid contextual_whitespace)) (then fail)) (if (all (expected (ref newline)) (not ${continuationContext}) (not ${braceContext})) (then fail)) (consume (repeat1 ${space})) (emit contextual_whitespace)))\n`;
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) process.stdout.write(odinScanner());
