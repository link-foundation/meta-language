import { execFileSync } from 'node:child_process';
import { withHistoricalJavaScriptSource } from '../historical-javascript-source.mjs';
import { ORIGINAL_LINK_RAW_SCHEMA_PRODUCER } from './raw-schema-source-closure.mjs';

const experiment = 'experiments/original-link-complete-table-projection/';
const generator = experiment + 'generate-raw.mjs';
const rawSources = [generator, ...['raw-rules.mjs', 'raw-policy.mjs', 'data/raw-policy.lino',
  'data/raw-native-definitions.lino'].map(file => experiment + file),
  'experiments/original-link-identity/policy.mjs', 'experiments/generated-owned-rust/data/policy.lino',
  'experiments/rust-text-value-projection/data/policy.lino'];

export function regenerateHistoricalRawInstructions() {
  // The original rule table consumes the historical schema's exact field order.
  // Regenerate it from that schema using the original policy and compiled graph reader.
  return withHistoricalJavaScriptSource(ORIGINAL_LINK_RAW_SCHEMA_PRODUCER, rawSources, directory => {
    const script = `import { generateOriginalLinkRawJavaScript } from './${generator}';
      process.stdout.write(generateOriginalLinkRawJavaScript(true));`;
    return execFileSync(process.execPath, ['--max-old-space-size=256', '--input-type=module', '-e', script],
      { cwd: directory, encoding: 'utf8', maxBuffer: 1024 * 1024 });
  }, ['js/scripts/project-data.mjs']);
}
