import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { withHistoricalJavaScriptSource } from '../historical-javascript-source.mjs';

export const ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT = '9a835e469572341e6a8e5c6e9fe6f40b8655dfbe';

const experiment = 'experiments/original-link-complete-table-projection/';
const generator = experiment + 'generate.mjs';
const frozenPaths = [generator, ...['policy.mjs', 'rules.mjs', 'data/policy.lino', 'data/native-definitions.lino'].map(file => experiment + file),
  'experiments/original-link-table-projection/policy.mjs',
  'experiments/original-link-role-projection/policy.mjs',
  'experiments/original-link-symbol-projection/policy.mjs',
  'experiments/rust-text-certificate/data/policy.lino'];

/** Re-emit the entire original artifact with the compiler that produced it. */
export function regenerateHistoricalOriginalLinkCompleteTable() {
  return withHistoricalJavaScriptSource(ORIGINAL_LINK_COMPLETE_TABLE_MODEL_COMMIT, frozenPaths, directory => {
    const source = `import { generateOriginalLinkCompleteTableProjection } from './${generator}';
      process.stdout.write(JSON.stringify(generateOriginalLinkCompleteTableProjection(true)));`;
    const artifact = execFileSync(process.execPath, ['--max-old-space-size=256', '--input-type=module', '-e', source],
      { cwd: directory, encoding: 'utf8', maxBuffer: 2 * 1024 * 1024 });
    return JSON.parse(artifact);
  }, ['js/scripts/project-data.mjs']);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.deepEqual(process.argv.slice(2), ['--check']);
  const artifact = regenerateHistoricalOriginalLinkCompleteTable();
  console.log(`Reproduced all ${artifact.declarations.length} original Rust definitions and the complete artifact.`);
}
