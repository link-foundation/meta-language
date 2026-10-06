// Prints the contract metadata (encodings, assumptions, theorems, mapping count) of every corpus pair.
import { readFileSync } from 'node:fs';
import { runStages } from '../scripts/translation-stage-lib.mjs';
const dir = '../parity/fixtures/translation-corpus';
for (const ext of ['mjs', 'rs', 'lean', 'v']) {
  const stages = runStages(ext, readFileSync(`${dir}/project.${ext}`, 'utf8'));
  for (const [target, result] of Object.entries(stages.emit)) {
    const v = result.value;
    console.log(`== ${ext} -> ${target}`);
    console.log(' encodings:', v.encodings.map((e) => e.id).join(', '));
    console.log(' assumptions:', JSON.stringify(v.assumptions));
    console.log(' theorems:', JSON.stringify(v.theorems));
    console.log(' mappings:', v.mappings.length, JSON.stringify(v.mappings[0]));
  }
}
