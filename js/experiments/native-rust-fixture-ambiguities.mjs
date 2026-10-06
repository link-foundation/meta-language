// Prints every ambiguity the native Rust grammar reports on the sources of
// its fixture's matches, with the text it covers.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/native-grammars/rust.json', import.meta.url), 'utf8'));
let count = 0;
for (const [index, { source }] of fixture.matches.entries()) {
  const { ambiguities } = parser.parseTree(source);
  const bytes = Buffer.from(source);
  for (const { rule, start, end } of ambiguities) {
    count += 1;
    console.log(index, rule, JSON.stringify(bytes.subarray(start, end).toString().slice(0, 120)));
  }
}
console.log('ambiguities', count);
