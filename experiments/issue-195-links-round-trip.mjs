// Scratch check: every feature-union fixture grammar survives the links form.
import { readFileSync } from 'node:fs';
import { parseNativeGrammar, renderNativeGrammar } from '../js/src/grammar-interchange.js';
import { parseGrammarLinks, renderGrammarLinks } from '../js/src/grammar-links.js';
import { createGrammarParser, renderSyntaxTree } from '../js/src/grammar-runtime.js';

const fixture = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-feature-union.json', import.meta.url)));
const languages = Object.fromEntries(Object.entries(fixture.languages).map(([n, l]) => [n, parseNativeGrammar(l)]));
const resolveGrammar = (n) => languages[n];
let failures = 0;
for (const feature of fixture.features) {
  const grammar = parseNativeGrammar(feature.listing);
  const links = renderGrammarLinks(grammar);
  let back;
  try { back = parseGrammarLinks(links); } catch (e) { console.log(feature.id, 'PARSE FAIL', e.message); console.log(links); failures++; continue; }
  const a = renderNativeGrammar(grammar), b = renderNativeGrammar(back);
  if (a !== b) { console.log(feature.id, 'DIFF'); console.log(a); console.log(b); failures++; }
  if (renderGrammarLinks(back) !== links) { console.log(feature.id, 'LINKS DIFF'); failures++; }
  const p = createGrammarParser(back, { resolveGrammar, ...(feature.options ?? {}) });
  for (const c of feature.positive) {
    const input = c.input ?? Buffer.from(c.inputHex, 'hex');
    const r = p.parseTree(input, c.options ?? {});
    if (renderSyntaxTree(r.tree) !== c.tree) { console.log(feature.id, 'TREE', c.input); failures++; }
  }
}
console.log('failures', failures);
