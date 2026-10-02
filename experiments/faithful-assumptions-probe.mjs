// Prints, for every directed pair of the translation corpus, the assumptions
// the translation contract records with the reasons each was recorded.
import { readFileSync } from 'node:fs';
import { translateProgram } from '../js/src/program-translation.js';

const root = new URL('../parity/fixtures/translation-corpus/', import.meta.url);
const sources = { JavaScript: 'project.mjs', Rust: 'project.rs', Lean: 'project.lean', Rocq: 'project.v' };
for (const [from, file] of Object.entries(sources)) {
  const text = readFileSync(new URL(file, root), 'utf8');
  for (const to of Object.keys(sources)) {
    if (to === from) continue;
    const { contract } = translateProgram(text, from, to);
    console.log(`${from} -> ${to}: ${contract.support}`);
    const { semantics } = translateProgram(text, from, to);
    for (const assumption of semantics?.assumptions ?? []) console.log("  ", assumption.id, JSON.stringify(assumption.details));
  }
}
