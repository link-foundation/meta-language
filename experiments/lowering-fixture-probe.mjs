// Prints the lowering check of each fixture grammar in every notation.
import { GRAMMAR_LOWERING_FORMATS, checkGrammarLowering, parseGrammarLinks, renderGrammarLinks } from '../js/src/index.js';
import { LOWERING_GRAMMARS } from './issue-195-lowering-fixture-grammars.mjs';

for (const { id, links, accepts, rejects } of LOWERING_GRAMMARS) {
  const grammar = parseGrammarLinks(links);
  if (renderGrammarLinks(grammar) !== links) console.log(id, 'links are not canonical:\n', renderGrammarLinks(grammar));
  for (const format of GRAMMAR_LOWERING_FORMATS) {
    const report = checkGrammarLowering(grammar, format, { accepts, rejects });
    console.log(id, format, report.status, report.lowering.steps.length, JSON.stringify(report.failures));
    if (process.env.VERBOSE) console.log(report.lowering.executable, report.lowering.metadata);
  }
}
