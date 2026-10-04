// Renders the native links form of every reverse-conversion source and checks
// that reading it back renders the same links and the same native listing.
import { grammarImporter, renderNativeGrammar } from '../js/src/index.js';
import { parseGrammarLinks, renderGrammarLinks } from '../js/src/grammar-links.js';
import { SOURCES } from './issue-195-reverse-conversion-sources.mjs';

for (const [format, source] of Object.entries(SOURCES)) {
  let grammar;
  try { grammar = grammarImporter(format)(source); } catch (error) { console.log(format, 'IMPORT', error.message); continue; }
  const links = renderGrammarLinks(grammar);
  const decoded = parseGrammarLinks(links);
  console.log(`=== ${format}`, renderGrammarLinks(decoded) === links, renderNativeGrammar(decoded) === renderNativeGrammar(grammar));
  console.log(links);
}
