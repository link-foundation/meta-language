import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { withHistoricalJavaScriptSource } from '../historical-javascript-source.mjs';
import { rustTextValueProjectionFixtures } from './generate-fixtures.mjs';

const directory = 'experiments/rust-text-value-projection/';
const historical = 'd14cf02a7e0a70ae1010b25c72b9a2e030e61291';
const producer = ['js/scripts/project-data.mjs', directory, 'experiments/generated-owned-rust',
  'parity/fixtures/rust-source-indirection.lino', 'parity/fixtures/rust-source-signatures.lino',
  'experiments/issue-199-rust-string-continuation-adapter.mjs', 'experiments/issue-199-rust-string-continuation-rules.mjs',
  'experiments/rust-owned-local-state-fixture.lino', 'experiments/owned-tuple-call-fixture.lino'];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  const file = new URL('./data/current-fixtures.lino', import.meta.url);
  if (check) {
    withHistoricalJavaScriptSource(historical, [directory + 'data/fixtures.lino'], cwd => {
      execFileSync(process.execPath, [directory + 'generate-fixtures.mjs', '--check'], { cwd, stdio: 'inherit' });
    }, producer);
    checkProjectData(file, rustTextValueProjectionFixtures());
  } else {
    writeProjectData(file, rustTextValueProjectionFixtures());
  }
}
