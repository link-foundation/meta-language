// Prints the ambiguities the native Rust grammar reports on each source of
// SOURCES (a JSON array), with the rule, the span and the text it covers.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
for (const source of JSON.parse(process.env.SOURCES)) {
  const outcome = parser.parseTree(source);
  const bytes = Buffer.from(source);
  console.log(JSON.stringify(source), outcome.ok, JSON.stringify(outcome.ambiguities.map(({ rule, start, end }) => [rule, bytes.subarray(start, end).toString()])));
}
