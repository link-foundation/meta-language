// Generate shared data for exact textual delimiter matching, without host callbacks.
import { writeFileSync } from 'node:fs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
const descriptors = [{ family: 'remembered-delimiter', name: 'tagged_text', startToken: 'opening_tag', contentToken: 'body', endToken: 'closing_tag', tagPattern: '\\$[^$\\s]*\\$' }];
const generated = scannerFamilies(descriptors);
const listing = `(grammar (start source))\n${generated}(rule source normal (repeat1 (seq (ref opening_tag) (ref body) (ref closing_tag))))\n`;
const accept = ['$$$$', '$$hello$$', '$name$a$other$b$name$', '$a$é😀\n$a$', '$left$x$left$$right$y$right$', '$tag$one$ta$two$tag$', '$A$case$a$still content$A$', '$long_label$body$long_label$'];
const reject = ['$$', '$a$body$b$', '$a$body$a', '$a$body$A$', '$ a$x$ a$', '$a$x$a$tail'];
const parser = compileGrammar(parseGrammarLinks(listing));
const trees = {};
for (const source of accept) { const result = parser.parseTree(source); if (!result.ok) throw Error(JSON.stringify({ source, rejection: result.rejection })); trees[source] = renderSyntaxTree(result.tree); }
for (const source of reject) if (parser.parseTree(source).ok) throw Error(`accepted ${JSON.stringify(source)}`);
const fixture = [{ name: 'remembered textual delimiters', scanners: descriptors, generated, listing, accept, reject, trees, assertion: 'rememberedDelimiterText' }];
writeFileSync(new URL('../../parity/fixtures/scanner-remembered-delimiters.json', import.meta.url), JSON.stringify(fixture, null, 2) + '\n');
console.log('generated exact delimiter fixtures');
