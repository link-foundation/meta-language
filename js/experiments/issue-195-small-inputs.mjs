// Runs the portable-core pipeline on the small inputs of the translation behavior tests.
import { runStages } from '../scripts/translation-stage-lib.mjs';
const cases = [['rs', 'pub fn answer() -> u32 { 42 }'], ['mjs', 'console.log(42);'], ['mjs', 'console.log(9007199254740993);'], ['rs', 'pub fn public() -> u32 { 1 }']];
for (const [ext, text] of cases) {
  const stages = runStages(ext, text);
  console.log(ext, JSON.stringify(text), stages.parse.error ?? stages.check?.error ?? Object.fromEntries(Object.entries(stages.emit).map(([k, v]) => [k, v.error ?? v.value.text.slice(-200)])));
}
