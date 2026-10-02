// Removes each top-level reconstruction step from the recorded lowering
// metadata and prints the failures the check reports, to confirm that no step
// can be dropped silently.
import { readFile } from 'node:fs/promises';
import { checkGrammarLowering, grammarEmitter, parseGrammarLinks, parseLoweringMetadata } from '../js/src/index.js';

const corpus = JSON.parse(await readFile(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
for (const entry of corpus.lowering) {
  const grammar = parseGrammarLinks(entry.links);
  for (const target of entry.targets) {
    const { order, steps } = parseLoweringMetadata(target.metadata);
    const lines = target.metadata.split('\n');
    steps.forEach((step, index) => {
      if (step.kind === 'helper' && !order.includes(step.owner)) return;
      const line = lines[4 + index];
      const report = checkGrammarLowering(grammar, target.format, {
        accepts: entry.accepts, rejects: entry.rejects,
        editMetadata: (metadata) => metadata.split('\n').filter((l) => l !== line).join('\n'),
      });
      console.log(entry.id, target.format, step.kind, step.helper ?? step.rule, report.status, JSON.stringify(report.failures));
    });
    if (entry.id.startsWith("lookahead")) continue;
    const raw = checkGrammarLowering(grammar, target.format, { emitGrammar: () => grammarEmitter(target.format)(grammar) });
    console.log(entry.id, target.format, 'raw-emitter', raw.status, JSON.stringify(raw.failures.map((f) => f.kind)));
  }
}
