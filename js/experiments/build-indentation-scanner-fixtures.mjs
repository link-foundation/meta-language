// Bounded layout examples. Validate independently specified block structure
// before storing the complete lossless trees shared by both executors.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';

export const descriptor = { family: 'indentation', name: 'layout', newlineToken: 'newline', indentToken: 'indent', dedentToken: 'dedent', bracketClosers: [] };
export const generated = scannerFamilies([descriptor]);
export const listing = `(grammar (start source) (matching longest))
(extra (repeat1 (class plain (char %20) (char %09) (char %0A) (char %0D) (char %0C))))
(extra (ref comment))
${generated}(rule comment token (seq (literal %23) (repeat0 (class negated (char %0A)))))
(rule source normal (repeat1 (ref statement)))
(rule statement normal (choice unordered (seq (ref name) (literal %3A) (immediateToken (ref indent)) (ref block) (immediateToken (ref dedent))) (seq (ref name) (immediateToken (ref newline)))))
(rule block normal (repeat1 (ref statement)))
(rule name token (repeat1 (choice unordered (class plain (range a z)) (literal %C3%A9) (literal %F0%9F%98%80))))
`;
export function blockStructure(tree) {
  return tree.children.filter((child) => child.kind === 'statement').map((statement) => {
    const name = statement.children.find((child) => child.kind === 'name').text;
    const block = statement.children.find((child) => child.kind === 'block');
    return block ? [name, blockStructure(block)] : name;
  });
}
const positive = [
  ['a\n', ['a']],
  ['a:\n  b\nc\n', [['a', ['b']], 'c']],
  ['a:\n  b\n  c\nd\n', [['a', ['b', 'c']], 'd']],
  ['a:\n  b:\n    c\nd\n', [['a', [['b', ['c']]]], 'd']],
  ['a:\n  b:\n    c', [['a', [['b', ['c']]]]]],
  ['a:\n  b', [['a', ['b']]]],
  ['a:\n\tb\nc\n', [['a', ['b']], 'c']],
  ['a:\n \tb\nc\n', [['a', ['b']], 'c']],
  ['a:\n \tb\n        c\nd\n', [['a', ['b']], 'c', 'd']],
  ['a:\r\n  b\r\nc\r\n', [['a', ['b']], 'c']],
  ['a:\n \f  b\n  c\nd\n', [['a', ['b', 'c']], 'd']],
  ['a:\n  # same\n  b\nc\n', [['a', ['b']], 'c']],
  ['a:\n  b\n  # block\nc\n', [['a', ['b']], 'c']],
  ['a:\n  b\n# outer\nc\n', [['a', ['b']], 'c']],
  ['a:\n  b\n\n  c\nd\n', [['a', ['b', 'c']], 'd']],
  ['é:\n  😀\na\n', [['é', ['😀']], 'a']],
  ['a:\n  b\nc:\n  d\ne\n', [['a', ['b']], ['c', ['d']], 'e']],
];
const reject = ['a:', 'a:\nb\n', 'a:\\\n  b\n', 'a:\\x\n  b\n', 'a:\n  b\\x'];
const parser = compileGrammar(parseGrammarLinks(listing));
const accept = positive.map(([input, structure]) => {
  const outcome = parser.parseTree(input);
  assert.equal(outcome.ok, true, JSON.stringify({ input, rejection: outcome.rejection }));
  assert.deepEqual(blockStructure(outcome.tree), structure, input);
  return { input, structure, tree: renderSyntaxTree(outcome.tree) };
});
for (const input of reject) assert.equal(parser.parseTree(input).ok, false, input);
const data = JSON.stringify({ scanners: [descriptor], generated, listing, accept, reject }, null, 2) + '\n';
const path = new URL('../../parity/fixtures/scanner-indentation.json', import.meta.url);
if (process.argv.includes('--check')) assert.equal(readFileSync(path, 'utf8'), data);
else writeFileSync(path, data);
console.log(`${accept.length} indentation cases and ${reject.length} rejections match independent block structures`);
