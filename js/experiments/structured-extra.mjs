// Probes how an extra of a rule that builds a node renders in the feature
// union's trivia grammar (see docs/grammar/feature-union.md).
import { readFileSync } from 'node:fs';
import { compileGrammar, parseNativeGrammar, renderNativeGrammar, renderSyntaxTree } from '../src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-feature-union.json', import.meta.url), 'utf8'));
const trivia = fixture.features.find((feature) => feature.id === 'trivia');
const listing = trivia.listing
  .replace('extra ref(comment)\n', 'extra ref(comment)\nextra ref(doc)\n')
  .concat('rule doc = normal seq(literal("@"), ref(name))\n');
const grammar = parseNativeGrammar(listing);
console.log(renderNativeGrammar(grammar) === listing ? 'canonical' : renderNativeGrammar(grammar));
const parser = compileGrammar(grammar);
for (const input of process.argv.slice(2)) {
  const result = parser.parseTree(input);
  console.log(JSON.stringify(input), result.ok ? renderSyntaxTree(parser.parse(input)) : JSON.stringify(result));
}
