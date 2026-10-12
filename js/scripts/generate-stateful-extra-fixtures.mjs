#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';

const cases = [
  {
    name: 'extras select later trivia using the committed lexer mode',
    listing: `(grammar (start source) (matching longest))
(mode quoted)
(extra (ref enter))
(extra (ref percent))
(rule enter token (literal %23) (action (setMode quoted)))
(rule percent token (literal %25) (modes quoted))
(rule source normal (seq (literal x) (literal y)))\n`,
    accept: ['x#%y'], reject: ['x%y'],
  },
  {
    name: 'a failed extra discards speculative queue changes',
    listing: `(grammar (start source) (matching longest))
(extra (ref body))
(scanner endings (tokens closing) (operations (if (equal (depth pending) (integer 0)) (then fail)) (consume (literal %5D)) (pop pending) (emit closing)))
(rule opening token (literal %5B) (action (push pending (integer 1))))
(rule body normal (seq (literal %23) (ref closing) (literal %21)))
(rule source normal (seq (ref opening) (choice ordered (predicate (literal %23%5Dx) (equal (depth pending) (integer 1))) (predicate (literal x) (equal (depth pending) (integer 0))))))\n`,
    accept: ['[#]x', '[#]!x'], reject: ['[#]!#]x', '[#x'],
  },
  {
    name: 'node extras commit scanner queue changes',
    listing: `(grammar (start source) (matching longest))
(extra (ref body))
(scanner endings (tokens closing) (operations (if (equal (depth pending) (integer 0)) (then fail)) (consume (literal %5D)) (pop pending) (emit closing)))
(rule opening token (literal %5B) (action (push pending (integer 1))))
(rule body normal (seq (literal %23) (ref closing)))
(rule final token (predicate (literal x) (equal (depth pending) (integer 0))))
(rule source normal (repeat1 (seq (ref opening) (ref final))))\n`,
    accept: ['[#]x', '[#]x[#]x'], reject: ['[#x', '[#]#]x', '[]x'],
  },
  {
    name: 'token extras preserve their actions for following predicates',
    listing: `(grammar (start source) (matching longest))
(extra (ref drop))
(rule opening token (literal %5B) (action (push pending (integer 1))))
(rule drop token (predicate (literal %23) (greater (depth pending) (integer 0))) (action (pop pending)))
(rule final token (predicate (literal x) (equal (depth pending) (integer 0))))
(rule source normal (repeat1 (seq (ref opening) (ref final))))\n`,
    accept: ['[#x', '[#x[#x'], reject: ['[x', '[##x'],
  },
  {
    name: 'external extras retain skipped trivia and state',
    listing: `(grammar (start source))
(extra (ref drop))
(scanner endings (tokens drop) (operations (if (equal (depth pending) (integer 0)) (then fail)) (while (next (literal %20)) (do (skip (literal %20)))) (consume (literal %C3%A9%F0%9F%98%80)) (pop pending) (emit drop)))
(rule opening token (literal %5B) (action (push pending (integer 1))))
(rule source normal (seq (ref opening) (predicate (literal x) (equal (depth pending) (integer 0)))))\n`,
    accept: ['[  é😀x'], reject: ['[  éx', '[  é😀é😀x'],
  },
  {
    name: 'an explicitly requested extra token applies its action once',
    listing: `(grammar (start source) (matching longest))
(extra (ref tick))
(rule tick token (literal %23) (action (push count (integer 1))))
(rule source normal (seq (literal x) (ref tick) (predicate (literal y) (equal (depth count) (integer 1)))))\n`,
    accept: ['x#y'], reject: ['x##y'],
  },
  {
    name: 'immediate tokens receive the state of preceding named extras',
    listing: `(grammar (start source) (matching longest))
(extra (ref tick))
(rule tick token (literal %23) (action (push count (integer 1))))
(rule source normal (seq (literal x) (immediateToken (predicate (literal y) (equal (depth count) (integer 1))))))\n`,
    accept: ['x#y'], reject: ['x##y', 'xy'],
  },
];
for (const entry of cases) {
  const parser = compileGrammar(parseGrammarLinks(entry.listing));
  entry.trees = {};
  for (const input of entry.accept) {
    const outcome = parser.parseTree(input);
    if (!outcome.ok) throw new Error(`${entry.name}: rejected ${JSON.stringify(input)}`);
    entry.trees[input] = renderSyntaxTree(outcome.tree);
  }
  for (const input of entry.reject) if (parser.parseTree(input).ok) throw new Error(`${entry.name}: accepted ${JSON.stringify(input)}`);
}
const target = new URL('../../parity/fixtures/scanner-stateful-extras.json', import.meta.url);
const content = `${JSON.stringify(cases, null, 2)}\n`;
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== content) throw new Error('stateful extra fixtures have drifted');
} else writeFileSync(target, content);
