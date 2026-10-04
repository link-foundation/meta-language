// Prints what the precedence feature's grammar of
// parity/fixtures/grammar-feature-union.json gives with named precedences
// `&` over `|` added: the trees of the inputs, the rejections, and the trees
// once the precedence order is swapped.
//   node experiments/named-precedence-feature.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseNativeGrammar, renderSyntaxTree } from '../src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-feature-union.json', import.meta.url), 'utf8'));
const feature = fixture.features.find(({ id }) => id === 'precedence');
const listing = process.argv[2] ?? feature.listing;
const run = (text, inputs) => {
  const parser = compileGrammar(parseNativeGrammar(text), feature.options);
  for (const input of inputs) {
    const outcome = parser.parseTree(input);
    console.log(JSON.stringify(input), outcome.ok ? renderSyntaxTree(outcome.tree) : JSON.stringify(outcome.rejection));
  }
};
console.log(listing);
run(listing, ['1&2|3', '1|2&3', '1&2&3', '1|2|3', '1+2&3', '1=2=3', '1+*2']);
const swapped = listing.replace('precedences name(and) name(or)', 'precedences name(or) name(and)');
console.log('swapped');
run(swapped, ['1&2|3', '1|2&3']);
