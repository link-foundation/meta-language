// Prints the ambiguities the JavaScript executor reports for the native C
// grammar on each source given, or on every fixture match with --all.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync('../parity/grammars/native/c.lino', 'utf8')));
const fixture = JSON.parse(readFileSync('../parity/fixtures/native-grammars/c.json', 'utf8'));
const sources = process.argv.includes('--all') ? fixture.matches.map(({ source }) => source) : process.argv.slice(2);
for (const source of sources) {
  const outcome = parser.parseTree(source);
  if (outcome.ambiguities.length > 0 || !process.argv.includes('--all')) {
    console.log(JSON.stringify(source), outcome.ok, JSON.stringify(outcome.ambiguities).slice(0, 600));
  }
}
