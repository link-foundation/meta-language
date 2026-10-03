// Prints the automatically recovered native trees of invalid inputs.
// Usage: node experiments/issue-195-native-recovery.mjs <grammar id> <source>...
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../js/src/index.js';

const [id, ...sources] = process.argv.slice(2);
const links = readFileSync(new URL(`../parity/grammars/native/${id}.lino`, import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(links), { errorRecovery: true });
for (const source of sources) {
  const outcome = parser.parseTree(source);
  console.log(JSON.stringify(source), outcome.rejection?.reason ?? 'ok');
  console.log('  ', outcome.tree ? renderSyntaxTree(outcome.tree) : null);
}
