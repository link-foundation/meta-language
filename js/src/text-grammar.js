/**
 * The built-in plain-text and natural-language grammars, mirrored by
 * `rust/src/structured_text_parser.rs`. Both build trees of
 * `builtin-grammar.js` nodes over string indices from one shared lexical
 * grammar.
 */
import { extra, errorNode, node, propagateErrors } from './builtin-grammar.js';

// The built-in lexical grammar shared with the Rust runtime: maximal runs of
// Unicode whitespace, of word characters, of control characters that are not
// whitespace (which the grammar rejects), and of any other characters.
const WORD = String.raw`\p{L}\p{N}\p{M}_'\-`;
const CONTROL = String.raw`\x00-\x08\x0E-\x1F\x7F-\x84\x86-\x9F`;
const LEXICAL = new RegExp(
  String.raw`\p{White_Space}+|[${WORD}]+|[${CONTROL}]+|[^\p{White_Space}${WORD}${CONTROL}]+`,
  'gu',
);
const WHITESPACE_TOKEN = /^\p{White_Space}+$/u;
const WORD_TOKEN = new RegExp(String.raw`^[${WORD}]+$`, 'u');
const CONTROL_TOKEN = new RegExp(String.raw`^[${CONTROL}]+$`, 'u');

// The built-in sentence grammar shared with the Rust runtime: a sentence ends
// after a run of terminal punctuation, any closing punctuation or quotes, and
// the whitespace that follows them.
const SENTENCE = /[^]*?[.!?।。؟۔！？]+[\p{Pe}\p{Pf}"']*\p{White_Space}*/uy;

/**
 * Parses plain text with the built-in line grammar: a `text_document` of lines
 * holding the shared lexical nodes. Unbalanced parentheses make the document
 * an `ERROR`-flagged root.
 */
export function parsePlainTextCst(text) {
  const lines = [];
  let start = 0;
  for (let end = text.indexOf('\n'); end !== -1; end = text.indexOf('\n', start)) {
    lines.push(node('line', start, end + 1, lexicalNodes(text, start, end + 1)));
    start = end + 1;
  }
  if (start < text.length || text.length === 0) {
    lines.push(node('line', start, text.length, lexicalNodes(text, start, text.length)));
  }
  const root = node('text_document', 0, text.length, lines);
  if (!balancedParentheses(text)) root.isError = true;
  propagateErrors(root);
  return root;
}

/**
 * Parses natural-language text with the built-in sentence grammar: a
 * `natural_language_document` of sentences holding the shared lexical nodes.
 */
export function parseNaturalLanguageCst(text) {
  const sentences = [];
  let start = 0;
  SENTENCE.lastIndex = 0;
  for (let match = SENTENCE.exec(text); match; match = SENTENCE.exec(text)) {
    sentences.push(node('sentence', start, SENTENCE.lastIndex, lexicalNodes(text, start, SENTENCE.lastIndex)));
    start = SENTENCE.lastIndex;
  }
  if (start < text.length) {
    sentences.push(node('sentence', start, text.length, lexicalNodes(text, start, text.length)));
  }
  const root = node('natural_language_document', 0, text.length, sentences);
  propagateErrors(root);
  return root;
}

/**
 * The shared lexical nodes of `text.slice(start, end)`: whitespace extras,
 * words, punctuation and `ERROR` nodes for runs of control characters.
 */
function lexicalNodes(text, start, end) {
  const nodes = [];
  for (const match of text.slice(start, end).matchAll(LEXICAL)) {
    const [value] = match;
    const from = start + match.index;
    const to = from + value.length;
    if (WHITESPACE_TOKEN.test(value)) nodes.push(extra('whitespace', false, from, to));
    else if (CONTROL_TOKEN.test(value)) nodes.push(errorNode(from, to, []));
    else if (WORD_TOKEN.test(value)) nodes.push(node('word', from, to));
    else nodes.push(node('punctuation', from, to));
  }
  return nodes;
}

function balancedParentheses(text) {
  let depth = 0;
  for (const character of text) {
    if (character === '(') depth += 1;
    if (character === ')' && depth-- === 0) return false;
  }
  return depth === 0;
}
