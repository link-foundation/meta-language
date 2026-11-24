import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { withHistoricalJavaScriptSource } from '../historical-javascript-source.mjs';
import { bundledTextCertificateSource } from './generate.mjs';
import { textCertificatePolicy } from './policy.mjs';

const directory = 'experiments/rust-text-certificate/';
const generator = directory + 'generate.mjs';
const original = [generator, ...['policy.mjs', 'rules.mjs', 'data/policy.lino', 'data/translation.lino'].map(file => directory + file)];
const dependencies = ['js/scripts/project-data.mjs', 'js/scripts/product-rule-style-corrections.mjs',
  'experiments/generate-rust-owned-local-state-rules.mjs', 'experiments/rust-text-value-projection'];

export function generateCurrentTextCertificate(check) {
  const program = checkProgram(parseJavaScript(bundledTextCertificateSource()));
  const emitted = emitRust(program);
  const artifact = { model: textCertificatePolicy.model, declarations: program.items.map(row => row.fullName),
    definitions: emitted.definitions, completeRust: emitted.text };
  const file = new URL('./data/current-translation.lino', import.meta.url);
  if (check) checkProjectData(file, artifact);
  else writeProjectData(file, artifact);
  return artifact;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  if (check) {
    withHistoricalJavaScriptSource('d490ca9e5c06c1a5231cdf09bbf7628bb75f5580', original, cwd => {
      execFileSync(process.execPath, [generator, '--check'], { cwd, stdio: 'inherit' });
    }, dependencies);
  }
  const artifact = generateCurrentTextCertificate(check);
  console.log(`Current portable text certificate replay covers ${artifact.declarations.length} declarations.`);
}
