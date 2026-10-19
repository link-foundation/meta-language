// Convert the original bounded experimental literal corpus into Links Notation.
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import { tokenize } from '../src/translation/lexer.js';
import { readProjectData, writeProjectData } from './project-data.mjs';

const input = new URL('../tests/translation-rust-string-continuation-draft.test.js', import.meta.url);
const output = new URL('../../parity/fixtures/translation-rust-string-continuations.lino', import.meta.url);
const moduleSource = readFileSync(input, 'utf8');
// Later tests deliberately contain a lone-surrogate refusal fixture. Restrict
// lexical decoding to the declaration section, rather than admitting that
// unrelated unsupported value into the converter's input profile.
const boundary = moduleSource.indexOf('\nfunction reloadRules()');
if (boundary < 0) throw new Error('the bounded fixture declaration section is missing');
const source = moduleSource.slice(0, boundary);
const { tokens } = tokenize(source, 'JavaScript');
const begin = tokens.findIndex((token, index) => token.value === 'const' && tokens[index + 1]?.value === 'fixtures'
  && tokens[index + 2]?.value === '=' && tokens[index + 3]?.value === '[') + 3;
if (begin < 3) throw new Error('the original continuation fixture declaration is missing');
let depth = 0;
let end = begin;
for (; end < tokens.length; end += 1) {
  const token = tokens[end];
  if (token.kind !== 'string' && !['[', ']', ','].includes(token.value)) throw new Error('the fixture corpus must contain only nested arrays of string literals');
  if (token.value === '[') depth += 1;
  if (token.value === ']') depth -= 1;
  if (depth === 0) break;
}
if (end === tokens.length) throw new Error('the fixture corpus is unclosed');
const values = vm.runInNewContext(source.slice(tokens[begin].start, tokens[end].end), {}, { timeout: 1000 });
const expected = { model: 'Rust Reference cooked string continuation', fixtures: values.map(([literal, value], index) => ({
  id: `rust-string-continuation-${index + 1}`, literal, value,
})) };
if (process.argv.includes('--check')) {
  if (JSON.stringify(readProjectData(output)) !== JSON.stringify(expected)) throw new Error('the linked continuation corpus is stale');
} else writeProjectData(output, expected);
