// Verify frozen binary generation using the compiler that owns its receipts.
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { withHistoricalJavaScriptSource } from '../historical-javascript-source.mjs';

export const binaryMetadataGenerationRevision = '1bb41fae1cbe4c88592e3241d01563346c97fac0';
const binaryRecordGenerationRevision = '93c65ca1ada6c5001a56acc4430500e55062027a';

const binarySource = 'js/src/link-network-binary-rules.js';
const binaryGenerator = 'js/scripts/generate-link-network-binary-rules.mjs';
const binaryStyles = 'parity/self-translation/link-network-binary-rules-decorators.lino';
const experiment = 'experiments/binary-metadata-buffer/';
const candidateSources = ['candidate.mjs', 'prior-rules.mjs', 'generation-gap.lino',
  'native-policy.lino', 'native-styles.lino', 'native-generation.lino', 'native']
  .map(file => experiment + file);
const candidateStyleBuilder = 'js/scripts/binary-metadata-candidate-styles.mjs';
const recordGenerator = 'js/scripts/generate-link-network-binary-record-rules.mjs';

export function readHistoricalBinaryRustEmission() {
  return withHistoricalJavaScriptSource(binaryMetadataGenerationRevision, candidateSources, directory => {
    const source = `
      import { readFileSync } from 'node:fs';
      import { parseJavaScript } from './js/src/translation/javascript.js';
      import { checkProgram } from './js/src/translation/check.js';
      import { emitRust } from './js/src/translation/emit-rust.js';
      const emit = (file, utf16Strings) => {
        const program = checkProgram(parseJavaScript(readFileSync(file, 'utf8'), {referenceSemantics: false}));
        const {definitions, preludes} = emitRust(utf16Strings ? {...program, utf16Strings} : program);
        return {definitions, preludes};
      };
      const candidate = emit('${experiment}candidate.mjs', true);
      const baseline = emit('${experiment}prior-rules.mjs', false);
      process.stdout.write(JSON.stringify({candidate, baseline}));
    `;
    const raw = execFileSync(process.execPath, ['--max-old-space-size=384', '--input-type=module', '-e', source],
      { cwd: directory, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
    return JSON.parse(raw);
  });
}

function checkGeneration(mode) {
  const sharedPaths = [binarySource, binaryGenerator, binaryStyles];
  const recordPaths = [recordGenerator, candidateStyleBuilder, 'js/src/link-network-binary-record-rules.js',
    'js/scripts/binary-record-block-return-styles.mjs', 'js/scripts/binary-record-compiler-styles.mjs',
    'parity/self-translation/binary-record-ci-block-return-review-116894548503.lino',
    'parity/self-translation/binary-record-ci-style-review-116854044616.lino',
    'parity/self-translation/link-network-binary-record-generator-adapter.lino',
    'parity/self-translation/link-network-binary-record-generation.lino',
    'parity/self-translation/link-network-binary-record-rules-decorators.lino',
    'rust/src/link_network_binary_record_rules'];
  const commonModules = ['js/scripts/project-data.mjs'];
  const styleDependencies = ['js/scripts/product-rule-style-corrections.mjs',
    'js/scripts/source-text-byte-library-style.mjs'];
  const runGenerator = (revision, paths, command) =>
    withHistoricalJavaScriptSource(revision, paths, directory => {
      execFileSync(process.execPath, command, { cwd: directory, stdio: 'inherit' });
    }, [...commonModules, ...styleDependencies, 'rust/src/translation/frontend_rules.rs']);

  if (mode === 'legacy') {
    runGenerator(binaryMetadataGenerationRevision, [...sharedPaths, 'rust/src/link_network_binary_rules'],
      [binaryGenerator, '--check']);
  } else if (mode === 'candidate') {
    runGenerator(binaryMetadataGenerationRevision, [...sharedPaths, ...candidateSources, candidateStyleBuilder],
      [binaryGenerator, '--metadata-buffer-candidate', '--check']);
  } else if (mode === 'record') {
    runGenerator(binaryRecordGenerationRevision, [...sharedPaths, ...recordPaths], [recordGenerator, '--check']);
  } else {
    throw new Error('Select the legacy, candidate or record binary generation check');
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  checkGeneration(process.argv[2]);
}
