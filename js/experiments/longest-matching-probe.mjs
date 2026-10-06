// Parses `l:-1` with the ambiguity feature grammar of the feature union
// fixture, with and without `matching longest`: generalized matching keeps
// both the minus-then-digit and the one-token parse (an ambiguity), longest
// matching keeps the parse whose first differing token is longer.
//   node experiments/longest-matching-probe.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseNativeGrammar } from '../src/grammar-interchange.js';

const fixture = JSON.parse(readFileSync(new URL('../../parity/fixtures/grammar-feature-union.json', import.meta.url), 'utf8'));
const { listing, options } = fixture.features.find((feature) => feature.id === 'ambiguity');
for (const text of [listing, listing.replace('matching longest\n', '')]) {
  const out = compileGrammar(parseNativeGrammar(text)).parseTree('l:-1', options);
  console.log(text.includes('matching longest') ? 'longest    ' : 'generalized', JSON.stringify(out.rejection ?? null), out.ambiguities?.length ?? 0);
}
