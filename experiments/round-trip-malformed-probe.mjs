// Checks that each shared importer rejects the malformed interchange sources.
import * as ml from '../js/src/index.js';
const imp = { abnf: ml.importAbnf, bnf: ml.importBnf, ebnf: ml.importEbnf, pest: ml.importPest, 'tree-sitter-json': ml.importTreeSitterJson };
const malformed = JSON.parse(process.argv[2]);
for (const m of malformed) {
  try { const g = imp[m.format](m.source); console.log('ACCEPTED', m.format, JSON.stringify(m.source), g.ruleNames()); }
  catch (e) { console.log('rejected', m.format, e.name, JSON.stringify(m.source), e.message); }
}
