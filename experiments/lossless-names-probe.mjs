// Prints, for every reverse-conversion probe source, the imported rule names
// and docs, and the emitted text, to design the lossless layout chunking.
import { grammarImporter, grammarEmitter } from '../js/src/index.js';
import { ruleDoc } from '../js/src/grammar-emitters/structural.js';
import { SOURCES } from './issue-195-reverse-conversion-sources.mjs';

for (const [format, source] of Object.entries(SOURCES)) {
  const grammar = grammarImporter(format)(source);
  console.log(`=== ${format} start=${grammar.startRule().name}`);
  for (const rule of grammar.rules.values()) console.log(`  ${rule.name} ${rule.kind} doc=${JSON.stringify(ruleDoc(grammar, rule))}`);
  console.log(grammarEmitter(format)(grammar).source);
}
