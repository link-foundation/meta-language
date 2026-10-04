// Runs the formal-ai JavaScript workload helpers on the local regression
// fixtures and compares them with a Rust probe output file.
// Usage: node experiments/issue-195-formal-ai-local-parity.mjs <rust-outputs.json> [js-outputs.json]
import { readFileSync, writeFileSync } from 'node:fs';
import * as api from '../js/src/index.js';
import {
  buildFormalAiInputs,
  compareRuntimeOutputs,
  formalAiHelpers,
} from '../js/scripts/issue-195-formal-ai-workload-probes.mjs';

const inputs = await buildFormalAiInputs(new URL('../parity/fixtures/formal-ai-regressions/', import.meta.url).pathname);
const started = Date.now();
const javascript = formalAiHelpers(api).outputs(inputs);
console.error(`javascript outputs in ${Date.now() - started} ms`);
if (process.argv[3]) writeFileSync(process.argv[3], JSON.stringify(javascript));
const rust = JSON.parse(readFileSync(process.argv[2], 'utf8'));
for (const { name, holds, detail } of compareRuntimeOutputs(javascript, rust)) {
  console.log(holds ? 'ok  ' : 'FAIL', name, holds ? JSON.stringify(detail) : detail);
}
