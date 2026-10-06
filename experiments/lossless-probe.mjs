// Probes the lossless mode on the commented reverse-conversion sources: the
// unchanged grammar must reconstruct the source exactly (also through the
// layout links), and a start-rule mutation and a rule rename must keep every
// untouched definition and re-import to the edited grammar.
import { mutateGrammarStartRule } from '../js/src/grammar-round-trip.js';
import { renameGrammarRule } from '../js/src/grammar-merge.js';
import { grammarImporter, renderNativeGrammar } from '../js/src/index.js';
import { parseGrammarLinks, renderGrammarLinks } from '../js/src/grammar-links.js';
import {
  emitGrammarLossless, importGrammarLossless, parseGrammarLayoutLinks, renderGrammarLayoutLinks,
} from '../js/src/grammar-lossless.js';
import { SOURCES } from './issue-195-reverse-conversion-sources.mjs';

for (const [format, source] of Object.entries(SOURCES)) {
  const { grammar, layout } = importGrammarLossless(source, format);
  const decodedLayout = parseGrammarLayoutLinks(renderGrammarLayoutLinks(layout));
  const decoded = parseGrammarLinks(renderGrammarLinks(grammar));
  const exact = emitGrammarLossless(decoded, decodedLayout).source === source;
  console.log(`=== ${format} exact=${exact} members=${layout.members.map((m) => m.name)} implicit=${layout.implicit.map((m) => m.name)}`);
  const word = grammar.ruleNames().includes('word') ? 'word' : grammar.ruleNames()[1];
  for (const [label, edited] of [
    ['mutated', mutateGrammarStartRule(decoded, 'round-trip-mutation')],
    ['renamed', renameGrammarRule(decoded, word, 'term').grammar],
  ]) {
    const out = emitGrammarLossless(edited, decodedLayout);
    const back = grammarImporter(format)(out.source);
    const same = renderGrammarLinks(back) === renderGrammarLinks(edited);
    console.log(`--- ${label} reimports-equal=${same} lossy=${JSON.stringify(out.report.lossy)}`);
    if (!same) console.log(renderGrammarLinks(edited), '\n', renderGrammarLinks(back));
    console.log(out.source);
  }
}
